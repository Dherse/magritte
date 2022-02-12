//![VK_GGP_frame_token](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_GGP_frame_token.html) - device extension
//!# Description
//!This extension allows an application that uses the `[`VK_KHR_swapchain`]`
//!extension in combination with a Google Games Platform surface provided by
//!the `[`VK_GGP_stream_descriptor_surface`]` extension to associate a
//!Google Games Platform frame token with a present operation.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_swapchain`]`
//! - Requires `[`VK_GGP_stream_descriptor_surface`]`
//!# Contacts
//! - Jean-Francois Roy [jfroy](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_GGP_frame_token]
//!   @jfroy%0A<<Here describe the issue or question you have about the VK_GGP_frame_token
//!   extension>>)
//!# New structures
//! - Extending [`PresentInfoKHR`]:
//! - [`PresentFrameTokenGGP`]
//!# New constants
//! - [`GGP_FRAME_TOKEN_EXTENSION_NAME`]
//! - [`GGP_FRAME_TOKEN_SPEC_VERSION`]
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_PRESENT_FRAME_TOKEN_GGP`
//!# Version History
//! - Revision 1, 2018-11-26 (Jean-Francois Roy)
//! - Initial revision.
//!# Other info
//! * 2019-01-28
//! * No known IP claims.
//!*
//! - Jean-Francois Roy, Google
//! - Richard O’Grady, Google
//!# Related
//! - [`PresentFrameTokenGGP`]
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
#[doc(alias = "VK_GGP_FRAME_TOKEN_SPEC_VERSION")]
pub const GGP_FRAME_TOKEN_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_GGP_FRAME_TOKEN_EXTENSION_NAME")]
pub const GGP_FRAME_TOKEN_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_GGP_frame_token");
