//! General length type utilities.

use az::SaturatingCast;
use fixed::types::extra::*;
use num_traits::{bounds::Bounded, sign::Signed, NumAssign, Zero};

/// A generic length trait.
///
/// A more specific number type is needed for CSS handling.
/// In most cases `f32` is what you need, but sometimes fixed-pointer types are preferred.
pub trait LengthNum:
    NumAssign + Bounded + Signed + PartialEq + PartialOrd + Clone + Copy + core::fmt::Debug + Zero
{
    /// A target type used for hashing.
    type Hashable: Eq + core::hash::Hash + core::fmt::Debug;

    /// Convert to the hashable type.
    fn to_hashable(&self) -> Self::Hashable;

    /// The number is normal (a.k.a. `f32::is_normal`) or not.
    fn is_normal(&self) -> bool;

    /// Convert from `f32`.
    fn from_f32(v: f32) -> Self;

    /// Convert to `f32`.
    fn to_f32(self) -> f32;

    /// Multiply `f32`.
    fn mul_f32(self, v: f32) -> Self {
        self * Self::from_f32(v)
    }

    /// Divide `f32`.
    fn div_f32(self, v: f32) -> Self {
        self / Self::from_f32(v)
    }

    /// Convert from `i32`.
    fn from_i32(v: i32) -> Self;

    /// Multiply `i32`.
    fn mul_i32(self, v: i32) -> Self {
        self * Self::from_i32(v)
    }

    /// Divide `i32`.
    fn div_i32(self, v: i32) -> Self {
        self / Self::from_i32(v)
    }

    /// Limit the number with an upper bound.
    fn upper_bound(self, v: Self) -> Self {
        if self <= v {
            self
        } else {
            v
        }
    }

    /// Limit the number with an lower bound.
    fn lower_bound(self, v: Self) -> Self {
        if self >= v {
            self
        } else {
            v
        }
    }
}

/// Get the sum of a series of `LengthNum`s.
pub fn length_sum<L: LengthNum>(iter: impl Iterator<Item = L>) -> L {
    let mut ret = L::zero();
    for x in iter {
        ret += x;
    }
    ret
}

impl LengthNum for f32 {
    type Hashable = u32;

    fn to_hashable(&self) -> Self::Hashable {
        if self.is_normal() {
            self.to_bits()
        } else {
            f32::NEG_INFINITY.to_bits()
        }
    }

    fn is_normal(&self) -> bool {
        f32::is_normal(*self)
    }

    fn from_f32(v: f32) -> Self {
        v
    }

    fn to_f32(self) -> f32 {
        self
    }

    fn from_i32(v: i32) -> Self {
        v as f32
    }
}

impl LengthNum for f64 {
    type Hashable = u64;

    fn to_hashable(&self) -> Self::Hashable {
        if self.is_normal() {
            self.to_bits()
        } else {
            f64::NEG_INFINITY.to_bits()
        }
    }

    fn is_normal(&self) -> bool {
        f64::is_normal(*self)
    }

    fn from_f32(v: f32) -> Self {
        v as f64
    }

    fn to_f32(self) -> f32 {
        self as f32
    }

    fn from_i32(v: i32) -> Self {
        v as f64
    }
}

macro_rules! raw_impl {
    ($base:ty) => {
        type Hashable = Self;

        fn to_hashable(&self) -> Self::Hashable {
            *self
        }

        fn is_normal(&self) -> bool {
            true
        }
        fn from_f32(v: f32) -> Self {
            let base = Self::from_num(1).to_bits() as f32;
            Self::from_bits((v * base) as $base)
        }

        fn to_f32(self) -> f32 {
            self.saturating_cast()
        }

        fn from_i32(v: i32) -> Self {
            v.saturating_cast()
        }
    };
}

impl<Frac> LengthNum for fixed::FixedI32<Frac>
where
    Frac: LeEqU32 + IsLessOrEqual<U30, Output = True>,
{
    raw_impl!(i32);
}

impl<Frac> LengthNum for fixed::FixedI64<Frac>
where
    Frac: LeEqU64 + IsLessOrEqual<U62, Output = True>,
{
    raw_impl!(i64);
}
