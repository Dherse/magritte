use crate::cstr;
use std::ffi::CStr;
#[doc(alias = "VK_AMD_GPU_SHADER_HALF_FLOAT_SPEC_VERSION")]
pub const AMD_GPU_SHADER_HALF_FLOAT_SPEC_VERSION: u32 = 2;
#[doc(alias = "VK_AMD_GPU_SHADER_HALF_FLOAT_EXTENSION_NAME")]
pub const AMD_GPU_SHADER_HALF_FLOAT_EXTENSION_NAME: &'static CStr = cstr!("VK_AMD_gpu_shader_half_float");
