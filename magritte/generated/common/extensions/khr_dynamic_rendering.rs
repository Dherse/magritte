use crate::cstr;
use std::ffi::CStr;
#[doc(alias = "VK_KHR_DYNAMIC_RENDERING_SPEC_VERSION")]
pub const KHR_DYNAMIC_RENDERING_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_KHR_DYNAMIC_RENDERING_EXTENSION_NAME")]
pub const KHR_DYNAMIC_RENDERING_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_dynamic_rendering");
