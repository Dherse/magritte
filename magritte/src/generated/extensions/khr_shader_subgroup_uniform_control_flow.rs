use crate::vulkan1_0::{BaseOutStructure, Bool32, StructureType};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_SHADER_SUBGROUP_UNIFORM_CONTROL_FLOW_SPEC_VERSION")]
pub const KHR_SHADER_SUBGROUP_UNIFORM_CONTROL_FLOW_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_SHADER_SUBGROUP_UNIFORM_CONTROL_FLOW_EXTENSION_NAME")]
pub const KHR_SHADER_SUBGROUP_UNIFORM_CONTROL_FLOW_EXTENSION_NAME: &'static CStr =
    crate::cstr!("VK_KHR_shader_subgroup_uniform_control_flow");
///[VkPhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR.html) - Structure describing support for shader subgroup uniform control flow by an implementation
///# C Specifications
///The [`PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR`]
///structure is defined as:
///```c
///// Provided by VK_KHR_shader_subgroup_uniform_control_flow
///typedef struct VkPhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           shaderSubgroupUniformControlFlow;
///} VkPhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR;
///```
///# Members
///This structure describes the following feature:
///# Description
/// - [`shader_subgroup_uniform_control_flow`] specifies whether the implementation supports the
///   shader execution mode `SubgroupUniformControlFlowKHR`
///If the [`PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR`] structure is included in
/// the [`p_next`] chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR`]**can** also be used in the
/// [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage (Implicit)
/// - [`s_type`]**must** be
///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_SUBGROUP_UNIFORM_CONTROL_FLOW_FEATURES_KHR`
///# Related
/// - [`VK_KHR_shader_subgroup_uniform_control_flow`]
/// - [`Bool32`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`]**must** be
    /// `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_SUBGROUP_UNIFORM_CONTROL_FLOW_FEATURES_KHR`
    s_type: StructureType,
    ///No documentation found
    p_next: *const BaseOutStructure<'lt>,
    ///[`shader_subgroup_uniform_control_flow`] specifies whether the
    ///implementation supports the shader execution mode
    ///`SubgroupUniformControlFlowKHR`
    shader_subgroup_uniform_control_flow: Bool32,
}
