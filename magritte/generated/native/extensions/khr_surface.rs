pub use crate::common::extensions::khr_surface::{SurfaceCapabilitiesKHR, SurfaceFormatKHR};
use crate::native::vulkan1_0::{AllocationCallbacks, Bool32, Instance, PhysicalDevice, VulkanResultCodes};
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(alias = "VkSurfaceKHR")]
#[repr(transparent)]
pub struct SurfaceKHR(u64);
impl SurfaceKHR {
    pub const fn null() -> Self {
        Self(0)
    }
    pub const fn raw(&self) -> u64 {
        self.0
    }
}
impl Default for SurfaceKHR {
    fn default() -> Self {
        Self::null()
    }
}
pub use crate::common::extensions::khr_surface::{
    ColorSpaceKHR, CompositeAlphaFlagBitsKHR, CompositeAlphaFlagsKHR, PresentModeKHR, SurfaceTransformFlagBitsKHR,
    KHR_SURFACE_EXTENSION_NAME, KHR_SURFACE_SPEC_VERSION,
};
#[doc(alias = "vkDestroySurfaceKHR")]
pub type FNDestroySurfaceKhr =
    unsafe extern "system" fn(instance: Instance, surface: SurfaceKHR, p_allocator: *const AllocationCallbacks);
#[doc(alias = "vkGetPhysicalDeviceSurfaceSupportKHR")]
pub type FNGetPhysicalDeviceSurfaceSupportKhr = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    queue_family_index: u32,
    surface: SurfaceKHR,
    p_supported: *mut Bool32,
) -> VulkanResultCodes;
#[doc(alias = "vkGetPhysicalDeviceSurfaceCapabilitiesKHR")]
pub type FNGetPhysicalDeviceSurfaceCapabilitiesKhr = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    surface: SurfaceKHR,
    p_surface_capabilities: *mut SurfaceCapabilitiesKHR,
) -> VulkanResultCodes;
#[doc(alias = "vkGetPhysicalDeviceSurfaceFormatsKHR")]
pub type FNGetPhysicalDeviceSurfaceFormatsKhr = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    surface: SurfaceKHR,
    p_surface_format_count: *mut u32,
    p_surface_formats: *mut SurfaceFormatKHR,
) -> VulkanResultCodes;
#[doc(alias = "vkGetPhysicalDeviceSurfacePresentModesKHR")]
pub type FNGetPhysicalDeviceSurfacePresentModesKhr = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    surface: SurfaceKHR,
    p_present_mode_count: *mut u32,
    p_present_modes: *mut PresentModeKHR,
) -> VulkanResultCodes;
