use crate::{
    native::{Display, Window},
    vulkan1_0::{BaseInStructure, StructureType},
};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_XLIB_SURFACE_SPEC_VERSION")]
pub const KHR_XLIB_SURFACE_SPEC_VERSION: u32 = 6;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_XLIB_SURFACE_EXTENSION_NAME")]
pub const KHR_XLIB_SURFACE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_xlib_surface");
///[VkXlibSurfaceCreateInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkXlibSurfaceCreateInfoKHR.html) - Structure specifying parameters of a newly created Xlib surface object
///# C Specifications
///The [`XlibSurfaceCreateInfoKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_xlib_surface
///typedef struct VkXlibSurfaceCreateInfoKHR {
///    VkStructureType                sType;
///    const void*                    pNext;
///    VkXlibSurfaceCreateFlagsKHR    flags;
///    Display*                       dpy;
///    Window                         window;
///} VkXlibSurfaceCreateInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is reserved for future use.
/// - [`dpy`] is a pointer to an Xlib [`Display`] connection to the X server.
/// - [`window`] is an Xlib [`Window`] to associate the surface with.
///# Description
///Valid Usage
/// - [`dpy`]**must** point to a valid Xlib [`Display`]
/// - [`window`]**must** be a valid Xlib [`Window`]
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_XLIB_SURFACE_CREATE_INFO_KHR`
/// - [`p_next`]**must** be `NULL`
/// - [`flags`]**must** be `0`
///# Related
/// - [`VK_KHR_xlib_surface`]
/// - [`StructureType`]
/// - [`XlibSurfaceCreateFlagsKHR`]
/// - [`CreateXlibSurfaceKHR`]
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
pub struct XlibSurfaceCreateInfoKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`flags`] is reserved for future use.
    flags: XlibSurfaceCreateFlagsKHR,
    ///[`dpy`] is a pointer to an Xlib [`Display`] connection to the X
    ///server.
    dpy: *const Display,
    ///[`window`] is an Xlib [`Window`] to associate the surface with.
    window: Window,
}
