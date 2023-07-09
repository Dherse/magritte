pub use crate::common::extensions::ext_line_rasterization::{
    LineRasterizationModeEXT, EXT_LINE_RASTERIZATION_EXTENSION_NAME, EXT_LINE_RASTERIZATION_SPEC_VERSION,
};
use crate::{context::Context, vulkan1_0::StructureType};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkPhysicalDeviceLineRasterizationFeaturesEXT")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceLineRasterizationFeaturesEXT {
    #[doc(alias = "rectangularLines")]
    pub rectangular_lines: bool,
    #[doc(alias = "bresenhamLines")]
    pub bresenham_lines: bool,
    #[doc(alias = "smoothLines")]
    pub smooth_lines: bool,
    #[doc(alias = "stippledRectangularLines")]
    pub stippled_rectangular_lines: bool,
    #[doc(alias = "stippledBresenhamLines")]
    pub stippled_bresenham_lines: bool,
    #[doc(alias = "stippledSmoothLines")]
    pub stippled_smooth_lines: bool,
}
impl PhysicalDeviceLineRasterizationFeaturesEXT {
    ///Get a reference to the `rectangular_lines` field.
    pub fn rectangular_lines(&self) -> &bool {
        &self.rectangular_lines
    }
    ///Get a reference to the `bresenham_lines` field.
    pub fn bresenham_lines(&self) -> &bool {
        &self.bresenham_lines
    }
    ///Get a reference to the `smooth_lines` field.
    pub fn smooth_lines(&self) -> &bool {
        &self.smooth_lines
    }
    ///Get a reference to the `stippled_rectangular_lines` field.
    pub fn stippled_rectangular_lines(&self) -> &bool {
        &self.stippled_rectangular_lines
    }
    ///Get a reference to the `stippled_bresenham_lines` field.
    pub fn stippled_bresenham_lines(&self) -> &bool {
        &self.stippled_bresenham_lines
    }
    ///Get a reference to the `stippled_smooth_lines` field.
    pub fn stippled_smooth_lines(&self) -> &bool {
        &self.stippled_smooth_lines
    }
    ///Get a mutable reference to the `rectangular_lines` field.
    pub fn rectangular_lines_mut(&mut self) -> &mut bool {
        &mut self.rectangular_lines
    }
    ///Get a mutable reference to the `bresenham_lines` field.
    pub fn bresenham_lines_mut(&mut self) -> &mut bool {
        &mut self.bresenham_lines
    }
    ///Get a mutable reference to the `smooth_lines` field.
    pub fn smooth_lines_mut(&mut self) -> &mut bool {
        &mut self.smooth_lines
    }
    ///Get a mutable reference to the `stippled_rectangular_lines` field.
    pub fn stippled_rectangular_lines_mut(&mut self) -> &mut bool {
        &mut self.stippled_rectangular_lines
    }
    ///Get a mutable reference to the `stippled_bresenham_lines` field.
    pub fn stippled_bresenham_lines_mut(&mut self) -> &mut bool {
        &mut self.stippled_bresenham_lines
    }
    ///Get a mutable reference to the `stippled_smooth_lines` field.
    pub fn stippled_smooth_lines_mut(&mut self) -> &mut bool {
        &mut self.stippled_smooth_lines
    }
    ///Sets the `rectangular_lines` field.
    pub fn set_rectangular_lines(&mut self, rectangular_lines: bool) -> &mut Self {
        self.rectangular_lines = rectangular_lines;
        self
    }
    ///Sets the `bresenham_lines` field.
    pub fn set_bresenham_lines(&mut self, bresenham_lines: bool) -> &mut Self {
        self.bresenham_lines = bresenham_lines;
        self
    }
    ///Sets the `smooth_lines` field.
    pub fn set_smooth_lines(&mut self, smooth_lines: bool) -> &mut Self {
        self.smooth_lines = smooth_lines;
        self
    }
    ///Sets the `stippled_rectangular_lines` field.
    pub fn set_stippled_rectangular_lines(&mut self, stippled_rectangular_lines: bool) -> &mut Self {
        self.stippled_rectangular_lines = stippled_rectangular_lines;
        self
    }
    ///Sets the `stippled_bresenham_lines` field.
    pub fn set_stippled_bresenham_lines(&mut self, stippled_bresenham_lines: bool) -> &mut Self {
        self.stippled_bresenham_lines = stippled_bresenham_lines;
        self
    }
    ///Sets the `stippled_smooth_lines` field.
    pub fn set_stippled_smooth_lines(&mut self, stippled_smooth_lines: bool) -> &mut Self {
        self.stippled_smooth_lines = stippled_smooth_lines;
        self
    }
    ///Sets the `rectangular_lines` field in a builder way.
    pub fn with_rectangular_lines(mut self, rectangular_lines: bool) -> Self {
        self.rectangular_lines = rectangular_lines;
        self
    }
    ///Sets the `bresenham_lines` field in a builder way.
    pub fn with_bresenham_lines(mut self, bresenham_lines: bool) -> Self {
        self.bresenham_lines = bresenham_lines;
        self
    }
    ///Sets the `smooth_lines` field in a builder way.
    pub fn with_smooth_lines(mut self, smooth_lines: bool) -> Self {
        self.smooth_lines = smooth_lines;
        self
    }
    ///Sets the `stippled_rectangular_lines` field in a builder way.
    pub fn with_stippled_rectangular_lines(mut self, stippled_rectangular_lines: bool) -> Self {
        self.stippled_rectangular_lines = stippled_rectangular_lines;
        self
    }
    ///Sets the `stippled_bresenham_lines` field in a builder way.
    pub fn with_stippled_bresenham_lines(mut self, stippled_bresenham_lines: bool) -> Self {
        self.stippled_bresenham_lines = stippled_bresenham_lines;
        self
    }
    ///Sets the `stippled_smooth_lines` field in a builder way.
    pub fn with_stippled_smooth_lines(mut self, stippled_smooth_lines: bool) -> Self {
        self.stippled_smooth_lines = stippled_smooth_lines;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceLineRasterizationFeaturesEXT {
    type LowLevel = crate::native::extensions::ext_line_rasterization::PhysicalDeviceLineRasterizationFeaturesEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_line_rasterization::PhysicalDeviceLineRasterizationFeaturesEXT {
            s_type: StructureType::PhysicalDeviceLineRasterizationFeaturesExt,
            p_next: std::ptr::null_mut(),
            rectangular_lines: self.rectangular_lines.into_low_level(context, bump),
            bresenham_lines: self.bresenham_lines.into_low_level(context, bump),
            smooth_lines: self.smooth_lines.into_low_level(context, bump),
            stippled_rectangular_lines: self.stippled_rectangular_lines.into_low_level(context, bump),
            stippled_bresenham_lines: self.stippled_bresenham_lines.into_low_level(context, bump),
            stippled_smooth_lines: self.stippled_smooth_lines.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceLineRasterizationFeaturesEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            rectangular_lines: crate::conv::FromLowLevel::from_low_level(context, value.rectangular_lines),
            bresenham_lines: crate::conv::FromLowLevel::from_low_level(context, value.bresenham_lines),
            smooth_lines: crate::conv::FromLowLevel::from_low_level(context, value.smooth_lines),
            stippled_rectangular_lines: crate::conv::FromLowLevel::from_low_level(
                context,
                value.stippled_rectangular_lines,
            ),
            stippled_bresenham_lines: crate::conv::FromLowLevel::from_low_level(
                context,
                value.stippled_bresenham_lines,
            ),
            stippled_smooth_lines: crate::conv::FromLowLevel::from_low_level(context, value.stippled_smooth_lines),
        }
    }
}
#[doc(alias = "VkPhysicalDeviceLineRasterizationPropertiesEXT")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceLineRasterizationPropertiesEXT {
    #[doc(alias = "lineSubPixelPrecisionBits")]
    pub line_sub_pixel_precision_bits: u32,
}
impl PhysicalDeviceLineRasterizationPropertiesEXT {
    ///Get a reference to the `line_sub_pixel_precision_bits` field.
    pub fn line_sub_pixel_precision_bits(&self) -> u32 {
        self.line_sub_pixel_precision_bits
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceLineRasterizationPropertiesEXT {
    type LowLevel = crate::native::extensions::ext_line_rasterization::PhysicalDeviceLineRasterizationPropertiesEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_line_rasterization::PhysicalDeviceLineRasterizationPropertiesEXT {
            s_type: StructureType::PhysicalDeviceLineRasterizationPropertiesExt,
            p_next: std::ptr::null_mut(),
            line_sub_pixel_precision_bits: self.line_sub_pixel_precision_bits.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceLineRasterizationPropertiesEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            line_sub_pixel_precision_bits: crate::conv::FromLowLevel::from_low_level(
                context,
                value.line_sub_pixel_precision_bits,
            ),
        }
    }
}
#[doc(alias = "VkPipelineRasterizationLineStateCreateInfoEXT")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PipelineRasterizationLineStateCreateInfoEXT {
    #[doc(alias = "lineRasterizationMode")]
    pub line_rasterization_mode: LineRasterizationModeEXT,
    #[doc(alias = "stippledLineEnable")]
    pub stippled_line_enable: bool,
    #[doc(alias = "lineStippleFactor")]
    pub line_stipple_factor: u32,
    #[doc(alias = "lineStipplePattern")]
    pub line_stipple_pattern: u16,
}
impl PipelineRasterizationLineStateCreateInfoEXT {
    ///Get a reference to the `line_rasterization_mode` field.
    pub fn line_rasterization_mode(&self) -> LineRasterizationModeEXT {
        self.line_rasterization_mode
    }
    ///Get a reference to the `stippled_line_enable` field.
    pub fn stippled_line_enable(&self) -> &bool {
        &self.stippled_line_enable
    }
    ///Get a reference to the `line_stipple_factor` field.
    pub fn line_stipple_factor(&self) -> u32 {
        self.line_stipple_factor
    }
    ///Get a reference to the `line_stipple_pattern` field.
    pub fn line_stipple_pattern(&self) -> u16 {
        self.line_stipple_pattern
    }
    ///Get a mutable reference to the `line_rasterization_mode` field.
    pub fn line_rasterization_mode_mut(&mut self) -> &mut LineRasterizationModeEXT {
        &mut self.line_rasterization_mode
    }
    ///Get a mutable reference to the `stippled_line_enable` field.
    pub fn stippled_line_enable_mut(&mut self) -> &mut bool {
        &mut self.stippled_line_enable
    }
    ///Get a mutable reference to the `line_stipple_factor` field.
    pub fn line_stipple_factor_mut(&mut self) -> &mut u32 {
        &mut self.line_stipple_factor
    }
    ///Get a mutable reference to the `line_stipple_pattern` field.
    pub fn line_stipple_pattern_mut(&mut self) -> &mut u16 {
        &mut self.line_stipple_pattern
    }
    ///Sets the `line_rasterization_mode` field.
    pub fn set_line_rasterization_mode(&mut self, line_rasterization_mode: LineRasterizationModeEXT) -> &mut Self {
        self.line_rasterization_mode = line_rasterization_mode;
        self
    }
    ///Sets the `stippled_line_enable` field.
    pub fn set_stippled_line_enable(&mut self, stippled_line_enable: bool) -> &mut Self {
        self.stippled_line_enable = stippled_line_enable;
        self
    }
    ///Sets the `line_stipple_factor` field.
    pub fn set_line_stipple_factor(&mut self, line_stipple_factor: u32) -> &mut Self {
        self.line_stipple_factor = line_stipple_factor;
        self
    }
    ///Sets the `line_stipple_pattern` field.
    pub fn set_line_stipple_pattern(&mut self, line_stipple_pattern: u16) -> &mut Self {
        self.line_stipple_pattern = line_stipple_pattern;
        self
    }
    ///Sets the `line_rasterization_mode` field in a builder way.
    pub fn with_line_rasterization_mode(mut self, line_rasterization_mode: LineRasterizationModeEXT) -> Self {
        self.line_rasterization_mode = line_rasterization_mode;
        self
    }
    ///Sets the `stippled_line_enable` field in a builder way.
    pub fn with_stippled_line_enable(mut self, stippled_line_enable: bool) -> Self {
        self.stippled_line_enable = stippled_line_enable;
        self
    }
    ///Sets the `line_stipple_factor` field in a builder way.
    pub fn with_line_stipple_factor(mut self, line_stipple_factor: u32) -> Self {
        self.line_stipple_factor = line_stipple_factor;
        self
    }
    ///Sets the `line_stipple_pattern` field in a builder way.
    pub fn with_line_stipple_pattern(mut self, line_stipple_pattern: u16) -> Self {
        self.line_stipple_pattern = line_stipple_pattern;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PipelineRasterizationLineStateCreateInfoEXT {
    type LowLevel = crate::native::extensions::ext_line_rasterization::PipelineRasterizationLineStateCreateInfoEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_line_rasterization::PipelineRasterizationLineStateCreateInfoEXT {
            s_type: StructureType::PipelineRasterizationLineStateCreateInfoExt,
            p_next: std::ptr::null(),
            line_rasterization_mode: self.line_rasterization_mode.into_low_level(context, bump),
            stippled_line_enable: self.stippled_line_enable.into_low_level(context, bump),
            line_stipple_factor: self.line_stipple_factor.into_low_level(context, bump),
            line_stipple_pattern: self.line_stipple_pattern.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PipelineRasterizationLineStateCreateInfoEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            line_rasterization_mode: crate::conv::FromLowLevel::from_low_level(context, value.line_rasterization_mode),
            stippled_line_enable: crate::conv::FromLowLevel::from_low_level(context, value.stippled_line_enable),
            line_stipple_factor: crate::conv::FromLowLevel::from_low_level(context, value.line_stipple_factor),
            line_stipple_pattern: crate::conv::FromLowLevel::from_low_level(context, value.line_stipple_pattern),
        }
    }
}
