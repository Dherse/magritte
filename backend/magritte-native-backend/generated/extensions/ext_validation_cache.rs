//!# [VK_EXT_validation_cache](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_validation_cache.html)
# ! [doc = include_str ! ("../../../../doc/extensions/ext_validation_cache/VK_EXT_validation_cache.md")]
use crate::{
    cstr,
    vulkan1_0::{AllocationCallbacks, BaseInStructure, Device, StructureType, VulkanResultCodes},
};
use std::ffi::CStr;
///# [VkValidationCacheCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkValidationCacheCreateInfoEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_validation_cache/VkValidationCacheCreateInfoEXT.md")]
#[doc(alias = "VkValidationCacheCreateInfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ValidationCacheCreateInfoEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    flags: ValidationCacheCreateFlagsEXT,
    #[doc(alias = "initialDataSize")]
    initial_data_size: usize,
    #[doc(alias = "pInitialData")]
    initial_data: *const std::ffi::c_void,
}
///# [VkShaderModuleValidationCacheCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkShaderModuleValidationCacheCreateInfoEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_validation_cache/VkShaderModuleValidationCacheCreateInfoEXT.md")]
#[doc(alias = "VkShaderModuleValidationCacheCreateInfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ShaderModuleValidationCacheCreateInfoEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "validationCache")]
    validation_cache: ValidationCacheEXT,
}
///# [VkValidationCacheEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkValidationCacheEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_validation_cache/VkValidationCacheEXT.md")]
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(alias = "VkValidationCacheEXT")]
#[repr(transparent)]
pub struct ValidationCacheEXT(u64);
impl ValidationCacheEXT {
    pub const fn null() -> Self {
        Self(0)
    }
}
impl const Default for ValidationCacheEXT {
    fn default() -> Self {
        Self::null()
    }
}
#[doc(alias = "VkValidationCacheCreateFlagsEXT")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ValidationCacheCreateFlagsEXT(u32);
impl ValidationCacheCreateFlagsEXT {
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
}
#[doc(alias = "VK_EXT_VALIDATION_CACHE_SPEC_VERSION")]
pub const EXT_VALIDATION_CACHE_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_EXT_VALIDATION_CACHE_EXTENSION_NAME")]
pub const EXT_VALIDATION_CACHE_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_validation_cache");
///# [VkValidationCacheHeaderVersionEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkValidationCacheHeaderVersionEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_validation_cache/VkValidationCacheHeaderVersionEXT.md")]
#[doc(alias = "VkValidationCacheHeaderVersionEXT")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct ValidationCacheHeaderVersionEXT(i32);
impl ValidationCacheHeaderVersionEXT {
    #[doc(alias = "VK_VALIDATION_CACHE_HEADER_VERSION_ONE_EXT")]
    pub const ONE: Self = Self(1);
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
    ///The bits of this variant
    #[inline]
    pub const fn bits(&self) -> i32 {
        self.0
    }
    ///Builds a bitmask from the bits of this variant
    #[inline]
    pub const fn from_bits(bits: i32) -> Option<Self> {
        match bits {
            x if x == Self::ONE.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: i32) -> Self {
        Self(bits)
    }
}
///# [vkCreateValidationCacheEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateValidationCacheEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_validation_cache/vkCreateValidationCacheEXT.md")]
#[doc(alias = "vkCreateValidationCacheEXT")]
pub type FNCreateValidationCacheExt = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const ValidationCacheCreateInfoEXT,
    p_allocator: *const AllocationCallbacks,
    p_validation_cache: *mut ValidationCacheEXT,
) -> VulkanResultCodes;
///# [vkDestroyValidationCacheEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyValidationCacheEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_validation_cache/vkDestroyValidationCacheEXT.md")]
#[doc(alias = "vkDestroyValidationCacheEXT")]
pub type FNDestroyValidationCacheExt = unsafe extern "system" fn(
    device: Device,
    validation_cache: ValidationCacheEXT,
    p_allocator: *const AllocationCallbacks,
);
///# [vkGetValidationCacheDataEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetValidationCacheDataEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_validation_cache/vkGetValidationCacheDataEXT.md")]
#[doc(alias = "vkGetValidationCacheDataEXT")]
pub type FNGetValidationCacheDataExt = unsafe extern "system" fn(
    device: Device,
    validation_cache: ValidationCacheEXT,
    p_data_size: *mut usize,
    p_data: *mut std::ffi::c_void,
) -> VulkanResultCodes;
///# [vkMergeValidationCachesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkMergeValidationCachesEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_validation_cache/vkMergeValidationCachesEXT.md")]
#[doc(alias = "vkMergeValidationCachesEXT")]
pub type FNMergeValidationCachesExt = unsafe extern "system" fn(
    device: Device,
    dst_cache: ValidationCacheEXT,
    src_cache_count: u32,
    p_src_caches: *const ValidationCacheEXT,
) -> VulkanResultCodes;
