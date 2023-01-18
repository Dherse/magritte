use crate::cstr;
use std::ffi::CStr;
#[doc(alias = "VK_KHR_SHADER_NON_SEMANTIC_INFO_SPEC_VERSION")]
pub const KHR_SHADER_NON_SEMANTIC_INFO_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_KHR_SHADER_NON_SEMANTIC_INFO_EXTENSION_NAME")]
pub const KHR_SHADER_NON_SEMANTIC_INFO_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_shader_non_semantic_info");
