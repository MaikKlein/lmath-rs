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

//! Coordinate vectors for positional data
//!
//! These types differ from the vector types implemented in `core::vec` because
//! they describe coordinates in geometric space and not a magnitude and a
//! direction. All positional data throughout the library uses these point
//! types, which allows for a clear, self-documenting API.

use std::cast;

use math::{Dimensioned, SwapComponents};
use math::{Ray2, Ray3};
use math::{Vec2, ToVec2, AsVec2};
use math::{Vec3, ToVec3, AsVec3};
use math::{Vec4, ToVec4};

/// A coordinate vector
pub trait Point<T, Vec, Ray>: Eq
                            + Add<Vec, Self>
                            + Sub<Self, Vec>
                            + Mul<Vec, Self>
                            + ApproxEq<T>
                            + ToStr {
    fn translate_v(&self, offset: &Vec) -> Self;
    fn scale_s(&self, factor: T) -> Self;
    fn scale_v(&self, factor: &Vec) -> Self;
    fn displacement(&self, other: &Self) -> Vec;
    fn distance2(&self, other: &Self) -> T;
    fn distance(&self, other: &Self) -> T;
    fn direction(&self, other: &Self) -> Vec;
    fn ray_to(&self, other: &Self) -> Ray;
}

/// A two-dimensional coordinate vector
#[deriving(Clone, Eq)]
pub struct Point2<T> { x: T, y: T }

impl_dimensioned!(Point2, T, 2)
impl_to_vec!(Point2, 2)
impl_as_vec!(Point2, 2)
impl_swap_components!(Point2)
impl_approx!(Point2 { x, y })

pub trait AsPoint2<T> {
    fn as_point2<'a>(&'a self) -> &'a Point2<T>;
    fn as_mut_point2<'a>(&'a mut self) -> &'a mut Point2<T>;
}

impl<T:Clone + Num> AsPoint2<T> for Vec2<T> {
    #[inline]
    pub fn as_point2<'a>(&'a self) -> &'a Point2<T> {
        unsafe { cast::transmute(self) }
    }

    #[inline]
    pub fn as_mut_point2<'a>(&'a mut self) -> &'a mut Point2<T> {
        unsafe { cast::transmute(self) }
    }
}

impl<T:Num> Point2<T> {
    /// Creates a new point from three coordinates.
    #[inline]
    pub fn new(x: T, y: T) -> Point2<T> {
        Point2 { x: x, y: y }
    }

    /// Converts a vector to a point.
    #[inline]
    pub fn from_vec2(vec: Vec2<T>) -> Point2<T> {
        unsafe { cast::transmute(vec) }
    }

    /// The coordinate [0, 0].
    #[inline]
    pub fn origin() -> Point2<T> {
        Point2::new(zero!(T), zero!(T))
    }
}

impl<T:Clone + Num> ToVec3<T> for Point2<T> {
    /// Converts the point to a three-dimensional homogeneous vector:
    /// `[x, y] -> [x, y, 1]`
    #[inline]
    pub fn to_vec3(&self) -> Vec3<T> {
        Vec3::new(self.x.clone(),
                  self.y.clone(),
                  one!(T))
    }
}

impl<T:Clone + Float> Point<T, Vec2<T>, Ray2<T>> for Point2<T> {
    /// Applies a displacement vector to the point.
    #[inline]
    pub fn translate_v(&self, offset: &Vec2<T>) -> Point2<T> {
        Point2::new(self.x + offset.x,
                    self.y + offset.y)
    }

    /// Scales the distance from the point to the origin by a scalar value.
    #[inline]
    pub fn scale_s(&self, factor: T) -> Point2<T> {
        Point2::new(self.x * factor,
                    self.y * factor)
    }

    /// Scales the distance from the point to the origin using the components
    /// of a vector.
    #[inline]
    pub fn scale_v(&self, factor: &Vec2<T>) -> Point2<T> {
        Point2::new(self.x * factor.x,
                    self.y * factor.y)
    }

    /// Calculates the displacement required to move the point to `other`.
    pub fn displacement(&self, other: &Point2<T>) -> Vec2<T> {
        Vec2::new(self.x - other.x,
                  self.y - other.y)
    }

    /// Returns the squared distance from the point to `other`. This does not
    /// perform a square root operation like in the `distance` method and can
    /// therefore be more efficient for distance comparisons where the actual
    /// distance is not needed.
    #[inline]
    pub fn distance2(&self, other: &Point2<T>) -> T {
        ((*other) - (*self)).magnitude2()
    }

    /// Returns the scalar distance to the other point.
    #[inline]
    pub fn distance(&self, other: &Point2<T>) -> T {
        other.distance2(self).sqrt()
    }

    /// Returns a normalized direction vector pointing to the other point.
    #[inline]
    pub fn direction(&self, other: &Point2<T>) -> Vec2<T> {
        ((*other) - (*self)).normalize()
    }

    /// Projects a normalized ray towards the other point.
    #[inline]
    pub fn ray_to(&self, other: &Point2<T>) -> Ray2<T> {
        Ray2::new(self.clone(), self.direction(other))
    }
}

impl<T:Clone + Float> Add<Vec2<T>, Point2<T>> for Point2<T> {
    /// Applies a displacement vector to the point.
    fn add(&self, offset: &Vec2<T>) -> Point2<T> {
        self.translate_v(offset)
    }
}

impl<T:Clone + Float> Sub<Point2<T>, Vec2<T>> for Point2<T> {
    /// Calculates the displacement vector from the point to `other`.
    fn sub(&self, other: &Point2<T>) -> Vec2<T> {
        self.displacement(other)
    }
}

