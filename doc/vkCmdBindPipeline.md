[vkCmdBindPipeline](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBindPipeline.html) - Bind a pipeline object to a command buffer

# C Specifications
Once a pipeline has been created, it  **can**  be bound to the command buffer
using the command:
```c
// Provided by VK_VERSION_1_0
void vkCmdBindPipeline(
    VkCommandBuffer                             commandBuffer,
    VkPipelineBindPoint                         pipelineBindPoint,
    VkPipeline                                  pipeline);
```

# Parameters
- [`command_buffer`] is the command buffer that the pipeline will be bound to.
- [`pipeline_bind_point`] is a [`PipelineBindPoint`] value specifying to which bind point the pipeline is bound. Binding one does not disturb the others.
- [`pipeline`] is the pipeline to be bound.

# Description
Once bound, a pipeline binding affects subsequent commands that interact
with the given pipeline type in the command buffer until a different
pipeline of the same type is bound to the bind point.
Commands that do not interact with the given pipeline type  **must**  not be
affected by the pipeline state.
- The pipeline bound to `VK_PIPELINE_BIND_POINT_COMPUTE` controls the behavior of all [dispatching commands](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#dispatch).
- The pipeline bound to `VK_PIPELINE_BIND_POINT_GRAPHICS` controls the behavior of all [drawing commands](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#drawing).
- The pipeline bound to `VK_PIPELINE_BIND_POINT_RAY_TRACING_KHR` controls the behavior of [`cmd_trace_rays_khr`] and [`cmd_trace_rays_indirect_khr`].
- The pipeline bound to `VK_PIPELINE_BIND_POINT_SUBPASS_SHADING_HUAWEI` controls the behavior of [`cmd_subpass_shading_huawei`].

## Valid Usage
-    If [`pipeline_bind_point`] is `VK_PIPELINE_BIND_POINT_COMPUTE`, the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support compute operations
-    If [`pipeline_bind_point`] is `VK_PIPELINE_BIND_POINT_GRAPHICS`, the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics operations
-    If [`pipeline_bind_point`] is `VK_PIPELINE_BIND_POINT_COMPUTE`, [`pipeline`] **must**  be a compute pipeline
-    If [`pipeline_bind_point`] is `VK_PIPELINE_BIND_POINT_GRAPHICS`, [`pipeline`] **must**  be a graphics pipeline
-    If the [variable multisample rate](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-variableMultisampleRate) feature is not supported, [`pipeline`] is a graphics pipeline, the current subpass [uses no attachments](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#renderpass-noattachments), and this is not the first call to this function with a graphics pipeline after transitioning to the current subpass, then the sample count specified by this pipeline  **must**  match that set in the previous pipeline
-    If [`PhysicalDeviceSampleLocationsPropertiesEXT::variable_sample_locations`] is [`FALSE`], and [`pipeline`] is a graphics pipeline created with a [`PipelineSampleLocationsStateCreateInfoEXT`] structure having its `sampleLocationsEnable` member set to [`TRUE`] but without `VK_DYNAMIC_STATE_SAMPLE_LOCATIONS_EXT` enabled then the current render pass instance  **must**  have been begun by specifying a [`RenderPassSampleLocationsBeginInfoEXT`] structure whose `pPostSubpassSampleLocations` member contains an element with a `subpassIndex` matching the current subpass index and the `sampleLocationsInfo` member of that element  **must**  match the `sampleLocationsInfo` specified in [`PipelineSampleLocationsStateCreateInfoEXT`] when the pipeline was created
-    This command  **must**  not be recorded when transform feedback is active
-    If [`pipeline_bind_point`] is `VK_PIPELINE_BIND_POINT_RAY_TRACING_KHR`, the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support compute operations
-    If [`pipeline_bind_point`] is `VK_PIPELINE_BIND_POINT_RAY_TRACING_KHR`, [`pipeline`] **must**  be a ray tracing pipeline
-  [`pipeline`] **must**  not have been created with `VK_PIPELINE_CREATE_LIBRARY_BIT_KHR` set
-    If [`command_buffer`] is a secondary command buffer with [`CommandBufferInheritanceViewportScissorInfoNV::viewport_scissor2_d`] enabled and [`pipeline_bind_point`] is `VK_PIPELINE_BIND_POINT_GRAPHICS`, then the [`pipeline`] **must**  have been created with `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` or `VK_DYNAMIC_STATE_VIEWPORT`, and `VK_DYNAMIC_STATE_SCISSOR_WITH_COUNT` or `VK_DYNAMIC_STATE_SCISSOR` enabled
-    If [`command_buffer`] is a secondary command buffer with [`CommandBufferInheritanceViewportScissorInfoNV::viewport_scissor2_d`] enabled and [`pipeline_bind_point`] is `VK_PIPELINE_BIND_POINT_GRAPHICS` and [`pipeline`] was created with [`PipelineDiscardRectangleStateCreateInfoEXT`] structure and its `discardRectangleCount` member is not `0`, then the pipeline  **must**  have been created with `VK_DYNAMIC_STATE_DISCARD_RECTANGLE_EXT` enabled
-    If [`pipeline_bind_point`] is `VK_PIPELINE_BIND_POINT_GRAPHICS` and the [provokingVertexModePerPipeline](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-provokingVertexModePerPipeline) limit is [`FALSE`], then pipeline’s [`PipelineRasterizationProvokingVertexStateCreateInfoEXT::provoking_vertex_mode`] **must**  be the same as that of any other pipelines previously bound to this bind point within the current render pass instance, including any pipeline already bound when beginning the render pass instance
-    If [`pipeline_bind_point`] is `VK_PIPELINE_BIND_POINT_SUBPASS_SHADING_HUAWEI`, the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support compute operations
-    If [`pipeline_bind_point`] is `VK_PIPELINE_BIND_POINT_SUBPASS_SHADING_HUAWEI`, [`pipeline`] **must**  be a subpass shading pipeline
-    If [`pipeline`] is a graphics pipeline, this command has been called inside a render pass instance started with [`cmd_begin_rendering`], and commands using the previously bound graphics pipeline have been recorded within the render pass instance, then the value of [`PipelineRenderingCreateInfo::color_attachment_count`] specified by this pipeline  **must**  match that set in the previous pipeline
-    If [`pipeline`] is a graphics pipeline, this command has been called inside a render pass instance started with [`cmd_begin_rendering`], and commands using the previously bound graphics pipeline have been recorded within the render pass instance, then the elements of [`PipelineRenderingCreateInfo::color_attachment_formats`] specified by this pipeline  **must**  match that set in the previous pipeline
-    If [`pipeline`] is a graphics pipeline, this command has been called inside a render pass instance started with [`cmd_begin_rendering`], and commands using the previously bound graphics pipeline have been recorded within the render pass instance, then the value of [`PipelineRenderingCreateInfo::depth_attachment_format`] specified by this pipeline  **must**  match that set in the previous pipeline
-    If [`pipeline`] is a graphics pipeline, this command has been called inside a render pass instance started with [`cmd_begin_rendering`], and commands using the previously bound graphics pipeline have been recorded within the render pass instance, then the value of [`PipelineRenderingCreateInfo::stencil_attachment_format`] specified by this pipeline  **must**  match that set in the previous pipeline

## Valid Usage (Implicit)
-  [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
-  [`pipeline_bind_point`] **must**  be a valid [`PipelineBindPoint`] value
-  [`pipeline`] **must**  be a valid [`Pipeline`] handle
-  [`command_buffer`] **must**  be in the [recording state]()
-    The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics, or compute operations
-    Both of [`command_buffer`], and [`pipeline`] **must**  have been created, allocated, or retrieved from the same [`Device`]

## Host Synchronization
- Host access to [`command_buffer`] **must**  be externally synchronized
- Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be externally synchronized

## Command Properties

# Related
- [`crate::vulkan1_0`]
- [`CommandBuffer`]
- [`Pipeline`]
- [`PipelineBindPoint`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        