use crate::native::vulkan1_0::{BaseInStructure, StructureType};
#[doc(alias = "VkPipelineCompilerControlCreateInfoAMD")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PipelineCompilerControlCreateInfoAMD {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "compilerControlFlags")]
    pub compiler_control_flags: PipelineCompilerControlFlagsAMD,
}
impl Default for PipelineCompilerControlCreateInfoAMD {
    fn default() -> Self {
        Self {
            s_type: StructureType::PipelineCompilerControlCreateInfoAmd,
            p_next: unsafe { std::mem::zeroed() },
            compiler_control_flags: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::amd_pipeline_compiler_control::{
    PipelineCompilerControlFlagBitsAMD, PipelineCompilerControlFlagsAMD, AMD_PIPELINE_COMPILER_CONTROL_EXTENSION_NAME,
    AMD_PIPELINE_COMPILER_CONTROL_SPEC_VERSION,
};
