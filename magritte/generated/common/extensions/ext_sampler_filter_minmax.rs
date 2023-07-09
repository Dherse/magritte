use crate::cstr;
use std::ffi::CStr;
#[doc(alias = "VK_EXT_SAMPLER_FILTER_MINMAX_SPEC_VERSION")]
pub const EXT_SAMPLER_FILTER_MINMAX_SPEC_VERSION: u32 = 2;
#[doc(alias = "VK_EXT_SAMPLER_FILTER_MINMAX_EXTENSION_NAME")]
pub const EXT_SAMPLER_FILTER_MINMAX_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_sampler_filter_minmax");
