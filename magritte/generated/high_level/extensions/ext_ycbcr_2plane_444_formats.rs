pub use crate::common::extensions::ext_ycbcr_2plane_444_formats::{
    EXT_YCBCR_2PLANE_444_FORMATS_EXTENSION_NAME, EXT_YCBCR_2PLANE_444_FORMATS_SPEC_VERSION,
};
use crate::{context::Context, vulkan1_0::StructureType};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkPhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT {
    #[doc(alias = "ycbcr2plane444Formats")]
    pub ycbcr2plane444_formats: bool,
}
impl PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT {
    ///Get a reference to the `ycbcr2plane444_formats` field.
    pub fn ycbcr2plane444_formats(&self) -> &bool {
        &self.ycbcr2plane444_formats
    }
    ///Get a mutable reference to the `ycbcr2plane444_formats` field.
    pub fn ycbcr2plane444_formats_mut(&mut self) -> &mut bool {
        &mut self.ycbcr2plane444_formats
    }
    ///Sets the `ycbcr2plane444_formats` field.
    pub fn set_ycbcr2plane444_formats(&mut self, ycbcr2plane444_formats: bool) -> &mut Self {
        self.ycbcr2plane444_formats = ycbcr2plane444_formats;
        self
    }
    ///Sets the `ycbcr2plane444_formats` field in a builder way.
    pub fn with_ycbcr2plane444_formats(mut self, ycbcr2plane444_formats: bool) -> Self {
        self.ycbcr2plane444_formats = ycbcr2plane444_formats;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT {
    type LowLevel =
        crate::native::extensions::ext_ycbcr_2plane_444_formats::PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_ycbcr_2plane_444_formats::PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT {
            s_type: StructureType::PhysicalDeviceYcbcr2Plane444FormatsFeaturesExt,
            p_next: std::ptr::null_mut(),
            ycbcr2plane444_formats: self.ycbcr2plane444_formats.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            ycbcr2plane444_formats: crate::conv::FromLowLevel::from_low_level(context, value.ycbcr2plane444_formats),
        }
    }
}
