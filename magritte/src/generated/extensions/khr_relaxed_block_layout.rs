//![VK_KHR_relaxed_block_layout](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_relaxed_block_layout.html) - device extension
//!# Description
//!The [`VK_KHR_relaxed_block_layout`] extension allows implementations to
//!indicate they can support more variation in block `Offset` decorations.
//!For example, placing a vector of three floats at an offset of
//!16×N +  4.See [Offset and Stride Assignment](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#interfaces-resources-layout) for
//!details.
//!# Revision
//!1
//!# Dependencies
//! - *Promoted* to
//![Vulkan 1.1](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.1-promotions)
//!# Dependencies
//! - Requires Vulkan 1.0
//!# Contacts
//! - John Kessenich [johnkslang](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_relaxed_block_layout]
//!   @johnkslang%0A<<Here describe the issue or question you have about the
//!   VK_KHR_relaxed_block_layout extension>>)
//!# New constants
//! - [`KHR_RELAXED_BLOCK_LAYOUT_EXTENSION_NAME`]
//! - [`KHR_RELAXED_BLOCK_LAYOUT_SPEC_VERSION`]
//!# Version History
//! - Revision 1, 2017-03-26 (JohnK)
//!# Other info
//! * 2017-03-26
//! * No known IP claims.
//!*
//! - Promoted to Vulkan 1.1 Core
//!*
//! - John Kessenich, Google
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
#[doc(alias = "VK_KHR_RELAXED_BLOCK_LAYOUT_SPEC_VERSION")]
pub const KHR_RELAXED_BLOCK_LAYOUT_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_RELAXED_BLOCK_LAYOUT_EXTENSION_NAME")]
pub const KHR_RELAXED_BLOCK_LAYOUT_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_relaxed_block_layout");
