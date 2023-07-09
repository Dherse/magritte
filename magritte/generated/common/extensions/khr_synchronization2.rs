use crate::cstr;
use std::ffi::CStr;
#[doc(alias = "VkFlags64")]
pub type Flags64 = u64;
#[doc(alias = "VK_KHR_SYNCHRONIZATION_2_SPEC_VERSION")]
pub const KHR_SYNCHRONIZATION_2_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_KHR_SYNCHRONIZATION_2_EXTENSION_NAME")]
pub const KHR_SYNCHRONIZATION_2_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_synchronization2");
