use crate::{
    cstr,
    extensions::khr_surface::{SurfaceCapabilitiesKHR, SurfaceFormatKHR, SurfaceKHR},
    vulkan1_0::{BaseInStructure, BaseOutStructure, PhysicalDevice, StructureType, VulkanResultCodes},
};
use std::ffi::CStr;
#[doc(alias = "VkPhysicalDeviceSurfaceInfo2KHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceSurfaceInfo2KHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    surface: SurfaceKHR,
}
#[doc(alias = "VkSurfaceCapabilities2KHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SurfaceCapabilities2KHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "surfaceCapabilities")]
    surface_capabilities: SurfaceCapabilitiesKHR,
}
#[doc(alias = "VkSurfaceFormat2KHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SurfaceFormat2KHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "surfaceFormat")]
    surface_format: SurfaceFormatKHR,
}
#[doc(alias = "VK_KHR_GET_SURFACE_CAPABILITIES_2_SPEC_VERSION")]
pub const KHR_GET_SURFACE_CAPABILITIES_2_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_KHR_GET_SURFACE_CAPABILITIES_2_EXTENSION_NAME")]
pub const KHR_GET_SURFACE_CAPABILITIES_2_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_get_surface_capabilities2");
#[doc(alias = "vkGetPhysicalDeviceSurfaceCapabilities2KHR")]
pub type FNGetPhysicalDeviceSurfaceCapabilities2Khr = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_surface_info: *const PhysicalDeviceSurfaceInfo2KHR,
    p_surface_capabilities: *mut SurfaceCapabilities2KHR,
) -> VulkanResultCodes;
#[doc(alias = "vkGetPhysicalDeviceSurfaceFormats2KHR")]
pub type FNGetPhysicalDeviceSurfaceFormats2Khr = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_surface_info: *const PhysicalDeviceSurfaceInfo2KHR,
    p_surface_format_count: *mut u32,
    p_surface_formats: *mut SurfaceFormat2KHR,
) -> VulkanResultCodes;
