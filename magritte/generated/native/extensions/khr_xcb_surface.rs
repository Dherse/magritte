//!# [VK_KHR_xcb_surface](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_xcb_surface.html)
# ! [doc = include_str ! ("../../../../doc/extensions/khr_xcb_surface/VK_KHR_xcb_surface.md")]
use crate::{
    cstr,
    extensions::khr_surface::SurfaceKHR,
    opaque::{xcb_connection_t, xcb_visualid_t, xcb_window_t},
    vulkan1_0::{
        AllocationCallbacks, BaseInStructure, Bool32, Instance, PhysicalDevice, StructureType, VulkanResultCodes,
    },
};
use std::ffi::CStr;
///# [VkXcbSurfaceCreateInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkXcbSurfaceCreateInfoKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_xcb_surface/VkXcbSurfaceCreateInfoKHR.md")]
#[doc(alias = "VkXcbSurfaceCreateInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct XcbSurfaceCreateInfoKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    flags: XcbSurfaceCreateFlagsKHR,
    connection: *mut xcb_connection_t,
    window: xcb_window_t,
}
#[doc(alias = "VkXcbSurfaceCreateFlagsKHR")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct XcbSurfaceCreateFlagsKHR(u32);
impl XcbSurfaceCreateFlagsKHR {
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
}
#[doc(alias = "VK_KHR_XCB_SURFACE_SPEC_VERSION")]
pub const KHR_XCB_SURFACE_SPEC_VERSION: u32 = 6;
#[doc(alias = "VK_KHR_XCB_SURFACE_EXTENSION_NAME")]
pub const KHR_XCB_SURFACE_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_xcb_surface");
///# [vkCreateXcbSurfaceKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateXcbSurfaceKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_xcb_surface/vkCreateXcbSurfaceKHR.md")]
#[doc(alias = "vkCreateXcbSurfaceKHR")]
pub type FNCreateXcbSurfaceKhr = unsafe extern "system" fn(
    instance: Instance,
    p_create_info: *const XcbSurfaceCreateInfoKHR,
    p_allocator: *const AllocationCallbacks,
    p_surface: *mut SurfaceKHR,
) -> VulkanResultCodes;
///# [vkGetPhysicalDeviceXcbPresentationSupportKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceXcbPresentationSupportKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_xcb_surface/vkGetPhysicalDeviceXcbPresentationSupportKHR.md")]
#[doc(alias = "vkGetPhysicalDeviceXcbPresentationSupportKHR")]
pub type FNGetPhysicalDeviceXcbPresentationSupportKhr = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    queue_family_index: u32,
    connection: *mut xcb_connection_t,
    visual_id: xcb_visualid_t,
) -> Bool32;
