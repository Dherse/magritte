use crate::cstr;
use std::ffi::CStr;
#[doc(alias = "VK_FUCHSIA_EXTERNAL_SEMAPHORE_SPEC_VERSION")]
pub const FUCHSIA_EXTERNAL_SEMAPHORE_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_FUCHSIA_EXTERNAL_SEMAPHORE_EXTENSION_NAME")]
pub const FUCHSIA_EXTERNAL_SEMAPHORE_EXTENSION_NAME: &'static CStr = cstr!("VK_FUCHSIA_external_semaphore");