use crate::cstr;
use std::ffi::CStr;
#[doc(alias = "VK_AMD_GCN_SHADER_SPEC_VERSION")]
pub const AMD_GCN_SHADER_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_AMD_GCN_SHADER_EXTENSION_NAME")]
pub const AMD_GCN_SHADER_EXTENSION_NAME: &'static CStr = cstr!("VK_AMD_gcn_shader");
