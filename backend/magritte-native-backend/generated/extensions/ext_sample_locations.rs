//!# [VK_EXT_sample_locations](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_sample_locations.html)
# ! [doc = include_str ! ("../../../../doc/extensions/ext_sample_locations/VK_EXT_sample_locations.md")]
use crate::{
    cstr,
    vulkan1_0::{
        BaseInStructure, BaseOutStructure, Bool32, CommandBuffer, Extent2D, PhysicalDevice, SampleCountFlagBits,
        SampleCountFlags, StructureType,
    },
};
use std::ffi::CStr;
///# [VkSampleLocationEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSampleLocationEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_sample_locations/VkSampleLocationEXT.md")]
#[doc(alias = "VkSampleLocationEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SampleLocationEXT {
    x: f32,
    y: f32,
}
///# [VkSampleLocationsInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSampleLocationsInfoEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_sample_locations/VkSampleLocationsInfoEXT.md")]
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
///# [VkAttachmentSampleLocationsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAttachmentSampleLocationsEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_sample_locations/VkAttachmentSampleLocationsEXT.md")]
#[doc(alias = "VkAttachmentSampleLocationsEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct AttachmentSampleLocationsEXT {
    #[doc(alias = "attachmentIndex")]
    attachment_index: u32,
    #[doc(alias = "sampleLocationsInfo")]
    sample_locations_info: SampleLocationsInfoEXT,
}
///# [VkSubpassSampleLocationsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSubpassSampleLocationsEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_sample_locations/VkSubpassSampleLocationsEXT.md")]
#[doc(alias = "VkSubpassSampleLocationsEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SubpassSampleLocationsEXT {
    #[doc(alias = "subpassIndex")]
    subpass_index: u32,
    #[doc(alias = "sampleLocationsInfo")]
    sample_locations_info: SampleLocationsInfoEXT,
}
///# [VkRenderPassSampleLocationsBeginInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRenderPassSampleLocationsBeginInfoEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_sample_locations/VkRenderPassSampleLocationsBeginInfoEXT.md")]
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
///# [VkPipelineSampleLocationsStateCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineSampleLocationsStateCreateInfoEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_sample_locations/VkPipelineSampleLocationsStateCreateInfoEXT.md")]
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
///# [VkPhysicalDeviceSampleLocationsPropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceSampleLocationsPropertiesEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_sample_locations/VkPhysicalDeviceSampleLocationsPropertiesEXT.md")]
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
///# [VkMultisamplePropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMultisamplePropertiesEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_sample_locations/VkMultisamplePropertiesEXT.md")]
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
///# [vkGetPhysicalDeviceMultisamplePropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceMultisamplePropertiesEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_sample_locations/vkGetPhysicalDeviceMultisamplePropertiesEXT.md")]
#[doc(alias = "vkGetPhysicalDeviceMultisamplePropertiesEXT")]
pub type FNGetPhysicalDeviceMultisamplePropertiesExt = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    samples: SampleCountFlagBits,
    p_multisample_properties: *mut MultisamplePropertiesEXT,
);
///# [vkCmdSetSampleLocationsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetSampleLocationsEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_sample_locations/vkCmdSetSampleLocationsEXT.md")]
#[doc(alias = "vkCmdSetSampleLocationsEXT")]
pub type FNCmdSetSampleLocationsExt =
    unsafe extern "system" fn(command_buffer: CommandBuffer, p_sample_locations_info: *const SampleLocationsInfoEXT);
