pub use crate::common::extensions::amd_memory_overallocation_behavior::{
    MemoryOverallocationBehaviorAMD, AMD_MEMORY_OVERALLOCATION_BEHAVIOR_EXTENSION_NAME,
    AMD_MEMORY_OVERALLOCATION_BEHAVIOR_SPEC_VERSION,
};
use crate::{context::Context, vulkan1_0::StructureType};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkDeviceMemoryOverallocationCreateInfoAMD")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DeviceMemoryOverallocationCreateInfoAMD {
    #[doc(alias = "overallocationBehavior")]
    pub overallocation_behavior: MemoryOverallocationBehaviorAMD,
}
impl DeviceMemoryOverallocationCreateInfoAMD {
    ///Get a reference to the `overallocation_behavior` field.
    pub fn overallocation_behavior(&self) -> MemoryOverallocationBehaviorAMD {
        self.overallocation_behavior
    }
    ///Get a mutable reference to the `overallocation_behavior` field.
    pub fn overallocation_behavior_mut(&mut self) -> &mut MemoryOverallocationBehaviorAMD {
        &mut self.overallocation_behavior
    }
    ///Sets the `overallocation_behavior` field.
    pub fn set_overallocation_behavior(
        &mut self,
        overallocation_behavior: MemoryOverallocationBehaviorAMD,
    ) -> &mut Self {
        self.overallocation_behavior = overallocation_behavior;
        self
    }
    ///Sets the `overallocation_behavior` field in a builder way.
    pub fn with_overallocation_behavior(mut self, overallocation_behavior: MemoryOverallocationBehaviorAMD) -> Self {
        self.overallocation_behavior = overallocation_behavior;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for DeviceMemoryOverallocationCreateInfoAMD {
    type LowLevel =
        crate::native::extensions::amd_memory_overallocation_behavior::DeviceMemoryOverallocationCreateInfoAMD;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::amd_memory_overallocation_behavior::DeviceMemoryOverallocationCreateInfoAMD {
            s_type: StructureType::DeviceMemoryOverallocationCreateInfoAmd,
            p_next: std::ptr::null(),
            overallocation_behavior: self.overallocation_behavior.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for DeviceMemoryOverallocationCreateInfoAMD {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            overallocation_behavior: crate::conv::FromLowLevel::from_low_level(context, value.overallocation_behavior),
        }
    }
}
