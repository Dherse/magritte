//!# [VK_KHR_driver_properties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_driver_properties.html)
# ! [doc = include_str ! ("../../../../doc/extensions/khr_driver_properties/VK_KHR_driver_properties.md")]
use crate::{
    cstr,
    vulkan1_0::{MAX_DRIVER_INFO_SIZE, MAX_DRIVER_NAME_SIZE},
    vulkan1_2::{ConformanceVersion, DriverId, PhysicalDeviceDriverProperties},
};
use std::ffi::CStr;
///See [`DriverId`]
#[doc(alias = "VkDriverIdKHR")]
pub type DriverIdKHR = DriverId;
///See [`ConformanceVersion`]
#[doc(alias = "VkConformanceVersionKHR")]
pub type ConformanceVersionKHR = ConformanceVersion;
///See [`PhysicalDeviceDriverProperties`]
#[doc(alias = "VkPhysicalDeviceDriverPropertiesKHR")]
pub type PhysicalDeviceDriverPropertiesKHR = PhysicalDeviceDriverProperties;
#[doc(alias = "VK_KHR_DRIVER_PROPERTIES_SPEC_VERSION")]
pub const KHR_DRIVER_PROPERTIES_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_KHR_DRIVER_PROPERTIES_EXTENSION_NAME")]
pub const KHR_DRIVER_PROPERTIES_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_driver_properties");
///See [`MAX_DRIVER_NAME_SIZE`]
#[doc(alias = "VK_MAX_DRIVER_NAME_SIZE_KHR")]
pub const MAX_DRIVER_NAME_SIZE_KHR: u32 = MAX_DRIVER_NAME_SIZE;
///See [`MAX_DRIVER_INFO_SIZE`]
#[doc(alias = "VK_MAX_DRIVER_INFO_SIZE_KHR")]
pub const MAX_DRIVER_INFO_SIZE_KHR: u32 = MAX_DRIVER_INFO_SIZE;
