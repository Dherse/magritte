use crate::native::vulkan1_0::{BaseOutStructure, Bool32, StructureType};
#[doc(alias = "VkPhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "shaderSubgroupUniformControlFlow")]
    pub shader_subgroup_uniform_control_flow: Bool32,
}
impl Default for PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKhr,
            p_next: unsafe { std::mem::zeroed() },
            shader_subgroup_uniform_control_flow: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::khr_shader_subgroup_uniform_control_flow::{
    KHR_SHADER_SUBGROUP_UNIFORM_CONTROL_FLOW_EXTENSION_NAME, KHR_SHADER_SUBGROUP_UNIFORM_CONTROL_FLOW_SPEC_VERSION,
};
