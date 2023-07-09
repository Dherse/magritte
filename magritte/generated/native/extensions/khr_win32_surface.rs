use crate::native::{
    extensions::khr_surface::SurfaceKHR,
    opaque::{HINSTANCE, HWND},
    vulkan1_0::{
        AllocationCallbacks, BaseInStructure, Bool32, Instance, PhysicalDevice, StructureType, VulkanResultCodes,
    },
};
#[doc(alias = "VkWin32SurfaceCreateInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct Win32SurfaceCreateInfoKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub flags: Win32SurfaceCreateFlagsKHR,
    pub hinstance: HINSTANCE,
    pub hwnd: HWND,
}
impl Default for Win32SurfaceCreateInfoKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::Win32SurfaceCreateInfoKhr,
            p_next: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
            hinstance: unsafe { std::mem::zeroed() },
            hwnd: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::khr_win32_surface::{
    Win32SurfaceCreateFlagsKHR, KHR_WIN32_SURFACE_EXTENSION_NAME, KHR_WIN32_SURFACE_SPEC_VERSION,
};
#[doc(alias = "vkCreateWin32SurfaceKHR")]
pub type FNCreateWin32SurfaceKhr = unsafe extern "system" fn(
    instance: Instance,
    p_create_info: *const Win32SurfaceCreateInfoKHR,
    p_allocator: *const AllocationCallbacks,
    p_surface: *mut SurfaceKHR,
) -> VulkanResultCodes;
#[doc(alias = "vkGetPhysicalDeviceWin32PresentationSupportKHR")]
pub type FNGetPhysicalDeviceWin32PresentationSupportKhr =
    unsafe extern "system" fn(physical_device: PhysicalDevice, queue_family_index: u32) -> Bool32;
