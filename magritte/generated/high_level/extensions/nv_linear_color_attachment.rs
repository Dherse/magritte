pub use crate::common::extensions::nv_linear_color_attachment::{
    NV_LINEAR_COLOR_ATTACHMENT_EXTENSION_NAME, NV_LINEAR_COLOR_ATTACHMENT_SPEC_VERSION,
};
use crate::{context::Context, vulkan1_0::StructureType};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkPhysicalDeviceLinearColorAttachmentFeaturesNV")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceLinearColorAttachmentFeaturesNV {
    #[doc(alias = "linearColorAttachment")]
    pub linear_color_attachment: bool,
}
impl PhysicalDeviceLinearColorAttachmentFeaturesNV {
    ///Get a reference to the `linear_color_attachment` field.
    pub fn linear_color_attachment(&self) -> &bool {
        &self.linear_color_attachment
    }
    ///Get a mutable reference to the `linear_color_attachment` field.
    pub fn linear_color_attachment_mut(&mut self) -> &mut bool {
        &mut self.linear_color_attachment
    }
    ///Sets the `linear_color_attachment` field.
    pub fn set_linear_color_attachment(&mut self, linear_color_attachment: bool) -> &mut Self {
        self.linear_color_attachment = linear_color_attachment;
        self
    }
    ///Sets the `linear_color_attachment` field in a builder way.
    pub fn with_linear_color_attachment(mut self, linear_color_attachment: bool) -> Self {
        self.linear_color_attachment = linear_color_attachment;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceLinearColorAttachmentFeaturesNV {
    type LowLevel =
        crate::native::extensions::nv_linear_color_attachment::PhysicalDeviceLinearColorAttachmentFeaturesNV;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::nv_linear_color_attachment::PhysicalDeviceLinearColorAttachmentFeaturesNV {
            s_type: StructureType::PhysicalDeviceLinearColorAttachmentFeaturesNv,
            p_next: std::ptr::null_mut(),
            linear_color_attachment: self.linear_color_attachment.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceLinearColorAttachmentFeaturesNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            linear_color_attachment: crate::conv::FromLowLevel::from_low_level(context, value.linear_color_attachment),
        }
    }
}
