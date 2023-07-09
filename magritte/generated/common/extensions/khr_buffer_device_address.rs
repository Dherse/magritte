use crate::cstr;
use std::ffi::CStr;
#[doc(alias = "VK_KHR_BUFFER_DEVICE_ADDRESS_SPEC_VERSION")]
pub const KHR_BUFFER_DEVICE_ADDRESS_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_KHR_BUFFER_DEVICE_ADDRESS_EXTENSION_NAME")]
pub const KHR_BUFFER_DEVICE_ADDRESS_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_buffer_device_address");