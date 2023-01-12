[vkCmdDrawMeshTasksIndirectCountNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawMeshTasksIndirectCountNV.html) - Perform an indirect mesh tasks draw with the draw count sourced from a buffer

# C Specifications
To record an indirect mesh tasks draw with the draw count sourced from a
buffer, call:
```c
// Provided by VK_NV_mesh_shader
void vkCmdDrawMeshTasksIndirectCountNV(
    VkCommandBuffer                             commandBuffer,
    VkBuffer                                    buffer,
    VkDeviceSize                                offset,
    VkBuffer                                    countBuffer,
    VkDeviceSize                                countBufferOffset,
    uint32_t                                    maxDrawCount,
    uint32_t                                    stride);
```

# Parameters
- [`command_buffer`] is the command buffer into which the command is recorded.
- [`buffer`] is the buffer containing draw parameters.
- [`offset`] is the byte offset into [`buffer`] where parameters begin.
- [`count_buffer`] is the buffer containing the draw count.
- [`count_buffer_offset`] is the byte offset into [`count_buffer`] where the draw count begins.
- [`max_draw_count`] specifies the maximum number of draws that will be executed. The actual number of executed draw calls is the minimum of the count specified in [`count_buffer`] and [`max_draw_count`].
- [`stride`] is the byte stride between successive sets of draw parameters.

