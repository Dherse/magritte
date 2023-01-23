//!# [VK_KHR_external_fence_capabilities](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_external_fence_capabilities.html)
# ! [doc = include_str ! ("../../../../doc/extensions/khr_external_fence_capabilities/VK_KHR_external_fence_capabilities.md")]
use crate::{
    cstr,
    vulkan1_0::LUID_SIZE,
    vulkan1_1::{
        ExternalFenceFeatureFlagBits, ExternalFenceFeatureFlags, ExternalFenceHandleTypeFlagBits,
        ExternalFenceHandleTypeFlags, ExternalFenceProperties, FNGetPhysicalDeviceExternalFenceProperties,
        PhysicalDeviceExternalFenceInfo, PhysicalDeviceIdProperties,
    },
};
use std::ffi::CStr;
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
#[doc(alias = "VK_KHR_EXTERNAL_FENCE_CAPABILITIES_SPEC_VERSION")]
pub const KHR_EXTERNAL_FENCE_CAPABILITIES_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_KHR_EXTERNAL_FENCE_CAPABILITIES_EXTENSION_NAME")]
pub const KHR_EXTERNAL_FENCE_CAPABILITIES_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_external_fence_capabilities");
///See [`LUID_SIZE`]
#[doc(alias = "VK_LUID_SIZE_KHR")]
pub const LUID_SIZE_KHR: u32 = LUID_SIZE;
///See [`get_physical_device_external_fence_properties`]
#[doc(alias = "vkGetPhysicalDeviceExternalFencePropertiesKHR")]
pub type FNGetPhysicalDeviceExternalFencePropertiesKhr = FNGetPhysicalDeviceExternalFenceProperties;
