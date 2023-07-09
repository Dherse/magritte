use crate::cstr;
use std::ffi::CStr;
#[doc(alias = "VK_KHR_SHADER_TERMINATE_INVOCATION_SPEC_VERSION")]
pub const KHR_SHADER_TERMINATE_INVOCATION_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_KHR_SHADER_TERMINATE_INVOCATION_EXTENSION_NAME")]
pub const KHR_SHADER_TERMINATE_INVOCATION_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_shader_terminate_invocation");
