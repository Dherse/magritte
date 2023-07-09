use crate::cstr;
use std::ffi::CStr;
#[doc(alias = "VK_EXT_SUBGROUP_SIZE_CONTROL_SPEC_VERSION")]
pub const EXT_SUBGROUP_SIZE_CONTROL_SPEC_VERSION: u32 = 2;
#[doc(alias = "VK_EXT_SUBGROUP_SIZE_CONTROL_EXTENSION_NAME")]
pub const EXT_SUBGROUP_SIZE_CONTROL_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_subgroup_size_control");
