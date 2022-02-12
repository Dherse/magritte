//![VK_EXT_shader_stencil_export](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_shader_stencil_export.html) - device extension
//!# Description
//!This extension adds support for the SPIR-V extension
//!`SPV_EXT_shader_stencil_export`, providing a mechanism whereby a shader may
//!generate the stencil reference value per invocation.
//!When stencil testing is enabled, this allows the test to be performed
//!against the value generated in the shader.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//!# Contacts
//! - Dominik Witczak [dominikwitczakamd](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_shader_stencil_export]
//!   @dominikwitczakamd%0A<<Here describe the issue or question you have about the
//!   VK_EXT_shader_stencil_export extension>>)
//!# New constants
//! - [`EXT_SHADER_STENCIL_EXPORT_EXTENSION_NAME`]
//! - [`EXT_SHADER_STENCIL_EXPORT_SPEC_VERSION`]
//!# Version History
//! - Revision 1, 2017-07-19 (Dominik Witczak)
//! - Initial draft
//!# Other info
//! * 2017-07-19
//! * No known IP claims.
//!*
//! - This extension requires
//![`SPV_EXT_shader_stencil_export`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/EXT/SPV_EXT_shader_stencil_export.html)
//! - This extension provides API support for
//![`GL_ARB_shader_stencil_export`](https://www.khronos.org/registry/OpenGL/extensions/ARB/ARB_shader_stencil_export.txt)
//!
//!*
//! - Dominik Witczak, AMD
//! - Daniel Rakos, AMD
//! - Rex Xu, AMD
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
#[doc(alias = "VK_EXT_SHADER_STENCIL_EXPORT_SPEC_VERSION")]
pub const EXT_SHADER_STENCIL_EXPORT_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_SHADER_STENCIL_EXPORT_EXTENSION_NAME")]
pub const EXT_SHADER_STENCIL_EXPORT_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_shader_stencil_export");
