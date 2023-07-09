use crate::{common::vulkan1_0::QUEUE_FAMILY_EXTERNAL, cstr};
use std::ffi::CStr;
#[doc(alias = "VK_KHR_EXTERNAL_MEMORY_SPEC_VERSION")]
pub const KHR_EXTERNAL_MEMORY_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_KHR_EXTERNAL_MEMORY_EXTENSION_NAME")]
pub const KHR_EXTERNAL_MEMORY_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_external_memory");
///See [`QUEUE_FAMILY_EXTERNAL`]
#[doc(alias = "VK_QUEUE_FAMILY_EXTERNAL_KHR")]
pub const QUEUE_FAMILY_EXTERNAL_KHR: u32 = QUEUE_FAMILY_EXTERNAL;
