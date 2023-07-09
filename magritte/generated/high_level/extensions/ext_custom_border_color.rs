pub use crate::common::extensions::ext_custom_border_color::{
    EXT_CUSTOM_BORDER_COLOR_EXTENSION_NAME, EXT_CUSTOM_BORDER_COLOR_SPEC_VERSION,
};
use crate::{
    context::Context,
    vulkan1_0::{ClearColorValue, Format, StructureType},
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkSamplerCustomBorderColorCreateInfoEXT")]
#[derive(Clone, PartialEq, Debug, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SamplerCustomBorderColorCreateInfoEXT {
    #[doc(alias = "customBorderColor")]
    pub custom_border_color: ClearColorValue,
    pub format: Format,
}
impl SamplerCustomBorderColorCreateInfoEXT {
    ///Get a reference to the `custom_border_color` field.
    pub fn custom_border_color(&self) -> ClearColorValue {
        self.custom_border_color
    }
    ///Get a reference to the `format` field.
    pub fn format(&self) -> Format {
        self.format
    }
    ///Get a mutable reference to the `custom_border_color` field.
    pub fn custom_border_color_mut(&mut self) -> &mut ClearColorValue {
        &mut self.custom_border_color
    }
    ///Get a mutable reference to the `format` field.
    pub fn format_mut(&mut self) -> &mut Format {
        &mut self.format
    }
    ///Sets the `custom_border_color` field.
    pub fn set_custom_border_color(&mut self, custom_border_color: ClearColorValue) -> &mut Self {
        self.custom_border_color = custom_border_color;
        self
    }
    ///Sets the `format` field.
    pub fn set_format(&mut self, format: Format) -> &mut Self {
        self.format = format;
        self
    }
    ///Sets the `custom_border_color` field in a builder way.
    pub fn with_custom_border_color(mut self, custom_border_color: ClearColorValue) -> Self {
        self.custom_border_color = custom_border_color;
        self
    }
    ///Sets the `format` field in a builder way.
    pub fn with_format(mut self, format: Format) -> Self {
        self.format = format;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for SamplerCustomBorderColorCreateInfoEXT {
    type LowLevel = crate::native::extensions::ext_custom_border_color::SamplerCustomBorderColorCreateInfoEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_custom_border_color::SamplerCustomBorderColorCreateInfoEXT {
            s_type: StructureType::SamplerCustomBorderColorCreateInfoExt,
            p_next: std::ptr::null(),
            custom_border_color: self.custom_border_color.into_low_level(context, bump),
            format: self.format.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for SamplerCustomBorderColorCreateInfoEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            custom_border_color: crate::conv::FromLowLevel::from_low_level(context, value.custom_border_color),
            format: crate::conv::FromLowLevel::from_low_level(context, value.format),
        }
    }
}
#[doc(alias = "VkPhysicalDeviceCustomBorderColorPropertiesEXT")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceCustomBorderColorPropertiesEXT {
    #[doc(alias = "maxCustomBorderColorSamplers")]
    pub max_custom_border_color_samplers: u32,
}
impl PhysicalDeviceCustomBorderColorPropertiesEXT {
    ///Get a reference to the `max_custom_border_color_samplers` field.
    pub fn max_custom_border_color_samplers(&self) -> u32 {
        self.max_custom_border_color_samplers
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceCustomBorderColorPropertiesEXT {
    type LowLevel = crate::native::extensions::ext_custom_border_color::PhysicalDeviceCustomBorderColorPropertiesEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_custom_border_color::PhysicalDeviceCustomBorderColorPropertiesEXT {
            s_type: StructureType::PhysicalDeviceCustomBorderColorPropertiesExt,
            p_next: std::ptr::null_mut(),
            max_custom_border_color_samplers: self.max_custom_border_color_samplers.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceCustomBorderColorPropertiesEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            max_custom_border_color_samplers: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_custom_border_color_samplers,
            ),
        }
    }
}
#[doc(alias = "VkPhysicalDeviceCustomBorderColorFeaturesEXT")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceCustomBorderColorFeaturesEXT {
    #[doc(alias = "customBorderColors")]
    pub custom_border_colors: bool,
    #[doc(alias = "customBorderColorWithoutFormat")]
    pub custom_border_color_without_format: bool,
}
impl PhysicalDeviceCustomBorderColorFeaturesEXT {
    ///Get a reference to the `custom_border_colors` field.
    pub fn custom_border_colors(&self) -> &bool {
        &self.custom_border_colors
    }
    ///Get a reference to the `custom_border_color_without_format` field.
    pub fn custom_border_color_without_format(&self) -> &bool {
        &self.custom_border_color_without_format
    }
    ///Get a mutable reference to the `custom_border_colors` field.
    pub fn custom_border_colors_mut(&mut self) -> &mut bool {
        &mut self.custom_border_colors
    }
    ///Get a mutable reference to the `custom_border_color_without_format` field.
    pub fn custom_border_color_without_format_mut(&mut self) -> &mut bool {
        &mut self.custom_border_color_without_format
    }
    ///Sets the `custom_border_colors` field.
    pub fn set_custom_border_colors(&mut self, custom_border_colors: bool) -> &mut Self {
        self.custom_border_colors = custom_border_colors;
        self
    }
    ///Sets the `custom_border_color_without_format` field.
    pub fn set_custom_border_color_without_format(&mut self, custom_border_color_without_format: bool) -> &mut Self {
        self.custom_border_color_without_format = custom_border_color_without_format;
        self
    }
    ///Sets the `custom_border_colors` field in a builder way.
    pub fn with_custom_border_colors(mut self, custom_border_colors: bool) -> Self {
        self.custom_border_colors = custom_border_colors;
        self
    }
    ///Sets the `custom_border_color_without_format` field in a builder way.
    pub fn with_custom_border_color_without_format(mut self, custom_border_color_without_format: bool) -> Self {
        self.custom_border_color_without_format = custom_border_color_without_format;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceCustomBorderColorFeaturesEXT {
    type LowLevel = crate::native::extensions::ext_custom_border_color::PhysicalDeviceCustomBorderColorFeaturesEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_custom_border_color::PhysicalDeviceCustomBorderColorFeaturesEXT {
            s_type: StructureType::PhysicalDeviceCustomBorderColorFeaturesExt,
            p_next: std::ptr::null_mut(),
            custom_border_colors: self.custom_border_colors.into_low_level(context, bump),
            custom_border_color_without_format: self.custom_border_color_without_format.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceCustomBorderColorFeaturesEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            custom_border_colors: crate::conv::FromLowLevel::from_low_level(context, value.custom_border_colors),
            custom_border_color_without_format: crate::conv::FromLowLevel::from_low_level(
                context,
                value.custom_border_color_without_format,
            ),
        }
    }
}
