pub use crate::common::extensions::ext_sample_locations::SampleLocationEXT;
use crate::native::vulkan1_0::{
    BaseInStructure, BaseOutStructure, Bool32, CommandBuffer, Extent2D, PhysicalDevice, SampleCountFlagBits,
    SampleCountFlags, StructureType,
};
#[doc(alias = "VkSampleLocationsInfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SampleLocationsInfoEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "sampleLocationsPerPixel")]
    pub sample_locations_per_pixel: SampleCountFlagBits,
    #[doc(alias = "sampleLocationGridSize")]
    pub sample_location_grid_size: Extent2D,
    #[doc(alias = "sampleLocationsCount")]
    pub sample_locations_count: u32,
    #[doc(alias = "pSampleLocations")]
    pub sample_locations: *const SampleLocationEXT,
}
impl Default for SampleLocationsInfoEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::SampleLocationsInfoExt,
            p_next: unsafe { std::mem::zeroed() },
            sample_locations_per_pixel: unsafe { std::mem::zeroed() },
            sample_location_grid_size: unsafe { std::mem::zeroed() },
            sample_locations_count: unsafe { std::mem::zeroed() },
            sample_locations: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkAttachmentSampleLocationsEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct AttachmentSampleLocationsEXT {
    #[doc(alias = "attachmentIndex")]
    pub attachment_index: u32,
    #[doc(alias = "sampleLocationsInfo")]
    pub sample_locations_info: SampleLocationsInfoEXT,
}
impl Default for AttachmentSampleLocationsEXT {
    fn default() -> Self {
        Self {
            attachment_index: unsafe { std::mem::zeroed() },
            sample_locations_info: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkSubpassSampleLocationsEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SubpassSampleLocationsEXT {
    #[doc(alias = "subpassIndex")]
    pub subpass_index: u32,
    #[doc(alias = "sampleLocationsInfo")]
    pub sample_locations_info: SampleLocationsInfoEXT,
}
impl Default for SubpassSampleLocationsEXT {
    fn default() -> Self {
        Self {
            subpass_index: unsafe { std::mem::zeroed() },
            sample_locations_info: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkRenderPassSampleLocationsBeginInfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct RenderPassSampleLocationsBeginInfoEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "attachmentInitialSampleLocationsCount")]
    pub attachment_initial_sample_locations_count: u32,
    #[doc(alias = "pAttachmentInitialSampleLocations")]
    pub attachment_initial_sample_locations: *const AttachmentSampleLocationsEXT,
    #[doc(alias = "postSubpassSampleLocationsCount")]
    pub post_subpass_sample_locations_count: u32,
    #[doc(alias = "pPostSubpassSampleLocations")]
    pub post_subpass_sample_locations: *const SubpassSampleLocationsEXT,
}
impl Default for RenderPassSampleLocationsBeginInfoEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::RenderPassSampleLocationsBeginInfoExt,
            p_next: unsafe { std::mem::zeroed() },
            attachment_initial_sample_locations_count: unsafe { std::mem::zeroed() },
            attachment_initial_sample_locations: unsafe { std::mem::zeroed() },
            post_subpass_sample_locations_count: unsafe { std::mem::zeroed() },
            post_subpass_sample_locations: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPipelineSampleLocationsStateCreateInfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PipelineSampleLocationsStateCreateInfoEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "sampleLocationsEnable")]
    pub sample_locations_enable: Bool32,
    #[doc(alias = "sampleLocationsInfo")]
    pub sample_locations_info: SampleLocationsInfoEXT,
}
impl Default for PipelineSampleLocationsStateCreateInfoEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::PipelineSampleLocationsStateCreateInfoExt,
            p_next: unsafe { std::mem::zeroed() },
            sample_locations_enable: unsafe { std::mem::zeroed() },
            sample_locations_info: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceSampleLocationsPropertiesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceSampleLocationsPropertiesEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "sampleLocationSampleCounts")]
    pub sample_location_sample_counts: SampleCountFlags,
    #[doc(alias = "maxSampleLocationGridSize")]
    pub max_sample_location_grid_size: Extent2D,
    #[doc(alias = "sampleLocationCoordinateRange")]
    pub sample_location_coordinate_range: [f32; 2 as usize],
    #[doc(alias = "sampleLocationSubPixelBits")]
    pub sample_location_sub_pixel_bits: u32,
    #[doc(alias = "variableSampleLocations")]
    pub variable_sample_locations: Bool32,
}
impl Default for PhysicalDeviceSampleLocationsPropertiesEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceSampleLocationsPropertiesExt,
            p_next: unsafe { std::mem::zeroed() },
            sample_location_sample_counts: unsafe { std::mem::zeroed() },
            max_sample_location_grid_size: unsafe { std::mem::zeroed() },
            sample_location_coordinate_range: unsafe { std::mem::zeroed() },
            sample_location_sub_pixel_bits: unsafe { std::mem::zeroed() },
            variable_sample_locations: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkMultisamplePropertiesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct MultisamplePropertiesEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "maxSampleLocationGridSize")]
    pub max_sample_location_grid_size: Extent2D,
}
impl Default for MultisamplePropertiesEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::MultisamplePropertiesExt,
            p_next: unsafe { std::mem::zeroed() },
            max_sample_location_grid_size: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::ext_sample_locations::{
    EXT_SAMPLE_LOCATIONS_EXTENSION_NAME, EXT_SAMPLE_LOCATIONS_SPEC_VERSION,
};
#[doc(alias = "vkGetPhysicalDeviceMultisamplePropertiesEXT")]
pub type FNGetPhysicalDeviceMultisamplePropertiesExt = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    samples: SampleCountFlagBits,
    p_multisample_properties: *mut MultisamplePropertiesEXT,
);
#[doc(alias = "vkCmdSetSampleLocationsEXT")]
pub type FNCmdSetSampleLocationsExt =
    unsafe extern "system" fn(command_buffer: CommandBuffer, p_sample_locations_info: *const SampleLocationsInfoEXT);
