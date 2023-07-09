pub use crate::common::extensions::ext_image_view_min_lod::{
    EXT_IMAGE_VIEW_MIN_LOD_EXTENSION_NAME, EXT_IMAGE_VIEW_MIN_LOD_SPEC_VERSION,
};
use crate::{context::Context, vulkan1_0::StructureType};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkPhysicalDeviceImageViewMinLodFeaturesEXT")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceImageViewMinLodFeaturesEXT {
    #[doc(alias = "minLod")]
    pub min_lod: bool,
}
impl PhysicalDeviceImageViewMinLodFeaturesEXT {
    ///Get a reference to the `min_lod` field.
    pub fn min_lod(&self) -> &bool {
        &self.min_lod
    }
    ///Get a mutable reference to the `min_lod` field.
    pub fn min_lod_mut(&mut self) -> &mut bool {
        &mut self.min_lod
    }
    ///Sets the `min_lod` field.
    pub fn set_min_lod(&mut self, min_lod: bool) -> &mut Self {
        self.min_lod = min_lod;
        self
    }
    ///Sets the `min_lod` field in a builder way.
    pub fn with_min_lod(mut self, min_lod: bool) -> Self {
        self.min_lod = min_lod;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceImageViewMinLodFeaturesEXT {
    type LowLevel = crate::native::extensions::ext_image_view_min_lod::PhysicalDeviceImageViewMinLodFeaturesEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_image_view_min_lod::PhysicalDeviceImageViewMinLodFeaturesEXT {
            s_type: StructureType::PhysicalDeviceImageViewMinLodFeaturesExt,
            p_next: std::ptr::null_mut(),
            min_lod: self.min_lod.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceImageViewMinLodFeaturesEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            min_lod: crate::conv::FromLowLevel::from_low_level(context, value.min_lod),
        }
    }
}
#[doc(alias = "VkImageViewMinLodCreateInfoEXT")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ImageViewMinLodCreateInfoEXT {
    #[doc(alias = "minLod")]
    pub min_lod: f32,
}
impl ImageViewMinLodCreateInfoEXT {
    ///Get a reference to the `min_lod` field.
    pub fn min_lod(&self) -> f32 {
        self.min_lod
    }
    ///Get a mutable reference to the `min_lod` field.
    pub fn min_lod_mut(&mut self) -> &mut f32 {
        &mut self.min_lod
    }
    ///Sets the `min_lod` field.
    pub fn set_min_lod(&mut self, min_lod: f32) -> &mut Self {
        self.min_lod = min_lod;
        self
    }
    ///Sets the `min_lod` field in a builder way.
    pub fn with_min_lod(mut self, min_lod: f32) -> Self {
        self.min_lod = min_lod;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for ImageViewMinLodCreateInfoEXT {
    type LowLevel = crate::native::extensions::ext_image_view_min_lod::ImageViewMinLodCreateInfoEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_image_view_min_lod::ImageViewMinLodCreateInfoEXT {
            s_type: StructureType::ImageViewMinLodCreateInfoExt,
            p_next: std::ptr::null(),
            min_lod: self.min_lod.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for ImageViewMinLodCreateInfoEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            min_lod: crate::conv::FromLowLevel::from_low_level(context, value.min_lod),
        }
    }
}
