pub use crate::common::extensions::khr_push_descriptor::{
    KHR_PUSH_DESCRIPTOR_EXTENSION_NAME, KHR_PUSH_DESCRIPTOR_SPEC_VERSION,
};
use crate::{context::Context, vulkan1_0::StructureType};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkPhysicalDevicePushDescriptorPropertiesKHR")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDevicePushDescriptorPropertiesKHR {
    #[doc(alias = "maxPushDescriptors")]
    pub max_push_descriptors: u32,
}
impl PhysicalDevicePushDescriptorPropertiesKHR {
    ///Get a reference to the `max_push_descriptors` field.
    pub fn max_push_descriptors(&self) -> u32 {
        self.max_push_descriptors
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDevicePushDescriptorPropertiesKHR {
    type LowLevel = crate::native::extensions::khr_push_descriptor::PhysicalDevicePushDescriptorPropertiesKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::khr_push_descriptor::PhysicalDevicePushDescriptorPropertiesKHR {
            s_type: StructureType::PhysicalDevicePushDescriptorPropertiesKhr,
            p_next: std::ptr::null_mut(),
            max_push_descriptors: self.max_push_descriptors.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDevicePushDescriptorPropertiesKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            max_push_descriptors: crate::conv::FromLowLevel::from_low_level(context, value.max_push_descriptors),
        }
    }
}
