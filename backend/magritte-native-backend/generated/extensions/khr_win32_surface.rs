//!# [VK_KHR_win32_surface](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_win32_surface.html)
# ! [doc = include_str ! ("../../../../doc/extensions/khr_win32_surface/VK_KHR_win32_surface.md")]
use crate::{
    cstr,
    extensions::khr_surface::SurfaceKHR,
    opaque::{HINSTANCE, HWND},
    vulkan1_0::{
        AllocationCallbacks, BaseInStructure, Bool32, Instance, PhysicalDevice, StructureType, VulkanResultCodes,
    },
};
use std::ffi::CStr;
///# [VkWin32SurfaceCreateInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkWin32SurfaceCreateInfoKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_win32_surface/VkWin32SurfaceCreateInfoKHR.md")]
#[doc(alias = "VkWin32SurfaceCreateInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct Win32SurfaceCreateInfoKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    flags: Win32SurfaceCreateFlagsKHR,
    hinstance: HINSTANCE,
    hwnd: HWND,
}
#[doc(alias = "VkWin32SurfaceCreateFlagsKHR")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Win32SurfaceCreateFlagsKHR(u32);
impl Win32SurfaceCreateFlagsKHR {
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
}
#[doc(alias = "VK_KHR_WIN32_SURFACE_SPEC_VERSION")]
pub const KHR_WIN32_SURFACE_SPEC_VERSION: u32 = 6;
#[doc(alias = "VK_KHR_WIN32_SURFACE_EXTENSION_NAME")]
pub const KHR_WIN32_SURFACE_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_win32_surface");
///# [vkCreateWin32SurfaceKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateWin32SurfaceKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_win32_surface/vkCreateWin32SurfaceKHR.md")]
#[doc(alias = "vkCreateWin32SurfaceKHR")]
pub type FNCreateWin32SurfaceKhr = unsafe extern "system" fn(
    instance: Instance,
    p_create_info: *const Win32SurfaceCreateInfoKHR,
    p_allocator: *const AllocationCallbacks,
    p_surface: *mut SurfaceKHR,
) -> VulkanResultCodes;
///# [vkGetPhysicalDeviceWin32PresentationSupportKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceWin32PresentationSupportKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_win32_surface/vkGetPhysicalDeviceWin32PresentationSupportKHR.md")]
#[doc(alias = "vkGetPhysicalDeviceWin32PresentationSupportKHR")]
pub type FNGetPhysicalDeviceWin32PresentationSupportKhr =
    unsafe extern "system" fn(physical_device: PhysicalDevice, queue_family_index: u32) -> Bool32;
