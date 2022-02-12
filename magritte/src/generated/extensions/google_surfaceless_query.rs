//![VK_GOOGLE_surfaceless_query](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_GOOGLE_surfaceless_query.html) - instance extension
//!# Description
//!This extension allows the [`GetPhysicalDeviceSurfaceFormatsKHR`] and
//![`GetPhysicalDeviceSurfacePresentModesKHR`] functions to accept
//![`crate::utils::Handle::null`] as their `surface` parameter, allowing potential
//!surface formats, colorspaces and present modes to be queried without
//!providing a surface.
//!Identically, [`GetPhysicalDeviceSurfaceFormats2KHR`] and
//![`GetPhysicalDeviceSurfacePresentModes2EXT`] would accept
//![`crate::utils::Handle::null`] in
//![`PhysicalDeviceSurfaceInfo2KHR::surface`].
//!**This can only be supported on platforms where the results of these queries
//!are surface-agnostic and a single presentation engine is the implicit target
//!of all present operations**.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_surface`]`
//!# Contacts
//! - Shahbaz Youssefi [syoussefi](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_GOOGLE_surfaceless_query]
//!   @syoussefi%0A<<Here describe the issue or question you have about the
//!   VK_GOOGLE_surfaceless_query extension>>)
//!# New constants
//! - [`GOOGLE_SURFACELESS_QUERY_EXTENSION_NAME`]
//! - [`GOOGLE_SURFACELESS_QUERY_SPEC_VERSION`]
//!# Version History
//! - Revision 1, 2021-12-14 (Shahbaz Youssefi)
//! - Internal revisions
//!# Other info
//! * 2021-12-14
//! * No known IP claims.
//!*
//! - Ian Elliott, Google
//! - Shahbaz Youssefi, Google
//! - James Jones, NVIDIA
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
#[doc(alias = "VK_GOOGLE_SURFACELESS_QUERY_SPEC_VERSION")]
pub const GOOGLE_SURFACELESS_QUERY_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_GOOGLE_SURFACELESS_QUERY_EXTENSION_NAME")]
pub const GOOGLE_SURFACELESS_QUERY_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_GOOGLE_surfaceless_query");
