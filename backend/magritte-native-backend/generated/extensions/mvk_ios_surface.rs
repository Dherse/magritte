use crate::{
    cstr,
    extensions::khr_surface::SurfaceKHR,
    vulkan1_0::{AllocationCallbacks, BaseInStructure, Instance, StructureType, VulkanResultCodes},
};
use std::ffi::CStr;
#[doc(alias = "VkIOSSurfaceCreateInfoMVK")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct IosSurfaceCreateInfoMVK {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    flags: IosSurfaceCreateFlagsMVK,
    #[doc(alias = "pView")]
    view: *const std::ffi::c_void,
}
#[doc(alias = "VkIOSSurfaceCreateFlagsMVK")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct IosSurfaceCreateFlagsMVK(u32);
impl IosSurfaceCreateFlagsMVK {
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
}
#[doc(alias = "VK_MVK_IOS_SURFACE_SPEC_VERSION")]
pub const MVK_IOS_SURFACE_SPEC_VERSION: u32 = 3;
#[doc(alias = "VK_MVK_IOS_SURFACE_EXTENSION_NAME")]
pub const MVK_IOS_SURFACE_EXTENSION_NAME: &'static CStr = cstr!("VK_MVK_ios_surface");
#[doc(alias = "vkCreateIOSSurfaceMVK")]
pub type FNCreateIosSurfaceMvk = unsafe extern "system" fn(
    instance: Instance,
    p_create_info: *const IosSurfaceCreateInfoMVK,
    p_allocator: *const AllocationCallbacks,
    p_surface: *mut SurfaceKHR,
) -> VulkanResultCodes;
