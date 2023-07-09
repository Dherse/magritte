use crate::native::{
    extensions::khr_surface::SurfaceKHR,
    opaque::{Display, VisualID, Window},
    vulkan1_0::{
        AllocationCallbacks, BaseInStructure, Bool32, Instance, PhysicalDevice, StructureType, VulkanResultCodes,
    },
};
#[doc(alias = "VkXlibSurfaceCreateInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct XlibSurfaceCreateInfoKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub flags: XlibSurfaceCreateFlagsKHR,
    pub dpy: *mut Display,
    pub window: Window,
}
impl Default for XlibSurfaceCreateInfoKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::XlibSurfaceCreateInfoKhr,
            p_next: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
            dpy: unsafe { std::mem::zeroed() },
            window: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::khr_xlib_surface::{
    XlibSurfaceCreateFlagsKHR, KHR_XLIB_SURFACE_EXTENSION_NAME, KHR_XLIB_SURFACE_SPEC_VERSION,
};
#[doc(alias = "vkCreateXlibSurfaceKHR")]
pub type FNCreateXlibSurfaceKhr = unsafe extern "system" fn(
    instance: Instance,
    p_create_info: *const XlibSurfaceCreateInfoKHR,
    p_allocator: *const AllocationCallbacks,
    p_surface: *mut SurfaceKHR,
) -> VulkanResultCodes;
#[doc(alias = "vkGetPhysicalDeviceXlibPresentationSupportKHR")]
pub type FNGetPhysicalDeviceXlibPresentationSupportKhr = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    queue_family_index: u32,
    dpy: *mut Display,
    visual_id: VisualID,
) -> Bool32;
