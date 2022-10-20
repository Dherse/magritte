//! # Version
//! Utilities for pretty-printing Vulkan versions.

use std::fmt::{Debug, Display};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};

/// Utilities for pretty printing Vulkan versions
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(transparent)]
pub struct Version(pub u32);

impl const Default for Version {
    fn default() -> Self {
        Self::VULKAN1_0
    }
}

impl PartialOrd<Self> for Version {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl Version {
    /// Vulkan 1.0
    pub const VULKAN1_0: Self = Self::from((1, 0, 0));

    /// Vulkan 1.1
    pub const VULKAN1_1: Self = Self::from((1, 1, 0));

    /// Vulkan 1.2
    pub const VULKAN1_2: Self = Self::from((1, 2, 0));

    /// Vulkan 1.3
    pub const VULKAN1_3: Self = Self::from((1, 3, 0));

    /// Creates a new Version from its components.
    #[must_use]
    #[inline]
    pub const fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self::with_variant(0, major, minor, patch)
    }

    /// Creates a new Version from its components and a variant.
    #[must_use]
    #[inline]
    pub const fn with_variant(variant: u32, major: u32, minor: u32, patch: u32) -> Self {
        Self((variant << 29) | (major << 22) | (minor << 12) | patch)
    }

    /// Gets the variant part of this version
    #[must_use]
    #[inline]
    pub const fn variant(&self) -> u32 {
        self.0 >> 29
    }

    /// Gets the major part of this version
    #[must_use]
    #[inline]
    pub const fn major(&self) -> u32 {
        (self.0 >> 22) & 0x7f
    }

    /// Gets the minor part of this version
    #[must_use]
    #[inline]
    pub const fn minor(&self) -> u32 {
        (self.0 >> 12) & 0x3ff
    }

    /// Gets the patch part of this version
    #[must_use]
    #[inline]
    pub const fn patch(&self) -> u32 {
        self.0 & 0xfff
    }
}

impl Debug for Version {
    #[allow(deprecated)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.variant() != 0 {
            f.debug_tuple("Version")
                .field(&format!(
                    "V{}-{}.{}.{}",
                    self.variant(),
                    self.major(),
                    self.minor(),
                    self.patch(),
                ))
                .finish()
        } else {
            f.debug_tuple("Version")
                .field(&format!("{}.{}.{}", self.major(), self.minor(), self.patch(),))
                .finish()
        }
    }
}

impl Display for Version {
    #[allow(deprecated)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.variant() != 0 {
            f.write_str(&format!(
                "V{}-{}.{}.{}",
                self.variant(),
                self.major(),
                self.minor(),
                self.patch(),
            ))
        } else {
            f.write_str(&format!("{}.{}.{}", self.major(), self.minor(), self.patch(),))
        }
    }
}

impl const From<u32> for Version {
    #[inline]
    fn from(i: u32) -> Self {
        Self(i)
    }
}

impl const Into<u32> for Version {
    #[inline]
    fn into(self) -> u32 {
        self.0
    }
}

impl const From<(u32, u32, u32)> for Version {
    #[inline]
    fn from((major, minor, patch): (u32, u32, u32)) -> Self {
        Self::new(major, minor, patch)
    }
}

impl const Into<(u32, u32, u32)> for Version {
    #[inline]
    fn into(self) -> (u32, u32, u32) {
        (self.major(), self.minor(), self.patch())
    }
}

impl const From<(u32, u32, u32, u32)> for Version {
    #[inline]
    fn from((variant, major, minor, patch): (u32, u32, u32, u32)) -> Self {
        Self::with_variant(variant, major, minor, patch)
    }
}

impl const Into<(u32, u32, u32, u32)> for Version {
    #[inline]
    fn into(self) -> (u32, u32, u32, u32) {
        (self.variant(), self.major(), self.minor(), self.patch())
    }
}
