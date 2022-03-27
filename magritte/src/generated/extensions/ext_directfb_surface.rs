use crate::{
    native::{IDirectFB, IDirectFBSurface},
    vulkan1_0::{BaseInStructure, StructureType},
};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_DIRECTFB_SURFACE_SPEC_VERSION")]
pub const EXT_DIRECTFB_SURFACE_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_DIRECTFB_SURFACE_EXTENSION_NAME")]
pub const EXT_DIRECTFB_SURFACE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_directfb_surface");
///[VkDirectFBSurfaceCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDirectFBSurfaceCreateInfoEXT.html) - Structure specifying parameters of a newly created DirectFB surface object
///# C Specifications
///The [`DirectFBSurfaceCreateInfoEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_directfb_surface
///typedef struct VkDirectFBSurfaceCreateInfoEXT {
///    VkStructureType                    sType;
///    const void*                        pNext;
///    VkDirectFBSurfaceCreateFlagsEXT    flags;
///    IDirectFB*                         dfb;
///    IDirectFBSurface*                  surface;
///} VkDirectFBSurfaceCreateInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is reserved for future use.
/// - [`dfb`] is a pointer to the [`IDirectFB`] main interface of DirectFB.
/// - [`surface`] is a pointer to a [`IDirectFBSurface`] surface interface.
///# Description
///Valid Usage
/// - [`dfb`]**must** point to a valid DirectFB [`IDirectFB`]
/// - [`surface`]**must** point to a valid DirectFB [`IDirectFBSurface`]
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_DIRECTFB_SURFACE_CREATE_INFO_EXT`
/// - [`p_next`]**must** be `NULL`
/// - [`flags`]**must** be `0`
///# Related
/// - [`VK_EXT_directfb_surface`]
/// - [`DirectFBSurfaceCreateFlagsEXT`]
/// - [`StructureType`]
/// - [`CreateDirectFBSurfaceEXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct DirectFBSurfaceCreateInfoEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`flags`] is reserved for future use.
    flags: DirectFBSurfaceCreateFlagsEXT,
    ///[`dfb`] is a pointer to the [`IDirectFB`] main interface of DirectFB.
    dfb: *const IDirectFB,
    ///[`surface`] is a pointer to a [`IDirectFBSurface`] surface interface.
    surface: *const IDirectFBSurface,
}
