//![VK_EXT_multi_draw](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_multi_draw.html) - device extension
//!# Description
//!Processing multiple draw commands in sequence incurs measurable overhead
//!within drivers due to repeated state checks and updates during dispatch.
//!This extension enables passing the entire sequence of draws directly to the
//!driver in order to avoid any such overhead, using an array of
//![`MultiDrawInfoEXT`] or [`MultiDrawIndexedInfoEXT`] structs with
//![`cmd_draw_multi_ext`] or [`cmd_draw_multi_indexed_ext`], respectively.
//!These functions could be used any time multiple draw commands are being
//!recorded without any state changes between them in order to maximize
//!performance.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//!# Contacts
//! - Mike Blumenkrantz [zmike](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_multi_draw]
//!   @zmike%0A<<Here describe the issue or question you have about the VK_EXT_multi_draw
//!   extension>>)
//!# New functions & commands
//! - [`cmd_draw_multi_ext`]
//! - [`cmd_draw_multi_indexed_ext`]
//!# New structures
//! - [`MultiDrawIndexedInfoEXT`]
//! - [`MultiDrawInfoEXT`]
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  -
//!   [`PhysicalDeviceMultiDrawFeaturesEXT`]
//! - Extending [`PhysicalDeviceProperties2`]:  - [`PhysicalDeviceMultiDrawPropertiesEXT`]
//!# New constants
//! - [`EXT_MULTI_DRAW_EXTENSION_NAME`]
//! - [`EXT_MULTI_DRAW_SPEC_VERSION`]
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTI_DRAW_FEATURES_EXT`  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTI_DRAW_PROPERTIES_EXT`
//!# Version History
//! - Revision 1, 2021-01-20 (Mike Blumenkrantz)  - Initial version
//!# Other info
//! * 2021-05-19
//! * No known IP claims.
//! * - Mike Blumenkrantz, VALVE  - Piers Daniell, NVIDIA  - Jason Ekstrand, INTEL  - Spencer
//!   Fricke, SAMSUNG  - Ricardo Garcia, IGALIA  - Jon Leech, KHRONOS  - Stu Smith, AMD
//!# Related
//! - [`MultiDrawIndexedInfoEXT`]
//! - [`MultiDrawInfoEXT`]
//! - [`PhysicalDeviceMultiDrawFeaturesEXT`]
//! - [`PhysicalDeviceMultiDrawPropertiesEXT`]
//! - [`cmd_draw_multi_ext`]
//! - [`cmd_draw_multi_indexed_ext`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::{
    vulkan1_0::{BaseOutStructure, Bool32, CommandBuffer, Device, StructureType},
    AsRaw, Unique,
};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_MULTI_DRAW_SPEC_VERSION")]
pub const EXT_MULTI_DRAW_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_MULTI_DRAW_EXTENSION_NAME")]
pub const EXT_MULTI_DRAW_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_multi_draw");
///[vkCmdDrawMultiEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawMultiEXT.html) - Draw primitives
///# C Specifications
///To record an ordered sequence of drawing operations which have no state
///changes between them, call:
///```c
///// Provided by VK_EXT_multi_draw
///void vkCmdDrawMultiEXT(
///    VkCommandBuffer                             commandBuffer,
///    uint32_t                                    drawCount,
///    const VkMultiDrawInfoEXT*                   pVertexInfo,
///    uint32_t                                    instanceCount,
///    uint32_t                                    firstInstance,
///    uint32_t                                    stride);
///```
/// # Parameters
/// - [`command_buffer`] is the command buffer into which the command is recorded.
/// - [`draw_count`] is the number of draws to execute, and  **can**  be zero.
/// - [`p_vertex_info`] is a pointer to an array of [`MultiDrawInfoEXT`] with vertex information to
///   be drawn.
/// - [`instance_count`] is the number of instances to draw.
/// - [`first_instance`] is the instance ID of the first instance to draw.
/// - [`stride`] is the byte stride between consecutive elements of [`p_vertex_info`].
/// # Description
/// [`draw_count`] draws are executed with parameters taken from
/// [`p_vertex_info`].
/// The number of draw commands recorded is [`draw_count`], with each command
/// reading, sequentially, a `firstVertex` and a `vertexCount` from
/// [`p_vertex_info`].
/// ## Valid Usage
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
/// - If [`command_buffer`] is a protected command buffer and [`protectedNoFault`]() is not
///   supported, any resource written to by the [`Pipeline`] object bound to the pipeline bind point
///   used by this command  **must**  not be an unprotected resource
/// - If [`command_buffer`] is a protected command buffer and [`protectedNoFault`]() is not
///   supported, pipeline stages other than the framebuffer-space and compute stages in the
///   [`Pipeline`] object bound to the pipeline bind point used by this command  **must**  not write
///   to any resource
/// - If any of the shader stages of the [`Pipeline`] bound to the pipeline bind point used by this
///   command uses the [RayQueryKHR]() capability, then [`command_buffer`] **must**  not be a
///   protected command buffer
///
/// - All vertex input bindings accessed via vertex input variables declared in the vertex shader
///   entry point’s interface  **must**  have either valid or [`crate::Handle::null`] buffers bound
/// - If the [nullDescriptor]() feature is not enabled, all vertex input bindings accessed via
///   vertex input variables declared in the vertex shader entry point’s interface  **must**  not be
///   [`crate::Handle::null`]
/// - For a given vertex buffer binding, any attribute data fetched  **must**  be entirely contained
///   within the corresponding vertex buffer binding, as described in [[fxvertex-input]]()
/// - If the bound graphics pipeline state was created with the
///   `VK_DYNAMIC_STATE_PRIMITIVE_TOPOLOGY_EXT` dynamic state enabled then
///   [`cmd_set_primitive_topology_ext`] **must**  have been called in the current command buffer
///   prior to this drawing command, and the `primitiveTopology` parameter of
///   [`cmd_set_primitive_topology_ext`] **must**  be of the same [topology class]() as the pipeline
///   [`PipelineInputAssemblyStateCreateInfo::topology`] state
/// - If the bound graphics pipeline was created with both the `VK_DYNAMIC_STATE_VERTEX_INPUT_EXT`
///   and `VK_DYNAMIC_STATE_VERTEX_INPUT_BINDING_STRIDE_EXT` dynamic states enabled, then
///   [`cmd_set_vertex_input_ext`] **must**  have been called in the current command buffer prior to
///   this draw command
/// - If the bound graphics pipeline was created with the
///   `VK_DYNAMIC_STATE_VERTEX_INPUT_BINDING_STRIDE_EXT` dynamic state enabled, but not the
///   `VK_DYNAMIC_STATE_VERTEX_INPUT_EXT` dynamic state enabled, then
///   [`cmd_bind_vertex_buffers2_ext`] **must**  have been called in the current command buffer
///   prior to this draw command, and the `pStrides` parameter of [`cmd_bind_vertex_buffers2_ext`]
///   **must**  not be `NULL`
/// - If the bound graphics pipeline state was created with the `VK_DYNAMIC_STATE_VERTEX_INPUT_EXT`
///   dynamic state enabled, then [`cmd_set_vertex_input_ext`] **must**  have been called in the
///   current command buffer prior to this draw command
/// - If the bound graphics pipeline state was created with the
///   `VK_DYNAMIC_STATE_PATCH_CONTROL_POINTS_EXT` dynamic state enabled then
///   [`cmd_set_patch_control_points_ext`] **must**  have been called in the current command buffer
///   prior to this drawing command
/// - If the bound graphics pipeline state was created with the
///   `VK_DYNAMIC_STATE_PRIMITIVE_RESTART_ENABLE_EXT` dynamic state enabled then
///   [`cmd_set_primitive_restart_enable_ext`] **must**  have been called in the current command
///   buffer prior to this drawing command
/// - The bound graphics pipeline  **must**  not have been created with the
///   [`PipelineShaderStageCreateInfo::stage`] member of an element of
///   [`GraphicsPipelineCreateInfo::stages`] set to `VK_SHADER_STAGE_TASK_BIT_NV` or
///   `VK_SHADER_STAGE_MESH_BIT_NV`
/// - The [multiDraw](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-multiDraw)
///   feature  **must**  be enabled
/// - [`draw_count`] **must**  be less than
///   [`PhysicalDeviceMultiDrawPropertiesEXT::max_multi_draw_count`]
/// - If [`draw_count`] is greater than zero, [`p_vertex_info`] **must**  be a valid pointer to
///   memory containing one or more valid instances of [`MultiDrawInfoEXT`] structures
/// - [`stride`] must be a multiple of 4
///
/// ## Valid Usage (Implicit)
/// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
/// - [`command_buffer`] **must**  be in the [recording state]()
/// - The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics
///   operations
/// - This command  **must**  only be called inside of a render pass instance
///
/// ## Host Synchronization
/// - Host access to [`command_buffer`] **must**  be externally synchronized
/// - Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be
///   externally synchronized
///
/// ## Command Properties
/// # Related
/// - [`VK_EXT_multi_draw`]
/// - [`CommandBuffer`]
/// - [`MultiDrawInfoEXT`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkCmdDrawMultiEXT")]
pub type FNCmdDrawMultiExt = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        draw_count: u32,
        p_vertex_info: *const MultiDrawInfoEXT,
        instance_count: u32,
        first_instance: u32,
        stride: u32,
    ),
