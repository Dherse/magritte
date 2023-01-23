//!# [VK_GOOGLE_decorate_string](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_GOOGLE_decorate_string.html)
# ! [doc = include_str ! ("../../../../doc/extensions/google_decorate_string/VK_GOOGLE_decorate_string.md")]
use crate::cstr;
use std::ffi::CStr;
#[doc(alias = "VK_GOOGLE_DECORATE_STRING_SPEC_VERSION")]
pub const GOOGLE_DECORATE_STRING_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_GOOGLE_DECORATE_STRING_EXTENSION_NAME")]
pub const GOOGLE_DECORATE_STRING_EXTENSION_NAME: &'static CStr = cstr!("VK_GOOGLE_decorate_string");
