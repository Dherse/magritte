use crate::cstr;
use std::ffi::CStr;
#[doc(alias = "VK_KHR_DRAW_INDIRECT_COUNT_SPEC_VERSION")]
pub const KHR_DRAW_INDIRECT_COUNT_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_KHR_DRAW_INDIRECT_COUNT_EXTENSION_NAME")]
pub const KHR_DRAW_INDIRECT_COUNT_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_draw_indirect_count");
