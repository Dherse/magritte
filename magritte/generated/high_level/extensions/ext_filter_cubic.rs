pub use crate::common::extensions::ext_filter_cubic::{EXT_FILTER_CUBIC_EXTENSION_NAME, EXT_FILTER_CUBIC_SPEC_VERSION};
use crate::{
    context::Context,
    vulkan1_0::{ImageViewType, StructureType},
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkPhysicalDeviceImageViewImageFormatInfoEXT")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceImageViewImageFormatInfoEXT {
    #[doc(alias = "imageViewType")]
    pub image_view_type: ImageViewType,
}
impl PhysicalDeviceImageViewImageFormatInfoEXT {
    ///Get a reference to the `image_view_type` field.
    pub fn image_view_type(&self) -> ImageViewType {
        self.image_view_type
    }
    ///Get a mutable reference to the `image_view_type` field.
    pub fn image_view_type_mut(&mut self) -> &mut ImageViewType {
        &mut self.image_view_type
    }
    ///Sets the `image_view_type` field.
    pub fn set_image_view_type(&mut self, image_view_type: ImageViewType) -> &mut Self {
        self.image_view_type = image_view_type;
        self
    }
    ///Sets the `image_view_type` field in a builder way.
    pub fn with_image_view_type(mut self, image_view_type: ImageViewType) -> Self {
        self.image_view_type = image_view_type;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceImageViewImageFormatInfoEXT {
    type LowLevel = crate::native::extensions::ext_filter_cubic::PhysicalDeviceImageViewImageFormatInfoEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_filter_cubic::PhysicalDeviceImageViewImageFormatInfoEXT {
            s_type: StructureType::PhysicalDeviceImageViewImageFormatInfoExt,
            p_next: std::ptr::null_mut(),
            image_view_type: self.image_view_type.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceImageViewImageFormatInfoEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            image_view_type: crate::conv::FromLowLevel::from_low_level(context, value.image_view_type),
        }
    }
}
#[doc(alias = "VkFilterCubicImageViewImageFormatPropertiesEXT")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FilterCubicImageViewImageFormatPropertiesEXT {
    #[doc(alias = "filterCubic")]
    pub filter_cubic: bool,
    #[doc(alias = "filterCubicMinmax")]
    pub filter_cubic_minmax: bool,
}
impl FilterCubicImageViewImageFormatPropertiesEXT {
    ///Get a reference to the `filter_cubic` field.
    pub fn filter_cubic(&self) -> &bool {
        &self.filter_cubic
    }
    ///Get a reference to the `filter_cubic_minmax` field.
    pub fn filter_cubic_minmax(&self) -> &bool {
        &self.filter_cubic_minmax
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for FilterCubicImageViewImageFormatPropertiesEXT {
    type LowLevel = crate::native::extensions::ext_filter_cubic::FilterCubicImageViewImageFormatPropertiesEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_filter_cubic::FilterCubicImageViewImageFormatPropertiesEXT {
            s_type: StructureType::FilterCubicImageViewImageFormatPropertiesExt,
            p_next: std::ptr::null_mut(),
            filter_cubic: self.filter_cubic.into_low_level(context, bump),
            filter_cubic_minmax: self.filter_cubic_minmax.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for FilterCubicImageViewImageFormatPropertiesEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            filter_cubic: crate::conv::FromLowLevel::from_low_level(context, value.filter_cubic),
            filter_cubic_minmax: crate::conv::FromLowLevel::from_low_level(context, value.filter_cubic_minmax),
        }
    }
}
