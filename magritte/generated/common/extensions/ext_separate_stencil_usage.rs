use crate::cstr;
use std::ffi::CStr;
#[doc(alias = "VK_EXT_SEPARATE_STENCIL_USAGE_SPEC_VERSION")]
pub const EXT_SEPARATE_STENCIL_USAGE_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_EXT_SEPARATE_STENCIL_USAGE_EXTENSION_NAME")]
pub const EXT_SEPARATE_STENCIL_USAGE_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_separate_stencil_usage");