use crate::native::vulkan1_1::{CommandPoolTrimFlags, FNTrimCommandPool};
///See [`CommandPoolTrimFlags`]
#[doc(alias = "VkCommandPoolTrimFlagsKHR")]
pub type CommandPoolTrimFlagsKHR = CommandPoolTrimFlags;
pub use crate::common::extensions::khr_maintenance1::{
    KHR_MAINTENANCE1_EXTENSION_NAME, KHR_MAINTENANCE1_SPEC_VERSION, KHR_MAINTENANCE_1_EXTENSION_NAME,
    KHR_MAINTENANCE_1_SPEC_VERSION,
};
///See [`trim_command_pool`]
#[doc(alias = "vkTrimCommandPoolKHR")]
pub type FNTrimCommandPoolKhr = FNTrimCommandPool;
