use crate::{
    cstr,
    extensions::khr_display::{
        DisplayKHR, DisplayModeKHR, DisplayModePropertiesKHR, DisplayPlaneCapabilitiesKHR, DisplayPlanePropertiesKHR,
        DisplayPropertiesKHR,
    },
    vulkan1_0::{BaseInStructure, BaseOutStructure, PhysicalDevice, StructureType, VulkanResultCodes},
};
use std::ffi::CStr;
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
#[doc(alias = "vkGetPhysicalDeviceDisplayProperties2KHR")]
pub type FNGetPhysicalDeviceDisplayProperties2Khr = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_property_count: *mut u32,
    p_properties: *mut DisplayProperties2KHR,
) -> VulkanResultCodes;
#[doc(alias = "vkGetPhysicalDeviceDisplayPlaneProperties2KHR")]
pub type FNGetPhysicalDeviceDisplayPlaneProperties2Khr = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_property_count: *mut u32,
    p_properties: *mut DisplayPlaneProperties2KHR,
) -> VulkanResultCodes;
#[doc(alias = "vkGetDisplayModeProperties2KHR")]
pub type FNGetDisplayModeProperties2Khr = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    display: DisplayKHR,
    p_property_count: *mut u32,
    p_properties: *mut DisplayModeProperties2KHR,
) -> VulkanResultCodes;
#[doc(alias = "vkGetDisplayPlaneCapabilities2KHR")]
pub type FNGetDisplayPlaneCapabilities2Khr = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_display_plane_info: *const DisplayPlaneInfo2KHR,
    p_capabilities: *mut DisplayPlaneCapabilities2KHR,
) -> VulkanResultCodes;
