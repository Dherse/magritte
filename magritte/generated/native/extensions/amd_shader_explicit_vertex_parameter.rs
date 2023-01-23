//!# [VK_AMD_shader_explicit_vertex_parameter](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_AMD_shader_explicit_vertex_parameter.html)
# ! [doc = include_str ! ("../../../../doc/extensions/amd_shader_explicit_vertex_parameter/VK_AMD_shader_explicit_vertex_parameter.md")]
use crate::cstr;
use std::ffi::CStr;
#[doc(alias = "VK_AMD_SHADER_EXPLICIT_VERTEX_PARAMETER_SPEC_VERSION")]
pub const AMD_SHADER_EXPLICIT_VERTEX_PARAMETER_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_AMD_SHADER_EXPLICIT_VERTEX_PARAMETER_EXTENSION_NAME")]
pub const AMD_SHADER_EXPLICIT_VERTEX_PARAMETER_EXTENSION_NAME: &'static CStr =
    cstr!("VK_AMD_shader_explicit_vertex_parameter");
