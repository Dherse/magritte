use crate::cstr;
use std::ffi::CStr;
#[doc(alias = "VK_GOOGLE_HLSL_FUNCTIONALITY_1_SPEC_VERSION")]
pub const GOOGLE_HLSL_FUNCTIONALITY_1_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_GOOGLE_HLSL_FUNCTIONALITY_1_EXTENSION_NAME")]
pub const GOOGLE_HLSL_FUNCTIONALITY_1_EXTENSION_NAME: &'static CStr = cstr!("VK_GOOGLE_hlsl_functionality1");
///See [`GOOGLE_HLSL_FUNCTIONALITY_1_SPEC_VERSION`]
#[doc(alias = "VK_GOOGLE_HLSL_FUNCTIONALITY1_SPEC_VERSION")]
pub const GOOGLE_HLSL_FUNCTIONALITY1_SPEC_VERSION: u32 = GOOGLE_HLSL_FUNCTIONALITY_1_SPEC_VERSION;
///See [`GOOGLE_HLSL_FUNCTIONALITY_1_EXTENSION_NAME`]
#[doc(alias = "VK_GOOGLE_HLSL_FUNCTIONALITY1_EXTENSION_NAME")]
pub const GOOGLE_HLSL_FUNCTIONALITY1_EXTENSION_NAME: &'static CStr = GOOGLE_HLSL_FUNCTIONALITY_1_EXTENSION_NAME;