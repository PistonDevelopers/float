#![deny(missing_docs)]

//! Traits for generic floats in game programming
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign};

/// Convenience trait for floats.
pub trait Float:
    'static + Send + Sync
    + Copy + Radians + One + Zero + Sqrt
    + FromPrimitive
    + Min + Max + Signum + Powf
    + Trig
    + PartialEq
    + PartialOrd
    + Add<Self, Output = Self> + AddAssign<Self>
    + Mul<Self, Output = Self> + MulAssign<Self>
    + Sub<Self, Output = Self> + SubAssign<Self>
    + Div<Self, Output = Self> + DivAssign<Self>
    + Rem<Self, Output = Self> + RemAssign<Self>
    + Neg<Output = Self>
    + Trig {}

impl<T> Float for T where
    T: 'static + Send + Sync
    + Copy + Radians + One + Zero + Sqrt
    + FromPrimitive
    + Min + Max + Signum + Powf
    + Trig
    + PartialEq
    + PartialOrd
    + Add<T, Output = T> + AddAssign<T>
    + Mul<T, Output = T> + MulAssign<T>
    + Sub<T, Output = T> + SubAssign<T>
    + Div<T, Output = T> + DivAssign<T>
    + Rem<T, Output = T> + RemAssign<T>
    + Neg<Output = T>
    + Trig {}

/// Minimum value.
pub trait Min {
    /// Returns the minimum value of self or other.
    fn min(self, other: Self) -> Self;
}

impl Min for f32 {
    #[inline(always)]
    fn min(self, other: Self) -> Self { self.min(other) }
}

impl Min for f64 {
    #[inline(always)]
    fn min(self, other: Self) -> Self { self.min(other) }
}

/// Maximum value.
pub trait Max {
    /// Returns the maximum value of self or other.
    fn max(self, other: Self) -> Self;
}

impl Max for f32 {
    #[inline(always)]
    fn max(self, other: Self) -> Self { self.max(other) }
}

impl Max for f64 {
    #[inline(always)]
    fn max(self, other: Self) -> Self { self.max(other) }
}

/// The sign of the number.
pub trait Signum {
    /// Returns number representing the sign of self
    fn signum(self) -> Self;
}

impl Signum for f32 {
    #[inline(always)]
    fn signum(self) -> Self { self.signum() }
}

impl Signum for f64 {
    #[inline(always)]
    fn signum(self) -> Self { self.signum() }
}

/// Floating number power.
pub trait Powf {
    /// Returns floating power of the number.
    fn powf(self, other: Self) -> Self;
}

impl Powf for f32 {
    #[inline(always)]
    fn powf(self, other: Self) -> Self { self.powf(other) }
}

impl Powf for f64 {
    #[inline(always)]
    fn powf(self, other: Self) -> Self { self.powf(other) }
}

/// Useful constants for radians.
pub trait Radians {
    /// Returns radians corresponding to 90 degrees.
    fn _90() -> Self;

    /// Returns radians corresponding to 180 degrees.
    fn _180() -> Self;

    /// Returns radians corresponding to 360 degrees.
    fn _360() -> Self;

    /// Convert a value to radians from degrees.
    /// Equivalent to ```value * (π / 180)```.
    fn deg_to_rad(self) -> Self;

    /// Convert a value to degrees from radians.
    /// Equivalent to ```value * (180 / π)```.
    fn rad_to_deg(self) -> Self;
}

impl Radians for f32 {
    #[inline(always)]
    fn _90() -> f32 {
        ::std::f32::consts::FRAC_PI_2
    }

    #[inline(always)]
    fn _180() -> f32 {
        ::std::f32::consts::PI
    }

    #[inline(always)]
    fn _360() -> f32 {
        <Self as Radians>::_180() * 2.0
    }

    #[inline(always)]
    fn deg_to_rad(self) -> Self {
        self * (::std::f32::consts::PI / 180.0_f32)
    }

