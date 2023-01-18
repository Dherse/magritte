use crate::{
    cstr,
    vulkan1_1::{
        BufferMemoryRequirementsInfo2, FNGetBufferMemoryRequirements2, FNGetImageMemoryRequirements2,
        FNGetImageSparseMemoryRequirements2, ImageMemoryRequirementsInfo2, ImageSparseMemoryRequirementsInfo2,
        SparseImageMemoryRequirements2,
    },
};
use std::ffi::CStr;
///See [`BufferMemoryRequirementsInfo2`]
#[doc(alias = "VkBufferMemoryRequirementsInfo2KHR")]
pub type BufferMemoryRequirementsInfo2KHR = BufferMemoryRequirementsInfo2;
///See [`ImageMemoryRequirementsInfo2`]
#[doc(alias = "VkImageMemoryRequirementsInfo2KHR")]
pub type ImageMemoryRequirementsInfo2KHR = ImageMemoryRequirementsInfo2;
///See [`ImageSparseMemoryRequirementsInfo2`]
#[doc(alias = "VkImageSparseMemoryRequirementsInfo2KHR")]
pub type ImageSparseMemoryRequirementsInfo2KHR = ImageSparseMemoryRequirementsInfo2;
///See [`SparseImageMemoryRequirements2`]
#[doc(alias = "VkSparseImageMemoryRequirements2KHR")]
pub type SparseImageMemoryRequirements2KHR = SparseImageMemoryRequirements2;
#[doc(alias = "VK_KHR_GET_MEMORY_REQUIREMENTS_2_SPEC_VERSION")]
pub const KHR_GET_MEMORY_REQUIREMENTS_2_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_KHR_GET_MEMORY_REQUIREMENTS_2_EXTENSION_NAME")]
pub const KHR_GET_MEMORY_REQUIREMENTS_2_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_get_memory_requirements2");
///See [`get_buffer_memory_requirements2`]
#[doc(alias = "vkGetBufferMemoryRequirements2KHR")]
pub type FNGetBufferMemoryRequirements2Khr = FNGetBufferMemoryRequirements2;
///See [`get_image_memory_requirements2`]
#[doc(alias = "vkGetImageMemoryRequirements2KHR")]
pub type FNGetImageMemoryRequirements2Khr = FNGetImageMemoryRequirements2;
///See [`get_image_sparse_memory_requirements2`]
#[doc(alias = "vkGetImageSparseMemoryRequirements2KHR")]
pub type FNGetImageSparseMemoryRequirements2Khr = FNGetImageSparseMemoryRequirements2;
