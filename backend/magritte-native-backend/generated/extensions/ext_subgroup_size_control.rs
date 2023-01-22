//!# [VK_EXT_subgroup_size_control](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_subgroup_size_control.html)
# ! [doc = include_str ! ("../../../../doc/extensions/ext_subgroup_size_control/VK_EXT_subgroup_size_control.md")]
use crate::{
    cstr,
    vulkan1_3::{
        PhysicalDeviceSubgroupSizeControlFeatures, PhysicalDeviceSubgroupSizeControlProperties,
        PipelineShaderStageRequiredSubgroupSizeCreateInfo,
    },
};
use std::ffi::CStr;
///See [`PhysicalDeviceSubgroupSizeControlFeatures`]
#[doc(alias = "VkPhysicalDeviceSubgroupSizeControlFeaturesEXT")]
pub type PhysicalDeviceSubgroupSizeControlFeaturesEXT = PhysicalDeviceSubgroupSizeControlFeatures;
///See [`PhysicalDeviceSubgroupSizeControlProperties`]
#[doc(alias = "VkPhysicalDeviceSubgroupSizeControlPropertiesEXT")]
pub type PhysicalDeviceSubgroupSizeControlPropertiesEXT = PhysicalDeviceSubgroupSizeControlProperties;
///See [`PipelineShaderStageRequiredSubgroupSizeCreateInfo`]
#[doc(alias = "VkPipelineShaderStageRequiredSubgroupSizeCreateInfoEXT")]
pub type PipelineShaderStageRequiredSubgroupSizeCreateInfoEXT = PipelineShaderStageRequiredSubgroupSizeCreateInfo;
#[doc(alias = "VK_EXT_SUBGROUP_SIZE_CONTROL_SPEC_VERSION")]
pub const EXT_SUBGROUP_SIZE_CONTROL_SPEC_VERSION: u32 = 2;
#[doc(alias = "VK_EXT_SUBGROUP_SIZE_CONTROL_EXTENSION_NAME")]
pub const EXT_SUBGROUP_SIZE_CONTROL_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_subgroup_size_control");