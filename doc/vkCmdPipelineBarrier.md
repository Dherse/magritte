[vkCmdPipelineBarrier](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdPipelineBarrier.html) - Insert a memory dependency

# C Specifications
To record a pipeline barrier, call:
```c
// Provided by VK_VERSION_1_0
void vkCmdPipelineBarrier(
    VkCommandBuffer                             commandBuffer,
    VkPipelineStageFlags                        srcStageMask,
    VkPipelineStageFlags                        dstStageMask,
    VkDependencyFlags                           dependencyFlags,
    uint32_t                                    memoryBarrierCount,
    const VkMemoryBarrier*                      pMemoryBarriers,
    uint32_t                                    bufferMemoryBarrierCount,
    const VkBufferMemoryBarrier*                pBufferMemoryBarriers,
    uint32_t                                    imageMemoryBarrierCount,
    const VkImageMemoryBarrier*                 pImageMemoryBarriers);
```

# Parameters
- [`command_buffer`] is the command buffer into which the command is recorded.
- [`src_stage_mask`] is a bitmask of [`PipelineStageFlagBits`] specifying the [source stages](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-pipeline-stages-masks).
- [`dst_stage_mask`] is a bitmask of [`PipelineStageFlagBits`] specifying the [destination stages](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-pipeline-stages-masks).
- [`dependency_flags`] is a bitmask of [`DependencyFlagBits`] specifying how execution and memory dependencies are formed.
- [`memory_barrier_count`] is the length of the [`p_memory_barriers`] array.
- [`p_memory_barriers`] is a pointer to an array of [`MemoryBarrier`] structures.
- [`buffer_memory_barrier_count`] is the length of the [`p_buffer_memory_barriers`] array.
- [`p_buffer_memory_barriers`] is a pointer to an array of [`BufferMemoryBarrier`] structures.
- [`image_memory_barrier_count`] is the length of the [`p_image_memory_barriers`] array.
- [`p_image_memory_barriers`] is a pointer to an array of [`ImageMemoryBarrier`] structures.

# Description
[`cmd_pipeline_barrier`] operates almost identically to
[`cmd_pipeline_barrier2`], except that the scopes and barriers are defined
as direct parameters rather than being defined by an [`DependencyInfo`].When [`cmd_pipeline_barrier`] is submitted to a queue, it defines a memory
dependency between commands that were submitted before it, and those
submitted after it.If [`cmd_pipeline_barrier`] was recorded outside a render pass instance,
the first [synchronization scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-scopes)
includes all commands that occur earlier in
[submission order](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-submission-order).
If [`cmd_pipeline_barrier`] was recorded inside a render pass instance,
the first synchronization scope includes only commands that occur earlier in
[submission order](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-submission-order) within the same
subpass.
In either case, the first synchronization scope is limited to operations on
the pipeline stages determined by the
[source stage mask](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-pipeline-stages-masks) specified by
[`src_stage_mask`].If [`cmd_pipeline_barrier`] was recorded outside a render pass instance,
the second [synchronization scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-scopes)
includes all commands that occur later in
[submission order](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-submission-order).
If [`cmd_pipeline_barrier`] was recorded inside a render pass instance,
the second synchronization scope includes only commands that occur later in
[submission order](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-submission-order) within the same
subpass.
In either case, the second synchronization scope is limited to operations on
the pipeline stages determined by the
[destination stage mask](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-pipeline-stages-masks) specified
by [`dst_stage_mask`].The first [access scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-access-scopes) is
limited to accesses in the pipeline stages determined by the
[source stage mask](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-pipeline-stages-masks) specified by
[`src_stage_mask`].
Within that, the first access scope only includes the first access scopes
defined by elements of the [`p_memory_barriers`],
[`p_buffer_memory_barriers`] and [`p_image_memory_barriers`] arrays, which
each define a set of [memory barriers](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-memory-barriers).
If no memory barriers are specified, then the first access scope includes no
accesses.The second [access scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-access-scopes) is
limited to accesses in the pipeline stages determined by the
[destination stage mask](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-pipeline-stages-masks) specified
by [`dst_stage_mask`].
Within that, the second access scope only includes the second access scopes
defined by elements of the [`p_memory_barriers`],
[`p_buffer_memory_barriers`] and [`p_image_memory_barriers`] arrays, which
each define a set of [memory barriers](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-memory-barriers).
If no memory barriers are specified, then the second access scope includes
no accesses.If [`dependency_flags`] includes `VK_DEPENDENCY_BY_REGION_BIT`, then
any dependency between [framebuffer-space](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-framebuffer-regions) pipeline stages is
[framebuffer-local](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-framebuffer-regions) - otherwise it is
[framebuffer-global](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-framebuffer-regions).
## Valid Usage
-    If the [geometry shaders](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-geometryShader) feature is not enabled, [`src_stage_mask`] **must**  not contain `VK_PIPELINE_STAGE_GEOMETRY_SHADER_BIT`
-    If the [tessellation shaders](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-tessellationShader) feature is not enabled, [`src_stage_mask`] **must**  not contain `VK_PIPELINE_STAGE_TESSELLATION_CONTROL_SHADER_BIT` or `VK_PIPELINE_STAGE_TESSELLATION_EVALUATION_SHADER_BIT`
-    If the [conditional rendering](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-conditionalRendering) feature is not enabled, [`src_stage_mask`] **must**  not contain `VK_PIPELINE_STAGE_CONDITIONAL_RENDERING_BIT_EXT`
-    If the [fragment density map](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-fragmentDensityMap) feature is not enabled, [`src_stage_mask`] **must**  not contain `VK_PIPELINE_STAGE_FRAGMENT_DENSITY_PROCESS_BIT_EXT`
-    If the [transform feedback](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-transformFeedback) feature is not enabled, [`src_stage_mask`] **must**  not contain `VK_PIPELINE_STAGE_TRANSFORM_FEEDBACK_BIT_EXT`
-    If the [mesh shaders](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-meshShader) feature is not enabled, [`src_stage_mask`] **must**  not contain `VK_PIPELINE_STAGE_MESH_SHADER_BIT_NV`
-    If the [task shaders](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-taskShader) feature is not enabled, [`src_stage_mask`] **must**  not contain `VK_PIPELINE_STAGE_TASK_SHADER_BIT_NV`
-    If the [shading rate image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-shadingRateImage) feature is not enabled, [`src_stage_mask`] **must**  not contain `VK_PIPELINE_STAGE_SHADING_RATE_IMAGE_BIT_NV`
-    If the [`synchronization2`](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-synchronization2) feature is not enabled, [`src_stage_mask`] **must**  not be `0`

