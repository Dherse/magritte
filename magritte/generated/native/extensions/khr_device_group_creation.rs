//!# [VK_KHR_device_group_creation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_device_group_creation.html)
# ! [doc = include_str ! ("../../../../doc/extensions/khr_device_group_creation/VK_KHR_device_group_creation.md")]
use crate::{
    cstr,
    vulkan1_0::MAX_DEVICE_GROUP_SIZE,
    vulkan1_1::{DeviceGroupDeviceCreateInfo, FNEnumeratePhysicalDeviceGroups, PhysicalDeviceGroupProperties},
};
use std::ffi::CStr;
///See [`PhysicalDeviceGroupProperties`]
#[doc(alias = "VkPhysicalDeviceGroupPropertiesKHR")]
pub type PhysicalDeviceGroupPropertiesKHR = PhysicalDeviceGroupProperties;
///See [`DeviceGroupDeviceCreateInfo`]
#[doc(alias = "VkDeviceGroupDeviceCreateInfoKHR")]
pub type DeviceGroupDeviceCreateInfoKHR = DeviceGroupDeviceCreateInfo;
#[doc(alias = "VK_KHR_DEVICE_GROUP_CREATION_SPEC_VERSION")]
pub const KHR_DEVICE_GROUP_CREATION_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_KHR_DEVICE_GROUP_CREATION_EXTENSION_NAME")]
pub const KHR_DEVICE_GROUP_CREATION_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_device_group_creation");
///See [`MAX_DEVICE_GROUP_SIZE`]
#[doc(alias = "VK_MAX_DEVICE_GROUP_SIZE_KHR")]
pub const MAX_DEVICE_GROUP_SIZE_KHR: u32 = MAX_DEVICE_GROUP_SIZE;
///See [`enumerate_physical_device_groups`]
#[doc(alias = "vkEnumeratePhysicalDeviceGroupsKHR")]
pub type FNEnumeratePhysicalDeviceGroupsKhr = FNEnumeratePhysicalDeviceGroups;
