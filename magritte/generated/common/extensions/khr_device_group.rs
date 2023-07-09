use crate::cstr;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ffi::CStr;
#[doc(alias = "VkDeviceGroupPresentModeFlagsKHR")]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DeviceGroupPresentModeFlagsKHR(u32);
impl DeviceGroupPresentModeFlagsKHR {
    #[doc(alias = "VK_DEVICE_GROUP_PRESENT_MODE_LOCAL_BIT_KHR")]
    #[cfg(feature = "VK_KHR_swapchain")]
    pub const LOCAL: Self = Self(1);
    #[doc(alias = "VK_DEVICE_GROUP_PRESENT_MODE_REMOTE_BIT_KHR")]
    #[cfg(feature = "VK_KHR_swapchain")]
    pub const REMOTE: Self = Self(2);
    #[doc(alias = "VK_DEVICE_GROUP_PRESENT_MODE_SUM_BIT_KHR")]
    #[cfg(feature = "VK_KHR_swapchain")]
    pub const SUM: Self = Self(4);
    #[doc(alias = "VK_DEVICE_GROUP_PRESENT_MODE_LOCAL_MULTI_DEVICE_BIT_KHR")]
    #[cfg(feature = "VK_KHR_swapchain")]
    pub const LOCAL_MULTI_DEVICE: Self = Self(8);
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
        #[cfg(feature = "VK_KHR_swapchain")]
        {
            all |= Self::LOCAL;
        }
        #[cfg(feature = "VK_KHR_swapchain")]
        {
            all |= Self::REMOTE;
        }
        #[cfg(feature = "VK_KHR_swapchain")]
        {
            all |= Self::SUM;
        }
        #[cfg(feature = "VK_KHR_swapchain")]
        {
            all |= Self::LOCAL_MULTI_DEVICE;
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
impl std::ops::BitOr for DeviceGroupPresentModeFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl std::ops::BitOrAssign for DeviceGroupPresentModeFlagsKHR {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl std::ops::BitXor for DeviceGroupPresentModeFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl std::ops::BitXorAssign for DeviceGroupPresentModeFlagsKHR {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl std::ops::BitAnd for DeviceGroupPresentModeFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl std::ops::BitAndAssign for DeviceGroupPresentModeFlagsKHR {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl std::ops::Sub for DeviceGroupPresentModeFlagsKHR {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl std::ops::SubAssign for DeviceGroupPresentModeFlagsKHR {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl std::ops::Not for DeviceGroupPresentModeFlagsKHR {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<DeviceGroupPresentModeFlagsKHR> for DeviceGroupPresentModeFlagsKHR {
    fn extend<T: IntoIterator<Item = DeviceGroupPresentModeFlagsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl FromIterator<DeviceGroupPresentModeFlagsKHR> for DeviceGroupPresentModeFlagsKHR {
    fn from_iter<T: IntoIterator<Item = DeviceGroupPresentModeFlagsKHR>>(
        iterator: T,
    ) -> DeviceGroupPresentModeFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<DeviceGroupPresentModeFlagsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl Default for DeviceGroupPresentModeFlagsKHR {
    fn default() -> Self {
        Self::empty()
    }
}
impl From<DeviceGroupPresentModeFlagBitsKHR> for DeviceGroupPresentModeFlagsKHR {
    fn from(bit: DeviceGroupPresentModeFlagBitsKHR) -> Self {
        unsafe { Self::from_bits_unchecked(bit.bits()) }
    }
}
impl Extend<DeviceGroupPresentModeFlagBitsKHR> for DeviceGroupPresentModeFlagsKHR {
    fn extend<T: IntoIterator<Item = DeviceGroupPresentModeFlagBitsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, Self::from(i));
        }
    }
}
impl FromIterator<DeviceGroupPresentModeFlagBitsKHR> for DeviceGroupPresentModeFlagsKHR {
    fn from_iter<T: IntoIterator<Item = DeviceGroupPresentModeFlagBitsKHR>>(
        iterator: T,
    ) -> DeviceGroupPresentModeFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<DeviceGroupPresentModeFlagBitsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for DeviceGroupPresentModeFlagsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(DeviceGroupPresentModeFlagsKHR);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == DeviceGroupPresentModeFlagsKHR::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    #[cfg(feature = "VK_KHR_swapchain")]
                    if self.0.contains(DeviceGroupPresentModeFlagsKHR::LOCAL) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(LOCAL))?;
                    }
                    #[cfg(feature = "VK_KHR_swapchain")]
                    if self.0.contains(DeviceGroupPresentModeFlagsKHR::REMOTE) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(REMOTE))?;
                    }
                    #[cfg(feature = "VK_KHR_swapchain")]
                    if self.0.contains(DeviceGroupPresentModeFlagsKHR::SUM) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(SUM))?;
                    }
                    #[cfg(feature = "VK_KHR_swapchain")]
                    if self.0.contains(DeviceGroupPresentModeFlagsKHR::LOCAL_MULTI_DEVICE) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(LOCAL_MULTI_DEVICE))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(DeviceGroupPresentModeFlagsKHR))
            .field(&Flags(*self))
            .finish()
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for DeviceGroupPresentModeFlagsKHR {
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
unsafe impl crate::conv::FromLowLevel for DeviceGroupPresentModeFlagsKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VK_KHR_DEVICE_GROUP_SPEC_VERSION")]
pub const KHR_DEVICE_GROUP_SPEC_VERSION: u32 = 4;
#[doc(alias = "VK_KHR_DEVICE_GROUP_EXTENSION_NAME")]
pub const KHR_DEVICE_GROUP_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_device_group");
#[doc(alias = "VkDeviceGroupPresentModeFlagBitsKHR")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct DeviceGroupPresentModeFlagBitsKHR(u32);
impl Default for DeviceGroupPresentModeFlagBitsKHR {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
impl DeviceGroupPresentModeFlagBitsKHR {
    #[doc(alias = "VK_DEVICE_GROUP_PRESENT_MODE_LOCAL_BIT_KHR")]
    #[cfg(feature = "VK_KHR_swapchain")]
    pub const LOCAL: Self = Self(1);
    #[doc(alias = "VK_DEVICE_GROUP_PRESENT_MODE_REMOTE_BIT_KHR")]
    #[cfg(feature = "VK_KHR_swapchain")]
    pub const REMOTE: Self = Self(2);
    #[doc(alias = "VK_DEVICE_GROUP_PRESENT_MODE_SUM_BIT_KHR")]
    #[cfg(feature = "VK_KHR_swapchain")]
    pub const SUM: Self = Self(4);
    #[doc(alias = "VK_DEVICE_GROUP_PRESENT_MODE_LOCAL_MULTI_DEVICE_BIT_KHR")]
    #[cfg(feature = "VK_KHR_swapchain")]
    pub const LOCAL_MULTI_DEVICE: Self = Self(8);
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
            #[cfg(feature = "VK_KHR_swapchain")]
            x if x == Self::LOCAL.bits() => Some(Self(x)),
            #[cfg(feature = "VK_KHR_swapchain")]
            x if x == Self::REMOTE.bits() => Some(Self(x)),
            #[cfg(feature = "VK_KHR_swapchain")]
            x if x == Self::SUM.bits() => Some(Self(x)),
            #[cfg(feature = "VK_KHR_swapchain")]
            x if x == Self::LOCAL_MULTI_DEVICE.bits() => Some(Self(x)),
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
unsafe impl crate::conv::IntoLowLevel for DeviceGroupPresentModeFlagBitsKHR {
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
unsafe impl crate::conv::FromLowLevel for DeviceGroupPresentModeFlagBitsKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
