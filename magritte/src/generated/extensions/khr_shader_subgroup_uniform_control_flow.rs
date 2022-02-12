//![VK_KHR_shader_subgroup_uniform_control_flow](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_shader_subgroup_uniform_control_flow.html) - device extension
//!# Description
//!This extension allows the use of the `SPV_KHR_subgroup_uniform_control_flow`
//!SPIR-V extension in shader modules.
//!`SPV_KHR_subgroup_uniform_control_flow` provides stronger guarantees that
//!diverged subgroups will reconverge.Developers should utilize this extension if they use subgroup
//! operations to
//!reduce the work performed by a uniform subgroup.
//!This extension will guarantee that uniform subgroup will reconverge in the
//!same manner as invocation groups (see “Uniform Control Flow” in the
//![Khronos SPIR-V Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#spirv-spec)).
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.1
//!# Contacts
//! - Alan Baker [alan-baker](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_shader_subgroup_uniform_control_flow]
//!   @alan-baker%0A<<Here describe the issue or question you have about the
//!   VK_KHR_shader_subgroup_uniform_control_flow extension>>)
//!# New structures
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:
//! - [`PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR`]
//!# New constants
//! - [`KHR_SHADER_SUBGROUP_UNIFORM_CONTROL_FLOW_EXTENSION_NAME`]
//! - [`KHR_SHADER_SUBGROUP_UNIFORM_CONTROL_FLOW_SPEC_VERSION`]
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_SUBGROUP_UNIFORM_CONTROL_FLOW_FEATURES_KHR`
//!# Version History
//! - Revision 1, 2020-08-27 (Alan Baker)
//! - Internal draft version
//!# Other info
//! * 2020-08-27
//! * No known IP claims.
//!*
//! - Requires SPIR-V 1.3.
//! - This extension requires
//![`SPV_KHR_subgroup_uniform_control_flow`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/KHR/SPV_KHR_subgroup_uniform_control_flow.html)
//! - This extension provides API support for
//![`GL_EXT_subgroupuniform_qualifier`](https://github.com/KhronosGroup/GLSL/blob/master/extensions/ext/GL_EXT_subgroupuniform_qualifier.txt)
//!*
//! - Alan Baker, Google
//! - Jeff Bolz, NVIDIA
//!# Related
//! - [`PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use std::ffi::CStr;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_SHADER_SUBGROUP_UNIFORM_CONTROL_FLOW_SPEC_VERSION")]
pub const KHR_SHADER_SUBGROUP_UNIFORM_CONTROL_FLOW_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_SHADER_SUBGROUP_UNIFORM_CONTROL_FLOW_EXTENSION_NAME")]
pub const KHR_SHADER_SUBGROUP_UNIFORM_CONTROL_FLOW_EXTENSION_NAME: &'static CStr =
    crate::cstr!("VK_KHR_shader_subgroup_uniform_control_flow");
