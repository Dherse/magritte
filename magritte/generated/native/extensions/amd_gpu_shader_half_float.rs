//!# [VK_AMD_gpu_shader_half_float](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_AMD_gpu_shader_half_float.html)
# ! [doc = include_str ! ("../../../../doc/extensions/amd_gpu_shader_half_float/VK_AMD_gpu_shader_half_float.md")]
use crate::cstr;
use std::ffi::CStr;
#[doc(alias = "VK_AMD_GPU_SHADER_HALF_FLOAT_SPEC_VERSION")]
pub const AMD_GPU_SHADER_HALF_FLOAT_SPEC_VERSION: u32 = 2;
#[doc(alias = "VK_AMD_GPU_SHADER_HALF_FLOAT_EXTENSION_NAME")]
pub const AMD_GPU_SHADER_HALF_FLOAT_EXTENSION_NAME: &'static CStr = cstr!("VK_AMD_gpu_shader_half_float");
