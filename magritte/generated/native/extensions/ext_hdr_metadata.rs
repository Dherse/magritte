//!# [VK_EXT_hdr_metadata](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_hdr_metadata.html)
# ! [doc = include_str ! ("../../../../doc/extensions/ext_hdr_metadata/VK_EXT_hdr_metadata.md")]
use crate::{
    cstr,
    extensions::khr_swapchain::SwapchainKHR,
    vulkan1_0::{BaseInStructure, Device, StructureType},
};
use std::ffi::CStr;
///# [VkXYColorEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkXYColorEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_hdr_metadata/VkXYColorEXT.md")]
#[doc(alias = "VkXYColorEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct XyColorEXT {
    x: f32,
    y: f32,
}
///# [VkHdrMetadataEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkHdrMetadataEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_hdr_metadata/VkHdrMetadataEXT.md")]
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
///# [vkSetHdrMetadataEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkSetHdrMetadataEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_hdr_metadata/vkSetHdrMetadataEXT.md")]
#[doc(alias = "vkSetHdrMetadataEXT")]
pub type FNSetHdrMetadataExt = unsafe extern "system" fn(
    device: Device,
    swapchain_count: u32,
    p_swapchains: *const SwapchainKHR,
    p_metadata: *const HdrMetadataEXT,
);
