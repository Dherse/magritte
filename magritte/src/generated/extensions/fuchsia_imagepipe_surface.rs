use crate::{
    native::zx_handle_t,
    vulkan1_0::{BaseInStructure, StructureType},
};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_FUCHSIA_IMAGEPIPE_SURFACE_SPEC_VERSION")]
pub const FUCHSIA_IMAGEPIPE_SURFACE_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_FUCHSIA_IMAGEPIPE_SURFACE_EXTENSION_NAME")]
pub const FUCHSIA_IMAGEPIPE_SURFACE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_FUCHSIA_imagepipe_surface");
///[VkImagePipeSurfaceCreateInfoFUCHSIA](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImagePipeSurfaceCreateInfoFUCHSIA.html) - Structure specifying parameters of a newly created ImagePipe surface object
///# C Specifications
///The [`ImagePipeSurfaceCreateInfoFUCHSIA`] structure is defined as:
///```c
///// Provided by VK_FUCHSIA_imagepipe_surface
///typedef struct VkImagePipeSurfaceCreateInfoFUCHSIA {
///    VkStructureType                         sType;
///    const void*                             pNext;
///    VkImagePipeSurfaceCreateFlagsFUCHSIA    flags;
///    zx_handle_t                             imagePipeHandle;
///} VkImagePipeSurfaceCreateInfoFUCHSIA;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is reserved for future use.
/// - [`image_pipe_handle`] is a [`zx_handle_t`] referring to the ImagePipe to associate with the
///   surface.
///# Description
///Valid Usage
/// - [`image_pipe_handle`]**must** be a valid [`zx_handle_t`]
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_IMAGEPIPE_SURFACE_CREATE_INFO_FUCHSIA`
/// - [`p_next`]**must** be `NULL`
/// - [`flags`]**must** be `0`
///# Related
/// - [`VK_FUCHSIA_imagepipe_surface`]
/// - [`ImagePipeSurfaceCreateFlagsFUCHSIA`]
/// - [`StructureType`]
/// - [`CreateImagePipeSurfaceFUCHSIA`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct ImagePipeSurfaceCreateInfoFUCHSIA<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`flags`] is reserved for future use.
    flags: ImagePipeSurfaceCreateFlagsFUCHSIA,
    ///[`image_pipe_handle`] is a [`zx_handle_t`] referring to the ImagePipe
    ///to associate with the surface.
    image_pipe_handle: zx_handle_t,
}
