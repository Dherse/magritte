pub use crate::common::extensions::khr_copy_commands2::{
    KHR_COPY_COMMANDS_2_EXTENSION_NAME, KHR_COPY_COMMANDS_2_SPEC_VERSION,
};
use crate::vulkan1_3::{
    BlitImageInfo2, BufferCopy2, BufferImageCopy2, CopyBufferInfo2, CopyBufferToImageInfo2, CopyImageInfo2,
    CopyImageToBufferInfo2, ImageBlit2, ImageCopy2, ImageResolve2, ResolveImageInfo2,
};
#[doc(alias = "VkBufferCopy2KHR")]
pub type BufferCopy2KHR = BufferCopy2;
#[doc(alias = "VkImageCopy2KHR")]
pub type ImageCopy2KHR = ImageCopy2;
#[doc(alias = "VkImageBlit2KHR")]
pub type ImageBlit2KHR = ImageBlit2;
#[doc(alias = "VkBufferImageCopy2KHR")]
pub type BufferImageCopy2KHR = BufferImageCopy2;
#[doc(alias = "VkImageResolve2KHR")]
pub type ImageResolve2KHR = ImageResolve2;
#[doc(alias = "VkCopyBufferInfo2KHR")]
pub type CopyBufferInfo2KHR = CopyBufferInfo2;
#[doc(alias = "VkCopyImageInfo2KHR")]
pub type CopyImageInfo2KHR = CopyImageInfo2;
#[doc(alias = "VkBlitImageInfo2KHR")]
pub type BlitImageInfo2KHR = BlitImageInfo2;
#[doc(alias = "VkCopyBufferToImageInfo2KHR")]
pub type CopyBufferToImageInfo2KHR = CopyBufferToImageInfo2;
#[doc(alias = "VkCopyImageToBufferInfo2KHR")]
pub type CopyImageToBufferInfo2KHR = CopyImageToBufferInfo2;
#[doc(alias = "VkResolveImageInfo2KHR")]
pub type ResolveImageInfo2KHR = ResolveImageInfo2;
