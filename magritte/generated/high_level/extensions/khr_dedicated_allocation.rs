pub use crate::common::extensions::khr_dedicated_allocation::{
    KHR_DEDICATED_ALLOCATION_EXTENSION_NAME, KHR_DEDICATED_ALLOCATION_SPEC_VERSION,
};
use crate::vulkan1_1::{MemoryDedicatedAllocateInfo, MemoryDedicatedRequirements};
#[doc(alias = "VkMemoryDedicatedRequirementsKHR")]
pub type MemoryDedicatedRequirementsKHR = MemoryDedicatedRequirements;
#[doc(alias = "VkMemoryDedicatedAllocateInfoKHR")]
pub type MemoryDedicatedAllocateInfoKHR = MemoryDedicatedAllocateInfo;
