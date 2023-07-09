use crate::cstr;
use std::ffi::CStr;
#[doc(alias = "VK_AMD_SHADER_EXPLICIT_VERTEX_PARAMETER_SPEC_VERSION")]
pub const AMD_SHADER_EXPLICIT_VERTEX_PARAMETER_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_AMD_SHADER_EXPLICIT_VERTEX_PARAMETER_EXTENSION_NAME")]
pub const AMD_SHADER_EXPLICIT_VERTEX_PARAMETER_EXTENSION_NAME: &'static CStr =
    cstr!("VK_AMD_shader_explicit_vertex_parameter");
