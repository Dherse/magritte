use crate::native::{
    extensions::khr_surface::SurfaceKHR,
    opaque::zx_handle_t,
    vulkan1_0::{AllocationCallbacks, BaseInStructure, Instance, StructureType, VulkanResultCodes},
};
#[doc(alias = "VkImagePipeSurfaceCreateInfoFUCHSIA")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ImagePipeSurfaceCreateInfoFUCHSIA {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub flags: ImagePipeSurfaceCreateFlagsFUCHSIA,
    #[doc(alias = "imagePipeHandle")]
    pub image_pipe_handle: zx_handle_t,
}
impl Default for ImagePipeSurfaceCreateInfoFUCHSIA {
    fn default() -> Self {
        Self {
            s_type: StructureType::ImagepipeSurfaceCreateInfoFuchsia,
            p_next: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
            image_pipe_handle: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::fuchsia_imagepipe_surface::{
    ImagePipeSurfaceCreateFlagsFUCHSIA, FUCHSIA_IMAGEPIPE_SURFACE_EXTENSION_NAME,
    FUCHSIA_IMAGEPIPE_SURFACE_SPEC_VERSION,
};
#[doc(alias = "vkCreateImagePipeSurfaceFUCHSIA")]
pub type FNCreateImagePipeSurfaceFuchsia = unsafe extern "system" fn(
    instance: Instance,
    p_create_info: *const ImagePipeSurfaceCreateInfoFUCHSIA,
    p_allocator: *const AllocationCallbacks,
    p_surface: *mut SurfaceKHR,
) -> VulkanResultCodes;
