use crate::cstr;
use std::ffi::CStr;
#[doc(alias = "VK_AMD_BUFFER_MARKER_SPEC_VERSION")]
pub const AMD_BUFFER_MARKER_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_AMD_BUFFER_MARKER_EXTENSION_NAME")]
pub const AMD_BUFFER_MARKER_EXTENSION_NAME: &'static CStr = cstr!("VK_AMD_buffer_marker");
