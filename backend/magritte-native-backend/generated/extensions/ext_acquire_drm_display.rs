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
#[doc(alias = "vkAcquireDrmDisplayEXT")]
pub type FNAcquireDrmDisplayExt =
    unsafe extern "system" fn(physical_device: PhysicalDevice, drm_fd: i32, display: DisplayKHR) -> VulkanResultCodes;
#[doc(alias = "vkGetDrmDisplayEXT")]
pub type FNGetDrmDisplayExt = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    drm_fd: i32,
    connector_id: u32,
    display: *mut DisplayKHR,
) -> VulkanResultCodes;
