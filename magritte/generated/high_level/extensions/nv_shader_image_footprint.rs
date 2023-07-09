pub use crate::common::extensions::nv_shader_image_footprint::{
    NV_SHADER_IMAGE_FOOTPRINT_EXTENSION_NAME, NV_SHADER_IMAGE_FOOTPRINT_SPEC_VERSION,
};
use crate::{context::Context, vulkan1_0::StructureType};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkPhysicalDeviceShaderImageFootprintFeaturesNV")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceShaderImageFootprintFeaturesNV {
    #[doc(alias = "imageFootprint")]
    pub image_footprint: bool,
}
impl PhysicalDeviceShaderImageFootprintFeaturesNV {
    ///Get a reference to the `image_footprint` field.
    pub fn image_footprint(&self) -> &bool {
        &self.image_footprint
    }
    ///Get a mutable reference to the `image_footprint` field.
    pub fn image_footprint_mut(&mut self) -> &mut bool {
        &mut self.image_footprint
    }
    ///Sets the `image_footprint` field.
    pub fn set_image_footprint(&mut self, image_footprint: bool) -> &mut Self {
        self.image_footprint = image_footprint;
        self
    }
    ///Sets the `image_footprint` field in a builder way.
    pub fn with_image_footprint(mut self, image_footprint: bool) -> Self {
        self.image_footprint = image_footprint;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceShaderImageFootprintFeaturesNV {
    type LowLevel = crate::native::extensions::nv_shader_image_footprint::PhysicalDeviceShaderImageFootprintFeaturesNV;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::nv_shader_image_footprint::PhysicalDeviceShaderImageFootprintFeaturesNV {
            s_type: StructureType::PhysicalDeviceShaderImageFootprintFeaturesNv,
            p_next: std::ptr::null_mut(),
            image_footprint: self.image_footprint.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceShaderImageFootprintFeaturesNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            image_footprint: crate::conv::FromLowLevel::from_low_level(context, value.image_footprint),
        }
    }
}
