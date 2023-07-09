use crate::cstr;
use std::ffi::CStr;
#[doc(alias = "VK_NV_SAMPLE_MASK_OVERRIDE_COVERAGE_SPEC_VERSION")]
pub const NV_SAMPLE_MASK_OVERRIDE_COVERAGE_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_NV_SAMPLE_MASK_OVERRIDE_COVERAGE_EXTENSION_NAME")]
pub const NV_SAMPLE_MASK_OVERRIDE_COVERAGE_EXTENSION_NAME: &'static CStr = cstr!("VK_NV_sample_mask_override_coverage");