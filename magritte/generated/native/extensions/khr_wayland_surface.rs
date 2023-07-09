use crate::native::{
    extensions::khr_surface::SurfaceKHR,
    opaque::{wl_display, wl_surface},
    vulkan1_0::{
        AllocationCallbacks, BaseInStructure, Bool32, Instance, PhysicalDevice, StructureType, VulkanResultCodes,
    },
};
#[doc(alias = "VkWaylandSurfaceCreateInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct WaylandSurfaceCreateInfoKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub flags: WaylandSurfaceCreateFlagsKHR,
    pub display: *mut wl_display,
    pub surface: *mut wl_surface,
}
impl Default for WaylandSurfaceCreateInfoKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::WaylandSurfaceCreateInfoKhr,
            p_next: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
            display: unsafe { std::mem::zeroed() },
            surface: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::khr_wayland_surface::{
    WaylandSurfaceCreateFlagsKHR, KHR_WAYLAND_SURFACE_EXTENSION_NAME, KHR_WAYLAND_SURFACE_SPEC_VERSION,
};
#[doc(alias = "vkCreateWaylandSurfaceKHR")]
pub type FNCreateWaylandSurfaceKhr = unsafe extern "system" fn(
    instance: Instance,
    p_create_info: *const WaylandSurfaceCreateInfoKHR,
    p_allocator: *const AllocationCallbacks,
    p_surface: *mut SurfaceKHR,
) -> VulkanResultCodes;
#[doc(alias = "vkGetPhysicalDeviceWaylandPresentationSupportKHR")]
pub type FNGetPhysicalDeviceWaylandPresentationSupportKhr = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    queue_family_index: u32,
    display: *mut wl_display,
) -> Bool32;
