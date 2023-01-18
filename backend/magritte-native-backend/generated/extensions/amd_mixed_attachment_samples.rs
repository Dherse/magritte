use crate::cstr;
use std::ffi::CStr;
#[doc(alias = "VK_AMD_MIXED_ATTACHMENT_SAMPLES_SPEC_VERSION")]
pub const AMD_MIXED_ATTACHMENT_SAMPLES_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_AMD_MIXED_ATTACHMENT_SAMPLES_EXTENSION_NAME")]
pub const AMD_MIXED_ATTACHMENT_SAMPLES_EXTENSION_NAME: &'static CStr = cstr!("VK_AMD_mixed_attachment_samples");
