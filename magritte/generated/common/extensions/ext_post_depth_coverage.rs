use crate::cstr;
use std::ffi::CStr;
#[doc(alias = "VK_EXT_POST_DEPTH_COVERAGE_SPEC_VERSION")]
pub const EXT_POST_DEPTH_COVERAGE_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_EXT_POST_DEPTH_COVERAGE_EXTENSION_NAME")]
pub const EXT_POST_DEPTH_COVERAGE_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_post_depth_coverage");