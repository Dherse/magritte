use crate::cstr;
use std::ffi::CStr;
#[doc(alias = "VK_KHR_SPIRV_1_4_SPEC_VERSION")]
pub const KHR_SPIRV_1_4_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_KHR_SPIRV_1_4_EXTENSION_NAME")]
pub const KHR_SPIRV_1_4_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_spirv_1_4");
