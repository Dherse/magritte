use crate::{
    cstr,
    extensions::khr_surface::SurfaceKHR,
    vulkan1_0::{AllocationCallbacks, BaseInStructure, Instance, StructureType, VulkanResultCodes},
};
use std::ffi::CStr;
#[doc(alias = "VkViSurfaceCreateInfoNN")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ViSurfaceCreateInfoNN {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    flags: ViSurfaceCreateFlagsNN,
    window: *mut std::ffi::c_void,
}
#[doc(alias = "VkViSurfaceCreateFlagsNN")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ViSurfaceCreateFlagsNN(u32);
impl ViSurfaceCreateFlagsNN {
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
}
#[doc(alias = "VK_NN_VI_SURFACE_SPEC_VERSION")]
pub const NN_VI_SURFACE_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_NN_VI_SURFACE_EXTENSION_NAME")]
pub const NN_VI_SURFACE_EXTENSION_NAME: &'static CStr = cstr!("VK_NN_vi_surface");
#[doc(alias = "vkCreateViSurfaceNN")]
pub type FNCreateViSurfaceNn = unsafe extern "system" fn(
    instance: Instance,
    p_create_info: *const ViSurfaceCreateInfoNN,
    p_allocator: *const AllocationCallbacks,
    p_surface: *mut SurfaceKHR,
) -> VulkanResultCodes;
