//!# [VK_KHR_shader_subgroup_uniform_control_flow](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_shader_subgroup_uniform_control_flow.html)
# ! [doc = include_str ! ("../../../../doc/extensions/khr_shader_subgroup_uniform_control_flow/VK_KHR_shader_subgroup_uniform_control_flow.md")]
use crate::{
    cstr,
    vulkan1_0::{BaseOutStructure, Bool32, StructureType},
};
use std::ffi::CStr;
///# [VkPhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_shader_subgroup_uniform_control_flow/VkPhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR.md")]
#[doc(alias = "VkPhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "shaderSubgroupUniformControlFlow")]
    shader_subgroup_uniform_control_flow: Bool32,
}
#[doc(alias = "VK_KHR_SHADER_SUBGROUP_UNIFORM_CONTROL_FLOW_SPEC_VERSION")]
pub const KHR_SHADER_SUBGROUP_UNIFORM_CONTROL_FLOW_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_KHR_SHADER_SUBGROUP_UNIFORM_CONTROL_FLOW_EXTENSION_NAME")]
pub const KHR_SHADER_SUBGROUP_UNIFORM_CONTROL_FLOW_EXTENSION_NAME: &'static CStr =
    cstr!("VK_KHR_shader_subgroup_uniform_control_flow");
