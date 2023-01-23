//!# [VK_EXT_acquire_drm_display](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_acquire_drm_display.html)
# ! [doc = include_str ! ("../../../../doc/extensions/ext_acquire_drm_display/VK_EXT_acquire_drm_display.md")]
use crate::{
    cstr,
    extensions::khr_display::DisplayKHR,
    vulkan1_0::{PhysicalDevice, VulkanResultCodes},
};
use std::ffi::CStr;
#[doc(alias = "VK_EXT_ACQUIRE_DRM_DISPLAY_SPEC_VERSION")]
pub const EXT_ACQUIRE_DRM_DISPLAY_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_EXT_ACQUIRE_DRM_DISPLAY_EXTENSION_NAME")]
pub const EXT_ACQUIRE_DRM_DISPLAY_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_acquire_drm_display");
///# [vkAcquireDrmDisplayEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkAcquireDrmDisplayEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_acquire_drm_display/vkAcquireDrmDisplayEXT.md")]
#[doc(alias = "vkAcquireDrmDisplayEXT")]
pub type FNAcquireDrmDisplayExt =
    unsafe extern "system" fn(physical_device: PhysicalDevice, drm_fd: i32, display: DisplayKHR) -> VulkanResultCodes;
///# [vkGetDrmDisplayEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDrmDisplayEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_acquire_drm_display/vkGetDrmDisplayEXT.md")]
#[doc(alias = "vkGetDrmDisplayEXT")]
pub type FNGetDrmDisplayExt = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    drm_fd: i32,
    connector_id: u32,
    display: *mut DisplayKHR,
) -> VulkanResultCodes;
