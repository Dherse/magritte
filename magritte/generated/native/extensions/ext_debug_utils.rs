//!# [VK_EXT_debug_utils](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_debug_utils.html)
# ! [doc = include_str ! ("../../../../doc/extensions/ext_debug_utils/VK_EXT_debug_utils.md")]
use crate::{
    cstr,
    vulkan1_0::{
        AllocationCallbacks, BaseInStructure, Bool32, CommandBuffer, Device, Instance, ObjectType, Queue,
        StructureType, VulkanResultCodes,
    },
};
use std::ffi::{c_char, CStr};
///# [VkDebugUtilsObjectNameInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDebugUtilsObjectNameInfoEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_debug_utils/VkDebugUtilsObjectNameInfoEXT.md")]
#[doc(alias = "VkDebugUtilsObjectNameInfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DebugUtilsObjectNameInfoEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "objectType")]
    object_type: ObjectType,
    #[doc(alias = "objectHandle")]
    object_handle: u64,
    #[doc(alias = "pObjectName")]
    object_name: *const c_char,
}
///# [VkDebugUtilsObjectTagInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDebugUtilsObjectTagInfoEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_debug_utils/VkDebugUtilsObjectTagInfoEXT.md")]
#[doc(alias = "VkDebugUtilsObjectTagInfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DebugUtilsObjectTagInfoEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "objectType")]
    object_type: ObjectType,
    #[doc(alias = "objectHandle")]
    object_handle: u64,
    #[doc(alias = "tagName")]
    tag_name: u64,
    #[doc(alias = "tagSize")]
    tag_size: usize,
    #[doc(alias = "pTag")]
    tag: *const std::ffi::c_void,
}
///# [VkDebugUtilsLabelEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDebugUtilsLabelEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_debug_utils/VkDebugUtilsLabelEXT.md")]
#[doc(alias = "VkDebugUtilsLabelEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DebugUtilsLabelEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "pLabelName")]
    label_name: *const c_char,
    color: [f32; 4 as usize],
}
///# [VkDebugUtilsMessengerCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDebugUtilsMessengerCreateInfoEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_debug_utils/VkDebugUtilsMessengerCreateInfoEXT.md")]
#[doc(alias = "VkDebugUtilsMessengerCreateInfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DebugUtilsMessengerCreateInfoEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    flags: DebugUtilsMessengerCreateFlagsEXT,
    #[doc(alias = "messageSeverity")]
    message_severity: DebugUtilsMessageSeverityFlagsEXT,
    #[doc(alias = "messageType")]
    message_type: DebugUtilsMessageTypeFlagsEXT,
    #[doc(alias = "pfnUserCallback")]
    pfn_user_callback: PFNDebugUtilsMessengerCallbackEXT,
    #[doc(alias = "pUserData")]
    user_data: *mut std::ffi::c_void,
}
///# [VkDebugUtilsMessengerCallbackDataEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDebugUtilsMessengerCallbackDataEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_debug_utils/VkDebugUtilsMessengerCallbackDataEXT.md")]
#[doc(alias = "VkDebugUtilsMessengerCallbackDataEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DebugUtilsMessengerCallbackDataEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    flags: DebugUtilsMessengerCallbackDataFlagsEXT,
    #[doc(alias = "pMessageIdName")]
    message_id_name: *const c_char,
    #[doc(alias = "messageIdNumber")]
    message_id_number: i32,
    #[doc(alias = "pMessage")]
    message: *const c_char,
    #[doc(alias = "queueLabelCount")]
    queue_label_count: u32,
    #[doc(alias = "pQueueLabels")]
    queue_labels: *const DebugUtilsLabelEXT,
    #[doc(alias = "cmdBufLabelCount")]
    cmd_buf_label_count: u32,
    #[doc(alias = "pCmdBufLabels")]
    cmd_buf_labels: *const DebugUtilsLabelEXT,
    #[doc(alias = "objectCount")]
    object_count: u32,
    #[doc(alias = "pObjects")]
    objects: *const DebugUtilsObjectNameInfoEXT,
}
///# [VkDebugUtilsMessengerEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDebugUtilsMessengerEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_debug_utils/VkDebugUtilsMessengerEXT.md")]
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(alias = "VkDebugUtilsMessengerEXT")]
#[repr(transparent)]
pub struct DebugUtilsMessengerEXT(u64);
impl DebugUtilsMessengerEXT {
    pub const fn null() -> Self {
        Self(0)
    }
}
impl const Default for DebugUtilsMessengerEXT {
    fn default() -> Self {
        Self::null()
    }
}
///# [PFN_vkDebugUtilsMessengerCallbackEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/PFN_vkDebugUtilsMessengerCallbackEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_debug_utils/PFN_vkDebugUtilsMessengerCallbackEXT.md")]
#[doc(alias = "PFN_vkDebugUtilsMessengerCallbackEXT")]
pub type PFNDebugUtilsMessengerCallbackEXT = unsafe extern "system" fn(
    message_severity: DebugUtilsMessageSeverityFlagBitsEXT,
    message_types: DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const DebugUtilsMessengerCallbackDataEXT,
    p_user_data: *mut std::ffi::c_void,
) -> Bool32;
///# [VkDebugUtilsMessageSeverityFlagBitsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDebugUtilsMessageSeverityFlagBitsEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_debug_utils/VkDebugUtilsMessageSeverityFlagBitsEXT.md")]
#[doc(alias = "VkDebugUtilsMessageSeverityFlagsEXT")]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DebugUtilsMessageSeverityFlagsEXT(u32);
impl DebugUtilsMessageSeverityFlagsEXT {
    #[doc(alias = "VK_DEBUG_UTILS_MESSAGE_SEVERITY_VERBOSE_BIT_EXT")]
    pub const VERBOSE: Self = Self(1);
    #[doc(alias = "VK_DEBUG_UTILS_MESSAGE_SEVERITY_INFO_BIT_EXT")]
    pub const INFO: Self = Self(16);
    #[doc(alias = "VK_DEBUG_UTILS_MESSAGE_SEVERITY_WARNING_BIT_EXT")]
    pub const WARNING: Self = Self(256);
    #[doc(alias = "VK_DEBUG_UTILS_MESSAGE_SEVERITY_ERROR_BIT_EXT")]
    pub const ERROR: Self = Self(4096);
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
            all |= Self::VERBOSE;
        }
        {
            all |= Self::INFO;
        }
        {
            all |= Self::WARNING;
        }
        {
            all |= Self::ERROR;
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
impl const std::ops::BitOr for DebugUtilsMessageSeverityFlagsEXT {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl const std::ops::BitOrAssign for DebugUtilsMessageSeverityFlagsEXT {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for DebugUtilsMessageSeverityFlagsEXT {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl const std::ops::BitXorAssign for DebugUtilsMessageSeverityFlagsEXT {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for DebugUtilsMessageSeverityFlagsEXT {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl const std::ops::BitAndAssign for DebugUtilsMessageSeverityFlagsEXT {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for DebugUtilsMessageSeverityFlagsEXT {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl const std::ops::SubAssign for DebugUtilsMessageSeverityFlagsEXT {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for DebugUtilsMessageSeverityFlagsEXT {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<DebugUtilsMessageSeverityFlagsEXT> for DebugUtilsMessageSeverityFlagsEXT {
    fn extend<T: IntoIterator<Item = DebugUtilsMessageSeverityFlagsEXT>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl FromIterator<DebugUtilsMessageSeverityFlagsEXT> for DebugUtilsMessageSeverityFlagsEXT {
    fn from_iter<T: IntoIterator<Item = DebugUtilsMessageSeverityFlagsEXT>>(
        iterator: T,
    ) -> DebugUtilsMessageSeverityFlagsEXT {
        let mut out = Self::empty();
        <Self as Extend<DebugUtilsMessageSeverityFlagsEXT>>::extend(&mut out, iterator);
        out
    }
}
impl const Default for DebugUtilsMessageSeverityFlagsEXT {
    fn default() -> Self {
        Self::empty()
    }
}
impl const From<DebugUtilsMessageSeverityFlagBitsEXT> for DebugUtilsMessageSeverityFlagsEXT {
    fn from(bit: DebugUtilsMessageSeverityFlagBitsEXT) -> Self {
        unsafe { Self::from_bits_unchecked(bit.bits()) }
    }
}
impl Extend<DebugUtilsMessageSeverityFlagBitsEXT> for DebugUtilsMessageSeverityFlagsEXT {
    fn extend<T: IntoIterator<Item = DebugUtilsMessageSeverityFlagBitsEXT>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, Self::from(i));
        }
    }
}
impl FromIterator<DebugUtilsMessageSeverityFlagBitsEXT> for DebugUtilsMessageSeverityFlagsEXT {
    fn from_iter<T: IntoIterator<Item = DebugUtilsMessageSeverityFlagBitsEXT>>(
        iterator: T,
    ) -> DebugUtilsMessageSeverityFlagsEXT {
        let mut out = Self::empty();
        <Self as Extend<DebugUtilsMessageSeverityFlagBitsEXT>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for DebugUtilsMessageSeverityFlagsEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(DebugUtilsMessageSeverityFlagsEXT);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == DebugUtilsMessageSeverityFlagsEXT::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(DebugUtilsMessageSeverityFlagsEXT::VERBOSE) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(VERBOSE))?;
                    }
                    if self.0.contains(DebugUtilsMessageSeverityFlagsEXT::INFO) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(INFO))?;
                    }
                    if self.0.contains(DebugUtilsMessageSeverityFlagsEXT::WARNING) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(WARNING))?;
                    }
                    if self.0.contains(DebugUtilsMessageSeverityFlagsEXT::ERROR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(ERROR))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(DebugUtilsMessageSeverityFlagsEXT))
            .field(&Flags(*self))
            .finish()
    }
}
///# [VkDebugUtilsMessageTypeFlagBitsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDebugUtilsMessageTypeFlagBitsEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_debug_utils/VkDebugUtilsMessageTypeFlagBitsEXT.md")]
#[doc(alias = "VkDebugUtilsMessageTypeFlagsEXT")]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DebugUtilsMessageTypeFlagsEXT(u32);
impl DebugUtilsMessageTypeFlagsEXT {
    #[doc(alias = "VK_DEBUG_UTILS_MESSAGE_TYPE_GENERAL_BIT_EXT")]
    pub const GENERAL: Self = Self(1);
    #[doc(alias = "VK_DEBUG_UTILS_MESSAGE_TYPE_VALIDATION_BIT_EXT")]
    pub const VALIDATION: Self = Self(2);
    #[doc(alias = "VK_DEBUG_UTILS_MESSAGE_TYPE_PERFORMANCE_BIT_EXT")]
    pub const PERFORMANCE: Self = Self(4);
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
            all |= Self::GENERAL;
        }
        {
            all |= Self::VALIDATION;
        }
        {
            all |= Self::PERFORMANCE;
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
impl const std::ops::BitOr for DebugUtilsMessageTypeFlagsEXT {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl const std::ops::BitOrAssign for DebugUtilsMessageTypeFlagsEXT {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for DebugUtilsMessageTypeFlagsEXT {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl const std::ops::BitXorAssign for DebugUtilsMessageTypeFlagsEXT {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for DebugUtilsMessageTypeFlagsEXT {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl const std::ops::BitAndAssign for DebugUtilsMessageTypeFlagsEXT {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for DebugUtilsMessageTypeFlagsEXT {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl const std::ops::SubAssign for DebugUtilsMessageTypeFlagsEXT {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for DebugUtilsMessageTypeFlagsEXT {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<DebugUtilsMessageTypeFlagsEXT> for DebugUtilsMessageTypeFlagsEXT {
    fn extend<T: IntoIterator<Item = DebugUtilsMessageTypeFlagsEXT>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl FromIterator<DebugUtilsMessageTypeFlagsEXT> for DebugUtilsMessageTypeFlagsEXT {
    fn from_iter<T: IntoIterator<Item = DebugUtilsMessageTypeFlagsEXT>>(iterator: T) -> DebugUtilsMessageTypeFlagsEXT {
        let mut out = Self::empty();
        <Self as Extend<DebugUtilsMessageTypeFlagsEXT>>::extend(&mut out, iterator);
        out
    }
}
impl const Default for DebugUtilsMessageTypeFlagsEXT {
    fn default() -> Self {
        Self::empty()
    }
}
impl const From<DebugUtilsMessageTypeFlagBitsEXT> for DebugUtilsMessageTypeFlagsEXT {
    fn from(bit: DebugUtilsMessageTypeFlagBitsEXT) -> Self {
        unsafe { Self::from_bits_unchecked(bit.bits()) }
    }
}
impl Extend<DebugUtilsMessageTypeFlagBitsEXT> for DebugUtilsMessageTypeFlagsEXT {
    fn extend<T: IntoIterator<Item = DebugUtilsMessageTypeFlagBitsEXT>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, Self::from(i));
        }
    }
}
impl FromIterator<DebugUtilsMessageTypeFlagBitsEXT> for DebugUtilsMessageTypeFlagsEXT {
    fn from_iter<T: IntoIterator<Item = DebugUtilsMessageTypeFlagBitsEXT>>(
        iterator: T,
    ) -> DebugUtilsMessageTypeFlagsEXT {
        let mut out = Self::empty();
        <Self as Extend<DebugUtilsMessageTypeFlagBitsEXT>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for DebugUtilsMessageTypeFlagsEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(DebugUtilsMessageTypeFlagsEXT);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == DebugUtilsMessageTypeFlagsEXT::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(DebugUtilsMessageTypeFlagsEXT::GENERAL) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(GENERAL))?;
                    }
                    if self.0.contains(DebugUtilsMessageTypeFlagsEXT::VALIDATION) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(VALIDATION))?;
                    }
                    if self.0.contains(DebugUtilsMessageTypeFlagsEXT::PERFORMANCE) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PERFORMANCE))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(DebugUtilsMessageTypeFlagsEXT))
            .field(&Flags(*self))
            .finish()
    }
}
#[doc(alias = "VkDebugUtilsMessengerCreateFlagsEXT")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DebugUtilsMessengerCreateFlagsEXT(u32);
impl DebugUtilsMessengerCreateFlagsEXT {
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
}
#[doc(alias = "VkDebugUtilsMessengerCallbackDataFlagsEXT")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DebugUtilsMessengerCallbackDataFlagsEXT(u32);
impl DebugUtilsMessengerCallbackDataFlagsEXT {
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
}
#[doc(alias = "VK_EXT_DEBUG_UTILS_SPEC_VERSION")]
pub const EXT_DEBUG_UTILS_SPEC_VERSION: u32 = 2;
#[doc(alias = "VK_EXT_DEBUG_UTILS_EXTENSION_NAME")]
pub const EXT_DEBUG_UTILS_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_debug_utils");
///# [VkDebugUtilsMessageSeverityFlagBitsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDebugUtilsMessageSeverityFlagBitsEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_debug_utils/VkDebugUtilsMessageSeverityFlagBitsEXT.md")]
#[doc(alias = "VkDebugUtilsMessageSeverityFlagBitsEXT")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct DebugUtilsMessageSeverityFlagBitsEXT(u32);
impl DebugUtilsMessageSeverityFlagBitsEXT {
    #[doc(alias = "VK_DEBUG_UTILS_MESSAGE_SEVERITY_VERBOSE_BIT_EXT")]
    pub const VERBOSE: Self = Self(1);
    #[doc(alias = "VK_DEBUG_UTILS_MESSAGE_SEVERITY_INFO_BIT_EXT")]
    pub const INFO: Self = Self(16);
    #[doc(alias = "VK_DEBUG_UTILS_MESSAGE_SEVERITY_WARNING_BIT_EXT")]
    pub const WARNING: Self = Self(256);
    #[doc(alias = "VK_DEBUG_UTILS_MESSAGE_SEVERITY_ERROR_BIT_EXT")]
    pub const ERROR: Self = Self(4096);
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
            x if x == Self::VERBOSE.bits() => Some(Self(x)),
            x if x == Self::INFO.bits() => Some(Self(x)),
            x if x == Self::WARNING.bits() => Some(Self(x)),
            x if x == Self::ERROR.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
}
///# [VkDebugUtilsMessageTypeFlagBitsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDebugUtilsMessageTypeFlagBitsEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_debug_utils/VkDebugUtilsMessageTypeFlagBitsEXT.md")]
#[doc(alias = "VkDebugUtilsMessageTypeFlagBitsEXT")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct DebugUtilsMessageTypeFlagBitsEXT(u32);
impl DebugUtilsMessageTypeFlagBitsEXT {
    #[doc(alias = "VK_DEBUG_UTILS_MESSAGE_TYPE_GENERAL_BIT_EXT")]
    pub const GENERAL: Self = Self(1);
    #[doc(alias = "VK_DEBUG_UTILS_MESSAGE_TYPE_VALIDATION_BIT_EXT")]
    pub const VALIDATION: Self = Self(2);
    #[doc(alias = "VK_DEBUG_UTILS_MESSAGE_TYPE_PERFORMANCE_BIT_EXT")]
    pub const PERFORMANCE: Self = Self(4);
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
            x if x == Self::GENERAL.bits() => Some(Self(x)),
            x if x == Self::VALIDATION.bits() => Some(Self(x)),
            x if x == Self::PERFORMANCE.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
}
///# [vkSetDebugUtilsObjectNameEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkSetDebugUtilsObjectNameEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_debug_utils/vkSetDebugUtilsObjectNameEXT.md")]
#[doc(alias = "vkSetDebugUtilsObjectNameEXT")]
pub type FNSetDebugUtilsObjectNameExt =
    unsafe extern "system" fn(device: Device, p_name_info: *const DebugUtilsObjectNameInfoEXT) -> VulkanResultCodes;
///# [vkSetDebugUtilsObjectTagEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkSetDebugUtilsObjectTagEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_debug_utils/vkSetDebugUtilsObjectTagEXT.md")]
#[doc(alias = "vkSetDebugUtilsObjectTagEXT")]
pub type FNSetDebugUtilsObjectTagExt =
    unsafe extern "system" fn(device: Device, p_tag_info: *const DebugUtilsObjectTagInfoEXT) -> VulkanResultCodes;
///# [vkQueueBeginDebugUtilsLabelEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkQueueBeginDebugUtilsLabelEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_debug_utils/vkQueueBeginDebugUtilsLabelEXT.md")]
#[doc(alias = "vkQueueBeginDebugUtilsLabelEXT")]
pub type FNQueueBeginDebugUtilsLabelExt =
    unsafe extern "system" fn(queue: Queue, p_label_info: *const DebugUtilsLabelEXT);
///# [vkQueueEndDebugUtilsLabelEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkQueueEndDebugUtilsLabelEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_debug_utils/vkQueueEndDebugUtilsLabelEXT.md")]
#[doc(alias = "vkQueueEndDebugUtilsLabelEXT")]
pub type FNQueueEndDebugUtilsLabelExt = unsafe extern "system" fn(queue: Queue);
///# [vkQueueInsertDebugUtilsLabelEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkQueueInsertDebugUtilsLabelEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_debug_utils/vkQueueInsertDebugUtilsLabelEXT.md")]
#[doc(alias = "vkQueueInsertDebugUtilsLabelEXT")]
pub type FNQueueInsertDebugUtilsLabelExt =
    unsafe extern "system" fn(queue: Queue, p_label_info: *const DebugUtilsLabelEXT);
///# [vkCreateDebugUtilsMessengerEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateDebugUtilsMessengerEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_debug_utils/vkCreateDebugUtilsMessengerEXT.md")]
#[doc(alias = "vkCreateDebugUtilsMessengerEXT")]
pub type FNCreateDebugUtilsMessengerExt = unsafe extern "system" fn(
    instance: Instance,
    p_create_info: *const DebugUtilsMessengerCreateInfoEXT,
    p_allocator: *const AllocationCallbacks,
    p_messenger: *mut DebugUtilsMessengerEXT,
) -> VulkanResultCodes;
///# [vkDestroyDebugUtilsMessengerEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyDebugUtilsMessengerEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_debug_utils/vkDestroyDebugUtilsMessengerEXT.md")]
#[doc(alias = "vkDestroyDebugUtilsMessengerEXT")]
pub type FNDestroyDebugUtilsMessengerExt = unsafe extern "system" fn(
    instance: Instance,
    messenger: DebugUtilsMessengerEXT,
    p_allocator: *const AllocationCallbacks,
);
///# [vkSubmitDebugUtilsMessageEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkSubmitDebugUtilsMessageEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_debug_utils/vkSubmitDebugUtilsMessageEXT.md")]
#[doc(alias = "vkSubmitDebugUtilsMessageEXT")]
pub type FNSubmitDebugUtilsMessageExt = unsafe extern "system" fn(
    instance: Instance,
    message_severity: DebugUtilsMessageSeverityFlagBitsEXT,
    message_types: DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const DebugUtilsMessengerCallbackDataEXT,
);
///# [vkCmdBeginDebugUtilsLabelEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBeginDebugUtilsLabelEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_debug_utils/vkCmdBeginDebugUtilsLabelEXT.md")]
#[doc(alias = "vkCmdBeginDebugUtilsLabelEXT")]
pub type FNCmdBeginDebugUtilsLabelExt =
    unsafe extern "system" fn(command_buffer: CommandBuffer, p_label_info: *const DebugUtilsLabelEXT);
///# [vkCmdEndDebugUtilsLabelEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdEndDebugUtilsLabelEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_debug_utils/vkCmdEndDebugUtilsLabelEXT.md")]
#[doc(alias = "vkCmdEndDebugUtilsLabelEXT")]
pub type FNCmdEndDebugUtilsLabelExt = unsafe extern "system" fn(command_buffer: CommandBuffer);
///# [vkCmdInsertDebugUtilsLabelEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdInsertDebugUtilsLabelEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_debug_utils/vkCmdInsertDebugUtilsLabelEXT.md")]
#[doc(alias = "vkCmdInsertDebugUtilsLabelEXT")]
pub type FNCmdInsertDebugUtilsLabelExt =
    unsafe extern "system" fn(command_buffer: CommandBuffer, p_label_info: *const DebugUtilsLabelEXT);
