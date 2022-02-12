//![VK_NV_mesh_shader](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_NV_mesh_shader.html) - device extension
//!# Description
//!This extension provides a new mechanism allowing applications to generate
//!collections of geometric primitives via programmable mesh shading.
//!It is an alternative to the existing programmable primitive shading
//!pipeline, which relied on generating input primitives by a fixed function
//!assembler as well as fixed function vertex fetch.There are new programmable shader types — the
//! task and mesh shader — to
//!generate these collections to be processed by fixed-function primitive
//!assembly and rasterization logic.
//!When task and mesh shaders are dispatched, they replace the core
//![pre-rasterization stages](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#pipeline-graphics-subsets-pre-rasterization),
//!including vertex array attribute fetching, vertex shader processing,
//!tessellation, and geometry shader processing.This extension also adds support for the following
//! SPIR-V extension in
//!Vulkan:
//! - [`SPV_NV_mesh_shader`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/NV/SPV_NV_mesh_shader.html)
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_get_physical_device_properties2`]`
//!# Contacts
//! - Christoph Kubisch [pixeljetstream](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_NV_mesh_shader]
//!   @pixeljetstream%0A<<Here describe the issue or question you have about the VK_NV_mesh_shader
//!   extension>>)
//!# New functions & commands
//! - [`CmdDrawMeshTasksIndirectCountNV`]
//! - [`CmdDrawMeshTasksIndirectNV`]
//! - [`CmdDrawMeshTasksNV`]
//!# New structures
//! - [`DrawMeshTasksIndirectCommandNV`]
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:
//! - [`PhysicalDeviceMeshShaderFeaturesNV`]
//!
//! - Extending [`PhysicalDeviceProperties2`]:
//! - [`PhysicalDeviceMeshShaderPropertiesNV`]
//!# New constants
//! - [`NV_MESH_SHADER_EXTENSION_NAME`]
//! - [`NV_MESH_SHADER_SPEC_VERSION`]
//! - Extending [`PipelineStageFlagBits`]:
//! - `VK_PIPELINE_STAGE_MESH_SHADER_BIT_NV`
//! - `VK_PIPELINE_STAGE_TASK_SHADER_BIT_NV`
//!
//! - Extending [`ShaderStageFlagBits`]:
//! - `VK_SHADER_STAGE_MESH_BIT_NV`
//! - `VK_SHADER_STAGE_TASK_BIT_NV`
//!
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MESH_SHADER_FEATURES_NV`
//! - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MESH_SHADER_PROPERTIES_NV`
//!# Known issues & F.A.Q
//!0. How to name this extension?**RESOLVED**: VK_NV_mesh_shaderOther options considered:
//! - VK_NV_mesh_shading
//! - VK_NV_programmable_mesh_shading
//! - VK_NV_primitive_group_shading
//! - VK_NV_grouped_drawing
//!
//!1. Do we need a new VkPrimitiveTopology?**RESOLVED**: No.
//!We skip the InputAssembler stage.
//!2. Should we allow Instancing?**RESOLVED**: No.
//!There is no fixed function input, other than the IDs.
//!However, allow offsetting with a “first” value.
//!3. Should we use existing vkCmdDraw or introduce new functions?**RESOLVED**: Introduce new
//! functions.New functions make it easier to separate from “programmable primitive
//!shading” chapter, less “dual use” language about existing functions
//!having alternative behavior.
//!The text around the existing “draws” is heavily based around emitting
//!vertices.
//!4. If new functions, how to name?**RESOLVED**: CmdDrawMeshTasks*Other options considered:
//! - CmdDrawMeshed
//! - CmdDrawTasked
//! - CmdDrawGrouped
//!
//!5. Should VK_SHADER_STAGE_ALL_GRAPHICS be updated to include the new stages?**RESOLVED**: No.
//!If an application were to be recompiled with headers that include additional
//!shader stage bits in VK_SHADER_STAGE_ALL_GRAPHICS, then the previously valid
//!application would no longer be valid on implementations that do not support
//!mesh or task shaders.
//!This means the change would not be backwards compatible.
//!It is too bad VkShaderStageFlagBits does not have a dedicated “all
//!supported graphics stages” bit like VK_PIPELINE_STAGE_ALL_GRAPHICS_BIT,
//!which would have avoided this problem.
//!# Version History
//! - Revision 1, 2018-07-19 (Christoph Kubisch, Daniel Koch)
//! - Internal revisions
//!# Other info
//! * 2018-07-19
//!*
//! - This extension requires
//![`SPV_NV_mesh_shader`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/NV/SPV_NV_mesh_shader.html)
//! - This extension provides API support for
//![`GLSL_NV_mesh_shader`](https://github.com/KhronosGroup/GLSL/blob/master/extensions/nv/GLSL_NV_mesh_shader.txt)
//!
//!*
//! - Pat Brown, NVIDIA
//! - Jeff Bolz, NVIDIA
//! - Daniel Koch, NVIDIA
//! - Piers Daniell, NVIDIA
//! - Pierre Boudier, NVIDIA
//!# Related
//! - [`DrawMeshTasksIndirectCommandNV`]
//! - [`PhysicalDeviceMeshShaderFeaturesNV`]
//! - [`PhysicalDeviceMeshShaderPropertiesNV`]
//! - [`CmdDrawMeshTasksIndirectCountNV`]
//! - [`CmdDrawMeshTasksIndirectNV`]
//! - [`CmdDrawMeshTasksNV`]
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
#[doc(alias = "VK_NV_MESH_SHADER_SPEC_VERSION")]
pub const NV_MESH_SHADER_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_MESH_SHADER_EXTENSION_NAME")]
pub const NV_MESH_SHADER_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_NV_mesh_shader");
