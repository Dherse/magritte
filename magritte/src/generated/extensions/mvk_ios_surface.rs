use crate::vulkan1_0::{BaseInStructure, StructureType};
use std::{
    ffi::{c_void, CStr},
    marker::PhantomData,
};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_MVK_IOS_SURFACE_SPEC_VERSION")]
pub const MVK_IOS_SURFACE_SPEC_VERSION: u32 = 3;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_MVK_IOS_SURFACE_EXTENSION_NAME")]
pub const MVK_IOS_SURFACE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_MVK_ios_surface");
///[VkIOSSurfaceCreateInfoMVK](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkIOSSurfaceCreateInfoMVK.html) - Structure specifying parameters of a newly created iOS surface object
///# C Specifications
///The [`IosSurfaceCreateInfoMVK`] structure is defined as:
///```c
///// Provided by VK_MVK_ios_surface
///typedef struct VkIOSSurfaceCreateInfoMVK {
///    VkStructureType               sType;
///    const void*                   pNext;
///    VkIOSSurfaceCreateFlagsMVK    flags;
///    const void*                   pView;
///} VkIOSSurfaceCreateInfoMVK;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is reserved for future use.
/// - [`p_view`] is a reference to either a [`CaMetalLayer`] object or a `UIView` object.
///# Description
///Valid Usage
/// - If [`p_view`] is a [`CaMetalLayer`] object, it **must** be a valid [`CaMetalLayer`]
/// - If [`p_view`] is a `UIView` object, it **must** be a valid `UIView`, **must** be backed by a
///   `CALayer` object of type [`CaMetalLayer`], and [`CreateIosSurfaceMVK`]**must** be called on
///   the main thread
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_IOS_SURFACE_CREATE_INFO_MVK`
/// - [`p_next`]**must** be `NULL`
/// - [`flags`]**must** be `0`
///# Related
/// - [`VK_MVK_ios_surface`]
/// - [`IosSurfaceCreateFlagsMVK`]
/// - [`StructureType`]
/// - [`CreateIosSurfaceMVK`]
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
pub struct IosSurfaceCreateInfoMVK<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`flags`] is reserved for future use.
    flags: IosSurfaceCreateFlagsMVK,
    ///[`p_view`] is a reference to either a [`CaMetalLayer`] object or a
    ///`UIView` object.
    p_view: *mut c_void,
}
