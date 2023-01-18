//! # Size
//! Helpers for pretty-printing byte sizes

use std::{
    fmt::{Debug, Display},
    mem::size_of,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};

/// Wrapper around a [`usize`] that allows pretty printing of byte sizes
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct Size(pub u64);

impl Size {
    /// Creates a constant [`Size`] from a sized type.
    #[inline]
    #[must_use]
    pub const fn of<T: Sized>() -> Self {
        Self(size_of::<T>() as _)
    }

    /// Gets the number of bytes
    #[inline]
    pub const fn bytes(&self) -> u64 {
        self.0
    }

    /// Gets the number of kilo bytes
    #[inline]
    pub const fn kilobytes(&self) -> u64 {
        self.0 / 1000
    }

    /// Gets the number of kibi bytes
    #[inline]
    pub const fn kibibytes(&self) -> u64 {
        self.0 >> 10
    }

    /// Gets the number of mega bytes
    #[inline]
    pub const fn megabytes(&self) -> u64 {
        self.0 / 1_000_000
    }

    /// Gets the number of mibi bytes
    #[inline]
    pub const fn mibibytes(&self) -> u64 {
        self.0 >> 20
    }

    /// Gets the number of giga bytes
    #[inline]
    pub const fn gigabytes(&self) -> u64 {
        self.0 / 1_000_000
    }

    /// Gets the number of gibi bytes
    #[inline]
    pub const fn gibibytes(&self) -> u64 {
        self.0 >> 30
    }
}

impl Mul<usize> for Size {
    type Output = Size;

    fn mul(self, rhs: usize) -> Self::Output {
        Self(self.0 * rhs as u64)
    }
}

impl MulAssign<usize> for Size {
    fn mul_assign(&mut self, rhs: usize) {
        self.0 *= rhs as u64;
    }
}

impl Mul<u32> for Size {
    type Output = Size;

    fn mul(self, rhs: u32) -> Self::Output {
        Self(self.0 * rhs as u64)
    }
}

impl MulAssign<u32> for Size {
    fn mul_assign(&mut self, rhs: u32) {
        self.0 *= rhs as u64;
    }
}

impl Mul<u64> for Size {
    type Output = Size;

    fn mul(self, rhs: u64) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl MulAssign<u64> for Size {
    fn mul_assign(&mut self, rhs: u64) {
        self.0 *= rhs;
    }
}

impl Div<usize> for Size {
    type Output = Size;

    fn div(self, rhs: usize) -> Self::Output {
        Self(self.0 / rhs as u64)
    }
}

impl DivAssign<usize> for Size {
    fn div_assign(&mut self, rhs: usize) {
        self.0 /= rhs as u64;
    }
}

impl Div<u32> for Size {
    type Output = Size;

    fn div(self, rhs: u32) -> Self::Output {
        Self(self.0 / rhs as u64)
    }
}

impl DivAssign<u32> for Size {
    fn div_assign(&mut self, rhs: u32) {
        self.0 /= rhs as u64;
    }
}

impl Div<u64> for Size {
    type Output = Size;

    fn div(self, rhs: u64) -> Self::Output {
        Self(self.0 / rhs)
    }
}

impl DivAssign<u64> for Size {
    fn div_assign(&mut self, rhs: u64) {
        self.0 /= rhs;
    }
}

impl Add for Size {
    type Output = Size;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for Size {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

impl Sub for Size {
    type Output = Size;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl SubAssign for Size {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl Debug for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (divider, unit) = bytes_of(self.0 as u64);

        let precision = f.precision().unwrap_or(1);

        f.debug_tuple("Size")
            .field(&format!("{:.*} {}", precision, self.0 as f64 / divider as f64, unit))
            .finish()
    }
}

impl Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (divider, unit) = bytes_of(self.0 as u64);

        let precision = f.precision().unwrap_or(1);

        f.write_str(&format!("{:.*} {}", precision, self.0 as f64 / divider as f64, unit))
    }
}

impl const From<usize> for Size {
    #[inline]
    fn from(size: usize) -> Self {
        Self(size as _)
    }
}

impl const Into<usize> for Size {
    #[inline]
    fn into(self) -> usize {
        self.0 as _
    }
}

impl const From<u64> for Size {
    #[inline]
    fn from(size: u64) -> Self {
        Self(size)
    }
}

impl const Into<u64> for Size {
    #[inline]
    fn into(self) -> u64 {
        self.0
    }
}

impl const From<u32> for Size {
    #[inline]
    fn from(size: u32) -> Self {
        Self(size as _)
    }
}

impl const Into<u32> for Size {
    #[inline]
    fn into(self) -> u32 {
        self.0 as _
    }
}

/// Memory size helper.
/// Returns a divider and a static string denoting the unit (B, KiB, etc.)
#[must_use]
#[doc(hidden)]
#[inline]
pub const fn bytes_of(value: u64) -> (u64, &'static str) {
    if num_bits::<u64>() as u32 - value.leading_zeros() == 0 {
        (1 << 60, "EiB")
    } else {
        match num_bits::<u64>() as u32 - value.leading_zeros() - 1 {
            0..=9 => (1, "B"),
            10..=19 => (1 << 10, "KiB"),
            20..=29 => (1 << 20, "MiB"),
            30..=39 => (1 << 30, "GiB"),
            40..=49 => (1 << 40, "TiB"),
            50..=59 => (1 << 50, "PiB"),
            _ => (1 << 60, "EiB"),
        }
    }
}

/// Computes the number of bits in a number
#[must_use]
#[doc(hidden)]
#[inline]
pub const fn num_bits<T>() -> usize {
    size_of::<T>() * 8
}
