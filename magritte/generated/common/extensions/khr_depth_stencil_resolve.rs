use crate::cstr;
use std::ffi::CStr;
#[doc(alias = "VK_KHR_DEPTH_STENCIL_RESOLVE_SPEC_VERSION")]
pub const KHR_DEPTH_STENCIL_RESOLVE_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_KHR_DEPTH_STENCIL_RESOLVE_EXTENSION_NAME")]
pub const KHR_DEPTH_STENCIL_RESOLVE_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_depth_stencil_resolve");
