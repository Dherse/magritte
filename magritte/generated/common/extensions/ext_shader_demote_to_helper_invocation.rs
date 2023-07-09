use crate::cstr;
use std::ffi::CStr;
#[doc(alias = "VK_EXT_SHADER_DEMOTE_TO_HELPER_INVOCATION_SPEC_VERSION")]
pub const EXT_SHADER_DEMOTE_TO_HELPER_INVOCATION_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_EXT_SHADER_DEMOTE_TO_HELPER_INVOCATION_EXTENSION_NAME")]
pub const EXT_SHADER_DEMOTE_TO_HELPER_INVOCATION_EXTENSION_NAME: &'static CStr =
    cstr!("VK_EXT_shader_demote_to_helper_invocation");
