use crate::cstr;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ffi::CStr;
#[doc(alias = "VkSwapchainCreateFlagsKHR")]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SwapchainCreateFlagsKHR(u32);
impl SwapchainCreateFlagsKHR {
    #[doc(alias = "VK_SWAPCHAIN_CREATE_SPLIT_INSTANCE_BIND_REGIONS_BIT_KHR")]
    #[cfg(feature = "VK_KHR_device_group")]
    pub const SPLIT_INSTANCE_BIND_REGIONS: Self = Self(1);
    #[doc(alias = "VK_SWAPCHAIN_CREATE_PROTECTED_BIT_KHR")]
    pub const PROTECTED: Self = Self(2);
    #[doc(alias = "VK_SWAPCHAIN_CREATE_MUTABLE_FORMAT_BIT_KHR")]
    #[cfg(feature = "VK_KHR_swapchain_mutable_format")]
    pub const MUTABLE_FORMAT: Self = Self(4);
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
    ///Returns a value with all of the flags enabled
    #[inline]
    #[allow(unused_mut)]
    pub const fn all() -> Self {
        let mut all = Self::empty();
        #[cfg(feature = "VK_KHR_device_group")]
        {
            all |= Self::SPLIT_INSTANCE_BIND_REGIONS;
        }
        {
            all |= Self::PROTECTED;
        }
        #[cfg(feature = "VK_KHR_swapchain_mutable_format")]
        {
            all |= Self::MUTABLE_FORMAT;
        }
        all
    }
    ///Returns the raw bits
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0
    }
    ///Convert raw bits into a bit flags checking that only valid
    ///bits are contained.
    #[inline]
    pub const fn from_bits(bits: u32) -> Option<Self> {
        if (bits & !Self::all().bits()) == 0 {
            Some(Self(bits))
        } else {
            None
        }
    }
    ///Convert raw bits into a bit flags truncating all invalid
    ///bits that may be contained.
    #[inline]
    pub const fn from_bits_truncate(bits: u32) -> Self {
        Self(Self::all().0 & bits)
    }
    ///Convert raw bits into a bit preserving all bits
    ///
    ///# Safety
    ///The caller of this function must ensure that all of the bits are valid.
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
    ///Returns `true` if no flags are currently set
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.bits() == Self::empty().bits()
    }
    ///Returns `true` if all flags are currently set
    #[inline]
    pub const fn is_all(&self) -> bool {
        self.bits() == Self::all().bits()
    }
    ///Returns `true` if there are flags in common to `self` and `other`
    #[inline]
    pub const fn intersects(&self, other: Self) -> bool {
        !Self(self.bits() & other.bits()).is_empty()
    }
    ///Returns `true` if all of the flags in `other` are contained `self`
    #[inline]
    pub const fn contains(&self, other: Self) -> bool {
        (self.bits() & other.bits()) == other.bits()
    }
    ///Inserts a set of flags in place
    #[inline]
    pub fn insert(&mut self, other: Self) {
        self.0 |= other.bits()
    }
    ///Removes a set of flags in place
    #[inline]
    pub fn remove(&mut self, other: Self) {
        self.0 &= !other.bits();
    }
    ///Toggles a set of flags in place
    #[inline]
    pub fn toggle(&mut self, other: Self) {
        self.0 ^= other.bits();
    }
    ///Inserts or removes the specified flags depending on the value of `is_insert`
    #[inline]
    pub fn set(&mut self, other: Self, is_insert: bool) {
        if is_insert {
            self.insert(other);
        } else {
            self.remove(other);
        }
    }
    ///Returns the intersection between `self` and `other`
    #[inline]
    pub const fn intersection(self, other: Self) -> Self {
        Self(self.bits() & other.bits())
    }
    ///Returns the union between `self` and `other`
    #[inline]
    pub const fn union(self, other: Self) -> Self {
        Self(self.bits() | other.bits())
    }
    ///Returns the difference between `self` and `other`
    #[inline]
    pub const fn difference(self, other: Self) -> Self {
        Self(self.bits() & !other.bits())
    }
    ///Returns the [symmetric difference][sym-diff] between `self` and `other`
    ///
    ///[sym-diff]: https://en.wikipedia.org/wiki/Symmetric_difference
    #[inline]
    pub const fn symmetric_difference(self, other: Self) -> Self {
        Self(self.bits() ^ other.bits())
    }
    ///Returns the complement of `self`.
    #[inline]
    pub const fn complement(self) -> Self {
        Self::from_bits_truncate(!self.bits())
    }
}
impl std::ops::BitOr for SwapchainCreateFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl std::ops::BitOrAssign for SwapchainCreateFlagsKHR {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl std::ops::BitXor for SwapchainCreateFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl std::ops::BitXorAssign for SwapchainCreateFlagsKHR {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl std::ops::BitAnd for SwapchainCreateFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl std::ops::BitAndAssign for SwapchainCreateFlagsKHR {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl std::ops::Sub for SwapchainCreateFlagsKHR {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl std::ops::SubAssign for SwapchainCreateFlagsKHR {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl std::ops::Not for SwapchainCreateFlagsKHR {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<SwapchainCreateFlagsKHR> for SwapchainCreateFlagsKHR {
    fn extend<T: IntoIterator<Item = SwapchainCreateFlagsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl FromIterator<SwapchainCreateFlagsKHR> for SwapchainCreateFlagsKHR {
    fn from_iter<T: IntoIterator<Item = SwapchainCreateFlagsKHR>>(iterator: T) -> SwapchainCreateFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<SwapchainCreateFlagsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl Default for SwapchainCreateFlagsKHR {
    fn default() -> Self {
        Self::empty()
    }
}
impl From<SwapchainCreateFlagBitsKHR> for SwapchainCreateFlagsKHR {
    fn from(bit: SwapchainCreateFlagBitsKHR) -> Self {
        unsafe { Self::from_bits_unchecked(bit.bits()) }
    }
}
impl Extend<SwapchainCreateFlagBitsKHR> for SwapchainCreateFlagsKHR {
    fn extend<T: IntoIterator<Item = SwapchainCreateFlagBitsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, Self::from(i));
        }
    }
}
impl FromIterator<SwapchainCreateFlagBitsKHR> for SwapchainCreateFlagsKHR {
    fn from_iter<T: IntoIterator<Item = SwapchainCreateFlagBitsKHR>>(iterator: T) -> SwapchainCreateFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<SwapchainCreateFlagBitsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for SwapchainCreateFlagsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(SwapchainCreateFlagsKHR);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == SwapchainCreateFlagsKHR::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    #[cfg(feature = "VK_KHR_device_group")]
                    if self.0.contains(SwapchainCreateFlagsKHR::SPLIT_INSTANCE_BIND_REGIONS) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(SPLIT_INSTANCE_BIND_REGIONS))?;
                    }
                    if self.0.contains(SwapchainCreateFlagsKHR::PROTECTED) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PROTECTED))?;
                    }
                    #[cfg(feature = "VK_KHR_swapchain_mutable_format")]
                    if self.0.contains(SwapchainCreateFlagsKHR::MUTABLE_FORMAT) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(MUTABLE_FORMAT))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(SwapchainCreateFlagsKHR))
            .field(&Flags(*self))
            .finish()
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for SwapchainCreateFlagsKHR {
    type LowLevel = Self;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        *self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for SwapchainCreateFlagsKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VK_KHR_SWAPCHAIN_SPEC_VERSION")]
pub const KHR_SWAPCHAIN_SPEC_VERSION: u32 = 70;
#[doc(alias = "VK_KHR_SWAPCHAIN_EXTENSION_NAME")]
pub const KHR_SWAPCHAIN_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_swapchain");
#[doc(alias = "VkSwapchainCreateFlagBitsKHR")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct SwapchainCreateFlagBitsKHR(u32);
impl Default for SwapchainCreateFlagBitsKHR {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
impl SwapchainCreateFlagBitsKHR {
    #[doc(alias = "VK_SWAPCHAIN_CREATE_SPLIT_INSTANCE_BIND_REGIONS_BIT_KHR")]
    #[cfg(feature = "VK_KHR_device_group")]
    pub const SPLIT_INSTANCE_BIND_REGIONS: Self = Self(1);
    #[doc(alias = "VK_SWAPCHAIN_CREATE_PROTECTED_BIT_KHR")]
    pub const PROTECTED: Self = Self(2);
    #[doc(alias = "VK_SWAPCHAIN_CREATE_MUTABLE_FORMAT_BIT_KHR")]
    #[cfg(feature = "VK_KHR_swapchain_mutable_format")]
    pub const MUTABLE_FORMAT: Self = Self(4);
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
    ///The bits of this variant
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0
    }
    ///Builds a bitmask from the bits of this variant
    #[inline]
    pub const fn from_bits(bits: u32) -> Option<Self> {
        match bits {
            #[cfg(feature = "VK_KHR_device_group")]
            x if x == Self::SPLIT_INSTANCE_BIND_REGIONS.bits() => Some(Self(x)),
            x if x == Self::PROTECTED.bits() => Some(Self(x)),
            #[cfg(feature = "VK_KHR_swapchain_mutable_format")]
            x if x == Self::MUTABLE_FORMAT.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for SwapchainCreateFlagBitsKHR {
    type LowLevel = Self;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        *self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for SwapchainCreateFlagBitsKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
