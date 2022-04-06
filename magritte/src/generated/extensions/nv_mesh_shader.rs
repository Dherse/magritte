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
//! - [`cmd_draw_mesh_tasks_indirect_count_nv`]
//! - [`cmd_draw_mesh_tasks_indirect_nv`]
//! - [`cmd_draw_mesh_tasks_nv`]
//!# New structures
//! - [`DrawMeshTasksIndirectCommandNV`]
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  -
//!   [`PhysicalDeviceMeshShaderFeaturesNV`]
//! - Extending [`PhysicalDeviceProperties2`]:  - [`PhysicalDeviceMeshShaderPropertiesNV`]
//!# New constants
//! - [`NV_MESH_SHADER_EXTENSION_NAME`]
//! - [`NV_MESH_SHADER_SPEC_VERSION`]
//! - Extending [`PipelineStageFlagBits`]:  - `VK_PIPELINE_STAGE_MESH_SHADER_BIT_NV`  -
//!   `VK_PIPELINE_STAGE_TASK_SHADER_BIT_NV`
//! - Extending [`ShaderStageFlagBits`]:  - `VK_SHADER_STAGE_MESH_BIT_NV`  -
//!   `VK_SHADER_STAGE_TASK_BIT_NV`
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MESH_SHADER_FEATURES_NV`  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MESH_SHADER_PROPERTIES_NV`
//!# Known issues & F.A.Q
//!0. How to name this extension? **RESOLVED** : VK_NV_mesh_shaderOther options considered:  -
//! VK_NV_mesh_shading  - VK_NV_programmable_mesh_shading  - VK_NV_primitive_group_shading  -
//! VK_NV_grouped_drawing
//!1. Do we need a new VkPrimitiveTopology? **RESOLVED** : No. We skip the InputAssembler stage.
//!2. Should we allow Instancing? **RESOLVED** : No. There is no fixed function input, other than
//! the IDs. However, allow offsetting with a “first” value.
//!3. Should we use existing vkCmdDraw or introduce new functions? **RESOLVED** : Introduce new
//! functions.New functions make it easier to separate from “programmable primitive shading”
//! chapter, less “dual use” language about existing functions having alternative behavior. The text
//! around the existing “draws” is heavily based around emitting vertices.
//!4. If new functions, how to name? **RESOLVED** : CmdDrawMeshTasks*Other options considered:  -
//! CmdDrawMeshed  - CmdDrawTasked  - CmdDrawGrouped
//!5. Should VK_SHADER_STAGE_ALL_GRAPHICS be updated to include the new stages? **RESOLVED** : No.
//! If an application were to be recompiled with headers that include additional shader stage bits
//! in VK_SHADER_STAGE_ALL_GRAPHICS, then the previously valid application would no longer be valid
//! on implementations that do not support mesh or task shaders. This means the change would not be
//! backwards compatible. It is too bad VkShaderStageFlagBits does not have a dedicated “all
//! supported graphics stages” bit like VK_PIPELINE_STAGE_ALL_GRAPHICS_BIT, which would have avoided
//! this problem.
//!# Version History
//! - Revision 1, 2018-07-19 (Christoph Kubisch, Daniel Koch)  - Internal revisions
//!# Other info
//! * 2018-07-19
//! * - This extension requires [`SPV_NV_mesh_shader`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/NV/SPV_NV_mesh_shader.html)
//!   - This extension provides API support for [`GLSL_NV_mesh_shader`](https://github.com/KhronosGroup/GLSL/blob/master/extensions/nv/GLSL_NV_mesh_shader.txt)
//! * - Pat Brown, NVIDIA  - Jeff Bolz, NVIDIA  - Daniel Koch, NVIDIA  - Piers Daniell, NVIDIA  -
//!   Pierre Boudier, NVIDIA
//!# Related
//! - [`DrawMeshTasksIndirectCommandNV`]
//! - [`PhysicalDeviceMeshShaderFeaturesNV`]
//! - [`PhysicalDeviceMeshShaderPropertiesNV`]
//! - [`cmd_draw_mesh_tasks_indirect_count_nv`]
//! - [`cmd_draw_mesh_tasks_indirect_nv`]
//! - [`cmd_draw_mesh_tasks_nv`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::{
    vulkan1_0::{BaseOutStructure, Bool32, Buffer, CommandBuffer, Device, DeviceSize, StructureType},
    AsRaw, Unique,
};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_MESH_SHADER_SPEC_VERSION")]
pub const NV_MESH_SHADER_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_MESH_SHADER_EXTENSION_NAME")]
pub const NV_MESH_SHADER_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_NV_mesh_shader");
///[vkCmdDrawMeshTasksNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawMeshTasksNV.html) - Draw mesh task work items
///# C Specifications
///To record a draw that uses the mesh pipeline, call:
///```c
///// Provided by VK_NV_mesh_shader
///void vkCmdDrawMeshTasksNV(
///    VkCommandBuffer                             commandBuffer,
///    uint32_t                                    taskCount,
///    uint32_t                                    firstTask);
///```
///# Parameters
/// - [`command_buffer`] is the command buffer into which the command will be recorded.
/// - [`task_count`] is the number of local workgroups to dispatch in the X dimension. Y and Z
///   dimension are implicitly set to one.
/// - [`first_task`] is the X component of the first workgroup ID.
///# Description
///When the command is executed, a global workgroup consisting of
///[`task_count`] local workgroups is assembled.
///## Valid Usage
/// - If a [`Sampler`] created with `magFilter` or `minFilter` equal to `VK_FILTER_LINEAR` and
///   `compareEnable` equal to [`FALSE`] is used to sample a [`ImageView`] as a result of this
///   command, then the image view’s [format features]() **must**  contain
///   `VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT`
/// - If a [`Sampler`] created with `mipmapMode` equal to `VK_SAMPLER_MIPMAP_MODE_LINEAR` and
///   `compareEnable` equal to [`FALSE`] is used to sample a [`ImageView`] as a result of this
///   command, then the image view’s [format features]() **must**  contain
///   `VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT`
/// - If a [`ImageView`] is sampled with [depth comparison](), the image view’s [format features]()
///   **must**  contain `VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_DEPTH_COMPARISON_BIT`
/// - If a [`ImageView`] is accessed using atomic operations as a result of this command, then the
///   image view’s [format features]() **must**  contain
///   `VK_FORMAT_FEATURE_STORAGE_IMAGE_ATOMIC_BIT`
/// - If a [`ImageView`] is sampled with `VK_FILTER_CUBIC_EXT` as a result of this command, then the
///   image view’s [format features]() **must**  contain
///   `VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_CUBIC_BIT_EXT`
/// - Any [`ImageView`] being sampled with `VK_FILTER_CUBIC_EXT` as a result of this command
///   **must**  have a [`ImageViewType`] and format that supports cubic filtering, as specified by
///   [`FilterCubicImageViewImageFormatPropertiesEXT::filter_cubic`] returned by
///   [`get_physical_device_image_format_properties2`]
/// - Any [`ImageView`] being sampled with `VK_FILTER_CUBIC_EXT` with a reduction mode of either
///   `VK_SAMPLER_REDUCTION_MODE_MIN` or `VK_SAMPLER_REDUCTION_MODE_MAX` as a result of this command
///   **must**  have a [`ImageViewType`] and format that supports cubic filtering together with
///   minmax filtering, as specified by
///   [`FilterCubicImageViewImageFormatPropertiesEXT::filter_cubic_minmax`] returned by
///   [`get_physical_device_image_format_properties2`]
/// - Any [`Image`] created with a [`ImageCreateInfo::flags`] containing
///   `VK_IMAGE_CREATE_CORNER_SAMPLED_BIT_NV` sampled as a result of this command  **must**  only be
///   sampled using a [`SamplerAddressMode`] of `VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE`
/// - Any [`ImageView`] or [`BufferView`] being written as a storage image or storage texel buffer
///   where the image format field of the `OpTypeImage` is `Unknown` then the view’s format feature
///   **must**  contain `VK_FORMAT_FEATURE_2_STORAGE_WRITE_WITHOUT_FORMAT_BIT`
/// - Any [`ImageView`] or [`BufferView`] being read as a storage image or storage texel buffer
///   where the image format field of the `OpTypeImage` is `Unknown` then the view’s format feature
///   **must**  contain `VK_FORMAT_FEATURE_2_STORAGE_READ_WITHOUT_FORMAT_BIT`
/// - For each set *n* that is statically used by the [`Pipeline`] bound to the pipeline bind point
///   used by this command, a descriptor set  **must**  have been bound to *n* at the same pipeline
///   bind point, with a [`PipelineLayout`] that is compatible for set *n*, with the
///   [`PipelineLayout`] used to create the current [`Pipeline`], as described in
///   [[descriptorsets-compatibility]]()
/// - If the [`maintenance4`]() feature is not enabled, then for each push constant that is
///   statically used by the [`Pipeline`] bound to the pipeline bind point used by this command, a
///   push constant value  **must**  have been set for the same pipeline bind point, with a
///   [`PipelineLayout`] that is compatible for push constants, with the [`PipelineLayout`] used to
///   create the current [`Pipeline`], as described in [[descriptorsets-compatibility]]()
/// - Descriptors in each bound descriptor set, specified via [`cmd_bind_descriptor_sets`],
///   **must**  be valid if they are statically used by the [`Pipeline`] bound to the pipeline bind
///   point used by this command
/// - A valid pipeline  **must**  be bound to the pipeline bind point used by this command
/// - If the [`Pipeline`] object bound to the pipeline bind point used by this command requires any
///   dynamic state, that state  **must**  have been set or inherited (if the
///   `[`VK_NV_inherited_viewport_scissor`]` extension is enabled) for [`command_buffer`], and done
///   so after any previously bound pipeline with the corresponding state not specified as dynamic
/// - There  **must**  not have been any calls to dynamic state setting commands for any state not
///   specified as dynamic in the [`Pipeline`] object bound to the pipeline bind point used by this
///   command, since that pipeline was bound
/// - If the [`Pipeline`] object bound to the pipeline bind point used by this command accesses a
///   [`Sampler`] object that uses unnormalized coordinates, that sampler  **must**  not be used to
///   sample from any [`Image`] with a [`ImageView`] of the type `VK_IMAGE_VIEW_TYPE_3D`,
///   `VK_IMAGE_VIEW_TYPE_CUBE`, `VK_IMAGE_VIEW_TYPE_1D_ARRAY`, `VK_IMAGE_VIEW_TYPE_2D_ARRAY` or
///   `VK_IMAGE_VIEW_TYPE_CUBE_ARRAY`, in any shader stage
/// - If the [`Pipeline`] object bound to the pipeline bind point used by this command accesses a
///   [`Sampler`] object that uses unnormalized coordinates, that sampler  **must**  not be used
///   with any of the SPIR-V `OpImageSample*` or `OpImageSparseSample*` instructions with
///   `ImplicitLod`, `Dref` or `Proj` in their name, in any shader stage
/// - If the [`Pipeline`] object bound to the pipeline bind point used by this command accesses a
///   [`Sampler`] object that uses unnormalized coordinates, that sampler  **must**  not be used
///   with any of the SPIR-V `OpImageSample*` or `OpImageSparseSample*` instructions that includes a
///   LOD bias or any offset values, in any shader stage
/// - If the [robust buffer access]() feature is not enabled, and if the [`Pipeline`] object bound
///   to the pipeline bind point used by this command accesses a uniform buffer, it  **must**  not
///   access values outside of the range of the buffer as specified in the descriptor set bound to
///   the same pipeline bind point
/// - If the [robust buffer access]() feature is not enabled, and if the [`Pipeline`] object bound
///   to the pipeline bind point used by this command accesses a storage buffer, it  **must**  not
///   access values outside of the range of the buffer as specified in the descriptor set bound to
///   the same pipeline bind point
/// - If [`command_buffer`] is an unprotected command buffer and [`protectedNoFault`]() is not
///   supported, any resource accessed by the [`Pipeline`] object bound to the pipeline bind point
///   used by this command  **must**  not be a protected resource
/// - If the [`Pipeline`] object bound to the pipeline bind point used by this command accesses a
///   [`Sampler`] or [`ImageView`] object that enables [sampler Y′C<sub>B</sub>C<sub>R</sub>
///   conversion](), that object  **must**  only be used with `OpImageSample*` or
///   `OpImageSparseSample*` instructions
/// - If the [`Pipeline`] object bound to the pipeline bind point used by this command accesses a
///   [`Sampler`] or [`ImageView`] object that enables [sampler Y′C<sub>B</sub>C<sub>R</sub>
///   conversion](), that object  **must**  not use the `ConstOffset` and `Offset` operands
/// - If a [`ImageView`] is accessed using `OpImageWrite` as a result of this command, then the
///   `Type` of the `Texel` operand of that instruction  **must**  have at least as many components
///   as the image view’s format
/// - If a [`BufferView`] is accessed using `OpImageWrite` as a result of this command, then the
///   `Type` of the `Texel` operand of that instruction  **must**  have at least as many components
///   as the buffer view’s format
/// - If a [`ImageView`] with a [`Format`] that has a 64-bit component width is accessed as a result
///   of this command, the `SampledType` of the `OpTypeImage` operand of that instruction  **must**
///   have a `Width` of 64
/// - If a [`ImageView`] with a [`Format`] that has a component width less than 64-bit is accessed
///   as a result of this command, the `SampledType` of the `OpTypeImage` operand of that
///   instruction  **must**  have a `Width` of 32
/// - If a [`BufferView`] with a [`Format`] that has a 64-bit component width is accessed as a
///   result of this command, the `SampledType` of the `OpTypeImage` operand of that instruction
///   **must**  have a `Width` of 64
/// - If a [`BufferView`] with a [`Format`] that has a component width less than 64-bit is accessed
///   as a result of this command, the `SampledType` of the `OpTypeImage` operand of that
///   instruction  **must**  have a `Width` of 32
/// - If the [`sparseImageInt64Atomics`]() feature is not enabled, [`Image`] objects created with
///   the `VK_IMAGE_CREATE_SPARSE_RESIDENCY_BIT` flag  **must**  not be accessed by atomic
///   instructions through an `OpTypeImage` with a `SampledType` with a `Width` of 64 by this
///   command
/// - If the [`sparseImageInt64Atomics`]() feature is not enabled, [`Buffer`] objects created with
///   the `VK_BUFFER_CREATE_SPARSE_RESIDENCY_BIT` flag  **must**  not be accessed by atomic
///   instructions through an `OpTypeImage` with a `SampledType` with a `Width` of 64 by this
///   command
/// - The current render pass  **must**  be [compatible]() with the `renderPass` member of the
///   [`GraphicsPipelineCreateInfo`] structure specified when creating the [`Pipeline`] bound to
///   `VK_PIPELINE_BIND_POINT_GRAPHICS`
/// - The subpass index of the current render pass  **must**  be equal to the `subpass` member of
///   the [`GraphicsPipelineCreateInfo`] structure specified when creating the [`Pipeline`] bound to
///   `VK_PIPELINE_BIND_POINT_GRAPHICS`
/// - Every input attachment used by the current subpass  **must**  be bound to the pipeline via a
///   descriptor set
/// - Memory backing image subresources used as attachments in the current render pass  **must**
///   not be written in any way other than as an attachment by this command
/// - If any recorded command in the current subpass will write to an image subresource as an
///   attachment, this command  **must**  not read from the memory backing that image subresource in
///   any other way than as an attachment
/// - If any recorded command in the current subpass will read from an image subresource used as an
///   attachment in any way other than as an attachment, this command  **must**  not write to that
///   image subresource as an attachment
/// - If the draw is recorded in a render pass instance with multiview enabled, the maximum instance
///   index  **must**  be less than or equal to
///   [`PhysicalDeviceMultiviewProperties::max_multiview_instance_index`]
/// - If the bound graphics pipeline was created with
///   [`PipelineSampleLocationsStateCreateInfoEXT::sample_locations_enable`] set to [`TRUE`] and the
///   current subpass has a depth/stencil attachment, then that attachment  **must**  have been
///   created with the `VK_IMAGE_CREATE_SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_BIT_EXT` bit set
/// - If the bound graphics pipeline state was created with the
///   `VK_DYNAMIC_STATE_SAMPLE_LOCATIONS_EXT` dynamic state enabled then
///   [`cmd_set_sample_locations_ext`] **must**  have been called in the current command buffer
///   prior to this drawing command
/// - If the bound graphics pipeline state was created with the
///   `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` dynamic state enabled, but not the
///   `VK_DYNAMIC_STATE_SCISSOR_WITH_COUNT` dynamic state enabled, then
///   [`cmd_set_viewport_with_count`] **must**  have been called in the current command buffer prior
///   to this drawing command, and the `viewportCount` parameter of [`cmd_set_viewport_with_count`]
///   **must**  match the [`PipelineViewportStateCreateInfo::scissor_count`] of the pipeline
/// - If the bound graphics pipeline state was created with the
///   `VK_DYNAMIC_STATE_SCISSOR_WITH_COUNT` dynamic state enabled, but not the
///   `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` dynamic state enabled, then
///   [`cmd_set_scissor_with_count`] **must**  have been called in the current command buffer prior
///   to this drawing command, and the `scissorCount` parameter of [`cmd_set_scissor_with_count`]
///   **must**  match the [`PipelineViewportStateCreateInfo::viewport_count`] of the pipeline
/// - If the bound graphics pipeline state was created with both the
///   `VK_DYNAMIC_STATE_SCISSOR_WITH_COUNT` and `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` dynamic
///   states enabled then both [`cmd_set_viewport_with_count`] and [`cmd_set_scissor_with_count`]
///   **must**  have been called in the current command buffer prior to this drawing command, and
///   the `viewportCount` parameter of [`cmd_set_viewport_with_count`] **must**  match the
///   `scissorCount` parameter of [`cmd_set_scissor_with_count`]
/// - If the bound graphics pipeline state was created with the
///   `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` dynamic state enabled, but not the
///   `VK_DYNAMIC_STATE_VIEWPORT_W_SCALING_NV` dynamic state enabled, then the bound graphics
///   pipeline  **must**  have been created with
///   [`PipelineViewportWScalingStateCreateInfoNV::viewport_count`] greater or equal to the
///   `viewportCount` parameter in the last call to [`cmd_set_viewport_with_count`]
/// - If the bound graphics pipeline state was created with the
///   `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` and `VK_DYNAMIC_STATE_VIEWPORT_W_SCALING_NV` dynamic
///   states enabled then the `viewportCount` parameter in the last call to
///   [`cmd_set_viewport_w_scaling_nv`] **must**  be greater than or equal to the `viewportCount`
///   parameter in the last call to [`cmd_set_viewport_with_count`]
/// - If the bound graphics pipeline state was created with the
///   `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` dynamic state enabled, but not the
///   `VK_DYNAMIC_STATE_VIEWPORT_SHADING_RATE_PALETTE_NV` dynamic state enabled, then the bound
///   graphics pipeline  **must**  have been created with
///   [`PipelineViewportShadingRateImageStateCreateInfoNV::viewport_count`] greater or equal to the
///   `viewportCount` parameter in the last call to [`cmd_set_viewport_with_count`]
/// - If the bound graphics pipeline state was created with the
///   `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` and `VK_DYNAMIC_STATE_VIEWPORT_SHADING_RATE_PALETTE_NV`
///   dynamic states enabled then the `viewportCount` parameter in the last call to
///   [`cmd_set_viewport_shading_rate_palette_nv`] **must**  be greater than or equal to the
///   `viewportCount` parameter in the last call to [`cmd_set_viewport_with_count`]
/// - If the bound graphics pipeline state was created with the
///   `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` dynamic state enabled and a
///   [`PipelineViewportSwizzleStateCreateInfoNV`] structure chained from
///   `VkPipelineVieportCreateInfo`, then the bound graphics pipeline  **must**  have been created
///   with [`PipelineViewportSwizzleStateCreateInfoNV::viewport_count`] greater or equal to the
///   `viewportCount` parameter in the last call to [`cmd_set_viewport_with_count`]
/// - If the bound graphics pipeline state was created with the
///   `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` dynamic state enabled and a
///   [`PipelineViewportExclusiveScissorStateCreateInfoNV`] structure chained from
///   `VkPipelineVieportCreateInfo`, then the bound graphics pipeline  **must**  have been created
///   with [`PipelineViewportExclusiveScissorStateCreateInfoNV::exclusive_scissor_count`] greater or
///   equal to the `viewportCount` parameter in the last call to [`cmd_set_viewport_with_count`]
/// - If the bound graphics pipeline state was created with the
///   `VK_DYNAMIC_STATE_RASTERIZER_DISCARD_ENABLE` dynamic state enabled then
///   [`cmd_set_rasterizer_discard_enable`] **must**  have been called in the current command buffer
///   prior to this drawing command
/// - If the bound graphics pipeline state was created with the `VK_DYNAMIC_STATE_DEPTH_BIAS_ENABLE`
///   dynamic state enabled then [`cmd_set_depth_bias_enable`] **must**  have been called in the
///   current command buffer prior to this drawing command
/// - If the bound graphics pipeline state was created with the `VK_DYNAMIC_STATE_LOGIC_OP_EXT`
///   dynamic state enabled then [`cmd_set_logic_op_ext`] **must**  have been called in the current
///   command buffer prior to this drawing command and the `logicOp` **must**  be a valid
///   [`LogicOp`] value
/// - If the [`primitiveFragmentShadingRateWithMultipleViewports`]() limit is not supported, the
///   bound graphics pipeline was created with the `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` dynamic
///   state enabled, and any of the shader stages of the bound graphics pipeline write to the
///   `PrimitiveShadingRateKHR` built-in, then [`cmd_set_viewport_with_count`] **must**  have been
///   called in the current command buffer prior to this drawing command, and the `viewportCount`
///   parameter of [`cmd_set_viewport_with_count`] **must**  be `1`
/// - If rasterization is not disabled in the bound graphics pipeline, then for each color
///   attachment in the subpass, if the corresponding image view’s [format features]() do not
///   contain `VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BLEND_BIT`, then the `blendEnable` member of the
///   corresponding element of the `pAttachments` member of `pColorBlendState` **must**  be
///   [`FALSE`]
/// - If rasterization is not disabled in the bound graphics pipeline, and neither the
///   `[`VK_AMD_mixed_attachment_samples`]` nor the `[`VK_NV_framebuffer_mixed_samples`]` extensions
///   are enabled, then [`PipelineMultisampleStateCreateInfo::rasterization_samples`] **must**  be
///   the same as the current subpass color and/or depth/stencil attachments
/// - If the current render pass instance was begun with [`cmd_begin_rendering`], the `imageView`
///   member of `pDepthAttachment` is not [`crate::Handle::null`], and the `layout` member of
///   `pDepthAttachment` is `VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL`, this command
///   **must**  not write any values to the depth attachment
/// - If the current render pass instance was begun with [`cmd_begin_rendering`], the `imageView`
///   member of `pStencilAttachment` is not [`crate::Handle::null`], and the `layout` member of
///   `pStencilAttachment` is `VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL`, this command
///   **must**  not write any values to the stencil attachment
/// - If the current render pass instance was begun with [`cmd_begin_rendering`], the `imageView`
///   member of `pDepthAttachment` is not [`crate::Handle::null`], and the `layout` member of
///   `pDepthAttachment` is `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL`, this
///   command  **must**  not write any values to the depth attachment
/// - If the current render pass instance was begun with [`cmd_begin_rendering`], the `imageView`
///   member of `pStencilAttachment` is not [`crate::Handle::null`], and the `layout` member of
///   `pStencilAttachment` is `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL`, this
///   command  **must**  not write any values to the stencil attachment
/// - If the current render pass instance was begun with [`cmd_begin_rendering`], the `imageView`
///   member of `pDepthAttachment` is not [`crate::Handle::null`], and the `layout` member of
///   `pDepthAttachment` is `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL`, this command  **must**  not
///   write any values to the depth attachment
/// - If the current render pass instance was begun with [`cmd_begin_rendering`], the `imageView`
///   member of `pStencilAttachment` is not [`crate::Handle::null`], and the `layout` member of
///   `pStencilAttachment` is `VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL`, this command  **must**
///   not write any values to the stencil attachment
/// - If the current render pass instance was begun with [`cmd_begin_rendering`], the currently
///   bound graphics pipeline  **must**  have been created with a
///   [`PipelineRenderingCreateInfo::view_mask`] equal to [`RenderingInfo::view_mask`]
/// - If the current render pass instance was begun with [`cmd_begin_rendering`], the currently
///   bound graphics pipeline  **must**  have been created with a
///   [`PipelineRenderingCreateInfo::color_attachment_count`] equal to
///   [`RenderingInfo::color_attachment_count`]
/// - If the current render pass instance was begun with [`cmd_begin_rendering`] and
///   [`RenderingInfo::color_attachment_count`] greater than `0`, then each element of the
///   [`RenderingInfo::color_attachments`] array with a `imageView` not equal to
///   [`crate::Handle::null`] **must**  have been created with a [`Format`] equal to the
///   corresponding element of [`PipelineRenderingCreateInfo::color_attachment_formats`] used to
///   create the currently bound graphics pipeline
/// - If the bound graphics pipeline state was created with the
///   `VK_DYNAMIC_STATE_COLOR_WRITE_ENABLE_EXT` dynamic state enabled then
///   [`cmd_set_color_write_enable_ext`] **must**  have been called in the current command buffer
///   prior to this drawing command, and the `attachmentCount` parameter of
///   [`cmd_set_color_write_enable_ext`] **must**  be equal to the
///   [`PipelineColorBlendStateCreateInfo::attachment_count`] of the currently bound graphics
///   pipeline
/// - If the current render pass instance was begun with [`cmd_begin_rendering`] and
///   [`RenderingInfo`]::`pDepthAttachment->pname`:imageView was not [`crate::Handle::null`], the
///   value of [`PipelineRenderingCreateInfo::depth_attachment_format`] used to create the currently
///   bound graphics pipeline  **must**  be equal to the [`Format`] used to create
///   [`RenderingInfo`]::`pDepthAttachment->pname`:imageView
/// - If the current render pass instance was begun with [`cmd_begin_rendering`] and
///   [`RenderingInfo`]::`pStencilAttachment->pname`:imageView was not [`crate::Handle::null`], the
///   value of [`PipelineRenderingCreateInfo::stencil_attachment_format`] used to create the
///   currently bound graphics pipeline  **must**  be equal to the [`Format`] used to create
///   [`RenderingInfo`]::`pStencilAttachment->pname`:imageView
/// - If the current render pass instance was begun with [`cmd_begin_rendering`] and
///   [`RenderingFragmentShadingRateAttachmentInfoKHR::image_view`] was not [`crate::Handle::null`],
///   the currently bound graphics pipeline  **must**  have been created with
///   `VK_PIPELINE_CREATE_RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR`
/// - If the current render pass instance was begun with [`cmd_begin_rendering`] and
///   [`RenderingFragmentDensityMapAttachmentInfoEXT::image_view`] was not [`crate::Handle::null`],
///   the currently bound graphics pipeline  **must**  have been created with
///   `VK_PIPELINE_CREATE_RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_BIT_EXT`
/// - If the currently bound pipeline was created with a [`AttachmentSampleCountInfoAMD`] or
///   [`AttachmentSampleCountInfoNV`] structure, and the current render pass instance was begun with
///   [`cmd_begin_rendering`] with a [`RenderingInfo::color_attachment_count`] parameter greater
///   than `0`, then each element of the [`RenderingInfo::color_attachments`] array with a
///   `imageView` not equal to [`crate::Handle::null`] **must**  have been created with a sample
///   count equal to the corresponding element of the `pColorAttachmentSamples` member of
///   [`AttachmentSampleCountInfoAMD`] or [`AttachmentSampleCountInfoNV`] used to create the
///   currently bound graphics pipeline
/// - If the current render pass instance was begun with [`cmd_begin_rendering`], the currently
///   bound pipeline was created with a [`AttachmentSampleCountInfoAMD`] or
///   [`AttachmentSampleCountInfoNV`] structure, and
///   [`RenderingInfo`]::`pDepthAttachment->pname`:imageView was not [`crate::Handle::null`], the
///   value of the `depthStencilAttachmentSamples` member of [`AttachmentSampleCountInfoAMD`] or
///   [`AttachmentSampleCountInfoNV`] used to create the currently bound graphics pipeline  **must**
///   be equal to the sample count used to create
///   [`RenderingInfo`]::`pDepthAttachment->pname`:imageView
/// - If the current render pass instance was begun with [`cmd_begin_rendering`], the currently
///   bound pipeline was created with a [`AttachmentSampleCountInfoAMD`] or
///   [`AttachmentSampleCountInfoNV`] structure, and
///   [`RenderingInfo`]::`pStencilAttachment->pname`:imageView was not [`crate::Handle::null`], the
///   value of the `depthStencilAttachmentSamples` member of [`AttachmentSampleCountInfoAMD`] or
///   [`AttachmentSampleCountInfoNV`] used to create the currently bound graphics pipeline  **must**
///   be equal to the sample count used to create
///   [`RenderingInfo`]::`pStencilAttachment->pname`:imageView
/// - If the currently bound pipeline was created without a [`AttachmentSampleCountInfoAMD`] or
///   [`AttachmentSampleCountInfoNV`] structure, and the current render pass instance was begun with
///   [`cmd_begin_rendering`] with a [`RenderingInfo::color_attachment_count`] parameter greater
///   than `0`, then each element of the [`RenderingInfo::color_attachments`] array with a
///   `imageView` not equal to [`crate::Handle::null`] **must**  have been created with a sample
///   count equal to the value of [`PipelineMultisampleStateCreateInfo::rasterization_samples`] used
///   to create the currently bound graphics pipeline
/// - If the current render pass instance was begun with [`cmd_begin_rendering`], the currently
///   bound pipeline was created without a [`AttachmentSampleCountInfoAMD`] or
///   [`AttachmentSampleCountInfoNV`] structure, and
///   [`RenderingInfo`]::`pDepthAttachment->pname`:imageView was not [`crate::Handle::null`], the
///   value of [`PipelineMultisampleStateCreateInfo::rasterization_samples`] used to create the
///   currently bound graphics pipeline  **must**  be equal to the sample count used to create
///   [`RenderingInfo`]::`pDepthAttachment->pname`:imageView
/// - If the current render pass instance was begun with [`cmd_begin_rendering`], the currently
///   bound pipeline was created without a [`AttachmentSampleCountInfoAMD`] or
///   [`AttachmentSampleCountInfoNV`] structure, and
///   [`RenderingInfo`]::`pStencilAttachment->pname`:imageView was not [`crate::Handle::null`], the
///   value of [`PipelineMultisampleStateCreateInfo::rasterization_samples`] used to create the
///   currently bound graphics pipeline  **must**  be equal to the sample count used to create
///   [`RenderingInfo`]::`pStencilAttachment->pname`:imageView
/// - If the current render pass instance was begun with [`cmd_begin_rendering`], the currently
///   bound pipeline  **must**  have been created with a [`GraphicsPipelineCreateInfo::render_pass`]
///   equal to [`crate::Handle::null`]
///
/// - The bound graphics pipeline  **must**  not have been created with the
///   [`PipelineShaderStageCreateInfo::stage`] member of an element of
///   [`GraphicsPipelineCreateInfo::stages`] set to `VK_SHADER_STAGE_VERTEX_BIT`,
///   `VK_SHADER_STAGE_TESSELLATION_CONTROL_BIT`, `VK_SHADER_STAGE_TESSELLATION_EVALUATION_BIT` or
///   `VK_SHADER_STAGE_GEOMETRY_BIT`
/// - [`task_count`] **must**  be less than or equal to
///   [`PhysicalDeviceMeshShaderPropertiesNV::max_draw_mesh_tasks_count`]
///
///## Valid Usage (Implicit)
/// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
/// - [`command_buffer`] **must**  be in the [recording state]()
/// - The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics
///   operations
/// - This command  **must**  only be called inside of a render pass instance
///
///## Host Synchronization
/// - Host access to [`command_buffer`] **must**  be externally synchronized
/// - Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be
///   externally synchronized
///
///## Command Properties
///# Related
/// - [`VK_NV_mesh_shader`]
/// - [`CommandBuffer`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkCmdDrawMeshTasksNV")]
pub type FNCmdDrawMeshTasksNv =
    Option<unsafe extern "system" fn(command_buffer: CommandBuffer, task_count: u32, first_task: u32)>;
///[vkCmdDrawMeshTasksIndirectNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawMeshTasksIndirectNV.html) - Issue an indirect mesh tasks draw into a command buffer
///# C Specifications
///To record an indirect mesh tasks draw, call:
///```c
///// Provided by VK_NV_mesh_shader
///void vkCmdDrawMeshTasksIndirectNV(
///    VkCommandBuffer                             commandBuffer,
///    VkBuffer                                    buffer,
///    VkDeviceSize                                offset,
///    uint32_t                                    drawCount,
///    uint32_t                                    stride);
///```
///# Parameters
/// - [`command_buffer`] is the command buffer into which the command is recorded.
/// - [`buffer`] is the buffer containing draw parameters.
/// - [`offset`] is the byte offset into [`buffer`] where parameters begin.
/// - [`draw_count`] is the number of draws to execute, and  **can**  be zero.
/// - [`stride`] is the byte stride between successive sets of draw parameters.
///# Description
///[`cmd_draw_mesh_tasks_indirect_nv`] behaves similarly to
///[`cmd_draw_mesh_tasks_nv`] except that the parameters are read by the device
///from a buffer during execution.
///[`draw_count`] draws are executed by the command, with parameters taken
///from [`buffer`] starting at [`offset`] and increasing by [`stride`]
///bytes for each successive draw.
///The parameters of each draw are encoded in an array of
///[`DrawMeshTasksIndirectCommandNV`] structures.
///If [`draw_count`] is less than or equal to one, [`stride`] is ignored.
///## Valid Usage
/// - If a [`Sampler`] created with `magFilter` or `minFilter` equal to `VK_FILTER_LINEAR` and
///   `compareEnable` equal to [`FALSE`] is used to sample a [`ImageView`] as a result of this
///   command, then the image view’s [format features]() **must**  contain
///   `VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT`
/// - If a [`Sampler`] created with `mipmapMode` equal to `VK_SAMPLER_MIPMAP_MODE_LINEAR` and
///   `compareEnable` equal to [`FALSE`] is used to sample a [`ImageView`] as a result of this
///   command, then the image view’s [format features]() **must**  contain
///   `VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT`
/// - If a [`ImageView`] is sampled with [depth comparison](), the image view’s [format features]()
///   **must**  contain `VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_DEPTH_COMPARISON_BIT`
/// - If a [`ImageView`] is accessed using atomic operations as a result of this command, then the
///   image view’s [format features]() **must**  contain
///   `VK_FORMAT_FEATURE_STORAGE_IMAGE_ATOMIC_BIT`
/// - If a [`ImageView`] is sampled with `VK_FILTER_CUBIC_EXT` as a result of this command, then the
///   image view’s [format features]() **must**  contain
///   `VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_CUBIC_BIT_EXT`
/// - Any [`ImageView`] being sampled with `VK_FILTER_CUBIC_EXT` as a result of this command
///   **must**  have a [`ImageViewType`] and format that supports cubic filtering, as specified by
///   [`FilterCubicImageViewImageFormatPropertiesEXT::filter_cubic`] returned by
///   [`get_physical_device_image_format_properties2`]
/// - Any [`ImageView`] being sampled with `VK_FILTER_CUBIC_EXT` with a reduction mode of either
///   `VK_SAMPLER_REDUCTION_MODE_MIN` or `VK_SAMPLER_REDUCTION_MODE_MAX` as a result of this command
///   **must**  have a [`ImageViewType`] and format that supports cubic filtering together with
///   minmax filtering, as specified by
///   [`FilterCubicImageViewImageFormatPropertiesEXT::filter_cubic_minmax`] returned by
///   [`get_physical_device_image_format_properties2`]
/// - Any [`Image`] created with a [`ImageCreateInfo::flags`] containing
///   `VK_IMAGE_CREATE_CORNER_SAMPLED_BIT_NV` sampled as a result of this command  **must**  only be
///   sampled using a [`SamplerAddressMode`] of `VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE`
/// - Any [`ImageView`] or [`BufferView`] being written as a storage image or storage texel buffer
///   where the image format field of the `OpTypeImage` is `Unknown` then the view’s format feature
///   **must**  contain `VK_FORMAT_FEATURE_2_STORAGE_WRITE_WITHOUT_FORMAT_BIT`
/// - Any [`ImageView`] or [`BufferView`] being read as a storage image or storage texel buffer
///   where the image format field of the `OpTypeImage` is `Unknown` then the view’s format feature
///   **must**  contain `VK_FORMAT_FEATURE_2_STORAGE_READ_WITHOUT_FORMAT_BIT`
/// - For each set *n* that is statically used by the [`Pipeline`] bound to the pipeline bind point
///   used by this command, a descriptor set  **must**  have been bound to *n* at the same pipeline
///   bind point, with a [`PipelineLayout`] that is compatible for set *n*, with the
///   [`PipelineLayout`] used to create the current [`Pipeline`], as described in
///   [[descriptorsets-compatibility]]()
/// - If the [`maintenance4`]() feature is not enabled, then for each push constant that is
///   statically used by the [`Pipeline`] bound to the pipeline bind point used by this command, a
///   push constant value  **must**  have been set for the same pipeline bind point, with a
///   [`PipelineLayout`] that is compatible for push constants, with the [`PipelineLayout`] used to
///   create the current [`Pipeline`], as described in [[descriptorsets-compatibility]]()
/// - Descriptors in each bound descriptor set, specified via [`cmd_bind_descriptor_sets`],
///   **must**  be valid if they are statically used by the [`Pipeline`] bound to the pipeline bind
///   point used by this command
/// - A valid pipeline  **must**  be bound to the pipeline bind point used by this command
/// - If the [`Pipeline`] object bound to the pipeline bind point used by this command requires any
///   dynamic state, that state  **must**  have been set or inherited (if the
///   `[`VK_NV_inherited_viewport_scissor`]` extension is enabled) for [`command_buffer`], and done
///   so after any previously bound pipeline with the corresponding state not specified as dynamic
/// - There  **must**  not have been any calls to dynamic state setting commands for any state not
///   specified as dynamic in the [`Pipeline`] object bound to the pipeline bind point used by this
///   command, since that pipeline was bound
/// - If the [`Pipeline`] object bound to the pipeline bind point used by this command accesses a
///   [`Sampler`] object that uses unnormalized coordinates, that sampler  **must**  not be used to
///   sample from any [`Image`] with a [`ImageView`] of the type `VK_IMAGE_VIEW_TYPE_3D`,
///   `VK_IMAGE_VIEW_TYPE_CUBE`, `VK_IMAGE_VIEW_TYPE_1D_ARRAY`, `VK_IMAGE_VIEW_TYPE_2D_ARRAY` or
///   `VK_IMAGE_VIEW_TYPE_CUBE_ARRAY`, in any shader stage
/// - If the [`Pipeline`] object bound to the pipeline bind point used by this command accesses a
///   [`Sampler`] object that uses unnormalized coordinates, that sampler  **must**  not be used
///   with any of the SPIR-V `OpImageSample*` or `OpImageSparseSample*` instructions with
///   `ImplicitLod`, `Dref` or `Proj` in their name, in any shader stage
/// - If the [`Pipeline`] object bound to the pipeline bind point used by this command accesses a
///   [`Sampler`] object that uses unnormalized coordinates, that sampler  **must**  not be used
///   with any of the SPIR-V `OpImageSample*` or `OpImageSparseSample*` instructions that includes a
///   LOD bias or any offset values, in any shader stage
/// - If the [robust buffer access]() feature is not enabled, and if the [`Pipeline`] object bound
///   to the pipeline bind point used by this command accesses a uniform buffer, it  **must**  not
///   access values outside of the range of the buffer as specified in the descriptor set bound to
///   the same pipeline bind point
/// - If the [robust buffer access]() feature is not enabled, and if the [`Pipeline`] object bound
///   to the pipeline bind point used by this command accesses a storage buffer, it  **must**  not
///   access values outside of the range of the buffer as specified in the descriptor set bound to
///   the same pipeline bind point
/// - If [`command_buffer`] is an unprotected command buffer and [`protectedNoFault`]() is not
///   supported, any resource accessed by the [`Pipeline`] object bound to the pipeline bind point
///   used by this command  **must**  not be a protected resource
/// - If the [`Pipeline`] object bound to the pipeline bind point used by this command accesses a
///   [`Sampler`] or [`ImageView`] object that enables [sampler Y′C<sub>B</sub>C<sub>R</sub>
///   conversion](), that object  **must**  only be used with `OpImageSample*` or
///   `OpImageSparseSample*` instructions
/// - If the [`Pipeline`] object bound to the pipeline bind point used by this command accesses a
///   [`Sampler`] or [`ImageView`] object that enables [sampler Y′C<sub>B</sub>C<sub>R</sub>
///   conversion](), that object  **must**  not use the `ConstOffset` and `Offset` operands
/// - If a [`ImageView`] is accessed using `OpImageWrite` as a result of this command, then the
///   `Type` of the `Texel` operand of that instruction  **must**  have at least as many components
///   as the image view’s format
/// - If a [`BufferView`] is accessed using `OpImageWrite` as a result of this command, then the
///   `Type` of the `Texel` operand of that instruction  **must**  have at least as many components
///   as the buffer view’s format
/// - If a [`ImageView`] with a [`Format`] that has a 64-bit component width is accessed as a result
///   of this command, the `SampledType` of the `OpTypeImage` operand of that instruction  **must**
///   have a `Width` of 64
/// - If a [`ImageView`] with a [`Format`] that has a component width less than 64-bit is accessed
///   as a result of this command, the `SampledType` of the `OpTypeImage` operand of that
///   instruction  **must**  have a `Width` of 32
/// - If a [`BufferView`] with a [`Format`] that has a 64-bit component width is accessed as a
///   result of this command, the `SampledType` of the `OpTypeImage` operand of that instruction
///   **must**  have a `Width` of 64
/// - If a [`BufferView`] with a [`Format`] that has a component width less than 64-bit is accessed
///   as a result of this command, the `SampledType` of the `OpTypeImage` operand of that
///   instruction  **must**  have a `Width` of 32
/// - If the [`sparseImageInt64Atomics`]() feature is not enabled, [`Image`] objects created with
///   the `VK_IMAGE_CREATE_SPARSE_RESIDENCY_BIT` flag  **must**  not be accessed by atomic
///   instructions through an `OpTypeImage` with a `SampledType` with a `Width` of 64 by this
///   command
/// - If the [`sparseImageInt64Atomics`]() feature is not enabled, [`Buffer`] objects created with
///   the `VK_BUFFER_CREATE_SPARSE_RESIDENCY_BIT` flag  **must**  not be accessed by atomic
///   instructions through an `OpTypeImage` with a `SampledType` with a `Width` of 64 by this
///   command
/// - The current render pass  **must**  be [compatible]() with the `renderPass` member of the
///   [`GraphicsPipelineCreateInfo`] structure specified when creating the [`Pipeline`] bound to
///   `VK_PIPELINE_BIND_POINT_GRAPHICS`
/// - The subpass index of the current render pass  **must**  be equal to the `subpass` member of
///   the [`GraphicsPipelineCreateInfo`] structure specified when creating the [`Pipeline`] bound to
///   `VK_PIPELINE_BIND_POINT_GRAPHICS`
/// - Every input attachment used by the current subpass  **must**  be bound to the pipeline via a
///   descriptor set
/// - Memory backing image subresources used as attachments in the current render pass  **must**
///   not be written in any way other than as an attachment by this command
/// - If any recorded command in the current subpass will write to an image subresource as an
///   attachment, this command  **must**  not read from the memory backing that image subresource in
///   any other way than as an attachment
/// - If any recorded command in the current subpass will read from an image subresource used as an
///   attachment in any way other than as an attachment, this command  **must**  not write to that
///   image subresource as an attachment
/// - If the draw is recorded in a render pass instance with multiview enabled, the maximum instance
///   index  **must**  be less than or equal to
///   [`PhysicalDeviceMultiviewProperties::max_multiview_instance_index`]
/// - If the bound graphics pipeline was created with
///   [`PipelineSampleLocationsStateCreateInfoEXT::sample_locations_enable`] set to [`TRUE`] and the
///   current subpass has a depth/stencil attachment, then that attachment  **must**  have been
///   created with the `VK_IMAGE_CREATE_SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_BIT_EXT` bit set
/// - If the bound graphics pipeline state was created with the
///   `VK_DYNAMIC_STATE_SAMPLE_LOCATIONS_EXT` dynamic state enabled then
///   [`cmd_set_sample_locations_ext`] **must**  have been called in the current command buffer
///   prior to this drawing command
/// - If the bound graphics pipeline state was created with the
///   `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` dynamic state enabled, but not the
///   `VK_DYNAMIC_STATE_SCISSOR_WITH_COUNT` dynamic state enabled, then
///   [`cmd_set_viewport_with_count`] **must**  have been called in the current command buffer prior
///   to this drawing command, and the `viewportCount` parameter of [`cmd_set_viewport_with_count`]
///   **must**  match the [`PipelineViewportStateCreateInfo::scissor_count`] of the pipeline
/// - If the bound graphics pipeline state was created with the
///   `VK_DYNAMIC_STATE_SCISSOR_WITH_COUNT` dynamic state enabled, but not the
///   `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` dynamic state enabled, then
///   [`cmd_set_scissor_with_count`] **must**  have been called in the current command buffer prior
///   to this drawing command, and the `scissorCount` parameter of [`cmd_set_scissor_with_count`]
///   **must**  match the [`PipelineViewportStateCreateInfo::viewport_count`] of the pipeline
/// - If the bound graphics pipeline state was created with both the
///   `VK_DYNAMIC_STATE_SCISSOR_WITH_COUNT` and `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` dynamic
///   states enabled then both [`cmd_set_viewport_with_count`] and [`cmd_set_scissor_with_count`]
///   **must**  have been called in the current command buffer prior to this drawing command, and
///   the `viewportCount` parameter of [`cmd_set_viewport_with_count`] **must**  match the
///   `scissorCount` parameter of [`cmd_set_scissor_with_count`]
/// - If the bound graphics pipeline state was created with the
///   `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` dynamic state enabled, but not the
///   `VK_DYNAMIC_STATE_VIEWPORT_W_SCALING_NV` dynamic state enabled, then the bound graphics
///   pipeline  **must**  have been created with
///   [`PipelineViewportWScalingStateCreateInfoNV::viewport_count`] greater or equal to the
///   `viewportCount` parameter in the last call to [`cmd_set_viewport_with_count`]
/// - If the bound graphics pipeline state was created with the
///   `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` and `VK_DYNAMIC_STATE_VIEWPORT_W_SCALING_NV` dynamic
///   states enabled then the `viewportCount` parameter in the last call to
///   [`cmd_set_viewport_w_scaling_nv`] **must**  be greater than or equal to the `viewportCount`
///   parameter in the last call to [`cmd_set_viewport_with_count`]
/// - If the bound graphics pipeline state was created with the
///   `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` dynamic state enabled, but not the
///   `VK_DYNAMIC_STATE_VIEWPORT_SHADING_RATE_PALETTE_NV` dynamic state enabled, then the bound
///   graphics pipeline  **must**  have been created with
///   [`PipelineViewportShadingRateImageStateCreateInfoNV::viewport_count`] greater or equal to the
///   `viewportCount` parameter in the last call to [`cmd_set_viewport_with_count`]
/// - If the bound graphics pipeline state was created with the
///   `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` and `VK_DYNAMIC_STATE_VIEWPORT_SHADING_RATE_PALETTE_NV`
///   dynamic states enabled then the `viewportCount` parameter in the last call to
///   [`cmd_set_viewport_shading_rate_palette_nv`] **must**  be greater than or equal to the
///   `viewportCount` parameter in the last call to [`cmd_set_viewport_with_count`]
/// - If the bound graphics pipeline state was created with the
///   `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` dynamic state enabled and a
///   [`PipelineViewportSwizzleStateCreateInfoNV`] structure chained from
///   `VkPipelineVieportCreateInfo`, then the bound graphics pipeline  **must**  have been created
///   with [`PipelineViewportSwizzleStateCreateInfoNV::viewport_count`] greater or equal to the
///   `viewportCount` parameter in the last call to [`cmd_set_viewport_with_count`]
/// - If the bound graphics pipeline state was created with the
///   `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` dynamic state enabled and a
///   [`PipelineViewportExclusiveScissorStateCreateInfoNV`] structure chained from
///   `VkPipelineVieportCreateInfo`, then the bound graphics pipeline  **must**  have been created
///   with [`PipelineViewportExclusiveScissorStateCreateInfoNV::exclusive_scissor_count`] greater or
///   equal to the `viewportCount` parameter in the last call to [`cmd_set_viewport_with_count`]
/// - If the bound graphics pipeline state was created with the
///   `VK_DYNAMIC_STATE_RASTERIZER_DISCARD_ENABLE` dynamic state enabled then
///   [`cmd_set_rasterizer_discard_enable`] **must**  have been called in the current command buffer
///   prior to this drawing command
/// - If the bound graphics pipeline state was created with the `VK_DYNAMIC_STATE_DEPTH_BIAS_ENABLE`
///   dynamic state enabled then [`cmd_set_depth_bias_enable`] **must**  have been called in the
///   current command buffer prior to this drawing command
/// - If the bound graphics pipeline state was created with the `VK_DYNAMIC_STATE_LOGIC_OP_EXT`
///   dynamic state enabled then [`cmd_set_logic_op_ext`] **must**  have been called in the current
///   command buffer prior to this drawing command and the `logicOp` **must**  be a valid
///   [`LogicOp`] value
/// - If the [`primitiveFragmentShadingRateWithMultipleViewports`]() limit is not supported, the
///   bound graphics pipeline was created with the `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` dynamic
///   state enabled, and any of the shader stages of the bound graphics pipeline write to the
///   `PrimitiveShadingRateKHR` built-in, then [`cmd_set_viewport_with_count`] **must**  have been
///   called in the current command buffer prior to this drawing command, and the `viewportCount`
///   parameter of [`cmd_set_viewport_with_count`] **must**  be `1`
/// - If rasterization is not disabled in the bound graphics pipeline, then for each color
///   attachment in the subpass, if the corresponding image view’s [format features]() do not
///   contain `VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BLEND_BIT`, then the `blendEnable` member of the
///   corresponding element of the `pAttachments` member of `pColorBlendState` **must**  be
///   [`FALSE`]
/// - If rasterization is not disabled in the bound graphics pipeline, and neither the
///   `[`VK_AMD_mixed_attachment_samples`]` nor the `[`VK_NV_framebuffer_mixed_samples`]` extensions
///   are enabled, then [`PipelineMultisampleStateCreateInfo::rasterization_samples`] **must**  be
///   the same as the current subpass color and/or depth/stencil attachments
/// - If the current render pass instance was begun with [`cmd_begin_rendering`], the `imageView`
///   member of `pDepthAttachment` is not [`crate::Handle::null`], and the `layout` member of
///   `pDepthAttachment` is `VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL`, this command
///   **must**  not write any values to the depth attachment
/// - If the current render pass instance was begun with [`cmd_begin_rendering`], the `imageView`
///   member of `pStencilAttachment` is not [`crate::Handle::null`], and the `layout` member of
///   `pStencilAttachment` is `VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL`, this command
///   **must**  not write any values to the stencil attachment
/// - If the current render pass instance was begun with [`cmd_begin_rendering`], the `imageView`
///   member of `pDepthAttachment` is not [`crate::Handle::null`], and the `layout` member of
///   `pDepthAttachment` is `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL`, this
///   command  **must**  not write any values to the depth attachment
/// - If the current render pass instance was begun with [`cmd_begin_rendering`], the `imageView`
///   member of `pStencilAttachment` is not [`crate::Handle::null`], and the `layout` member of
///   `pStencilAttachment` is `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL`, this
///   command  **must**  not write any values to the stencil attachment
/// - If the current render pass instance was begun with [`cmd_begin_rendering`], the `imageView`
///   member of `pDepthAttachment` is not [`crate::Handle::null`], and the `layout` member of
///   `pDepthAttachment` is `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL`, this command  **must**  not
///   write any values to the depth attachment
/// - If the current render pass instance was begun with [`cmd_begin_rendering`], the `imageView`
///   member of `pStencilAttachment` is not [`crate::Handle::null`], and the `layout` member of
///   `pStencilAttachment` is `VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL`, this command  **must**
///   not write any values to the stencil attachment
/// - If the current render pass instance was begun with [`cmd_begin_rendering`], the currently
///   bound graphics pipeline  **must**  have been created with a
///   [`PipelineRenderingCreateInfo::view_mask`] equal to [`RenderingInfo::view_mask`]
/// - If the current render pass instance was begun with [`cmd_begin_rendering`], the currently
///   bound graphics pipeline  **must**  have been created with a
///   [`PipelineRenderingCreateInfo::color_attachment_count`] equal to
///   [`RenderingInfo::color_attachment_count`]
/// - If the current render pass instance was begun with [`cmd_begin_rendering`] and
///   [`RenderingInfo::color_attachment_count`] greater than `0`, then each element of the
///   [`RenderingInfo::color_attachments`] array with a `imageView` not equal to
///   [`crate::Handle::null`] **must**  have been created with a [`Format`] equal to the
///   corresponding element of [`PipelineRenderingCreateInfo::color_attachment_formats`] used to
///   create the currently bound graphics pipeline
/// - If the bound graphics pipeline state was created with the
///   `VK_DYNAMIC_STATE_COLOR_WRITE_ENABLE_EXT` dynamic state enabled then
///   [`cmd_set_color_write_enable_ext`] **must**  have been called in the current command buffer
///   prior to this drawing command, and the `attachmentCount` parameter of
///   [`cmd_set_color_write_enable_ext`] **must**  be equal to the
///   [`PipelineColorBlendStateCreateInfo::attachment_count`] of the currently bound graphics
///   pipeline
/// - If the current render pass instance was begun with [`cmd_begin_rendering`] and
///   [`RenderingInfo`]::`pDepthAttachment->pname`:imageView was not [`crate::Handle::null`], the
///   value of [`PipelineRenderingCreateInfo::depth_attachment_format`] used to create the currently
///   bound graphics pipeline  **must**  be equal to the [`Format`] used to create
///   [`RenderingInfo`]::`pDepthAttachment->pname`:imageView
/// - If the current render pass instance was begun with [`cmd_begin_rendering`] and
///   [`RenderingInfo`]::`pStencilAttachment->pname`:imageView was not [`crate::Handle::null`], the
///   value of [`PipelineRenderingCreateInfo::stencil_attachment_format`] used to create the
///   currently bound graphics pipeline  **must**  be equal to the [`Format`] used to create
///   [`RenderingInfo`]::`pStencilAttachment->pname`:imageView
/// - If the current render pass instance was begun with [`cmd_begin_rendering`] and
///   [`RenderingFragmentShadingRateAttachmentInfoKHR::image_view`] was not [`crate::Handle::null`],
///   the currently bound graphics pipeline  **must**  have been created with
///   `VK_PIPELINE_CREATE_RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR`
/// - If the current render pass instance was begun with [`cmd_begin_rendering`] and
///   [`RenderingFragmentDensityMapAttachmentInfoEXT::image_view`] was not [`crate::Handle::null`],
///   the currently bound graphics pipeline  **must**  have been created with
///   `VK_PIPELINE_CREATE_RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_BIT_EXT`
/// - If the currently bound pipeline was created with a [`AttachmentSampleCountInfoAMD`] or
///   [`AttachmentSampleCountInfoNV`] structure, and the current render pass instance was begun with
///   [`cmd_begin_rendering`] with a [`RenderingInfo::color_attachment_count`] parameter greater
///   than `0`, then each element of the [`RenderingInfo::color_attachments`] array with a
///   `imageView` not equal to [`crate::Handle::null`] **must**  have been created with a sample
///   count equal to the corresponding element of the `pColorAttachmentSamples` member of
///   [`AttachmentSampleCountInfoAMD`] or [`AttachmentSampleCountInfoNV`] used to create the
///   currently bound graphics pipeline
/// - If the current render pass instance was begun with [`cmd_begin_rendering`], the currently
///   bound pipeline was created with a [`AttachmentSampleCountInfoAMD`] or
///   [`AttachmentSampleCountInfoNV`] structure, and
///   [`RenderingInfo`]::`pDepthAttachment->pname`:imageView was not [`crate::Handle::null`], the
///   value of the `depthStencilAttachmentSamples` member of [`AttachmentSampleCountInfoAMD`] or
///   [`AttachmentSampleCountInfoNV`] used to create the currently bound graphics pipeline  **must**
///   be equal to the sample count used to create
///   [`RenderingInfo`]::`pDepthAttachment->pname`:imageView
/// - If the current render pass instance was begun with [`cmd_begin_rendering`], the currently
///   bound pipeline was created with a [`AttachmentSampleCountInfoAMD`] or
///   [`AttachmentSampleCountInfoNV`] structure, and
///   [`RenderingInfo`]::`pStencilAttachment->pname`:imageView was not [`crate::Handle::null`], the
///   value of the `depthStencilAttachmentSamples` member of [`AttachmentSampleCountInfoAMD`] or
///   [`AttachmentSampleCountInfoNV`] used to create the currently bound graphics pipeline  **must**
///   be equal to the sample count used to create
///   [`RenderingInfo`]::`pStencilAttachment->pname`:imageView
/// - If the currently bound pipeline was created without a [`AttachmentSampleCountInfoAMD`] or
///   [`AttachmentSampleCountInfoNV`] structure, and the current render pass instance was begun with
///   [`cmd_begin_rendering`] with a [`RenderingInfo::color_attachment_count`] parameter greater
///   than `0`, then each element of the [`RenderingInfo::color_attachments`] array with a
///   `imageView` not equal to [`crate::Handle::null`] **must**  have been created with a sample
///   count equal to the value of [`PipelineMultisampleStateCreateInfo::rasterization_samples`] used
///   to create the currently bound graphics pipeline
/// - If the current render pass instance was begun with [`cmd_begin_rendering`], the currently
///   bound pipeline was created without a [`AttachmentSampleCountInfoAMD`] or
///   [`AttachmentSampleCountInfoNV`] structure, and
///   [`RenderingInfo`]::`pDepthAttachment->pname`:imageView was not [`crate::Handle::null`], the
///   value of [`PipelineMultisampleStateCreateInfo::rasterization_samples`] used to create the
///   currently bound graphics pipeline  **must**  be equal to the sample count used to create
///   [`RenderingInfo`]::`pDepthAttachment->pname`:imageView
/// - If the current render pass instance was begun with [`cmd_begin_rendering`], the currently
///   bound pipeline was created without a [`AttachmentSampleCountInfoAMD`] or
///   [`AttachmentSampleCountInfoNV`] structure, and
///   [`RenderingInfo`]::`pStencilAttachment->pname`:imageView was not [`crate::Handle::null`], the
///   value of [`PipelineMultisampleStateCreateInfo::rasterization_samples`] used to create the
///   currently bound graphics pipeline  **must**  be equal to the sample count used to create
///   [`RenderingInfo`]::`pStencilAttachment->pname`:imageView
/// - If the current render pass instance was begun with [`cmd_begin_rendering`], the currently
///   bound pipeline  **must**  have been created with a [`GraphicsPipelineCreateInfo::render_pass`]
///   equal to [`crate::Handle::null`]
///
/// - The bound graphics pipeline  **must**  not have been created with the
///   [`PipelineShaderStageCreateInfo::stage`] member of an element of
///   [`GraphicsPipelineCreateInfo::stages`] set to `VK_SHADER_STAGE_VERTEX_BIT`,
///   `VK_SHADER_STAGE_TESSELLATION_CONTROL_BIT`, `VK_SHADER_STAGE_TESSELLATION_EVALUATION_BIT` or
///   `VK_SHADER_STAGE_GEOMETRY_BIT`
///
/// - If [`buffer`] is non-sparse then it  **must**  be bound completely and contiguously to a
///   single [`DeviceMemory`] object
/// - [`buffer`] **must**  have been created with the `VK_BUFFER_USAGE_INDIRECT_BUFFER_BIT` bit set
/// - [`offset`] **must**  be a multiple of `4`
/// - [`command_buffer`] **must**  not be a protected command buffer
///
/// - If the [multi-draw indirect]() feature is not enabled, [`draw_count`] **must**  be `0` or `1`
/// - [`draw_count`] **must**  be less than or equal to
///   [`PhysicalDeviceLimits::max_draw_indirect_count`]
/// - If [`draw_count`] is greater than `1`, [`stride`] **must**  be a multiple of `4` and  **must**
///   be greater than or equal to `sizeof`([`DrawMeshTasksIndirectCommandNV`])
/// - If [`draw_count`] is equal to `1`, ([`offset`] +
///   `sizeof`([`DrawMeshTasksIndirectCommandNV`])) **must**  be less than or equal to the size of
///   [`buffer`]
/// - If [`draw_count`] is greater than `1`, ([`stride`] × ([`draw_count`] - 1) +  [`offset`] +
///   `sizeof`([`DrawMeshTasksIndirectCommandNV`])) **must**  be less than or equal to the size of
///   [`buffer`]
///
///## Valid Usage (Implicit)
/// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
/// - [`buffer`] **must**  be a valid [`Buffer`] handle
/// - [`command_buffer`] **must**  be in the [recording state]()
/// - The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics
///   operations
/// - This command  **must**  only be called inside of a render pass instance
/// - Both of [`buffer`], and [`command_buffer`] **must**  have been created, allocated, or
///   retrieved from the same [`Device`]
///
///## Host Synchronization
/// - Host access to [`command_buffer`] **must**  be externally synchronized
/// - Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be
///   externally synchronized
///
///## Command Properties
///# Related
/// - [`VK_NV_mesh_shader`]
/// - [`Buffer`]
/// - [`CommandBuffer`]
/// - [`DeviceSize`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkCmdDrawMeshTasksIndirectNV")]
pub type FNCmdDrawMeshTasksIndirectNv = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        draw_count: u32,
        stride: u32,
    ),
