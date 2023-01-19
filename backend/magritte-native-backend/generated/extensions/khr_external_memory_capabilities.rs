//!# [VK_KHR_external_memory_capabilities](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_external_memory_capabilities.html)
# ! [doc = include_str ! ("../../../../doc/extensions/khr_external_memory_capabilities/VK_KHR_external_memory_capabilities.md")]
use crate::{
    cstr,
    vulkan1_1::{
        ExternalBufferProperties, ExternalImageFormatProperties, ExternalMemoryFeatureFlagBits,
        ExternalMemoryFeatureFlags, ExternalMemoryHandleTypeFlagBits, ExternalMemoryHandleTypeFlags,
        ExternalMemoryProperties, FNGetPhysicalDeviceExternalBufferProperties, PhysicalDeviceExternalBufferInfo,
        PhysicalDeviceExternalImageFormatInfo,
    },
};
use std::ffi::CStr;
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
#[doc(alias = "VK_KHR_EXTERNAL_MEMORY_CAPABILITIES_SPEC_VERSION")]
pub const KHR_EXTERNAL_MEMORY_CAPABILITIES_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_KHR_EXTERNAL_MEMORY_CAPABILITIES_EXTENSION_NAME")]
pub const KHR_EXTERNAL_MEMORY_CAPABILITIES_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_external_memory_capabilities");
///See [`get_physical_device_external_buffer_properties`]
#[doc(alias = "vkGetPhysicalDeviceExternalBufferPropertiesKHR")]
pub type FNGetPhysicalDeviceExternalBufferPropertiesKhr = FNGetPhysicalDeviceExternalBufferProperties;
