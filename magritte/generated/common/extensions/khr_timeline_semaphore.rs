use crate::cstr;
use std::ffi::CStr;
#[doc(alias = "VK_KHR_TIMELINE_SEMAPHORE_SPEC_VERSION")]
pub const KHR_TIMELINE_SEMAPHORE_SPEC_VERSION: u32 = 2;
#[doc(alias = "VK_KHR_TIMELINE_SEMAPHORE_EXTENSION_NAME")]
pub const KHR_TIMELINE_SEMAPHORE_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_timeline_semaphore");
