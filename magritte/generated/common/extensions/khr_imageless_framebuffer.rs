use crate::cstr;
use std::ffi::CStr;
#[doc(alias = "VK_KHR_IMAGELESS_FRAMEBUFFER_SPEC_VERSION")]
pub const KHR_IMAGELESS_FRAMEBUFFER_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_KHR_IMAGELESS_FRAMEBUFFER_EXTENSION_NAME")]
pub const KHR_IMAGELESS_FRAMEBUFFER_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_imageless_framebuffer");