-    If the [geometry shaders](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-geometryShader) feature is not enabled, [`dst_stage_mask`] **must**  not contain `VK_PIPELINE_STAGE_GEOMETRY_SHADER_BIT`
-    If the [tessellation shaders](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-tessellationShader) feature is not enabled, [`dst_stage_mask`] **must**  not contain `VK_PIPELINE_STAGE_TESSELLATION_CONTROL_SHADER_BIT` or `VK_PIPELINE_STAGE_TESSELLATION_EVALUATION_SHADER_BIT`
-    If the [conditional rendering](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-conditionalRendering) feature is not enabled, [`dst_stage_mask`] **must**  not contain `VK_PIPELINE_STAGE_CONDITIONAL_RENDERING_BIT_EXT`
-    If the [fragment density map](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-fragmentDensityMap) feature is not enabled, [`dst_stage_mask`] **must**  not contain `VK_PIPELINE_STAGE_FRAGMENT_DENSITY_PROCESS_BIT_EXT`
-    If the [transform feedback](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-transformFeedback) feature is not enabled, [`dst_stage_mask`] **must**  not contain `VK_PIPELINE_STAGE_TRANSFORM_FEEDBACK_BIT_EXT`
-    If the [mesh shaders](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-meshShader) feature is not enabled, [`dst_stage_mask`] **must**  not contain `VK_PIPELINE_STAGE_MESH_SHADER_BIT_NV`
-    If the [task shaders](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-taskShader) feature is not enabled, [`dst_stage_mask`] **must**  not contain `VK_PIPELINE_STAGE_TASK_SHADER_BIT_NV`
-    If the [shading rate image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-shadingRateImage) feature is not enabled, [`dst_stage_mask`] **must**  not contain `VK_PIPELINE_STAGE_SHADING_RATE_IMAGE_BIT_NV`
-    If the [`synchronization2`](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-synchronization2) feature is not enabled, [`dst_stage_mask`] **must**  not be `0`

