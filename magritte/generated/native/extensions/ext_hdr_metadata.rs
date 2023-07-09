pub use crate::common::extensions::ext_hdr_metadata::XyColorEXT;
use crate::native::{
    extensions::khr_swapchain::SwapchainKHR,
    vulkan1_0::{BaseInStructure, Device, StructureType},
};
#[doc(alias = "VkHdrMetadataEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct HdrMetadataEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "displayPrimaryRed")]
    pub display_primary_red: XyColorEXT,
    #[doc(alias = "displayPrimaryGreen")]
    pub display_primary_green: XyColorEXT,
    #[doc(alias = "displayPrimaryBlue")]
    pub display_primary_blue: XyColorEXT,
    #[doc(alias = "whitePoint")]
    pub white_point: XyColorEXT,
    #[doc(alias = "maxLuminance")]
    pub max_luminance: f32,
    #[doc(alias = "minLuminance")]
    pub min_luminance: f32,
    #[doc(alias = "maxContentLightLevel")]
    pub max_content_light_level: f32,
    #[doc(alias = "maxFrameAverageLightLevel")]
    pub max_frame_average_light_level: f32,
}
impl Default for HdrMetadataEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::HdrMetadataExt,
            p_next: unsafe { std::mem::zeroed() },
            display_primary_red: unsafe { std::mem::zeroed() },
            display_primary_green: unsafe { std::mem::zeroed() },
            display_primary_blue: unsafe { std::mem::zeroed() },
            white_point: unsafe { std::mem::zeroed() },
            max_luminance: unsafe { std::mem::zeroed() },
            min_luminance: unsafe { std::mem::zeroed() },
            max_content_light_level: unsafe { std::mem::zeroed() },
            max_frame_average_light_level: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::ext_hdr_metadata::{EXT_HDR_METADATA_EXTENSION_NAME, EXT_HDR_METADATA_SPEC_VERSION};
#[doc(alias = "vkSetHdrMetadataEXT")]
pub type FNSetHdrMetadataExt = unsafe extern "system" fn(
    device: Device,
    swapchain_count: u32,
    p_swapchains: *const SwapchainKHR,
    p_metadata: *const HdrMetadataEXT,
);
