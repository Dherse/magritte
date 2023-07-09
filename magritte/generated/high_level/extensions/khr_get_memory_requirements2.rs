pub use crate::common::extensions::khr_get_memory_requirements2::{
    KHR_GET_MEMORY_REQUIREMENTS_2_EXTENSION_NAME, KHR_GET_MEMORY_REQUIREMENTS_2_SPEC_VERSION,
};
use crate::vulkan1_1::{
    BufferMemoryRequirementsInfo2, ImageMemoryRequirementsInfo2, ImageSparseMemoryRequirementsInfo2,
    SparseImageMemoryRequirements2,
};
#[doc(alias = "VkBufferMemoryRequirementsInfo2KHR")]
pub type BufferMemoryRequirementsInfo2KHR = BufferMemoryRequirementsInfo2;
#[doc(alias = "VkImageMemoryRequirementsInfo2KHR")]
pub type ImageMemoryRequirementsInfo2KHR = ImageMemoryRequirementsInfo2;
#[doc(alias = "VkImageSparseMemoryRequirementsInfo2KHR")]
pub type ImageSparseMemoryRequirementsInfo2KHR = ImageSparseMemoryRequirementsInfo2;
#[doc(alias = "VkSparseImageMemoryRequirements2KHR")]
pub type SparseImageMemoryRequirements2KHR = SparseImageMemoryRequirements2;
