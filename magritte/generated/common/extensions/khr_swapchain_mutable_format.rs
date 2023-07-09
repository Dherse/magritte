use crate::cstr;
use std::ffi::CStr;
#[doc(alias = "VK_KHR_SWAPCHAIN_MUTABLE_FORMAT_SPEC_VERSION")]
pub const KHR_SWAPCHAIN_MUTABLE_FORMAT_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_KHR_SWAPCHAIN_MUTABLE_FORMAT_EXTENSION_NAME")]
pub const KHR_SWAPCHAIN_MUTABLE_FORMAT_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_swapchain_mutable_format");