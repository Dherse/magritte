//![VK_AMD_shader_trinary_minmax](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_AMD_shader_trinary_minmax.html) - device extension
//!# Description
//!This extension adds support for the following SPIR-V extension in Vulkan:
//! - [`SPV_AMD_shader_trinary_minmax`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/AMD/SPV_AMD_shader_trinary_minmax.html)
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//!# Contacts
//! - Qun Lin [linqun](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_AMD_shader_trinary_minmax]
//!   @linqun%0A<<Here describe the issue or question you have about the
//!   VK_AMD_shader_trinary_minmax extension>>)
//!# New constants
//! - [`AMD_SHADER_TRINARY_MINMAX_EXTENSION_NAME`]
//! - [`AMD_SHADER_TRINARY_MINMAX_SPEC_VERSION`]
//!# Version history
//! - Revision 1, 2016-05-10 (Daniel Rakos)  - Initial draft
//!# Other information
//! * 2016-05-10
//! * No known IP claims.
//! * - This extension requires [`SPV_AMD_shader_trinary_minmax`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/AMD/SPV_AMD_shader_trinary_minmax.html)
//!   - This extension provides API support for [`GL_AMD_shader_trinary_minmax`](https://www.khronos.org/registry/OpenGL/extensions/AMD/AMD_shader_trinary_minmax.txt)
//! * - Matthaeus G. Chajdas, AMD  - Qun Lin, AMD  - Daniel Rakos, AMD  - Graham Sellers, AMD  - Rex
//!   Xu, AMD
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
#[doc(alias = "VK_AMD_SHADER_TRINARY_MINMAX_SPEC_VERSION")]
pub const AMD_SHADER_TRINARY_MINMAX_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_AMD_SHADER_TRINARY_MINMAX_EXTENSION_NAME")]
pub const AMD_SHADER_TRINARY_MINMAX_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_AMD_shader_trinary_minmax");
