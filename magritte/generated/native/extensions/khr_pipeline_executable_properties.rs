use crate::native::vulkan1_0::{
    BaseInStructure, BaseOutStructure, Bool32, Device, Pipeline, ShaderStageFlags, StructureType, VulkanResultCodes,
    MAX_DESCRIPTION_SIZE,
};
#[doc(alias = "VkPhysicalDevicePipelineExecutablePropertiesFeaturesKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDevicePipelineExecutablePropertiesFeaturesKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "pipelineExecutableInfo")]
    pub pipeline_executable_info: Bool32,
}
impl Default for PhysicalDevicePipelineExecutablePropertiesFeaturesKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDevicePipelineExecutablePropertiesFeaturesKhr,
            p_next: unsafe { std::mem::zeroed() },
            pipeline_executable_info: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPipelineInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PipelineInfoKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub pipeline: Pipeline,
}
impl Default for PipelineInfoKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::PipelineInfoKhr,
            p_next: unsafe { std::mem::zeroed() },
            pipeline: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPipelineExecutablePropertiesKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PipelineExecutablePropertiesKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    pub stages: ShaderStageFlags,
    pub name: [std::ffi::c_char; MAX_DESCRIPTION_SIZE as usize],
    pub description: [std::ffi::c_char; MAX_DESCRIPTION_SIZE as usize],
    #[doc(alias = "subgroupSize")]
    pub subgroup_size: u32,
}
impl Default for PipelineExecutablePropertiesKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::PipelineExecutablePropertiesKhr,
            p_next: unsafe { std::mem::zeroed() },
            stages: unsafe { std::mem::zeroed() },
            name: unsafe { std::mem::zeroed() },
            description: unsafe { std::mem::zeroed() },
            subgroup_size: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPipelineExecutableInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PipelineExecutableInfoKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub pipeline: Pipeline,
    #[doc(alias = "executableIndex")]
    pub executable_index: u32,
}
impl Default for PipelineExecutableInfoKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::PipelineExecutableInfoKhr,
            p_next: unsafe { std::mem::zeroed() },
            pipeline: unsafe { std::mem::zeroed() },
            executable_index: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPipelineExecutableStatisticKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PipelineExecutableStatisticKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    pub name: [std::ffi::c_char; MAX_DESCRIPTION_SIZE as usize],
    pub description: [std::ffi::c_char; MAX_DESCRIPTION_SIZE as usize],
    pub format: PipelineExecutableStatisticFormatKHR,
    pub value: PipelineExecutableStatisticValueKHR,
}
impl Default for PipelineExecutableStatisticKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::PipelineExecutableStatisticKhr,
            p_next: unsafe { std::mem::zeroed() },
            name: unsafe { std::mem::zeroed() },
            description: unsafe { std::mem::zeroed() },
            format: unsafe { std::mem::zeroed() },
            value: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPipelineExecutableInternalRepresentationKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PipelineExecutableInternalRepresentationKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    pub name: [std::ffi::c_char; MAX_DESCRIPTION_SIZE as usize],
    pub description: [std::ffi::c_char; MAX_DESCRIPTION_SIZE as usize],
    #[doc(alias = "isText")]
    pub is_text: Bool32,
    #[doc(alias = "dataSize")]
    pub data_size: usize,
    #[doc(alias = "pData")]
    pub data: *mut std::ffi::c_void,
}
impl Default for PipelineExecutableInternalRepresentationKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::PipelineExecutableInternalRepresentationKhr,
            p_next: unsafe { std::mem::zeroed() },
            name: unsafe { std::mem::zeroed() },
            description: unsafe { std::mem::zeroed() },
            is_text: unsafe { std::mem::zeroed() },
            data_size: unsafe { std::mem::zeroed() },
            data: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPipelineExecutableStatisticValueKHR")]
#[repr(C)]
#[derive(Clone, Copy)]
pub union PipelineExecutableStatisticValueKHR {
    pub b32: Bool32,
    pub i64: i64,
    pub u64: u64,
    pub f64: f64,
}
pub use crate::common::extensions::khr_pipeline_executable_properties::{
    PipelineExecutableStatisticFormatKHR, KHR_PIPELINE_EXECUTABLE_PROPERTIES_EXTENSION_NAME,
    KHR_PIPELINE_EXECUTABLE_PROPERTIES_SPEC_VERSION,
};
#[doc(alias = "vkGetPipelineExecutablePropertiesKHR")]
pub type FNGetPipelineExecutablePropertiesKhr = unsafe extern "system" fn(
    device: Device,
    p_pipeline_info: *const PipelineInfoKHR,
    p_executable_count: *mut u32,
    p_properties: *mut PipelineExecutablePropertiesKHR,
) -> VulkanResultCodes;
#[doc(alias = "vkGetPipelineExecutableStatisticsKHR")]
pub type FNGetPipelineExecutableStatisticsKhr = unsafe extern "system" fn(
    device: Device,
    p_executable_info: *const PipelineExecutableInfoKHR,
    p_statistic_count: *mut u32,
    p_statistics: *mut PipelineExecutableStatisticKHR,
) -> VulkanResultCodes;
#[doc(alias = "vkGetPipelineExecutableInternalRepresentationsKHR")]
pub type FNGetPipelineExecutableInternalRepresentationsKhr = unsafe extern "system" fn(
    device: Device,
    p_executable_info: *const PipelineExecutableInfoKHR,
    p_internal_representation_count: *mut u32,
    p_internal_representations: *mut PipelineExecutableInternalRepresentationKHR,
) -> VulkanResultCodes;
