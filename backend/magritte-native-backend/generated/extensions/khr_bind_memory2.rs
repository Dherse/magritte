use crate::{
    cstr,
    vulkan1_1::{BindBufferMemoryInfo, BindImageMemoryInfo, FNBindBufferMemory2, FNBindImageMemory2},
};
use std::ffi::CStr;
///See [`BindBufferMemoryInfo`]
#[doc(alias = "VkBindBufferMemoryInfoKHR")]
pub type BindBufferMemoryInfoKHR = BindBufferMemoryInfo;
///See [`BindImageMemoryInfo`]
#[doc(alias = "VkBindImageMemoryInfoKHR")]
pub type BindImageMemoryInfoKHR = BindImageMemoryInfo;
#[doc(alias = "VK_KHR_BIND_MEMORY_2_SPEC_VERSION")]
pub const KHR_BIND_MEMORY_2_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_KHR_BIND_MEMORY_2_EXTENSION_NAME")]
pub const KHR_BIND_MEMORY_2_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_bind_memory2");
///See [`bind_buffer_memory2`]
#[doc(alias = "vkBindBufferMemory2KHR")]
pub type FNBindBufferMemory2Khr = FNBindBufferMemory2;
///See [`bind_image_memory2`]
#[doc(alias = "vkBindImageMemory2KHR")]
pub type FNBindImageMemory2Khr = FNBindImageMemory2;
