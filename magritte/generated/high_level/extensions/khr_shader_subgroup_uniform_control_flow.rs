pub use crate::common::extensions::khr_shader_subgroup_uniform_control_flow::{
    KHR_SHADER_SUBGROUP_UNIFORM_CONTROL_FLOW_EXTENSION_NAME, KHR_SHADER_SUBGROUP_UNIFORM_CONTROL_FLOW_SPEC_VERSION,
};
use crate::{context::Context, vulkan1_0::StructureType};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkPhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR {
    #[doc(alias = "shaderSubgroupUniformControlFlow")]
    pub shader_subgroup_uniform_control_flow: bool,
}
impl PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR {
    ///Get a reference to the `shader_subgroup_uniform_control_flow` field.
    pub fn shader_subgroup_uniform_control_flow(&self) -> &bool {
        &self.shader_subgroup_uniform_control_flow
    }
    ///Get a mutable reference to the `shader_subgroup_uniform_control_flow` field.
    pub fn shader_subgroup_uniform_control_flow_mut(&mut self) -> &mut bool {
        &mut self.shader_subgroup_uniform_control_flow
    }
    ///Sets the `shader_subgroup_uniform_control_flow` field.
    pub fn set_shader_subgroup_uniform_control_flow(
        &mut self,
        shader_subgroup_uniform_control_flow: bool,
    ) -> &mut Self {
        self.shader_subgroup_uniform_control_flow = shader_subgroup_uniform_control_flow;
        self
    }
    ///Sets the `shader_subgroup_uniform_control_flow` field in a builder way.
    pub fn with_shader_subgroup_uniform_control_flow(mut self, shader_subgroup_uniform_control_flow: bool) -> Self {
        self.shader_subgroup_uniform_control_flow = shader_subgroup_uniform_control_flow;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR {
    type LowLevel = crate :: native :: extensions :: khr_shader_subgroup_uniform_control_flow :: PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR ;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate :: native :: extensions :: khr_shader_subgroup_uniform_control_flow :: PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR { s_type : StructureType :: PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKhr , p_next : std :: ptr :: null_mut () , shader_subgroup_uniform_control_flow : self . shader_subgroup_uniform_control_flow . into_low_level (context , bump) }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            shader_subgroup_uniform_control_flow: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_subgroup_uniform_control_flow,
            ),
        }
    }
}
