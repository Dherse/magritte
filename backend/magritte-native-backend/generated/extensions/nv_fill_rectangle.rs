use crate::cstr;
use std::ffi::CStr;
#[doc(alias = "VK_NV_FILL_RECTANGLE_SPEC_VERSION")]
pub const NV_FILL_RECTANGLE_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_NV_FILL_RECTANGLE_EXTENSION_NAME")]
pub const NV_FILL_RECTANGLE_EXTENSION_NAME: &'static CStr = cstr!("VK_NV_fill_rectangle");
