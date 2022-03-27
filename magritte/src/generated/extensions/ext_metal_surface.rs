use crate::vulkan1_0::{BaseInStructure, StructureType};
use std::{
    ffi::{c_void, CStr},
    marker::PhantomData,
};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_METAL_SURFACE_SPEC_VERSION")]
pub const EXT_METAL_SURFACE_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_METAL_SURFACE_EXTENSION_NAME")]
pub const EXT_METAL_SURFACE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_metal_surface");
///[CAMetalLayer](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/CAMetalLayer.html) - CoreAnimation native layer type for Metal
///# C Specifications
///To remove an unnecessary compile-time dependency, an incomplete type
///definition of [`CaMetalLayer`] is provided in the Vulkan headers:
///```c
///// Provided by VK_EXT_metal_surface
///
///#ifdef __OBJC__
///@class CAMetalLayer;
///#else
///typedef void CAMetalLayer;
///#endif
///```
///# Related
/// - [`VK_EXT_metal_surface`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "CAMetalLayer")]
pub type CaMetalLayer = c_void;
///[VkMetalSurfaceCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMetalSurfaceCreateInfoEXT.html) - Structure specifying parameters of a newly created Metal surface object
///# C Specifications
///The [`MetalSurfaceCreateInfoEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_metal_surface
///typedef struct VkMetalSurfaceCreateInfoEXT {
///    VkStructureType                 sType;
///    const void*                     pNext;
///    VkMetalSurfaceCreateFlagsEXT    flags;
///    const CAMetalLayer*             pLayer;
///} VkMetalSurfaceCreateInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is reserved for future use.
/// - [`p_layer`] is a reference to a [`CaMetalLayer`] object representing a renderable surface.
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_METAL_SURFACE_CREATE_INFO_EXT`
/// - [`p_next`]**must** be `NULL`
/// - [`flags`]**must** be `0`
///# Related
/// - [`VK_EXT_metal_surface`]
/// - [`MetalSurfaceCreateFlagsEXT`]
/// - [`StructureType`]
/// - [`CreateMetalSurfaceEXT`]
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
pub struct MetalSurfaceCreateInfoEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`flags`] is reserved for future use.
    flags: MetalSurfaceCreateFlagsEXT,
    ///[`p_layer`] is a reference to a [`CaMetalLayer`] object
    ///representing a renderable surface.
    p_layer: *mut CaMetalLayer,
}
