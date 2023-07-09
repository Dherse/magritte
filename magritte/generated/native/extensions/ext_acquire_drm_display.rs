pub use crate::common::extensions::ext_acquire_drm_display::{
    EXT_ACQUIRE_DRM_DISPLAY_EXTENSION_NAME, EXT_ACQUIRE_DRM_DISPLAY_SPEC_VERSION,
};
use crate::native::{
    extensions::khr_display::DisplayKHR,
    vulkan1_0::{PhysicalDevice, VulkanResultCodes},
};
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