# Description
[`cmd_draw_mesh_tasks_indirect_count_nv`] behaves similarly to
[`cmd_draw_mesh_tasks_indirect_nv`] except that the draw count is read by the
device from a buffer during execution.
The command will read an unsigned 32-bit integer from [`count_buffer`]
located at [`count_buffer_offset`] and use this as the draw count.
## Valid Usage
-    If a [`Sampler`] created with `magFilter` or `minFilter` equal to `VK_FILTER_LINEAR` and `compareEnable` equal to `VK_FALSE` is used to sample a [`ImageView`] as a result of this command, then the image view’s [format features]() **must**  contain `VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT`
-    If a [`Sampler`] created with `mipmapMode` equal to `VK_SAMPLER_MIPMAP_MODE_LINEAR` and `compareEnable` equal to `VK_FALSE` is used to sample a [`ImageView`] as a result of this command, then the image view’s [format features]() **must**  contain `VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT`
-    If a [`ImageView`] is sampled with [depth comparison](), the image view’s [format features]() **must**  contain `VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_DEPTH_COMPARISON_BIT`
-    If a [`ImageView`] is accessed using atomic operations as a result of this command, then the image view’s [format features]() **must**  contain `VK_FORMAT_FEATURE_STORAGE_IMAGE_ATOMIC_BIT`
-    If a [`ImageView`] is sampled with `VK_FILTER_CUBIC_EXT` as a result of this command, then the image view’s [format features]() **must**  contain `VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_CUBIC_BIT_EXT`
-    Any [`ImageView`] being sampled with `VK_FILTER_CUBIC_EXT` as a result of this command  **must**  have a [`ImageViewType`] and format that supports cubic filtering, as specified by [`FilterCubicImageViewImageFormatPropertiesEXT::filter_cubic`] returned by [`get_physical_device_image_format_properties2`]
-    Any [`ImageView`] being sampled with `VK_FILTER_CUBIC_EXT` with a reduction mode of either `VK_SAMPLER_REDUCTION_MODE_MIN` or `VK_SAMPLER_REDUCTION_MODE_MAX` as a result of this command  **must**  have a [`ImageViewType`] and format that supports cubic filtering together with minmax filtering, as specified by [`FilterCubicImageViewImageFormatPropertiesEXT::filter_cubic_minmax`] returned by [`get_physical_device_image_format_properties2`]
-    Any [`Image`] created with a [`ImageCreateInfo::flags`] containing `VK_IMAGE_CREATE_CORNER_SAMPLED_BIT_NV` sampled as a result of this command  **must**  only be sampled using a [`SamplerAddressMode`] of `VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE`
-    Any [`ImageView`] or [`BufferView`] being written as a storage image or storage texel buffer where the image format field of the `OpTypeImage` is `Unknown` then the view’s format feature  **must**  contain `VK_FORMAT_FEATURE_2_STORAGE_WRITE_WITHOUT_FORMAT_BIT`
-    Any [`ImageView`] or [`BufferView`] being read as a storage image or storage texel buffer where the image format field of the `OpTypeImage` is `Unknown` then the view’s format feature  **must**  contain `VK_FORMAT_FEATURE_2_STORAGE_READ_WITHOUT_FORMAT_BIT`
-    For each set *n* that is statically used by the [`Pipeline`] bound to the pipeline bind point used by this command, a descriptor set  **must**  have been bound to *n* at the same pipeline bind point, with a [`PipelineLayout`] that is compatible for set *n*, with the [`PipelineLayout`] used to create the current [`Pipeline`], as described in [[descriptorsets-compatibility]]()
-    If the [`maintenance4`]() feature is not enabled, then for each push constant that is statically used by the [`Pipeline`] bound to the pipeline bind point used by this command, a push constant value  **must**  have been set for the same pipeline bind point, with a [`PipelineLayout`] that is compatible for push constants, with the [`PipelineLayout`] used to create the current [`Pipeline`], as described in [[descriptorsets-compatibility]]()
-    Descriptors in each bound descriptor set, specified via [`cmd_bind_descriptor_sets`],  **must**  be valid if they are statically used by the [`Pipeline`] bound to the pipeline bind point used by this command
-    A valid pipeline  **must**  be bound to the pipeline bind point used by this command
-    If the [`Pipeline`] object bound to the pipeline bind point used by this command requires any dynamic state, that state  **must**  have been set or inherited (if the `[`nv_inherited_viewport_scissor`]` extension is enabled) for [`command_buffer`], and done so after any previously bound pipeline with the corresponding state not specified as dynamic
-    There  **must**  not have been any calls to dynamic state setting commands for any state not specified as dynamic in the [`Pipeline`] object bound to the pipeline bind point used by this command, since that pipeline was bound
-    If the [`Pipeline`] object bound to the pipeline bind point used by this command accesses a [`Sampler`] object that uses unnormalized coordinates, that sampler  **must**  not be used to sample from any [`Image`] with a [`ImageView`] of the type `VK_IMAGE_VIEW_TYPE_3D`, `VK_IMAGE_VIEW_TYPE_CUBE`, `VK_IMAGE_VIEW_TYPE_1D_ARRAY`, `VK_IMAGE_VIEW_TYPE_2D_ARRAY` or `VK_IMAGE_VIEW_TYPE_CUBE_ARRAY`, in any shader stage
-    If the [`Pipeline`] object bound to the pipeline bind point used by this command accesses a [`Sampler`] object that uses unnormalized coordinates, that sampler  **must**  not be used with any of the SPIR-V `OpImageSample*` or `OpImageSparseSample*` instructions with `ImplicitLod`, `Dref` or `Proj` in their name, in any shader stage
-    If the [`Pipeline`] object bound to the pipeline bind point used by this command accesses a [`Sampler`] object that uses unnormalized coordinates, that sampler  **must**  not be used with any of the SPIR-V `OpImageSample*` or `OpImageSparseSample*` instructions that includes a LOD bias or any offset values, in any shader stage
-    If the [robust buffer access]() feature is not enabled, and if the [`Pipeline`] object bound to the pipeline bind point used by this command accesses a uniform buffer, it  **must**  not access values outside of the range of the buffer as specified in the descriptor set bound to the same pipeline bind point
-    If the [robust buffer access]() feature is not enabled, and if the [`Pipeline`] object bound to the pipeline bind point used by this command accesses a storage buffer, it  **must**  not access values outside of the range of the buffer as specified in the descriptor set bound to the same pipeline bind point
-    If [`command_buffer`] is an unprotected command buffer and [`protectedNoFault`]() is not supported, any resource accessed by the [`Pipeline`] object bound to the pipeline bind point used by this command  **must**  not be a protected resource
-    If the [`Pipeline`] object bound to the pipeline bind point used by this command accesses a [`Sampler`] or [`ImageView`] object that enables [sampler Y′C<sub>B</sub>C<sub>R</sub> conversion](), that object  **must**  only be used with `OpImageSample*` or `OpImageSparseSample*` instructions
-    If the [`Pipeline`] object bound to the pipeline bind point used by this command accesses a [`Sampler`] or [`ImageView`] object that enables [sampler Y′C<sub>B</sub>C<sub>R</sub> conversion](), that object  **must**  not use the `ConstOffset` and `Offset` operands
-    If a [`ImageView`] is accessed using `OpImageWrite` as a result of this command, then the `Type` of the `Texel` operand of that instruction  **must**  have at least as many components as the image view’s format
-    If a [`BufferView`] is accessed using `OpImageWrite` as a result of this command, then the `Type` of the `Texel` operand of that instruction  **must**  have at least as many components as the buffer view’s format
-    If a [`ImageView`] with a [`Format`] that has a 64-bit component width is accessed as a result of this command, the `SampledType` of the `OpTypeImage` operand of that instruction  **must**  have a `Width` of 64
-    If a [`ImageView`] with a [`Format`] that has a component width less than 64-bit is accessed as a result of this command, the `SampledType` of the `OpTypeImage` operand of that instruction  **must**  have a `Width` of 32
-    If a [`BufferView`] with a [`Format`] that has a 64-bit component width is accessed as a result of this command, the `SampledType` of the `OpTypeImage` operand of that instruction  **must**  have a `Width` of 64
-    If a [`BufferView`] with a [`Format`] that has a component width less than 64-bit is accessed as a result of this command, the `SampledType` of the `OpTypeImage` operand of that instruction  **must**  have a `Width` of 32
-    If the [`sparseImageInt64Atomics`]() feature is not enabled, [`Image`] objects created with the `VK_IMAGE_CREATE_SPARSE_RESIDENCY_BIT` flag  **must**  not be accessed by atomic instructions through an `OpTypeImage` with a `SampledType` with a `Width` of 64 by this command
-    If the [`sparseImageInt64Atomics`]() feature is not enabled, [`Buffer`] objects created with the `VK_BUFFER_CREATE_SPARSE_RESIDENCY_BIT` flag  **must**  not be accessed by atomic instructions through an `OpTypeImage` with a `SampledType` with a `Width` of 64 by this command
-    The current render pass  **must**  be [compatible]() with the `renderPass` member of the [`GraphicsPipelineCreateInfo`] structure specified when creating the [`Pipeline`] bound to `VK_PIPELINE_BIND_POINT_GRAPHICS`
-    The subpass index of the current render pass  **must**  be equal to the `subpass` member of the [`GraphicsPipelineCreateInfo`] structure specified when creating the [`Pipeline`] bound to `VK_PIPELINE_BIND_POINT_GRAPHICS`
-    Every input attachment used by the current subpass  **must**  be bound to the pipeline via a descriptor set
-    Memory backing image subresources used as attachments in the current render pass  **must**  not be written in any way other than as an attachment by this command
-    If any recorded command in the current subpass will write to an image subresource as an attachment, this command  **must**  not read from the memory backing that image subresource in any other way than as an attachment
-    If any recorded command in the current subpass will read from an image subresource used as an attachment in any way other than as an attachment, this command  **must**  not write to that image subresource as an attachment
-    If the draw is recorded in a render pass instance with multiview enabled, the maximum instance index  **must**  be less than or equal to [`PhysicalDeviceMultiviewProperties::max_multiview_instance_index`]
-    If the bound graphics pipeline was created with [`PipelineSampleLocationsStateCreateInfoEXT::sample_locations_enable`] set to `VK_TRUE` and the current subpass has a depth/stencil attachment, then that attachment  **must**  have been created with the `VK_IMAGE_CREATE_SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_BIT_EXT` bit set
-    If the bound graphics pipeline state was created with the `VK_DYNAMIC_STATE_SAMPLE_LOCATIONS_EXT` dynamic state enabled then [`cmd_set_sample_locations_ext`] **must**  have been called in the current command buffer prior to this drawing command
-    If the bound graphics pipeline state was created with the `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` dynamic state enabled, but not the `VK_DYNAMIC_STATE_SCISSOR_WITH_COUNT` dynamic state enabled, then [`cmd_set_viewport_with_count`] **must**  have been called in the current command buffer prior to this drawing command, and the `viewportCount` parameter of [`cmd_set_viewport_with_count`] **must**  match the [`PipelineViewportStateCreateInfo::scissor_count`] of the pipeline
-    If the bound graphics pipeline state was created with the `VK_DYNAMIC_STATE_SCISSOR_WITH_COUNT` dynamic state enabled, but not the `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` dynamic state enabled, then [`cmd_set_scissor_with_count`] **must**  have been called in the current command buffer prior to this drawing command, and the `scissorCount` parameter of [`cmd_set_scissor_with_count`] **must**  match the [`PipelineViewportStateCreateInfo::viewport_count`] of the pipeline
-    If the bound graphics pipeline state was created with both the `VK_DYNAMIC_STATE_SCISSOR_WITH_COUNT` and `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` dynamic states enabled then both [`cmd_set_viewport_with_count`] and [`cmd_set_scissor_with_count`] **must**  have been called in the current command buffer prior to this drawing command, and the `viewportCount` parameter of [`cmd_set_viewport_with_count`] **must**  match the `scissorCount` parameter of [`cmd_set_scissor_with_count`]
-    If the bound graphics pipeline state was created with the `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` dynamic state enabled, but not the `VK_DYNAMIC_STATE_VIEWPORT_W_SCALING_NV` dynamic state enabled, then the bound graphics pipeline  **must**  have been created with [`PipelineViewportWScalingStateCreateInfoNV::viewport_count`] greater or equal to the `viewportCount` parameter in the last call to [`cmd_set_viewport_with_count`]
-    If the bound graphics pipeline state was created with the `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` and `VK_DYNAMIC_STATE_VIEWPORT_W_SCALING_NV` dynamic states enabled then the `viewportCount` parameter in the last call to [`cmd_set_viewport_w_scaling_nv`] **must**  be greater than or equal to the `viewportCount` parameter in the last call to [`cmd_set_viewport_with_count`]
-    If the bound graphics pipeline state was created with the `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` dynamic state enabled, but not the `VK_DYNAMIC_STATE_VIEWPORT_SHADING_RATE_PALETTE_NV` dynamic state enabled, then the bound graphics pipeline  **must**  have been created with [`PipelineViewportShadingRateImageStateCreateInfoNV::viewport_count`] greater or equal to the `viewportCount` parameter in the last call to [`cmd_set_viewport_with_count`]
-    If the bound graphics pipeline state was created with the `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` and `VK_DYNAMIC_STATE_VIEWPORT_SHADING_RATE_PALETTE_NV` dynamic states enabled then the `viewportCount` parameter in the last call to [`cmd_set_viewport_shading_rate_palette_nv`] **must**  be greater than or equal to the `viewportCount` parameter in the last call to [`cmd_set_viewport_with_count`]
-    If the bound graphics pipeline state was created with the `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` dynamic state enabled and a [`PipelineViewportSwizzleStateCreateInfoNV`] structure chained from `VkPipelineVieportCreateInfo`, then the bound graphics pipeline  **must**  have been created with [`PipelineViewportSwizzleStateCreateInfoNV::viewport_count`] greater or equal to the `viewportCount` parameter in the last call to [`cmd_set_viewport_with_count`]
-    If the bound graphics pipeline state was created with the `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` dynamic state enabled and a [`PipelineViewportExclusiveScissorStateCreateInfoNV`] structure chained from `VkPipelineVieportCreateInfo`, then the bound graphics pipeline  **must**  have been created with [`PipelineViewportExclusiveScissorStateCreateInfoNV::exclusive_scissor_count`] greater or equal to the `viewportCount` parameter in the last call to [`cmd_set_viewport_with_count`]
-    If the bound graphics pipeline state was created with the `VK_DYNAMIC_STATE_RASTERIZER_DISCARD_ENABLE` dynamic state enabled then [`cmd_set_rasterizer_discard_enable`] **must**  have been called in the current command buffer prior to this drawing command
-    If the bound graphics pipeline state was created with the `VK_DYNAMIC_STATE_DEPTH_BIAS_ENABLE` dynamic state enabled then [`cmd_set_depth_bias_enable`] **must**  have been called in the current command buffer prior to this drawing command
-    If the bound graphics pipeline state was created with the `VK_DYNAMIC_STATE_LOGIC_OP_EXT` dynamic state enabled then [`cmd_set_logic_op_ext`] **must**  have been called in the current command buffer prior to this drawing command and the `logicOp` **must**  be a valid [`LogicOp`] value
-    If the [`primitiveFragmentShadingRateWithMultipleViewports`]() limit is not supported, the bound graphics pipeline was created with the `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` dynamic state enabled, and any of the shader stages of the bound graphics pipeline write to the `PrimitiveShadingRateKHR` built-in, then [`cmd_set_viewport_with_count`] **must**  have been called in the current command buffer prior to this drawing command, and the `viewportCount` parameter of [`cmd_set_viewport_with_count`] **must**  be `1`
-    If rasterization is not disabled in the bound graphics pipeline, then for each color attachment in the subpass, if the corresponding image view’s [format features]() do not contain `VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BLEND_BIT`, then the `blendEnable` member of the corresponding element of the `pAttachments` member of `pColorBlendState` **must**  be `VK_FALSE`
-    If rasterization is not disabled in the bound graphics pipeline, and neither the `[`amd_mixed_attachment_samples`]` nor the `[`nv_framebuffer_mixed_samples`]` extensions are enabled, then [`PipelineMultisampleStateCreateInfo::rasterization_samples`] **must**  be the same as the current subpass color and/or depth/stencil attachments
-    If the current render pass instance was begun with [`cmd_begin_rendering`], the `imageView` member of `pDepthAttachment` is not [`crate::Handle::null`], and the `layout` member of `pDepthAttachment` is `VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL`, this command  **must**  not write any values to the depth attachment
-    If the current render pass instance was begun with [`cmd_begin_rendering`], the `imageView` member of `pStencilAttachment` is not [`crate::Handle::null`], and the `layout` member of `pStencilAttachment` is `VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL`, this command  **must**  not write any values to the stencil attachment
-    If the current render pass instance was begun with [`cmd_begin_rendering`], the `imageView` member of `pDepthAttachment` is not [`crate::Handle::null`], and the `layout` member of `pDepthAttachment` is `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL`, this command  **must**  not write any values to the depth attachment
-    If the current render pass instance was begun with [`cmd_begin_rendering`], the `imageView` member of `pStencilAttachment` is not [`crate::Handle::null`], and the `layout` member of `pStencilAttachment` is `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL`, this command  **must**  not write any values to the stencil attachment
-    If the current render pass instance was begun with [`cmd_begin_rendering`], the `imageView` member of `pDepthAttachment` is not [`crate::Handle::null`], and the `layout` member of `pDepthAttachment` is `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL`, this command  **must**  not write any values to the depth attachment
-    If the current render pass instance was begun with [`cmd_begin_rendering`], the `imageView` member of `pStencilAttachment` is not [`crate::Handle::null`], and the `layout` member of `pStencilAttachment` is `VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL`, this command  **must**  not write any values to the stencil attachment
-    If the current render pass instance was begun with [`cmd_begin_rendering`], the currently bound graphics pipeline  **must**  have been created with a [`PipelineRenderingCreateInfo::view_mask`] equal to [`RenderingInfo::view_mask`]
-    If the current render pass instance was begun with [`cmd_begin_rendering`], the currently bound graphics pipeline  **must**  have been created with a [`PipelineRenderingCreateInfo::color_attachment_count`] equal to [`RenderingInfo::color_attachment_count`]
-    If the current render pass instance was begun with [`cmd_begin_rendering`] and [`RenderingInfo::color_attachment_count`] greater than `0`, then each element of the [`RenderingInfo::color_attachments`] array with a `imageView` not equal to [`crate::Handle::null`] **must**  have been created with a [`Format`] equal to the corresponding element of [`PipelineRenderingCreateInfo::color_attachment_formats`] used to create the currently bound graphics pipeline
-    If the bound graphics pipeline state was created with the `VK_DYNAMIC_STATE_COLOR_WRITE_ENABLE_EXT` dynamic state enabled then [`cmd_set_color_write_enable_ext`] **must**  have been called in the current command buffer prior to this drawing command, and the `attachmentCount` parameter of [`cmd_set_color_write_enable_ext`] **must**  be equal to the [`PipelineColorBlendStateCreateInfo::attachment_count`] of the currently bound graphics pipeline
-    If the current render pass instance was begun with [`cmd_begin_rendering`] and [`RenderingInfo`]::`pDepthAttachment->pname`:imageView was not [`crate::Handle::null`], the value of [`PipelineRenderingCreateInfo::depth_attachment_format`] used to create the currently bound graphics pipeline  **must**  be equal to the [`Format`] used to create [`RenderingInfo`]::`pDepthAttachment->pname`:imageView
-    If the current render pass instance was begun with [`cmd_begin_rendering`] and [`RenderingInfo`]::`pStencilAttachment->pname`:imageView was not [`crate::Handle::null`], the value of [`PipelineRenderingCreateInfo::stencil_attachment_format`] used to create the currently bound graphics pipeline  **must**  be equal to the [`Format`] used to create [`RenderingInfo`]::`pStencilAttachment->pname`:imageView
-    If the current render pass instance was begun with [`cmd_begin_rendering`] and [`RenderingFragmentShadingRateAttachmentInfoKHR::image_view`] was not [`crate::Handle::null`], the currently bound graphics pipeline  **must**  have been created with `VK_PIPELINE_CREATE_RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR`
-    If the current render pass instance was begun with [`cmd_begin_rendering`] and [`RenderingFragmentDensityMapAttachmentInfoEXT::image_view`] was not [`crate::Handle::null`], the currently bound graphics pipeline  **must**  have been created with `VK_PIPELINE_CREATE_RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_BIT_EXT`
-    If the currently bound pipeline was created with a [`AttachmentSampleCountInfoAMD`] or [`AttachmentSampleCountInfoNV`] structure, and the current render pass instance was begun with [`cmd_begin_rendering`] with a [`RenderingInfo::color_attachment_count`] parameter greater than `0`, then each element of the [`RenderingInfo::color_attachments`] array with a `imageView` not equal to [`crate::Handle::null`] **must**  have been created with a sample count equal to the corresponding element of the `pColorAttachmentSamples` member of [`AttachmentSampleCountInfoAMD`] or [`AttachmentSampleCountInfoNV`] used to create the currently bound graphics pipeline
-    If the current render pass instance was begun with [`cmd_begin_rendering`], the currently bound pipeline was created with a [`AttachmentSampleCountInfoAMD`] or [`AttachmentSampleCountInfoNV`] structure, and [`RenderingInfo`]::`pDepthAttachment->pname`:imageView was not [`crate::Handle::null`], the value of the `depthStencilAttachmentSamples` member of [`AttachmentSampleCountInfoAMD`] or [`AttachmentSampleCountInfoNV`] used to create the currently bound graphics pipeline  **must**  be equal to the sample count used to create [`RenderingInfo`]::`pDepthAttachment->pname`:imageView
-    If the current render pass instance was begun with [`cmd_begin_rendering`], the currently bound pipeline was created with a [`AttachmentSampleCountInfoAMD`] or [`AttachmentSampleCountInfoNV`] structure, and [`RenderingInfo`]::`pStencilAttachment->pname`:imageView was not [`crate::Handle::null`], the value of the `depthStencilAttachmentSamples` member of [`AttachmentSampleCountInfoAMD`] or [`AttachmentSampleCountInfoNV`] used to create the currently bound graphics pipeline  **must**  be equal to the sample count used to create [`RenderingInfo`]::`pStencilAttachment->pname`:imageView
-    If the currently bound pipeline was created without a [`AttachmentSampleCountInfoAMD`] or [`AttachmentSampleCountInfoNV`] structure, and the current render pass instance was begun with [`cmd_begin_rendering`] with a [`RenderingInfo::color_attachment_count`] parameter greater than `0`, then each element of the [`RenderingInfo::color_attachments`] array with a `imageView` not equal to [`crate::Handle::null`] **must**  have been created with a sample count equal to the value of [`PipelineMultisampleStateCreateInfo::rasterization_samples`] used to create the currently bound graphics pipeline
-    If the current render pass instance was begun with [`cmd_begin_rendering`], the currently bound pipeline was created without a [`AttachmentSampleCountInfoAMD`] or [`AttachmentSampleCountInfoNV`] structure, and [`RenderingInfo`]::`pDepthAttachment->pname`:imageView was not [`crate::Handle::null`], the value of [`PipelineMultisampleStateCreateInfo::rasterization_samples`] used to create the currently bound graphics pipeline  **must**  be equal to the sample count used to create [`RenderingInfo`]::`pDepthAttachment->pname`:imageView
-    If the current render pass instance was begun with [`cmd_begin_rendering`], the currently bound pipeline was created without a [`AttachmentSampleCountInfoAMD`] or [`AttachmentSampleCountInfoNV`] structure, and [`RenderingInfo`]::`pStencilAttachment->pname`:imageView was not [`crate::Handle::null`], the value of [`PipelineMultisampleStateCreateInfo::rasterization_samples`] used to create the currently bound graphics pipeline  **must**  be equal to the sample count used to create [`RenderingInfo`]::`pStencilAttachment->pname`:imageView
-    If the current render pass instance was begun with [`cmd_begin_rendering`], the currently bound pipeline  **must**  have been created with a [`GraphicsPipelineCreateInfo::render_pass`] equal to [`crate::Handle::null`]

