pub use crate::common::extensions::nv_fragment_coverage_to_color::{
    PipelineCoverageToColorStateCreateFlagsNV, NV_FRAGMENT_COVERAGE_TO_COLOR_EXTENSION_NAME,
    NV_FRAGMENT_COVERAGE_TO_COLOR_SPEC_VERSION,
};
use crate::{context::Context, vulkan1_0::StructureType};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkPipelineCoverageToColorStateCreateInfoNV")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PipelineCoverageToColorStateCreateInfoNV {
    pub flags: PipelineCoverageToColorStateCreateFlagsNV,
    #[doc(alias = "coverageToColorEnable")]
    pub coverage_to_color_enable: bool,
    #[doc(alias = "coverageToColorLocation")]
    pub coverage_to_color_location: u32,
}
impl PipelineCoverageToColorStateCreateInfoNV {
    ///Get a reference to the `flags` field.
    pub fn flags(&self) -> PipelineCoverageToColorStateCreateFlagsNV {
        self.flags
    }
    ///Get a reference to the `coverage_to_color_enable` field.
    pub fn coverage_to_color_enable(&self) -> &bool {
        &self.coverage_to_color_enable
    }
    ///Get a reference to the `coverage_to_color_location` field.
    pub fn coverage_to_color_location(&self) -> u32 {
        self.coverage_to_color_location
    }
    ///Get a mutable reference to the `flags` field.
    pub fn flags_mut(&mut self) -> &mut PipelineCoverageToColorStateCreateFlagsNV {
        &mut self.flags
    }
    ///Get a mutable reference to the `coverage_to_color_enable` field.
    pub fn coverage_to_color_enable_mut(&mut self) -> &mut bool {
        &mut self.coverage_to_color_enable
    }
    ///Get a mutable reference to the `coverage_to_color_location` field.
    pub fn coverage_to_color_location_mut(&mut self) -> &mut u32 {
        &mut self.coverage_to_color_location
    }
    ///Sets the `flags` field.
    pub fn set_flags(&mut self, flags: PipelineCoverageToColorStateCreateFlagsNV) -> &mut Self {
        self.flags = flags;
        self
    }
    ///Sets the `coverage_to_color_enable` field.
    pub fn set_coverage_to_color_enable(&mut self, coverage_to_color_enable: bool) -> &mut Self {
        self.coverage_to_color_enable = coverage_to_color_enable;
        self
    }
    ///Sets the `coverage_to_color_location` field.
    pub fn set_coverage_to_color_location(&mut self, coverage_to_color_location: u32) -> &mut Self {
        self.coverage_to_color_location = coverage_to_color_location;
        self
    }
    ///Sets the `flags` field in a builder way.
    pub fn with_flags(mut self, flags: PipelineCoverageToColorStateCreateFlagsNV) -> Self {
        self.flags = flags;
        self
    }
    ///Sets the `coverage_to_color_enable` field in a builder way.
    pub fn with_coverage_to_color_enable(mut self, coverage_to_color_enable: bool) -> Self {
        self.coverage_to_color_enable = coverage_to_color_enable;
        self
    }
    ///Sets the `coverage_to_color_location` field in a builder way.
    pub fn with_coverage_to_color_location(mut self, coverage_to_color_location: u32) -> Self {
        self.coverage_to_color_location = coverage_to_color_location;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PipelineCoverageToColorStateCreateInfoNV {
    type LowLevel = crate::native::extensions::nv_fragment_coverage_to_color::PipelineCoverageToColorStateCreateInfoNV;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::nv_fragment_coverage_to_color::PipelineCoverageToColorStateCreateInfoNV {
            s_type: StructureType::PipelineCoverageToColorStateCreateInfoNv,
            p_next: std::ptr::null(),
            flags: self.flags.into_low_level(context, bump),
            coverage_to_color_enable: self.coverage_to_color_enable.into_low_level(context, bump),
            coverage_to_color_location: self.coverage_to_color_location.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PipelineCoverageToColorStateCreateInfoNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            flags: crate::conv::FromLowLevel::from_low_level(context, value.flags),
            coverage_to_color_enable: crate::conv::FromLowLevel::from_low_level(
                context,
                value.coverage_to_color_enable,
            ),
            coverage_to_color_location: crate::conv::FromLowLevel::from_low_level(
                context,
                value.coverage_to_color_location,
            ),
        }
    }
}
