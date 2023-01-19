//!# [VK_KHR_external_semaphore_capabilities](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_external_semaphore_capabilities.html)
# ! [doc = include_str ! ("../../../../doc/extensions/khr_external_semaphore_capabilities/VK_KHR_external_semaphore_capabilities.md")]
use crate::{
    cstr,
    vulkan1_1::{
        ExternalSemaphoreFeatureFlagBits, ExternalSemaphoreFeatureFlags, ExternalSemaphoreHandleTypeFlagBits,
        ExternalSemaphoreHandleTypeFlags, ExternalSemaphoreProperties, FNGetPhysicalDeviceExternalSemaphoreProperties,
        PhysicalDeviceExternalSemaphoreInfo,
    },
};
use std::ffi::CStr;
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
#[doc(alias = "VK_KHR_EXTERNAL_SEMAPHORE_CAPABILITIES_SPEC_VERSION")]
pub const KHR_EXTERNAL_SEMAPHORE_CAPABILITIES_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_KHR_EXTERNAL_SEMAPHORE_CAPABILITIES_EXTENSION_NAME")]
pub const KHR_EXTERNAL_SEMAPHORE_CAPABILITIES_EXTENSION_NAME: &'static CStr =
    cstr!("VK_KHR_external_semaphore_capabilities");
///See [`get_physical_device_external_semaphore_properties`]
#[doc(alias = "vkGetPhysicalDeviceExternalSemaphorePropertiesKHR")]
pub type FNGetPhysicalDeviceExternalSemaphorePropertiesKhr = FNGetPhysicalDeviceExternalSemaphoreProperties;
