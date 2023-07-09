pub use crate::common::extensions::nv_device_diagnostic_checkpoints::{
    NV_DEVICE_DIAGNOSTIC_CHECKPOINTS_EXTENSION_NAME, NV_DEVICE_DIAGNOSTIC_CHECKPOINTS_SPEC_VERSION,
};
use crate::{
    context::Context,
    vulkan1_0::{PipelineStageFlags, StructureType},
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkQueueFamilyCheckpointPropertiesNV")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueueFamilyCheckpointPropertiesNV {
    #[doc(alias = "checkpointExecutionStageMask")]
    pub checkpoint_execution_stage_mask: PipelineStageFlags,
}
impl QueueFamilyCheckpointPropertiesNV {
    ///Get a reference to the `checkpoint_execution_stage_mask` field.
    pub fn checkpoint_execution_stage_mask(&self) -> PipelineStageFlags {
        self.checkpoint_execution_stage_mask
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for QueueFamilyCheckpointPropertiesNV {
    type LowLevel = crate::native::extensions::nv_device_diagnostic_checkpoints::QueueFamilyCheckpointPropertiesNV;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::nv_device_diagnostic_checkpoints::QueueFamilyCheckpointPropertiesNV {
            s_type: StructureType::QueueFamilyCheckpointPropertiesNv,
            p_next: std::ptr::null_mut(),
            checkpoint_execution_stage_mask: self.checkpoint_execution_stage_mask.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for QueueFamilyCheckpointPropertiesNV {
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
