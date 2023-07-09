pub use crate::common::extensions::ext_hdr_metadata::{
    XyColorEXT, EXT_HDR_METADATA_EXTENSION_NAME, EXT_HDR_METADATA_SPEC_VERSION,
};
use crate::{context::Context, vulkan1_0::StructureType};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
impl XyColorEXT {
    ///Get a reference to the `x` field.
    pub fn x(&self) -> f32 {
        self.x
    }
    ///Get a reference to the `y` field.
    pub fn y(&self) -> f32 {
        self.y
    }
    ///Get a mutable reference to the `x` field.
    pub fn x_mut(&mut self) -> &mut f32 {
        &mut self.x
    }
    ///Get a mutable reference to the `y` field.
    pub fn y_mut(&mut self) -> &mut f32 {
        &mut self.y
    }
    ///Sets the `x` field.
    pub fn set_x(&mut self, x: f32) -> &mut Self {
        self.x = x;
        self
    }
    ///Sets the `y` field.
    pub fn set_y(&mut self, y: f32) -> &mut Self {
        self.y = y;
        self
    }
    ///Sets the `x` field in a builder way.
    pub fn with_x(mut self, x: f32) -> Self {
        self.x = x;
        self
    }
    ///Sets the `y` field in a builder way.
    pub fn with_y(mut self, y: f32) -> Self {
        self.y = y;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for XyColorEXT {
    type LowLevel = crate::native::extensions::ext_hdr_metadata::XyColorEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_hdr_metadata::XyColorEXT {
            x: self.x.into_low_level(context, bump),
            y: self.y.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for XyColorEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            x: crate::conv::FromLowLevel::from_low_level(context, value.x),
            y: crate::conv::FromLowLevel::from_low_level(context, value.y),
        }
    }
}
#[doc(alias = "VkHdrMetadataEXT")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct HdrMetadataEXT {
    #[doc(alias = "displayPrimaryRed")]
    pub display_primary_red: XyColorEXT,
    #[doc(alias = "displayPrimaryGreen")]
    pub display_primary_green: XyColorEXT,
    #[doc(alias = "displayPrimaryBlue")]
    pub display_primary_blue: XyColorEXT,
    #[doc(alias = "whitePoint")]
    pub white_point: XyColorEXT,
    #[doc(alias = "maxLuminance")]
    pub max_luminance: f32,
    #[doc(alias = "minLuminance")]
    pub min_luminance: f32,
    #[doc(alias = "maxContentLightLevel")]
    pub max_content_light_level: f32,
    #[doc(alias = "maxFrameAverageLightLevel")]
    pub max_frame_average_light_level: f32,
}
impl HdrMetadataEXT {
    ///Get a reference to the `display_primary_red` field.
    pub fn display_primary_red(&self) -> XyColorEXT {
        self.display_primary_red
    }
    ///Get a reference to the `display_primary_green` field.
    pub fn display_primary_green(&self) -> XyColorEXT {
        self.display_primary_green
    }
    ///Get a reference to the `display_primary_blue` field.
    pub fn display_primary_blue(&self) -> XyColorEXT {
        self.display_primary_blue
    }
    ///Get a reference to the `white_point` field.
    pub fn white_point(&self) -> XyColorEXT {
        self.white_point
    }
    ///Get a reference to the `max_luminance` field.
    pub fn max_luminance(&self) -> f32 {
        self.max_luminance
    }
    ///Get a reference to the `min_luminance` field.
    pub fn min_luminance(&self) -> f32 {
        self.min_luminance
    }
    ///Get a reference to the `max_content_light_level` field.
    pub fn max_content_light_level(&self) -> f32 {
        self.max_content_light_level
    }
    ///Get a reference to the `max_frame_average_light_level` field.
    pub fn max_frame_average_light_level(&self) -> f32 {
        self.max_frame_average_light_level
    }
    ///Get a mutable reference to the `display_primary_red` field.
    pub fn display_primary_red_mut(&mut self) -> &mut XyColorEXT {
        &mut self.display_primary_red
    }
    ///Get a mutable reference to the `display_primary_green` field.
    pub fn display_primary_green_mut(&mut self) -> &mut XyColorEXT {
        &mut self.display_primary_green
    }
    ///Get a mutable reference to the `display_primary_blue` field.
    pub fn display_primary_blue_mut(&mut self) -> &mut XyColorEXT {
        &mut self.display_primary_blue
    }
    ///Get a mutable reference to the `white_point` field.
    pub fn white_point_mut(&mut self) -> &mut XyColorEXT {
        &mut self.white_point
    }
    ///Get a mutable reference to the `max_luminance` field.
    pub fn max_luminance_mut(&mut self) -> &mut f32 {
        &mut self.max_luminance
    }
    ///Get a mutable reference to the `min_luminance` field.
    pub fn min_luminance_mut(&mut self) -> &mut f32 {
        &mut self.min_luminance
    }
    ///Get a mutable reference to the `max_content_light_level` field.
    pub fn max_content_light_level_mut(&mut self) -> &mut f32 {
        &mut self.max_content_light_level
    }
    ///Get a mutable reference to the `max_frame_average_light_level` field.
    pub fn max_frame_average_light_level_mut(&mut self) -> &mut f32 {
        &mut self.max_frame_average_light_level
    }
    ///Sets the `display_primary_red` field.
    pub fn set_display_primary_red(&mut self, display_primary_red: XyColorEXT) -> &mut Self {
        self.display_primary_red = display_primary_red;
        self
    }
    ///Sets the `display_primary_green` field.
    pub fn set_display_primary_green(&mut self, display_primary_green: XyColorEXT) -> &mut Self {
        self.display_primary_green = display_primary_green;
        self
    }
    ///Sets the `display_primary_blue` field.
    pub fn set_display_primary_blue(&mut self, display_primary_blue: XyColorEXT) -> &mut Self {
        self.display_primary_blue = display_primary_blue;
        self
    }
    ///Sets the `white_point` field.
    pub fn set_white_point(&mut self, white_point: XyColorEXT) -> &mut Self {
        self.white_point = white_point;
        self
    }
    ///Sets the `max_luminance` field.
    pub fn set_max_luminance(&mut self, max_luminance: f32) -> &mut Self {
        self.max_luminance = max_luminance;
        self
    }
    ///Sets the `min_luminance` field.
    pub fn set_min_luminance(&mut self, min_luminance: f32) -> &mut Self {
        self.min_luminance = min_luminance;
        self
    }
    ///Sets the `max_content_light_level` field.
    pub fn set_max_content_light_level(&mut self, max_content_light_level: f32) -> &mut Self {
        self.max_content_light_level = max_content_light_level;
        self
    }
    ///Sets the `max_frame_average_light_level` field.
    pub fn set_max_frame_average_light_level(&mut self, max_frame_average_light_level: f32) -> &mut Self {
        self.max_frame_average_light_level = max_frame_average_light_level;
        self
    }
    ///Sets the `display_primary_red` field in a builder way.
    pub fn with_display_primary_red(mut self, display_primary_red: XyColorEXT) -> Self {
        self.display_primary_red = display_primary_red;
        self
    }
    ///Sets the `display_primary_green` field in a builder way.
    pub fn with_display_primary_green(mut self, display_primary_green: XyColorEXT) -> Self {
        self.display_primary_green = display_primary_green;
        self
    }
    ///Sets the `display_primary_blue` field in a builder way.
    pub fn with_display_primary_blue(mut self, display_primary_blue: XyColorEXT) -> Self {
        self.display_primary_blue = display_primary_blue;
        self
    }
    ///Sets the `white_point` field in a builder way.
    pub fn with_white_point(mut self, white_point: XyColorEXT) -> Self {
        self.white_point = white_point;
        self
    }
    ///Sets the `max_luminance` field in a builder way.
    pub fn with_max_luminance(mut self, max_luminance: f32) -> Self {
        self.max_luminance = max_luminance;
        self
    }
    ///Sets the `min_luminance` field in a builder way.
    pub fn with_min_luminance(mut self, min_luminance: f32) -> Self {
        self.min_luminance = min_luminance;
        self
    }
    ///Sets the `max_content_light_level` field in a builder way.
    pub fn with_max_content_light_level(mut self, max_content_light_level: f32) -> Self {
        self.max_content_light_level = max_content_light_level;
        self
    }
    ///Sets the `max_frame_average_light_level` field in a builder way.
    pub fn with_max_frame_average_light_level(mut self, max_frame_average_light_level: f32) -> Self {
        self.max_frame_average_light_level = max_frame_average_light_level;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for HdrMetadataEXT {
    type LowLevel = crate::native::extensions::ext_hdr_metadata::HdrMetadataEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_hdr_metadata::HdrMetadataEXT {
            s_type: StructureType::HdrMetadataExt,
            p_next: std::ptr::null(),
            display_primary_red: self.display_primary_red.into_low_level(context, bump),
            display_primary_green: self.display_primary_green.into_low_level(context, bump),
            display_primary_blue: self.display_primary_blue.into_low_level(context, bump),
            white_point: self.white_point.into_low_level(context, bump),
            max_luminance: self.max_luminance.into_low_level(context, bump),
            min_luminance: self.min_luminance.into_low_level(context, bump),
            max_content_light_level: self.max_content_light_level.into_low_level(context, bump),
            max_frame_average_light_level: self.max_frame_average_light_level.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for HdrMetadataEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            display_primary_red: crate::conv::FromLowLevel::from_low_level(context, value.display_primary_red),
            display_primary_green: crate::conv::FromLowLevel::from_low_level(context, value.display_primary_green),
            display_primary_blue: crate::conv::FromLowLevel::from_low_level(context, value.display_primary_blue),
            white_point: crate::conv::FromLowLevel::from_low_level(context, value.white_point),
            max_luminance: crate::conv::FromLowLevel::from_low_level(context, value.max_luminance),
            min_luminance: crate::conv::FromLowLevel::from_low_level(context, value.min_luminance),
            max_content_light_level: crate::conv::FromLowLevel::from_low_level(context, value.max_content_light_level),
            max_frame_average_light_level: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_frame_average_light_level,
            ),
        }
    }
}
