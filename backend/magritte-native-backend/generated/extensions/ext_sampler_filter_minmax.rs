//!# [VK_EXT_sampler_filter_minmax](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_sampler_filter_minmax.html)
# ! [doc = include_str ! ("../../../../doc/extensions/ext_sampler_filter_minmax/VK_EXT_sampler_filter_minmax.md")]
use crate::{
    cstr,
    vulkan1_2::{PhysicalDeviceSamplerFilterMinmaxProperties, SamplerReductionMode, SamplerReductionModeCreateInfo},
};
use std::ffi::CStr;
///See [`SamplerReductionMode`]
#[doc(alias = "VkSamplerReductionModeEXT")]
pub type SamplerReductionModeEXT = SamplerReductionMode;
///See [`PhysicalDeviceSamplerFilterMinmaxProperties`]
#[doc(alias = "VkPhysicalDeviceSamplerFilterMinmaxPropertiesEXT")]
pub type PhysicalDeviceSamplerFilterMinmaxPropertiesEXT = PhysicalDeviceSamplerFilterMinmaxProperties;
///See [`SamplerReductionModeCreateInfo`]
#[doc(alias = "VkSamplerReductionModeCreateInfoEXT")]
pub type SamplerReductionModeCreateInfoEXT = SamplerReductionModeCreateInfo;
#[doc(alias = "VK_EXT_SAMPLER_FILTER_MINMAX_SPEC_VERSION")]
pub const EXT_SAMPLER_FILTER_MINMAX_SPEC_VERSION: u32 = 2;
#[doc(alias = "VK_EXT_SAMPLER_FILTER_MINMAX_EXTENSION_NAME")]
pub const EXT_SAMPLER_FILTER_MINMAX_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_sampler_filter_minmax");