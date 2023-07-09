use crate::cstr;
use std::ffi::CStr;
#[doc(alias = "VK_AMD_SHADER_CORE_PROPERTIES_SPEC_VERSION")]
pub const AMD_SHADER_CORE_PROPERTIES_SPEC_VERSION: u32 = 2;
#[doc(alias = "VK_AMD_SHADER_CORE_PROPERTIES_EXTENSION_NAME")]
pub const AMD_SHADER_CORE_PROPERTIES_EXTENSION_NAME: &'static CStr = cstr!("VK_AMD_shader_core_properties");
