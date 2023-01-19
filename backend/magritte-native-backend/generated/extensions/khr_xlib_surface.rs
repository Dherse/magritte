//!# [VK_KHR_xlib_surface](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_xlib_surface.html)
# ! [doc = include_str ! ("../../../../doc/extensions/khr_xlib_surface/VK_KHR_xlib_surface.md")]
use crate::{
    cstr,
    extensions::khr_surface::SurfaceKHR,
    opaque::{Display, VisualID, Window},
    vulkan1_0::{
        AllocationCallbacks, BaseInStructure, Bool32, Instance, PhysicalDevice, StructureType, VulkanResultCodes,
    },
};
use std::ffi::CStr;
///# [VkXlibSurfaceCreateInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkXlibSurfaceCreateInfoKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_xlib_surface/VkXlibSurfaceCreateInfoKHR.md")]
#[doc(alias = "VkXlibSurfaceCreateInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct XlibSurfaceCreateInfoKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    flags: XlibSurfaceCreateFlagsKHR,
    dpy: *mut Display,
    window: Window,
}
#[doc(alias = "VkXlibSurfaceCreateFlagsKHR")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct XlibSurfaceCreateFlagsKHR(u32);
impl XlibSurfaceCreateFlagsKHR {
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
}
#[doc(alias = "VK_KHR_XLIB_SURFACE_SPEC_VERSION")]
pub const KHR_XLIB_SURFACE_SPEC_VERSION: u32 = 6;
#[doc(alias = "VK_KHR_XLIB_SURFACE_EXTENSION_NAME")]
pub const KHR_XLIB_SURFACE_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_xlib_surface");
///# [vkCreateXlibSurfaceKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateXlibSurfaceKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_xlib_surface/vkCreateXlibSurfaceKHR.md")]
#[doc(alias = "vkCreateXlibSurfaceKHR")]
pub type FNCreateXlibSurfaceKhr = unsafe extern "system" fn(
    instance: Instance,
    p_create_info: *const XlibSurfaceCreateInfoKHR,
    p_allocator: *const AllocationCallbacks,
    p_surface: *mut SurfaceKHR,
) -> VulkanResultCodes;
///# [vkGetPhysicalDeviceXlibPresentationSupportKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceXlibPresentationSupportKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_xlib_surface/vkGetPhysicalDeviceXlibPresentationSupportKHR.md")]
#[doc(alias = "vkGetPhysicalDeviceXlibPresentationSupportKHR")]
pub type FNGetPhysicalDeviceXlibPresentationSupportKhr = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    queue_family_index: u32,
    dpy: *mut Display,
    visual_id: VisualID,
) -> Bool32;
