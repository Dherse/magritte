pub use crate::common::extensions::khr_external_semaphore_capabilities::{
    KHR_EXTERNAL_SEMAPHORE_CAPABILITIES_EXTENSION_NAME, KHR_EXTERNAL_SEMAPHORE_CAPABILITIES_SPEC_VERSION,
};
use crate::vulkan1_1::{
    ExternalSemaphoreFeatureFlagBits, ExternalSemaphoreFeatureFlags, ExternalSemaphoreHandleTypeFlagBits,
    ExternalSemaphoreHandleTypeFlags, ExternalSemaphoreProperties, PhysicalDeviceExternalSemaphoreInfo,
};
#[doc(alias = "VkExternalSemaphoreHandleTypeFlagsKHR")]
pub type ExternalSemaphoreHandleTypeFlagsKHR = ExternalSemaphoreHandleTypeFlags;
#[doc(alias = "VkExternalSemaphoreFeatureFlagsKHR")]
pub type ExternalSemaphoreFeatureFlagsKHR = ExternalSemaphoreFeatureFlags;
#[doc(alias = "VkExternalSemaphoreHandleTypeFlagBitsKHR")]
pub type ExternalSemaphoreHandleTypeFlagBitsKHR = ExternalSemaphoreHandleTypeFlagBits;
#[doc(alias = "VkExternalSemaphoreFeatureFlagBitsKHR")]
pub type ExternalSemaphoreFeatureFlagBitsKHR = ExternalSemaphoreFeatureFlagBits;
#[doc(alias = "VkPhysicalDeviceExternalSemaphoreInfoKHR")]
pub type PhysicalDeviceExternalSemaphoreInfoKHR = PhysicalDeviceExternalSemaphoreInfo;
#[doc(alias = "VkExternalSemaphorePropertiesKHR")]
pub type ExternalSemaphorePropertiesKHR = ExternalSemaphoreProperties;
