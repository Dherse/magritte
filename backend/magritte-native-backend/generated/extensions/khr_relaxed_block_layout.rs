use crate::cstr;
use std::ffi::CStr;
#[doc(alias = "VK_KHR_RELAXED_BLOCK_LAYOUT_SPEC_VERSION")]
pub const KHR_RELAXED_BLOCK_LAYOUT_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_KHR_RELAXED_BLOCK_LAYOUT_EXTENSION_NAME")]
pub const KHR_RELAXED_BLOCK_LAYOUT_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_relaxed_block_layout");
