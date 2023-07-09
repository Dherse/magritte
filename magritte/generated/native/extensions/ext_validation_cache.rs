use crate::native::vulkan1_0::{AllocationCallbacks, BaseInStructure, Device, StructureType, VulkanResultCodes};
#[doc(alias = "VkValidationCacheCreateInfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ValidationCacheCreateInfoEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub flags: ValidationCacheCreateFlagsEXT,
    #[doc(alias = "initialDataSize")]
    pub initial_data_size: usize,
    #[doc(alias = "pInitialData")]
    pub initial_data: *const std::ffi::c_void,
}
impl Default for ValidationCacheCreateInfoEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::ValidationCacheCreateInfoExt,
            p_next: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
            initial_data_size: unsafe { std::mem::zeroed() },
            initial_data: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkShaderModuleValidationCacheCreateInfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ShaderModuleValidationCacheCreateInfoEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "validationCache")]
    pub validation_cache: ValidationCacheEXT,
}
impl Default for ShaderModuleValidationCacheCreateInfoEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::ShaderModuleValidationCacheCreateInfoExt,
            p_next: unsafe { std::mem::zeroed() },
            validation_cache: unsafe { std::mem::zeroed() },
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(alias = "VkValidationCacheEXT")]
#[repr(transparent)]
pub struct ValidationCacheEXT(u64);
impl ValidationCacheEXT {
    pub const fn null() -> Self {
        Self(0)
    }
    pub const fn raw(&self) -> u64 {
        self.0
    }
}
impl Default for ValidationCacheEXT {
    fn default() -> Self {
        Self::null()
    }
}
pub use crate::common::extensions::ext_validation_cache::{
    ValidationCacheCreateFlagsEXT, ValidationCacheHeaderVersionEXT, EXT_VALIDATION_CACHE_EXTENSION_NAME,
    EXT_VALIDATION_CACHE_SPEC_VERSION,
};
#[doc(alias = "vkCreateValidationCacheEXT")]
pub type FNCreateValidationCacheExt = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const ValidationCacheCreateInfoEXT,
    p_allocator: *const AllocationCallbacks,
    p_validation_cache: *mut ValidationCacheEXT,
) -> VulkanResultCodes;
#[doc(alias = "vkDestroyValidationCacheEXT")]
pub type FNDestroyValidationCacheExt = unsafe extern "system" fn(
    device: Device,
    validation_cache: ValidationCacheEXT,
    p_allocator: *const AllocationCallbacks,
);
#[doc(alias = "vkGetValidationCacheDataEXT")]
pub type FNGetValidationCacheDataExt = unsafe extern "system" fn(
    device: Device,
    validation_cache: ValidationCacheEXT,
    p_data_size: *mut usize,
    p_data: *mut std::ffi::c_void,
) -> VulkanResultCodes;
#[doc(alias = "vkMergeValidationCachesEXT")]
pub type FNMergeValidationCachesExt = unsafe extern "system" fn(
    device: Device,
    dst_cache: ValidationCacheEXT,
    src_cache_count: u32,
    p_src_caches: *const ValidationCacheEXT,
) -> VulkanResultCodes;
