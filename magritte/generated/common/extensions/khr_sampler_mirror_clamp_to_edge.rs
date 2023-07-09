use crate::cstr;
use std::ffi::CStr;
#[doc(alias = "VK_KHR_SAMPLER_MIRROR_CLAMP_TO_EDGE_SPEC_VERSION")]
pub const KHR_SAMPLER_MIRROR_CLAMP_TO_EDGE_SPEC_VERSION: u32 = 3;
#[doc(alias = "VK_KHR_SAMPLER_MIRROR_CLAMP_TO_EDGE_EXTENSION_NAME")]
pub const KHR_SAMPLER_MIRROR_CLAMP_TO_EDGE_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_sampler_mirror_clamp_to_edge");