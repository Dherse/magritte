use crate::{
    native::{_screen_context, _screen_window},
    vulkan1_0::{BaseInStructure, StructureType},
};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_QNX_SCREEN_SURFACE_SPEC_VERSION")]
pub const QNX_SCREEN_SURFACE_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_QNX_SCREEN_SURFACE_EXTENSION_NAME")]
pub const QNX_SCREEN_SURFACE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_QNX_screen_surface");
///[VkScreenSurfaceCreateInfoQNX](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkScreenSurfaceCreateInfoQNX.html) - Structure specifying parameters of a newly created QNX Screen surface object
///# C Specifications
///The [`ScreenSurfaceCreateInfoQNX`] structure is defined as:
///```c
///// Provided by VK_QNX_screen_surface
///typedef struct VkScreenSurfaceCreateInfoQNX {
///    VkStructureType                  sType;
///    const void*                      pNext;
///    VkScreenSurfaceCreateFlagsQNX    flags;
///    struct _screen_context*          context;
///    struct _screen_window*           window;
///} VkScreenSurfaceCreateInfoQNX;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is reserved for future use.
/// - [`context`] and [`window`] are QNX Screen [`context`] and [`window`] to associate the surface
///   with.
///# Description
///Valid Usage
/// - [`context`]**must** point to a valid QNX Screen `struct` _screen_context
/// - [`window`]**must** point to a valid QNX Screen `struct` _screen_window
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_SCREEN_SURFACE_CREATE_INFO_QNX`
/// - [`p_next`]**must** be `NULL`
/// - [`flags`]**must** be `0`
///# Related
/// - [`VK_QNX_screen_surface`]
/// - [`ScreenSurfaceCreateFlagsQNX`]
/// - [`StructureType`]
/// - [`CreateScreenSurfaceQNX`]
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
pub struct ScreenSurfaceCreateInfoQNX<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`flags`] is reserved for future use.
    flags: ScreenSurfaceCreateFlagsQNX,
    ///[`context`] and [`window`] are QNX Screen [`context`] and
    ///[`window`] to associate the surface with.
    context: *const _screen_context,
    ///No documentation found
    window: *const _screen_window,
}
