use crate::{common::vulkan1_0::MAX_DEVICE_GROUP_SIZE, cstr};
use std::ffi::CStr;
#[doc(alias = "VK_KHR_DEVICE_GROUP_CREATION_SPEC_VERSION")]
pub const KHR_DEVICE_GROUP_CREATION_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_KHR_DEVICE_GROUP_CREATION_EXTENSION_NAME")]
pub const KHR_DEVICE_GROUP_CREATION_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_device_group_creation");
///See [`MAX_DEVICE_GROUP_SIZE`]
#[doc(alias = "VK_MAX_DEVICE_GROUP_SIZE_KHR")]
pub const MAX_DEVICE_GROUP_SIZE_KHR: u32 = MAX_DEVICE_GROUP_SIZE;
