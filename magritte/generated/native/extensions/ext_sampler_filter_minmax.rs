use crate::native::vulkan1_2::{
    PhysicalDeviceSamplerFilterMinmaxProperties, SamplerReductionMode, SamplerReductionModeCreateInfo,
};
///See [`SamplerReductionMode`]
#[doc(alias = "VkSamplerReductionModeEXT")]
pub type SamplerReductionModeEXT = SamplerReductionMode;
///See [`PhysicalDeviceSamplerFilterMinmaxProperties`]
#[doc(alias = "VkPhysicalDeviceSamplerFilterMinmaxPropertiesEXT")]
pub type PhysicalDeviceSamplerFilterMinmaxPropertiesEXT = PhysicalDeviceSamplerFilterMinmaxProperties;
///See [`SamplerReductionModeCreateInfo`]
#[doc(alias = "VkSamplerReductionModeCreateInfoEXT")]
pub type SamplerReductionModeCreateInfoEXT = SamplerReductionModeCreateInfo;
pub use crate::common::extensions::ext_sampler_filter_minmax::{
    EXT_SAMPLER_FILTER_MINMAX_EXTENSION_NAME, EXT_SAMPLER_FILTER_MINMAX_SPEC_VERSION,
};
