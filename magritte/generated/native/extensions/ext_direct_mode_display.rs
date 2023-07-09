pub use crate::common::extensions::ext_direct_mode_display::{
    EXT_DIRECT_MODE_DISPLAY_EXTENSION_NAME, EXT_DIRECT_MODE_DISPLAY_SPEC_VERSION,
};
use crate::native::{
    extensions::khr_display::DisplayKHR,
    vulkan1_0::{PhysicalDevice, VulkanResultCodes},
};
#[doc(alias = "vkReleaseDisplayEXT")]
pub type FNReleaseDisplayExt =
    unsafe extern "system" fn(physical_device: PhysicalDevice, display: DisplayKHR) -> VulkanResultCodes;
