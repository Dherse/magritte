pub use crate::common::extensions::khr_bind_memory2::{
    KHR_BIND_MEMORY_2_EXTENSION_NAME, KHR_BIND_MEMORY_2_SPEC_VERSION,
};
use crate::vulkan1_1::{BindBufferMemoryInfo, BindImageMemoryInfo};
#[doc(alias = "VkBindBufferMemoryInfoKHR")]
pub type BindBufferMemoryInfoKHR = BindBufferMemoryInfo;
#[doc(alias = "VkBindImageMemoryInfoKHR")]
pub type BindImageMemoryInfoKHR = BindImageMemoryInfo;
