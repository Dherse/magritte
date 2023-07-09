use crate::cstr;
use std::ffi::CStr;
#[doc(alias = "VK_EXT_QUEUE_FAMILY_FOREIGN_SPEC_VERSION")]
pub const EXT_QUEUE_FAMILY_FOREIGN_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_EXT_QUEUE_FAMILY_FOREIGN_EXTENSION_NAME")]
pub const EXT_QUEUE_FAMILY_FOREIGN_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_queue_family_foreign");
