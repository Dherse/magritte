//!# [VK_EXT_host_query_reset](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_host_query_reset.html)
# ! [doc = include_str ! ("../../../../doc/extensions/ext_host_query_reset/VK_EXT_host_query_reset.md")]
use crate::{
    cstr,
    vulkan1_2::{FNResetQueryPool, PhysicalDeviceHostQueryResetFeatures},
};
use std::ffi::CStr;
///See [`PhysicalDeviceHostQueryResetFeatures`]
#[doc(alias = "VkPhysicalDeviceHostQueryResetFeaturesEXT")]
pub type PhysicalDeviceHostQueryResetFeaturesEXT = PhysicalDeviceHostQueryResetFeatures;
#[doc(alias = "VK_EXT_HOST_QUERY_RESET_SPEC_VERSION")]
pub const EXT_HOST_QUERY_RESET_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_EXT_HOST_QUERY_RESET_EXTENSION_NAME")]
pub const EXT_HOST_QUERY_RESET_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_host_query_reset");
///See [`reset_query_pool`]
#[doc(alias = "vkResetQueryPoolEXT")]
pub type FNResetQueryPoolExt = FNResetQueryPool;
