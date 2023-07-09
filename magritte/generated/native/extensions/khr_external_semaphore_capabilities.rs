use crate::native::vulkan1_1::{
    ExternalSemaphoreFeatureFlagBits, ExternalSemaphoreFeatureFlags, ExternalSemaphoreHandleTypeFlagBits,
    ExternalSemaphoreHandleTypeFlags, ExternalSemaphoreProperties, FNGetPhysicalDeviceExternalSemaphoreProperties,
    PhysicalDeviceExternalSemaphoreInfo,
};
///See [`ExternalSemaphoreHandleTypeFlags`]
#[doc(alias = "VkExternalSemaphoreHandleTypeFlagsKHR")]
pub type ExternalSemaphoreHandleTypeFlagsKHR = ExternalSemaphoreHandleTypeFlags;
///See [`ExternalSemaphoreFeatureFlags`]
#[doc(alias = "VkExternalSemaphoreFeatureFlagsKHR")]
pub type ExternalSemaphoreFeatureFlagsKHR = ExternalSemaphoreFeatureFlags;
///See [`ExternalSemaphoreHandleTypeFlagBits`]
#[doc(alias = "VkExternalSemaphoreHandleTypeFlagBitsKHR")]
pub type ExternalSemaphoreHandleTypeFlagBitsKHR = ExternalSemaphoreHandleTypeFlagBits;
///See [`ExternalSemaphoreFeatureFlagBits`]
#[doc(alias = "VkExternalSemaphoreFeatureFlagBitsKHR")]
pub type ExternalSemaphoreFeatureFlagBitsKHR = ExternalSemaphoreFeatureFlagBits;
///See [`PhysicalDeviceExternalSemaphoreInfo`]
#[doc(alias = "VkPhysicalDeviceExternalSemaphoreInfoKHR")]
pub type PhysicalDeviceExternalSemaphoreInfoKHR = PhysicalDeviceExternalSemaphoreInfo;
///See [`ExternalSemaphoreProperties`]
#[doc(alias = "VkExternalSemaphorePropertiesKHR")]
pub type ExternalSemaphorePropertiesKHR = ExternalSemaphoreProperties;
pub use crate::common::extensions::khr_external_semaphore_capabilities::{
    KHR_EXTERNAL_SEMAPHORE_CAPABILITIES_EXTENSION_NAME, KHR_EXTERNAL_SEMAPHORE_CAPABILITIES_SPEC_VERSION,
};
///See [`get_physical_device_external_semaphore_properties`]
#[doc(alias = "vkGetPhysicalDeviceExternalSemaphorePropertiesKHR")]
pub type FNGetPhysicalDeviceExternalSemaphorePropertiesKhr = FNGetPhysicalDeviceExternalSemaphoreProperties;
