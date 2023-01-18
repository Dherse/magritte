use crate::{
    cstr,
    extensions::khr_surface::SurfaceKHR,
    opaque::{IDirectFB, IDirectFBSurface},
    vulkan1_0::{
        AllocationCallbacks, BaseInStructure, Bool32, Instance, PhysicalDevice, StructureType, VulkanResultCodes,
    },
};
use std::ffi::CStr;
#[doc(alias = "VkDirectFBSurfaceCreateInfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DirectFBSurfaceCreateInfoEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    flags: DirectFBSurfaceCreateFlagsEXT,
    dfb: *mut IDirectFB,
    surface: *mut IDirectFBSurface,
}
#[doc(alias = "VkDirectFBSurfaceCreateFlagsEXT")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DirectFBSurfaceCreateFlagsEXT(u32);
impl DirectFBSurfaceCreateFlagsEXT {
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
}
#[doc(alias = "VK_EXT_DIRECTFB_SURFACE_SPEC_VERSION")]
pub const EXT_DIRECTFB_SURFACE_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_EXT_DIRECTFB_SURFACE_EXTENSION_NAME")]
pub const EXT_DIRECTFB_SURFACE_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_directfb_surface");
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