>;
///[vkCmdDrawMeshTasksIndirectCountNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawMeshTasksIndirectCountNV.html) - Perform an indirect mesh tasks draw with the draw count sourced from a buffer
///# C Specifications
///To record an indirect mesh tasks draw with the draw count sourced from a
///buffer, call:
///```c
///// Provided by VK_NV_mesh_shader
///void vkCmdDrawMeshTasksIndirectCountNV(
///    VkCommandBuffer                             commandBuffer,
///    VkBuffer                                    buffer,
///    VkDeviceSize                                offset,
///    VkBuffer                                    countBuffer,
///    VkDeviceSize                                countBufferOffset,
///    uint32_t                                    maxDrawCount,
///    uint32_t                                    stride);
///```
///# Parameters
/// - [`command_buffer`] is the command buffer into which the command is recorded.
/// - [`buffer`] is the buffer containing draw parameters.
/// - [`offset`] is the byte offset into [`buffer`] where parameters begin.
/// - [`count_buffer`] is the buffer containing the draw count.
/// - [`count_buffer_offset`] is the byte offset into [`count_buffer`] where the draw count begins.
/// - [`max_draw_count`] specifies the maximum number of draws that will be executed. The actual
///   number of executed draw calls is the minimum of the count specified in [`count_buffer`] and
///   [`max_draw_count`].
/// - [`stride`] is the byte stride between successive sets of draw parameters.
///# Description
///[`cmd_draw_mesh_tasks_indirect_count_nv`] behaves similarly to
///[`cmd_draw_mesh_tasks_indirect_nv`] except that the draw count is read by the
///device from a buffer during execution.
///The command will read an unsigned 32-bit integer from [`count_buffer`]
///located at [`count_buffer_offset`] and use this as the draw count.
///## Valid Usage
/// - If a [`Sampler`] created with `magFilter` or `minFilter` equal to `VK_FILTER_LINEAR` and
///   `compareEnable` equal to [`FALSE`] is used to sample a [`ImageView`] as a result of this
///   command, then the image view’s [format features]() **must**  contain
///   `VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT`
/// - If a [`Sampler`] created with `mipmapMode` equal to `VK_SAMPLER_MIPMAP_MODE_LINEAR` and
///   `compareEnable` equal to [`FALSE`] is used to sample a [`ImageView`] as a result of this
///   command, then the image view’s [format features]() **must**  contain
///   `VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT`
/// - If a [`ImageView`] is sampled with [depth comparison](), the image view’s [format features]()
///   **must**  contain `VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_DEPTH_COMPARISON_BIT`
/// - If a [`ImageView`] is accessed using atomic operations as a result of this command, then the
///   image view’s [format features]() **must**  contain
///   `VK_FORMAT_FEATURE_STORAGE_IMAGE_ATOMIC_BIT`
/// - If a [`ImageView`] is sampled with `VK_FILTER_CUBIC_EXT` as a result of this command, then the
///   image view’s [format features]() **must**  contain
///   `VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_CUBIC_BIT_EXT`
/// - Any [`ImageView`] being sampled with `VK_FILTER_CUBIC_EXT` as a result of this command
///   **must**  have a [`ImageViewType`] and format that supports cubic filtering, as specified by
///   [`FilterCubicImageViewImageFormatPropertiesEXT::filter_cubic`] returned by
///   [`get_physical_device_image_format_properties2`]
/// - Any [`ImageView`] being sampled with `VK_FILTER_CUBIC_EXT` with a reduction mode of either
///   `VK_SAMPLER_REDUCTION_MODE_MIN` or `VK_SAMPLER_REDUCTION_MODE_MAX` as a result of this command
///   **must**  have a [`ImageViewType`] and format that supports cubic filtering together with
///   minmax filtering, as specified by
///   [`FilterCubicImageViewImageFormatPropertiesEXT::filter_cubic_minmax`] returned by
///   [`get_physical_device_image_format_properties2`]
/// - Any [`Image`] created with a [`ImageCreateInfo::flags`] containing
///   `VK_IMAGE_CREATE_CORNER_SAMPLED_BIT_NV` sampled as a result of this command  **must**  only be
///   sampled using a [`SamplerAddressMode`] of `VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE`
/// - Any [`ImageView`] or [`BufferView`] being written as a storage image or storage texel buffer
///   where the image format field of the `OpTypeImage` is `Unknown` then the view’s format feature
///   **must**  contain `VK_FORMAT_FEATURE_2_STORAGE_WRITE_WITHOUT_FORMAT_BIT`
/// - Any [`ImageView`] or [`BufferView`] being read as a storage image or storage texel buffer
///   where the image format field of the `OpTypeImage` is `Unknown` then the view’s format feature
///   **must**  contain `VK_FORMAT_FEATURE_2_STORAGE_READ_WITHOUT_FORMAT_BIT`
/// - For each set *n* that is statically used by the [`Pipeline`] bound to the pipeline bind point
///   used by this command, a descriptor set  **must**  have been bound to *n* at the same pipeline
///   bind point, with a [`PipelineLayout`] that is compatible for set *n*, with the
///   [`PipelineLayout`] used to create the current [`Pipeline`], as described in
///   [[descriptorsets-compatibility]]()
/// - If the [`maintenance4`]() feature is not enabled, then for each push constant that is
///   statically used by the [`Pipeline`] bound to the pipeline bind point used by this command, a
///   push constant value  **must**  have been set for the same pipeline bind point, with a
///   [`PipelineLayout`] that is compatible for push constants, with the [`PipelineLayout`] used to
///   create the current [`Pipeline`], as described in [[descriptorsets-compatibility]]()
/// - Descriptors in each bound descriptor set, specified via [`cmd_bind_descriptor_sets`],
///   **must**  be valid if they are statically used by the [`Pipeline`] bound to the pipeline bind
///   point used by this command
/// - A valid pipeline  **must**  be bound to the pipeline bind point used by this command
/// - If the [`Pipeline`] object bound to the pipeline bind point used by this command requires any
///   dynamic state, that state  **must**  have been set or inherited (if the
///   `[`VK_NV_inherited_viewport_scissor`]` extension is enabled) for [`command_buffer`], and done
///   so after any previously bound pipeline with the corresponding state not specified as dynamic
/// - There  **must**  not have been any calls to dynamic state setting commands for any state not
///   specified as dynamic in the [`Pipeline`] object bound to the pipeline bind point used by this
///   command, since that pipeline was bound
/// - If the [`Pipeline`] object bound to the pipeline bind point used by this command accesses a
///   [`Sampler`] object that uses unnormalized coordinates, that sampler  **must**  not be used to
///   sample from any [`Image`] with a [`ImageView`] of the type `VK_IMAGE_VIEW_TYPE_3D`,
///   `VK_IMAGE_VIEW_TYPE_CUBE`, `VK_IMAGE_VIEW_TYPE_1D_ARRAY`, `VK_IMAGE_VIEW_TYPE_2D_ARRAY` or
///   `VK_IMAGE_VIEW_TYPE_CUBE_ARRAY`, in any shader stage
/// - If the [`Pipeline`] object bound to the pipeline bind point used by this command accesses a
///   [`Sampler`] object that uses unnormalized coordinates, that sampler  **must**  not be used
///   with any of the SPIR-V `OpImageSample*` or `OpImageSparseSample*` instructions with
///   `ImplicitLod`, `Dref` or `Proj` in their name, in any shader stage
/// - If the [`Pipeline`] object bound to the pipeline bind point used by this command accesses a
///   [`Sampler`] object that uses unnormalized coordinates, that sampler  **must**  not be used
///   with any of the SPIR-V `OpImageSample*` or `OpImageSparseSample*` instructions that includes a
///   LOD bias or any offset values, in any shader stage
/// - If the [robust buffer access]() feature is not enabled, and if the [`Pipeline`] object bound
///   to the pipeline bind point used by this command accesses a uniform buffer, it  **must**  not
///   access values outside of the range of the buffer as specified in the descriptor set bound to
///   the same pipeline bind point
/// - If the [robust buffer access]() feature is not enabled, and if the [`Pipeline`] object bound
///   to the pipeline bind point used by this command accesses a storage buffer, it  **must**  not
///   access values outside of the range of the buffer as specified in the descriptor set bound to
///   the same pipeline bind point
/// - If [`command_buffer`] is an unprotected command buffer and [`protectedNoFault`]() is not
///   supported, any resource accessed by the [`Pipeline`] object bound to the pipeline bind point
///   used by this command  **must**  not be a protected resource
/// - If the [`Pipeline`] object bound to the pipeline bind point used by this command accesses a
///   [`Sampler`] or [`ImageView`] object that enables [sampler Y′C<sub>B</sub>C<sub>R</sub>
///   conversion](), that object  **must**  only be used with `OpImageSample*` or
///   `OpImageSparseSample*` instructions
/// - If the [`Pipeline`] object bound to the pipeline bind point used by this command accesses a
///   [`Sampler`] or [`ImageView`] object that enables [sampler Y′C<sub>B</sub>C<sub>R</sub>
///   conversion](), that object  **must**  not use the `ConstOffset` and `Offset` operands
/// - If a [`ImageView`] is accessed using `OpImageWrite` as a result of this command, then the
///   `Type` of the `Texel` operand of that instruction  **must**  have at least as many components
///   as the image view’s format
/// - If a [`BufferView`] is accessed using `OpImageWrite` as a result of this command, then the
///   `Type` of the `Texel` operand of that instruction  **must**  have at least as many components
///   as the buffer view’s format
/// - If a [`ImageView`] with a [`Format`] that has a 64-bit component width is accessed as a result
///   of this command, the `SampledType` of the `OpTypeImage` operand of that instruction  **must**
///   have a `Width` of 64
/// - If a [`ImageView`] with a [`Format`] that has a component width less than 64-bit is accessed
///   as a result of this command, the `SampledType` of the `OpTypeImage` operand of that
///   instruction  **must**  have a `Width` of 32
/// - If a [`BufferView`] with a [`Format`] that has a 64-bit component width is accessed as a
///   result of this command, the `SampledType` of the `OpTypeImage` operand of that instruction
///   **must**  have a `Width` of 64
/// - If a [`BufferView`] with a [`Format`] that has a component width less than 64-bit is accessed
///   as a result of this command, the `SampledType` of the `OpTypeImage` operand of that
///   instruction  **must**  have a `Width` of 32
/// - If the [`sparseImageInt64Atomics`]() feature is not enabled, [`Image`] objects created with
///   the `VK_IMAGE_CREATE_SPARSE_RESIDENCY_BIT` flag  **must**  not be accessed by atomic
///   instructions through an `OpTypeImage` with a `SampledType` with a `Width` of 64 by this
///   command
/// - If the [`sparseImageInt64Atomics`]() feature is not enabled, [`Buffer`] objects created with
///   the `VK_BUFFER_CREATE_SPARSE_RESIDENCY_BIT` flag  **must**  not be accessed by atomic
///   instructions through an `OpTypeImage` with a `SampledType` with a `Width` of 64 by this
///   command
/// - The current render pass  **must**  be [compatible]() with the `renderPass` member of the
///   [`GraphicsPipelineCreateInfo`] structure specified when creating the [`Pipeline`] bound to
///   `VK_PIPELINE_BIND_POINT_GRAPHICS`
/// - The subpass index of the current render pass  **must**  be equal to the `subpass` member of
///   the [`GraphicsPipelineCreateInfo`] structure specified when creating the [`Pipeline`] bound to
///   `VK_PIPELINE_BIND_POINT_GRAPHICS`
/// - Every input attachment used by the current subpass  **must**  be bound to the pipeline via a
///   descriptor set
/// - Memory backing image subresources used as attachments in the current render pass  **must**
///   not be written in any way other than as an attachment by this command
/// - If any recorded command in the current subpass will write to an image subresource as an
///   attachment, this command  **must**  not read from the memory backing that image subresource in
///   any other way than as an attachment
/// - If any recorded command in the current subpass will read from an image subresource used as an
///   attachment in any way other than as an attachment, this command  **must**  not write to that
///   image subresource as an attachment
/// - If the draw is recorded in a render pass instance with multiview enabled, the maximum instance
///   index  **must**  be less than or equal to
///   [`PhysicalDeviceMultiviewProperties::max_multiview_instance_index`]
/// - If the bound graphics pipeline was created with
///   [`PipelineSampleLocationsStateCreateInfoEXT::sample_locations_enable`] set to [`TRUE`] and the
///   current subpass has a depth/stencil attachment, then that attachment  **must**  have been
///   created with the `VK_IMAGE_CREATE_SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_BIT_EXT` bit set
/// - If the bound graphics pipeline state was created with the
///   `VK_DYNAMIC_STATE_SAMPLE_LOCATIONS_EXT` dynamic state enabled then
///   [`cmd_set_sample_locations_ext`] **must**  have been called in the current command buffer
///   prior to this drawing command
/// - If the bound graphics pipeline state was created with the
///   `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` dynamic state enabled, but not the
///   `VK_DYNAMIC_STATE_SCISSOR_WITH_COUNT` dynamic state enabled, then
///   [`cmd_set_viewport_with_count`] **must**  have been called in the current command buffer prior
///   to this drawing command, and the `viewportCount` parameter of [`cmd_set_viewport_with_count`]
///   **must**  match the [`PipelineViewportStateCreateInfo::scissor_count`] of the pipeline
/// - If the bound graphics pipeline state was created with the
///   `VK_DYNAMIC_STATE_SCISSOR_WITH_COUNT` dynamic state enabled, but not the
///   `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` dynamic state enabled, then
///   [`cmd_set_scissor_with_count`] **must**  have been called in the current command buffer prior
///   to this drawing command, and the `scissorCount` parameter of [`cmd_set_scissor_with_count`]
///   **must**  match the [`PipelineViewportStateCreateInfo::viewport_count`] of the pipeline
/// - If the bound graphics pipeline state was created with both the
///   `VK_DYNAMIC_STATE_SCISSOR_WITH_COUNT` and `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` dynamic
///   states enabled then both [`cmd_set_viewport_with_count`] and [`cmd_set_scissor_with_count`]
///   **must**  have been called in the current command buffer prior to this drawing command, and
///   the `viewportCount` parameter of [`cmd_set_viewport_with_count`] **must**  match the
///   `scissorCount` parameter of [`cmd_set_scissor_with_count`]
/// - If the bound graphics pipeline state was created with the
///   `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` dynamic state enabled, but not the
///   `VK_DYNAMIC_STATE_VIEWPORT_W_SCALING_NV` dynamic state enabled, then the bound graphics
///   pipeline  **must**  have been created with
///   [`PipelineViewportWScalingStateCreateInfoNV::viewport_count`] greater or equal to the
///   `viewportCount` parameter in the last call to [`cmd_set_viewport_with_count`]
/// - If the bound graphics pipeline state was created with the
///   `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` and `VK_DYNAMIC_STATE_VIEWPORT_W_SCALING_NV` dynamic
///   states enabled then the `viewportCount` parameter in the last call to
///   [`cmd_set_viewport_w_scaling_nv`] **must**  be greater than or equal to the `viewportCount`
///   parameter in the last call to [`cmd_set_viewport_with_count`]
/// - If the bound graphics pipeline state was created with the
///   `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` dynamic state enabled, but not the
///   `VK_DYNAMIC_STATE_VIEWPORT_SHADING_RATE_PALETTE_NV` dynamic state enabled, then the bound
///   graphics pipeline  **must**  have been created with
///   [`PipelineViewportShadingRateImageStateCreateInfoNV::viewport_count`] greater or equal to the
///   `viewportCount` parameter in the last call to [`cmd_set_viewport_with_count`]
/// - If the bound graphics pipeline state was created with the
///   `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` and `VK_DYNAMIC_STATE_VIEWPORT_SHADING_RATE_PALETTE_NV`
///   dynamic states enabled then the `viewportCount` parameter in the last call to
///   [`cmd_set_viewport_shading_rate_palette_nv`] **must**  be greater than or equal to the
///   `viewportCount` parameter in the last call to [`cmd_set_viewport_with_count`]
/// - If the bound graphics pipeline state was created with the
///   `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` dynamic state enabled and a
///   [`PipelineViewportSwizzleStateCreateInfoNV`] structure chained from
///   `VkPipelineVieportCreateInfo`, then the bound graphics pipeline  **must**  have been created
///   with [`PipelineViewportSwizzleStateCreateInfoNV::viewport_count`] greater or equal to the
///   `viewportCount` parameter in the last call to [`cmd_set_viewport_with_count`]
/// - If the bound graphics pipeline state was created with the
///   `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` dynamic state enabled and a
///   [`PipelineViewportExclusiveScissorStateCreateInfoNV`] structure chained from
///   `VkPipelineVieportCreateInfo`, then the bound graphics pipeline  **must**  have been created
///   with [`PipelineViewportExclusiveScissorStateCreateInfoNV::exclusive_scissor_count`] greater or
///   equal to the `viewportCount` parameter in the last call to [`cmd_set_viewport_with_count`]
/// - If the bound graphics pipeline state was created with the
///   `VK_DYNAMIC_STATE_RASTERIZER_DISCARD_ENABLE` dynamic state enabled then
///   [`cmd_set_rasterizer_discard_enable`] **must**  have been called in the current command buffer
///   prior to this drawing command
/// - If the bound graphics pipeline state was created with the `VK_DYNAMIC_STATE_DEPTH_BIAS_ENABLE`
///   dynamic state enabled then [`cmd_set_depth_bias_enable`] **must**  have been called in the
///   current command buffer prior to this drawing command
/// - If the bound graphics pipeline state was created with the `VK_DYNAMIC_STATE_LOGIC_OP_EXT`
///   dynamic state enabled then [`cmd_set_logic_op_ext`] **must**  have been called in the current
///   command buffer prior to this drawing command and the `logicOp` **must**  be a valid
///   [`LogicOp`] value
/// - If the [`primitiveFragmentShadingRateWithMultipleViewports`]() limit is not supported, the
///   bound graphics pipeline was created with the `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` dynamic
///   state enabled, and any of the shader stages of the bound graphics pipeline write to the
///   `PrimitiveShadingRateKHR` built-in, then [`cmd_set_viewport_with_count`] **must**  have been
///   called in the current command buffer prior to this drawing command, and the `viewportCount`
///   parameter of [`cmd_set_viewport_with_count`] **must**  be `1`
/// - If rasterization is not disabled in the bound graphics pipeline, then for each color
///   attachment in the subpass, if the corresponding image view’s [format features]() do not
///   contain `VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BLEND_BIT`, then the `blendEnable` member of the
///   corresponding element of the `pAttachments` member of `pColorBlendState` **must**  be
///   [`FALSE`]
/// - If rasterization is not disabled in the bound graphics pipeline, and neither the
///   `[`VK_AMD_mixed_attachment_samples`]` nor the `[`VK_NV_framebuffer_mixed_samples`]` extensions
///   are enabled, then [`PipelineMultisampleStateCreateInfo::rasterization_samples`] **must**  be
///   the same as the current subpass color and/or depth/stencil attachments
/// - If the current render pass instance was begun with [`cmd_begin_rendering`], the `imageView`
///   member of `pDepthAttachment` is not [`crate::Handle::null`], and the `layout` member of
///   `pDepthAttachment` is `VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL`, this command
///   **must**  not write any values to the depth attachment
/// - If the current render pass instance was begun with [`cmd_begin_rendering`], the `imageView`
///   member of `pStencilAttachment` is not [`crate::Handle::null`], and the `layout` member of
///   `pStencilAttachment` is `VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL`, this command
///   **must**  not write any values to the stencil attachment
/// - If the current render pass instance was begun with [`cmd_begin_rendering`], the `imageView`
///   member of `pDepthAttachment` is not [`crate::Handle::null`], and the `layout` member of
///   `pDepthAttachment` is `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL`, this
///   command  **must**  not write any values to the depth attachment
/// - If the current render pass instance was begun with [`cmd_begin_rendering`], the `imageView`
///   member of `pStencilAttachment` is not [`crate::Handle::null`], and the `layout` member of
///   `pStencilAttachment` is `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL`, this
///   command  **must**  not write any values to the stencil attachment
/// - If the current render pass instance was begun with [`cmd_begin_rendering`], the `imageView`
///   member of `pDepthAttachment` is not [`crate::Handle::null`], and the `layout` member of
///   `pDepthAttachment` is `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL`, this command  **must**  not
///   write any values to the depth attachment
/// - If the current render pass instance was begun with [`cmd_begin_rendering`], the `imageView`
///   member of `pStencilAttachment` is not [`crate::Handle::null`], and the `layout` member of
///   `pStencilAttachment` is `VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL`, this command  **must**
///   not write any values to the stencil attachment
/// - If the current render pass instance was begun with [`cmd_begin_rendering`], the currently
///   bound graphics pipeline  **must**  have been created with a
///   [`PipelineRenderingCreateInfo::view_mask`] equal to [`RenderingInfo::view_mask`]
/// - If the current render pass instance was begun with [`cmd_begin_rendering`], the currently
///   bound graphics pipeline  **must**  have been created with a
///   [`PipelineRenderingCreateInfo::color_attachment_count`] equal to
///   [`RenderingInfo::color_attachment_count`]
/// - If the current render pass instance was begun with [`cmd_begin_rendering`] and
///   [`RenderingInfo::color_attachment_count`] greater than `0`, then each element of the
///   [`RenderingInfo::color_attachments`] array with a `imageView` not equal to
///   [`crate::Handle::null`] **must**  have been created with a [`Format`] equal to the
///   corresponding element of [`PipelineRenderingCreateInfo::color_attachment_formats`] used to
///   create the currently bound graphics pipeline
/// - If the bound graphics pipeline state was created with the
///   `VK_DYNAMIC_STATE_COLOR_WRITE_ENABLE_EXT` dynamic state enabled then
///   [`cmd_set_color_write_enable_ext`] **must**  have been called in the current command buffer
///   prior to this drawing command, and the `attachmentCount` parameter of
///   [`cmd_set_color_write_enable_ext`] **must**  be equal to the
///   [`PipelineColorBlendStateCreateInfo::attachment_count`] of the currently bound graphics
///   pipeline
/// - If the current render pass instance was begun with [`cmd_begin_rendering`] and
///   [`RenderingInfo`]::`pDepthAttachment->pname`:imageView was not [`crate::Handle::null`], the
///   value of [`PipelineRenderingCreateInfo::depth_attachment_format`] used to create the currently
///   bound graphics pipeline  **must**  be equal to the [`Format`] used to create
///   [`RenderingInfo`]::`pDepthAttachment->pname`:imageView
/// - If the current render pass instance was begun with [`cmd_begin_rendering`] and
///   [`RenderingInfo`]::`pStencilAttachment->pname`:imageView was not [`crate::Handle::null`], the
///   value of [`PipelineRenderingCreateInfo::stencil_attachment_format`] used to create the
///   currently bound graphics pipeline  **must**  be equal to the [`Format`] used to create
///   [`RenderingInfo`]::`pStencilAttachment->pname`:imageView
/// - If the current render pass instance was begun with [`cmd_begin_rendering`] and
///   [`RenderingFragmentShadingRateAttachmentInfoKHR::image_view`] was not [`crate::Handle::null`],
///   the currently bound graphics pipeline  **must**  have been created with
///   `VK_PIPELINE_CREATE_RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR`
/// - If the current render pass instance was begun with [`cmd_begin_rendering`] and
///   [`RenderingFragmentDensityMapAttachmentInfoEXT::image_view`] was not [`crate::Handle::null`],
///   the currently bound graphics pipeline  **must**  have been created with
///   `VK_PIPELINE_CREATE_RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_BIT_EXT`
/// - If the currently bound pipeline was created with a [`AttachmentSampleCountInfoAMD`] or
///   [`AttachmentSampleCountInfoNV`] structure, and the current render pass instance was begun with
///   [`cmd_begin_rendering`] with a [`RenderingInfo::color_attachment_count`] parameter greater
///   than `0`, then each element of the [`RenderingInfo::color_attachments`] array with a
///   `imageView` not equal to [`crate::Handle::null`] **must**  have been created with a sample
///   count equal to the corresponding element of the `pColorAttachmentSamples` member of
///   [`AttachmentSampleCountInfoAMD`] or [`AttachmentSampleCountInfoNV`] used to create the
///   currently bound graphics pipeline
/// - If the current render pass instance was begun with [`cmd_begin_rendering`], the currently
///   bound pipeline was created with a [`AttachmentSampleCountInfoAMD`] or
///   [`AttachmentSampleCountInfoNV`] structure, and
///   [`RenderingInfo`]::`pDepthAttachment->pname`:imageView was not [`crate::Handle::null`], the
///   value of the `depthStencilAttachmentSamples` member of [`AttachmentSampleCountInfoAMD`] or
///   [`AttachmentSampleCountInfoNV`] used to create the currently bound graphics pipeline  **must**
///   be equal to the sample count used to create
///   [`RenderingInfo`]::`pDepthAttachment->pname`:imageView
/// - If the current render pass instance was begun with [`cmd_begin_rendering`], the currently
///   bound pipeline was created with a [`AttachmentSampleCountInfoAMD`] or
///   [`AttachmentSampleCountInfoNV`] structure, and
///   [`RenderingInfo`]::`pStencilAttachment->pname`:imageView was not [`crate::Handle::null`], the
///   value of the `depthStencilAttachmentSamples` member of [`AttachmentSampleCountInfoAMD`] or
///   [`AttachmentSampleCountInfoNV`] used to create the currently bound graphics pipeline  **must**
///   be equal to the sample count used to create
///   [`RenderingInfo`]::`pStencilAttachment->pname`:imageView
/// - If the currently bound pipeline was created without a [`AttachmentSampleCountInfoAMD`] or
///   [`AttachmentSampleCountInfoNV`] structure, and the current render pass instance was begun with
///   [`cmd_begin_rendering`] with a [`RenderingInfo::color_attachment_count`] parameter greater
///   than `0`, then each element of the [`RenderingInfo::color_attachments`] array with a
///   `imageView` not equal to [`crate::Handle::null`] **must**  have been created with a sample
///   count equal to the value of [`PipelineMultisampleStateCreateInfo::rasterization_samples`] used
///   to create the currently bound graphics pipeline
/// - If the current render pass instance was begun with [`cmd_begin_rendering`], the currently
///   bound pipeline was created without a [`AttachmentSampleCountInfoAMD`] or
///   [`AttachmentSampleCountInfoNV`] structure, and
///   [`RenderingInfo`]::`pDepthAttachment->pname`:imageView was not [`crate::Handle::null`], the
///   value of [`PipelineMultisampleStateCreateInfo::rasterization_samples`] used to create the
///   currently bound graphics pipeline  **must**  be equal to the sample count used to create
///   [`RenderingInfo`]::`pDepthAttachment->pname`:imageView
/// - If the current render pass instance was begun with [`cmd_begin_rendering`], the currently
///   bound pipeline was created without a [`AttachmentSampleCountInfoAMD`] or
///   [`AttachmentSampleCountInfoNV`] structure, and
///   [`RenderingInfo`]::`pStencilAttachment->pname`:imageView was not [`crate::Handle::null`], the
///   value of [`PipelineMultisampleStateCreateInfo::rasterization_samples`] used to create the
///   currently bound graphics pipeline  **must**  be equal to the sample count used to create
///   [`RenderingInfo`]::`pStencilAttachment->pname`:imageView
/// - If the current render pass instance was begun with [`cmd_begin_rendering`], the currently
///   bound pipeline  **must**  have been created with a [`GraphicsPipelineCreateInfo::render_pass`]
///   equal to [`crate::Handle::null`]
///
/// - The bound graphics pipeline  **must**  not have been created with the
///   [`PipelineShaderStageCreateInfo::stage`] member of an element of
///   [`GraphicsPipelineCreateInfo::stages`] set to `VK_SHADER_STAGE_VERTEX_BIT`,
///   `VK_SHADER_STAGE_TESSELLATION_CONTROL_BIT`, `VK_SHADER_STAGE_TESSELLATION_EVALUATION_BIT` or
///   `VK_SHADER_STAGE_GEOMETRY_BIT`
///
/// - If [`buffer`] is non-sparse then it  **must**  be bound completely and contiguously to a
///   single [`DeviceMemory`] object
/// - [`buffer`] **must**  have been created with the `VK_BUFFER_USAGE_INDIRECT_BUFFER_BIT` bit set
/// - [`offset`] **must**  be a multiple of `4`
/// - [`command_buffer`] **must**  not be a protected command buffer
///
/// - If [`count_buffer`] is non-sparse then it  **must**  be bound completely and contiguously to a
///   single [`DeviceMemory`] object
/// - [`count_buffer`] **must**  have been created with the `VK_BUFFER_USAGE_INDIRECT_BUFFER_BIT`
///   bit set
/// - [`count_buffer_offset`] **must**  be a multiple of `4`
/// - The count stored in [`count_buffer`] **must**  be less than or equal to
///   [`PhysicalDeviceLimits::max_draw_indirect_count`]
/// - ([`count_buffer_offset`] +  `sizeof`(uint32_t)) **must**  be less than or equal to the size of
///   [`count_buffer`]
/// - If [drawIndirectCount]() is not enabled this function  **must**  not be used
/// - [`stride`] **must**  be a multiple of `4` and  **must**  be greater than or equal to
///   `sizeof`([`DrawMeshTasksIndirectCommandNV`])
/// - If [`max_draw_count`] is greater than or equal to `1`, ([`stride`] × ([`max_draw_count`] - 1)
///   +  [`offset`] +  `sizeof`([`DrawMeshTasksIndirectCommandNV`])) **must**  be less than or equal
///   to the size of [`buffer`]
/// - If the count stored in [`count_buffer`] is equal to `1`, ([`offset`] +
///   `sizeof`([`DrawMeshTasksIndirectCommandNV`])) **must**  be less than or equal to the size of
///   [`buffer`]
/// - If the count stored in [`count_buffer`] is greater than `1`, ([`stride`] × (`drawCount` - 1) +
///   [`offset`] +  `sizeof`([`DrawMeshTasksIndirectCommandNV`])) **must**  be less than or equal to
///   the size of [`buffer`]
///
///## Valid Usage (Implicit)
/// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
/// - [`buffer`] **must**  be a valid [`Buffer`] handle
/// - [`count_buffer`] **must**  be a valid [`Buffer`] handle
/// - [`command_buffer`] **must**  be in the [recording state]()
/// - The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics
///   operations
/// - This command  **must**  only be called inside of a render pass instance
/// - Each of [`buffer`], [`command_buffer`], and [`count_buffer`] **must**  have been created,
///   allocated, or retrieved from the same [`Device`]
///
///## Host Synchronization
/// - Host access to [`command_buffer`] **must**  be externally synchronized
/// - Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be
///   externally synchronized
///
///## Command Properties
///# Related
/// - [`VK_NV_mesh_shader`]
/// - [`Buffer`]
/// - [`CommandBuffer`]
/// - [`DeviceSize`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkCmdDrawMeshTasksIndirectCountNV")]
pub type FNCmdDrawMeshTasksIndirectCountNv = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        count_buffer: Buffer,
        count_buffer_offset: DeviceSize,
        max_draw_count: u32,
        stride: u32,
    ),
