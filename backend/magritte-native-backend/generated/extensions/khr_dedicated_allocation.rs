//!# [VK_KHR_dedicated_allocation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_dedicated_allocation.html)
# ! [doc = include_str ! ("../../../../doc/extensions/khr_dedicated_allocation/VK_KHR_dedicated_allocation.md")]
use crate::{
    cstr,
    vulkan1_1::{MemoryDedicatedAllocateInfo, MemoryDedicatedRequirements},
};
use std::ffi::CStr;
///See [`MemoryDedicatedRequirements`]
#[doc(alias = "VkMemoryDedicatedRequirementsKHR")]
pub type MemoryDedicatedRequirementsKHR = MemoryDedicatedRequirements;
///See [`MemoryDedicatedAllocateInfo`]
#[doc(alias = "VkMemoryDedicatedAllocateInfoKHR")]
pub type MemoryDedicatedAllocateInfoKHR = MemoryDedicatedAllocateInfo;
#[doc(alias = "VK_KHR_DEDICATED_ALLOCATION_SPEC_VERSION")]
pub const KHR_DEDICATED_ALLOCATION_SPEC_VERSION: u32 = 3;
#[doc(alias = "VK_KHR_DEDICATED_ALLOCATION_EXTENSION_NAME")]
pub const KHR_DEDICATED_ALLOCATION_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_dedicated_allocation");