-    The `srcAccessMask` member of each element of [`p_memory_barriers`] **must**  only include access flags that are supported by one or more of the pipeline stages in [`src_stage_mask`], as specified in the [table of supported access types]()
-    The `dstAccessMask` member of each element of [`p_memory_barriers`] **must**  only include access flags that are supported by one or more of the pipeline stages in [`dst_stage_mask`], as specified in the [table of supported access types]()
-    For any element of [`p_buffer_memory_barriers`], if its `srcQueueFamilyIndex` and `dstQueueFamilyIndex` members are equal, or if its `srcQueueFamilyIndex` is the queue family index that was used to create the command pool that [`command_buffer`] was allocated from, then its `srcAccessMask` member  **must**  only contain access flags that are supported by one or more of the pipeline stages in [`src_stage_mask`], as specified in the [table of supported access types]()
-    For any element of [`p_buffer_memory_barriers`], if its `srcQueueFamilyIndex` and `dstQueueFamilyIndex` members are equal, or if its `dstQueueFamilyIndex` is the queue family index that was used to create the command pool that [`command_buffer`] was allocated from, then its `dstAccessMask` member  **must**  only contain access flags that are supported by one or more of the pipeline stages in [`dst_stage_mask`], as specified in the [table of supported access types]()
-    For any element of [`p_image_memory_barriers`], if its `srcQueueFamilyIndex` and `dstQueueFamilyIndex` members are equal, or if its `srcQueueFamilyIndex` is the queue family index that was used to create the command pool that [`command_buffer`] was allocated from, then its `srcAccessMask` member  **must**  only contain access flags that are supported by one or more of the pipeline stages in [`src_stage_mask`], as specified in the [table of supported access types]()
-    For any element of [`p_image_memory_barriers`], if its `srcQueueFamilyIndex` and `dstQueueFamilyIndex` members are equal, or if its `dstQueueFamilyIndex` is the queue family index that was used to create the command pool that [`command_buffer`] was allocated from, then its `dstAccessMask` member  **must**  only contain access flags that are supported by one or more of the pipeline stages in [`dst_stage_mask`], as specified in the [table of supported access types]()

-    If [`cmd_pipeline_barrier`] is called within a render pass instance, the render pass  **must**  have been created with at least one [`SubpassDependency`] instance in [`RenderPassCreateInfo::dependencies`] that expresses a dependency from the current subpass to itself, with [synchronization scopes]() and [access scopes]() that are all supersets of the scopes defined in this command
-    If [`cmd_pipeline_barrier`] is called within a render pass instance, it  **must**  not include any buffer memory barriers
-    If [`cmd_pipeline_barrier`] is called within a render pass instance, the `image` member of any image memory barrier included in this command  **must**  be an attachment used in the current subpass both as an input attachment, and as either a color or depth/stencil attachment
-    If [`cmd_pipeline_barrier`] is called within a render pass instance, the `oldLayout` and `newLayout` members of any image memory barrier included in this command  **must**  be equal
-    If [`cmd_pipeline_barrier`] is called within a render pass instance, the `srcQueueFamilyIndex` and `dstQueueFamilyIndex` members of any image memory barrier included in this command  **must**  be equal
-    If [`cmd_pipeline_barrier`] is called outside of a render pass instance, `VK_DEPENDENCY_VIEW_LOCAL_BIT` **must**  not be included in the dependency flags
-    If [`cmd_pipeline_barrier`] is called within a render pass instance, the render pass  **must**  not have been started with [`cmd_begin_rendering`]
-    Any pipeline stage included in [`src_stage_mask`] **must**  be supported by the capabilities of the queue family specified by the `queueFamilyIndex` member of the [`CommandPoolCreateInfo`] structure that was used to create the [`CommandPool`] that [`command_buffer`] was allocated from, as specified in the [table of supported pipeline stages](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-pipeline-stages-supported)
-    Any pipeline stage included in [`dst_stage_mask`] **must**  be supported by the capabilities of the queue family specified by the `queueFamilyIndex` member of the [`CommandPoolCreateInfo`] structure that was used to create the [`CommandPool`] that [`command_buffer`] was allocated from, as specified in the [table of supported pipeline stages](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-pipeline-stages-supported)

## Valid Usage (Implicit)
-  [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
-  [`src_stage_mask`] **must**  be a valid combination of [`PipelineStageFlagBits`] values
-  [`dst_stage_mask`] **must**  be a valid combination of [`PipelineStageFlagBits`] values
-  [`dependency_flags`] **must**  be a valid combination of [`DependencyFlagBits`] values
-    If [`memory_barrier_count`] is not `0`, [`p_memory_barriers`] **must**  be a valid pointer to an array of [`memory_barrier_count`] valid [`MemoryBarrier`] structures
-    If [`buffer_memory_barrier_count`] is not `0`, [`p_buffer_memory_barriers`] **must**  be a valid pointer to an array of [`buffer_memory_barrier_count`] valid [`BufferMemoryBarrier`] structures
-    If [`image_memory_barrier_count`] is not `0`, [`p_image_memory_barriers`] **must**  be a valid pointer to an array of [`image_memory_barrier_count`] valid [`ImageMemoryBarrier`] structures
-  [`command_buffer`] **must**  be in the [recording state]()
-    The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support transfer, graphics, or compute operations

## Host Synchronization
- Host access to [`command_buffer`] **must**  be externally synchronized
- Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be externally synchronized

## Command Properties

# Related
- [`crate::vulkan1_0`]
- [`BufferMemoryBarrier`]
- [`CommandBuffer`]
- [`DependencyFlags`]
- [`ImageMemoryBarrier`]
- [`MemoryBarrier`]
- [`PipelineStageFlags`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        