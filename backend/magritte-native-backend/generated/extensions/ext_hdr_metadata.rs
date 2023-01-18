use crate::{
    cstr,
    extensions::khr_swapchain::SwapchainKHR,
    vulkan1_0::{BaseInStructure, Device, StructureType},
};
use std::ffi::CStr;
#[doc(alias = "VkXYColorEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct XyColorEXT {
    x: f32,
    y: f32,
}
#[doc(alias = "VkHdrMetadataEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct HdrMetadataEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "displayPrimaryRed")]
    display_primary_red: XyColorEXT,
    #[doc(alias = "displayPrimaryGreen")]
    display_primary_green: XyColorEXT,
    #[doc(alias = "displayPrimaryBlue")]
    display_primary_blue: XyColorEXT,
    #[doc(alias = "whitePoint")]
    white_point: XyColorEXT,
    #[doc(alias = "maxLuminance")]
    max_luminance: f32,
    #[doc(alias = "minLuminance")]
    min_luminance: f32,
    #[doc(alias = "maxContentLightLevel")]
    max_content_light_level: f32,
    #[doc(alias = "maxFrameAverageLightLevel")]
    max_frame_average_light_level: f32,
}
#[doc(alias = "VK_EXT_HDR_METADATA_SPEC_VERSION")]
pub const EXT_HDR_METADATA_SPEC_VERSION: u32 = 2;
#[doc(alias = "VK_EXT_HDR_METADATA_EXTENSION_NAME")]
pub const EXT_HDR_METADATA_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_hdr_metadata");
#[doc(alias = "vkSetHdrMetadataEXT")]
pub type FNSetHdrMetadataExt = unsafe extern "system" fn(
    device: Device,
    swapchain_count: u32,
    p_swapchains: *const SwapchainKHR,
    p_metadata: *const HdrMetadataEXT,
);
