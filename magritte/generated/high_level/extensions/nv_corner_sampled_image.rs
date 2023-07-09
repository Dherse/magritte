pub use crate::common::extensions::nv_corner_sampled_image::{
    NV_CORNER_SAMPLED_IMAGE_EXTENSION_NAME, NV_CORNER_SAMPLED_IMAGE_SPEC_VERSION,
};
use crate::{context::Context, vulkan1_0::StructureType};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkPhysicalDeviceCornerSampledImageFeaturesNV")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceCornerSampledImageFeaturesNV {
    #[doc(alias = "cornerSampledImage")]
    pub corner_sampled_image: bool,
}
impl PhysicalDeviceCornerSampledImageFeaturesNV {
    ///Get a reference to the `corner_sampled_image` field.
    pub fn corner_sampled_image(&self) -> &bool {
        &self.corner_sampled_image
    }
    ///Get a mutable reference to the `corner_sampled_image` field.
    pub fn corner_sampled_image_mut(&mut self) -> &mut bool {
        &mut self.corner_sampled_image
    }
    ///Sets the `corner_sampled_image` field.
    pub fn set_corner_sampled_image(&mut self, corner_sampled_image: bool) -> &mut Self {
        self.corner_sampled_image = corner_sampled_image;
        self
    }
    ///Sets the `corner_sampled_image` field in a builder way.
    pub fn with_corner_sampled_image(mut self, corner_sampled_image: bool) -> Self {
        self.corner_sampled_image = corner_sampled_image;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceCornerSampledImageFeaturesNV {
    type LowLevel = crate::native::extensions::nv_corner_sampled_image::PhysicalDeviceCornerSampledImageFeaturesNV;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::nv_corner_sampled_image::PhysicalDeviceCornerSampledImageFeaturesNV {
            s_type: StructureType::PhysicalDeviceCornerSampledImageFeaturesNv,
            p_next: std::ptr::null_mut(),
            corner_sampled_image: self.corner_sampled_image.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceCornerSampledImageFeaturesNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            corner_sampled_image: crate::conv::FromLowLevel::from_low_level(context, value.corner_sampled_image),
        }
    }
}
