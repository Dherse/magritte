//![VK_FUCHSIA_imagepipe_surface](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_FUCHSIA_imagepipe_surface.html) - instance extension
//!# Description
//!The [`VK_FUCHSIA_imagepipe_surface`] extension is an instance extension.
//!It provides a mechanism to create a [`SurfaceKHR`] object (defined by
//!the `[`VK_KHR_surface`]` extension) that refers to a Fuchsia
//!`imagePipeHandle`.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_surface`]`
//!# Contacts
//! - Craig Stout [cdotstout](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_FUCHSIA_imagepipe_surface]
//!   @cdotstout%0A<<Here describe the issue or question you have about the
//!   VK_FUCHSIA_imagepipe_surface extension>>)
//!# New functions & commands
//! - [`CreateImagePipeSurfaceFUCHSIA`]
//!# New structures
//! - [`ImagePipeSurfaceCreateInfoFUCHSIA`]
//!# New bitmasks
//! - [`ImagePipeSurfaceCreateFlagsFUCHSIA`]
//!# New constants
//! - [`FUCHSIA_IMAGEPIPE_SURFACE_EXTENSION_NAME`]
//! - [`FUCHSIA_IMAGEPIPE_SURFACE_SPEC_VERSION`]
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_IMAGEPIPE_SURFACE_CREATE_INFO_FUCHSIA`
//!# Version History
//! - Revision 1, 2018-07-27 (Craig Stout)  - Initial draft.
//!# Other info
//! * 2018-07-27
//! * No known IP claims.
//! * - Craig Stout, Google  - Ian Elliott, Google  - Jesse Hall, Google
//!# Related
//! - [`ImagePipeSurfaceCreateFlagsFUCHSIA`]
//! - [`ImagePipeSurfaceCreateInfoFUCHSIA`]
//! - [`CreateImagePipeSurfaceFUCHSIA`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
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
///[VkImagePipeSurfaceCreateFlagsFUCHSIA](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImagePipeSurfaceCreateFlagsFUCHSIA.html) - Reserved for future use
///# C Specifications
///```c
///// Provided by VK_FUCHSIA_imagepipe_surface
///typedef VkFlags VkImagePipeSurfaceCreateFlagsFUCHSIA;
///```
///# Related
/// - [`VK_FUCHSIA_imagepipe_surface`]
/// - [`ImagePipeSurfaceCreateInfoFUCHSIA`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct ImagePipeSurfaceCreateFlagsFUCHSIA(u32);
impl const Default for ImagePipeSurfaceCreateFlagsFUCHSIA {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for ImagePipeSurfaceCreateFlagsFUCHSIA {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple(stringify!(ImagePipeSurfaceCreateFlagsFUCHSIA))
            .field(&self.0)
            .finish()
    }
}
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
///## Valid Usage
/// - [`image_pipe_handle`] **must**  be a valid [`zx_handle_t`]
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_IMAGEPIPE_SURFACE_CREATE_INFO_FUCHSIA`
/// - [`p_next`] **must**  be `NULL`
/// - [`flags`] **must**  be `0`
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
#[doc(alias = "VkImagePipeSurfaceCreateInfoFUCHSIA")]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct ImagePipeSurfaceCreateInfoFUCHSIA<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`flags`] is reserved for future use.
    pub flags: ImagePipeSurfaceCreateFlagsFUCHSIA,
    ///[`image_pipe_handle`] is a [`zx_handle_t`] referring to the ImagePipe
    ///to associate with the surface.
    pub image_pipe_handle: zx_handle_t,
}
impl<'lt> Default for ImagePipeSurfaceCreateInfoFUCHSIA<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            flags: Default::default(),
            image_pipe_handle: Default::default(),
        }
    }
}
impl<'lt> ImagePipeSurfaceCreateInfoFUCHSIA<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::image_pipe_handle`]
    pub fn image_pipe_handle_raw(&self) -> &zx_handle_t {
        &self.image_pipe_handle
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::image_pipe_handle`]
    pub fn set_image_pipe_handle_raw(&mut self, value: zx_handle_t) -> &mut Self {
        self.image_pipe_handle = value;
        self
    }
    ///Gets the value of [`Self::s_type`]
    pub fn s_type(&self) -> StructureType {
        self.s_type
    }
    ///Gets the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next(&self) -> &BaseInStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::flags`]
    pub fn flags(&self) -> ImagePipeSurfaceCreateFlagsFUCHSIA {
        self.flags
    }
    ///Gets the value of [`Self::image_pipe_handle`]
    pub fn image_pipe_handle(&self) -> &zx_handle_t {
        &self.image_pipe_handle
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut ImagePipeSurfaceCreateFlagsFUCHSIA {
        &mut self.flags
    }
    ///Gets a mutable reference to the value of [`Self::image_pipe_handle`]
    pub fn image_pipe_handle_mut(&mut self) -> &mut zx_handle_t {
        &mut self.image_pipe_handle
    }
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the raw value of [`Self::flags`]
    pub fn set_flags(
        &mut self,
        value: crate::extensions::fuchsia_imagepipe_surface::ImagePipeSurfaceCreateFlagsFUCHSIA,
    ) -> &mut Self {
        self.flags = value;
        self
    }
    ///Sets the raw value of [`Self::image_pipe_handle`]
    pub fn set_image_pipe_handle(&mut self, value: crate::native::zx_handle_t) -> &mut Self {
        self.image_pipe_handle = value;
        self
    }
}
