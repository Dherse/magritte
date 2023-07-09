use crate::native::vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, StructureType};
#[doc(alias = "VkPhysicalDeviceProvokingVertexFeaturesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceProvokingVertexFeaturesEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "provokingVertexLast")]
    pub provoking_vertex_last: Bool32,
    #[doc(alias = "transformFeedbackPreservesProvokingVertex")]
    pub transform_feedback_preserves_provoking_vertex: Bool32,
}
impl Default for PhysicalDeviceProvokingVertexFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceProvokingVertexFeaturesExt,
            p_next: unsafe { std::mem::zeroed() },
            provoking_vertex_last: unsafe { std::mem::zeroed() },
            transform_feedback_preserves_provoking_vertex: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceProvokingVertexPropertiesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceProvokingVertexPropertiesEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "provokingVertexModePerPipeline")]
    pub provoking_vertex_mode_per_pipeline: Bool32,
    #[doc(alias = "transformFeedbackPreservesTriangleFanProvokingVertex")]
    pub transform_feedback_preserves_triangle_fan_provoking_vertex: Bool32,
}
impl Default for PhysicalDeviceProvokingVertexPropertiesEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceProvokingVertexPropertiesExt,
            p_next: unsafe { std::mem::zeroed() },
            provoking_vertex_mode_per_pipeline: unsafe { std::mem::zeroed() },
            transform_feedback_preserves_triangle_fan_provoking_vertex: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPipelineRasterizationProvokingVertexStateCreateInfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PipelineRasterizationProvokingVertexStateCreateInfoEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "provokingVertexMode")]
    pub provoking_vertex_mode: ProvokingVertexModeEXT,
}
impl Default for PipelineRasterizationProvokingVertexStateCreateInfoEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::PipelineRasterizationProvokingVertexStateCreateInfoExt,
            p_next: unsafe { std::mem::zeroed() },
            provoking_vertex_mode: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::ext_provoking_vertex::{
    ProvokingVertexModeEXT, EXT_PROVOKING_VERTEX_EXTENSION_NAME, EXT_PROVOKING_VERTEX_SPEC_VERSION,
};
