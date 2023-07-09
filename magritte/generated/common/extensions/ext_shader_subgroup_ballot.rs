use crate::cstr;
use std::ffi::CStr;
#[doc(alias = "VK_EXT_SHADER_SUBGROUP_BALLOT_SPEC_VERSION")]
pub const EXT_SHADER_SUBGROUP_BALLOT_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_EXT_SHADER_SUBGROUP_BALLOT_EXTENSION_NAME")]
pub const EXT_SHADER_SUBGROUP_BALLOT_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_shader_subgroup_ballot");
