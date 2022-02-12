//![VK_KHR_draw_indirect_count](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_draw_indirect_count.html) - device extension
//!# Description
//!This extension is based off the `[`VK_AMD_draw_indirect_count`]`
//!extension.
//!This extension allows an application to source the number of draws for
//!indirect drawing calls from a buffer.Applications might want to do culling on the GPU via a
//! compute shader prior
//!to drawing.
//!This enables the application to generate an arbitrary number of drawing
//!commands and execute them without host intervention.
//!# Revision
//!1
//!# Dependencies
//! - *Promoted* to
//![Vulkan 1.2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.2-promotions)
//!# Dependencies
//! - Requires Vulkan 1.0
//!# Contacts
//! - Piers Daniell [pdaniell-nv](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_draw_indirect_count]
//!   @pdaniell-nv%0A<<Here describe the issue or question you have about the
//!   VK_KHR_draw_indirect_count extension>>)
//!# New functions & commands
//! - [`CmdDrawIndexedIndirectCountKHR`]
//! - [`CmdDrawIndirectCountKHR`]
//!# New constants
//! - [`KHR_DRAW_INDIRECT_COUNT_EXTENSION_NAME`]
//! - [`KHR_DRAW_INDIRECT_COUNT_SPEC_VERSION`]
//!# Version History
//! - Revision 1, 2017-08-25 (Piers Daniell)
//! - Initial draft based off VK_AMD_draw_indirect_count
//!# Other info
//! * 2017-08-25
//!*
//! - Promoted to Vulkan 1.2 Core
//! * No known IP claims.
//!*
//! - Matthaeus G. Chajdas, AMD
//! - Derrick Owens, AMD
//! - Graham Sellers, AMD
//! - Daniel Rakos, AMD
//! - Dominik Witczak, AMD
//! - Piers Daniell, NVIDIA
//!# Related
//! - [`CmdDrawIndexedIndirectCountKHR`]
//! - [`CmdDrawIndirectCountKHR`]
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
#[doc(alias = "VK_KHR_DRAW_INDIRECT_COUNT_SPEC_VERSION")]
pub const KHR_DRAW_INDIRECT_COUNT_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_DRAW_INDIRECT_COUNT_EXTENSION_NAME")]
pub const KHR_DRAW_INDIRECT_COUNT_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_draw_indirect_count");
