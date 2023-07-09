use crate::{common::vulkan1_0::LUID_SIZE, cstr};
use std::ffi::CStr;
#[doc(alias = "VK_KHR_EXTERNAL_FENCE_CAPABILITIES_SPEC_VERSION")]
pub const KHR_EXTERNAL_FENCE_CAPABILITIES_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_KHR_EXTERNAL_FENCE_CAPABILITIES_EXTENSION_NAME")]
pub const KHR_EXTERNAL_FENCE_CAPABILITIES_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_external_fence_capabilities");
///See [`LUID_SIZE`]
#[doc(alias = "VK_LUID_SIZE_KHR")]
pub const LUID_SIZE_KHR: u32 = LUID_SIZE;
