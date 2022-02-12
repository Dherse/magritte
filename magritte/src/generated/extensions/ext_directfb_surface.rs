//![VK_EXT_directfb_surface](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_directfb_surface.html) - instance extension
//!# Description
//!The [`VK_EXT_directfb_surface`] extension is an instance extension.
//!It provides a mechanism to create a [`SurfaceKHR`] object (defined by
//!the `[`VK_KHR_surface`]` extension) that refers to a DirectFB
//![`IDirectFBSurface`], as well as a query to determine support for rendering
//!via DirectFB.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_surface`]`
//!# Contacts
//! - Nicolas Caramelli [caramelli](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_directfb_surface]
//!   @caramelli%0A<<Here describe the issue or question you have about the VK_EXT_directfb_surface
//!   extension>>)
//!# New functions & commands
//! - [`CreateDirectFBSurfaceEXT`]
//! - [`GetPhysicalDeviceDirectFBPresentationSupportEXT`]
//!# New structures
//! - [`DirectFBSurfaceCreateInfoEXT`]
//!# New bitmasks
//! - [`DirectFBSurfaceCreateFlagsEXT`]
//!# New constants
//! - [`EXT_DIRECTFB_SURFACE_EXTENSION_NAME`]
//! - [`EXT_DIRECTFB_SURFACE_SPEC_VERSION`]
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_DIRECTFB_SURFACE_CREATE_INFO_EXT`
//!# Version History
//! - Revision 1, 2020-06-16 (Nicolas Caramelli)
//! - Initial version
//!# Other info
//! * 2020-06-16
//! * No known IP claims.
//!*
//! - Nicolas Caramelli
//!# Related
//! - [`DirectFBSurfaceCreateFlagsEXT`]
//! - [`DirectFBSurfaceCreateInfoEXT`]
//! - [`CreateDirectFBSurfaceEXT`]
//! - [`GetPhysicalDeviceDirectFBPresentationSupportEXT`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use std::ffi::CStr;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_DIRECTFB_SURFACE_SPEC_VERSION")]
pub const EXT_DIRECTFB_SURFACE_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_DIRECTFB_SURFACE_EXTENSION_NAME")]
pub const EXT_DIRECTFB_SURFACE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_directfb_surface");
