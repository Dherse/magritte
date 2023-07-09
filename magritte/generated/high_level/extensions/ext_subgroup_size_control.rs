pub use crate::common::extensions::ext_subgroup_size_control::{
    EXT_SUBGROUP_SIZE_CONTROL_EXTENSION_NAME, EXT_SUBGROUP_SIZE_CONTROL_SPEC_VERSION,
};
use crate::vulkan1_3::{
    PhysicalDeviceSubgroupSizeControlFeatures, PhysicalDeviceSubgroupSizeControlProperties,
    PipelineShaderStageRequiredSubgroupSizeCreateInfo,
};
#[doc(alias = "VkPhysicalDeviceSubgroupSizeControlFeaturesEXT")]
pub type PhysicalDeviceSubgroupSizeControlFeaturesEXT = PhysicalDeviceSubgroupSizeControlFeatures;
#[doc(alias = "VkPhysicalDeviceSubgroupSizeControlPropertiesEXT")]
pub type PhysicalDeviceSubgroupSizeControlPropertiesEXT = PhysicalDeviceSubgroupSizeControlProperties;
#[doc(alias = "VkPipelineShaderStageRequiredSubgroupSizeCreateInfoEXT")]
pub type PipelineShaderStageRequiredSubgroupSizeCreateInfoEXT = PipelineShaderStageRequiredSubgroupSizeCreateInfo;
