pub use crate::common::extensions::amd_pipeline_compiler_control::{
    PipelineCompilerControlFlagBitsAMD, PipelineCompilerControlFlagsAMD, AMD_PIPELINE_COMPILER_CONTROL_EXTENSION_NAME,
    AMD_PIPELINE_COMPILER_CONTROL_SPEC_VERSION,
};
use crate::{context::Context, vulkan1_0::StructureType};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkPipelineCompilerControlCreateInfoAMD")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PipelineCompilerControlCreateInfoAMD {
    #[doc(alias = "compilerControlFlags")]
    pub compiler_control_flags: PipelineCompilerControlFlagsAMD,
}
impl PipelineCompilerControlCreateInfoAMD {
    ///Get a reference to the `compiler_control_flags` field.
    pub fn compiler_control_flags(&self) -> PipelineCompilerControlFlagsAMD {
        self.compiler_control_flags
    }
    ///Get a mutable reference to the `compiler_control_flags` field.
    pub fn compiler_control_flags_mut(&mut self) -> &mut PipelineCompilerControlFlagsAMD {
        &mut self.compiler_control_flags
    }
    ///Sets the `compiler_control_flags` field.
    pub fn set_compiler_control_flags(&mut self, compiler_control_flags: PipelineCompilerControlFlagsAMD) -> &mut Self {
        self.compiler_control_flags = compiler_control_flags;
        self
    }
    ///Sets the `compiler_control_flags` field in a builder way.
    pub fn with_compiler_control_flags(mut self, compiler_control_flags: PipelineCompilerControlFlagsAMD) -> Self {
        self.compiler_control_flags = compiler_control_flags;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PipelineCompilerControlCreateInfoAMD {
    type LowLevel = crate::native::extensions::amd_pipeline_compiler_control::PipelineCompilerControlCreateInfoAMD;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::amd_pipeline_compiler_control::PipelineCompilerControlCreateInfoAMD {
            s_type: StructureType::PipelineCompilerControlCreateInfoAmd,
            p_next: std::ptr::null(),
            compiler_control_flags: self.compiler_control_flags.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PipelineCompilerControlCreateInfoAMD {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            compiler_control_flags: crate::conv::FromLowLevel::from_low_level(context, value.compiler_control_flags),
        }
    }
}
