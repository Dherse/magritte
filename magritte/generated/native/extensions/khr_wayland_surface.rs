//!# [VK_KHR_wayland_surface](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_wayland_surface.html)
# ! [doc = include_str ! ("../../../../doc/extensions/khr_wayland_surface/VK_KHR_wayland_surface.md")]
use crate::{
    cstr,
    extensions::khr_surface::SurfaceKHR,
    opaque::{wl_display, wl_surface},
    vulkan1_0::{
        AllocationCallbacks, BaseInStructure, Bool32, Instance, PhysicalDevice, StructureType, VulkanResultCodes,
    },
};
use std::ffi::CStr;
///# [VkWaylandSurfaceCreateInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkWaylandSurfaceCreateInfoKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_wayland_surface/VkWaylandSurfaceCreateInfoKHR.md")]
#[doc(alias = "VkWaylandSurfaceCreateInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct WaylandSurfaceCreateInfoKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    flags: WaylandSurfaceCreateFlagsKHR,
    display: *mut wl_display,
    surface: *mut wl_surface,
}
#[doc(alias = "VkWaylandSurfaceCreateFlagsKHR")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct WaylandSurfaceCreateFlagsKHR(u32);
impl WaylandSurfaceCreateFlagsKHR {
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
}
#[doc(alias = "VK_KHR_WAYLAND_SURFACE_SPEC_VERSION")]
pub const KHR_WAYLAND_SURFACE_SPEC_VERSION: u32 = 6;
#[doc(alias = "VK_KHR_WAYLAND_SURFACE_EXTENSION_NAME")]
pub const KHR_WAYLAND_SURFACE_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_wayland_surface");
///# [vkCreateWaylandSurfaceKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateWaylandSurfaceKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_wayland_surface/vkCreateWaylandSurfaceKHR.md")]
#[doc(alias = "vkCreateWaylandSurfaceKHR")]
pub type FNCreateWaylandSurfaceKhr = unsafe extern "system" fn(
    instance: Instance,
    p_create_info: *const WaylandSurfaceCreateInfoKHR,
    p_allocator: *const AllocationCallbacks,
    p_surface: *mut SurfaceKHR,
) -> VulkanResultCodes;
///# [vkGetPhysicalDeviceWaylandPresentationSupportKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceWaylandPresentationSupportKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_wayland_surface/vkGetPhysicalDeviceWaylandPresentationSupportKHR.md")]
#[doc(alias = "vkGetPhysicalDeviceWaylandPresentationSupportKHR")]
pub type FNGetPhysicalDeviceWaylandPresentationSupportKhr = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    queue_family_index: u32,
    display: *mut wl_display,
) -> Bool32;
