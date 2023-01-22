//!# [VK_FUCHSIA_imagepipe_surface](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_FUCHSIA_imagepipe_surface.html)
# ! [doc = include_str ! ("../../../../doc/extensions/fuchsia_imagepipe_surface/VK_FUCHSIA_imagepipe_surface.md")]
use crate::{
    cstr,
    extensions::khr_surface::SurfaceKHR,
    opaque::zx_handle_t,
    vulkan1_0::{AllocationCallbacks, BaseInStructure, Instance, StructureType, VulkanResultCodes},
};
use std::ffi::CStr;
///# [VkImagePipeSurfaceCreateInfoFUCHSIA](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImagePipeSurfaceCreateInfoFUCHSIA.html)
# [doc = include_str ! ("../../../../doc/extensions/fuchsia_imagepipe_surface/VkImagePipeSurfaceCreateInfoFUCHSIA.md")]
#[doc(alias = "VkImagePipeSurfaceCreateInfoFUCHSIA")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ImagePipeSurfaceCreateInfoFUCHSIA {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    flags: ImagePipeSurfaceCreateFlagsFUCHSIA,
    #[doc(alias = "imagePipeHandle")]
    image_pipe_handle: zx_handle_t,
}
#[doc(alias = "VkImagePipeSurfaceCreateFlagsFUCHSIA")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ImagePipeSurfaceCreateFlagsFUCHSIA(u32);
impl ImagePipeSurfaceCreateFlagsFUCHSIA {
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
}
#[doc(alias = "VK_FUCHSIA_IMAGEPIPE_SURFACE_SPEC_VERSION")]
pub const FUCHSIA_IMAGEPIPE_SURFACE_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_FUCHSIA_IMAGEPIPE_SURFACE_EXTENSION_NAME")]
pub const FUCHSIA_IMAGEPIPE_SURFACE_EXTENSION_NAME: &'static CStr = cstr!("VK_FUCHSIA_imagepipe_surface");
///# [vkCreateImagePipeSurfaceFUCHSIA](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateImagePipeSurfaceFUCHSIA.html)
# [doc = include_str ! ("../../../../doc/extensions/fuchsia_imagepipe_surface/vkCreateImagePipeSurfaceFUCHSIA.md")]
#[doc(alias = "vkCreateImagePipeSurfaceFUCHSIA")]
pub type FNCreateImagePipeSurfaceFuchsia = unsafe extern "system" fn(
    instance: Instance,
    p_create_info: *const ImagePipeSurfaceCreateInfoFUCHSIA,
    p_allocator: *const AllocationCallbacks,
    p_surface: *mut SurfaceKHR,
) -> VulkanResultCodes;