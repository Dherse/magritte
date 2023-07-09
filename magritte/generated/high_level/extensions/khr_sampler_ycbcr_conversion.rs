pub use crate::common::extensions::khr_sampler_ycbcr_conversion::{
    KHR_SAMPLER_YCBCR_CONVERSION_EXTENSION_NAME, KHR_SAMPLER_YCBCR_CONVERSION_SPEC_VERSION,
};
use crate::vulkan1_1::{
    BindImagePlaneMemoryInfo, ChromaLocation, ImagePlaneMemoryRequirementsInfo,
    PhysicalDeviceSamplerYcbcrConversionFeatures, SamplerYcbcrConversion, SamplerYcbcrConversionCreateInfo,
    SamplerYcbcrConversionImageFormatProperties, SamplerYcbcrConversionInfo, SamplerYcbcrModelConversion,
    SamplerYcbcrRange,
};
#[doc(alias = "VkSamplerYcbcrConversionKHR")]
pub type SamplerYcbcrConversionKHR = SamplerYcbcrConversion;
#[doc(alias = "VkSamplerYcbcrModelConversionKHR")]
pub type SamplerYcbcrModelConversionKHR = SamplerYcbcrModelConversion;
#[doc(alias = "VkSamplerYcbcrRangeKHR")]
pub type SamplerYcbcrRangeKHR = SamplerYcbcrRange;
#[doc(alias = "VkChromaLocationKHR")]
pub type ChromaLocationKHR = ChromaLocation;
#[doc(alias = "VkSamplerYcbcrConversionInfoKHR")]
pub type SamplerYcbcrConversionInfoKHR = SamplerYcbcrConversionInfo;
#[doc(alias = "VkSamplerYcbcrConversionCreateInfoKHR")]
pub type SamplerYcbcrConversionCreateInfoKHR = SamplerYcbcrConversionCreateInfo;
#[doc(alias = "VkBindImagePlaneMemoryInfoKHR")]
pub type BindImagePlaneMemoryInfoKHR = BindImagePlaneMemoryInfo;
#[doc(alias = "VkImagePlaneMemoryRequirementsInfoKHR")]
pub type ImagePlaneMemoryRequirementsInfoKHR = ImagePlaneMemoryRequirementsInfo;
#[doc(alias = "VkPhysicalDeviceSamplerYcbcrConversionFeaturesKHR")]
pub type PhysicalDeviceSamplerYcbcrConversionFeaturesKHR = PhysicalDeviceSamplerYcbcrConversionFeatures;
#[doc(alias = "VkSamplerYcbcrConversionImageFormatPropertiesKHR")]
pub type SamplerYcbcrConversionImageFormatPropertiesKHR = SamplerYcbcrConversionImageFormatProperties;
