use crate::native::vulkan1_1::{BindBufferMemoryInfo, BindImageMemoryInfo, FNBindBufferMemory2, FNBindImageMemory2};
///See [`BindBufferMemoryInfo`]
#[doc(alias = "VkBindBufferMemoryInfoKHR")]
pub type BindBufferMemoryInfoKHR = BindBufferMemoryInfo;
///See [`BindImageMemoryInfo`]
#[doc(alias = "VkBindImageMemoryInfoKHR")]
pub type BindImageMemoryInfoKHR = BindImageMemoryInfo;
pub use crate::common::extensions::khr_bind_memory2::{
    KHR_BIND_MEMORY_2_EXTENSION_NAME, KHR_BIND_MEMORY_2_SPEC_VERSION,
};
///See [`bind_buffer_memory2`]
#[doc(alias = "vkBindBufferMemory2KHR")]
pub type FNBindBufferMemory2Khr = FNBindBufferMemory2;
///See [`bind_image_memory2`]
#[doc(alias = "vkBindImageMemory2KHR")]
pub type FNBindImageMemory2Khr = FNBindImageMemory2;
