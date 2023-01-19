//!# [VK_KHR_get_display_properties2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_get_display_properties2.html)
# ! [doc = include_str ! ("../../../../doc/extensions/khr_get_display_properties2/VK_KHR_get_display_properties2.md")]
use crate::{
    cstr,
    extensions::khr_display::{
        DisplayKHR, DisplayModeKHR, DisplayModePropertiesKHR, DisplayPlaneCapabilitiesKHR, DisplayPlanePropertiesKHR,
        DisplayPropertiesKHR,
    },
    vulkan1_0::{BaseInStructure, BaseOutStructure, PhysicalDevice, StructureType, VulkanResultCodes},
};
use std::ffi::CStr;
///# [VkDisplayProperties2KHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDisplayProperties2KHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_get_display_properties2/VkDisplayProperties2KHR.md")]
#[doc(alias = "VkDisplayProperties2KHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DisplayProperties2KHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "displayProperties")]
    display_properties: DisplayPropertiesKHR,
}
///# [VkDisplayPlaneProperties2KHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDisplayPlaneProperties2KHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_get_display_properties2/VkDisplayPlaneProperties2KHR.md")]
#[doc(alias = "VkDisplayPlaneProperties2KHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DisplayPlaneProperties2KHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "displayPlaneProperties")]
    display_plane_properties: DisplayPlanePropertiesKHR,
}
///# [VkDisplayModeProperties2KHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDisplayModeProperties2KHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_get_display_properties2/VkDisplayModeProperties2KHR.md")]
#[doc(alias = "VkDisplayModeProperties2KHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DisplayModeProperties2KHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "displayModeProperties")]
    display_mode_properties: DisplayModePropertiesKHR,
}
///# [VkDisplayPlaneInfo2KHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDisplayPlaneInfo2KHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_get_display_properties2/VkDisplayPlaneInfo2KHR.md")]
#[doc(alias = "VkDisplayPlaneInfo2KHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DisplayPlaneInfo2KHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    mode: DisplayModeKHR,
    #[doc(alias = "planeIndex")]
    plane_index: u32,
}
///# [VkDisplayPlaneCapabilities2KHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDisplayPlaneCapabilities2KHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_get_display_properties2/VkDisplayPlaneCapabilities2KHR.md")]
#[doc(alias = "VkDisplayPlaneCapabilities2KHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DisplayPlaneCapabilities2KHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    capabilities: DisplayPlaneCapabilitiesKHR,
}
#[doc(alias = "VK_KHR_GET_DISPLAY_PROPERTIES_2_SPEC_VERSION")]
pub const KHR_GET_DISPLAY_PROPERTIES_2_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_KHR_GET_DISPLAY_PROPERTIES_2_EXTENSION_NAME")]
pub const KHR_GET_DISPLAY_PROPERTIES_2_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_get_display_properties2");
///# [vkGetPhysicalDeviceDisplayProperties2KHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceDisplayProperties2KHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_get_display_properties2/vkGetPhysicalDeviceDisplayProperties2KHR.md")]
#[doc(alias = "vkGetPhysicalDeviceDisplayProperties2KHR")]
pub type FNGetPhysicalDeviceDisplayProperties2Khr = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_property_count: *mut u32,
    p_properties: *mut DisplayProperties2KHR,
) -> VulkanResultCodes;
///# [vkGetPhysicalDeviceDisplayPlaneProperties2KHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceDisplayPlaneProperties2KHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_get_display_properties2/vkGetPhysicalDeviceDisplayPlaneProperties2KHR.md")]
#[doc(alias = "vkGetPhysicalDeviceDisplayPlaneProperties2KHR")]
pub type FNGetPhysicalDeviceDisplayPlaneProperties2Khr = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_property_count: *mut u32,
    p_properties: *mut DisplayPlaneProperties2KHR,
) -> VulkanResultCodes;
///# [vkGetDisplayModeProperties2KHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDisplayModeProperties2KHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_get_display_properties2/vkGetDisplayModeProperties2KHR.md")]
#[doc(alias = "vkGetDisplayModeProperties2KHR")]
pub type FNGetDisplayModeProperties2Khr = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    display: DisplayKHR,
    p_property_count: *mut u32,
    p_properties: *mut DisplayModeProperties2KHR,
) -> VulkanResultCodes;
///# [vkGetDisplayPlaneCapabilities2KHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDisplayPlaneCapabilities2KHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_get_display_properties2/vkGetDisplayPlaneCapabilities2KHR.md")]
#[doc(alias = "vkGetDisplayPlaneCapabilities2KHR")]
pub type FNGetDisplayPlaneCapabilities2Khr = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_display_plane_info: *const DisplayPlaneInfo2KHR,
    p_capabilities: *mut DisplayPlaneCapabilities2KHR,
) -> VulkanResultCodes;
