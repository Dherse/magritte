use crate::native::vulkan1_0::{BaseInStructure, Bool32, StructureType};
#[doc(alias = "VkPipelineCoverageToColorStateCreateInfoNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PipelineCoverageToColorStateCreateInfoNV {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub flags: PipelineCoverageToColorStateCreateFlagsNV,
    #[doc(alias = "coverageToColorEnable")]
    pub coverage_to_color_enable: Bool32,
    #[doc(alias = "coverageToColorLocation")]
    pub coverage_to_color_location: u32,
}
impl Default for PipelineCoverageToColorStateCreateInfoNV {
    fn default() -> Self {
        Self {
            s_type: StructureType::PipelineCoverageToColorStateCreateInfoNv,
            p_next: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
            coverage_to_color_enable: unsafe { std::mem::zeroed() },
            coverage_to_color_location: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::nv_fragment_coverage_to_color::{
    PipelineCoverageToColorStateCreateFlagsNV, NV_FRAGMENT_COVERAGE_TO_COLOR_EXTENSION_NAME,
    NV_FRAGMENT_COVERAGE_TO_COLOR_SPEC_VERSION,
};
