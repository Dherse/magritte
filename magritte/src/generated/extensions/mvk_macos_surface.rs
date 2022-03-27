use crate::vulkan1_0::{BaseInStructure, StructureType};
use std::{
    ffi::{c_void, CStr},
    marker::PhantomData,
};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_MVK_MACOS_SURFACE_SPEC_VERSION")]
pub const MVK_MACOS_SURFACE_SPEC_VERSION: u32 = 3;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_MVK_MACOS_SURFACE_EXTENSION_NAME")]
pub const MVK_MACOS_SURFACE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_MVK_macos_surface");
///[VkMacOSSurfaceCreateInfoMVK](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMacOSSurfaceCreateInfoMVK.html) - Structure specifying parameters of a newly created macOS surface object
///# C Specifications
///The [`MacOsSurfaceCreateInfoMVK`] structure is defined as:
///```c
///// Provided by VK_MVK_macos_surface
///typedef struct VkMacOSSurfaceCreateInfoMVK {
///    VkStructureType                 sType;
///    const void*                     pNext;
///    VkMacOSSurfaceCreateFlagsMVK    flags;
///    const void*                     pView;
///} VkMacOSSurfaceCreateInfoMVK;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is reserved for future use.
/// - [`p_view`] is a reference to either a [`CaMetalLayer`] object or an `NSView` object.
///# Description
///Valid Usage
/// - If [`p_view`] is a [`CaMetalLayer`] object, it **must** be a valid [`CaMetalLayer`]
/// - If [`p_view`] is an `NSView` object, it **must** be a valid `NSView`, **must** be backed by a
///   `CALayer` object of type [`CaMetalLayer`], and [`CreateMacOsSurfaceMVK`]**must** be called on
///   the main thread
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_MACOS_SURFACE_CREATE_INFO_MVK`
/// - [`p_next`]**must** be `NULL`
/// - [`flags`]**must** be `0`
///# Related
/// - [`VK_MVK_macos_surface`]
/// - [`MacOsSurfaceCreateFlagsMVK`]
/// - [`StructureType`]
/// - [`CreateMacOsSurfaceMVK`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct MacOsSurfaceCreateInfoMVK<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`flags`] is reserved for future use.
    flags: MacOsSurfaceCreateFlagsMVK,
    ///[`p_view`] is a reference to either a [`CaMetalLayer`] object or
    ///an `NSView` object.
    p_view: *mut c_void,
}
