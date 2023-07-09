pub use crate::common::extensions::khr_shared_presentable_image::{
    KHR_SHARED_PRESENTABLE_IMAGE_EXTENSION_NAME, KHR_SHARED_PRESENTABLE_IMAGE_SPEC_VERSION,
};
use crate::{
    context::Context,
    vulkan1_0::{ImageUsageFlags, StructureType},
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkSharedPresentSurfaceCapabilitiesKHR")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SharedPresentSurfaceCapabilitiesKHR {
    #[doc(alias = "sharedPresentSupportedUsageFlags")]
    pub shared_present_supported_usage_flags: ImageUsageFlags,
}
impl SharedPresentSurfaceCapabilitiesKHR {
    ///Get a reference to the `shared_present_supported_usage_flags` field.
    pub fn shared_present_supported_usage_flags(&self) -> ImageUsageFlags {
        self.shared_present_supported_usage_flags
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for SharedPresentSurfaceCapabilitiesKHR {
    type LowLevel = crate::native::extensions::khr_shared_presentable_image::SharedPresentSurfaceCapabilitiesKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::khr_shared_presentable_image::SharedPresentSurfaceCapabilitiesKHR {
            s_type: StructureType::SharedPresentSurfaceCapabilitiesKhr,
            p_next: std::ptr::null_mut(),
            shared_present_supported_usage_flags: self
                .shared_present_supported_usage_flags
                .into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for SharedPresentSurfaceCapabilitiesKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            shared_present_supported_usage_flags: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shared_present_supported_usage_flags,
            ),
        }
    }
}