-    The bound graphics pipeline  **must**  not have been created with the [`PipelineShaderStageCreateInfo::stage`] member of an element of [`GraphicsPipelineCreateInfo::stages`] set to `VK_SHADER_STAGE_VERTEX_BIT`, `VK_SHADER_STAGE_TESSELLATION_CONTROL_BIT`, `VK_SHADER_STAGE_TESSELLATION_EVALUATION_BIT` or `VK_SHADER_STAGE_GEOMETRY_BIT`

-    If [`buffer`] is non-sparse then it  **must**  be bound completely and contiguously to a single [`DeviceMemory`] object
-  [`buffer`] **must**  have been created with the `VK_BUFFER_USAGE_INDIRECT_BUFFER_BIT` bit set
-  [`offset`] **must**  be a multiple of `4`
-  [`command_buffer`] **must**  not be a protected command buffer

-    If [`count_buffer`] is non-sparse then it  **must**  be bound completely and contiguously to a single [`DeviceMemory`] object
-  [`count_buffer`] **must**  have been created with the `VK_BUFFER_USAGE_INDIRECT_BUFFER_BIT` bit set
-  [`count_buffer_offset`] **must**  be a multiple of `4`
-    The count stored in [`count_buffer`] **must**  be less than or equal to [`PhysicalDeviceLimits::max_draw_indirect_count`]
-  ([`count_buffer_offset`] +  `sizeof`(uint32_t)) **must**  be less than or equal to the size of [`count_buffer`]
-    If [drawIndirectCount]() is not enabled this function  **must**  not be used
-  [`stride`] **must**  be a multiple of `4` and  **must**  be greater than or equal to `sizeof`([`DrawMeshTasksIndirectCommandNV`])
-    If [`max_draw_count`] is greater than or equal to `1`, ([`stride`] × ([`max_draw_count`] - 1) +  [`offset`] +  `sizeof`([`DrawMeshTasksIndirectCommandNV`])) **must**  be less than or equal to the size of [`buffer`]
-    If the count stored in [`count_buffer`] is equal to `1`, ([`offset`] +  `sizeof`([`DrawMeshTasksIndirectCommandNV`])) **must**  be less than or equal to the size of [`buffer`]
-    If the count stored in [`count_buffer`] is greater than `1`, ([`stride`] × (`drawCount` - 1) +  [`offset`] +  `sizeof`([`DrawMeshTasksIndirectCommandNV`])) **must**  be less than or equal to the size of [`buffer`]

## Valid Usage (Implicit)
-  [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
-  [`buffer`] **must**  be a valid [`Buffer`] handle
-  [`count_buffer`] **must**  be a valid [`Buffer`] handle
-  [`command_buffer`] **must**  be in the [recording state]()
-    The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics operations
-    This command  **must**  only be called inside of a render pass instance
-    Each of [`buffer`], [`command_buffer`], and [`count_buffer`] **must**  have been created, allocated, or retrieved from the same [`Device`]

## Host Synchronization
- Host access to [`command_buffer`] **must**  be externally synchronized
- Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be externally synchronized

## Command Properties

# Related
- [`nv_mesh_shader`]
- [`Buffer`]
- [`CommandBuffer`]
- [`DeviceSize`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        