    #[inline(always)]
    fn rad_to_deg(self) -> Self {
        self * (180.0_f32 / ::std::f32::consts::PI)
    }
}

impl Radians for f64 {
    #[inline(always)]
    fn _90() -> f64 {
        ::std::f64::consts::FRAC_PI_2
    }

    #[inline(always)]
    fn _180() -> f64 {
        ::std::f64::consts::PI
    }

    #[inline(always)]
    fn _360() -> f64 {
        <Self as Radians>::_180() * 2.0
    }

    #[inline(always)]
    fn deg_to_rad(self) -> Self {
        self * (::std::f64::consts::PI / 180.0_f64)
    }

    #[inline(always)]
    fn rad_to_deg(self) -> Self {
        self * (180.0_f64 / ::std::f64::consts::PI)
    }
}

/// Number 1.
pub trait One {
    /// Returns 1.
    fn one() -> Self;
}

/// Number 0.
pub trait Zero {
    /// Returns 0.
    fn zero() -> Self;
}

impl One for f32 {
    #[inline(always)]
    fn one() -> f32 { 1.0 }
}

impl One for f64 {
    #[inline(always)]
    fn one() -> f64 { 1.0 }
}

impl Zero for f32 {
    #[inline(always)]
    fn zero() -> f32 { 0.0 }
}

impl Zero for f64 {
    #[inline(always)]
    fn zero() -> f64 { 0.0 }
}

/// Square root.
pub trait Sqrt {
    /// Returns square root.
    fn sqrt(self) -> Self;
}

impl Sqrt for f32 {
    #[inline(always)]
    fn sqrt(self) -> f32 { self.sqrt() }
}

impl Sqrt for f64 {
    #[inline(always)]
    fn sqrt(self) -> f64 { self.sqrt() }
}

/// Basic trigonometry functions
pub trait Trig {
    /// Returns sine of self.
    fn sin(self) -> Self;
    /// Returns cosine of self.
    fn cos(self) -> Self;
    /// Returns tangent of self.
    fn tan(self) -> Self;
    /// Returns inverse sine of self.
    fn asin(self) -> Self;
    /// Returns inverse cosine of self.
    fn acos(self) -> Self;
    /// Returns inverse tangent of self.
    fn atan(self) -> Self;
    /// Returns the four quadrant arctangent of self (y) and other (x).
    fn atan2(self, other: Self) -> Self;
    /// Returns hyperbolic sine of self.
    fn sinh(self) -> Self;
    /// Returns hyperbolic cosine of self.
    fn cosh(self) -> Self;
    /// Returns hyperbolic tangent of self.
    fn tanh(self) -> Self;
    /// Returns inverse hyperbolic sine of self.
    fn asinh(self) -> Self;
    /// Returns inverse hyperbolic cosine of self.
    fn acosh(self) -> Self;
    /// Returns inverse hyperbolic tangent of self.
    fn atanh(self) -> Self;
}

impl Trig for f32 {
    #[inline(always)]
    fn sin(self) -> f32 { self.sin() }

    #[inline(always)]
    fn cos(self) -> f32 { self.cos() }

    #[inline(always)]
    fn tan(self) -> f32 { self.tan() }

    #[inline(always)]
    fn asin(self) -> f32 { self.asin() }

    #[inline(always)]
    fn acos(self) -> f32 { self.acos() }

    #[inline(always)]
    fn atan(self) -> f32 { self.atan() }

    #[inline(always)]
    fn atan2(self, other: f32) -> f32 { self.atan2(other) }

    #[inline(always)]
    fn sinh(self) -> f32 { self.sinh() }

    #[inline(always)]
    fn cosh(self) -> f32 { self.cosh() }

    #[inline(always)]
    fn tanh(self) -> f32 { self.tanh() }

    #[inline(always)]
    fn asinh(self) -> f32 { self.asinh() }

    #[inline(always)]
    fn acosh(self) -> f32 { self.acosh() }

    #[inline(always)]
    fn atanh(self) -> f32 { self.atanh() }
}

