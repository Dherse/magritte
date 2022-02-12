//![VK_AMD_buffer_marker](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_AMD_buffer_marker.html) - device extension
//!# Description
//!This extension adds a new operation to execute pipelined writes of small
//!marker values into a [`Buffer`] object.The primary purpose of these markers is to facilitate the
//! development of
//!debugging tools for tracking which pipelined command contributed to device
//!loss.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//!# Contacts
//! - Daniel Rakos [drakos-amd](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_AMD_buffer_marker]
//!   @drakos-amd%0A<<Here describe the issue or question you have about the VK_AMD_buffer_marker
//!   extension>>)
//!# New functions & commands
//! - [`CmdWriteBufferMarkerAMD`]
//!# New constants
//! - [`AMD_BUFFER_MARKER_EXTENSION_NAME`]
//! - [`AMD_BUFFER_MARKER_SPEC_VERSION`]
//!# Version History
//! - Revision 1, 2018-01-26 (Jaakko Konttinen)
//! - Initial revision
//!# Other info
//! * 2018-01-26
//! * No known IP claims.
//!*
//! - Matthaeus G. Chajdas, AMD
//! - Jaakko Konttinen, AMD
//! - Daniel Rakos, AMD
//!# Related
//! - [`CmdWriteBufferMarkerAMD`]
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
#[doc(alias = "VK_AMD_BUFFER_MARKER_SPEC_VERSION")]
pub const AMD_BUFFER_MARKER_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_AMD_BUFFER_MARKER_EXTENSION_NAME")]
pub const AMD_BUFFER_MARKER_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_AMD_buffer_marker");
