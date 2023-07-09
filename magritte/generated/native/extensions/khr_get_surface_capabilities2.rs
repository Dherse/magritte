use crate::native::{
    extensions::khr_surface::{SurfaceCapabilitiesKHR, SurfaceFormatKHR, SurfaceKHR},
    vulkan1_0::{BaseInStructure, BaseOutStructure, PhysicalDevice, StructureType, VulkanResultCodes},
};
#[doc(alias = "VkPhysicalDeviceSurfaceInfo2KHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceSurfaceInfo2KHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub surface: SurfaceKHR,
}
impl Default for PhysicalDeviceSurfaceInfo2KHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceSurfaceInfo2Khr,
            p_next: unsafe { std::mem::zeroed() },
            surface: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkSurfaceCapabilities2KHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SurfaceCapabilities2KHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "surfaceCapabilities")]
    pub surface_capabilities: SurfaceCapabilitiesKHR,
}
impl Default for SurfaceCapabilities2KHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::SurfaceCapabilities2Khr,
            p_next: unsafe { std::mem::zeroed() },
            surface_capabilities: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkSurfaceFormat2KHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SurfaceFormat2KHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "surfaceFormat")]
    pub surface_format: SurfaceFormatKHR,
}
impl Default for SurfaceFormat2KHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::SurfaceFormat2Khr,
            p_next: unsafe { std::mem::zeroed() },
            surface_format: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::khr_get_surface_capabilities2::{
    KHR_GET_SURFACE_CAPABILITIES_2_EXTENSION_NAME, KHR_GET_SURFACE_CAPABILITIES_2_SPEC_VERSION,
};
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
