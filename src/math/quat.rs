// Copyright 2013 The Lmath Developers. For a full listing of the authors,
// refer to the AUTHORS file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Quaternion type

use math::{Dimensioned, SwapComponents};
use math::{Mat3, ToMat3};
use math::Vec3;

// GLSL-style type aliases

pub type quat  = Quat<f32>;
pub type dquat = Quat<f64>;

// Rust-style type aliases

pub type Quatf   = Quat<float>;
pub type Quatf32 = Quat<f32>;
pub type Quatf64 = Quat<f64>;

/// A quaternion in scalar/vector form
#[deriving(Clone, Eq)]
pub struct Quat<T> { s: T, v: Vec3<T> }

impl_dimensioned!(Quat, T, 4)
impl_swap_components!(Quat)
impl_approx!(Quat { s, v })

pub trait ToQuat<T> {
    fn to_quat(&self) -> Quat<T>;
}

impl<T> Quat<T> {
    /// Construct the quaternion from one scalar component and three
    /// imaginary components
    ///
    /// # Arguments
    ///
    /// - `w`: the scalar component
    /// - `xi`: the fist imaginary component
    /// - `yj`: the second imaginary component
    /// - `zk`: the third imaginary component
    #[inline]
    pub fn new(w: T, xi: T, yj: T, zk: T) -> Quat<T> {
        Quat::from_sv(w, Vec3::new(xi, yj, zk))
    }

    /// Construct the quaternion from a scalar and a vector
    ///
    /// # Arguments
    ///
    /// - `s`: the scalar component
    /// - `v`: a vector containing the three imaginary components
    #[inline]
    pub fn from_sv(s: T, v: Vec3<T>) -> Quat<T> {
        Quat { s: s, v: v }
    }
}

impl<T:Clone + Float> Quat<T> {
    #[inline]
    pub fn look_at(dir: &Vec3<T>, up: &Vec3<T>) -> Quat<T> {
        Mat3::look_at(dir, up).to_quat()
    }

    #[inline]
    pub fn from_axes(x: Vec3<T>, y: Vec3<T>, z: Vec3<T>) -> Quat<T> {
        Mat3::from_axes(x, y, z).to_quat()
    }

    /// The multiplicative identity, ie: `q = 1 + 0i + 0j + 0i`
    #[inline]
    pub fn identity() -> Quat<T> {
        Quat::from_sv(one!(T), Vec3::zero())
    }

    /// The additive identity, ie: `q = 0 + 0i + 0j + 0i`
    #[inline]
    pub fn zero() -> Quat<T> {
        Quat::new(zero!(T), zero!(T), zero!(T), zero!(T))
    }

    /// The result of multiplying the quaternion a scalar
    #[inline]
    pub fn mul_s(&self, value: T) -> Quat<T> {
        Quat::from_sv(self.s * value, self.v.mul_s(value))
    }

    /// The result of dividing the quaternion a scalar
    #[inline]
    pub fn div_s(&self, value: T) -> Quat<T> {
        Quat::from_sv(self.s / value, self.v.div_s(value))
    }

    /// The result of multiplying the quaternion by a vector
    #[inline]
    pub fn mul_v(&self, vec: &Vec3<T>) -> Vec3<T>  {
        let tmp = self.v.cross(vec).add_v(&vec.mul_s(self.s.clone()));
        self.v.cross(&tmp).mul_s(two!(T)).add_v(vec)
    }

    /// The sum of this quaternion and `other`
    #[inline]
    pub fn add_q(&self, other: &Quat<T>) -> Quat<T> {
        Quat::new(*self.i(0) + *other.i(0),
                  *self.i(1) + *other.i(1),
                  *self.i(2) + *other.i(2),
                  *self.i(3) + *other.i(3))
    }

    /// The sum of this quaternion and `other`
    #[inline]
    pub fn sub_q(&self, other: &Quat<T>) -> Quat<T> {
        Quat::new(*self.i(0) - *other.i(0),
                  *self.i(1) - *other.i(1),
                  *self.i(2) - *other.i(2),
                  *self.i(3) - *other.i(3))
    }

    /// The the result of multipliplying the quaternion by `other`
    pub fn mul_q(&self, other: &Quat<T>) -> Quat<T> {
        Quat::new(self.s * other.s - self.v.x * other.v.x - self.v.y * other.v.y - self.v.z * other.v.z,
                  self.s * other.v.x + self.v.x * other.s + self.v.y * other.v.z - self.v.z * other.v.y,
                  self.s * other.v.y + self.v.y * other.s + self.v.z * other.v.x - self.v.x * other.v.z,
                  self.s * other.v.z + self.v.z * other.s + self.v.x * other.v.y - self.v.y * other.v.x)
    }