>;
///[VkPhysicalDeviceMeshShaderFeaturesNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMeshShaderFeaturesNV.html) - Structure describing mesh shading features that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceMeshShaderFeaturesNV`] structure is defined as:
///```c
///// Provided by VK_NV_mesh_shader
///typedef struct VkPhysicalDeviceMeshShaderFeaturesNV {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           taskShader;
///    VkBool32           meshShader;
///} VkPhysicalDeviceMeshShaderFeaturesNV;
///```
///# Members
///This structure describes the following features:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`task_shader`] indicates whether the task shader stage is supported.
/// - [`mesh_shader`] indicates whether the mesh shader stage is supported.
///If the [`PhysicalDeviceMeshShaderFeaturesNV`] structure is included in the [`p_next`] chain of
/// the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`get_physical_device_features2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceMeshShaderFeaturesNV`] **can**  also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MESH_SHADER_FEATURES_NV`
///# Related
/// - [`VK_NV_mesh_shader`]
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
#[doc(alias = "VkPhysicalDeviceMeshShaderFeaturesNV")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceMeshShaderFeaturesNV<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`task_shader`] indicates whether the task
    ///shader stage is supported.
    pub task_shader: Bool32,
    ///[`mesh_shader`] indicates whether the mesh
    ///shader stage is supported.
    pub mesh_shader: Bool32,
}
impl<'lt> Default for PhysicalDeviceMeshShaderFeaturesNV<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PHYSICAL_DEVICE_MESH_SHADER_FEATURES_NV,
            p_next: std::ptr::null_mut(),
            task_shader: 0,
            mesh_shader: 0,
        }
    }
}
impl<'lt> PhysicalDeviceMeshShaderFeaturesNV<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *mut BaseOutStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::task_shader`]
    pub fn task_shader_raw(&self) -> Bool32 {
        self.task_shader
    }
    ///Gets the raw value of [`Self::mesh_shader`]
    pub fn mesh_shader_raw(&self) -> Bool32 {
        self.mesh_shader
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *mut BaseOutStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::task_shader`]
    pub fn set_task_shader_raw(mut self, value: Bool32) -> Self {
        self.task_shader = value;
        self
    }
    ///Sets the raw value of [`Self::mesh_shader`]
    pub fn set_mesh_shader_raw(mut self, value: Bool32) -> Self {
        self.mesh_shader = value;
        self
    }
    ///Gets the value of [`Self::s_type`]
    pub fn s_type(&self) -> StructureType {
        self.s_type
    }
    ///Gets the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next(&self) -> &BaseOutStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::task_shader`]
    pub fn task_shader(&self) -> bool {
        unsafe { std::mem::transmute(self.task_shader as u8) }
    }
    ///Gets the value of [`Self::mesh_shader`]
    pub fn mesh_shader(&self) -> bool {
        unsafe { std::mem::transmute(self.mesh_shader as u8) }
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next_mut(&mut self) -> &mut BaseOutStructure<'lt> {
        &mut *self.p_next
    }
    ///Gets a mutable reference to the value of [`Self::task_shader`]
    pub fn task_shader_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.task_shader as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.task_shader as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::mesh_shader`]
    pub fn mesh_shader_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.mesh_shader as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.mesh_shader as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the value of [`Self::task_shader`]
    pub fn set_task_shader(mut self, value: bool) -> Self {
        self.task_shader = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::mesh_shader`]
    pub fn set_mesh_shader(mut self, value: bool) -> Self {
        self.mesh_shader = value as u8 as u32;
        self
    }
}
///[VkPhysicalDeviceMeshShaderPropertiesNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMeshShaderPropertiesNV.html) - Structure describing mesh shading properties
///# C Specifications
///The [`PhysicalDeviceMeshShaderPropertiesNV`] structure is defined as:
///```c
///// Provided by VK_NV_mesh_shader
///typedef struct VkPhysicalDeviceMeshShaderPropertiesNV {
///    VkStructureType    sType;
///    void*              pNext;
///    uint32_t           maxDrawMeshTasksCount;
///    uint32_t           maxTaskWorkGroupInvocations;
///    uint32_t           maxTaskWorkGroupSize[3];
///    uint32_t           maxTaskTotalMemorySize;
///    uint32_t           maxTaskOutputCount;
///    uint32_t           maxMeshWorkGroupInvocations;
///    uint32_t           maxMeshWorkGroupSize[3];
///    uint32_t           maxMeshTotalMemorySize;
///    uint32_t           maxMeshOutputVertices;
///    uint32_t           maxMeshOutputPrimitives;
///    uint32_t           maxMeshMultiviewViewCount;
///    uint32_t           meshOutputPerVertexGranularity;
///    uint32_t           meshOutputPerPrimitiveGranularity;
///} VkPhysicalDeviceMeshShaderPropertiesNV;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`max_draw_mesh_tasks_count`] is the maximum number of local workgroups that  **can**  be launched by a single draw mesh tasks command. See [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#drawing-mesh-shading](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#drawing-mesh-shading).
/// - [`max_task_work_group_invocations`] is the maximum total number of task     shader invocations
///   in a single local workgroup.     The product of the X, Y, and Z sizes, as specified by the
///   `LocalSize` or `LocalSizeId`     execution mode in shader modules or by the object decorated
///   by the     `WorkgroupSize` decoration,  **must**  be less than or equal to this     limit.
/// - [`max_task_work_group_size`][3] is the maximum size of a local task     workgroup.     These
///   three values represent the maximum local workgroup size in the X,     Y, and Z dimensions,
///   respectively.     The `x`, `y`, and `z` sizes, as specified by the     `LocalSize` or
///   `LocalSizeId`     execution mode or by the object decorated by the `WorkgroupSize`
///   decoration in shader modules,  **must**  be less than or equal to the     corresponding limit.
/// - [`max_task_total_memory_size`] is the maximum number of bytes that the task shader can use in
///   total for shared and output memory combined.
/// - [`max_task_output_count`] is the maximum number of output tasks a single task shader workgroup
///   can emit.
/// - [`max_mesh_work_group_invocations`] is the maximum total number of mesh     shader invocations
///   in a single local workgroup.     The product of the X, Y, and Z sizes, as specified by the
///   `LocalSize` or `LocalSizeId`     execution mode in shader modules or by the object decorated
///   by the     `WorkgroupSize` decoration,  **must**  be less than or equal to this     limit.
/// - [`max_mesh_work_group_size`][3] is the maximum size of a local mesh     workgroup.     These
///   three values represent the maximum local workgroup size in the X,     Y, and Z dimensions,
///   respectively.     The `x`, `y`, and `z` sizes, as specified by the     `LocalSize` or
///   `LocalSizeId`     execution mode or by the object decorated by the `WorkgroupSize`
///   decoration in shader modules,  **must**  be less than or equal to the     corresponding limit.
/// - [`max_mesh_total_memory_size`] is the maximum number of bytes that the mesh shader can use in
///   total for shared and output memory combined.
/// - [`max_mesh_output_vertices`] is the maximum number of vertices a mesh shader output can store.
/// - [`max_mesh_output_primitives`] is the maximum number of primitives a mesh shader output can
///   store.
/// - [`max_mesh_multiview_view_count`] is the maximum number of multi-view views a mesh shader can
///   use.
/// - [`mesh_output_per_vertex_granularity`] is the granularity with which mesh vertex outputs are
///   allocated. The value can be used to compute the memory size used by the mesh shader, which
///   must be less than or equal to [`max_mesh_total_memory_size`].
/// - [`mesh_output_per_primitive_granularity`] is the granularity with which mesh outputs qualified
///   as per-primitive are allocated. The value can be used to compute the memory size used by the
///   mesh shader, which must be less than or equal to [`max_mesh_total_memory_size`].
///# Description
///If the [`PhysicalDeviceMeshShaderPropertiesNV`] structure is included in the [`p_next`] chain of
/// the
///[`PhysicalDeviceProperties2`] structure passed to
///[`get_physical_device_properties2`], it is filled in with each
///corresponding implementation-dependent property.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MESH_SHADER_PROPERTIES_NV`
///# Related
/// - [`VK_NV_mesh_shader`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPhysicalDeviceMeshShaderPropertiesNV")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceMeshShaderPropertiesNV<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`max_draw_mesh_tasks_count`] is the maximum number of local workgroups
    ///that  **can**  be launched by a single draw mesh tasks command.
    ///See [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#drawing-mesh-shading](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#drawing-mesh-shading).
    pub max_draw_mesh_tasks_count: u32,
    ///[`max_task_work_group_invocations`] is the maximum total number of task
    ///    shader invocations in a single local workgroup.
    ///    The product of the X, Y, and Z sizes, as specified by the `LocalSize`
    ///or `LocalSizeId`
    ///    execution mode in shader modules or by the object decorated by the
    ///    `WorkgroupSize` decoration,  **must**  be less than or equal to this
    ///    limit.
    pub max_task_work_group_invocations: u32,
    ///[`max_task_work_group_size`][3] is the maximum size of a local task
    ///    workgroup.
    ///    These three values represent the maximum local workgroup size in the X,
    ///    Y, and Z dimensions, respectively.
    ///    The `x`, `y`, and `z` sizes, as specified by the
    ///    `LocalSize`
    ///or `LocalSizeId`
    ///    execution mode or by the object decorated by the `WorkgroupSize`
    ///    decoration in shader modules,  **must**  be less than or equal to the
    ///    corresponding limit.
    pub max_task_work_group_size: [u32; 3 as usize],
    ///[`max_task_total_memory_size`] is the maximum number of bytes that the
    ///task shader can use in total for shared and output memory combined.
    pub max_task_total_memory_size: u32,
    ///[`max_task_output_count`] is the maximum number of output tasks a single
    ///task shader workgroup can emit.
    pub max_task_output_count: u32,
    ///[`max_mesh_work_group_invocations`] is the maximum total number of mesh
    ///    shader invocations in a single local workgroup.
    ///    The product of the X, Y, and Z sizes, as specified by the `LocalSize`
    ///or `LocalSizeId`
    ///    execution mode in shader modules or by the object decorated by the
    ///    `WorkgroupSize` decoration,  **must**  be less than or equal to this
    ///    limit.
    pub max_mesh_work_group_invocations: u32,
    ///[`max_mesh_work_group_size`][3] is the maximum size of a local mesh
    ///    workgroup.
    ///    These three values represent the maximum local workgroup size in the X,
    ///    Y, and Z dimensions, respectively.
    ///    The `x`, `y`, and `z` sizes, as specified by the
    ///    `LocalSize`
    ///or `LocalSizeId`
    ///    execution mode or by the object decorated by the `WorkgroupSize`
    ///    decoration in shader modules,  **must**  be less than or equal to the
    ///    corresponding limit.
    pub max_mesh_work_group_size: [u32; 3 as usize],
    ///[`max_mesh_total_memory_size`] is the maximum number of bytes that the
    ///mesh shader can use in total for shared and output memory combined.
    pub max_mesh_total_memory_size: u32,
    ///[`max_mesh_output_vertices`] is the maximum number of vertices a mesh
    ///shader output can store.
    pub max_mesh_output_vertices: u32,
    ///[`max_mesh_output_primitives`] is the maximum number of primitives a mesh
    ///shader output can store.
    pub max_mesh_output_primitives: u32,
    ///[`max_mesh_multiview_view_count`] is the maximum number of multi-view
    ///views a mesh shader can use.
    pub max_mesh_multiview_view_count: u32,
    ///[`mesh_output_per_vertex_granularity`] is the granularity with which mesh
    ///vertex outputs are allocated.
    ///The value can be used to compute the memory size used by the mesh
    ///shader, which must be less than or equal to
    ///[`max_mesh_total_memory_size`].
    pub mesh_output_per_vertex_granularity: u32,
    ///[`mesh_output_per_primitive_granularity`] is the granularity with which
    ///mesh outputs qualified as per-primitive are allocated.
    ///The value can be used to compute the memory size used by the mesh
    ///shader, which must be less than or equal to
    ///[`max_mesh_total_memory_size`].
    pub mesh_output_per_primitive_granularity: u32,
}
impl<'lt> Default for PhysicalDeviceMeshShaderPropertiesNV<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PHYSICAL_DEVICE_MESH_SHADER_PROPERTIES_NV,
            p_next: std::ptr::null_mut(),
            max_draw_mesh_tasks_count: 0,
            max_task_work_group_invocations: 0,
            max_task_work_group_size: [0; 3 as usize],
            max_task_total_memory_size: 0,
            max_task_output_count: 0,
            max_mesh_work_group_invocations: 0,
            max_mesh_work_group_size: [0; 3 as usize],
            max_mesh_total_memory_size: 0,
            max_mesh_output_vertices: 0,
            max_mesh_output_primitives: 0,
            max_mesh_multiview_view_count: 0,
            mesh_output_per_vertex_granularity: 0,
            mesh_output_per_primitive_granularity: 0,
        }
    }
}
impl<'lt> PhysicalDeviceMeshShaderPropertiesNV<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *mut BaseOutStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *mut BaseOutStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Gets the value of [`Self::s_type`]
    pub fn s_type(&self) -> StructureType {
        self.s_type
    }
    ///Gets the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next(&self) -> &BaseOutStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::max_draw_mesh_tasks_count`]
    pub fn max_draw_mesh_tasks_count(&self) -> u32 {
        self.max_draw_mesh_tasks_count
    }
    ///Gets the value of [`Self::max_task_work_group_invocations`]
    pub fn max_task_work_group_invocations(&self) -> u32 {
        self.max_task_work_group_invocations
    }
    ///Gets the value of [`Self::max_task_work_group_size`]
    pub fn max_task_work_group_size(&self) -> &[u32; 3 as usize] {
        &self.max_task_work_group_size
    }
    ///Gets the value of [`Self::max_task_total_memory_size`]
    pub fn max_task_total_memory_size(&self) -> u32 {
        self.max_task_total_memory_size
    }
    ///Gets the value of [`Self::max_task_output_count`]
    pub fn max_task_output_count(&self) -> u32 {
        self.max_task_output_count
    }
    ///Gets the value of [`Self::max_mesh_work_group_invocations`]
    pub fn max_mesh_work_group_invocations(&self) -> u32 {
        self.max_mesh_work_group_invocations
    }
    ///Gets the value of [`Self::max_mesh_work_group_size`]
    pub fn max_mesh_work_group_size(&self) -> &[u32; 3 as usize] {
        &self.max_mesh_work_group_size
    }
    ///Gets the value of [`Self::max_mesh_total_memory_size`]
    pub fn max_mesh_total_memory_size(&self) -> u32 {
        self.max_mesh_total_memory_size
    }
    ///Gets the value of [`Self::max_mesh_output_vertices`]
    pub fn max_mesh_output_vertices(&self) -> u32 {
        self.max_mesh_output_vertices
    }
    ///Gets the value of [`Self::max_mesh_output_primitives`]
    pub fn max_mesh_output_primitives(&self) -> u32 {
        self.max_mesh_output_primitives
    }
    ///Gets the value of [`Self::max_mesh_multiview_view_count`]
    pub fn max_mesh_multiview_view_count(&self) -> u32 {
        self.max_mesh_multiview_view_count
    }
    ///Gets the value of [`Self::mesh_output_per_vertex_granularity`]
    pub fn mesh_output_per_vertex_granularity(&self) -> u32 {
        self.mesh_output_per_vertex_granularity
    }
    ///Gets the value of [`Self::mesh_output_per_primitive_granularity`]
    pub fn mesh_output_per_primitive_granularity(&self) -> u32 {
        self.mesh_output_per_primitive_granularity
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next_mut(&mut self) -> &mut BaseOutStructure<'lt> {
        &mut *self.p_next
    }
    ///Gets a mutable reference to the value of [`Self::max_draw_mesh_tasks_count`]
    pub fn max_draw_mesh_tasks_count_mut(&mut self) -> &mut u32 {
        &mut self.max_draw_mesh_tasks_count
    }
    ///Gets a mutable reference to the value of [`Self::max_task_work_group_invocations`]
    pub fn max_task_work_group_invocations_mut(&mut self) -> &mut u32 {
        &mut self.max_task_work_group_invocations
    }
    ///Gets a mutable reference to the value of [`Self::max_task_work_group_size`]
    pub fn max_task_work_group_size_mut(&mut self) -> &mut [u32; 3 as usize] {
        &mut self.max_task_work_group_size
    }
    ///Gets a mutable reference to the value of [`Self::max_task_total_memory_size`]
    pub fn max_task_total_memory_size_mut(&mut self) -> &mut u32 {
        &mut self.max_task_total_memory_size
    }
    ///Gets a mutable reference to the value of [`Self::max_task_output_count`]
    pub fn max_task_output_count_mut(&mut self) -> &mut u32 {
        &mut self.max_task_output_count
    }
    ///Gets a mutable reference to the value of [`Self::max_mesh_work_group_invocations`]
    pub fn max_mesh_work_group_invocations_mut(&mut self) -> &mut u32 {
        &mut self.max_mesh_work_group_invocations
    }
    ///Gets a mutable reference to the value of [`Self::max_mesh_work_group_size`]
    pub fn max_mesh_work_group_size_mut(&mut self) -> &mut [u32; 3 as usize] {
        &mut self.max_mesh_work_group_size
    }
    ///Gets a mutable reference to the value of [`Self::max_mesh_total_memory_size`]
    pub fn max_mesh_total_memory_size_mut(&mut self) -> &mut u32 {
        &mut self.max_mesh_total_memory_size
    }
    ///Gets a mutable reference to the value of [`Self::max_mesh_output_vertices`]
    pub fn max_mesh_output_vertices_mut(&mut self) -> &mut u32 {
        &mut self.max_mesh_output_vertices
    }
    ///Gets a mutable reference to the value of [`Self::max_mesh_output_primitives`]
    pub fn max_mesh_output_primitives_mut(&mut self) -> &mut u32 {
        &mut self.max_mesh_output_primitives
    }
    ///Gets a mutable reference to the value of [`Self::max_mesh_multiview_view_count`]
    pub fn max_mesh_multiview_view_count_mut(&mut self) -> &mut u32 {
        &mut self.max_mesh_multiview_view_count
    }
    ///Gets a mutable reference to the value of [`Self::mesh_output_per_vertex_granularity`]
    pub fn mesh_output_per_vertex_granularity_mut(&mut self) -> &mut u32 {
        &mut self.mesh_output_per_vertex_granularity
    }
    ///Gets a mutable reference to the value of [`Self::mesh_output_per_primitive_granularity`]
    pub fn mesh_output_per_primitive_granularity_mut(&mut self) -> &mut u32 {
        &mut self.mesh_output_per_primitive_granularity
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the value of [`Self::max_draw_mesh_tasks_count`]
    pub fn set_max_draw_mesh_tasks_count(mut self, value: u32) -> Self {
        self.max_draw_mesh_tasks_count = value;
        self
    }
    ///Sets the value of [`Self::max_task_work_group_invocations`]
    pub fn set_max_task_work_group_invocations(mut self, value: u32) -> Self {
        self.max_task_work_group_invocations = value;
        self
    }
    ///Sets the value of [`Self::max_task_work_group_size`]
    pub fn set_max_task_work_group_size(mut self, value: [u32; 3 as usize]) -> Self {
        self.max_task_work_group_size = value;
        self
    }
    ///Sets the value of [`Self::max_task_total_memory_size`]
    pub fn set_max_task_total_memory_size(mut self, value: u32) -> Self {
        self.max_task_total_memory_size = value;
        self
    }
    ///Sets the value of [`Self::max_task_output_count`]
    pub fn set_max_task_output_count(mut self, value: u32) -> Self {
        self.max_task_output_count = value;
        self
    }
    ///Sets the value of [`Self::max_mesh_work_group_invocations`]
    pub fn set_max_mesh_work_group_invocations(mut self, value: u32) -> Self {
        self.max_mesh_work_group_invocations = value;
        self
    }
    ///Sets the value of [`Self::max_mesh_work_group_size`]
    pub fn set_max_mesh_work_group_size(mut self, value: [u32; 3 as usize]) -> Self {
        self.max_mesh_work_group_size = value;
        self
    }
    ///Sets the value of [`Self::max_mesh_total_memory_size`]
    pub fn set_max_mesh_total_memory_size(mut self, value: u32) -> Self {
        self.max_mesh_total_memory_size = value;
        self
    }
    ///Sets the value of [`Self::max_mesh_output_vertices`]
    pub fn set_max_mesh_output_vertices(mut self, value: u32) -> Self {
        self.max_mesh_output_vertices = value;
        self
    }
    ///Sets the value of [`Self::max_mesh_output_primitives`]
    pub fn set_max_mesh_output_primitives(mut self, value: u32) -> Self {
        self.max_mesh_output_primitives = value;
        self
    }
    ///Sets the value of [`Self::max_mesh_multiview_view_count`]
    pub fn set_max_mesh_multiview_view_count(mut self, value: u32) -> Self {
        self.max_mesh_multiview_view_count = value;
        self
    }
    ///Sets the value of [`Self::mesh_output_per_vertex_granularity`]
    pub fn set_mesh_output_per_vertex_granularity(mut self, value: u32) -> Self {
        self.mesh_output_per_vertex_granularity = value;
        self
    }
    ///Sets the value of [`Self::mesh_output_per_primitive_granularity`]
    pub fn set_mesh_output_per_primitive_granularity(mut self, value: u32) -> Self {
        self.mesh_output_per_primitive_granularity = value;
        self
    }
}
///[VkDrawMeshTasksIndirectCommandNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDrawMeshTasksIndirectCommandNV.html) - Structure specifying a mesh tasks draw indirect command
///# C Specifications
///The [`DrawMeshTasksIndirectCommandNV`] structure is defined as:
///```c
///// Provided by VK_NV_mesh_shader
///typedef struct VkDrawMeshTasksIndirectCommandNV {
///    uint32_t    taskCount;
///    uint32_t    firstTask;
///} VkDrawMeshTasksIndirectCommandNV;
///```
///# Members
/// - [`task_count`] is the number of local workgroups to dispatch in the X dimension. Y and Z
///   dimension are implicitly set to one.
/// - [`first_task`] is the X component of the first workgroup ID.
///# Description
///The members of [`DrawMeshTasksIndirectCommandNV`] have the same meaning
///as the similarly named parameters of [`cmd_draw_mesh_tasks_nv`].
///## Valid Usage
/// - [`task_count`] **must**  be less than or equal to
///   [`PhysicalDeviceMeshShaderPropertiesNV::max_draw_mesh_tasks_count`]
///# Related
/// - [`VK_NV_mesh_shader`]
/// - [`cmd_draw_mesh_tasks_indirect_nv`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkDrawMeshTasksIndirectCommandNV")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct DrawMeshTasksIndirectCommandNV {
    ///[`task_count`] is the number of local workgroups to dispatch in the X
    ///dimension.
    ///Y and Z dimension are implicitly set to one.
    pub task_count: u32,
    ///[`first_task`] is the X component of the first workgroup ID.
    pub first_task: u32,
}
impl Default for DrawMeshTasksIndirectCommandNV {
    fn default() -> Self {
        Self {
            task_count: 0,
            first_task: 0,
        }
    }
}
impl DrawMeshTasksIndirectCommandNV {
    ///Gets the value of [`Self::task_count`]
    pub fn task_count(&self) -> u32 {
        self.task_count
    }
    ///Gets the value of [`Self::first_task`]
    pub fn first_task(&self) -> u32 {
        self.first_task
    }
    ///Gets a mutable reference to the value of [`Self::task_count`]
    pub fn task_count_mut(&mut self) -> &mut u32 {
        &mut self.task_count
    }
    ///Gets a mutable reference to the value of [`Self::first_task`]
    pub fn first_task_mut(&mut self) -> &mut u32 {
        &mut self.first_task
    }
    ///Sets the value of [`Self::task_count`]
    pub fn set_task_count(mut self, value: u32) -> Self {
        self.task_count = value;
        self
    }
    ///Sets the value of [`Self::first_task`]
    pub fn set_first_task(mut self, value: u32) -> Self {
        self.first_task = value;
        self
    }
}
impl CommandBuffer {
    ///[vkCmdDrawMeshTasksNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawMeshTasksNV.html) - Draw mesh task work items
    ///# C Specifications
    ///To record a draw that uses the mesh pipeline, call:
    ///```c
    ///// Provided by VK_NV_mesh_shader
    ///void vkCmdDrawMeshTasksNV(
    ///    VkCommandBuffer                             commandBuffer,
    ///    uint32_t                                    taskCount,
    ///    uint32_t                                    firstTask);
    ///```
    ///# Parameters
    /// - [`command_buffer`] is the command buffer into which the command will be recorded.
    /// - [`task_count`] is the number of local workgroups to dispatch in the X dimension. Y and Z
    ///   dimension are implicitly set to one.
    /// - [`first_task`] is the X component of the first workgroup ID.
    ///# Description
    ///When the command is executed, a global workgroup consisting of
    ///[`task_count`] local workgroups is assembled.
    ///## Valid Usage
    /// - If a [`Sampler`] created with `magFilter` or `minFilter` equal to `VK_FILTER_LINEAR` and
    ///   `compareEnable` equal to [`FALSE`] is used to sample a [`ImageView`] as a result of this
    ///   command, then the image view’s [format features]() **must**  contain
    ///   `VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT`
    /// - If a [`Sampler`] created with `mipmapMode` equal to `VK_SAMPLER_MIPMAP_MODE_LINEAR` and
    ///   `compareEnable` equal to [`FALSE`] is used to sample a [`ImageView`] as a result of this
    ///   command, then the image view’s [format features]() **must**  contain
    ///   `VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT`
    /// - If a [`ImageView`] is sampled with [depth comparison](), the image view’s [format
    ///   features]() **must**  contain `VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_DEPTH_COMPARISON_BIT`
    /// - If a [`ImageView`] is accessed using atomic operations as a result of this command, then
    ///   the image view’s [format features]() **must**  contain
    ///   `VK_FORMAT_FEATURE_STORAGE_IMAGE_ATOMIC_BIT`
    /// - If a [`ImageView`] is sampled with `VK_FILTER_CUBIC_EXT` as a result of this command, then
    ///   the image view’s [format features]() **must**  contain
    ///   `VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_CUBIC_BIT_EXT`
    /// - Any [`ImageView`] being sampled with `VK_FILTER_CUBIC_EXT` as a result of this command
    ///   **must**  have a [`ImageViewType`] and format that supports cubic filtering, as specified
    ///   by [`FilterCubicImageViewImageFormatPropertiesEXT::filter_cubic`] returned by
    ///   [`get_physical_device_image_format_properties2`]
    /// - Any [`ImageView`] being sampled with `VK_FILTER_CUBIC_EXT` with a reduction mode of either
    ///   `VK_SAMPLER_REDUCTION_MODE_MIN` or `VK_SAMPLER_REDUCTION_MODE_MAX` as a result of this
    ///   command  **must**  have a [`ImageViewType`] and format that supports cubic filtering
    ///   together with minmax filtering, as specified by
    ///   [`FilterCubicImageViewImageFormatPropertiesEXT::filter_cubic_minmax`] returned by
    ///   [`get_physical_device_image_format_properties2`]
    /// - Any [`Image`] created with a [`ImageCreateInfo::flags`] containing
    ///   `VK_IMAGE_CREATE_CORNER_SAMPLED_BIT_NV` sampled as a result of this command  **must**
    ///   only be sampled using a [`SamplerAddressMode`] of `VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE`
    /// - Any [`ImageView`] or [`BufferView`] being written as a storage image or storage texel
    ///   buffer where the image format field of the `OpTypeImage` is `Unknown` then the view’s
    ///   format feature  **must**  contain `VK_FORMAT_FEATURE_2_STORAGE_WRITE_WITHOUT_FORMAT_BIT`
    /// - Any [`ImageView`] or [`BufferView`] being read as a storage image or storage texel buffer
    ///   where the image format field of the `OpTypeImage` is `Unknown` then the view’s format
    ///   feature  **must**  contain `VK_FORMAT_FEATURE_2_STORAGE_READ_WITHOUT_FORMAT_BIT`
    /// - For each set *n* that is statically used by the [`Pipeline`] bound to the pipeline bind
    ///   point used by this command, a descriptor set  **must**  have been bound to *n* at the same
    ///   pipeline bind point, with a [`PipelineLayout`] that is compatible for set *n*, with the
    ///   [`PipelineLayout`] used to create the current [`Pipeline`], as described in
    ///   [[descriptorsets-compatibility]]()
    /// - If the [`maintenance4`]() feature is not enabled, then for each push constant that is
    ///   statically used by the [`Pipeline`] bound to the pipeline bind point used by this command,
    ///   a push constant value  **must**  have been set for the same pipeline bind point, with a
    ///   [`PipelineLayout`] that is compatible for push constants, with the [`PipelineLayout`] used
    ///   to create the current [`Pipeline`], as described in [[descriptorsets-compatibility]]()
    /// - Descriptors in each bound descriptor set, specified via [`cmd_bind_descriptor_sets`],
    ///   **must**  be valid if they are statically used by the [`Pipeline`] bound to the pipeline
    ///   bind point used by this command
    /// - A valid pipeline  **must**  be bound to the pipeline bind point used by this command
    /// - If the [`Pipeline`] object bound to the pipeline bind point used by this command requires
    ///   any dynamic state, that state  **must**  have been set or inherited (if the
    ///   `[`VK_NV_inherited_viewport_scissor`]` extension is enabled) for [`command_buffer`], and
    ///   done so after any previously bound pipeline with the corresponding state not specified as
    ///   dynamic
    /// - There  **must**  not have been any calls to dynamic state setting commands for any state
    ///   not specified as dynamic in the [`Pipeline`] object bound to the pipeline bind point used
    ///   by this command, since that pipeline was bound
    /// - If the [`Pipeline`] object bound to the pipeline bind point used by this command accesses
    ///   a [`Sampler`] object that uses unnormalized coordinates, that sampler  **must**  not be
    ///   used to sample from any [`Image`] with a [`ImageView`] of the type
    ///   `VK_IMAGE_VIEW_TYPE_3D`, `VK_IMAGE_VIEW_TYPE_CUBE`, `VK_IMAGE_VIEW_TYPE_1D_ARRAY`,
    ///   `VK_IMAGE_VIEW_TYPE_2D_ARRAY` or `VK_IMAGE_VIEW_TYPE_CUBE_ARRAY`, in any shader stage
    /// - If the [`Pipeline`] object bound to the pipeline bind point used by this command accesses
    ///   a [`Sampler`] object that uses unnormalized coordinates, that sampler  **must**  not be
    ///   used with any of the SPIR-V `OpImageSample*` or `OpImageSparseSample*` instructions with
    ///   `ImplicitLod`, `Dref` or `Proj` in their name, in any shader stage
    /// - If the [`Pipeline`] object bound to the pipeline bind point used by this command accesses
    ///   a [`Sampler`] object that uses unnormalized coordinates, that sampler  **must**  not be
    ///   used with any of the SPIR-V `OpImageSample*` or `OpImageSparseSample*` instructions that
    ///   includes a LOD bias or any offset values, in any shader stage
    /// - If the [robust buffer access]() feature is not enabled, and if the [`Pipeline`] object
    ///   bound to the pipeline bind point used by this command accesses a uniform buffer, it
    ///   **must**  not access values outside of the range of the buffer as specified in the
    ///   descriptor set bound to the same pipeline bind point
    /// - If the [robust buffer access]() feature is not enabled, and if the [`Pipeline`] object
    ///   bound to the pipeline bind point used by this command accesses a storage buffer, it
    ///   **must**  not access values outside of the range of the buffer as specified in the
    ///   descriptor set bound to the same pipeline bind point
    /// - If [`command_buffer`] is an unprotected command buffer and [`protectedNoFault`]() is not
    ///   supported, any resource accessed by the [`Pipeline`] object bound to the pipeline bind
    ///   point used by this command  **must**  not be a protected resource
    /// - If the [`Pipeline`] object bound to the pipeline bind point used by this command accesses
    ///   a [`Sampler`] or [`ImageView`] object that enables [sampler Y′C<sub>B</sub>C<sub>R</sub>
    ///   conversion](), that object  **must**  only be used with `OpImageSample*` or
    ///   `OpImageSparseSample*` instructions
    /// - If the [`Pipeline`] object bound to the pipeline bind point used by this command accesses
    ///   a [`Sampler`] or [`ImageView`] object that enables [sampler Y′C<sub>B</sub>C<sub>R</sub>
    ///   conversion](), that object  **must**  not use the `ConstOffset` and `Offset` operands
    /// - If a [`ImageView`] is accessed using `OpImageWrite` as a result of this command, then the
    ///   `Type` of the `Texel` operand of that instruction  **must**  have at least as many
    ///   components as the image view’s format
    /// - If a [`BufferView`] is accessed using `OpImageWrite` as a result of this command, then the
    ///   `Type` of the `Texel` operand of that instruction  **must**  have at least as many
    ///   components as the buffer view’s format
    /// - If a [`ImageView`] with a [`Format`] that has a 64-bit component width is accessed as a
    ///   result of this command, the `SampledType` of the `OpTypeImage` operand of that instruction
    ///   **must**  have a `Width` of 64
    /// - If a [`ImageView`] with a [`Format`] that has a component width less than 64-bit is
    ///   accessed as a result of this command, the `SampledType` of the `OpTypeImage` operand of
    ///   that instruction  **must**  have a `Width` of 32
    /// - If a [`BufferView`] with a [`Format`] that has a 64-bit component width is accessed as a
    ///   result of this command, the `SampledType` of the `OpTypeImage` operand of that instruction
    ///   **must**  have a `Width` of 64
    /// - If a [`BufferView`] with a [`Format`] that has a component width less than 64-bit is
    ///   accessed as a result of this command, the `SampledType` of the `OpTypeImage` operand of
    ///   that instruction  **must**  have a `Width` of 32
    /// - If the [`sparseImageInt64Atomics`]() feature is not enabled, [`Image`] objects created
    ///   with the `VK_IMAGE_CREATE_SPARSE_RESIDENCY_BIT` flag  **must**  not be accessed by atomic
    ///   instructions through an `OpTypeImage` with a `SampledType` with a `Width` of 64 by this
    ///   command
    /// - If the [`sparseImageInt64Atomics`]() feature is not enabled, [`Buffer`] objects created
    ///   with the `VK_BUFFER_CREATE_SPARSE_RESIDENCY_BIT` flag  **must**  not be accessed by atomic
    ///   instructions through an `OpTypeImage` with a `SampledType` with a `Width` of 64 by this
    ///   command
    /// - The current render pass  **must**  be [compatible]() with the `renderPass` member of the
    ///   [`GraphicsPipelineCreateInfo`] structure specified when creating the [`Pipeline`] bound to
    ///   `VK_PIPELINE_BIND_POINT_GRAPHICS`
    /// - The subpass index of the current render pass  **must**  be equal to the `subpass` member
    ///   of the [`GraphicsPipelineCreateInfo`] structure specified when creating the [`Pipeline`]
    ///   bound to `VK_PIPELINE_BIND_POINT_GRAPHICS`
    /// - Every input attachment used by the current subpass  **must**  be bound to the pipeline via
    ///   a descriptor set
    /// - Memory backing image subresources used as attachments in the current render pass  **must**
    ///   not be written in any way other than as an attachment by this command
    /// - If any recorded command in the current subpass will write to an image subresource as an
    ///   attachment, this command  **must**  not read from the memory backing that image
    ///   subresource in any other way than as an attachment
    /// - If any recorded command in the current subpass will read from an image subresource used as
    ///   an attachment in any way other than as an attachment, this command  **must**  not write to
    ///   that image subresource as an attachment
    /// - If the draw is recorded in a render pass instance with multiview enabled, the maximum
    ///   instance index  **must**  be less than or equal to
    ///   [`PhysicalDeviceMultiviewProperties::max_multiview_instance_index`]
    /// - If the bound graphics pipeline was created with
    ///   [`PipelineSampleLocationsStateCreateInfoEXT::sample_locations_enable`] set to [`TRUE`] and
    ///   the current subpass has a depth/stencil attachment, then that attachment  **must**  have
    ///   been created with the `VK_IMAGE_CREATE_SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_BIT_EXT` bit set
    /// - If the bound graphics pipeline state was created with the
    ///   `VK_DYNAMIC_STATE_SAMPLE_LOCATIONS_EXT` dynamic state enabled then
    ///   [`cmd_set_sample_locations_ext`] **must**  have been called in the current command buffer
    ///   prior to this drawing command
    /// - If the bound graphics pipeline state was created with the
    ///   `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` dynamic state enabled, but not the
    ///   `VK_DYNAMIC_STATE_SCISSOR_WITH_COUNT` dynamic state enabled, then
    ///   [`cmd_set_viewport_with_count`] **must**  have been called in the current command buffer
    ///   prior to this drawing command, and the `viewportCount` parameter of
    ///   [`cmd_set_viewport_with_count`] **must**  match the
    ///   [`PipelineViewportStateCreateInfo::scissor_count`] of the pipeline
    /// - If the bound graphics pipeline state was created with the
    ///   `VK_DYNAMIC_STATE_SCISSOR_WITH_COUNT` dynamic state enabled, but not the
    ///   `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` dynamic state enabled, then
    ///   [`cmd_set_scissor_with_count`] **must**  have been called in the current command buffer
    ///   prior to this drawing command, and the `scissorCount` parameter of
    ///   [`cmd_set_scissor_with_count`] **must**  match the
    ///   [`PipelineViewportStateCreateInfo::viewport_count`] of the pipeline
    /// - If the bound graphics pipeline state was created with both the
    ///   `VK_DYNAMIC_STATE_SCISSOR_WITH_COUNT` and `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` dynamic
    ///   states enabled then both [`cmd_set_viewport_with_count`] and
    ///   [`cmd_set_scissor_with_count`] **must**  have been called in the current command buffer
    ///   prior to this drawing command, and the `viewportCount` parameter of
    ///   [`cmd_set_viewport_with_count`] **must**  match the `scissorCount` parameter of
    ///   [`cmd_set_scissor_with_count`]
    /// - If the bound graphics pipeline state was created with the
    ///   `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` dynamic state enabled, but not the
    ///   `VK_DYNAMIC_STATE_VIEWPORT_W_SCALING_NV` dynamic state enabled, then the bound graphics
    ///   pipeline  **must**  have been created with
    ///   [`PipelineViewportWScalingStateCreateInfoNV::viewport_count`] greater or equal to the
    ///   `viewportCount` parameter in the last call to [`cmd_set_viewport_with_count`]
    /// - If the bound graphics pipeline state was created with the
    ///   `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` and `VK_DYNAMIC_STATE_VIEWPORT_W_SCALING_NV`
    ///   dynamic states enabled then the `viewportCount` parameter in the last call to
    ///   [`cmd_set_viewport_w_scaling_nv`] **must**  be greater than or equal to the
    ///   `viewportCount` parameter in the last call to [`cmd_set_viewport_with_count`]
    /// - If the bound graphics pipeline state was created with the
    ///   `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` dynamic state enabled, but not the
    ///   `VK_DYNAMIC_STATE_VIEWPORT_SHADING_RATE_PALETTE_NV` dynamic state enabled, then the bound
    ///   graphics pipeline  **must**  have been created with
    ///   [`PipelineViewportShadingRateImageStateCreateInfoNV::viewport_count`] greater or equal to
    ///   the `viewportCount` parameter in the last call to [`cmd_set_viewport_with_count`]
    /// - If the bound graphics pipeline state was created with the
    ///   `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` and
    ///   `VK_DYNAMIC_STATE_VIEWPORT_SHADING_RATE_PALETTE_NV` dynamic states enabled then the
    ///   `viewportCount` parameter in the last call to [`cmd_set_viewport_shading_rate_palette_nv`]
    ///   **must**  be greater than or equal to the `viewportCount` parameter in the last call to
    ///   [`cmd_set_viewport_with_count`]
    /// - If the bound graphics pipeline state was created with the
    ///   `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` dynamic state enabled and a
    ///   [`PipelineViewportSwizzleStateCreateInfoNV`] structure chained from
    ///   `VkPipelineVieportCreateInfo`, then the bound graphics pipeline  **must**  have been
    ///   created with [`PipelineViewportSwizzleStateCreateInfoNV::viewport_count`] greater or equal
    ///   to the `viewportCount` parameter in the last call to [`cmd_set_viewport_with_count`]
    /// - If the bound graphics pipeline state was created with the
    ///   `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` dynamic state enabled and a
    ///   [`PipelineViewportExclusiveScissorStateCreateInfoNV`] structure chained from
    ///   `VkPipelineVieportCreateInfo`, then the bound graphics pipeline  **must**  have been
    ///   created with
    ///   [`PipelineViewportExclusiveScissorStateCreateInfoNV::exclusive_scissor_count`] greater or
    ///   equal to the `viewportCount` parameter in the last call to [`cmd_set_viewport_with_count`]
    /// - If the bound graphics pipeline state was created with the
    ///   `VK_DYNAMIC_STATE_RASTERIZER_DISCARD_ENABLE` dynamic state enabled then
    ///   [`cmd_set_rasterizer_discard_enable`] **must**  have been called in the current command
    ///   buffer prior to this drawing command
    /// - If the bound graphics pipeline state was created with the
    ///   `VK_DYNAMIC_STATE_DEPTH_BIAS_ENABLE` dynamic state enabled then
    ///   [`cmd_set_depth_bias_enable`] **must**  have been called in the current command buffer
    ///   prior to this drawing command
    /// - If the bound graphics pipeline state was created with the `VK_DYNAMIC_STATE_LOGIC_OP_EXT`
    ///   dynamic state enabled then [`cmd_set_logic_op_ext`] **must**  have been called in the
    ///   current command buffer prior to this drawing command and the `logicOp` **must**  be a
    ///   valid [`LogicOp`] value
    /// - If the [`primitiveFragmentShadingRateWithMultipleViewports`]() limit is not supported, the
    ///   bound graphics pipeline was created with the `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT`
    ///   dynamic state enabled, and any of the shader stages of the bound graphics pipeline write
    ///   to the `PrimitiveShadingRateKHR` built-in, then [`cmd_set_viewport_with_count`] **must**
    ///   have been called in the current command buffer prior to this drawing command, and the
    ///   `viewportCount` parameter of [`cmd_set_viewport_with_count`] **must**  be `1`
    /// - If rasterization is not disabled in the bound graphics pipeline, then for each color
    ///   attachment in the subpass, if the corresponding image view’s [format features]() do not
    ///   contain `VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BLEND_BIT`, then the `blendEnable` member of
    ///   the corresponding element of the `pAttachments` member of `pColorBlendState` **must**  be
    ///   [`FALSE`]
    /// - If rasterization is not disabled in the bound graphics pipeline, and neither the
    ///   `[`VK_AMD_mixed_attachment_samples`]` nor the `[`VK_NV_framebuffer_mixed_samples`]`
    ///   extensions are enabled, then [`PipelineMultisampleStateCreateInfo::rasterization_samples`]
    ///   **must**  be the same as the current subpass color and/or depth/stencil attachments
    /// - If the current render pass instance was begun with [`cmd_begin_rendering`], the
    ///   `imageView` member of `pDepthAttachment` is not [`crate::Handle::null`], and the `layout`
    ///   member of `pDepthAttachment` is `VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL`, this
    ///   command  **must**  not write any values to the depth attachment
    /// - If the current render pass instance was begun with [`cmd_begin_rendering`], the
    ///   `imageView` member of `pStencilAttachment` is not [`crate::Handle::null`], and the
    ///   `layout` member of `pStencilAttachment` is
    ///   `VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL`, this command  **must**  not write any
    ///   values to the stencil attachment
    /// - If the current render pass instance was begun with [`cmd_begin_rendering`], the
    ///   `imageView` member of `pDepthAttachment` is not [`crate::Handle::null`], and the `layout`
    ///   member of `pDepthAttachment` is
    ///   `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL`, this command  **must**  not
    ///   write any values to the depth attachment
    /// - If the current render pass instance was begun with [`cmd_begin_rendering`], the
    ///   `imageView` member of `pStencilAttachment` is not [`crate::Handle::null`], and the
    ///   `layout` member of `pStencilAttachment` is
    ///   `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL`, this command  **must**  not
    ///   write any values to the stencil attachment
    /// - If the current render pass instance was begun with [`cmd_begin_rendering`], the
    ///   `imageView` member of `pDepthAttachment` is not [`crate::Handle::null`], and the `layout`
    ///   member of `pDepthAttachment` is `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL`, this command
    ///   **must**  not write any values to the depth attachment
    /// - If the current render pass instance was begun with [`cmd_begin_rendering`], the
    ///   `imageView` member of `pStencilAttachment` is not [`crate::Handle::null`], and the
    ///   `layout` member of `pStencilAttachment` is `VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL`,
    ///   this command  **must**  not write any values to the stencil attachment
    /// - If the current render pass instance was begun with [`cmd_begin_rendering`], the currently
    ///   bound graphics pipeline  **must**  have been created with a
    ///   [`PipelineRenderingCreateInfo::view_mask`] equal to [`RenderingInfo::view_mask`]
    /// - If the current render pass instance was begun with [`cmd_begin_rendering`], the currently
    ///   bound graphics pipeline  **must**  have been created with a
    ///   [`PipelineRenderingCreateInfo::color_attachment_count`] equal to
    ///   [`RenderingInfo::color_attachment_count`]
    /// - If the current render pass instance was begun with [`cmd_begin_rendering`] and
    ///   [`RenderingInfo::color_attachment_count`] greater than `0`, then each element of the
    ///   [`RenderingInfo::color_attachments`] array with a `imageView` not equal to
    ///   [`crate::Handle::null`] **must**  have been created with a [`Format`] equal to the
    ///   corresponding element of [`PipelineRenderingCreateInfo::color_attachment_formats`] used to
    ///   create the currently bound graphics pipeline
    /// - If the bound graphics pipeline state was created with the
    ///   `VK_DYNAMIC_STATE_COLOR_WRITE_ENABLE_EXT` dynamic state enabled then
    ///   [`cmd_set_color_write_enable_ext`] **must**  have been called in the current command
    ///   buffer prior to this drawing command, and the `attachmentCount` parameter of
    ///   [`cmd_set_color_write_enable_ext`] **must**  be equal to the
    ///   [`PipelineColorBlendStateCreateInfo::attachment_count`] of the currently bound graphics
    ///   pipeline
    /// - If the current render pass instance was begun with [`cmd_begin_rendering`] and
    ///   [`RenderingInfo`]::`pDepthAttachment->pname`:imageView was not [`crate::Handle::null`],
    ///   the value of [`PipelineRenderingCreateInfo::depth_attachment_format`] used to create the
    ///   currently bound graphics pipeline  **must**  be equal to the [`Format`] used to create
    ///   [`RenderingInfo`]::`pDepthAttachment->pname`:imageView
    /// - If the current render pass instance was begun with [`cmd_begin_rendering`] and
    ///   [`RenderingInfo`]::`pStencilAttachment->pname`:imageView was not [`crate::Handle::null`],
    ///   the value of [`PipelineRenderingCreateInfo::stencil_attachment_format`] used to create the
    ///   currently bound graphics pipeline  **must**  be equal to the [`Format`] used to create
    ///   [`RenderingInfo`]::`pStencilAttachment->pname`:imageView
    /// - If the current render pass instance was begun with [`cmd_begin_rendering`] and
    ///   [`RenderingFragmentShadingRateAttachmentInfoKHR::image_view`] was not
    ///   [`crate::Handle::null`], the currently bound graphics pipeline  **must**  have been
    ///   created with `VK_PIPELINE_CREATE_RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR`
    /// - If the current render pass instance was begun with [`cmd_begin_rendering`] and
    ///   [`RenderingFragmentDensityMapAttachmentInfoEXT::image_view`] was not
    ///   [`crate::Handle::null`], the currently bound graphics pipeline  **must**  have been
    ///   created with `VK_PIPELINE_CREATE_RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_BIT_EXT`
    /// - If the currently bound pipeline was created with a [`AttachmentSampleCountInfoAMD`] or
    ///   [`AttachmentSampleCountInfoNV`] structure, and the current render pass instance was begun
    ///   with [`cmd_begin_rendering`] with a [`RenderingInfo::color_attachment_count`] parameter
    ///   greater than `0`, then each element of the [`RenderingInfo::color_attachments`] array with
    ///   a `imageView` not equal to [`crate::Handle::null`] **must**  have been created with a
    ///   sample count equal to the corresponding element of the `pColorAttachmentSamples` member of
    ///   [`AttachmentSampleCountInfoAMD`] or [`AttachmentSampleCountInfoNV`] used to create the
    ///   currently bound graphics pipeline
    /// - If the current render pass instance was begun with [`cmd_begin_rendering`], the currently
    ///   bound pipeline was created with a [`AttachmentSampleCountInfoAMD`] or
    ///   [`AttachmentSampleCountInfoNV`] structure, and
    ///   [`RenderingInfo`]::`pDepthAttachment->pname`:imageView was not [`crate::Handle::null`],
    ///   the value of the `depthStencilAttachmentSamples` member of
    ///   [`AttachmentSampleCountInfoAMD`] or [`AttachmentSampleCountInfoNV`] used to create the
    ///   currently bound graphics pipeline  **must**  be equal to the sample count used to create
    ///   [`RenderingInfo`]::`pDepthAttachment->pname`:imageView
    /// - If the current render pass instance was begun with [`cmd_begin_rendering`], the currently
    ///   bound pipeline was created with a [`AttachmentSampleCountInfoAMD`] or
    ///   [`AttachmentSampleCountInfoNV`] structure, and
    ///   [`RenderingInfo`]::`pStencilAttachment->pname`:imageView was not [`crate::Handle::null`],
    ///   the value of the `depthStencilAttachmentSamples` member of
    ///   [`AttachmentSampleCountInfoAMD`] or [`AttachmentSampleCountInfoNV`] used to create the
    ///   currently bound graphics pipeline  **must**  be equal to the sample count used to create
    ///   [`RenderingInfo`]::`pStencilAttachment->pname`:imageView
    /// - If the currently bound pipeline was created without a [`AttachmentSampleCountInfoAMD`] or
    ///   [`AttachmentSampleCountInfoNV`] structure, and the current render pass instance was begun
    ///   with [`cmd_begin_rendering`] with a [`RenderingInfo::color_attachment_count`] parameter
    ///   greater than `0`, then each element of the [`RenderingInfo::color_attachments`] array with
    ///   a `imageView` not equal to [`crate::Handle::null`] **must**  have been created with a
    ///   sample count equal to the value of
    ///   [`PipelineMultisampleStateCreateInfo::rasterization_samples`] used to create the currently
    ///   bound graphics pipeline
    /// - If the current render pass instance was begun with [`cmd_begin_rendering`], the currently
    ///   bound pipeline was created without a [`AttachmentSampleCountInfoAMD`] or
    ///   [`AttachmentSampleCountInfoNV`] structure, and
    ///   [`RenderingInfo`]::`pDepthAttachment->pname`:imageView was not [`crate::Handle::null`],
    ///   the value of [`PipelineMultisampleStateCreateInfo::rasterization_samples`] used to create
    ///   the currently bound graphics pipeline  **must**  be equal to the sample count used to
    ///   create [`RenderingInfo`]::`pDepthAttachment->pname`:imageView
    /// - If the current render pass instance was begun with [`cmd_begin_rendering`], the currently
    ///   bound pipeline was created without a [`AttachmentSampleCountInfoAMD`] or
    ///   [`AttachmentSampleCountInfoNV`] structure, and
    ///   [`RenderingInfo`]::`pStencilAttachment->pname`:imageView was not [`crate::Handle::null`],
    ///   the value of [`PipelineMultisampleStateCreateInfo::rasterization_samples`] used to create
    ///   the currently bound graphics pipeline  **must**  be equal to the sample count used to
    ///   create [`RenderingInfo`]::`pStencilAttachment->pname`:imageView
    /// - If the current render pass instance was begun with [`cmd_begin_rendering`], the currently
    ///   bound pipeline  **must**  have been created with a
    ///   [`GraphicsPipelineCreateInfo::render_pass`] equal to [`crate::Handle::null`]
    ///
    /// - The bound graphics pipeline  **must**  not have been created with the
    ///   [`PipelineShaderStageCreateInfo::stage`] member of an element of
    ///   [`GraphicsPipelineCreateInfo::stages`] set to `VK_SHADER_STAGE_VERTEX_BIT`,
    ///   `VK_SHADER_STAGE_TESSELLATION_CONTROL_BIT`, `VK_SHADER_STAGE_TESSELLATION_EVALUATION_BIT`
    ///   or `VK_SHADER_STAGE_GEOMETRY_BIT`
    /// - [`task_count`] **must**  be less than or equal to
    ///   [`PhysicalDeviceMeshShaderPropertiesNV::max_draw_mesh_tasks_count`]
    ///
    ///## Valid Usage (Implicit)
    /// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
    /// - [`command_buffer`] **must**  be in the [recording state]()
    /// - The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics
    ///   operations
    /// - This command  **must**  only be called inside of a render pass instance
    ///
    ///## Host Synchronization
    /// - Host access to [`command_buffer`] **must**  be externally synchronized
    /// - Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**
    ///   be externally synchronized
    ///
    ///## Command Properties
    ///# Related
    /// - [`VK_NV_mesh_shader`]
    /// - [`CommandBuffer`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkCmdDrawMeshTasksNV")]
    #[track_caller]
    #[inline]
    pub unsafe fn cmd_draw_mesh_tasks_nv<'a: 'this, 'b: 'a + 'this, 'this>(
        self: &'this mut Unique<'a, 'b, CommandBuffer>,
        task_count: Option<u32>,
        first_task: Option<u32>,
    ) -> () {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .device()
            .vtable()
            .nv_mesh_shader()
            .and_then(|vtable| vtable.cmd_draw_mesh_tasks_nv())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .device()
            .vtable()
            .nv_mesh_shader()
            .and_then(|vtable| vtable.cmd_draw_mesh_tasks_nv())
            .unwrap_unchecked();
        let _return = _function(
            self.as_raw(),
            task_count.unwrap_or_default() as _,
            first_task.unwrap_or_default() as _,
        );
        ()
    }
}
impl CommandBuffer {
    ///[vkCmdDrawMeshTasksIndirectNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawMeshTasksIndirectNV.html) - Issue an indirect mesh tasks draw into a command buffer
    ///# C Specifications
    ///To record an indirect mesh tasks draw, call:
    ///```c
    ///// Provided by VK_NV_mesh_shader
    ///void vkCmdDrawMeshTasksIndirectNV(
    ///    VkCommandBuffer                             commandBuffer,
    ///    VkBuffer                                    buffer,
    ///    VkDeviceSize                                offset,
    ///    uint32_t                                    drawCount,
    ///    uint32_t                                    stride);
    ///```
    ///# Parameters
    /// - [`command_buffer`] is the command buffer into which the command is recorded.
    /// - [`buffer`] is the buffer containing draw parameters.
    /// - [`offset`] is the byte offset into [`buffer`] where parameters begin.
    /// - [`draw_count`] is the number of draws to execute, and  **can**  be zero.
    /// - [`stride`] is the byte stride between successive sets of draw parameters.
    ///# Description
    ///[`cmd_draw_mesh_tasks_indirect_nv`] behaves similarly to
    ///[`cmd_draw_mesh_tasks_nv`] except that the parameters are read by the device
    ///from a buffer during execution.
    ///[`draw_count`] draws are executed by the command, with parameters taken
    ///from [`buffer`] starting at [`offset`] and increasing by [`stride`]
    ///bytes for each successive draw.
    ///The parameters of each draw are encoded in an array of
    ///[`DrawMeshTasksIndirectCommandNV`] structures.
    ///If [`draw_count`] is less than or equal to one, [`stride`] is ignored.
    ///## Valid Usage
    /// - If a [`Sampler`] created with `magFilter` or `minFilter` equal to `VK_FILTER_LINEAR` and
    ///   `compareEnable` equal to [`FALSE`] is used to sample a [`ImageView`] as a result of this
    ///   command, then the image view’s [format features]() **must**  contain
    ///   `VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT`
    /// - If a [`Sampler`] created with `mipmapMode` equal to `VK_SAMPLER_MIPMAP_MODE_LINEAR` and
    ///   `compareEnable` equal to [`FALSE`] is used to sample a [`ImageView`] as a result of this
    ///   command, then the image view’s [format features]() **must**  contain
    ///   `VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT`
    /// - If a [`ImageView`] is sampled with [depth comparison](), the image view’s [format
    ///   features]() **must**  contain `VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_DEPTH_COMPARISON_BIT`
    /// - If a [`ImageView`] is accessed using atomic operations as a result of this command, then
    ///   the image view’s [format features]() **must**  contain
    ///   `VK_FORMAT_FEATURE_STORAGE_IMAGE_ATOMIC_BIT`
    /// - If a [`ImageView`] is sampled with `VK_FILTER_CUBIC_EXT` as a result of this command, then
    ///   the image view’s [format features]() **must**  contain
    ///   `VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_CUBIC_BIT_EXT`
    /// - Any [`ImageView`] being sampled with `VK_FILTER_CUBIC_EXT` as a result of this command
    ///   **must**  have a [`ImageViewType`] and format that supports cubic filtering, as specified
    ///   by [`FilterCubicImageViewImageFormatPropertiesEXT::filter_cubic`] returned by
    ///   [`get_physical_device_image_format_properties2`]
    /// - Any [`ImageView`] being sampled with `VK_FILTER_CUBIC_EXT` with a reduction mode of either
    ///   `VK_SAMPLER_REDUCTION_MODE_MIN` or `VK_SAMPLER_REDUCTION_MODE_MAX` as a result of this
    ///   command  **must**  have a [`ImageViewType`] and format that supports cubic filtering
    ///   together with minmax filtering, as specified by
    ///   [`FilterCubicImageViewImageFormatPropertiesEXT::filter_cubic_minmax`] returned by
    ///   [`get_physical_device_image_format_properties2`]
    /// - Any [`Image`] created with a [`ImageCreateInfo::flags`] containing
    ///   `VK_IMAGE_CREATE_CORNER_SAMPLED_BIT_NV` sampled as a result of this command  **must**
    ///   only be sampled using a [`SamplerAddressMode`] of `VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE`
    /// - Any [`ImageView`] or [`BufferView`] being written as a storage image or storage texel
    ///   buffer where the image format field of the `OpTypeImage` is `Unknown` then the view’s
    ///   format feature  **must**  contain `VK_FORMAT_FEATURE_2_STORAGE_WRITE_WITHOUT_FORMAT_BIT`
    /// - Any [`ImageView`] or [`BufferView`] being read as a storage image or storage texel buffer
    ///   where the image format field of the `OpTypeImage` is `Unknown` then the view’s format
    ///   feature  **must**  contain `VK_FORMAT_FEATURE_2_STORAGE_READ_WITHOUT_FORMAT_BIT`
    /// - For each set *n* that is statically used by the [`Pipeline`] bound to the pipeline bind
    ///   point used by this command, a descriptor set  **must**  have been bound to *n* at the same
    ///   pipeline bind point, with a [`PipelineLayout`] that is compatible for set *n*, with the
    ///   [`PipelineLayout`] used to create the current [`Pipeline`], as described in
    ///   [[descriptorsets-compatibility]]()
    /// - If the [`maintenance4`]() feature is not enabled, then for each push constant that is
    ///   statically used by the [`Pipeline`] bound to the pipeline bind point used by this command,
    ///   a push constant value  **must**  have been set for the same pipeline bind point, with a
    ///   [`PipelineLayout`] that is compatible for push constants, with the [`PipelineLayout`] used
    ///   to create the current [`Pipeline`], as described in [[descriptorsets-compatibility]]()
    /// - Descriptors in each bound descriptor set, specified via [`cmd_bind_descriptor_sets`],
    ///   **must**  be valid if they are statically used by the [`Pipeline`] bound to the pipeline
    ///   bind point used by this command
    /// - A valid pipeline  **must**  be bound to the pipeline bind point used by this command
    /// - If the [`Pipeline`] object bound to the pipeline bind point used by this command requires
    ///   any dynamic state, that state  **must**  have been set or inherited (if the
    ///   `[`VK_NV_inherited_viewport_scissor`]` extension is enabled) for [`command_buffer`], and
    ///   done so after any previously bound pipeline with the corresponding state not specified as
    ///   dynamic
    /// - There  **must**  not have been any calls to dynamic state setting commands for any state
    ///   not specified as dynamic in the [`Pipeline`] object bound to the pipeline bind point used
    ///   by this command, since that pipeline was bound
    /// - If the [`Pipeline`] object bound to the pipeline bind point used by this command accesses
    ///   a [`Sampler`] object that uses unnormalized coordinates, that sampler  **must**  not be
    ///   used to sample from any [`Image`] with a [`ImageView`] of the type
    ///   `VK_IMAGE_VIEW_TYPE_3D`, `VK_IMAGE_VIEW_TYPE_CUBE`, `VK_IMAGE_VIEW_TYPE_1D_ARRAY`,
    ///   `VK_IMAGE_VIEW_TYPE_2D_ARRAY` or `VK_IMAGE_VIEW_TYPE_CUBE_ARRAY`, in any shader stage
    /// - If the [`Pipeline`] object bound to the pipeline bind point used by this command accesses
    ///   a [`Sampler`] object that uses unnormalized coordinates, that sampler  **must**  not be
    ///   used with any of the SPIR-V `OpImageSample*` or `OpImageSparseSample*` instructions with
    ///   `ImplicitLod`, `Dref` or `Proj` in their name, in any shader stage
    /// - If the [`Pipeline`] object bound to the pipeline bind point used by this command accesses
    ///   a [`Sampler`] object that uses unnormalized coordinates, that sampler  **must**  not be
    ///   used with any of the SPIR-V `OpImageSample*` or `OpImageSparseSample*` instructions that
    ///   includes a LOD bias or any offset values, in any shader stage
    /// - If the [robust buffer access]() feature is not enabled, and if the [`Pipeline`] object
    ///   bound to the pipeline bind point used by this command accesses a uniform buffer, it
    ///   **must**  not access values outside of the range of the buffer as specified in the
    ///   descriptor set bound to the same pipeline bind point
    /// - If the [robust buffer access]() feature is not enabled, and if the [`Pipeline`] object
    ///   bound to the pipeline bind point used by this command accesses a storage buffer, it
    ///   **must**  not access values outside of the range of the buffer as specified in the
    ///   descriptor set bound to the same pipeline bind point
    /// - If [`command_buffer`] is an unprotected command buffer and [`protectedNoFault`]() is not
    ///   supported, any resource accessed by the [`Pipeline`] object bound to the pipeline bind
    ///   point used by this command  **must**  not be a protected resource
    /// - If the [`Pipeline`] object bound to the pipeline bind point used by this command accesses
    ///   a [`Sampler`] or [`ImageView`] object that enables [sampler Y′C<sub>B</sub>C<sub>R</sub>
    ///   conversion](), that object  **must**  only be used with `OpImageSample*` or
    ///   `OpImageSparseSample*` instructions
    /// - If the [`Pipeline`] object bound to the pipeline bind point used by this command accesses
    ///   a [`Sampler`] or [`ImageView`] object that enables [sampler Y′C<sub>B</sub>C<sub>R</sub>
    ///   conversion](), that object  **must**  not use the `ConstOffset` and `Offset` operands
    /// - If a [`ImageView`] is accessed using `OpImageWrite` as a result of this command, then the
    ///   `Type` of the `Texel` operand of that instruction  **must**  have at least as many
    ///   components as the image view’s format
    /// - If a [`BufferView`] is accessed using `OpImageWrite` as a result of this command, then the
    ///   `Type` of the `Texel` operand of that instruction  **must**  have at least as many
    ///   components as the buffer view’s format
    /// - If a [`ImageView`] with a [`Format`] that has a 64-bit component width is accessed as a
    ///   result of this command, the `SampledType` of the `OpTypeImage` operand of that instruction
    ///   **must**  have a `Width` of 64
    /// - If a [`ImageView`] with a [`Format`] that has a component width less than 64-bit is
    ///   accessed as a result of this command, the `SampledType` of the `OpTypeImage` operand of
    ///   that instruction  **must**  have a `Width` of 32
    /// - If a [`BufferView`] with a [`Format`] that has a 64-bit component width is accessed as a
    ///   result of this command, the `SampledType` of the `OpTypeImage` operand of that instruction
    ///   **must**  have a `Width` of 64
    /// - If a [`BufferView`] with a [`Format`] that has a component width less than 64-bit is
    ///   accessed as a result of this command, the `SampledType` of the `OpTypeImage` operand of
    ///   that instruction  **must**  have a `Width` of 32
    /// - If the [`sparseImageInt64Atomics`]() feature is not enabled, [`Image`] objects created
    ///   with the `VK_IMAGE_CREATE_SPARSE_RESIDENCY_BIT` flag  **must**  not be accessed by atomic
    ///   instructions through an `OpTypeImage` with a `SampledType` with a `Width` of 64 by this
    ///   command
    /// - If the [`sparseImageInt64Atomics`]() feature is not enabled, [`Buffer`] objects created
    ///   with the `VK_BUFFER_CREATE_SPARSE_RESIDENCY_BIT` flag  **must**  not be accessed by atomic
    ///   instructions through an `OpTypeImage` with a `SampledType` with a `Width` of 64 by this
    ///   command
    /// - The current render pass  **must**  be [compatible]() with the `renderPass` member of the
    ///   [`GraphicsPipelineCreateInfo`] structure specified when creating the [`Pipeline`] bound to
    ///   `VK_PIPELINE_BIND_POINT_GRAPHICS`
    /// - The subpass index of the current render pass  **must**  be equal to the `subpass` member
    ///   of the [`GraphicsPipelineCreateInfo`] structure specified when creating the [`Pipeline`]
    ///   bound to `VK_PIPELINE_BIND_POINT_GRAPHICS`
    /// - Every input attachment used by the current subpass  **must**  be bound to the pipeline via
    ///   a descriptor set
    /// - Memory backing image subresources used as attachments in the current render pass  **must**
    ///   not be written in any way other than as an attachment by this command
    /// - If any recorded command in the current subpass will write to an image subresource as an
    ///   attachment, this command  **must**  not read from the memory backing that image
    ///   subresource in any other way than as an attachment
    /// - If any recorded command in the current subpass will read from an image subresource used as
    ///   an attachment in any way other than as an attachment, this command  **must**  not write to
    ///   that image subresource as an attachment
    /// - If the draw is recorded in a render pass instance with multiview enabled, the maximum
    ///   instance index  **must**  be less than or equal to
    ///   [`PhysicalDeviceMultiviewProperties::max_multiview_instance_index`]
    /// - If the bound graphics pipeline was created with
    ///   [`PipelineSampleLocationsStateCreateInfoEXT::sample_locations_enable`] set to [`TRUE`] and
    ///   the current subpass has a depth/stencil attachment, then that attachment  **must**  have
    ///   been created with the `VK_IMAGE_CREATE_SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_BIT_EXT` bit set
    /// - If the bound graphics pipeline state was created with the
    ///   `VK_DYNAMIC_STATE_SAMPLE_LOCATIONS_EXT` dynamic state enabled then
    ///   [`cmd_set_sample_locations_ext`] **must**  have been called in the current command buffer
    ///   prior to this drawing command
    /// - If the bound graphics pipeline state was created with the
    ///   `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` dynamic state enabled, but not the
    ///   `VK_DYNAMIC_STATE_SCISSOR_WITH_COUNT` dynamic state enabled, then
    ///   [`cmd_set_viewport_with_count`] **must**  have been called in the current command buffer
    ///   prior to this drawing command, and the `viewportCount` parameter of
    ///   [`cmd_set_viewport_with_count`] **must**  match the
    ///   [`PipelineViewportStateCreateInfo::scissor_count`] of the pipeline
    /// - If the bound graphics pipeline state was created with the
    ///   `VK_DYNAMIC_STATE_SCISSOR_WITH_COUNT` dynamic state enabled, but not the
    ///   `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` dynamic state enabled, then
    ///   [`cmd_set_scissor_with_count`] **must**  have been called in the current command buffer
    ///   prior to this drawing command, and the `scissorCount` parameter of
    ///   [`cmd_set_scissor_with_count`] **must**  match the
    ///   [`PipelineViewportStateCreateInfo::viewport_count`] of the pipeline
    /// - If the bound graphics pipeline state was created with both the
    ///   `VK_DYNAMIC_STATE_SCISSOR_WITH_COUNT` and `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` dynamic
    ///   states enabled then both [`cmd_set_viewport_with_count`] and
    ///   [`cmd_set_scissor_with_count`] **must**  have been called in the current command buffer
    ///   prior to this drawing command, and the `viewportCount` parameter of
    ///   [`cmd_set_viewport_with_count`] **must**  match the `scissorCount` parameter of
    ///   [`cmd_set_scissor_with_count`]
    /// - If the bound graphics pipeline state was created with the
    ///   `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` dynamic state enabled, but not the
    ///   `VK_DYNAMIC_STATE_VIEWPORT_W_SCALING_NV` dynamic state enabled, then the bound graphics
    ///   pipeline  **must**  have been created with
    ///   [`PipelineViewportWScalingStateCreateInfoNV::viewport_count`] greater or equal to the
    ///   `viewportCount` parameter in the last call to [`cmd_set_viewport_with_count`]
    /// - If the bound graphics pipeline state was created with the
    ///   `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` and `VK_DYNAMIC_STATE_VIEWPORT_W_SCALING_NV`
    ///   dynamic states enabled then the `viewportCount` parameter in the last call to
    ///   [`cmd_set_viewport_w_scaling_nv`] **must**  be greater than or equal to the
    ///   `viewportCount` parameter in the last call to [`cmd_set_viewport_with_count`]
    /// - If the bound graphics pipeline state was created with the
    ///   `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` dynamic state enabled, but not the
    ///   `VK_DYNAMIC_STATE_VIEWPORT_SHADING_RATE_PALETTE_NV` dynamic state enabled, then the bound
    ///   graphics pipeline  **must**  have been created with
    ///   [`PipelineViewportShadingRateImageStateCreateInfoNV::viewport_count`] greater or equal to
    ///   the `viewportCount` parameter in the last call to [`cmd_set_viewport_with_count`]
    /// - If the bound graphics pipeline state was created with the
    ///   `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` and
    ///   `VK_DYNAMIC_STATE_VIEWPORT_SHADING_RATE_PALETTE_NV` dynamic states enabled then the
    ///   `viewportCount` parameter in the last call to [`cmd_set_viewport_shading_rate_palette_nv`]
    ///   **must**  be greater than or equal to the `viewportCount` parameter in the last call to
    ///   [`cmd_set_viewport_with_count`]
    /// - If the bound graphics pipeline state was created with the
    ///   `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` dynamic state enabled and a
    ///   [`PipelineViewportSwizzleStateCreateInfoNV`] structure chained from
    ///   `VkPipelineVieportCreateInfo`, then the bound graphics pipeline  **must**  have been
    ///   created with [`PipelineViewportSwizzleStateCreateInfoNV::viewport_count`] greater or equal
    ///   to the `viewportCount` parameter in the last call to [`cmd_set_viewport_with_count`]
    /// - If the bound graphics pipeline state was created with the
    ///   `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` dynamic state enabled and a
    ///   [`PipelineViewportExclusiveScissorStateCreateInfoNV`] structure chained from
    ///   `VkPipelineVieportCreateInfo`, then the bound graphics pipeline  **must**  have been
    ///   created with
    ///   [`PipelineViewportExclusiveScissorStateCreateInfoNV::exclusive_scissor_count`] greater or
    ///   equal to the `viewportCount` parameter in the last call to [`cmd_set_viewport_with_count`]
    /// - If the bound graphics pipeline state was created with the
    ///   `VK_DYNAMIC_STATE_RASTERIZER_DISCARD_ENABLE` dynamic state enabled then
    ///   [`cmd_set_rasterizer_discard_enable`] **must**  have been called in the current command
    ///   buffer prior to this drawing command
    /// - If the bound graphics pipeline state was created with the
    ///   `VK_DYNAMIC_STATE_DEPTH_BIAS_ENABLE` dynamic state enabled then
    ///   [`cmd_set_depth_bias_enable`] **must**  have been called in the current command buffer
    ///   prior to this drawing command
    /// - If the bound graphics pipeline state was created with the `VK_DYNAMIC_STATE_LOGIC_OP_EXT`
    ///   dynamic state enabled then [`cmd_set_logic_op_ext`] **must**  have been called in the
    ///   current command buffer prior to this drawing command and the `logicOp` **must**  be a
    ///   valid [`LogicOp`] value
    /// - If the [`primitiveFragmentShadingRateWithMultipleViewports`]() limit is not supported, the
    ///   bound graphics pipeline was created with the `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT`
    ///   dynamic state enabled, and any of the shader stages of the bound graphics pipeline write
    ///   to the `PrimitiveShadingRateKHR` built-in, then [`cmd_set_viewport_with_count`] **must**
    ///   have been called in the current command buffer prior to this drawing command, and the
    ///   `viewportCount` parameter of [`cmd_set_viewport_with_count`] **must**  be `1`
    /// - If rasterization is not disabled in the bound graphics pipeline, then for each color
    ///   attachment in the subpass, if the corresponding image view’s [format features]() do not
    ///   contain `VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BLEND_BIT`, then the `blendEnable` member of
    ///   the corresponding element of the `pAttachments` member of `pColorBlendState` **must**  be
    ///   [`FALSE`]
    /// - If rasterization is not disabled in the bound graphics pipeline, and neither the
    ///   `[`VK_AMD_mixed_attachment_samples`]` nor the `[`VK_NV_framebuffer_mixed_samples`]`
    ///   extensions are enabled, then [`PipelineMultisampleStateCreateInfo::rasterization_samples`]
    ///   **must**  be the same as the current subpass color and/or depth/stencil attachments
    /// - If the current render pass instance was begun with [`cmd_begin_rendering`], the
    ///   `imageView` member of `pDepthAttachment` is not [`crate::Handle::null`], and the `layout`
    ///   member of `pDepthAttachment` is `VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL`, this
    ///   command  **must**  not write any values to the depth attachment
    /// - If the current render pass instance was begun with [`cmd_begin_rendering`], the
    ///   `imageView` member of `pStencilAttachment` is not [`crate::Handle::null`], and the
    ///   `layout` member of `pStencilAttachment` is
    ///   `VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL`, this command  **must**  not write any
    ///   values to the stencil attachment
    /// - If the current render pass instance was begun with [`cmd_begin_rendering`], the
    ///   `imageView` member of `pDepthAttachment` is not [`crate::Handle::null`], and the `layout`
    ///   member of `pDepthAttachment` is
    ///   `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL`, this command  **must**  not
    ///   write any values to the depth attachment
    /// - If the current render pass instance was begun with [`cmd_begin_rendering`], the
    ///   `imageView` member of `pStencilAttachment` is not [`crate::Handle::null`], and the
    ///   `layout` member of `pStencilAttachment` is
    ///   `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL`, this command  **must**  not
    ///   write any values to the stencil attachment
    /// - If the current render pass instance was begun with [`cmd_begin_rendering`], the
    ///   `imageView` member of `pDepthAttachment` is not [`crate::Handle::null`], and the `layout`
    ///   member of `pDepthAttachment` is `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL`, this command
    ///   **must**  not write any values to the depth attachment
    /// - If the current render pass instance was begun with [`cmd_begin_rendering`], the
    ///   `imageView` member of `pStencilAttachment` is not [`crate::Handle::null`], and the
    ///   `layout` member of `pStencilAttachment` is `VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL`,
    ///   this command  **must**  not write any values to the stencil attachment
    /// - If the current render pass instance was begun with [`cmd_begin_rendering`], the currently
    ///   bound graphics pipeline  **must**  have been created with a
    ///   [`PipelineRenderingCreateInfo::view_mask`] equal to [`RenderingInfo::view_mask`]
    /// - If the current render pass instance was begun with [`cmd_begin_rendering`], the currently
    ///   bound graphics pipeline  **must**  have been created with a
    ///   [`PipelineRenderingCreateInfo::color_attachment_count`] equal to
    ///   [`RenderingInfo::color_attachment_count`]
    /// - If the current render pass instance was begun with [`cmd_begin_rendering`] and
    ///   [`RenderingInfo::color_attachment_count`] greater than `0`, then each element of the
    ///   [`RenderingInfo::color_attachments`] array with a `imageView` not equal to
    ///   [`crate::Handle::null`] **must**  have been created with a [`Format`] equal to the
    ///   corresponding element of [`PipelineRenderingCreateInfo::color_attachment_formats`] used to
    ///   create the currently bound graphics pipeline
    /// - If the bound graphics pipeline state was created with the
    ///   `VK_DYNAMIC_STATE_COLOR_WRITE_ENABLE_EXT` dynamic state enabled then
    ///   [`cmd_set_color_write_enable_ext`] **must**  have been called in the current command
    ///   buffer prior to this drawing command, and the `attachmentCount` parameter of
    ///   [`cmd_set_color_write_enable_ext`] **must**  be equal to the
    ///   [`PipelineColorBlendStateCreateInfo::attachment_count`] of the currently bound graphics
    ///   pipeline
    /// - If the current render pass instance was begun with [`cmd_begin_rendering`] and
    ///   [`RenderingInfo`]::`pDepthAttachment->pname`:imageView was not [`crate::Handle::null`],
    ///   the value of [`PipelineRenderingCreateInfo::depth_attachment_format`] used to create the
    ///   currently bound graphics pipeline  **must**  be equal to the [`Format`] used to create
    ///   [`RenderingInfo`]::`pDepthAttachment->pname`:imageView
    /// - If the current render pass instance was begun with [`cmd_begin_rendering`] and
    ///   [`RenderingInfo`]::`pStencilAttachment->pname`:imageView was not [`crate::Handle::null`],
    ///   the value of [`PipelineRenderingCreateInfo::stencil_attachment_format`] used to create the
    ///   currently bound graphics pipeline  **must**  be equal to the [`Format`] used to create
    ///   [`RenderingInfo`]::`pStencilAttachment->pname`:imageView
    /// - If the current render pass instance was begun with [`cmd_begin_rendering`] and
    ///   [`RenderingFragmentShadingRateAttachmentInfoKHR::image_view`] was not
    ///   [`crate::Handle::null`], the currently bound graphics pipeline  **must**  have been
    ///   created with `VK_PIPELINE_CREATE_RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR`
    /// - If the current render pass instance was begun with [`cmd_begin_rendering`] and
    ///   [`RenderingFragmentDensityMapAttachmentInfoEXT::image_view`] was not
    ///   [`crate::Handle::null`], the currently bound graphics pipeline  **must**  have been
    ///   created with `VK_PIPELINE_CREATE_RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_BIT_EXT`
    /// - If the currently bound pipeline was created with a [`AttachmentSampleCountInfoAMD`] or
    ///   [`AttachmentSampleCountInfoNV`] structure, and the current render pass instance was begun
    ///   with [`cmd_begin_rendering`] with a [`RenderingInfo::color_attachment_count`] parameter
    ///   greater than `0`, then each element of the [`RenderingInfo::color_attachments`] array with
    ///   a `imageView` not equal to [`crate::Handle::null`] **must**  have been created with a
    ///   sample count equal to the corresponding element of the `pColorAttachmentSamples` member of
    ///   [`AttachmentSampleCountInfoAMD`] or [`AttachmentSampleCountInfoNV`] used to create the
    ///   currently bound graphics pipeline
    /// - If the current render pass instance was begun with [`cmd_begin_rendering`], the currently
    ///   bound pipeline was created with a [`AttachmentSampleCountInfoAMD`] or
    ///   [`AttachmentSampleCountInfoNV`] structure, and
    ///   [`RenderingInfo`]::`pDepthAttachment->pname`:imageView was not [`crate::Handle::null`],
    ///   the value of the `depthStencilAttachmentSamples` member of
    ///   [`AttachmentSampleCountInfoAMD`] or [`AttachmentSampleCountInfoNV`] used to create the
    ///   currently bound graphics pipeline  **must**  be equal to the sample count used to create
    ///   [`RenderingInfo`]::`pDepthAttachment->pname`:imageView
    /// - If the current render pass instance was begun with [`cmd_begin_rendering`], the currently
    ///   bound pipeline was created with a [`AttachmentSampleCountInfoAMD`] or
    ///   [`AttachmentSampleCountInfoNV`] structure, and
    ///   [`RenderingInfo`]::`pStencilAttachment->pname`:imageView was not [`crate::Handle::null`],
    ///   the value of the `depthStencilAttachmentSamples` member of
    ///   [`AttachmentSampleCountInfoAMD`] or [`AttachmentSampleCountInfoNV`] used to create the
    ///   currently bound graphics pipeline  **must**  be equal to the sample count used to create
    ///   [`RenderingInfo`]::`pStencilAttachment->pname`:imageView
    /// - If the currently bound pipeline was created without a [`AttachmentSampleCountInfoAMD`] or
    ///   [`AttachmentSampleCountInfoNV`] structure, and the current render pass instance was begun
    ///   with [`cmd_begin_rendering`] with a [`RenderingInfo::color_attachment_count`] parameter
    ///   greater than `0`, then each element of the [`RenderingInfo::color_attachments`] array with
    ///   a `imageView` not equal to [`crate::Handle::null`] **must**  have been created with a
    ///   sample count equal to the value of
    ///   [`PipelineMultisampleStateCreateInfo::rasterization_samples`] used to create the currently
    ///   bound graphics pipeline
    /// - If the current render pass instance was begun with [`cmd_begin_rendering`], the currently
    ///   bound pipeline was created without a [`AttachmentSampleCountInfoAMD`] or
    ///   [`AttachmentSampleCountInfoNV`] structure, and
    ///   [`RenderingInfo`]::`pDepthAttachment->pname`:imageView was not [`crate::Handle::null`],
    ///   the value of [`PipelineMultisampleStateCreateInfo::rasterization_samples`] used to create
    ///   the currently bound graphics pipeline  **must**  be equal to the sample count used to
    ///   create [`RenderingInfo`]::`pDepthAttachment->pname`:imageView
    /// - If the current render pass instance was begun with [`cmd_begin_rendering`], the currently
    ///   bound pipeline was created without a [`AttachmentSampleCountInfoAMD`] or
    ///   [`AttachmentSampleCountInfoNV`] structure, and
    ///   [`RenderingInfo`]::`pStencilAttachment->pname`:imageView was not [`crate::Handle::null`],
    ///   the value of [`PipelineMultisampleStateCreateInfo::rasterization_samples`] used to create
    ///   the currently bound graphics pipeline  **must**  be equal to the sample count used to
    ///   create [`RenderingInfo`]::`pStencilAttachment->pname`:imageView
    /// - If the current render pass instance was begun with [`cmd_begin_rendering`], the currently
    ///   bound pipeline  **must**  have been created with a
    ///   [`GraphicsPipelineCreateInfo::render_pass`] equal to [`crate::Handle::null`]
    ///
    /// - The bound graphics pipeline  **must**  not have been created with the
    ///   [`PipelineShaderStageCreateInfo::stage`] member of an element of
    ///   [`GraphicsPipelineCreateInfo::stages`] set to `VK_SHADER_STAGE_VERTEX_BIT`,
    ///   `VK_SHADER_STAGE_TESSELLATION_CONTROL_BIT`, `VK_SHADER_STAGE_TESSELLATION_EVALUATION_BIT`
    ///   or `VK_SHADER_STAGE_GEOMETRY_BIT`
    ///
    /// - If [`buffer`] is non-sparse then it  **must**  be bound completely and contiguously to a
    ///   single [`DeviceMemory`] object
    /// - [`buffer`] **must**  have been created with the `VK_BUFFER_USAGE_INDIRECT_BUFFER_BIT` bit
    ///   set
    /// - [`offset`] **must**  be a multiple of `4`
    /// - [`command_buffer`] **must**  not be a protected command buffer
    ///
    /// - If the [multi-draw indirect]() feature is not enabled, [`draw_count`] **must**  be `0` or
    ///   `1`
    /// - [`draw_count`] **must**  be less than or equal to
    ///   [`PhysicalDeviceLimits::max_draw_indirect_count`]
    /// - If [`draw_count`] is greater than `1`, [`stride`] **must**  be a multiple of `4` and
    ///   **must**  be greater than or equal to `sizeof`([`DrawMeshTasksIndirectCommandNV`])
    /// - If [`draw_count`] is equal to `1`, ([`offset`] +
    ///   `sizeof`([`DrawMeshTasksIndirectCommandNV`])) **must**  be less than or equal to the size
    ///   of [`buffer`]
    /// - If [`draw_count`] is greater than `1`, ([`stride`] × ([`draw_count`] - 1) +  [`offset`] +
    ///   `sizeof`([`DrawMeshTasksIndirectCommandNV`])) **must**  be less than or equal to the size
    ///   of [`buffer`]
    ///
    ///## Valid Usage (Implicit)
    /// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
    /// - [`buffer`] **must**  be a valid [`Buffer`] handle
    /// - [`command_buffer`] **must**  be in the [recording state]()
    /// - The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics
    ///   operations
    /// - This command  **must**  only be called inside of a render pass instance
    /// - Both of [`buffer`], and [`command_buffer`] **must**  have been created, allocated, or
    ///   retrieved from the same [`Device`]
    ///
    ///## Host Synchronization
    /// - Host access to [`command_buffer`] **must**  be externally synchronized
    /// - Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**
    ///   be externally synchronized
    ///
    ///## Command Properties
    ///# Related
    /// - [`VK_NV_mesh_shader`]
    /// - [`Buffer`]
    /// - [`CommandBuffer`]
    /// - [`DeviceSize`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkCmdDrawMeshTasksIndirectNV")]
    #[track_caller]
    #[inline]
    pub unsafe fn cmd_draw_mesh_tasks_indirect_nv<'a: 'this, 'b: 'a + 'this, 'this>(
        self: &'this mut Unique<'a, 'b, CommandBuffer>,
        buffer: Buffer,
        offset: DeviceSize,
        draw_count: Option<u32>,
        stride: Option<u32>,
    ) -> () {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .device()
            .vtable()
            .nv_mesh_shader()
            .and_then(|vtable| vtable.cmd_draw_mesh_tasks_indirect_nv())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .device()
            .vtable()
            .nv_mesh_shader()
            .and_then(|vtable| vtable.cmd_draw_mesh_tasks_indirect_nv())
            .unwrap_unchecked();
        let _return = _function(
            self.as_raw(),
            buffer,
            offset,
            draw_count.unwrap_or_default() as _,
            stride.unwrap_or_default() as _,
        );
        ()
    }
}
impl CommandBuffer {
    ///[vkCmdDrawMeshTasksIndirectCountNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawMeshTasksIndirectCountNV.html) - Perform an indirect mesh tasks draw with the draw count sourced from a buffer
    ///# C Specifications
    ///To record an indirect mesh tasks draw with the draw count sourced from a
    ///buffer, call:
    ///```c
    ///// Provided by VK_NV_mesh_shader
    ///void vkCmdDrawMeshTasksIndirectCountNV(
    ///    VkCommandBuffer                             commandBuffer,
    ///    VkBuffer                                    buffer,
    ///    VkDeviceSize                                offset,
    ///    VkBuffer                                    countBuffer,
    ///    VkDeviceSize                                countBufferOffset,
    ///    uint32_t                                    maxDrawCount,
    ///    uint32_t                                    stride);
    ///```
    ///# Parameters
    /// - [`command_buffer`] is the command buffer into which the command is recorded.
    /// - [`buffer`] is the buffer containing draw parameters.
    /// - [`offset`] is the byte offset into [`buffer`] where parameters begin.
    /// - [`count_buffer`] is the buffer containing the draw count.
    /// - [`count_buffer_offset`] is the byte offset into [`count_buffer`] where the draw count
    ///   begins.
    /// - [`max_draw_count`] specifies the maximum number of draws that will be executed. The actual
    ///   number of executed draw calls is the minimum of the count specified in [`count_buffer`]
    ///   and [`max_draw_count`].
    /// - [`stride`] is the byte stride between successive sets of draw parameters.
    ///# Description
    ///[`cmd_draw_mesh_tasks_indirect_count_nv`] behaves similarly to
    ///[`cmd_draw_mesh_tasks_indirect_nv`] except that the draw count is read by the
    ///device from a buffer during execution.
    ///The command will read an unsigned 32-bit integer from [`count_buffer`]
    ///located at [`count_buffer_offset`] and use this as the draw count.
    ///## Valid Usage
    /// - If a [`Sampler`] created with `magFilter` or `minFilter` equal to `VK_FILTER_LINEAR` and
    ///   `compareEnable` equal to [`FALSE`] is used to sample a [`ImageView`] as a result of this
    ///   command, then the image view’s [format features]() **must**  contain
    ///   `VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT`
    /// - If a [`Sampler`] created with `mipmapMode` equal to `VK_SAMPLER_MIPMAP_MODE_LINEAR` and
    ///   `compareEnable` equal to [`FALSE`] is used to sample a [`ImageView`] as a result of this
    ///   command, then the image view’s [format features]() **must**  contain
    ///   `VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT`
    /// - If a [`ImageView`] is sampled with [depth comparison](), the image view’s [format
    ///   features]() **must**  contain `VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_DEPTH_COMPARISON_BIT`
    /// - If a [`ImageView`] is accessed using atomic operations as a result of this command, then
    ///   the image view’s [format features]() **must**  contain
    ///   `VK_FORMAT_FEATURE_STORAGE_IMAGE_ATOMIC_BIT`
    /// - If a [`ImageView`] is sampled with `VK_FILTER_CUBIC_EXT` as a result of this command, then
    ///   the image view’s [format features]() **must**  contain
    ///   `VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_CUBIC_BIT_EXT`
    /// - Any [`ImageView`] being sampled with `VK_FILTER_CUBIC_EXT` as a result of this command
    ///   **must**  have a [`ImageViewType`] and format that supports cubic filtering, as specified
    ///   by [`FilterCubicImageViewImageFormatPropertiesEXT::filter_cubic`] returned by
    ///   [`get_physical_device_image_format_properties2`]
    /// - Any [`ImageView`] being sampled with `VK_FILTER_CUBIC_EXT` with a reduction mode of either
    ///   `VK_SAMPLER_REDUCTION_MODE_MIN` or `VK_SAMPLER_REDUCTION_MODE_MAX` as a result of this
    ///   command  **must**  have a [`ImageViewType`] and format that supports cubic filtering
    ///   together with minmax filtering, as specified by
    ///   [`FilterCubicImageViewImageFormatPropertiesEXT::filter_cubic_minmax`] returned by
    ///   [`get_physical_device_image_format_properties2`]
    /// - Any [`Image`] created with a [`ImageCreateInfo::flags`] containing
    ///   `VK_IMAGE_CREATE_CORNER_SAMPLED_BIT_NV` sampled as a result of this command  **must**
    ///   only be sampled using a [`SamplerAddressMode`] of `VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE`
    /// - Any [`ImageView`] or [`BufferView`] being written as a storage image or storage texel
    ///   buffer where the image format field of the `OpTypeImage` is `Unknown` then the view’s
    ///   format feature  **must**  contain `VK_FORMAT_FEATURE_2_STORAGE_WRITE_WITHOUT_FORMAT_BIT`
    /// - Any [`ImageView`] or [`BufferView`] being read as a storage image or storage texel buffer
    ///   where the image format field of the `OpTypeImage` is `Unknown` then the view’s format
    ///   feature  **must**  contain `VK_FORMAT_FEATURE_2_STORAGE_READ_WITHOUT_FORMAT_BIT`
    /// - For each set *n* that is statically used by the [`Pipeline`] bound to the pipeline bind
    ///   point used by this command, a descriptor set  **must**  have been bound to *n* at the same
    ///   pipeline bind point, with a [`PipelineLayout`] that is compatible for set *n*, with the
    ///   [`PipelineLayout`] used to create the current [`Pipeline`], as described in
    ///   [[descriptorsets-compatibility]]()
    /// - If the [`maintenance4`]() feature is not enabled, then for each push constant that is
    ///   statically used by the [`Pipeline`] bound to the pipeline bind point used by this command,
    ///   a push constant value  **must**  have been set for the same pipeline bind point, with a
    ///   [`PipelineLayout`] that is compatible for push constants, with the [`PipelineLayout`] used
    ///   to create the current [`Pipeline`], as described in [[descriptorsets-compatibility]]()
    /// - Descriptors in each bound descriptor set, specified via [`cmd_bind_descriptor_sets`],
    ///   **must**  be valid if they are statically used by the [`Pipeline`] bound to the pipeline
    ///   bind point used by this command
    /// - A valid pipeline  **must**  be bound to the pipeline bind point used by this command
    /// - If the [`Pipeline`] object bound to the pipeline bind point used by this command requires
    ///   any dynamic state, that state  **must**  have been set or inherited (if the
    ///   `[`VK_NV_inherited_viewport_scissor`]` extension is enabled) for [`command_buffer`], and
    ///   done so after any previously bound pipeline with the corresponding state not specified as
    ///   dynamic
    /// - There  **must**  not have been any calls to dynamic state setting commands for any state
    ///   not specified as dynamic in the [`Pipeline`] object bound to the pipeline bind point used
    ///   by this command, since that pipeline was bound
    /// - If the [`Pipeline`] object bound to the pipeline bind point used by this command accesses
    ///   a [`Sampler`] object that uses unnormalized coordinates, that sampler  **must**  not be
    ///   used to sample from any [`Image`] with a [`ImageView`] of the type
    ///   `VK_IMAGE_VIEW_TYPE_3D`, `VK_IMAGE_VIEW_TYPE_CUBE`, `VK_IMAGE_VIEW_TYPE_1D_ARRAY`,
    ///   `VK_IMAGE_VIEW_TYPE_2D_ARRAY` or `VK_IMAGE_VIEW_TYPE_CUBE_ARRAY`, in any shader stage
    /// - If the [`Pipeline`] object bound to the pipeline bind point used by this command accesses
    ///   a [`Sampler`] object that uses unnormalized coordinates, that sampler  **must**  not be
    ///   used with any of the SPIR-V `OpImageSample*` or `OpImageSparseSample*` instructions with
    ///   `ImplicitLod`, `Dref` or `Proj` in their name, in any shader stage
    /// - If the [`Pipeline`] object bound to the pipeline bind point used by this command accesses
    ///   a [`Sampler`] object that uses unnormalized coordinates, that sampler  **must**  not be
    ///   used with any of the SPIR-V `OpImageSample*` or `OpImageSparseSample*` instructions that
    ///   includes a LOD bias or any offset values, in any shader stage
    /// - If the [robust buffer access]() feature is not enabled, and if the [`Pipeline`] object
    ///   bound to the pipeline bind point used by this command accesses a uniform buffer, it
    ///   **must**  not access values outside of the range of the buffer as specified in the
    ///   descriptor set bound to the same pipeline bind point
    /// - If the [robust buffer access]() feature is not enabled, and if the [`Pipeline`] object
    ///   bound to the pipeline bind point used by this command accesses a storage buffer, it
    ///   **must**  not access values outside of the range of the buffer as specified in the
    ///   descriptor set bound to the same pipeline bind point
    /// - If [`command_buffer`] is an unprotected command buffer and [`protectedNoFault`]() is not
    ///   supported, any resource accessed by the [`Pipeline`] object bound to the pipeline bind
    ///   point used by this command  **must**  not be a protected resource
    /// - If the [`Pipeline`] object bound to the pipeline bind point used by this command accesses
    ///   a [`Sampler`] or [`ImageView`] object that enables [sampler Y′C<sub>B</sub>C<sub>R</sub>
    ///   conversion](), that object  **must**  only be used with `OpImageSample*` or
    ///   `OpImageSparseSample*` instructions
    /// - If the [`Pipeline`] object bound to the pipeline bind point used by this command accesses
    ///   a [`Sampler`] or [`ImageView`] object that enables [sampler Y′C<sub>B</sub>C<sub>R</sub>
    ///   conversion](), that object  **must**  not use the `ConstOffset` and `Offset` operands
    /// - If a [`ImageView`] is accessed using `OpImageWrite` as a result of this command, then the
    ///   `Type` of the `Texel` operand of that instruction  **must**  have at least as many
    ///   components as the image view’s format
    /// - If a [`BufferView`] is accessed using `OpImageWrite` as a result of this command, then the
    ///   `Type` of the `Texel` operand of that instruction  **must**  have at least as many
    ///   components as the buffer view’s format
    /// - If a [`ImageView`] with a [`Format`] that has a 64-bit component width is accessed as a
    ///   result of this command, the `SampledType` of the `OpTypeImage` operand of that instruction
    ///   **must**  have a `Width` of 64
    /// - If a [`ImageView`] with a [`Format`] that has a component width less than 64-bit is
    ///   accessed as a result of this command, the `SampledType` of the `OpTypeImage` operand of
    ///   that instruction  **must**  have a `Width` of 32
    /// - If a [`BufferView`] with a [`Format`] that has a 64-bit component width is accessed as a
    ///   result of this command, the `SampledType` of the `OpTypeImage` operand of that instruction
    ///   **must**  have a `Width` of 64
    /// - If a [`BufferView`] with a [`Format`] that has a component width less than 64-bit is
    ///   accessed as a result of this command, the `SampledType` of the `OpTypeImage` operand of
    ///   that instruction  **must**  have a `Width` of 32
    /// - If the [`sparseImageInt64Atomics`]() feature is not enabled, [`Image`] objects created
    ///   with the `VK_IMAGE_CREATE_SPARSE_RESIDENCY_BIT` flag  **must**  not be accessed by atomic
    ///   instructions through an `OpTypeImage` with a `SampledType` with a `Width` of 64 by this
    ///   command
    /// - If the [`sparseImageInt64Atomics`]() feature is not enabled, [`Buffer`] objects created
    ///   with the `VK_BUFFER_CREATE_SPARSE_RESIDENCY_BIT` flag  **must**  not be accessed by atomic
    ///   instructions through an `OpTypeImage` with a `SampledType` with a `Width` of 64 by this
    ///   command
    /// - The current render pass  **must**  be [compatible]() with the `renderPass` member of the
    ///   [`GraphicsPipelineCreateInfo`] structure specified when creating the [`Pipeline`] bound to
    ///   `VK_PIPELINE_BIND_POINT_GRAPHICS`
    /// - The subpass index of the current render pass  **must**  be equal to the `subpass` member
    ///   of the [`GraphicsPipelineCreateInfo`] structure specified when creating the [`Pipeline`]
    ///   bound to `VK_PIPELINE_BIND_POINT_GRAPHICS`
    /// - Every input attachment used by the current subpass  **must**  be bound to the pipeline via
    ///   a descriptor set
    /// - Memory backing image subresources used as attachments in the current render pass  **must**
    ///   not be written in any way other than as an attachment by this command
    /// - If any recorded command in the current subpass will write to an image subresource as an
    ///   attachment, this command  **must**  not read from the memory backing that image
    ///   subresource in any other way than as an attachment
    /// - If any recorded command in the current subpass will read from an image subresource used as
    ///   an attachment in any way other than as an attachment, this command  **must**  not write to
    ///   that image subresource as an attachment
    /// - If the draw is recorded in a render pass instance with multiview enabled, the maximum
    ///   instance index  **must**  be less than or equal to
    ///   [`PhysicalDeviceMultiviewProperties::max_multiview_instance_index`]
    /// - If the bound graphics pipeline was created with
    ///   [`PipelineSampleLocationsStateCreateInfoEXT::sample_locations_enable`] set to [`TRUE`] and
    ///   the current subpass has a depth/stencil attachment, then that attachment  **must**  have
    ///   been created with the `VK_IMAGE_CREATE_SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_BIT_EXT` bit set
    /// - If the bound graphics pipeline state was created with the
    ///   `VK_DYNAMIC_STATE_SAMPLE_LOCATIONS_EXT` dynamic state enabled then
    ///   [`cmd_set_sample_locations_ext`] **must**  have been called in the current command buffer
    ///   prior to this drawing command
    /// - If the bound graphics pipeline state was created with the
    ///   `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` dynamic state enabled, but not the
    ///   `VK_DYNAMIC_STATE_SCISSOR_WITH_COUNT` dynamic state enabled, then
    ///   [`cmd_set_viewport_with_count`] **must**  have been called in the current command buffer
    ///   prior to this drawing command, and the `viewportCount` parameter of
    ///   [`cmd_set_viewport_with_count`] **must**  match the
    ///   [`PipelineViewportStateCreateInfo::scissor_count`] of the pipeline
    /// - If the bound graphics pipeline state was created with the
    ///   `VK_DYNAMIC_STATE_SCISSOR_WITH_COUNT` dynamic state enabled, but not the
    ///   `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` dynamic state enabled, then
    ///   [`cmd_set_scissor_with_count`] **must**  have been called in the current command buffer
    ///   prior to this drawing command, and the `scissorCount` parameter of
    ///   [`cmd_set_scissor_with_count`] **must**  match the
    ///   [`PipelineViewportStateCreateInfo::viewport_count`] of the pipeline
    /// - If the bound graphics pipeline state was created with both the
    ///   `VK_DYNAMIC_STATE_SCISSOR_WITH_COUNT` and `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` dynamic
    ///   states enabled then both [`cmd_set_viewport_with_count`] and
    ///   [`cmd_set_scissor_with_count`] **must**  have been called in the current command buffer
    ///   prior to this drawing command, and the `viewportCount` parameter of
    ///   [`cmd_set_viewport_with_count`] **must**  match the `scissorCount` parameter of
    ///   [`cmd_set_scissor_with_count`]
    /// - If the bound graphics pipeline state was created with the
    ///   `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` dynamic state enabled, but not the
    ///   `VK_DYNAMIC_STATE_VIEWPORT_W_SCALING_NV` dynamic state enabled, then the bound graphics
    ///   pipeline  **must**  have been created with
    ///   [`PipelineViewportWScalingStateCreateInfoNV::viewport_count`] greater or equal to the
    ///   `viewportCount` parameter in the last call to [`cmd_set_viewport_with_count`]
    /// - If the bound graphics pipeline state was created with the
    ///   `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` and `VK_DYNAMIC_STATE_VIEWPORT_W_SCALING_NV`
    ///   dynamic states enabled then the `viewportCount` parameter in the last call to
    ///   [`cmd_set_viewport_w_scaling_nv`] **must**  be greater than or equal to the
    ///   `viewportCount` parameter in the last call to [`cmd_set_viewport_with_count`]
    /// - If the bound graphics pipeline state was created with the
    ///   `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` dynamic state enabled, but not the
    ///   `VK_DYNAMIC_STATE_VIEWPORT_SHADING_RATE_PALETTE_NV` dynamic state enabled, then the bound
    ///   graphics pipeline  **must**  have been created with
    ///   [`PipelineViewportShadingRateImageStateCreateInfoNV::viewport_count`] greater or equal to
    ///   the `viewportCount` parameter in the last call to [`cmd_set_viewport_with_count`]
    /// - If the bound graphics pipeline state was created with the
    ///   `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` and
    ///   `VK_DYNAMIC_STATE_VIEWPORT_SHADING_RATE_PALETTE_NV` dynamic states enabled then the
    ///   `viewportCount` parameter in the last call to [`cmd_set_viewport_shading_rate_palette_nv`]
    ///   **must**  be greater than or equal to the `viewportCount` parameter in the last call to
    ///   [`cmd_set_viewport_with_count`]
    /// - If the bound graphics pipeline state was created with the
    ///   `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` dynamic state enabled and a
    ///   [`PipelineViewportSwizzleStateCreateInfoNV`] structure chained from
    ///   `VkPipelineVieportCreateInfo`, then the bound graphics pipeline  **must**  have been
    ///   created with [`PipelineViewportSwizzleStateCreateInfoNV::viewport_count`] greater or equal
    ///   to the `viewportCount` parameter in the last call to [`cmd_set_viewport_with_count`]
    /// - If the bound graphics pipeline state was created with the
    ///   `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` dynamic state enabled and a
    ///   [`PipelineViewportExclusiveScissorStateCreateInfoNV`] structure chained from
    ///   `VkPipelineVieportCreateInfo`, then the bound graphics pipeline  **must**  have been
    ///   created with
    ///   [`PipelineViewportExclusiveScissorStateCreateInfoNV::exclusive_scissor_count`] greater or
    ///   equal to the `viewportCount` parameter in the last call to [`cmd_set_viewport_with_count`]
    /// - If the bound graphics pipeline state was created with the
    ///   `VK_DYNAMIC_STATE_RASTERIZER_DISCARD_ENABLE` dynamic state enabled then
    ///   [`cmd_set_rasterizer_discard_enable`] **must**  have been called in the current command
    ///   buffer prior to this drawing command
    /// - If the bound graphics pipeline state was created with the
    ///   `VK_DYNAMIC_STATE_DEPTH_BIAS_ENABLE` dynamic state enabled then
    ///   [`cmd_set_depth_bias_enable`] **must**  have been called in the current command buffer
    ///   prior to this drawing command
    /// - If the bound graphics pipeline state was created with the `VK_DYNAMIC_STATE_LOGIC_OP_EXT`
    ///   dynamic state enabled then [`cmd_set_logic_op_ext`] **must**  have been called in the
    ///   current command buffer prior to this drawing command and the `logicOp` **must**  be a
    ///   valid [`LogicOp`] value
    /// - If the [`primitiveFragmentShadingRateWithMultipleViewports`]() limit is not supported, the
    ///   bound graphics pipeline was created with the `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT`
    ///   dynamic state enabled, and any of the shader stages of the bound graphics pipeline write
    ///   to the `PrimitiveShadingRateKHR` built-in, then [`cmd_set_viewport_with_count`] **must**
    ///   have been called in the current command buffer prior to this drawing command, and the
    ///   `viewportCount` parameter of [`cmd_set_viewport_with_count`] **must**  be `1`
    /// - If rasterization is not disabled in the bound graphics pipeline, then for each color
    ///   attachment in the subpass, if the corresponding image view’s [format features]() do not
    ///   contain `VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BLEND_BIT`, then the `blendEnable` member of
    ///   the corresponding element of the `pAttachments` member of `pColorBlendState` **must**  be
    ///   [`FALSE`]
    /// - If rasterization is not disabled in the bound graphics pipeline, and neither the
    ///   `[`VK_AMD_mixed_attachment_samples`]` nor the `[`VK_NV_framebuffer_mixed_samples`]`
    ///   extensions are enabled, then [`PipelineMultisampleStateCreateInfo::rasterization_samples`]
    ///   **must**  be the same as the current subpass color and/or depth/stencil attachments
    /// - If the current render pass instance was begun with [`cmd_begin_rendering`], the
    ///   `imageView` member of `pDepthAttachment` is not [`crate::Handle::null`], and the `layout`
    ///   member of `pDepthAttachment` is `VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL`, this
    ///   command  **must**  not write any values to the depth attachment
    /// - If the current render pass instance was begun with [`cmd_begin_rendering`], the
    ///   `imageView` member of `pStencilAttachment` is not [`crate::Handle::null`], and the
    ///   `layout` member of `pStencilAttachment` is
    ///   `VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL`, this command  **must**  not write any
    ///   values to the stencil attachment
    /// - If the current render pass instance was begun with [`cmd_begin_rendering`], the
    ///   `imageView` member of `pDepthAttachment` is not [`crate::Handle::null`], and the `layout`
    ///   member of `pDepthAttachment` is
    ///   `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL`, this command  **must**  not
    ///   write any values to the depth attachment
    /// - If the current render pass instance was begun with [`cmd_begin_rendering`], the
    ///   `imageView` member of `pStencilAttachment` is not [`crate::Handle::null`], and the
    ///   `layout` member of `pStencilAttachment` is
    ///   `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL`, this command  **must**  not
    ///   write any values to the stencil attachment
    /// - If the current render pass instance was begun with [`cmd_begin_rendering`], the
    ///   `imageView` member of `pDepthAttachment` is not [`crate::Handle::null`], and the `layout`
    ///   member of `pDepthAttachment` is `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL`, this command
    ///   **must**  not write any values to the depth attachment
    /// - If the current render pass instance was begun with [`cmd_begin_rendering`], the
    ///   `imageView` member of `pStencilAttachment` is not [`crate::Handle::null`], and the
    ///   `layout` member of `pStencilAttachment` is `VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL`,
    ///   this command  **must**  not write any values to the stencil attachment
    /// - If the current render pass instance was begun with [`cmd_begin_rendering`], the currently
    ///   bound graphics pipeline  **must**  have been created with a
    ///   [`PipelineRenderingCreateInfo::view_mask`] equal to [`RenderingInfo::view_mask`]
    /// - If the current render pass instance was begun with [`cmd_begin_rendering`], the currently
    ///   bound graphics pipeline  **must**  have been created with a
    ///   [`PipelineRenderingCreateInfo::color_attachment_count`] equal to
    ///   [`RenderingInfo::color_attachment_count`]
    /// - If the current render pass instance was begun with [`cmd_begin_rendering`] and
    ///   [`RenderingInfo::color_attachment_count`] greater than `0`, then each element of the
    ///   [`RenderingInfo::color_attachments`] array with a `imageView` not equal to
    ///   [`crate::Handle::null`] **must**  have been created with a [`Format`] equal to the
    ///   corresponding element of [`PipelineRenderingCreateInfo::color_attachment_formats`] used to
    ///   create the currently bound graphics pipeline
    /// - If the bound graphics pipeline state was created with the
    ///   `VK_DYNAMIC_STATE_COLOR_WRITE_ENABLE_EXT` dynamic state enabled then
    ///   [`cmd_set_color_write_enable_ext`] **must**  have been called in the current command
    ///   buffer prior to this drawing command, and the `attachmentCount` parameter of
    ///   [`cmd_set_color_write_enable_ext`] **must**  be equal to the
    ///   [`PipelineColorBlendStateCreateInfo::attachment_count`] of the currently bound graphics
    ///   pipeline
    /// - If the current render pass instance was begun with [`cmd_begin_rendering`] and
    ///   [`RenderingInfo`]::`pDepthAttachment->pname`:imageView was not [`crate::Handle::null`],
    ///   the value of [`PipelineRenderingCreateInfo::depth_attachment_format`] used to create the
    ///   currently bound graphics pipeline  **must**  be equal to the [`Format`] used to create
    ///   [`RenderingInfo`]::`pDepthAttachment->pname`:imageView
    /// - If the current render pass instance was begun with [`cmd_begin_rendering`] and
    ///   [`RenderingInfo`]::`pStencilAttachment->pname`:imageView was not [`crate::Handle::null`],
    ///   the value of [`PipelineRenderingCreateInfo::stencil_attachment_format`] used to create the
    ///   currently bound graphics pipeline  **must**  be equal to the [`Format`] used to create
    ///   [`RenderingInfo`]::`pStencilAttachment->pname`:imageView
    /// - If the current render pass instance was begun with [`cmd_begin_rendering`] and
    ///   [`RenderingFragmentShadingRateAttachmentInfoKHR::image_view`] was not
    ///   [`crate::Handle::null`], the currently bound graphics pipeline  **must**  have been
    ///   created with `VK_PIPELINE_CREATE_RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR`
    /// - If the current render pass instance was begun with [`cmd_begin_rendering`] and
    ///   [`RenderingFragmentDensityMapAttachmentInfoEXT::image_view`] was not
    ///   [`crate::Handle::null`], the currently bound graphics pipeline  **must**  have been
    ///   created with `VK_PIPELINE_CREATE_RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_BIT_EXT`
    /// - If the currently bound pipeline was created with a [`AttachmentSampleCountInfoAMD`] or
    ///   [`AttachmentSampleCountInfoNV`] structure, and the current render pass instance was begun
    ///   with [`cmd_begin_rendering`] with a [`RenderingInfo::color_attachment_count`] parameter
    ///   greater than `0`, then each element of the [`RenderingInfo::color_attachments`] array with
    ///   a `imageView` not equal to [`crate::Handle::null`] **must**  have been created with a
    ///   sample count equal to the corresponding element of the `pColorAttachmentSamples` member of
    ///   [`AttachmentSampleCountInfoAMD`] or [`AttachmentSampleCountInfoNV`] used to create the
    ///   currently bound graphics pipeline
    /// - If the current render pass instance was begun with [`cmd_begin_rendering`], the currently
    ///   bound pipeline was created with a [`AttachmentSampleCountInfoAMD`] or
    ///   [`AttachmentSampleCountInfoNV`] structure, and
    ///   [`RenderingInfo`]::`pDepthAttachment->pname`:imageView was not [`crate::Handle::null`],
    ///   the value of the `depthStencilAttachmentSamples` member of
    ///   [`AttachmentSampleCountInfoAMD`] or [`AttachmentSampleCountInfoNV`] used to create the
    ///   currently bound graphics pipeline  **must**  be equal to the sample count used to create
    ///   [`RenderingInfo`]::`pDepthAttachment->pname`:imageView
    /// - If the current render pass instance was begun with [`cmd_begin_rendering`], the currently
    ///   bound pipeline was created with a [`AttachmentSampleCountInfoAMD`] or
    ///   [`AttachmentSampleCountInfoNV`] structure, and
    ///   [`RenderingInfo`]::`pStencilAttachment->pname`:imageView was not [`crate::Handle::null`],
    ///   the value of the `depthStencilAttachmentSamples` member of
    ///   [`AttachmentSampleCountInfoAMD`] or [`AttachmentSampleCountInfoNV`] used to create the
    ///   currently bound graphics pipeline  **must**  be equal to the sample count used to create
    ///   [`RenderingInfo`]::`pStencilAttachment->pname`:imageView
    /// - If the currently bound pipeline was created without a [`AttachmentSampleCountInfoAMD`] or
    ///   [`AttachmentSampleCountInfoNV`] structure, and the current render pass instance was begun
    ///   with [`cmd_begin_rendering`] with a [`RenderingInfo::color_attachment_count`] parameter
    ///   greater than `0`, then each element of the [`RenderingInfo::color_attachments`] array with
    ///   a `imageView` not equal to [`crate::Handle::null`] **must**  have been created with a
    ///   sample count equal to the value of
    ///   [`PipelineMultisampleStateCreateInfo::rasterization_samples`] used to create the currently
    ///   bound graphics pipeline
    /// - If the current render pass instance was begun with [`cmd_begin_rendering`], the currently
    ///   bound pipeline was created without a [`AttachmentSampleCountInfoAMD`] or
    ///   [`AttachmentSampleCountInfoNV`] structure, and
    ///   [`RenderingInfo`]::`pDepthAttachment->pname`:imageView was not [`crate::Handle::null`],
    ///   the value of [`PipelineMultisampleStateCreateInfo::rasterization_samples`] used to create
    ///   the currently bound graphics pipeline  **must**  be equal to the sample count used to
    ///   create [`RenderingInfo`]::`pDepthAttachment->pname`:imageView
    /// - If the current render pass instance was begun with [`cmd_begin_rendering`], the currently
    ///   bound pipeline was created without a [`AttachmentSampleCountInfoAMD`] or
    ///   [`AttachmentSampleCountInfoNV`] structure, and
    ///   [`RenderingInfo`]::`pStencilAttachment->pname`:imageView was not [`crate::Handle::null`],
    ///   the value of [`PipelineMultisampleStateCreateInfo::rasterization_samples`] used to create
    ///   the currently bound graphics pipeline  **must**  be equal to the sample count used to
    ///   create [`RenderingInfo`]::`pStencilAttachment->pname`:imageView
    /// - If the current render pass instance was begun with [`cmd_begin_rendering`], the currently
    ///   bound pipeline  **must**  have been created with a
    ///   [`GraphicsPipelineCreateInfo::render_pass`] equal to [`crate::Handle::null`]
    ///
    /// - The bound graphics pipeline  **must**  not have been created with the
    ///   [`PipelineShaderStageCreateInfo::stage`] member of an element of
    ///   [`GraphicsPipelineCreateInfo::stages`] set to `VK_SHADER_STAGE_VERTEX_BIT`,
    ///   `VK_SHADER_STAGE_TESSELLATION_CONTROL_BIT`, `VK_SHADER_STAGE_TESSELLATION_EVALUATION_BIT`
    ///   or `VK_SHADER_STAGE_GEOMETRY_BIT`
    ///
    /// - If [`buffer`] is non-sparse then it  **must**  be bound completely and contiguously to a
    ///   single [`DeviceMemory`] object
    /// - [`buffer`] **must**  have been created with the `VK_BUFFER_USAGE_INDIRECT_BUFFER_BIT` bit
    ///   set
    /// - [`offset`] **must**  be a multiple of `4`
    /// - [`command_buffer`] **must**  not be a protected command buffer
    ///
    /// - If [`count_buffer`] is non-sparse then it  **must**  be bound completely and contiguously
    ///   to a single [`DeviceMemory`] object
    /// - [`count_buffer`] **must**  have been created with the
    ///   `VK_BUFFER_USAGE_INDIRECT_BUFFER_BIT` bit set
    /// - [`count_buffer_offset`] **must**  be a multiple of `4`
    /// - The count stored in [`count_buffer`] **must**  be less than or equal to
    ///   [`PhysicalDeviceLimits::max_draw_indirect_count`]
    /// - ([`count_buffer_offset`] +  `sizeof`(uint32_t)) **must**  be less than or equal to the
    ///   size of [`count_buffer`]
    /// - If [drawIndirectCount]() is not enabled this function  **must**  not be used
    /// - [`stride`] **must**  be a multiple of `4` and  **must**  be greater than or equal to
    ///   `sizeof`([`DrawMeshTasksIndirectCommandNV`])
    /// - If [`max_draw_count`] is greater than or equal to `1`, ([`stride`] × ([`max_draw_count`] -
    ///   1) +  [`offset`] +  `sizeof`([`DrawMeshTasksIndirectCommandNV`])) **must**  be less than
    ///   or equal to the size of [`buffer`]
    /// - If the count stored in [`count_buffer`] is equal to `1`, ([`offset`] +
    ///   `sizeof`([`DrawMeshTasksIndirectCommandNV`])) **must**  be less than or equal to the size
    ///   of [`buffer`]
    /// - If the count stored in [`count_buffer`] is greater than `1`, ([`stride`] × (`drawCount` -
    ///   1) +  [`offset`] +  `sizeof`([`DrawMeshTasksIndirectCommandNV`])) **must**  be less than
    ///   or equal to the size of [`buffer`]
    ///
    ///## Valid Usage (Implicit)
    /// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
    /// - [`buffer`] **must**  be a valid [`Buffer`] handle
    /// - [`count_buffer`] **must**  be a valid [`Buffer`] handle
    /// - [`command_buffer`] **must**  be in the [recording state]()
    /// - The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics
    ///   operations
    /// - This command  **must**  only be called inside of a render pass instance
    /// - Each of [`buffer`], [`command_buffer`], and [`count_buffer`] **must**  have been created,
    ///   allocated, or retrieved from the same [`Device`]
    ///
    ///## Host Synchronization
    /// - Host access to [`command_buffer`] **must**  be externally synchronized
    /// - Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**
    ///   be externally synchronized
    ///
    ///## Command Properties
    ///# Related
    /// - [`VK_NV_mesh_shader`]
    /// - [`Buffer`]
    /// - [`CommandBuffer`]
    /// - [`DeviceSize`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkCmdDrawMeshTasksIndirectCountNV")]
    #[track_caller]
    #[inline]
    pub unsafe fn cmd_draw_mesh_tasks_indirect_count_nv<'a: 'this, 'b: 'a + 'this, 'this>(
        self: &'this mut Unique<'a, 'b, CommandBuffer>,
        buffer: Buffer,
        offset: DeviceSize,
        count_buffer: Buffer,
        count_buffer_offset: DeviceSize,
        max_draw_count: Option<u32>,
        stride: Option<u32>,
    ) -> () {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .device()
            .vtable()
            .nv_mesh_shader()
            .and_then(|vtable| vtable.cmd_draw_mesh_tasks_indirect_count_nv())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .device()
            .vtable()
            .nv_mesh_shader()
            .and_then(|vtable| vtable.cmd_draw_mesh_tasks_indirect_count_nv())
            .unwrap_unchecked();
        let _return = _function(
            self.as_raw(),
            buffer,
            offset,
            count_buffer,
            count_buffer_offset,
            max_draw_count.unwrap_or_default() as _,
            stride.unwrap_or_default() as _,
        );
        ()
    }
}
///The V-table of [`Device`] for functions from `VK_NV_mesh_shader`
pub struct DeviceNvMeshShaderVTable {
    ///See [`FNCmdDrawMeshTasksNv`] for more information.
    pub cmd_draw_mesh_tasks_nv: FNCmdDrawMeshTasksNv,
    ///See [`FNCmdDrawMeshTasksIndirectNv`] for more information.
    pub cmd_draw_mesh_tasks_indirect_nv: FNCmdDrawMeshTasksIndirectNv,
    ///See [`FNCmdDrawMeshTasksIndirectCountNv`] for more information.
    pub cmd_draw_mesh_tasks_indirect_count_nv: FNCmdDrawMeshTasksIndirectCountNv,
}
impl DeviceNvMeshShaderVTable {
    ///Loads the VTable from the owner and the names
    #[track_caller]
    pub fn load(
        loader_fn: unsafe extern "system" fn(
            Device,
            *const std::os::raw::c_char,
        ) -> Option<unsafe extern "system" fn()>,
        loader: Device,
    ) -> Self {
        Self {
            cmd_draw_mesh_tasks_nv: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkCmdDrawMeshTasksNV").as_ptr()))
            },
            cmd_draw_mesh_tasks_indirect_nv: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkCmdDrawMeshTasksIndirectNV").as_ptr()))
            },
            cmd_draw_mesh_tasks_indirect_count_nv: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkCmdDrawMeshTasksIndirectCountNV").as_ptr(),
                ))
            },
        }
    }
    ///Gets [`Self::cmd_draw_mesh_tasks_nv`]. See [`FNCmdDrawMeshTasksNv`] for more information.
    pub fn cmd_draw_mesh_tasks_nv(&self) -> FNCmdDrawMeshTasksNv {
        self.cmd_draw_mesh_tasks_nv
    }
    ///Gets [`Self::cmd_draw_mesh_tasks_indirect_nv`]. See [`FNCmdDrawMeshTasksIndirectNv`] for
    /// more information.
    pub fn cmd_draw_mesh_tasks_indirect_nv(&self) -> FNCmdDrawMeshTasksIndirectNv {
        self.cmd_draw_mesh_tasks_indirect_nv
    }
    ///Gets [`Self::cmd_draw_mesh_tasks_indirect_count_nv`]. See
    /// [`FNCmdDrawMeshTasksIndirectCountNv`] for more information.
    pub fn cmd_draw_mesh_tasks_indirect_count_nv(&self) -> FNCmdDrawMeshTasksIndirectCountNv {
        self.cmd_draw_mesh_tasks_indirect_count_nv
    }
}
