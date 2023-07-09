use crate::cstr;
use std::ffi::CStr;
#[doc(alias = "VK_NV_VIEWPORT_ARRAY_2_SPEC_VERSION")]
pub const NV_VIEWPORT_ARRAY_2_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_NV_VIEWPORT_ARRAY_2_EXTENSION_NAME")]
pub const NV_VIEWPORT_ARRAY_2_EXTENSION_NAME: &'static CStr = cstr!("VK_NV_viewport_array2");
///See [`NV_VIEWPORT_ARRAY_2_SPEC_VERSION`]
#[doc(alias = "VK_NV_VIEWPORT_ARRAY2_SPEC_VERSION")]
pub const NV_VIEWPORT_ARRAY2_SPEC_VERSION: u32 = NV_VIEWPORT_ARRAY_2_SPEC_VERSION;
///See [`NV_VIEWPORT_ARRAY_2_EXTENSION_NAME`]
#[doc(alias = "VK_NV_VIEWPORT_ARRAY2_EXTENSION_NAME")]
pub const NV_VIEWPORT_ARRAY2_EXTENSION_NAME: &'static CStr = NV_VIEWPORT_ARRAY_2_EXTENSION_NAME;