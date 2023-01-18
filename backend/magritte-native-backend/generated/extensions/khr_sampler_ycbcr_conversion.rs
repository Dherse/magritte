use crate::{
    cstr,
    vulkan1_1::{
        BindImagePlaneMemoryInfo, ChromaLocation, FNCreateSamplerYcbcrConversion, FNDestroySamplerYcbcrConversion,
        ImagePlaneMemoryRequirementsInfo, PhysicalDeviceSamplerYcbcrConversionFeatures, SamplerYcbcrConversion,
        SamplerYcbcrConversionCreateInfo, SamplerYcbcrConversionImageFormatProperties, SamplerYcbcrConversionInfo,
        SamplerYcbcrModelConversion, SamplerYcbcrRange,
    },
};
use std::ffi::CStr;
///See [`SamplerYcbcrConversion`]
#[doc(alias = "VkSamplerYcbcrConversionKHR")]
pub type SamplerYcbcrConversionKHR = SamplerYcbcrConversion;
///See [`SamplerYcbcrModelConversion`]
#[doc(alias = "VkSamplerYcbcrModelConversionKHR")]
pub type SamplerYcbcrModelConversionKHR = SamplerYcbcrModelConversion;
///See [`SamplerYcbcrRange`]
#[doc(alias = "VkSamplerYcbcrRangeKHR")]
pub type SamplerYcbcrRangeKHR = SamplerYcbcrRange;
///See [`ChromaLocation`]
#[doc(alias = "VkChromaLocationKHR")]
pub type ChromaLocationKHR = ChromaLocation;
///See [`SamplerYcbcrConversionInfo`]
#[doc(alias = "VkSamplerYcbcrConversionInfoKHR")]
pub type SamplerYcbcrConversionInfoKHR = SamplerYcbcrConversionInfo;
///See [`SamplerYcbcrConversionCreateInfo`]
#[doc(alias = "VkSamplerYcbcrConversionCreateInfoKHR")]
pub type SamplerYcbcrConversionCreateInfoKHR = SamplerYcbcrConversionCreateInfo;
///See [`BindImagePlaneMemoryInfo`]
#[doc(alias = "VkBindImagePlaneMemoryInfoKHR")]
pub type BindImagePlaneMemoryInfoKHR = BindImagePlaneMemoryInfo;
///See [`ImagePlaneMemoryRequirementsInfo`]
#[doc(alias = "VkImagePlaneMemoryRequirementsInfoKHR")]
pub type ImagePlaneMemoryRequirementsInfoKHR = ImagePlaneMemoryRequirementsInfo;
///See [`PhysicalDeviceSamplerYcbcrConversionFeatures`]
#[doc(alias = "VkPhysicalDeviceSamplerYcbcrConversionFeaturesKHR")]
pub type PhysicalDeviceSamplerYcbcrConversionFeaturesKHR = PhysicalDeviceSamplerYcbcrConversionFeatures;
///See [`SamplerYcbcrConversionImageFormatProperties`]
#[doc(alias = "VkSamplerYcbcrConversionImageFormatPropertiesKHR")]
pub type SamplerYcbcrConversionImageFormatPropertiesKHR = SamplerYcbcrConversionImageFormatProperties;
#[doc(alias = "VK_KHR_SAMPLER_YCBCR_CONVERSION_SPEC_VERSION")]
pub const KHR_SAMPLER_YCBCR_CONVERSION_SPEC_VERSION: u32 = 14;
#[doc(alias = "VK_KHR_SAMPLER_YCBCR_CONVERSION_EXTENSION_NAME")]
pub const KHR_SAMPLER_YCBCR_CONVERSION_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_sampler_ycbcr_conversion");
///See [`create_sampler_ycbcr_conversion`]
#[doc(alias = "vkCreateSamplerYcbcrConversionKHR")]
pub type FNCreateSamplerYcbcrConversionKhr = FNCreateSamplerYcbcrConversion;
///See [`destroy_sampler_ycbcr_conversion`]
#[doc(alias = "vkDestroySamplerYcbcrConversionKHR")]
pub type FNDestroySamplerYcbcrConversionKhr = FNDestroySamplerYcbcrConversion;
