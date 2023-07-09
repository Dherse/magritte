pub use crate::common::extensions::nv_coverage_reduction_mode::{
    CoverageReductionModeNV, PipelineCoverageReductionStateCreateFlagsNV, NV_COVERAGE_REDUCTION_MODE_EXTENSION_NAME,
    NV_COVERAGE_REDUCTION_MODE_SPEC_VERSION,
};
use crate::{
    context::Context,
    vulkan1_0::{SampleCountFlagBits, SampleCountFlags, StructureType},
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkPhysicalDeviceCoverageReductionModeFeaturesNV")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceCoverageReductionModeFeaturesNV {
    #[doc(alias = "coverageReductionMode")]
    pub coverage_reduction_mode: bool,
}
impl PhysicalDeviceCoverageReductionModeFeaturesNV {
    ///Get a reference to the `coverage_reduction_mode` field.
    pub fn coverage_reduction_mode(&self) -> &bool {
        &self.coverage_reduction_mode
    }
    ///Get a mutable reference to the `coverage_reduction_mode` field.
    pub fn coverage_reduction_mode_mut(&mut self) -> &mut bool {
        &mut self.coverage_reduction_mode
    }
    ///Sets the `coverage_reduction_mode` field.
    pub fn set_coverage_reduction_mode(&mut self, coverage_reduction_mode: bool) -> &mut Self {
        self.coverage_reduction_mode = coverage_reduction_mode;
        self
    }
    ///Sets the `coverage_reduction_mode` field in a builder way.
    pub fn with_coverage_reduction_mode(mut self, coverage_reduction_mode: bool) -> Self {
        self.coverage_reduction_mode = coverage_reduction_mode;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceCoverageReductionModeFeaturesNV {
    type LowLevel =
        crate::native::extensions::nv_coverage_reduction_mode::PhysicalDeviceCoverageReductionModeFeaturesNV;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::nv_coverage_reduction_mode::PhysicalDeviceCoverageReductionModeFeaturesNV {
            s_type: StructureType::PhysicalDeviceCoverageReductionModeFeaturesNv,
            p_next: std::ptr::null_mut(),
            coverage_reduction_mode: self.coverage_reduction_mode.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceCoverageReductionModeFeaturesNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            coverage_reduction_mode: crate::conv::FromLowLevel::from_low_level(context, value.coverage_reduction_mode),
        }
    }
}
#[doc(alias = "VkPipelineCoverageReductionStateCreateInfoNV")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PipelineCoverageReductionStateCreateInfoNV {
    pub flags: PipelineCoverageReductionStateCreateFlagsNV,
    #[doc(alias = "coverageReductionMode")]
    pub coverage_reduction_mode: CoverageReductionModeNV,
}
impl PipelineCoverageReductionStateCreateInfoNV {
    ///Get a reference to the `flags` field.
    pub fn flags(&self) -> PipelineCoverageReductionStateCreateFlagsNV {
        self.flags
    }
    ///Get a reference to the `coverage_reduction_mode` field.
    pub fn coverage_reduction_mode(&self) -> CoverageReductionModeNV {
        self.coverage_reduction_mode
    }
    ///Get a mutable reference to the `flags` field.
    pub fn flags_mut(&mut self) -> &mut PipelineCoverageReductionStateCreateFlagsNV {
        &mut self.flags
    }
    ///Get a mutable reference to the `coverage_reduction_mode` field.
    pub fn coverage_reduction_mode_mut(&mut self) -> &mut CoverageReductionModeNV {
        &mut self.coverage_reduction_mode
    }
    ///Sets the `flags` field.
    pub fn set_flags(&mut self, flags: PipelineCoverageReductionStateCreateFlagsNV) -> &mut Self {
        self.flags = flags;
        self
    }
    ///Sets the `coverage_reduction_mode` field.
    pub fn set_coverage_reduction_mode(&mut self, coverage_reduction_mode: CoverageReductionModeNV) -> &mut Self {
        self.coverage_reduction_mode = coverage_reduction_mode;
        self
    }
    ///Sets the `flags` field in a builder way.
    pub fn with_flags(mut self, flags: PipelineCoverageReductionStateCreateFlagsNV) -> Self {
        self.flags = flags;
        self
    }
    ///Sets the `coverage_reduction_mode` field in a builder way.
    pub fn with_coverage_reduction_mode(mut self, coverage_reduction_mode: CoverageReductionModeNV) -> Self {
        self.coverage_reduction_mode = coverage_reduction_mode;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PipelineCoverageReductionStateCreateInfoNV {
    type LowLevel = crate::native::extensions::nv_coverage_reduction_mode::PipelineCoverageReductionStateCreateInfoNV;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::nv_coverage_reduction_mode::PipelineCoverageReductionStateCreateInfoNV {
            s_type: StructureType::PipelineCoverageReductionStateCreateInfoNv,
            p_next: std::ptr::null(),
            flags: self.flags.into_low_level(context, bump),
            coverage_reduction_mode: self.coverage_reduction_mode.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PipelineCoverageReductionStateCreateInfoNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            flags: crate::conv::FromLowLevel::from_low_level(context, value.flags),
            coverage_reduction_mode: crate::conv::FromLowLevel::from_low_level(context, value.coverage_reduction_mode),
        }
    }
}
#[doc(alias = "VkFramebufferMixedSamplesCombinationNV")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FramebufferMixedSamplesCombinationNV {
    #[doc(alias = "coverageReductionMode")]
    pub coverage_reduction_mode: CoverageReductionModeNV,
    #[doc(alias = "rasterizationSamples")]
    pub rasterization_samples: SampleCountFlagBits,
    #[doc(alias = "depthStencilSamples")]
    pub depth_stencil_samples: SampleCountFlags,
    #[doc(alias = "colorSamples")]
    pub color_samples: SampleCountFlags,
}
impl FramebufferMixedSamplesCombinationNV {
    ///Get a reference to the `coverage_reduction_mode` field.
    pub fn coverage_reduction_mode(&self) -> CoverageReductionModeNV {
        self.coverage_reduction_mode
    }
    ///Get a reference to the `rasterization_samples` field.
    pub fn rasterization_samples(&self) -> SampleCountFlagBits {
        self.rasterization_samples
    }
    ///Get a reference to the `depth_stencil_samples` field.
    pub fn depth_stencil_samples(&self) -> SampleCountFlags {
        self.depth_stencil_samples
    }
    ///Get a reference to the `color_samples` field.
    pub fn color_samples(&self) -> SampleCountFlags {
        self.color_samples
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for FramebufferMixedSamplesCombinationNV {
    type LowLevel = crate::native::extensions::nv_coverage_reduction_mode::FramebufferMixedSamplesCombinationNV;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::nv_coverage_reduction_mode::FramebufferMixedSamplesCombinationNV {
            s_type: StructureType::FramebufferMixedSamplesCombinationNv,
            p_next: std::ptr::null_mut(),
            coverage_reduction_mode: self.coverage_reduction_mode.into_low_level(context, bump),
            rasterization_samples: self.rasterization_samples.into_low_level(context, bump),
            depth_stencil_samples: self.depth_stencil_samples.into_low_level(context, bump),
            color_samples: self.color_samples.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for FramebufferMixedSamplesCombinationNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            coverage_reduction_mode: crate::conv::FromLowLevel::from_low_level(context, value.coverage_reduction_mode),
            rasterization_samples: crate::conv::FromLowLevel::from_low_level(context, value.rasterization_samples),
            depth_stencil_samples: crate::conv::FromLowLevel::from_low_level(context, value.depth_stencil_samples),
            color_samples: crate::conv::FromLowLevel::from_low_level(context, value.color_samples),
        }
    }
}