>;
///[vkCmdDrawMultiIndexedEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawMultiIndexedEXT.html) - Draw primitives
///# C Specifications
///To record an ordered sequence of indexed drawing operations which have no
///state changes between them, call:
///```c
///// Provided by VK_EXT_multi_draw
///void vkCmdDrawMultiIndexedEXT(
///    VkCommandBuffer                             commandBuffer,
///    uint32_t                                    drawCount,
///    const VkMultiDrawIndexedInfoEXT*            pIndexInfo,
///    uint32_t                                    instanceCount,
///    uint32_t                                    firstInstance,
///    uint32_t                                    stride,
///    const int32_t*                              pVertexOffset);
///```
/// # Parameters
/// - [`command_buffer`] is the command buffer into which the command is recorded.
/// - [`draw_count`] is the number of draws to execute, and  **can**  be zero.
/// - [`p_index_info`] is a pointer to an array of [`MultiDrawIndexedInfoEXT`] with index
///   information to be drawn.
/// - [`instance_count`] is the number of instances to draw.
/// - [`first_instance`] is the instance ID of the first instance to draw.
/// - [`stride`] is the byte stride between consecutive elements of [`p_index_info`].
/// - [`p_vertex_offset`] is `NULL` or a pointer to the value added to the vertex index before
///   indexing into the vertex buffer. When specified, [`MultiDrawIndexedInfoEXT`]`::offset` is
///   ignored.
/// # Description
/// [`draw_count`] indexed draws are executed with parameters taken from
/// [`p_index_info`].
/// The number of draw commands recorded is [`draw_count`], with each command
/// reading, sequentially, a `firstIndex` and an `indexCount` from
/// [`p_index_info`].
/// If [`p_vertex_offset`] is `NULL`, a `vertexOffset` is also read from
/// [`p_index_info`], otherwise the value from dereferencing [`p_vertex_offset`]
/// is used.
/// ## Valid Usage
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
/// - If [`command_buffer`] is a protected command buffer and [`protectedNoFault`]() is not
///   supported, any resource written to by the [`Pipeline`] object bound to the pipeline bind point
///   used by this command  **must**  not be an unprotected resource
/// - If [`command_buffer`] is a protected command buffer and [`protectedNoFault`]() is not
///   supported, pipeline stages other than the framebuffer-space and compute stages in the
///   [`Pipeline`] object bound to the pipeline bind point used by this command  **must**  not write
///   to any resource
/// - If any of the shader stages of the [`Pipeline`] bound to the pipeline bind point used by this
///   command uses the [RayQueryKHR]() capability, then [`command_buffer`] **must**  not be a
///   protected command buffer
///
/// - All vertex input bindings accessed via vertex input variables declared in the vertex shader
///   entry point’s interface  **must**  have either valid or [`crate::Handle::null`] buffers bound
/// - If the [nullDescriptor]() feature is not enabled, all vertex input bindings accessed via
///   vertex input variables declared in the vertex shader entry point’s interface  **must**  not be
///   [`crate::Handle::null`]
/// - For a given vertex buffer binding, any attribute data fetched  **must**  be entirely contained
///   within the corresponding vertex buffer binding, as described in [[fxvertex-input]]()
/// - If the bound graphics pipeline state was created with the
///   `VK_DYNAMIC_STATE_PRIMITIVE_TOPOLOGY_EXT` dynamic state enabled then
///   [`cmd_set_primitive_topology_ext`] **must**  have been called in the current command buffer
///   prior to this drawing command, and the `primitiveTopology` parameter of
///   [`cmd_set_primitive_topology_ext`] **must**  be of the same [topology class]() as the pipeline
///   [`PipelineInputAssemblyStateCreateInfo::topology`] state
/// - If the bound graphics pipeline was created with both the `VK_DYNAMIC_STATE_VERTEX_INPUT_EXT`
///   and `VK_DYNAMIC_STATE_VERTEX_INPUT_BINDING_STRIDE_EXT` dynamic states enabled, then
///   [`cmd_set_vertex_input_ext`] **must**  have been called in the current command buffer prior to
///   this draw command
/// - If the bound graphics pipeline was created with the
///   `VK_DYNAMIC_STATE_VERTEX_INPUT_BINDING_STRIDE_EXT` dynamic state enabled, but not the
///   `VK_DYNAMIC_STATE_VERTEX_INPUT_EXT` dynamic state enabled, then
///   [`cmd_bind_vertex_buffers2_ext`] **must**  have been called in the current command buffer
///   prior to this draw command, and the `pStrides` parameter of [`cmd_bind_vertex_buffers2_ext`]
///   **must**  not be `NULL`
/// - If the bound graphics pipeline state was created with the `VK_DYNAMIC_STATE_VERTEX_INPUT_EXT`
///   dynamic state enabled, then [`cmd_set_vertex_input_ext`] **must**  have been called in the
///   current command buffer prior to this draw command
/// - If the bound graphics pipeline state was created with the
///   `VK_DYNAMIC_STATE_PATCH_CONTROL_POINTS_EXT` dynamic state enabled then
///   [`cmd_set_patch_control_points_ext`] **must**  have been called in the current command buffer
///   prior to this drawing command
/// - If the bound graphics pipeline state was created with the
///   `VK_DYNAMIC_STATE_PRIMITIVE_RESTART_ENABLE_EXT` dynamic state enabled then
///   [`cmd_set_primitive_restart_enable_ext`] **must**  have been called in the current command
///   buffer prior to this drawing command
/// - The bound graphics pipeline  **must**  not have been created with the
///   [`PipelineShaderStageCreateInfo::stage`] member of an element of
///   [`GraphicsPipelineCreateInfo::stages`] set to `VK_SHADER_STAGE_TASK_BIT_NV` or
///   `VK_SHADER_STAGE_MESH_BIT_NV`
/// - The [multiDraw](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-multiDraw)
///   feature  **must**  be enabled
/// - (`indexSize` × (`firstIndex` +  `indexCount`) +  `offset`) **must**  be less than or equal to
///   the size of the bound index buffer, with `indexSize` being based on the type specified by
///   `indexType`, where the index buffer, `indexType`, and `offset` are specified via
///   [`cmd_bind_index_buffer`]
/// - [`draw_count`] **must**  be less than
///   [`PhysicalDeviceMultiDrawPropertiesEXT::max_multi_draw_count`]
/// - If [`draw_count`] is greater than zero, [`p_index_info`] **must**  be a valid pointer to
///   memory containing one or more valid instances of [`MultiDrawIndexedInfoEXT`] structures
/// - [`stride`] must be a multiple of 4
///
/// ## Valid Usage (Implicit)
/// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
/// - If [`p_vertex_offset`] is not `NULL`, [`p_vertex_offset`] **must**  be a valid pointer to a
///   valid `int32_t` value
/// - [`command_buffer`] **must**  be in the [recording state]()
/// - The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics
///   operations
/// - This command  **must**  only be called inside of a render pass instance
///
/// ## Host Synchronization
/// - Host access to [`command_buffer`] **must**  be externally synchronized
/// - Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be
///   externally synchronized
///
/// ## Command Properties
/// # Related
/// - [`VK_EXT_multi_draw`]
/// - [`CommandBuffer`]
/// - [`MultiDrawIndexedInfoEXT`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkCmdDrawMultiIndexedEXT")]
pub type FNCmdDrawMultiIndexedExt = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        draw_count: u32,
        p_index_info: *const MultiDrawIndexedInfoEXT,
        instance_count: u32,
        first_instance: u32,
        stride: u32,
        p_vertex_offset: *const i32,
    ),
