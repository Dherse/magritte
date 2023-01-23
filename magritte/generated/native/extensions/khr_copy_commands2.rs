//!# [VK_KHR_copy_commands2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_copy_commands2.html)
# ! [doc = include_str ! ("../../../../doc/extensions/khr_copy_commands2/VK_KHR_copy_commands2.md")]
use crate::{
    cstr,
    vulkan1_3::{
        BlitImageInfo2, BufferCopy2, BufferImageCopy2, CopyBufferInfo2, CopyBufferToImageInfo2, CopyImageInfo2,
        CopyImageToBufferInfo2, FNCmdBlitImage2, FNCmdCopyBuffer2, FNCmdCopyBufferToImage2, FNCmdCopyImage2,
        FNCmdCopyImageToBuffer2, FNCmdResolveImage2, ImageBlit2, ImageCopy2, ImageResolve2, ResolveImageInfo2,
    },
};
use std::ffi::CStr;
///See [`BufferCopy2`]
#[doc(alias = "VkBufferCopy2KHR")]
pub type BufferCopy2KHR = BufferCopy2;
///See [`ImageCopy2`]
#[doc(alias = "VkImageCopy2KHR")]
pub type ImageCopy2KHR = ImageCopy2;
///See [`ImageBlit2`]
#[doc(alias = "VkImageBlit2KHR")]
pub type ImageBlit2KHR = ImageBlit2;
///See [`BufferImageCopy2`]
#[doc(alias = "VkBufferImageCopy2KHR")]
pub type BufferImageCopy2KHR = BufferImageCopy2;
///See [`ImageResolve2`]
#[doc(alias = "VkImageResolve2KHR")]
pub type ImageResolve2KHR = ImageResolve2;
///See [`CopyBufferInfo2`]
#[doc(alias = "VkCopyBufferInfo2KHR")]
pub type CopyBufferInfo2KHR = CopyBufferInfo2;
///See [`CopyImageInfo2`]
#[doc(alias = "VkCopyImageInfo2KHR")]
pub type CopyImageInfo2KHR = CopyImageInfo2;
///See [`BlitImageInfo2`]
#[doc(alias = "VkBlitImageInfo2KHR")]
pub type BlitImageInfo2KHR = BlitImageInfo2;
///See [`CopyBufferToImageInfo2`]
#[doc(alias = "VkCopyBufferToImageInfo2KHR")]
pub type CopyBufferToImageInfo2KHR = CopyBufferToImageInfo2;
///See [`CopyImageToBufferInfo2`]
#[doc(alias = "VkCopyImageToBufferInfo2KHR")]
pub type CopyImageToBufferInfo2KHR = CopyImageToBufferInfo2;
///See [`ResolveImageInfo2`]
#[doc(alias = "VkResolveImageInfo2KHR")]
pub type ResolveImageInfo2KHR = ResolveImageInfo2;
#[doc(alias = "VK_KHR_COPY_COMMANDS_2_SPEC_VERSION")]
pub const KHR_COPY_COMMANDS_2_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_KHR_COPY_COMMANDS_2_EXTENSION_NAME")]
pub const KHR_COPY_COMMANDS_2_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_copy_commands2");
///See [`cmd_copy_buffer2`]
#[doc(alias = "vkCmdCopyBuffer2KHR")]
pub type FNCmdCopyBuffer2Khr = FNCmdCopyBuffer2;
///See [`cmd_copy_image2`]
#[doc(alias = "vkCmdCopyImage2KHR")]
pub type FNCmdCopyImage2Khr = FNCmdCopyImage2;
///See [`cmd_blit_image2`]
#[doc(alias = "vkCmdBlitImage2KHR")]
pub type FNCmdBlitImage2Khr = FNCmdBlitImage2;
///See [`cmd_copy_buffer_to_image2`]
#[doc(alias = "vkCmdCopyBufferToImage2KHR")]
pub type FNCmdCopyBufferToImage2Khr = FNCmdCopyBufferToImage2;
///See [`cmd_copy_image_to_buffer2`]
#[doc(alias = "vkCmdCopyImageToBuffer2KHR")]
pub type FNCmdCopyImageToBuffer2Khr = FNCmdCopyImageToBuffer2;
///See [`cmd_resolve_image2`]
#[doc(alias = "vkCmdResolveImage2KHR")]
pub type FNCmdResolveImage2Khr = FNCmdResolveImage2;
