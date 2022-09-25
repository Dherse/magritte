//![VK_AMD_gpu_shader_int16](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_AMD_gpu_shader_int16.html) - device extension
//!# Description
//!This extension adds support for using 16-bit integer variables in shaders.
# ! [doc = concat ! ("# " , "Revision")]
//!2
# ! [doc = concat ! ("# " , "Dependencies")]
//! - Requires Vulkan 1.0
# ! [doc = concat ! ("# " , "Deprecation State")]
//! - *Deprecated* by `[`khr_shader_float16_int8`]` extension  - Which in turn was *promoted* to [Vulkan 1.2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.2-promotions)
# ! [doc = concat ! ("# " , "Contacts")]
//! - Qun Lin [linqun](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_AMD_gpu_shader_int16]
//!   @linqun%0A<<Here describe the issue or question you have about the VK_AMD_gpu_shader_int16
//!   extension>>)
# ! [doc = concat ! ("# " , "New constants")]
//! - [`AMD_GPU_SHADER_INT16_EXTENSION_NAME`]
//! - [`AMD_GPU_SHADER_INT16_SPEC_VERSION`]
# ! [doc = concat ! ("# " , "Version history")]
//! - Revision 2, 2019-04-11 (Tobias Hector)  - Marked as deprecated
//! - Revision 1, 2017-06-18 (Dominik Witczak)  - First version
//!# Other info
//! * 2019-04-11
//! * No known IP claims.
//! * - This extension requires [`SPV_AMD_gpu_shader_int16`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/AMD/SPV_AMD_gpu_shader_int16.html)
//!   - This extension provides API support for [`GL_AMD_gpu_shader_int16`](https://www.khronos.org/registry/OpenGL/extensions/AMD/AMD_gpu_shader_int16.txt)
//! * - Daniel Rakos, AMD  - Dominik Witczak, AMD  - Matthaeus G. Chajdas, AMD  - Rex Xu, AMD  -
//!   Timothy Lottes, AMD  - Zhi Cai, AMD
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
#[doc(alias = "VK_AMD_GPU_SHADER_INT16_SPEC_VERSION")]
pub const AMD_GPU_SHADER_INT16_SPEC_VERSION: u32 = 2;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_AMD_GPU_SHADER_INT16_EXTENSION_NAME")]
pub const AMD_GPU_SHADER_INT16_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_AMD_gpu_shader_int16");
