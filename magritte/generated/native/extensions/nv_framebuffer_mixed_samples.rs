use crate::native::vulkan1_0::{BaseInStructure, Bool32, StructureType};
#[doc(alias = "VkPipelineCoverageModulationStateCreateInfoNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PipelineCoverageModulationStateCreateInfoNV {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub flags: PipelineCoverageModulationStateCreateFlagsNV,
    #[doc(alias = "coverageModulationMode")]
    pub coverage_modulation_mode: CoverageModulationModeNV,
    #[doc(alias = "coverageModulationTableEnable")]
    pub coverage_modulation_table_enable: Bool32,
    #[doc(alias = "coverageModulationTableCount")]
    pub coverage_modulation_table_count: u32,
    #[doc(alias = "pCoverageModulationTable")]
    pub coverage_modulation_table: *const f32,
}
impl Default for PipelineCoverageModulationStateCreateInfoNV {
    fn default() -> Self {
        Self {
            s_type: StructureType::PipelineCoverageModulationStateCreateInfoNv,
            p_next: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
            coverage_modulation_mode: unsafe { std::mem::zeroed() },
            coverage_modulation_table_enable: unsafe { std::mem::zeroed() },
            coverage_modulation_table_count: unsafe { std::mem::zeroed() },
            coverage_modulation_table: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::nv_framebuffer_mixed_samples::{
    CoverageModulationModeNV, PipelineCoverageModulationStateCreateFlagsNV,
    NV_FRAMEBUFFER_MIXED_SAMPLES_EXTENSION_NAME, NV_FRAMEBUFFER_MIXED_SAMPLES_SPEC_VERSION,
};
