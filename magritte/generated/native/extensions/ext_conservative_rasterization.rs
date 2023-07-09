use crate::native::vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, StructureType};
#[doc(alias = "VkPhysicalDeviceConservativeRasterizationPropertiesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceConservativeRasterizationPropertiesEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "primitiveOverestimationSize")]
    pub primitive_overestimation_size: f32,
    #[doc(alias = "maxExtraPrimitiveOverestimationSize")]
    pub max_extra_primitive_overestimation_size: f32,
    #[doc(alias = "extraPrimitiveOverestimationSizeGranularity")]
    pub extra_primitive_overestimation_size_granularity: f32,
    #[doc(alias = "primitiveUnderestimation")]
    pub primitive_underestimation: Bool32,
    #[doc(alias = "conservativePointAndLineRasterization")]
    pub conservative_point_and_line_rasterization: Bool32,
    #[doc(alias = "degenerateTrianglesRasterized")]
    pub degenerate_triangles_rasterized: Bool32,
    #[doc(alias = "degenerateLinesRasterized")]
    pub degenerate_lines_rasterized: Bool32,
    #[doc(alias = "fullyCoveredFragmentShaderInputVariable")]
    pub fully_covered_fragment_shader_input_variable: Bool32,
    #[doc(alias = "conservativeRasterizationPostDepthCoverage")]
    pub conservative_rasterization_post_depth_coverage: Bool32,
}
impl Default for PhysicalDeviceConservativeRasterizationPropertiesEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceConservativeRasterizationPropertiesExt,
            p_next: unsafe { std::mem::zeroed() },
            primitive_overestimation_size: unsafe { std::mem::zeroed() },
            max_extra_primitive_overestimation_size: unsafe { std::mem::zeroed() },
            extra_primitive_overestimation_size_granularity: unsafe { std::mem::zeroed() },
            primitive_underestimation: unsafe { std::mem::zeroed() },
            conservative_point_and_line_rasterization: unsafe { std::mem::zeroed() },
            degenerate_triangles_rasterized: unsafe { std::mem::zeroed() },
            degenerate_lines_rasterized: unsafe { std::mem::zeroed() },
            fully_covered_fragment_shader_input_variable: unsafe { std::mem::zeroed() },
            conservative_rasterization_post_depth_coverage: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPipelineRasterizationConservativeStateCreateInfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PipelineRasterizationConservativeStateCreateInfoEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub flags: PipelineRasterizationConservativeStateCreateFlagsEXT,
    #[doc(alias = "conservativeRasterizationMode")]
    pub conservative_rasterization_mode: ConservativeRasterizationModeEXT,
    #[doc(alias = "extraPrimitiveOverestimationSize")]
    pub extra_primitive_overestimation_size: f32,
}
impl Default for PipelineRasterizationConservativeStateCreateInfoEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::PipelineRasterizationConservativeStateCreateInfoExt,
            p_next: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
            conservative_rasterization_mode: unsafe { std::mem::zeroed() },
            extra_primitive_overestimation_size: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::ext_conservative_rasterization::{
    ConservativeRasterizationModeEXT, PipelineRasterizationConservativeStateCreateFlagsEXT,
    EXT_CONSERVATIVE_RASTERIZATION_EXTENSION_NAME, EXT_CONSERVATIVE_RASTERIZATION_SPEC_VERSION,
};
