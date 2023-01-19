//!# [VK_KHR_pipeline_executable_properties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_pipeline_executable_properties.html)
# ! [doc = include_str ! ("../../../../doc/extensions/khr_pipeline_executable_properties/VK_KHR_pipeline_executable_properties.md")]
use crate::{
    cstr,
    vulkan1_0::{
        BaseInStructure, BaseOutStructure, Bool32, Device, Pipeline, ShaderStageFlags, StructureType,
        VulkanResultCodes, MAX_DESCRIPTION_SIZE,
    },
};
use std::ffi::CStr;
///# [VkPhysicalDevicePipelineExecutablePropertiesFeaturesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePipelineExecutablePropertiesFeaturesKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_pipeline_executable_properties/VkPhysicalDevicePipelineExecutablePropertiesFeaturesKHR.md")]
#[doc(alias = "VkPhysicalDevicePipelineExecutablePropertiesFeaturesKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDevicePipelineExecutablePropertiesFeaturesKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "pipelineExecutableInfo")]
    pipeline_executable_info: Bool32,
}
///# [VkPipelineInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineInfoKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_pipeline_executable_properties/VkPipelineInfoKHR.md")]
#[doc(alias = "VkPipelineInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PipelineInfoKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    pipeline: Pipeline,
}
///# [VkPipelineExecutablePropertiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineExecutablePropertiesKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_pipeline_executable_properties/VkPipelineExecutablePropertiesKHR.md")]
#[doc(alias = "VkPipelineExecutablePropertiesKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PipelineExecutablePropertiesKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    stages: ShaderStageFlags,
    name: [std::ffi::c_char; MAX_DESCRIPTION_SIZE as usize],
    description: [std::ffi::c_char; MAX_DESCRIPTION_SIZE as usize],
    #[doc(alias = "subgroupSize")]
    subgroup_size: u32,
}
///# [VkPipelineExecutableInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineExecutableInfoKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_pipeline_executable_properties/VkPipelineExecutableInfoKHR.md")]
#[doc(alias = "VkPipelineExecutableInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PipelineExecutableInfoKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    pipeline: Pipeline,
    #[doc(alias = "executableIndex")]
    executable_index: u32,
}
///# [VkPipelineExecutableStatisticKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineExecutableStatisticKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_pipeline_executable_properties/VkPipelineExecutableStatisticKHR.md")]
#[doc(alias = "VkPipelineExecutableStatisticKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PipelineExecutableStatisticKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    name: [std::ffi::c_char; MAX_DESCRIPTION_SIZE as usize],
    description: [std::ffi::c_char; MAX_DESCRIPTION_SIZE as usize],
    format: PipelineExecutableStatisticFormatKHR,
    value: PipelineExecutableStatisticValueKHR,
}
///# [VkPipelineExecutableInternalRepresentationKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineExecutableInternalRepresentationKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_pipeline_executable_properties/VkPipelineExecutableInternalRepresentationKHR.md")]
#[doc(alias = "VkPipelineExecutableInternalRepresentationKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PipelineExecutableInternalRepresentationKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    name: [std::ffi::c_char; MAX_DESCRIPTION_SIZE as usize],
    description: [std::ffi::c_char; MAX_DESCRIPTION_SIZE as usize],
    #[doc(alias = "isText")]
    is_text: Bool32,
    #[doc(alias = "dataSize")]
    data_size: usize,
    #[doc(alias = "pData")]
    data: *mut std::ffi::c_void,
}
///# [VkPipelineExecutableStatisticValueKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineExecutableStatisticValueKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_pipeline_executable_properties/VkPipelineExecutableStatisticValueKHR.md")]
#[doc(alias = "VkPipelineExecutableStatisticValueKHR")]
#[repr(C)]
#[derive(Clone, Copy)]
pub union PipelineExecutableStatisticValueKHR {
    b32: Bool32,
    i64: i64,
    u64: u64,
    f64: f64,
}
#[doc(alias = "VK_KHR_PIPELINE_EXECUTABLE_PROPERTIES_SPEC_VERSION")]
pub const KHR_PIPELINE_EXECUTABLE_PROPERTIES_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_KHR_PIPELINE_EXECUTABLE_PROPERTIES_EXTENSION_NAME")]
pub const KHR_PIPELINE_EXECUTABLE_PROPERTIES_EXTENSION_NAME: &'static CStr =
    cstr!("VK_KHR_pipeline_executable_properties");
///# [VkPipelineExecutableStatisticFormatKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineExecutableStatisticFormatKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_pipeline_executable_properties/VkPipelineExecutableStatisticFormatKHR.md")]
#[doc(alias = "VkPipelineExecutableStatisticFormatKHR")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct PipelineExecutableStatisticFormatKHR(i32);
impl PipelineExecutableStatisticFormatKHR {
    #[doc(alias = "VK_PIPELINE_EXECUTABLE_STATISTIC_FORMAT_BOOL32_KHR")]
    pub const BOOL32: Self = Self(0);
    #[doc(alias = "VK_PIPELINE_EXECUTABLE_STATISTIC_FORMAT_INT64_KHR")]
    pub const INT64: Self = Self(1);
    #[doc(alias = "VK_PIPELINE_EXECUTABLE_STATISTIC_FORMAT_UINT64_KHR")]
    pub const UINT64: Self = Self(2);
    #[doc(alias = "VK_PIPELINE_EXECUTABLE_STATISTIC_FORMAT_FLOAT64_KHR")]
    pub const FLOAT64: Self = Self(3);
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
    ///The bits of this variant
    #[inline]
    pub const fn bits(&self) -> i32 {
        self.0
    }
    ///Builds a bitmask from the bits of this variant
    #[inline]
    pub const fn from_bits(bits: i32) -> Option<Self> {
        match bits {
            x if x == Self::BOOL32.bits() => Some(Self(x)),
            x if x == Self::INT64.bits() => Some(Self(x)),
            x if x == Self::UINT64.bits() => Some(Self(x)),
            x if x == Self::FLOAT64.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: i32) -> Self {
        Self(bits)
    }
}
///# [vkGetPipelineExecutablePropertiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPipelineExecutablePropertiesKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_pipeline_executable_properties/vkGetPipelineExecutablePropertiesKHR.md")]
#[doc(alias = "vkGetPipelineExecutablePropertiesKHR")]
pub type FNGetPipelineExecutablePropertiesKhr = unsafe extern "system" fn(
    device: Device,
    p_pipeline_info: *const PipelineInfoKHR,
    p_executable_count: *mut u32,
    p_properties: *mut PipelineExecutablePropertiesKHR,
) -> VulkanResultCodes;
///# [vkGetPipelineExecutableStatisticsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPipelineExecutableStatisticsKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_pipeline_executable_properties/vkGetPipelineExecutableStatisticsKHR.md")]
#[doc(alias = "vkGetPipelineExecutableStatisticsKHR")]
pub type FNGetPipelineExecutableStatisticsKhr = unsafe extern "system" fn(
    device: Device,
    p_executable_info: *const PipelineExecutableInfoKHR,
    p_statistic_count: *mut u32,
    p_statistics: *mut PipelineExecutableStatisticKHR,
) -> VulkanResultCodes;
///# [vkGetPipelineExecutableInternalRepresentationsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPipelineExecutableInternalRepresentationsKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_pipeline_executable_properties/vkGetPipelineExecutableInternalRepresentationsKHR.md")]
#[doc(alias = "vkGetPipelineExecutableInternalRepresentationsKHR")]
pub type FNGetPipelineExecutableInternalRepresentationsKhr = unsafe extern "system" fn(
    device: Device,
    p_executable_info: *const PipelineExecutableInfoKHR,
    p_internal_representation_count: *mut u32,
    p_internal_representations: *mut PipelineExecutableInternalRepresentationKHR,
) -> VulkanResultCodes;
