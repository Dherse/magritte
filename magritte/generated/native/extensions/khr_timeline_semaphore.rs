use crate::native::vulkan1_2::{
    FNGetSemaphoreCounterValue, FNSignalSemaphore, FNWaitSemaphores, PhysicalDeviceTimelineSemaphoreFeatures,
    PhysicalDeviceTimelineSemaphoreProperties, SemaphoreSignalInfo, SemaphoreType, SemaphoreTypeCreateInfo,
    SemaphoreWaitFlagBits, SemaphoreWaitFlags, SemaphoreWaitInfo, TimelineSemaphoreSubmitInfo,
};
///See [`SemaphoreWaitFlags`]
#[doc(alias = "VkSemaphoreWaitFlagsKHR")]
pub type SemaphoreWaitFlagsKHR = SemaphoreWaitFlags;
///See [`SemaphoreType`]
#[doc(alias = "VkSemaphoreTypeKHR")]
pub type SemaphoreTypeKHR = SemaphoreType;
///See [`SemaphoreWaitFlagBits`]
#[doc(alias = "VkSemaphoreWaitFlagBitsKHR")]
pub type SemaphoreWaitFlagBitsKHR = SemaphoreWaitFlagBits;
///See [`PhysicalDeviceTimelineSemaphoreFeatures`]
#[doc(alias = "VkPhysicalDeviceTimelineSemaphoreFeaturesKHR")]
pub type PhysicalDeviceTimelineSemaphoreFeaturesKHR = PhysicalDeviceTimelineSemaphoreFeatures;
///See [`PhysicalDeviceTimelineSemaphoreProperties`]
#[doc(alias = "VkPhysicalDeviceTimelineSemaphorePropertiesKHR")]
pub type PhysicalDeviceTimelineSemaphorePropertiesKHR = PhysicalDeviceTimelineSemaphoreProperties;
///See [`SemaphoreTypeCreateInfo`]
#[doc(alias = "VkSemaphoreTypeCreateInfoKHR")]
pub type SemaphoreTypeCreateInfoKHR = SemaphoreTypeCreateInfo;
///See [`TimelineSemaphoreSubmitInfo`]
#[doc(alias = "VkTimelineSemaphoreSubmitInfoKHR")]
pub type TimelineSemaphoreSubmitInfoKHR = TimelineSemaphoreSubmitInfo;
///See [`SemaphoreWaitInfo`]
#[doc(alias = "VkSemaphoreWaitInfoKHR")]
pub type SemaphoreWaitInfoKHR = SemaphoreWaitInfo;
///See [`SemaphoreSignalInfo`]
#[doc(alias = "VkSemaphoreSignalInfoKHR")]
pub type SemaphoreSignalInfoKHR = SemaphoreSignalInfo;
pub use crate::common::extensions::khr_timeline_semaphore::{
    KHR_TIMELINE_SEMAPHORE_EXTENSION_NAME, KHR_TIMELINE_SEMAPHORE_SPEC_VERSION,
};
///See [`get_semaphore_counter_value`]
#[doc(alias = "vkGetSemaphoreCounterValueKHR")]
pub type FNGetSemaphoreCounterValueKhr = FNGetSemaphoreCounterValue;
///See [`wait_semaphores`]
#[doc(alias = "vkWaitSemaphoresKHR")]
pub type FNWaitSemaphoresKhr = FNWaitSemaphores;
///See [`signal_semaphore`]
#[doc(alias = "vkSignalSemaphoreKHR")]
pub type FNSignalSemaphoreKhr = FNSignalSemaphore;
