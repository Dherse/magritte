use crate::native::{
    extensions::khr_surface::SurfaceKHR,
    opaque::{xcb_connection_t, xcb_visualid_t, xcb_window_t},
    vulkan1_0::{
        AllocationCallbacks, BaseInStructure, Bool32, Instance, PhysicalDevice, StructureType, VulkanResultCodes,
    },
};
#[doc(alias = "VkXcbSurfaceCreateInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct XcbSurfaceCreateInfoKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub flags: XcbSurfaceCreateFlagsKHR,
    pub connection: *mut xcb_connection_t,
    pub window: xcb_window_t,
}
impl Default for XcbSurfaceCreateInfoKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::XcbSurfaceCreateInfoKhr,
            p_next: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
            connection: unsafe { std::mem::zeroed() },
            window: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::khr_xcb_surface::{
    XcbSurfaceCreateFlagsKHR, KHR_XCB_SURFACE_EXTENSION_NAME, KHR_XCB_SURFACE_SPEC_VERSION,
};
#[doc(alias = "vkCreateXcbSurfaceKHR")]
pub type FNCreateXcbSurfaceKhr = unsafe extern "system" fn(
    instance: Instance,
    p_create_info: *const XcbSurfaceCreateInfoKHR,
    p_allocator: *const AllocationCallbacks,
    p_surface: *mut SurfaceKHR,
) -> VulkanResultCodes;
#[doc(alias = "vkGetPhysicalDeviceXcbPresentationSupportKHR")]
pub type FNGetPhysicalDeviceXcbPresentationSupportKhr = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    queue_family_index: u32,
    connection: *mut xcb_connection_t,
    visual_id: xcb_visualid_t,
) -> Bool32;
