pub use crate::common::extensions::khr_present_wait::{KHR_PRESENT_WAIT_EXTENSION_NAME, KHR_PRESENT_WAIT_SPEC_VERSION};
use crate::{context::Context, vulkan1_0::StructureType};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkPhysicalDevicePresentWaitFeaturesKHR")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDevicePresentWaitFeaturesKHR {
    #[doc(alias = "presentWait")]
    pub present_wait: bool,
}
impl PhysicalDevicePresentWaitFeaturesKHR {
    ///Get a reference to the `present_wait` field.
    pub fn present_wait(&self) -> &bool {
        &self.present_wait
    }
    ///Get a mutable reference to the `present_wait` field.
    pub fn present_wait_mut(&mut self) -> &mut bool {
        &mut self.present_wait
    }
    ///Sets the `present_wait` field.
    pub fn set_present_wait(&mut self, present_wait: bool) -> &mut Self {
        self.present_wait = present_wait;
        self
    }
    ///Sets the `present_wait` field in a builder way.
    pub fn with_present_wait(mut self, present_wait: bool) -> Self {
        self.present_wait = present_wait;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDevicePresentWaitFeaturesKHR {
    type LowLevel = crate::native::extensions::khr_present_wait::PhysicalDevicePresentWaitFeaturesKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::khr_present_wait::PhysicalDevicePresentWaitFeaturesKHR {
            s_type: StructureType::PhysicalDevicePresentWaitFeaturesKhr,
            p_next: std::ptr::null_mut(),
            present_wait: self.present_wait.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDevicePresentWaitFeaturesKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            present_wait: crate::conv::FromLowLevel::from_low_level(context, value.present_wait),
        }
    }
}
