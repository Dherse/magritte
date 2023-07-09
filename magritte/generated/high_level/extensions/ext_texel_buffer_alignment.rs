pub use crate::common::extensions::ext_texel_buffer_alignment::{
    EXT_TEXEL_BUFFER_ALIGNMENT_EXTENSION_NAME, EXT_TEXEL_BUFFER_ALIGNMENT_SPEC_VERSION,
};
use crate::{context::Context, vulkan1_0::StructureType, vulkan1_3::PhysicalDeviceTexelBufferAlignmentProperties};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkPhysicalDeviceTexelBufferAlignmentPropertiesEXT")]
pub type PhysicalDeviceTexelBufferAlignmentPropertiesEXT = PhysicalDeviceTexelBufferAlignmentProperties;
#[doc(alias = "VkPhysicalDeviceTexelBufferAlignmentFeaturesEXT")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceTexelBufferAlignmentFeaturesEXT {
    #[doc(alias = "texelBufferAlignment")]
    pub texel_buffer_alignment: bool,
}
impl PhysicalDeviceTexelBufferAlignmentFeaturesEXT {
    ///Get a reference to the `texel_buffer_alignment` field.
    pub fn texel_buffer_alignment(&self) -> &bool {
        &self.texel_buffer_alignment
    }
    ///Get a mutable reference to the `texel_buffer_alignment` field.
    pub fn texel_buffer_alignment_mut(&mut self) -> &mut bool {
        &mut self.texel_buffer_alignment
    }
    ///Sets the `texel_buffer_alignment` field.
    pub fn set_texel_buffer_alignment(&mut self, texel_buffer_alignment: bool) -> &mut Self {
        self.texel_buffer_alignment = texel_buffer_alignment;
        self
    }
    ///Sets the `texel_buffer_alignment` field in a builder way.
    pub fn with_texel_buffer_alignment(mut self, texel_buffer_alignment: bool) -> Self {
        self.texel_buffer_alignment = texel_buffer_alignment;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceTexelBufferAlignmentFeaturesEXT {
    type LowLevel =
        crate::native::extensions::ext_texel_buffer_alignment::PhysicalDeviceTexelBufferAlignmentFeaturesEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_texel_buffer_alignment::PhysicalDeviceTexelBufferAlignmentFeaturesEXT {
            s_type: StructureType::PhysicalDeviceTexelBufferAlignmentFeaturesExt,
            p_next: std::ptr::null_mut(),
            texel_buffer_alignment: self.texel_buffer_alignment.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceTexelBufferAlignmentFeaturesEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            texel_buffer_alignment: crate::conv::FromLowLevel::from_low_level(context, value.texel_buffer_alignment),
        }
    }
}
