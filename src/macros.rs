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

#[macro_escape];

macro_rules! zero(
    ($T:ty) => ({
        use std::num::Zero;
        Zero::zero::<$T>()
    });
)

macro_rules! one(
    ($T:ty) => ({
        use std::num::One;
        One::one::<$T>()
    });
)

macro_rules! two(
    ($T:ty) => (one!(T) + one!(T));
)

macro_rules! impl_approx(
    ($T:ident { $($field:ident),+ }) => (
        impl<T:Clone + Eq + ApproxEq<T>> ApproxEq<T> for $T<T> {
            #[inline]
            pub fn approx_epsilon() -> T {
                ApproxEq::approx_epsilon::<T,T>()
            }

            #[inline]
            pub fn approx_eq(&self, other: &$T<T>) -> bool {
                self.approx_eq_eps(other, &ApproxEq::approx_epsilon::<T,T>())
            }

            #[inline]
            pub fn approx_eq_eps(&self, other: &$T<T>, epsilon: &T) -> bool {
                $( self.$field.approx_eq_eps(&other.$field, epsilon) )&&+
            }
        }
    );
    ($T:ident) => (
        impl<T:Clone + Eq + ApproxEq<T>> ApproxEq<T> for $T<T> {
            #[inline]
            pub fn approx_epsilon() -> T {
                ApproxEq::approx_epsilon::<T,T>()
            }

            #[inline]
            pub fn approx_eq(&self, other: &$T<T>) -> bool {
                self.approx_eq_eps(other, &ApproxEq::approx_epsilon::<T,T>())
            }

            #[inline]
            pub fn approx_eq_eps(&self, other: &$T<T>, epsilon: &T) -> bool {
                (**self).approx_eq_eps(&**other, epsilon)
            }
        }
    )
)
