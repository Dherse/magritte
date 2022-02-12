//![VK_KHR_shader_non_semantic_info](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_shader_non_semantic_info.html) - device extension
//!# Description
//!This extension allows the use of the `SPV_KHR_non_semantic_info` extension
//!in SPIR-V shader modules.
//!# Revision
//!1
//!# Dependencies
//! - *Promoted* to
//![Vulkan 1.3](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.3-promotions)
//!# Dependencies
//! - Requires Vulkan 1.0
//!# Contacts
//! - Baldur Karlsson [baldurk](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_shader_non_semantic_info]
//!   @baldurk%0A<<Here describe the issue or question you have about the
//!   VK_KHR_shader_non_semantic_info extension>>)
//!# New constants
//! - [`KHR_SHADER_NON_SEMANTIC_INFO_EXTENSION_NAME`]
//! - [`KHR_SHADER_NON_SEMANTIC_INFO_SPEC_VERSION`]
//!# Version History
//! - Revision 1, 2019-10-16 (Baldur Karlsson)
//! - Initial revision
//!# Other info
//! * 2019-10-16
//!*
//! - Promoted to Vulkan 1.3 Core
//! - This extension requires
//![`SPV_KHR_non_semantic_info`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/KHR/SPV_KHR_non_semantic_info.html)
//!
//! * No known IP claims.
//!*
//! - Baldur Karlsson, Valve
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
#[doc(alias = "VK_KHR_SHADER_NON_SEMANTIC_INFO_SPEC_VERSION")]
pub const KHR_SHADER_NON_SEMANTIC_INFO_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_SHADER_NON_SEMANTIC_INFO_EXTENSION_NAME")]
pub const KHR_SHADER_NON_SEMANTIC_INFO_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_shader_non_semantic_info");
