use crate::native::{
    extensions::khr_surface::SurfaceKHR,
    vulkan1_0::{AllocationCallbacks, BaseInStructure, Instance, StructureType, VulkanResultCodes},
};
#[doc(alias = "VkMacOSSurfaceCreateInfoMVK")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct MacOsSurfaceCreateInfoMVK {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub flags: MacOsSurfaceCreateFlagsMVK,
    #[doc(alias = "pView")]
    pub view: *const std::ffi::c_void,
}
impl Default for MacOsSurfaceCreateInfoMVK {
    fn default() -> Self {
        Self {
            s_type: StructureType::MacosSurfaceCreateInfoMvk,
            p_next: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
            view: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::mvk_macos_surface::{
    MacOsSurfaceCreateFlagsMVK, MVK_MACOS_SURFACE_EXTENSION_NAME, MVK_MACOS_SURFACE_SPEC_VERSION,
};
#[doc(alias = "vkCreateMacOSSurfaceMVK")]
pub type FNCreateMacOsSurfaceMvk = unsafe extern "system" fn(
    instance: Instance,
    p_create_info: *const MacOsSurfaceCreateInfoMVK,
    p_allocator: *const AllocationCallbacks,
    p_surface: *mut SurfaceKHR,
) -> VulkanResultCodes;
