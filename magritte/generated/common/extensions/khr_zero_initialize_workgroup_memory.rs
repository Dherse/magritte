use crate::cstr;
use std::ffi::CStr;
#[doc(alias = "VK_KHR_ZERO_INITIALIZE_WORKGROUP_MEMORY_SPEC_VERSION")]
pub const KHR_ZERO_INITIALIZE_WORKGROUP_MEMORY_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_KHR_ZERO_INITIALIZE_WORKGROUP_MEMORY_EXTENSION_NAME")]
pub const KHR_ZERO_INITIALIZE_WORKGROUP_MEMORY_EXTENSION_NAME: &'static CStr =
    cstr!("VK_KHR_zero_initialize_workgroup_memory");
