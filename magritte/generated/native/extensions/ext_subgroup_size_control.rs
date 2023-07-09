use crate::native::vulkan1_3::{
    PhysicalDeviceSubgroupSizeControlFeatures, PhysicalDeviceSubgroupSizeControlProperties,
    PipelineShaderStageRequiredSubgroupSizeCreateInfo,
};
///See [`PhysicalDeviceSubgroupSizeControlFeatures`]
#[doc(alias = "VkPhysicalDeviceSubgroupSizeControlFeaturesEXT")]
pub type PhysicalDeviceSubgroupSizeControlFeaturesEXT = PhysicalDeviceSubgroupSizeControlFeatures;
///See [`PhysicalDeviceSubgroupSizeControlProperties`]
#[doc(alias = "VkPhysicalDeviceSubgroupSizeControlPropertiesEXT")]
pub type PhysicalDeviceSubgroupSizeControlPropertiesEXT = PhysicalDeviceSubgroupSizeControlProperties;
///See [`PipelineShaderStageRequiredSubgroupSizeCreateInfo`]
#[doc(alias = "VkPipelineShaderStageRequiredSubgroupSizeCreateInfoEXT")]
pub type PipelineShaderStageRequiredSubgroupSizeCreateInfoEXT = PipelineShaderStageRequiredSubgroupSizeCreateInfo;
pub use crate::common::extensions::ext_subgroup_size_control::{
    EXT_SUBGROUP_SIZE_CONTROL_EXTENSION_NAME, EXT_SUBGROUP_SIZE_CONTROL_SPEC_VERSION,
};
