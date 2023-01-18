use crate::{
    cstr,
    vulkan1_0::{
        BaseInStructure, BaseOutStructure, Bool32, CommandBuffer, Extent2D, PhysicalDevice, SampleCountFlagBits,
        SampleCountFlags, StructureType,
    },
};
use std::ffi::CStr;
#[doc(alias = "VkSampleLocationEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SampleLocationEXT {
    x: f32,
    y: f32,
}
#[doc(alias = "VkSampleLocationsInfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SampleLocationsInfoEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "sampleLocationsPerPixel")]
    sample_locations_per_pixel: SampleCountFlagBits,
    #[doc(alias = "sampleLocationGridSize")]
    sample_location_grid_size: Extent2D,
    #[doc(alias = "sampleLocationsCount")]
    sample_locations_count: u32,
    #[doc(alias = "pSampleLocations")]
    sample_locations: *const SampleLocationEXT,
}
#[doc(alias = "VkAttachmentSampleLocationsEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct AttachmentSampleLocationsEXT {
    #[doc(alias = "attachmentIndex")]
    attachment_index: u32,
    #[doc(alias = "sampleLocationsInfo")]
    sample_locations_info: SampleLocationsInfoEXT,
}
#[doc(alias = "VkSubpassSampleLocationsEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SubpassSampleLocationsEXT {
    #[doc(alias = "subpassIndex")]
    subpass_index: u32,
    #[doc(alias = "sampleLocationsInfo")]
    sample_locations_info: SampleLocationsInfoEXT,
}
#[doc(alias = "VkRenderPassSampleLocationsBeginInfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct RenderPassSampleLocationsBeginInfoEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "attachmentInitialSampleLocationsCount")]
    attachment_initial_sample_locations_count: u32,
    #[doc(alias = "pAttachmentInitialSampleLocations")]
    attachment_initial_sample_locations: *const AttachmentSampleLocationsEXT,
    #[doc(alias = "postSubpassSampleLocationsCount")]
    post_subpass_sample_locations_count: u32,
    #[doc(alias = "pPostSubpassSampleLocations")]
    post_subpass_sample_locations: *const SubpassSampleLocationsEXT,
}
#[doc(alias = "VkPipelineSampleLocationsStateCreateInfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PipelineSampleLocationsStateCreateInfoEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "sampleLocationsEnable")]
    sample_locations_enable: Bool32,
    #[doc(alias = "sampleLocationsInfo")]
    sample_locations_info: SampleLocationsInfoEXT,
}
#[doc(alias = "VkPhysicalDeviceSampleLocationsPropertiesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceSampleLocationsPropertiesEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "sampleLocationSampleCounts")]
    sample_location_sample_counts: SampleCountFlags,
    #[doc(alias = "maxSampleLocationGridSize")]
    max_sample_location_grid_size: Extent2D,
    #[doc(alias = "sampleLocationCoordinateRange")]
    sample_location_coordinate_range: [f32; 2 as usize],
    #[doc(alias = "sampleLocationSubPixelBits")]
    sample_location_sub_pixel_bits: u32,
    #[doc(alias = "variableSampleLocations")]
    variable_sample_locations: Bool32,
}
#[doc(alias = "VkMultisamplePropertiesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct MultisamplePropertiesEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "maxSampleLocationGridSize")]
    max_sample_location_grid_size: Extent2D,
}
#[doc(alias = "VK_EXT_SAMPLE_LOCATIONS_SPEC_VERSION")]
pub const EXT_SAMPLE_LOCATIONS_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_EXT_SAMPLE_LOCATIONS_EXTENSION_NAME")]
pub const EXT_SAMPLE_LOCATIONS_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_sample_locations");
#[doc(alias = "vkGetPhysicalDeviceMultisamplePropertiesEXT")]
pub type FNGetPhysicalDeviceMultisamplePropertiesExt = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    samples: SampleCountFlagBits,
    p_multisample_properties: *mut MultisamplePropertiesEXT,
);
#[doc(alias = "vkCmdSetSampleLocationsEXT")]
pub type FNCmdSetSampleLocationsExt =
    unsafe extern "system" fn(command_buffer: CommandBuffer, p_sample_locations_info: *const SampleLocationsInfoEXT);
