//![VK_AMD_gpu_shader_half_float](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_AMD_gpu_shader_half_float.html) - device extension
//!# Description
//!This extension adds support for using half float variables in shaders.
//!# Revision
//!2
//!# Dependencies
//! - *Deprecated* by `[`khr_shader_float16_int8`]` extension  - Which in turn was *promoted* to [Vulkan 1.2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.2-promotions)
//!# Dependencies
//! - Requires Vulkan 1.0
//!# Contacts
//! - Dominik Witczak [dominikwitczakamd](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_AMD_gpu_shader_half_float]
//!   @dominikwitczakamd%0A<<Here describe the issue or question you have about the
//!   VK_AMD_gpu_shader_half_float extension>>)
//!# New constants
//! - [`AMD_GPU_SHADER_HALF_FLOAT_EXTENSION_NAME`]
//! - [`AMD_GPU_SHADER_HALF_FLOAT_SPEC_VERSION`]
//!# Version History
//! - Revision 2, 2019-04-11 (Tobias Hector)  - Marked as deprecated
//! - Revision 1, 2016-09-21 (Dominik Witczak)  - Initial draft
//!# Other info
//! * 2019-04-11
//! * No known IP claims.
//! * - This extension requires [`SPV_AMD_gpu_shader_half_float`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/AMD/SPV_AMD_gpu_shader_half_float.html)
//!   - This extension provides API support for [`GL_AMD_gpu_shader_half_float`](https://www.khronos.org/registry/OpenGL/extensions/AMD/AMD_gpu_shader_half_float.txt)
//! * - Daniel Rakos, AMD  - Dominik Witczak, AMD  - Donglin Wei, AMD  - Graham Sellers, AMD  - Qun
//!   Lin, AMD  - Rex Xu, AMD
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
#[doc(alias = "VK_AMD_GPU_SHADER_HALF_FLOAT_SPEC_VERSION")]
pub const AMD_GPU_SHADER_HALF_FLOAT_SPEC_VERSION: u32 = 2;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_AMD_GPU_SHADER_HALF_FLOAT_EXTENSION_NAME")]
pub const AMD_GPU_SHADER_HALF_FLOAT_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_AMD_gpu_shader_half_float");
