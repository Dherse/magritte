//!# [VK_EXT_debug_report](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_debug_report.html)
# ! [doc = include_str ! ("../../../../doc/extensions/ext_debug_report/VK_EXT_debug_report.md")]
#[cfg(feature = "VK_EXT_debug_marker")]
use crate::extensions::ext_debug_marker::DebugReportObjectTypeEXT;
use crate::{
    cstr,
    vulkan1_0::{AllocationCallbacks, BaseInStructure, Bool32, Instance, StructureType, VulkanResultCodes},
};
use std::ffi::CStr;
///# [VkDebugReportCallbackCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDebugReportCallbackCreateInfoEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_debug_report/VkDebugReportCallbackCreateInfoEXT.md")]
#[doc(alias = "VkDebugReportCallbackCreateInfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DebugReportCallbackCreateInfoEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    flags: DebugReportFlagsEXT,
    #[doc(alias = "pfnCallback")]
    pfn_callback: PFNDebugReportCallbackEXT,
    #[doc(alias = "pUserData")]
    user_data: *mut std::ffi::c_void,
}
///# [VkDebugReportCallbackEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDebugReportCallbackEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_debug_report/VkDebugReportCallbackEXT.md")]
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(alias = "VkDebugReportCallbackEXT")]
#[repr(transparent)]
pub struct DebugReportCallbackEXT(u64);
impl DebugReportCallbackEXT {
    pub const fn null() -> Self {
        Self(0)
    }
}
impl const Default for DebugReportCallbackEXT {
    fn default() -> Self {
        Self::null()
    }
}
///# [PFN_vkDebugReportCallbackEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/PFN_vkDebugReportCallbackEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_debug_report/PFN_vkDebugReportCallbackEXT.md")]
#[doc(alias = "PFN_vkDebugReportCallbackEXT")]
pub type PFNDebugReportCallbackEXT = unsafe extern "system" fn(
    flags: DebugReportFlagsEXT,
    object_type: DebugReportObjectTypeEXT,
    object: u64,
    location: usize,
    message_code: i32,
    p_layer_prefix: *const std::ffi::c_char,
    p_message: *const std::ffi::c_char,
    p_user_data: *mut std::ffi::c_void,
) -> Bool32;
///# [VkDebugReportFlagBitsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDebugReportFlagBitsEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_debug_report/VkDebugReportFlagBitsEXT.md")]
#[doc(alias = "VkDebugReportFlagsEXT")]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DebugReportFlagsEXT(u32);
impl DebugReportFlagsEXT {
    #[doc(alias = "VK_DEBUG_REPORT_INFORMATION_BIT_EXT")]
    pub const INFORMATION: Self = Self(1);
    #[doc(alias = "VK_DEBUG_REPORT_WARNING_BIT_EXT")]
    pub const WARNING: Self = Self(2);
    #[doc(alias = "VK_DEBUG_REPORT_PERFORMANCE_WARNING_BIT_EXT")]
    pub const PERFORMANCE_WARNING: Self = Self(4);
    #[doc(alias = "VK_DEBUG_REPORT_ERROR_BIT_EXT")]
    pub const ERROR: Self = Self(8);
    #[doc(alias = "VK_DEBUG_REPORT_DEBUG_BIT_EXT")]
    pub const DEBUG: Self = Self(16);
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
            all |= Self::INFORMATION;
        }
        {
            all |= Self::WARNING;
        }
        {
            all |= Self::PERFORMANCE_WARNING;
        }
        {
            all |= Self::ERROR;
        }
        {
            all |= Self::DEBUG;
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
impl const std::ops::BitOr for DebugReportFlagsEXT {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl const std::ops::BitOrAssign for DebugReportFlagsEXT {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for DebugReportFlagsEXT {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl const std::ops::BitXorAssign for DebugReportFlagsEXT {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for DebugReportFlagsEXT {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl const std::ops::BitAndAssign for DebugReportFlagsEXT {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for DebugReportFlagsEXT {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl const std::ops::SubAssign for DebugReportFlagsEXT {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for DebugReportFlagsEXT {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<DebugReportFlagsEXT> for DebugReportFlagsEXT {
    fn extend<T: IntoIterator<Item = DebugReportFlagsEXT>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl FromIterator<DebugReportFlagsEXT> for DebugReportFlagsEXT {
    fn from_iter<T: IntoIterator<Item = DebugReportFlagsEXT>>(iterator: T) -> DebugReportFlagsEXT {
        let mut out = Self::empty();
        <Self as Extend<DebugReportFlagsEXT>>::extend(&mut out, iterator);
        out
    }
}
impl const Default for DebugReportFlagsEXT {
    fn default() -> Self {
        Self::empty()
    }
}
impl const From<DebugReportFlagBitsEXT> for DebugReportFlagsEXT {
    fn from(bit: DebugReportFlagBitsEXT) -> Self {
        unsafe { Self::from_bits_unchecked(bit.bits()) }
    }
}
impl Extend<DebugReportFlagBitsEXT> for DebugReportFlagsEXT {
    fn extend<T: IntoIterator<Item = DebugReportFlagBitsEXT>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, Self::from(i));
        }
    }
}
impl FromIterator<DebugReportFlagBitsEXT> for DebugReportFlagsEXT {
    fn from_iter<T: IntoIterator<Item = DebugReportFlagBitsEXT>>(iterator: T) -> DebugReportFlagsEXT {
        let mut out = Self::empty();
        <Self as Extend<DebugReportFlagBitsEXT>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for DebugReportFlagsEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(DebugReportFlagsEXT);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == DebugReportFlagsEXT::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(DebugReportFlagsEXT::INFORMATION) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(INFORMATION))?;
                    }
                    if self.0.contains(DebugReportFlagsEXT::WARNING) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(WARNING))?;
                    }
                    if self.0.contains(DebugReportFlagsEXT::PERFORMANCE_WARNING) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PERFORMANCE_WARNING))?;
                    }
                    if self.0.contains(DebugReportFlagsEXT::ERROR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(ERROR))?;
                    }
                    if self.0.contains(DebugReportFlagsEXT::DEBUG) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(DEBUG))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(DebugReportFlagsEXT))
            .field(&Flags(*self))
            .finish()
    }
}
#[doc(alias = "VK_EXT_DEBUG_REPORT_SPEC_VERSION")]
pub const EXT_DEBUG_REPORT_SPEC_VERSION: u32 = 10;
#[doc(alias = "VK_EXT_DEBUG_REPORT_EXTENSION_NAME")]
pub const EXT_DEBUG_REPORT_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_debug_report");
///# [VkDebugReportFlagBitsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDebugReportFlagBitsEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_debug_report/VkDebugReportFlagBitsEXT.md")]
#[doc(alias = "VkDebugReportFlagBitsEXT")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct DebugReportFlagBitsEXT(u32);
impl DebugReportFlagBitsEXT {
    #[doc(alias = "VK_DEBUG_REPORT_INFORMATION_BIT_EXT")]
    pub const INFORMATION: Self = Self(1);
    #[doc(alias = "VK_DEBUG_REPORT_WARNING_BIT_EXT")]
    pub const WARNING: Self = Self(2);
    #[doc(alias = "VK_DEBUG_REPORT_PERFORMANCE_WARNING_BIT_EXT")]
    pub const PERFORMANCE_WARNING: Self = Self(4);
    #[doc(alias = "VK_DEBUG_REPORT_ERROR_BIT_EXT")]
    pub const ERROR: Self = Self(8);
    #[doc(alias = "VK_DEBUG_REPORT_DEBUG_BIT_EXT")]
    pub const DEBUG: Self = Self(16);
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
            x if x == Self::INFORMATION.bits() => Some(Self(x)),
            x if x == Self::WARNING.bits() => Some(Self(x)),
            x if x == Self::PERFORMANCE_WARNING.bits() => Some(Self(x)),
            x if x == Self::ERROR.bits() => Some(Self(x)),
            x if x == Self::DEBUG.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
}
///# [vkCreateDebugReportCallbackEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateDebugReportCallbackEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_debug_report/vkCreateDebugReportCallbackEXT.md")]
#[doc(alias = "vkCreateDebugReportCallbackEXT")]
pub type FNCreateDebugReportCallbackExt = unsafe extern "system" fn(
    instance: Instance,
    p_create_info: *const DebugReportCallbackCreateInfoEXT,
    p_allocator: *const AllocationCallbacks,
    p_callback: *mut DebugReportCallbackEXT,
) -> VulkanResultCodes;
///# [vkDestroyDebugReportCallbackEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyDebugReportCallbackEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_debug_report/vkDestroyDebugReportCallbackEXT.md")]
#[doc(alias = "vkDestroyDebugReportCallbackEXT")]
pub type FNDestroyDebugReportCallbackExt = unsafe extern "system" fn(
    instance: Instance,
    callback: DebugReportCallbackEXT,
    p_allocator: *const AllocationCallbacks,
);
///# [vkDebugReportMessageEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDebugReportMessageEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_debug_report/vkDebugReportMessageEXT.md")]
#[doc(alias = "vkDebugReportMessageEXT")]
pub type FNDebugReportMessageExt = unsafe extern "system" fn(
    instance: Instance,
    flags: DebugReportFlagsEXT,
    object_type: DebugReportObjectTypeEXT,
    object: u64,
    location: usize,
    message_code: i32,
    p_layer_prefix: *const CStr,
    p_message: *const CStr,
);