impl<T:Clone + Float> Mul<Vec2<T>, Point2<T>> for Point2<T> {
    /// Scales the distance from the point to the origin using the components
    /// of a vector.
    fn mul(&self, factor: &Vec2<T>) -> Point2<T> {
        self.scale_v(factor)
    }
}

impl<T> ToStr for Point2<T> {
    pub fn to_str(&self) -> ~str {
        fmt!("[%?, %?]", self.x, self.y)
    }
}

#[cfg(test)]
mod test_point2 {
    use math::point::*;

    #[test]
    fn test_to_str() {
        assert_eq!(Point2::new(1, 2).to_str(), ~"[1, 2]");
    }
}

/// A three-dimensional coordinate vector
#[deriving(Clone, Eq)]
pub struct Point3<T> { x: T, y: T, z: T }

impl_dimensioned!(Point3, T, 3)
impl_to_vec!(Point3, 3)
impl_as_vec!(Point3, 3)
impl_swap_components!(Point3)
impl_approx!(Point3 { x, y, z })

pub trait AsPoint3<T> {
    fn as_point3<'a>(&'a self) -> &'a Point3<T>;
    fn as_mut_point3<'a>(&'a mut self) -> &'a mut Point3<T>;
}

impl<T:Clone + Num> AsPoint3<T> for Vec3<T> {
    #[inline]
    pub fn as_point3<'a>(&'a self) -> &'a Point3<T> {
        unsafe { cast::transmute(self) }
    }

    #[inline]
    pub fn as_mut_point3<'a>(&'a mut self) -> &'a mut Point3<T> {
        unsafe { cast::transmute(self) }
    }
}

impl<T:Num> Point3<T> {
    /// Creates a new point from three coordinates.
    #[inline]
    pub fn new(x: T, y: T, z: T) -> Point3<T> {
        Point3 { x: x, y: y, z: z }
    }

    /// Converts a vector to a point.
    #[inline]
    pub fn from_vec3(vec: Vec3<T>) -> Point3<T> {
        unsafe { cast::transmute(vec) }
    }

    /// The coordinate [0, 0, 0].
    #[inline]
    pub fn origin() -> Point3<T> {
        Point3::new(zero!(T), zero!(T), zero!(T))
    }
}

impl<T:Clone + Num> ToVec4<T> for Point3<T> {
    /// Converts the point to a four-dimensional homogeneous vector:
    /// `[x, y, z] -> [x, y, z, 1]`
    #[inline]
    pub fn to_vec4(&self) -> Vec4<T> {
        Vec4::new(self.x.clone(),
                  self.y.clone(),
                  self.z.clone(),
                  one!(T))
    }
}

impl<T:Clone + Float> Point<T, Vec3<T>, Ray3<T>> for Point3<T> {
    /// Applies a displacement vector to the point.
    #[inline]
    pub fn translate_v(&self, offset: &Vec3<T>) -> Point3<T> {
        Point3::new(self.x + offset.x,
                    self.y + offset.y,
                    self.z + offset.z)
    }

    /// Scales the distance from the point to the origin by a scalar value.
    #[inline]
    pub fn scale_s(&self, factor: T) -> Point3<T> {
        Point3::new(self.x * factor,
                    self.y * factor,
                    self.z * factor)
    }

    /// Scales the distance from the point to the origin using the components
    /// of a vector.
    #[inline]
    pub fn scale_v(&self, factor: &Vec3<T>) -> Point3<T> {
        Point3::new(self.x * factor.x,
                    self.y * factor.y,
                    self.z * factor.z)
    }

    /// Calculates the displacement required to move the point to `other`.
    pub fn displacement(&self, other: &Point3<T>) -> Vec3<T> {
        Vec3::new(self.x - other.x,
                  self.y - other.y,
                  self.z - other.z)
    }

    /// Returns the squared distance from the point to `other`. This does not
    /// perform a square root operation like in the `distance` method and can
    /// therefore be more efficient for distance comparisons where the actual
    /// distance is not needed.
    #[inline]
    pub fn distance2(&self, other: &Point3<T>) -> T {
        ((*other) - (*self)).magnitude2()
    }

    /// Returns the scalar distance to the other point.
    #[inline]
    pub fn distance(&self, other: &Point3<T>) -> T {
        other.distance2(self).sqrt()
    }

    /// Returns a normalized direction vector pointing to the other point.
    #[inline]
    pub fn direction(&self, other: &Point3<T>) -> Vec3<T> {
        ((*other) - (*self)).normalize()
    }

    /// Projects a normalized ray towards the other point.
    #[inline]
    pub fn ray_to(&self, other: &Point3<T>) -> Ray3<T> {
        Ray3::new(self.clone(), self.direction(other))
    }
}

impl<T:Clone + Float> Add<Vec3<T>, Point3<T>> for Point3<T> {
    /// Applies a displacement vector to the point
    fn add(&self, offset: &Vec3<T>) -> Point3<T> {
        self.translate_v(offset)
    }
}

impl<T:Clone + Float> Sub<Point3<T>, Vec3<T>> for Point3<T> {
    /// Calculates the displacement required to move the point to `other`.
    fn sub(&self, other: &Point3<T>) -> Vec3<T> {
        self.displacement(other)
    }
}

impl<T:Clone + Float> Mul<Vec3<T>, Point3<T>> for Point3<T> {
    /// Scales the distance from the point to the origin using the components
    /// of a vector.
    fn mul(&self, factor: &Vec3<T>) -> Point3<T> {
        self.scale_v(factor)
    }
}

impl<T> ToStr for Point3<T> {
    pub fn to_str(&self) -> ~str {
        fmt!("[%?, %?, %?]", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod test_point3 {
    use math::point::*;

    #[test]
    fn test_to_str() {
        assert_eq!(Point3::new(1, 2, 3).to_str(), ~"[1, 2, 3]");
    }
}
