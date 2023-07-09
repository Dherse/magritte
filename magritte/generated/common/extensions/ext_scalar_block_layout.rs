use crate::cstr;
use std::ffi::CStr;
#[doc(alias = "VK_EXT_SCALAR_BLOCK_LAYOUT_SPEC_VERSION")]
pub const EXT_SCALAR_BLOCK_LAYOUT_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_EXT_SCALAR_BLOCK_LAYOUT_EXTENSION_NAME")]
pub const EXT_SCALAR_BLOCK_LAYOUT_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_scalar_block_layout");
