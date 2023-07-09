use crate::native::vulkan1_1::{MemoryDedicatedAllocateInfo, MemoryDedicatedRequirements};
///See [`MemoryDedicatedRequirements`]
#[doc(alias = "VkMemoryDedicatedRequirementsKHR")]
pub type MemoryDedicatedRequirementsKHR = MemoryDedicatedRequirements;
///See [`MemoryDedicatedAllocateInfo`]
#[doc(alias = "VkMemoryDedicatedAllocateInfoKHR")]
pub type MemoryDedicatedAllocateInfoKHR = MemoryDedicatedAllocateInfo;
pub use crate::common::extensions::khr_dedicated_allocation::{
    KHR_DEDICATED_ALLOCATION_EXTENSION_NAME, KHR_DEDICATED_ALLOCATION_SPEC_VERSION,
};
