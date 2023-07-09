pub use crate::common::extensions::khr_external_memory_capabilities::{
    KHR_EXTERNAL_MEMORY_CAPABILITIES_EXTENSION_NAME, KHR_EXTERNAL_MEMORY_CAPABILITIES_SPEC_VERSION,
};
use crate::vulkan1_1::{
    ExternalBufferProperties, ExternalImageFormatProperties, ExternalMemoryFeatureFlagBits, ExternalMemoryFeatureFlags,
    ExternalMemoryHandleTypeFlagBits, ExternalMemoryHandleTypeFlags, ExternalMemoryProperties,
    PhysicalDeviceExternalBufferInfo, PhysicalDeviceExternalImageFormatInfo,
};
#[doc(alias = "VkExternalMemoryHandleTypeFlagsKHR")]
pub type ExternalMemoryHandleTypeFlagsKHR = ExternalMemoryHandleTypeFlags;
#[doc(alias = "VkExternalMemoryFeatureFlagsKHR")]
pub type ExternalMemoryFeatureFlagsKHR = ExternalMemoryFeatureFlags;
#[doc(alias = "VkExternalMemoryHandleTypeFlagBitsKHR")]
pub type ExternalMemoryHandleTypeFlagBitsKHR = ExternalMemoryHandleTypeFlagBits;
#[doc(alias = "VkExternalMemoryFeatureFlagBitsKHR")]
pub type ExternalMemoryFeatureFlagBitsKHR = ExternalMemoryFeatureFlagBits;
#[doc(alias = "VkExternalMemoryPropertiesKHR")]
pub type ExternalMemoryPropertiesKHR = ExternalMemoryProperties;
#[doc(alias = "VkPhysicalDeviceExternalImageFormatInfoKHR")]
pub type PhysicalDeviceExternalImageFormatInfoKHR = PhysicalDeviceExternalImageFormatInfo;
#[doc(alias = "VkExternalImageFormatPropertiesKHR")]
pub type ExternalImageFormatPropertiesKHR = ExternalImageFormatProperties;
#[doc(alias = "VkPhysicalDeviceExternalBufferInfoKHR")]
pub type PhysicalDeviceExternalBufferInfoKHR = PhysicalDeviceExternalBufferInfo;
#[doc(alias = "VkExternalBufferPropertiesKHR")]
pub type ExternalBufferPropertiesKHR = ExternalBufferProperties;
