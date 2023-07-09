use crate::native::vulkan1_1::{
    ExternalBufferProperties, ExternalImageFormatProperties, ExternalMemoryFeatureFlagBits, ExternalMemoryFeatureFlags,
    ExternalMemoryHandleTypeFlagBits, ExternalMemoryHandleTypeFlags, ExternalMemoryProperties,
    FNGetPhysicalDeviceExternalBufferProperties, PhysicalDeviceExternalBufferInfo,
    PhysicalDeviceExternalImageFormatInfo,
};
///See [`ExternalMemoryHandleTypeFlags`]
#[doc(alias = "VkExternalMemoryHandleTypeFlagsKHR")]
pub type ExternalMemoryHandleTypeFlagsKHR = ExternalMemoryHandleTypeFlags;
///See [`ExternalMemoryFeatureFlags`]
#[doc(alias = "VkExternalMemoryFeatureFlagsKHR")]
pub type ExternalMemoryFeatureFlagsKHR = ExternalMemoryFeatureFlags;
///See [`ExternalMemoryHandleTypeFlagBits`]
#[doc(alias = "VkExternalMemoryHandleTypeFlagBitsKHR")]
pub type ExternalMemoryHandleTypeFlagBitsKHR = ExternalMemoryHandleTypeFlagBits;
///See [`ExternalMemoryFeatureFlagBits`]
#[doc(alias = "VkExternalMemoryFeatureFlagBitsKHR")]
pub type ExternalMemoryFeatureFlagBitsKHR = ExternalMemoryFeatureFlagBits;
///See [`ExternalMemoryProperties`]
#[doc(alias = "VkExternalMemoryPropertiesKHR")]
pub type ExternalMemoryPropertiesKHR = ExternalMemoryProperties;
///See [`PhysicalDeviceExternalImageFormatInfo`]
#[doc(alias = "VkPhysicalDeviceExternalImageFormatInfoKHR")]
pub type PhysicalDeviceExternalImageFormatInfoKHR = PhysicalDeviceExternalImageFormatInfo;
///See [`ExternalImageFormatProperties`]
#[doc(alias = "VkExternalImageFormatPropertiesKHR")]
pub type ExternalImageFormatPropertiesKHR = ExternalImageFormatProperties;
///See [`PhysicalDeviceExternalBufferInfo`]
#[doc(alias = "VkPhysicalDeviceExternalBufferInfoKHR")]
pub type PhysicalDeviceExternalBufferInfoKHR = PhysicalDeviceExternalBufferInfo;
///See [`ExternalBufferProperties`]
#[doc(alias = "VkExternalBufferPropertiesKHR")]
pub type ExternalBufferPropertiesKHR = ExternalBufferProperties;
pub use crate::common::extensions::khr_external_memory_capabilities::{
    KHR_EXTERNAL_MEMORY_CAPABILITIES_EXTENSION_NAME, KHR_EXTERNAL_MEMORY_CAPABILITIES_SPEC_VERSION,
};
///See [`get_physical_device_external_buffer_properties`]
#[doc(alias = "vkGetPhysicalDeviceExternalBufferPropertiesKHR")]
pub type FNGetPhysicalDeviceExternalBufferPropertiesKhr = FNGetPhysicalDeviceExternalBufferProperties;
