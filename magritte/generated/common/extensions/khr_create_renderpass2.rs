use crate::cstr;
use std::ffi::CStr;
#[doc(alias = "VK_KHR_CREATE_RENDERPASS_2_SPEC_VERSION")]
pub const KHR_CREATE_RENDERPASS_2_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_KHR_CREATE_RENDERPASS_2_EXTENSION_NAME")]
pub const KHR_CREATE_RENDERPASS_2_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_create_renderpass2");