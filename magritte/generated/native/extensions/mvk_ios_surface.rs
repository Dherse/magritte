use crate::native::{
    extensions::khr_surface::SurfaceKHR,
    vulkan1_0::{AllocationCallbacks, BaseInStructure, Instance, StructureType, VulkanResultCodes},
};
#[doc(alias = "VkIOSSurfaceCreateInfoMVK")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct IosSurfaceCreateInfoMVK {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub flags: IosSurfaceCreateFlagsMVK,
    #[doc(alias = "pView")]
    pub view: *const std::ffi::c_void,
}
impl Default for IosSurfaceCreateInfoMVK {
    fn default() -> Self {
        Self {
            s_type: StructureType::IosSurfaceCreateInfoMvk,
            p_next: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
            view: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::mvk_ios_surface::{
    IosSurfaceCreateFlagsMVK, MVK_IOS_SURFACE_EXTENSION_NAME, MVK_IOS_SURFACE_SPEC_VERSION,
};
#[doc(alias = "vkCreateIOSSurfaceMVK")]
pub type FNCreateIosSurfaceMvk = unsafe extern "system" fn(
    instance: Instance,
    p_create_info: *const IosSurfaceCreateInfoMVK,
    p_allocator: *const AllocationCallbacks,
    p_surface: *mut SurfaceKHR,
) -> VulkanResultCodes;
