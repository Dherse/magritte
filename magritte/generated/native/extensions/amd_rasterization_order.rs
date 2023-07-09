use crate::native::vulkan1_0::{BaseInStructure, StructureType};
#[doc(alias = "VkPipelineRasterizationStateRasterizationOrderAMD")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PipelineRasterizationStateRasterizationOrderAMD {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "rasterizationOrder")]
    pub rasterization_order: RasterizationOrderAMD,
}
impl Default for PipelineRasterizationStateRasterizationOrderAMD {
    fn default() -> Self {
        Self {
            s_type: StructureType::PipelineRasterizationStateRasterizationOrderAmd,
            p_next: unsafe { std::mem::zeroed() },
            rasterization_order: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::amd_rasterization_order::{
    RasterizationOrderAMD, AMD_RASTERIZATION_ORDER_EXTENSION_NAME, AMD_RASTERIZATION_ORDER_SPEC_VERSION,
};
