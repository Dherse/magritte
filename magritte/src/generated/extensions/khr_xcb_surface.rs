use crate::{
    native::{xcb_connection_t, xcb_window_t},
    vulkan1_0::{BaseInStructure, StructureType},
};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_XCB_SURFACE_SPEC_VERSION")]
pub const KHR_XCB_SURFACE_SPEC_VERSION: u32 = 6;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_XCB_SURFACE_EXTENSION_NAME")]
pub const KHR_XCB_SURFACE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_xcb_surface");
///[VkXcbSurfaceCreateInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkXcbSurfaceCreateInfoKHR.html) - Structure specifying parameters of a newly created Xcb surface object
///# C Specifications
///The [`XcbSurfaceCreateInfoKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_xcb_surface
///typedef struct VkXcbSurfaceCreateInfoKHR {
///    VkStructureType               sType;
///    const void*                   pNext;
///    VkXcbSurfaceCreateFlagsKHR    flags;
///    xcb_connection_t*             connection;
///    xcb_window_t                  window;
///} VkXcbSurfaceCreateInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is reserved for future use.
/// - [`connection`] is a pointer to an [`xcb_connection_t`] to the X server.
/// - [`window`] is the [`xcb_window_t`] for the X11 window to associate the surface with.
///# Description
///Valid Usage
/// - [`connection`]**must** point to a valid X11 [`xcb_connection_t`]
/// - [`window`]**must** be a valid X11 [`xcb_window_t`]
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_XCB_SURFACE_CREATE_INFO_KHR`
/// - [`p_next`]**must** be `NULL`
/// - [`flags`]**must** be `0`
///# Related
/// - [`VK_KHR_xcb_surface`]
/// - [`StructureType`]
/// - [`XcbSurfaceCreateFlagsKHR`]
/// - [`CreateXcbSurfaceKHR`]
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
pub struct XcbSurfaceCreateInfoKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`flags`] is reserved for future use.
    flags: XcbSurfaceCreateFlagsKHR,
    ///[`connection`] is a pointer to an [`xcb_connection_t`] to the X
    ///server.
    connection: *const xcb_connection_t,
    ///[`window`] is the [`xcb_window_t`] for the X11 window to associate
    ///the surface with.
    window: xcb_window_t,
}