    /// The dot product of the quaternion and `other`
    #[inline]
    pub fn dot(&self, other: &Quat<T>) -> T {
        self.s * other.s + self.v.dot(&other.v)
    }

    /// The conjugate of the quaternion
    #[inline]
    pub fn conjugate(&self) -> Quat<T> {
        Quat::from_sv(self.s.clone(), -self.v.clone())
    }

    /// The multiplicative inverse of the quaternion
    #[inline]
    pub fn inverse(&self) -> Quat<T> {
        self.conjugate().div_s(self.magnitude2())
    }

    /// The squared magnitude of the quaternion. This is useful for
    /// magnitude comparisons where the exact magnitude does not need to be
    /// calculated.
    #[inline]
    pub fn magnitude2(&self) -> T {
        self.s * self.s + self.v.magnitude2()
    }

    /// The magnitude of the quaternion
    ///
    /// # Performance notes
    ///
    /// For instances where the exact magnitude of the quaternion does not need
    /// to be known, for example for quaternion-quaternion magnitude comparisons,
    /// it is advisable to use the `magnitude2` method instead.
    #[inline]
    pub fn magnitude(&self) -> T {
        self.magnitude2().sqrt()
    }

    /// The normalized quaternion
    #[inline]
    pub fn normalize(&self) -> Quat<T> {
        self.mul_s(one!(T) / self.magnitude())
    }

    /// Normalised linear interpolation
    ///
    /// # Return value
    ///
    /// The intoperlated quaternion
    pub fn nlerp(&self, other: &Quat<T>, amount: T) -> Quat<T> {
        self.mul_s(one!(T) - amount).add_q(&other.mul_s(amount)).normalize()
    }

    /// Spherical Linear Intoperlation
    ///
    /// Perform a spherical linear interpolation between the quaternion and
    /// `other`. Both quaternions should be normalized first.
    ///
    /// # Return value
    ///
    /// The intoperlated quaternion
    ///
    /// # Performance notes
    ///
    /// The `acos` operation used in `slerp` is an expensive operation, so unless
    /// your quarternions a far away from each other it's generally more advisable
    /// to use `nlerp` when you know your rotations are going to be small.
    ///
    /// - [Understanding Slerp, Then Not Using It]
    ///   (http://number-none.com/product/Understanding%20Slerp,%20Then%20Not%20Using%20It/)
    /// - [Arcsynthesis OpenGL tutorial]
    ///   (http://www.arcsynthesis.org/gltut/Positioning/Tut08%20Interpolation.html)
    pub fn slerp(&self, other: &Quat<T>, amount: T) -> Quat<T> {
        use std::num::cast;

        let dot = self.dot(other);
        let dot_threshold = cast(0.9995);

        // if quaternions are close together use `nlerp`
        if dot > dot_threshold {
            self.nlerp(other, amount)
        } else {
            // stay within the domain of acos()
            let robust_dot = dot.clamp(&-one!(T), &one!(T));

            let theta_0 = robust_dot.acos();    // the angle between the quaternions
            let theta = theta_0 * amount;       // the fraction of theta specified by `amount`

            let q = other.sub_q(&self.mul_s(robust_dot))
                         .normalize();

            self.mul_s(theta.cos())
                .add_q(&q.mul_s(theta.sin()))
        }
    }
}

impl<T:Clone + Num> ToMat3<T> for Quat<T> {
    /// Convert the quaternion to a 3 x 3 rotation matrix
    pub fn to_mat3(&self) -> Mat3<T> {
        let x2 = self.v.x + self.v.x;
        let y2 = self.v.y + self.v.y;
        let z2 = self.v.z + self.v.z;

        let xx2 = x2 * self.v.x;
        let xy2 = x2 * self.v.y;
        let xz2 = x2 * self.v.z;

        let yy2 = y2 * self.v.y;
        let yz2 = y2 * self.v.z;
        let zz2 = z2 * self.v.z;

        let sy2 = y2 * self.s;
        let sz2 = z2 * self.s;
        let sx2 = x2 * self.s;

        let _1: T = one!(T);

        Mat3::new(_1 - yy2 - zz2, xy2 + sz2, xz2 - sy2,
                  xy2 - sz2, _1 - xx2 - zz2, yz2 + sx2,
                  xz2 + sy2, yz2 - sx2, _1 - xx2 - yy2)
    }
}

impl<T:Clone + Float> Neg<Quat<T>> for Quat<T> {
    #[inline]
    pub fn neg(&self) -> Quat<T> {
        Quat::from_sv(-self.s, -self.v)
    }
}
