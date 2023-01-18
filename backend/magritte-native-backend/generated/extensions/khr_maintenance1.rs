use crate::{
    cstr,
    vulkan1_1::{CommandPoolTrimFlags, FNTrimCommandPool},
};
use std::ffi::CStr;
///See [`CommandPoolTrimFlags`]
#[doc(alias = "VkCommandPoolTrimFlagsKHR")]
pub type CommandPoolTrimFlagsKHR = CommandPoolTrimFlags;
#[doc(alias = "VK_KHR_MAINTENANCE_1_SPEC_VERSION")]
pub const KHR_MAINTENANCE_1_SPEC_VERSION: u32 = 2;
#[doc(alias = "VK_KHR_MAINTENANCE_1_EXTENSION_NAME")]
pub const KHR_MAINTENANCE_1_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_maintenance1");
///See [`KHR_MAINTENANCE_1_SPEC_VERSION`]
#[doc(alias = "VK_KHR_MAINTENANCE1_SPEC_VERSION")]
pub const KHR_MAINTENANCE1_SPEC_VERSION: u32 = KHR_MAINTENANCE_1_SPEC_VERSION;
///See [`KHR_MAINTENANCE_1_EXTENSION_NAME`]
#[doc(alias = "VK_KHR_MAINTENANCE1_EXTENSION_NAME")]
pub const KHR_MAINTENANCE1_EXTENSION_NAME: &'static CStr = KHR_MAINTENANCE_1_EXTENSION_NAME;
///See [`trim_command_pool`]
#[doc(alias = "vkTrimCommandPoolKHR")]
pub type FNTrimCommandPoolKhr = FNTrimCommandPool;
