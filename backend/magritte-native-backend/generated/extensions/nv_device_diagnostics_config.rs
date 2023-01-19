//!# [VK_NV_device_diagnostics_config](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_NV_device_diagnostics_config.html)
# ! [doc = include_str ! ("../../../../doc/extensions/nv_device_diagnostics_config/VK_NV_device_diagnostics_config.md")]
use crate::{
    cstr,
    vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, StructureType},
};
use std::ffi::CStr;
///# [VkPhysicalDeviceDiagnosticsConfigFeaturesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDiagnosticsConfigFeaturesNV.html)
# [doc = include_str ! ("../../../../doc/extensions/nv_device_diagnostics_config/VkPhysicalDeviceDiagnosticsConfigFeaturesNV.md")]
#[doc(alias = "VkPhysicalDeviceDiagnosticsConfigFeaturesNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceDiagnosticsConfigFeaturesNV {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "diagnosticsConfig")]
    diagnostics_config: Bool32,
}
///# [VkDeviceDiagnosticsConfigCreateInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceDiagnosticsConfigCreateInfoNV.html)
# [doc = include_str ! ("../../../../doc/extensions/nv_device_diagnostics_config/VkDeviceDiagnosticsConfigCreateInfoNV.md")]
#[doc(alias = "VkDeviceDiagnosticsConfigCreateInfoNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DeviceDiagnosticsConfigCreateInfoNV {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    flags: DeviceDiagnosticsConfigFlagsNV,
}
///# [VkDeviceDiagnosticsConfigFlagBitsNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceDiagnosticsConfigFlagBitsNV.html)
# [doc = include_str ! ("../../../../doc/extensions/nv_device_diagnostics_config/VkDeviceDiagnosticsConfigFlagBitsNV.md")]
#[doc(alias = "VkDeviceDiagnosticsConfigFlagsNV")]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DeviceDiagnosticsConfigFlagsNV(u32);
impl DeviceDiagnosticsConfigFlagsNV {
    #[doc(alias = "VK_DEVICE_DIAGNOSTICS_CONFIG_ENABLE_SHADER_DEBUG_INFO_BIT_NV")]
    pub const ENABLE_SHADER_DEBUG_INFO: Self = Self(1);
    #[doc(alias = "VK_DEVICE_DIAGNOSTICS_CONFIG_ENABLE_RESOURCE_TRACKING_BIT_NV")]
    pub const ENABLE_RESOURCE_TRACKING: Self = Self(2);
    #[doc(alias = "VK_DEVICE_DIAGNOSTICS_CONFIG_ENABLE_AUTOMATIC_CHECKPOINTS_BIT_NV")]
    pub const ENABLE_AUTOMATIC_CHECKPOINTS: Self = Self(4);
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
        {
            all |= Self::ENABLE_SHADER_DEBUG_INFO;
        }
        {
            all |= Self::ENABLE_RESOURCE_TRACKING;
        }
        {
            all |= Self::ENABLE_AUTOMATIC_CHECKPOINTS;
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
impl const std::ops::BitOr for DeviceDiagnosticsConfigFlagsNV {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl const std::ops::BitOrAssign for DeviceDiagnosticsConfigFlagsNV {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for DeviceDiagnosticsConfigFlagsNV {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl const std::ops::BitXorAssign for DeviceDiagnosticsConfigFlagsNV {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for DeviceDiagnosticsConfigFlagsNV {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl const std::ops::BitAndAssign for DeviceDiagnosticsConfigFlagsNV {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for DeviceDiagnosticsConfigFlagsNV {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl const std::ops::SubAssign for DeviceDiagnosticsConfigFlagsNV {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for DeviceDiagnosticsConfigFlagsNV {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<DeviceDiagnosticsConfigFlagsNV> for DeviceDiagnosticsConfigFlagsNV {
    fn extend<T: IntoIterator<Item = DeviceDiagnosticsConfigFlagsNV>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl FromIterator<DeviceDiagnosticsConfigFlagsNV> for DeviceDiagnosticsConfigFlagsNV {
    fn from_iter<T: IntoIterator<Item = DeviceDiagnosticsConfigFlagsNV>>(
        iterator: T,
    ) -> DeviceDiagnosticsConfigFlagsNV {
        let mut out = Self::empty();
        <Self as Extend<DeviceDiagnosticsConfigFlagsNV>>::extend(&mut out, iterator);
        out
    }
}
impl const Default for DeviceDiagnosticsConfigFlagsNV {
    fn default() -> Self {
        Self::empty()
    }
}
impl const From<DeviceDiagnosticsConfigFlagBitsNV> for DeviceDiagnosticsConfigFlagsNV {
    fn from(bit: DeviceDiagnosticsConfigFlagBitsNV) -> Self {
        unsafe { Self::from_bits_unchecked(bit.bits()) }
    }
}
impl Extend<DeviceDiagnosticsConfigFlagBitsNV> for DeviceDiagnosticsConfigFlagsNV {
    fn extend<T: IntoIterator<Item = DeviceDiagnosticsConfigFlagBitsNV>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, Self::from(i));
        }
    }
}
impl FromIterator<DeviceDiagnosticsConfigFlagBitsNV> for DeviceDiagnosticsConfigFlagsNV {
    fn from_iter<T: IntoIterator<Item = DeviceDiagnosticsConfigFlagBitsNV>>(
        iterator: T,
    ) -> DeviceDiagnosticsConfigFlagsNV {
        let mut out = Self::empty();
        <Self as Extend<DeviceDiagnosticsConfigFlagBitsNV>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for DeviceDiagnosticsConfigFlagsNV {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(DeviceDiagnosticsConfigFlagsNV);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == DeviceDiagnosticsConfigFlagsNV::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self
                        .0
                        .contains(DeviceDiagnosticsConfigFlagsNV::ENABLE_SHADER_DEBUG_INFO)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(ENABLE_SHADER_DEBUG_INFO))?;
                    }
                    if self
                        .0
                        .contains(DeviceDiagnosticsConfigFlagsNV::ENABLE_RESOURCE_TRACKING)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(ENABLE_RESOURCE_TRACKING))?;
                    }
                    if self
                        .0
                        .contains(DeviceDiagnosticsConfigFlagsNV::ENABLE_AUTOMATIC_CHECKPOINTS)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(ENABLE_AUTOMATIC_CHECKPOINTS))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(DeviceDiagnosticsConfigFlagsNV))
            .field(&Flags(*self))
            .finish()
    }
}
#[doc(alias = "VK_NV_DEVICE_DIAGNOSTICS_CONFIG_SPEC_VERSION")]
pub const NV_DEVICE_DIAGNOSTICS_CONFIG_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_NV_DEVICE_DIAGNOSTICS_CONFIG_EXTENSION_NAME")]
pub const NV_DEVICE_DIAGNOSTICS_CONFIG_EXTENSION_NAME: &'static CStr = cstr!("VK_NV_device_diagnostics_config");
///# [VkDeviceDiagnosticsConfigFlagBitsNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceDiagnosticsConfigFlagBitsNV.html)
# [doc = include_str ! ("../../../../doc/extensions/nv_device_diagnostics_config/VkDeviceDiagnosticsConfigFlagBitsNV.md")]
#[doc(alias = "VkDeviceDiagnosticsConfigFlagBitsNV")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct DeviceDiagnosticsConfigFlagBitsNV(u32);
impl DeviceDiagnosticsConfigFlagBitsNV {
    #[doc(alias = "VK_DEVICE_DIAGNOSTICS_CONFIG_ENABLE_SHADER_DEBUG_INFO_BIT_NV")]
    pub const ENABLE_SHADER_DEBUG_INFO: Self = Self(1);
    #[doc(alias = "VK_DEVICE_DIAGNOSTICS_CONFIG_ENABLE_RESOURCE_TRACKING_BIT_NV")]
    pub const ENABLE_RESOURCE_TRACKING: Self = Self(2);
    #[doc(alias = "VK_DEVICE_DIAGNOSTICS_CONFIG_ENABLE_AUTOMATIC_CHECKPOINTS_BIT_NV")]
    pub const ENABLE_AUTOMATIC_CHECKPOINTS: Self = Self(4);
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
            x if x == Self::ENABLE_SHADER_DEBUG_INFO.bits() => Some(Self(x)),
            x if x == Self::ENABLE_RESOURCE_TRACKING.bits() => Some(Self(x)),
            x if x == Self::ENABLE_AUTOMATIC_CHECKPOINTS.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
}
