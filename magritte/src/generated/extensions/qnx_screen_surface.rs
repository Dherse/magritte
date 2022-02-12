//![VK_QNX_screen_surface](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_QNX_screen_surface.html) - instance extension
//!# Description
//!The [`VK_QNX_screen_surface`] extension is an instance extension.
//!It provides a mechanism to create a [`SurfaceKHR`] object (defined by
//!the `[`VK_KHR_surface`]` extension) that refers to a QNX Screen
//!`window`, as well as a query to determine support for rendering to a QNX
//!Screen compositor.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_surface`]`
//!# Contacts
//! - Mike Gorchak [mgorchak-blackberry](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_QNX_screen_surface]
//!   @mgorchak-blackberry%0A<<Here describe the issue or question you have about the
//!   VK_QNX_screen_surface extension>>)
//!# New functions & commands
//! - [`CreateScreenSurfaceQNX`]
//! - [`GetPhysicalDeviceScreenPresentationSupportQNX`]
//!# New structures
//! - [`ScreenSurfaceCreateInfoQNX`]
//!# New bitmasks
//! - [`ScreenSurfaceCreateFlagsQNX`]
//!# New constants
//! - [`QNX_SCREEN_SURFACE_EXTENSION_NAME`]
//! - [`QNX_SCREEN_SURFACE_SPEC_VERSION`]
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_SCREEN_SURFACE_CREATE_INFO_QNX`
//!# Version History
//! - Revision 1, 2021-01-11 (Mike Gorchak)
//! - Initial draft.
//!# Other info
//! * 2021-01-11
//! * No known IP claims.
//!*
//! - Mike Gorchak, BlackBerry Limited
//!# Related
//! - [`ScreenSurfaceCreateFlagsQNX`]
//! - [`ScreenSurfaceCreateInfoQNX`]
//! - [`CreateScreenSurfaceQNX`]
//! - [`GetPhysicalDeviceScreenPresentationSupportQNX`]
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
#[doc(alias = "VK_QNX_SCREEN_SURFACE_SPEC_VERSION")]
pub const QNX_SCREEN_SURFACE_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_QNX_SCREEN_SURFACE_EXTENSION_NAME")]
pub const QNX_SCREEN_SURFACE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_QNX_screen_surface");
