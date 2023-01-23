//!# [VK_KHR_timeline_semaphore](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_timeline_semaphore.html)
# ! [doc = include_str ! ("../../../../doc/extensions/khr_timeline_semaphore/VK_KHR_timeline_semaphore.md")]
use crate::{
    cstr,
    vulkan1_2::{
        FNGetSemaphoreCounterValue, FNSignalSemaphore, FNWaitSemaphores, PhysicalDeviceTimelineSemaphoreFeatures,
        PhysicalDeviceTimelineSemaphoreProperties, SemaphoreSignalInfo, SemaphoreType, SemaphoreTypeCreateInfo,
        SemaphoreWaitFlagBits, SemaphoreWaitFlags, SemaphoreWaitInfo, TimelineSemaphoreSubmitInfo,
    },
};
use std::ffi::CStr;
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
#[doc(alias = "VK_KHR_TIMELINE_SEMAPHORE_SPEC_VERSION")]
pub const KHR_TIMELINE_SEMAPHORE_SPEC_VERSION: u32 = 2;
#[doc(alias = "VK_KHR_TIMELINE_SEMAPHORE_EXTENSION_NAME")]
pub const KHR_TIMELINE_SEMAPHORE_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_timeline_semaphore");
///See [`get_semaphore_counter_value`]
#[doc(alias = "vkGetSemaphoreCounterValueKHR")]
pub type FNGetSemaphoreCounterValueKhr = FNGetSemaphoreCounterValue;
///See [`wait_semaphores`]
#[doc(alias = "vkWaitSemaphoresKHR")]
pub type FNWaitSemaphoresKhr = FNWaitSemaphores;
///See [`signal_semaphore`]
#[doc(alias = "vkSignalSemaphoreKHR")]
pub type FNSignalSemaphoreKhr = FNSignalSemaphore;
