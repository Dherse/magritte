use crate::cstr;
use std::ffi::CStr;
#[doc(alias = "VK_KHR_MAINTENANCE_2_SPEC_VERSION")]
pub const KHR_MAINTENANCE_2_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_KHR_MAINTENANCE_2_EXTENSION_NAME")]
pub const KHR_MAINTENANCE_2_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_maintenance2");
///See [`KHR_MAINTENANCE_2_SPEC_VERSION`]
#[doc(alias = "VK_KHR_MAINTENANCE2_SPEC_VERSION")]
pub const KHR_MAINTENANCE2_SPEC_VERSION: u32 = KHR_MAINTENANCE_2_SPEC_VERSION;
///See [`KHR_MAINTENANCE_2_EXTENSION_NAME`]
#[doc(alias = "VK_KHR_MAINTENANCE2_EXTENSION_NAME")]
pub const KHR_MAINTENANCE2_EXTENSION_NAME: &'static CStr = KHR_MAINTENANCE_2_EXTENSION_NAME;