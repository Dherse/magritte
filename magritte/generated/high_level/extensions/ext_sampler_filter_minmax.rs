pub use crate::common::extensions::ext_sampler_filter_minmax::{
    EXT_SAMPLER_FILTER_MINMAX_EXTENSION_NAME, EXT_SAMPLER_FILTER_MINMAX_SPEC_VERSION,
};
use crate::vulkan1_2::{
    PhysicalDeviceSamplerFilterMinmaxProperties, SamplerReductionMode, SamplerReductionModeCreateInfo,
};
#[doc(alias = "VkSamplerReductionModeEXT")]
pub type SamplerReductionModeEXT = SamplerReductionMode;
#[doc(alias = "VkPhysicalDeviceSamplerFilterMinmaxPropertiesEXT")]
pub type PhysicalDeviceSamplerFilterMinmaxPropertiesEXT = PhysicalDeviceSamplerFilterMinmaxProperties;
#[doc(alias = "VkSamplerReductionModeCreateInfoEXT")]
pub type SamplerReductionModeCreateInfoEXT = SamplerReductionModeCreateInfo;
