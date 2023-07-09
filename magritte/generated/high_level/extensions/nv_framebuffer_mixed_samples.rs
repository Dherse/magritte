pub use crate::common::extensions::nv_framebuffer_mixed_samples::{
    CoverageModulationModeNV, PipelineCoverageModulationStateCreateFlagsNV,
    NV_FRAMEBUFFER_MIXED_SAMPLES_EXTENSION_NAME, NV_FRAMEBUFFER_MIXED_SAMPLES_SPEC_VERSION,
};
use crate::{context::Context, vulkan1_0::StructureType};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use std::sync::Arc;
#[doc(alias = "VkPipelineCoverageModulationStateCreateInfoNV")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PipelineCoverageModulationStateCreateInfoNV {
    pub flags: PipelineCoverageModulationStateCreateFlagsNV,
    #[doc(alias = "coverageModulationMode")]
    pub coverage_modulation_mode: CoverageModulationModeNV,
    #[doc(alias = "coverageModulationTableEnable")]
    pub coverage_modulation_table_enable: bool,
    #[doc(alias = "pCoverageModulationTable")]
    pub coverage_modulation_table: SmallVec<[f32; 8]>,
}
impl PipelineCoverageModulationStateCreateInfoNV {
    ///Get a reference to the `flags` field.
    pub fn flags(&self) -> PipelineCoverageModulationStateCreateFlagsNV {
        self.flags
    }
    ///Get a reference to the `coverage_modulation_mode` field.
    pub fn coverage_modulation_mode(&self) -> CoverageModulationModeNV {
        self.coverage_modulation_mode
    }
    ///Get a reference to the `coverage_modulation_table_enable` field.
    pub fn coverage_modulation_table_enable(&self) -> &bool {
        &self.coverage_modulation_table_enable
    }
    ///Get a reference to the `coverage_modulation_table` field.
    pub fn coverage_modulation_table(&self) -> &SmallVec<[f32; 8]> {
        &self.coverage_modulation_table
    }
    ///Get a mutable reference to the `flags` field.
    pub fn flags_mut(&mut self) -> &mut PipelineCoverageModulationStateCreateFlagsNV {
        &mut self.flags
    }
    ///Get a mutable reference to the `coverage_modulation_mode` field.
    pub fn coverage_modulation_mode_mut(&mut self) -> &mut CoverageModulationModeNV {
        &mut self.coverage_modulation_mode
    }
    ///Get a mutable reference to the `coverage_modulation_table_enable` field.
    pub fn coverage_modulation_table_enable_mut(&mut self) -> &mut bool {
        &mut self.coverage_modulation_table_enable
    }
    ///Get a mutable reference to the `coverage_modulation_table` field.
    pub fn coverage_modulation_table_mut(&mut self) -> &mut SmallVec<[f32; 8]> {
        &mut self.coverage_modulation_table
    }
    ///Sets the `flags` field.
    pub fn set_flags(&mut self, flags: PipelineCoverageModulationStateCreateFlagsNV) -> &mut Self {
        self.flags = flags;
        self
    }
    ///Sets the `coverage_modulation_mode` field.
    pub fn set_coverage_modulation_mode(&mut self, coverage_modulation_mode: CoverageModulationModeNV) -> &mut Self {
        self.coverage_modulation_mode = coverage_modulation_mode;
        self
    }
    ///Sets the `coverage_modulation_table_enable` field.
    pub fn set_coverage_modulation_table_enable(&mut self, coverage_modulation_table_enable: bool) -> &mut Self {
        self.coverage_modulation_table_enable = coverage_modulation_table_enable;
        self
    }
    ///Sets the `coverage_modulation_table` field.
    pub fn set_coverage_modulation_table(&mut self, coverage_modulation_table: SmallVec<[f32; 8]>) -> &mut Self {
        self.coverage_modulation_table = coverage_modulation_table;
        self
    }
    ///Sets the `flags` field in a builder way.
    pub fn with_flags(mut self, flags: PipelineCoverageModulationStateCreateFlagsNV) -> Self {
        self.flags = flags;
        self
    }
    ///Sets the `coverage_modulation_mode` field in a builder way.
    pub fn with_coverage_modulation_mode(mut self, coverage_modulation_mode: CoverageModulationModeNV) -> Self {
        self.coverage_modulation_mode = coverage_modulation_mode;
        self
    }
    ///Sets the `coverage_modulation_table_enable` field in a builder way.
    pub fn with_coverage_modulation_table_enable(mut self, coverage_modulation_table_enable: bool) -> Self {
        self.coverage_modulation_table_enable = coverage_modulation_table_enable;
        self
    }
    ///Sets the `coverage_modulation_table` field in a builder way.
    pub fn with_coverage_modulation_table(mut self, coverage_modulation_table: SmallVec<[f32; 8]>) -> Self {
        self.coverage_modulation_table = coverage_modulation_table;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PipelineCoverageModulationStateCreateInfoNV {
    type LowLevel =
        crate::native::extensions::nv_framebuffer_mixed_samples::PipelineCoverageModulationStateCreateInfoNV;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let len_coverage_modulation_table = self.coverage_modulation_table.len() as u32;
        let coverage_modulation_table = bump
            .alloc_slice_fill_iter(
                self.coverage_modulation_table
                    .iter()
                    .map(|x| x.into_low_level(context, bump)),
            )
            .as_ptr()
            .cast();
        crate::native::extensions::nv_framebuffer_mixed_samples::PipelineCoverageModulationStateCreateInfoNV {
            s_type: StructureType::PipelineCoverageModulationStateCreateInfoNv,
            p_next: std::ptr::null(),
            flags: self.flags.into_low_level(context, bump),
            coverage_modulation_mode: self.coverage_modulation_mode.into_low_level(context, bump),
            coverage_modulation_table_enable: self.coverage_modulation_table_enable.into_low_level(context, bump),
            coverage_modulation_table_count: len_coverage_modulation_table,
            coverage_modulation_table: coverage_modulation_table,
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PipelineCoverageModulationStateCreateInfoNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let coverage_modulation_table_len = value.coverage_modulation_table_count;
        let mut coverage_modulation_table = SmallVec::with_capacity(coverage_modulation_table_len as usize);
        for i in 0..coverage_modulation_table_len {
            coverage_modulation_table.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.coverage_modulation_table.add(i as usize).read(),
            ));
        }
        Self {
            flags: crate::conv::FromLowLevel::from_low_level(context, value.flags),
            coverage_modulation_mode: crate::conv::FromLowLevel::from_low_level(
                context,
                value.coverage_modulation_mode,
            ),
            coverage_modulation_table_enable: crate::conv::FromLowLevel::from_low_level(
                context,
                value.coverage_modulation_table_enable,
            ),
            coverage_modulation_table: coverage_modulation_table,
        }
    }
}
