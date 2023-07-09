use crate::cstr;
use std::ffi::CStr;
#[doc(alias = "VK_KHR_SEPARATE_DEPTH_STENCIL_LAYOUTS_SPEC_VERSION")]
pub const KHR_SEPARATE_DEPTH_STENCIL_LAYOUTS_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_KHR_SEPARATE_DEPTH_STENCIL_LAYOUTS_EXTENSION_NAME")]
pub const KHR_SEPARATE_DEPTH_STENCIL_LAYOUTS_EXTENSION_NAME: &'static CStr =
    cstr!("VK_KHR_separate_depth_stencil_layouts");
