pub use crate::common::extensions::khr_timeline_semaphore::{
    KHR_TIMELINE_SEMAPHORE_EXTENSION_NAME, KHR_TIMELINE_SEMAPHORE_SPEC_VERSION,
};
use crate::vulkan1_2::{
    PhysicalDeviceTimelineSemaphoreFeatures, PhysicalDeviceTimelineSemaphoreProperties, SemaphoreSignalInfo,
    SemaphoreType, SemaphoreTypeCreateInfo, SemaphoreWaitFlagBits, SemaphoreWaitFlags, SemaphoreWaitInfo,
    TimelineSemaphoreSubmitInfo,
};
#[doc(alias = "VkSemaphoreWaitFlagsKHR")]
pub type SemaphoreWaitFlagsKHR = SemaphoreWaitFlags;
#[doc(alias = "VkSemaphoreTypeKHR")]
pub type SemaphoreTypeKHR = SemaphoreType;
#[doc(alias = "VkSemaphoreWaitFlagBitsKHR")]
pub type SemaphoreWaitFlagBitsKHR = SemaphoreWaitFlagBits;
#[doc(alias = "VkPhysicalDeviceTimelineSemaphoreFeaturesKHR")]
pub type PhysicalDeviceTimelineSemaphoreFeaturesKHR = PhysicalDeviceTimelineSemaphoreFeatures;
#[doc(alias = "VkPhysicalDeviceTimelineSemaphorePropertiesKHR")]
pub type PhysicalDeviceTimelineSemaphorePropertiesKHR = PhysicalDeviceTimelineSemaphoreProperties;
#[doc(alias = "VkSemaphoreTypeCreateInfoKHR")]
pub type SemaphoreTypeCreateInfoKHR = SemaphoreTypeCreateInfo;
#[doc(alias = "VkTimelineSemaphoreSubmitInfoKHR")]
pub type TimelineSemaphoreSubmitInfoKHR = TimelineSemaphoreSubmitInfo;
#[doc(alias = "VkSemaphoreWaitInfoKHR")]
pub type SemaphoreWaitInfoKHR = SemaphoreWaitInfo;
#[doc(alias = "VkSemaphoreSignalInfoKHR")]
pub type SemaphoreSignalInfoKHR = SemaphoreSignalInfo;
