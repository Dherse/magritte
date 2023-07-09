pub use crate::common::extensions::khr_synchronization2::{
    Flags64, KHR_SYNCHRONIZATION_2_EXTENSION_NAME, KHR_SYNCHRONIZATION_2_SPEC_VERSION,
};
use crate::{
    context::Context,
    vulkan1_0::StructureType,
    vulkan1_3::{
        AccessFlagBits2, AccessFlags2, BufferMemoryBarrier2, CommandBufferSubmitInfo, DependencyInfo,
        ImageMemoryBarrier2, MemoryBarrier2, PhysicalDeviceSynchronization2Features, PipelineStageFlagBits2,
        PipelineStageFlags2, SemaphoreSubmitInfo, SubmitFlagBits, SubmitFlags, SubmitInfo2,
    },
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkAccessFlags2KHR")]
pub type AccessFlags2KHR = AccessFlags2;
#[doc(alias = "VkPipelineStageFlags2KHR")]
pub type PipelineStageFlags2KHR = PipelineStageFlags2;
#[doc(alias = "VkSubmitFlagsKHR")]
pub type SubmitFlagsKHR = SubmitFlags;
#[doc(alias = "VkAccessFlagBits2KHR")]
pub type AccessFlagBits2KHR = AccessFlagBits2;
#[doc(alias = "VkPipelineStageFlagBits2KHR")]
pub type PipelineStageFlagBits2KHR = PipelineStageFlagBits2;
#[doc(alias = "VkSubmitFlagBitsKHR")]
pub type SubmitFlagBitsKHR = SubmitFlagBits;
#[doc(alias = "VkMemoryBarrier2KHR")]
pub type MemoryBarrier2KHR = MemoryBarrier2;
#[doc(alias = "VkImageMemoryBarrier2KHR")]
pub type ImageMemoryBarrier2KHR = ImageMemoryBarrier2;
#[doc(alias = "VkBufferMemoryBarrier2KHR")]
pub type BufferMemoryBarrier2KHR = BufferMemoryBarrier2;
#[doc(alias = "VkDependencyInfoKHR")]
pub type DependencyInfoKHR = DependencyInfo;
#[doc(alias = "VkSemaphoreSubmitInfoKHR")]
pub type SemaphoreSubmitInfoKHR = SemaphoreSubmitInfo;
#[doc(alias = "VkCommandBufferSubmitInfoKHR")]
pub type CommandBufferSubmitInfoKHR = CommandBufferSubmitInfo;
#[doc(alias = "VkSubmitInfo2KHR")]
pub type SubmitInfo2KHR = SubmitInfo2;
#[doc(alias = "VkPhysicalDeviceSynchronization2FeaturesKHR")]
pub type PhysicalDeviceSynchronization2FeaturesKHR = PhysicalDeviceSynchronization2Features;
#[doc(alias = "VkQueueFamilyCheckpointProperties2NV")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueueFamilyCheckpointProperties2NV {
    #[doc(alias = "checkpointExecutionStageMask")]
    pub checkpoint_execution_stage_mask: PipelineStageFlags2,
}
impl QueueFamilyCheckpointProperties2NV {
    ///Get a reference to the `checkpoint_execution_stage_mask` field.
    pub fn checkpoint_execution_stage_mask(&self) -> PipelineStageFlags2 {
        self.checkpoint_execution_stage_mask
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for QueueFamilyCheckpointProperties2NV {
    type LowLevel = crate::native::extensions::khr_synchronization2::QueueFamilyCheckpointProperties2NV;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::khr_synchronization2::QueueFamilyCheckpointProperties2NV {
            s_type: StructureType::QueueFamilyCheckpointProperties2Nv,
            p_next: std::ptr::null_mut(),
            checkpoint_execution_stage_mask: self.checkpoint_execution_stage_mask.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for QueueFamilyCheckpointProperties2NV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            checkpoint_execution_stage_mask: crate::conv::FromLowLevel::from_low_level(
                context,
                value.checkpoint_execution_stage_mask,
            ),
        }
    }
}
