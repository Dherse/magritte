use crate::native::{
    extensions::khr_surface::SurfaceKHR,
    vulkan1_0::{AllocationCallbacks, BaseInStructure, Instance, StructureType, VulkanResultCodes},
};
#[doc(alias = "VkViSurfaceCreateInfoNN")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ViSurfaceCreateInfoNN {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub flags: ViSurfaceCreateFlagsNN,
    pub window: *mut std::ffi::c_void,
}
impl Default for ViSurfaceCreateInfoNN {
    fn default() -> Self {
        Self {
            s_type: StructureType::ViSurfaceCreateInfoNn,
            p_next: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
            window: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::nn_vi_surface::{
    ViSurfaceCreateFlagsNN, NN_VI_SURFACE_EXTENSION_NAME, NN_VI_SURFACE_SPEC_VERSION,
};
#[doc(alias = "vkCreateViSurfaceNN")]
pub type FNCreateViSurfaceNn = unsafe extern "system" fn(
    instance: Instance,
    p_create_info: *const ViSurfaceCreateInfoNN,
    p_allocator: *const AllocationCallbacks,
    p_surface: *mut SurfaceKHR,
) -> VulkanResultCodes;
