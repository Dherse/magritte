//![VK_EXT_metal_surface](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_metal_surface.html) - instance extension
//!# Description
//!The [`VK_EXT_metal_surface`] extension is an instance extension.
//!It provides a mechanism to create a [`SurfaceKHR`] object (defined by
//!the `[`VK_KHR_surface`]` extension) from [`CaMetalLayer`], which is
//!the native rendering surface of Apple’s Metal framework.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_surface`]`
//!# Contacts
//! - Dzmitry Malyshau [kvark](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_metal_surface]
//!   @kvark%0A<<Here describe the issue or question you have about the VK_EXT_metal_surface
//!   extension>>)
//!# New functions & commands
//! - [`CreateMetalSurfaceEXT`]
//!# New structures
//! - [`MetalSurfaceCreateInfoEXT`]
//!# New bitmasks
//! - [`MetalSurfaceCreateFlagsEXT`]
//!# New constants
//! - [`EXT_METAL_SURFACE_EXTENSION_NAME`]
//! - [`EXT_METAL_SURFACE_SPEC_VERSION`]
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_METAL_SURFACE_CREATE_INFO_EXT`
//!# Version History
//! - Revision 1, 2018-10-01 (Dzmitry Malyshau)
//! - Initial version
//!# Other info
//! * 2018-10-01
//! * No known IP claims.
//!*
//! - Dzmitry Malyshau, Mozilla Corp.
//!# Related
//! - [`CaMetalLayer`]
//! - [`MetalSurfaceCreateFlagsEXT`]
//! - [`MetalSurfaceCreateInfoEXT`]
//! - [`CreateMetalSurfaceEXT`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use std::ffi::{c_void, CStr};
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
