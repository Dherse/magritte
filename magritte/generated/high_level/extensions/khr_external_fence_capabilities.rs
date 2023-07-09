pub use crate::common::extensions::khr_external_fence_capabilities::{
    KHR_EXTERNAL_FENCE_CAPABILITIES_EXTENSION_NAME, KHR_EXTERNAL_FENCE_CAPABILITIES_SPEC_VERSION, LUID_SIZE_KHR,
};
use crate::vulkan1_1::{
    ExternalFenceFeatureFlagBits, ExternalFenceFeatureFlags, ExternalFenceHandleTypeFlagBits,
    ExternalFenceHandleTypeFlags, ExternalFenceProperties, PhysicalDeviceExternalFenceInfo, PhysicalDeviceIdProperties,
};
#[doc(alias = "VkExternalFenceHandleTypeFlagsKHR")]
pub type ExternalFenceHandleTypeFlagsKHR = ExternalFenceHandleTypeFlags;
#[doc(alias = "VkExternalFenceFeatureFlagsKHR")]
pub type ExternalFenceFeatureFlagsKHR = ExternalFenceFeatureFlags;
#[doc(alias = "VkExternalFenceHandleTypeFlagBitsKHR")]
pub type ExternalFenceHandleTypeFlagBitsKHR = ExternalFenceHandleTypeFlagBits;
#[doc(alias = "VkExternalFenceFeatureFlagBitsKHR")]
pub type ExternalFenceFeatureFlagBitsKHR = ExternalFenceFeatureFlagBits;
#[doc(alias = "VkPhysicalDeviceIDPropertiesKHR")]
pub type PhysicalDeviceIdPropertiesKHR = PhysicalDeviceIdProperties;
#[doc(alias = "VkPhysicalDeviceExternalFenceInfoKHR")]
pub type PhysicalDeviceExternalFenceInfoKHR = PhysicalDeviceExternalFenceInfo;
#[doc(alias = "VkExternalFencePropertiesKHR")]
pub type ExternalFencePropertiesKHR = ExternalFenceProperties;
