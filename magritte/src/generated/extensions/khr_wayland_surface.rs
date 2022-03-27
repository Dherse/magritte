use crate::{
    native::{wl_display, wl_surface},
    vulkan1_0::{BaseInStructure, StructureType},
};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_WAYLAND_SURFACE_SPEC_VERSION")]
pub const KHR_WAYLAND_SURFACE_SPEC_VERSION: u32 = 6;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_WAYLAND_SURFACE_EXTENSION_NAME")]
pub const KHR_WAYLAND_SURFACE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_wayland_surface");
///[VkWaylandSurfaceCreateInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkWaylandSurfaceCreateInfoKHR.html) - Structure specifying parameters of a newly created Wayland surface object
///# C Specifications
///The [`WaylandSurfaceCreateInfoKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_wayland_surface
///typedef struct VkWaylandSurfaceCreateInfoKHR {
///    VkStructureType                   sType;
///    const void*                       pNext;
///    VkWaylandSurfaceCreateFlagsKHR    flags;
///    struct wl_display*                display;
///    struct wl_surface*                surface;
///} VkWaylandSurfaceCreateInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is reserved for future use.
/// - [`display`] and [`surface`] are pointers to the Wayland [`wl_display`] and [`wl_surface`] to
///   associate the surface with.
///# Description
///Valid Usage
/// - [`display`]**must** point to a valid Wayland [`wl_display`]
/// - [`surface`]**must** point to a valid Wayland [`wl_surface`]
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_WAYLAND_SURFACE_CREATE_INFO_KHR`
/// - [`p_next`]**must** be `NULL`
/// - [`flags`]**must** be `0`
///# Related
/// - [`VK_KHR_wayland_surface`]
/// - [`StructureType`]
/// - [`WaylandSurfaceCreateFlagsKHR`]
/// - [`CreateWaylandSurfaceKHR`]
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
pub struct WaylandSurfaceCreateInfoKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`flags`] is reserved for future use.
    flags: WaylandSurfaceCreateFlagsKHR,
    ///[`display`] and [`surface`] are pointers to the Wayland
    ///[`wl_display`] and [`wl_surface`] to associate the surface with.
    display: *const wl_display,
    ///No documentation found
    surface: *const wl_surface,
}
