use crate::native::vulkan1_1::{
    ExternalFenceFeatureFlagBits, ExternalFenceFeatureFlags, ExternalFenceHandleTypeFlagBits,
    ExternalFenceHandleTypeFlags, ExternalFenceProperties, FNGetPhysicalDeviceExternalFenceProperties,
    PhysicalDeviceExternalFenceInfo, PhysicalDeviceIdProperties,
};
///See [`ExternalFenceHandleTypeFlags`]
#[doc(alias = "VkExternalFenceHandleTypeFlagsKHR")]
pub type ExternalFenceHandleTypeFlagsKHR = ExternalFenceHandleTypeFlags;
///See [`ExternalFenceFeatureFlags`]
#[doc(alias = "VkExternalFenceFeatureFlagsKHR")]
pub type ExternalFenceFeatureFlagsKHR = ExternalFenceFeatureFlags;
///See [`ExternalFenceHandleTypeFlagBits`]
#[doc(alias = "VkExternalFenceHandleTypeFlagBitsKHR")]
pub type ExternalFenceHandleTypeFlagBitsKHR = ExternalFenceHandleTypeFlagBits;
///See [`ExternalFenceFeatureFlagBits`]
#[doc(alias = "VkExternalFenceFeatureFlagBitsKHR")]
pub type ExternalFenceFeatureFlagBitsKHR = ExternalFenceFeatureFlagBits;
///See [`PhysicalDeviceIdProperties`]
#[doc(alias = "VkPhysicalDeviceIDPropertiesKHR")]
pub type PhysicalDeviceIdPropertiesKHR = PhysicalDeviceIdProperties;
///See [`PhysicalDeviceExternalFenceInfo`]
#[doc(alias = "VkPhysicalDeviceExternalFenceInfoKHR")]
pub type PhysicalDeviceExternalFenceInfoKHR = PhysicalDeviceExternalFenceInfo;
///See [`ExternalFenceProperties`]
#[doc(alias = "VkExternalFencePropertiesKHR")]
pub type ExternalFencePropertiesKHR = ExternalFenceProperties;
pub use crate::common::extensions::khr_external_fence_capabilities::{
    KHR_EXTERNAL_FENCE_CAPABILITIES_EXTENSION_NAME, KHR_EXTERNAL_FENCE_CAPABILITIES_SPEC_VERSION, LUID_SIZE_KHR,
};
///See [`get_physical_device_external_fence_properties`]
#[doc(alias = "vkGetPhysicalDeviceExternalFencePropertiesKHR")]
pub type FNGetPhysicalDeviceExternalFencePropertiesKhr = FNGetPhysicalDeviceExternalFenceProperties;