>;
///[VkMultiDrawInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMultiDrawInfoEXT.html) - Structure specifying a multi-draw command
///# C Specifications
///The [`MultiDrawInfoEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_multi_draw
///typedef struct VkMultiDrawInfoEXT {
///    uint32_t    firstVertex;
///    uint32_t    vertexCount;
///} VkMultiDrawInfoEXT;
///```
/// # Members
/// - [`first_vertex`] is the first vertex to draw.
/// - [`vertex_count`] is the number of vertices to draw.
/// # Description
/// The members of [`MultiDrawInfoEXT`] have the same meaning as the
/// [`first_vertex`] and [`vertex_count`] parameters in [`cmd_draw`].
/// # Related
/// - [`VK_EXT_multi_draw`]
/// - [`cmd_draw_multi_ext`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkMultiDrawInfoEXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct MultiDrawInfoEXT {
    ///[`first_vertex`] is the first vertex to draw.
    pub first_vertex: u32,
    ///[`vertex_count`] is the number of vertices to draw.
    pub vertex_count: u32,
}
impl Default for MultiDrawInfoEXT {
    fn default() -> Self {
        Self {
            first_vertex: 0,
            vertex_count: 0,
        }
    }
}
impl MultiDrawInfoEXT {
    ///Gets the value of [`Self::first_vertex`]
    pub fn first_vertex(&self) -> u32 {
        self.first_vertex
    }
    ///Gets the value of [`Self::vertex_count`]
    pub fn vertex_count(&self) -> u32 {
        self.vertex_count
    }
    ///Gets a mutable reference to the value of [`Self::first_vertex`]
    pub fn first_vertex_mut(&mut self) -> &mut u32 {
        &mut self.first_vertex
    }
    ///Gets a mutable reference to the value of [`Self::vertex_count`]
    pub fn vertex_count_mut(&mut self) -> &mut u32 {
        &mut self.vertex_count
    }
    ///Sets the value of [`Self::first_vertex`]
    pub fn set_first_vertex(mut self, value: u32) -> Self {
        self.first_vertex = value;
        self
    }
    ///Sets the value of [`Self::vertex_count`]
    pub fn set_vertex_count(mut self, value: u32) -> Self {
        self.vertex_count = value;
        self
    }
}
///[VkMultiDrawIndexedInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMultiDrawIndexedInfoEXT.html) - Structure specifying a multi-draw command
///# C Specifications
///The [`MultiDrawIndexedInfoEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_multi_draw
///typedef struct VkMultiDrawIndexedInfoEXT {
///    uint32_t    firstIndex;
///    uint32_t    indexCount;
///    int32_t     vertexOffset;
///} VkMultiDrawIndexedInfoEXT;
///```
/// # Members
/// - [`first_index`] is the first index to draw.
/// - [`index_count`] is the number of vertices to draw.
/// - [`vertex_offset`] is the value added to the vertex index before indexing into the vertex
///   buffer for indexed multidraws.
/// # Description
/// The [`first_index`], [`index_count`], and [`vertex_offset`] members of
/// [`MultiDrawIndexedInfoEXT`] have the same meaning as the
/// [`first_index`], [`index_count`], and [`vertex_offset`] parameters,
/// respectively, of [`cmd_draw_indexed`].
/// # Related
/// - [`VK_EXT_multi_draw`]
/// - [`cmd_draw_multi_indexed_ext`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkMultiDrawIndexedInfoEXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct MultiDrawIndexedInfoEXT {
    ///[`first_index`] is the first index to draw.
    pub first_index: u32,
    ///[`index_count`] is the number of vertices to draw.
    pub index_count: u32,
    ///[`vertex_offset`] is the value added to the vertex index before
    ///indexing into the vertex buffer for indexed multidraws.
    pub vertex_offset: i32,
}
impl Default for MultiDrawIndexedInfoEXT {
    fn default() -> Self {
        Self {
            first_index: 0,
            index_count: 0,
            vertex_offset: 0,
        }
    }
}
impl MultiDrawIndexedInfoEXT {
    ///Gets the value of [`Self::first_index`]
    pub fn first_index(&self) -> u32 {
        self.first_index
    }
    ///Gets the value of [`Self::index_count`]
    pub fn index_count(&self) -> u32 {
        self.index_count
    }
    ///Gets the value of [`Self::vertex_offset`]
    pub fn vertex_offset(&self) -> i32 {
        self.vertex_offset
    }
    ///Gets a mutable reference to the value of [`Self::first_index`]
    pub fn first_index_mut(&mut self) -> &mut u32 {
        &mut self.first_index
    }
    ///Gets a mutable reference to the value of [`Self::index_count`]
    pub fn index_count_mut(&mut self) -> &mut u32 {
        &mut self.index_count
    }
    ///Gets a mutable reference to the value of [`Self::vertex_offset`]
    pub fn vertex_offset_mut(&mut self) -> &mut i32 {
        &mut self.vertex_offset
    }
    ///Sets the value of [`Self::first_index`]
    pub fn set_first_index(mut self, value: u32) -> Self {
        self.first_index = value;
        self
    }
    ///Sets the value of [`Self::index_count`]
    pub fn set_index_count(mut self, value: u32) -> Self {
        self.index_count = value;
        self
    }
    ///Sets the value of [`Self::vertex_offset`]
    pub fn set_vertex_offset(mut self, value: i32) -> Self {
        self.vertex_offset = value;
        self
    }
}
///[VkPhysicalDeviceMultiDrawPropertiesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMultiDrawPropertiesEXT.html) - Structure describing multidraw limits of an implementation
///# C Specifications
///The [`PhysicalDeviceMultiDrawPropertiesEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_multi_draw
///typedef struct VkPhysicalDeviceMultiDrawPropertiesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    uint32_t           maxMultiDrawCount;
///} VkPhysicalDeviceMultiDrawPropertiesEXT;
///```
/// # Members
/// The members of the [`PhysicalDeviceMultiDrawPropertiesEXT`] structure
/// describe the following features:
/// # Description
/// - [`max_multi_draw_count`] indicates the maximum number of draw calls which  **can**  be batched
///   into a single multidraw.
/// If the [`PhysicalDeviceMultiDrawPropertiesEXT`] structure is included in the [`p_next`] chain of
/// the
/// [`PhysicalDeviceProperties2`] structure passed to
/// [`get_physical_device_properties2`], it is filled in with each
/// corresponding implementation-dependent property.
/// ## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTI_DRAW_PROPERTIES_EXT`
/// # Related
/// - [`VK_EXT_multi_draw`]
/// - [`StructureType`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPhysicalDeviceMultiDrawPropertiesEXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceMultiDrawPropertiesEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTI_DRAW_PROPERTIES_EXT`
    pub s_type: StructureType,
    ///No documentation found
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`max_multi_draw_count`] indicates the
    ///maximum number of draw calls which  **can**  be batched into a single
    ///multidraw.
    pub max_multi_draw_count: u32,
}
impl<'lt> Default for PhysicalDeviceMultiDrawPropertiesEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PHYSICAL_DEVICE_MULTI_DRAW_PROPERTIES_EXT,
            p_next: std::ptr::null_mut(),
            max_multi_draw_count: 0,
        }
    }
}
impl<'lt> PhysicalDeviceMultiDrawPropertiesEXT<'lt> {
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
    ///Gets the value of [`Self::max_multi_draw_count`]
    pub fn max_multi_draw_count(&self) -> u32 {
        self.max_multi_draw_count
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
    ///Gets a mutable reference to the value of [`Self::max_multi_draw_count`]
    pub fn max_multi_draw_count_mut(&mut self) -> &mut u32 {
        &mut self.max_multi_draw_count
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
    ///Sets the value of [`Self::max_multi_draw_count`]
    pub fn set_max_multi_draw_count(mut self, value: u32) -> Self {
        self.max_multi_draw_count = value;
        self
    }
}
///[VkPhysicalDeviceMultiDrawFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMultiDrawFeaturesEXT.html) - Structure describing whether the implementation supports multi draw functionality
///# C Specifications
///The [`PhysicalDeviceMultiDrawFeaturesEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_multi_draw
///typedef struct VkPhysicalDeviceMultiDrawFeaturesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           multiDraw;
///} VkPhysicalDeviceMultiDrawFeaturesEXT;
///```
/// # Members
/// The members of the [`PhysicalDeviceMultiDrawFeaturesEXT`] structure
/// describe the following features:
/// # Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`multi_draw`] indicates that the implementation supports [`cmd_draw_multi_ext`] and
///   [`cmd_draw_multi_indexed_ext`].
/// If the [`PhysicalDeviceMultiDrawFeaturesEXT`] structure is included in the [`p_next`] chain of
/// the
/// [`PhysicalDeviceFeatures2`] structure passed to
/// [`get_physical_device_features2`], it is filled in to indicate whether each
/// corresponding feature is supported.
/// [`PhysicalDeviceMultiDrawFeaturesEXT`] **can**  also be used in the [`p_next`] chain of
/// [`DeviceCreateInfo`] to selectively enable these features.
/// ## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTI_DRAW_FEATURES_EXT`
/// # Related
/// - [`VK_EXT_multi_draw`]
/// - [`Bool32`]
/// - [`StructureType`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPhysicalDeviceMultiDrawFeaturesEXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceMultiDrawFeaturesEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`multi_draw`] indicates that the implementation
    ///supports [`cmd_draw_multi_ext`] and [`cmd_draw_multi_indexed_ext`].
    pub multi_draw: Bool32,
}
impl<'lt> Default for PhysicalDeviceMultiDrawFeaturesEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PHYSICAL_DEVICE_MULTI_DRAW_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            multi_draw: 0,
        }
    }
}
impl<'lt> PhysicalDeviceMultiDrawFeaturesEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *mut BaseOutStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::multi_draw`]
    pub fn multi_draw_raw(&self) -> Bool32 {
        self.multi_draw
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *mut BaseOutStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::multi_draw`]
    pub fn set_multi_draw_raw(mut self, value: Bool32) -> Self {
        self.multi_draw = value;
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
    ///Gets the value of [`Self::multi_draw`]
    pub fn multi_draw(&self) -> bool {
        unsafe { std::mem::transmute(self.multi_draw as u8) }
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
    ///Gets a mutable reference to the value of [`Self::multi_draw`]
    pub fn multi_draw_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.multi_draw as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.multi_draw as *mut Bool32)
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
    ///Sets the value of [`Self::multi_draw`]
    pub fn set_multi_draw(mut self, value: bool) -> Self {
        self.multi_draw = value as u8 as u32;
        self
    }
}
impl CommandBuffer {
    ///[vkCmdDrawMultiEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawMultiEXT.html) - Draw primitives
    ///# C Specifications
    ///To record an ordered sequence of drawing operations which have no state
    ///changes between them, call:
    ///```c
    ///// Provided by VK_EXT_multi_draw
    ///void vkCmdDrawMultiEXT(
    ///    VkCommandBuffer                             commandBuffer,
    ///    uint32_t                                    drawCount,
    ///    const VkMultiDrawInfoEXT*                   pVertexInfo,
    ///    uint32_t                                    instanceCount,
    ///    uint32_t                                    firstInstance,
    ///    uint32_t                                    stride);
    ///```
    /// # Parameters
    /// - [`command_buffer`] is the command buffer into which the command is recorded.
    /// - [`draw_count`] is the number of draws to execute, and  **can**  be zero.
    /// - [`p_vertex_info`] is a pointer to an array of [`MultiDrawInfoEXT`] with vertex information
    ///   to be drawn.
    /// - [`instance_count`] is the number of instances to draw.
    /// - [`first_instance`] is the instance ID of the first instance to draw.
    /// - [`stride`] is the byte stride between consecutive elements of [`p_vertex_info`].
    /// # Description
    /// [`draw_count`] draws are executed with parameters taken from
    /// [`p_vertex_info`].
    /// The number of draw commands recorded is [`draw_count`], with each command
    /// reading, sequentially, a `firstVertex` and a `vertexCount` from
    /// [`p_vertex_info`].
    /// ## Valid Usage
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
    /// - If [`command_buffer`] is a protected command buffer and [`protectedNoFault`]() is not
    ///   supported, any resource written to by the [`Pipeline`] object bound to the pipeline bind
    ///   point used by this command  **must**  not be an unprotected resource
    /// - If [`command_buffer`] is a protected command buffer and [`protectedNoFault`]() is not
    ///   supported, pipeline stages other than the framebuffer-space and compute stages in the
    ///   [`Pipeline`] object bound to the pipeline bind point used by this command  **must**  not
    ///   write to any resource
    /// - If any of the shader stages of the [`Pipeline`] bound to the pipeline bind point used by
    ///   this command uses the [RayQueryKHR]() capability, then [`command_buffer`] **must**  not be
    ///   a protected command buffer
    ///
    /// - All vertex input bindings accessed via vertex input variables declared in the vertex
    ///   shader entry point’s interface  **must**  have either valid or [`crate::Handle::null`]
    ///   buffers bound
    /// - If the [nullDescriptor]() feature is not enabled, all vertex input bindings accessed via
    ///   vertex input variables declared in the vertex shader entry point’s interface  **must**
    ///   not be [`crate::Handle::null`]
    /// - For a given vertex buffer binding, any attribute data fetched  **must**  be entirely
    ///   contained within the corresponding vertex buffer binding, as described in
    ///   [[fxvertex-input]]()
    /// - If the bound graphics pipeline state was created with the
    ///   `VK_DYNAMIC_STATE_PRIMITIVE_TOPOLOGY_EXT` dynamic state enabled then
    ///   [`cmd_set_primitive_topology_ext`] **must**  have been called in the current command
    ///   buffer prior to this drawing command, and the `primitiveTopology` parameter of
    ///   [`cmd_set_primitive_topology_ext`] **must**  be of the same [topology class]() as the
    ///   pipeline [`PipelineInputAssemblyStateCreateInfo::topology`] state
    /// - If the bound graphics pipeline was created with both the
    ///   `VK_DYNAMIC_STATE_VERTEX_INPUT_EXT` and `VK_DYNAMIC_STATE_VERTEX_INPUT_BINDING_STRIDE_EXT`
    ///   dynamic states enabled, then [`cmd_set_vertex_input_ext`] **must**  have been called in
    ///   the current command buffer prior to this draw command
    /// - If the bound graphics pipeline was created with the
    ///   `VK_DYNAMIC_STATE_VERTEX_INPUT_BINDING_STRIDE_EXT` dynamic state enabled, but not the
    ///   `VK_DYNAMIC_STATE_VERTEX_INPUT_EXT` dynamic state enabled, then
    ///   [`cmd_bind_vertex_buffers2_ext`] **must**  have been called in the current command buffer
    ///   prior to this draw command, and the `pStrides` parameter of
    ///   [`cmd_bind_vertex_buffers2_ext`] **must**  not be `NULL`
    /// - If the bound graphics pipeline state was created with the
    ///   `VK_DYNAMIC_STATE_VERTEX_INPUT_EXT` dynamic state enabled, then
    ///   [`cmd_set_vertex_input_ext`] **must**  have been called in the current command buffer
    ///   prior to this draw command
    /// - If the bound graphics pipeline state was created with the
    ///   `VK_DYNAMIC_STATE_PATCH_CONTROL_POINTS_EXT` dynamic state enabled then
    ///   [`cmd_set_patch_control_points_ext`] **must**  have been called in the current command
    ///   buffer prior to this drawing command
    /// - If the bound graphics pipeline state was created with the
    ///   `VK_DYNAMIC_STATE_PRIMITIVE_RESTART_ENABLE_EXT` dynamic state enabled then
    ///   [`cmd_set_primitive_restart_enable_ext`] **must**  have been called in the current command
    ///   buffer prior to this drawing command
    /// - The bound graphics pipeline  **must**  not have been created with the
    ///   [`PipelineShaderStageCreateInfo::stage`] member of an element of
    ///   [`GraphicsPipelineCreateInfo::stages`] set to `VK_SHADER_STAGE_TASK_BIT_NV` or
    ///   `VK_SHADER_STAGE_MESH_BIT_NV`
    /// - The [multiDraw](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-multiDraw)
    ///   feature  **must**  be enabled
    /// - [`draw_count`] **must**  be less than
    ///   [`PhysicalDeviceMultiDrawPropertiesEXT::max_multi_draw_count`]
    /// - If [`draw_count`] is greater than zero, [`p_vertex_info`] **must**  be a valid pointer to
    ///   memory containing one or more valid instances of [`MultiDrawInfoEXT`] structures
    /// - [`stride`] must be a multiple of 4
    ///
    /// ## Valid Usage (Implicit)
    /// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
    /// - [`command_buffer`] **must**  be in the [recording state]()
    /// - The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics
    ///   operations
    /// - This command  **must**  only be called inside of a render pass instance
    ///
    /// ## Host Synchronization
    /// - Host access to [`command_buffer`] **must**  be externally synchronized
    /// - Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**
    ///   be externally synchronized
    ///
    /// ## Command Properties
    /// # Related
    /// - [`VK_EXT_multi_draw`]
    /// - [`CommandBuffer`]
    /// - [`MultiDrawInfoEXT`]
    ///
    /// # Notes and documentation
    /// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    /// This documentation is generated from the Vulkan specification and documentation.
    /// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    /// This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkCmdDrawMultiEXT")]
    #[track_caller]
    #[inline]
    pub unsafe fn cmd_draw_multi_ext(
        self: &Unique<CommandBuffer>,
        p_vertex_info: &[crate::extensions::ext_multi_draw::MultiDrawInfoEXT],
        instance_count: Option<u32>,
        first_instance: Option<u32>,
        stride: Option<u32>,
    ) -> () {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .device()
            .vtable()
            .ext_multi_draw()
            .and_then(|vtable| vtable.cmd_draw_multi_ext())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .device()
            .vtable()
            .ext_multi_draw()
            .and_then(|vtable| vtable.cmd_draw_multi_ext())
            .unwrap_unchecked();
        let draw_count = (|len: usize| len)(p_vertex_info.len()) as _;
        let _return = _function(
            self.as_raw(),
            draw_count,
            p_vertex_info.as_ptr(),
            instance_count.unwrap_or_default() as _,
            first_instance.unwrap_or_default() as _,
            stride.unwrap_or_default() as _,
        );
        ()
    }
}
impl CommandBuffer {
    ///[vkCmdDrawMultiIndexedEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawMultiIndexedEXT.html) - Draw primitives
    ///# C Specifications
    ///To record an ordered sequence of indexed drawing operations which have no
    ///state changes between them, call:
    ///```c
    ///// Provided by VK_EXT_multi_draw
    ///void vkCmdDrawMultiIndexedEXT(
    ///    VkCommandBuffer                             commandBuffer,
    ///    uint32_t                                    drawCount,
    ///    const VkMultiDrawIndexedInfoEXT*            pIndexInfo,
    ///    uint32_t                                    instanceCount,
    ///    uint32_t                                    firstInstance,
    ///    uint32_t                                    stride,
    ///    const int32_t*                              pVertexOffset);
    ///```
    /// # Parameters
    /// - [`command_buffer`] is the command buffer into which the command is recorded.
    /// - [`draw_count`] is the number of draws to execute, and  **can**  be zero.
    /// - [`p_index_info`] is a pointer to an array of [`MultiDrawIndexedInfoEXT`] with index
    ///   information to be drawn.
    /// - [`instance_count`] is the number of instances to draw.
    /// - [`first_instance`] is the instance ID of the first instance to draw.
    /// - [`stride`] is the byte stride between consecutive elements of [`p_index_info`].
    /// - [`p_vertex_offset`] is `NULL` or a pointer to the value added to the vertex index before
    ///   indexing into the vertex buffer. When specified, [`MultiDrawIndexedInfoEXT`]`::offset` is
    ///   ignored.
    /// # Description
    /// [`draw_count`] indexed draws are executed with parameters taken from
    /// [`p_index_info`].
    /// The number of draw commands recorded is [`draw_count`], with each command
    /// reading, sequentially, a `firstIndex` and an `indexCount` from
    /// [`p_index_info`].
    /// If [`p_vertex_offset`] is `NULL`, a `vertexOffset` is also read from
    /// [`p_index_info`], otherwise the value from dereferencing [`p_vertex_offset`]
    /// is used.
    /// ## Valid Usage
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
    /// - If [`command_buffer`] is a protected command buffer and [`protectedNoFault`]() is not
    ///   supported, any resource written to by the [`Pipeline`] object bound to the pipeline bind
    ///   point used by this command  **must**  not be an unprotected resource
    /// - If [`command_buffer`] is a protected command buffer and [`protectedNoFault`]() is not
    ///   supported, pipeline stages other than the framebuffer-space and compute stages in the
    ///   [`Pipeline`] object bound to the pipeline bind point used by this command  **must**  not
    ///   write to any resource
    /// - If any of the shader stages of the [`Pipeline`] bound to the pipeline bind point used by
    ///   this command uses the [RayQueryKHR]() capability, then [`command_buffer`] **must**  not be
    ///   a protected command buffer
    ///
    /// - All vertex input bindings accessed via vertex input variables declared in the vertex
    ///   shader entry point’s interface  **must**  have either valid or [`crate::Handle::null`]
    ///   buffers bound
    /// - If the [nullDescriptor]() feature is not enabled, all vertex input bindings accessed via
    ///   vertex input variables declared in the vertex shader entry point’s interface  **must**
    ///   not be [`crate::Handle::null`]
    /// - For a given vertex buffer binding, any attribute data fetched  **must**  be entirely
    ///   contained within the corresponding vertex buffer binding, as described in
    ///   [[fxvertex-input]]()
    /// - If the bound graphics pipeline state was created with the
    ///   `VK_DYNAMIC_STATE_PRIMITIVE_TOPOLOGY_EXT` dynamic state enabled then
    ///   [`cmd_set_primitive_topology_ext`] **must**  have been called in the current command
    ///   buffer prior to this drawing command, and the `primitiveTopology` parameter of
    ///   [`cmd_set_primitive_topology_ext`] **must**  be of the same [topology class]() as the
    ///   pipeline [`PipelineInputAssemblyStateCreateInfo::topology`] state
    /// - If the bound graphics pipeline was created with both the
    ///   `VK_DYNAMIC_STATE_VERTEX_INPUT_EXT` and `VK_DYNAMIC_STATE_VERTEX_INPUT_BINDING_STRIDE_EXT`
    ///   dynamic states enabled, then [`cmd_set_vertex_input_ext`] **must**  have been called in
    ///   the current command buffer prior to this draw command
    /// - If the bound graphics pipeline was created with the
    ///   `VK_DYNAMIC_STATE_VERTEX_INPUT_BINDING_STRIDE_EXT` dynamic state enabled, but not the
    ///   `VK_DYNAMIC_STATE_VERTEX_INPUT_EXT` dynamic state enabled, then
    ///   [`cmd_bind_vertex_buffers2_ext`] **must**  have been called in the current command buffer
    ///   prior to this draw command, and the `pStrides` parameter of
    ///   [`cmd_bind_vertex_buffers2_ext`] **must**  not be `NULL`
    /// - If the bound graphics pipeline state was created with the
    ///   `VK_DYNAMIC_STATE_VERTEX_INPUT_EXT` dynamic state enabled, then
    ///   [`cmd_set_vertex_input_ext`] **must**  have been called in the current command buffer
    ///   prior to this draw command
    /// - If the bound graphics pipeline state was created with the
    ///   `VK_DYNAMIC_STATE_PATCH_CONTROL_POINTS_EXT` dynamic state enabled then
    ///   [`cmd_set_patch_control_points_ext`] **must**  have been called in the current command
    ///   buffer prior to this drawing command
    /// - If the bound graphics pipeline state was created with the
    ///   `VK_DYNAMIC_STATE_PRIMITIVE_RESTART_ENABLE_EXT` dynamic state enabled then
    ///   [`cmd_set_primitive_restart_enable_ext`] **must**  have been called in the current command
    ///   buffer prior to this drawing command
    /// - The bound graphics pipeline  **must**  not have been created with the
    ///   [`PipelineShaderStageCreateInfo::stage`] member of an element of
    ///   [`GraphicsPipelineCreateInfo::stages`] set to `VK_SHADER_STAGE_TASK_BIT_NV` or
    ///   `VK_SHADER_STAGE_MESH_BIT_NV`
    /// - The [multiDraw](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-multiDraw)
    ///   feature  **must**  be enabled
    /// - (`indexSize` × (`firstIndex` +  `indexCount`) +  `offset`) **must**  be less than or equal
    ///   to the size of the bound index buffer, with `indexSize` being based on the type specified
    ///   by `indexType`, where the index buffer, `indexType`, and `offset` are specified via
    ///   [`cmd_bind_index_buffer`]
    /// - [`draw_count`] **must**  be less than
    ///   [`PhysicalDeviceMultiDrawPropertiesEXT::max_multi_draw_count`]
    /// - If [`draw_count`] is greater than zero, [`p_index_info`] **must**  be a valid pointer to
    ///   memory containing one or more valid instances of [`MultiDrawIndexedInfoEXT`] structures
    /// - [`stride`] must be a multiple of 4
    ///
    /// ## Valid Usage (Implicit)
    /// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
    /// - If [`p_vertex_offset`] is not `NULL`, [`p_vertex_offset`] **must**  be a valid pointer to
    ///   a valid `int32_t` value
    /// - [`command_buffer`] **must**  be in the [recording state]()
    /// - The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics
    ///   operations
    /// - This command  **must**  only be called inside of a render pass instance
    ///
    /// ## Host Synchronization
    /// - Host access to [`command_buffer`] **must**  be externally synchronized
    /// - Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**
    ///   be externally synchronized
    ///
    /// ## Command Properties
    /// # Related
    /// - [`VK_EXT_multi_draw`]
    /// - [`CommandBuffer`]
    /// - [`MultiDrawIndexedInfoEXT`]
    ///
    /// # Notes and documentation
    /// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    /// This documentation is generated from the Vulkan specification and documentation.
    /// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    /// This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkCmdDrawMultiIndexedEXT")]
    #[track_caller]
    #[inline]
    pub unsafe fn cmd_draw_multi_indexed_ext(
        self: &Unique<CommandBuffer>,
        p_index_info: &[crate::extensions::ext_multi_draw::MultiDrawIndexedInfoEXT],
        instance_count: Option<u32>,
        first_instance: Option<u32>,
        stride: Option<u32>,
        p_vertex_offset: Option<&i32>,
    ) -> () {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .device()
            .vtable()
            .ext_multi_draw()
            .and_then(|vtable| vtable.cmd_draw_multi_indexed_ext())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .device()
            .vtable()
            .ext_multi_draw()
            .and_then(|vtable| vtable.cmd_draw_multi_indexed_ext())
            .unwrap_unchecked();
        let draw_count = (|len: usize| len)(p_index_info.len()) as _;
        let _return = _function(
            self.as_raw(),
            draw_count,
            p_index_info.as_ptr(),
            instance_count.unwrap_or_default() as _,
            first_instance.unwrap_or_default() as _,
            stride.unwrap_or_default() as _,
            p_vertex_offset.map(|v| v as *const i32).unwrap_or_else(std::ptr::null),
        );
        ()
    }
}
///The V-table of [`Device`] for functions from `VK_EXT_multi_draw`
pub struct DeviceExtMultiDrawVTable {
    ///See [`FNCmdDrawMultiExt`] for more information.
    pub cmd_draw_multi_ext: FNCmdDrawMultiExt,
    ///See [`FNCmdDrawMultiIndexedExt`] for more information.
    pub cmd_draw_multi_indexed_ext: FNCmdDrawMultiIndexedExt,
}
impl DeviceExtMultiDrawVTable {
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
            cmd_draw_multi_ext: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkCmdDrawMultiEXT").as_ptr()))
            },
            cmd_draw_multi_indexed_ext: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkCmdDrawMultiIndexedEXT").as_ptr()))
            },
        }
    }
    ///Gets [`Self::cmd_draw_multi_ext`]. See [`FNCmdDrawMultiExt`] for more information.
    pub fn cmd_draw_multi_ext(&self) -> FNCmdDrawMultiExt {
        self.cmd_draw_multi_ext
    }
    ///Gets [`Self::cmd_draw_multi_indexed_ext`]. See [`FNCmdDrawMultiIndexedExt`] for more
    /// information.
    pub fn cmd_draw_multi_indexed_ext(&self) -> FNCmdDrawMultiIndexedExt {
        self.cmd_draw_multi_indexed_ext
    }
}
