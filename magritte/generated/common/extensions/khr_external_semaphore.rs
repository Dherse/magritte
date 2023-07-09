use crate::cstr;
use std::ffi::CStr;
#[doc(alias = "VK_KHR_EXTERNAL_SEMAPHORE_SPEC_VERSION")]
pub const KHR_EXTERNAL_SEMAPHORE_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_KHR_EXTERNAL_SEMAPHORE_EXTENSION_NAME")]
pub const KHR_EXTERNAL_SEMAPHORE_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_external_semaphore");
