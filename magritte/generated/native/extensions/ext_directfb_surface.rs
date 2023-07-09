use crate::native::{
    extensions::khr_surface::SurfaceKHR,
    opaque::{IDirectFB, IDirectFBSurface},
    vulkan1_0::{
        AllocationCallbacks, BaseInStructure, Bool32, Instance, PhysicalDevice, StructureType, VulkanResultCodes,
    },
};
#[doc(alias = "VkDirectFBSurfaceCreateInfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DirectFBSurfaceCreateInfoEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub flags: DirectFBSurfaceCreateFlagsEXT,
    pub dfb: *mut IDirectFB,
    pub surface: *mut IDirectFBSurface,
}
impl Default for DirectFBSurfaceCreateInfoEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::DirectfbSurfaceCreateInfoExt,
            p_next: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
            dfb: unsafe { std::mem::zeroed() },
            surface: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::ext_directfb_surface::{
    DirectFBSurfaceCreateFlagsEXT, EXT_DIRECTFB_SURFACE_EXTENSION_NAME, EXT_DIRECTFB_SURFACE_SPEC_VERSION,
};
#[doc(alias = "vkCreateDirectFBSurfaceEXT")]
pub type FNCreateDirectFbSurfaceExt = unsafe extern "system" fn(
    instance: Instance,
    p_create_info: *const DirectFBSurfaceCreateInfoEXT,
    p_allocator: *const AllocationCallbacks,
    p_surface: *mut SurfaceKHR,
) -> VulkanResultCodes;
#[doc(alias = "vkGetPhysicalDeviceDirectFBPresentationSupportEXT")]
pub type FNGetPhysicalDeviceDirectFbPresentationSupportExt =
    unsafe extern "system" fn(physical_device: PhysicalDevice, queue_family_index: u32, dfb: *mut IDirectFB) -> Bool32;
