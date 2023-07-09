use crate::cstr;
use std::ffi::CStr;
#[doc(alias = "VK_EXT_INLINE_UNIFORM_BLOCK_SPEC_VERSION")]
pub const EXT_INLINE_UNIFORM_BLOCK_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_EXT_INLINE_UNIFORM_BLOCK_EXTENSION_NAME")]
pub const EXT_INLINE_UNIFORM_BLOCK_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_inline_uniform_block");
