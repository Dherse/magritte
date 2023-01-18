use crate::cstr;
use std::ffi::CStr;
#[doc(alias = "VK_EXT_LOAD_STORE_OP_NONE_SPEC_VERSION")]
pub const EXT_LOAD_STORE_OP_NONE_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_EXT_LOAD_STORE_OP_NONE_EXTENSION_NAME")]
pub const EXT_LOAD_STORE_OP_NONE_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_load_store_op_none");
