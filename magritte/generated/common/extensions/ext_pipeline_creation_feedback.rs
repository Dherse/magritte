use crate::cstr;
use std::ffi::CStr;
#[doc(alias = "VK_EXT_PIPELINE_CREATION_FEEDBACK_SPEC_VERSION")]
pub const EXT_PIPELINE_CREATION_FEEDBACK_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_EXT_PIPELINE_CREATION_FEEDBACK_EXTENSION_NAME")]
pub const EXT_PIPELINE_CREATION_FEEDBACK_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_pipeline_creation_feedback");