impl Trig for f64 {
    #[inline(always)]
    fn sin(self) -> f64 { self.sin() }

    #[inline(always)]
    fn cos(self) -> f64 { self.cos() }

    #[inline(always)]
    fn tan(self) -> f64 { self.tan() }

    #[inline(always)]
    fn asin(self) -> f64 { self.asin() }

    #[inline(always)]
    fn acos(self) -> f64 { self.acos() }

    #[inline(always)]
    fn atan(self) -> f64 { self.atan() }

    #[inline(always)]
    fn atan2(self, other: f64) -> f64 { self.atan2(other) }

    #[inline(always)]
    fn sinh(self) -> f64 { self.sinh() }

    #[inline(always)]
    fn cosh(self) -> f64 { self.cosh() }

    #[inline(always)]
    fn tanh(self) -> f64 { self.tanh() }

    #[inline(always)]
    fn asinh(self) -> f64 { self.asinh() }

    #[inline(always)]
    fn acosh(self) -> f64 { self.acosh() }

    #[inline(always)]
    fn atanh(self) -> f64 { self.atanh() }
}

/// Casts into another type.
pub trait Cast<T> {
    /// Casts into other type.
    fn cast(self) -> T;
}

impl Cast<f32> for f64 {
    #[inline(always)]
    fn cast(self) -> f32 { self as f32 }
}

impl Cast<f64> for f32 {
    #[inline(always)]
    fn cast(self) -> f64 { self as f64 }
}

impl Cast<f32> for f32 {
    #[inline(always)]
    fn cast(self) -> f32 { self }
}

impl Cast<f64> for f64 {
    #[inline(always)]
    fn cast(self) -> f64 { self }
}

/// Trait for converting from different numeric types
pub trait FromPrimitive {
    /// from a f64
    fn from_f64(t: f64) -> Self;
    /// from a f32
    fn from_f32(t: f32) -> Self;
    /// from a isze
    fn from_isize(t: isize) -> Self;
    /// from a u32
    fn from_u32(t: u32) -> Self;
    /// from a i32
    fn from_i32(t: i32) -> Self;
    // Add more as needed..
}

impl FromPrimitive for f64 {
    #[inline(always)]
    fn from_f64(t: f64) -> Self { t }
    #[inline(always)]
    fn from_f32(t: f32) -> Self { t as f64 }
    #[inline(always)]
    fn from_isize(t: isize) -> Self { t as f64 }
    #[inline(always)]
    fn from_u32(t: u32) -> Self { t as f64 }
    #[inline(always)]
    fn from_i32(t: i32) -> Self { t as f64 }
}

impl FromPrimitive for f32 {
    #[inline(always)]
    fn from_f64(t: f64) -> Self { t as f32 }
    #[inline(always)]
    fn from_f32(t: f32) -> Self { t }
    #[inline(always)]
    fn from_isize(t: isize) -> Self { t as f32 }
    #[inline(always)]
    fn from_u32(t: u32) -> Self { t as f32 }
    #[inline(always)]
    fn from_i32(t: i32) -> Self { t as f32 }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_f32_sqrt() {
        let a = 4.0_f32;
        let b = <f32 as Sqrt>::sqrt(a);
        assert!((b - 2.0_f32).abs() < f32::EPSILON)
    }

    #[test]
    fn test_f64_sqrt() {
        let a = 4.0_f64;
        let b = <f64 as Sqrt>::sqrt(a);
        assert!((b - 2.0_f64).abs() < f64::EPSILON)
    }

    #[test]
    fn test_f32_deg_to_rad() {
        let degrees = 23.0_f32;
        let radians = degrees.deg_to_rad();
        assert!((radians - 0.401_425).abs() > f32::EPSILON);
    }

    #[test]
    fn test_f64_deg_to_rad() {
        let degrees = 60.0_f64;
        let radians = degrees.deg_to_rad();
        assert!((radians - std::f64::consts::FRAC_PI_3).abs()  == f64::EPSILON);
    }
}
