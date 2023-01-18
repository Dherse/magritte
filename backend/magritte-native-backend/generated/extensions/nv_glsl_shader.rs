use crate::cstr;
use std::ffi::CStr;
#[doc(alias = "VK_NV_GLSL_SHADER_SPEC_VERSION")]
pub const NV_GLSL_SHADER_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_NV_GLSL_SHADER_EXTENSION_NAME")]
pub const NV_GLSL_SHADER_EXTENSION_NAME: &'static CStr = cstr!("VK_NV_glsl_shader");
