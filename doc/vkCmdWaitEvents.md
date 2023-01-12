[vkCmdWaitEvents](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdWaitEvents.html) - Wait for one or more events and insert a set of memory

# C Specifications
To wait for one or more events to enter the signaled state on a device,
call:
```c
// Provided by VK_VERSION_1_0
void vkCmdWaitEvents(
    VkCommandBuffer                             commandBuffer,
    uint32_t                                    eventCount,
    const VkEvent*                              pEvents,
    VkPipelineStageFlags                        srcStageMask,
    VkPipelineStageFlags                        dstStageMask,
    uint32_t                                    memoryBarrierCount,
    const VkMemoryBarrier*                      pMemoryBarriers,
    uint32_t                                    bufferMemoryBarrierCount,
    const VkBufferMemoryBarrier*                pBufferMemoryBarriers,
    uint32_t                                    imageMemoryBarrierCount,
    const VkImageMemoryBarrier*                 pImageMemoryBarriers);
```

# Parameters
- [`command_buffer`] is the command buffer into which the command is recorded.
- [`event_count`] is the length of the [`p_events`] array.
- [`p_events`] is a pointer to an array of event object handles to wait on.
- [`src_stage_mask`] is a bitmask of [`PipelineStageFlagBits`] specifying the [source stage mask](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-pipeline-stages).
- [`dst_stage_mask`] is a bitmask of [`PipelineStageFlagBits`] specifying the [destination stage mask](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-pipeline-stages).
- [`memory_barrier_count`] is the length of the [`p_memory_barriers`] array.
- [`p_memory_barriers`] is a pointer to an array of [`MemoryBarrier`] structures.
- [`buffer_memory_barrier_count`] is the length of the [`p_buffer_memory_barriers`] array.
- [`p_buffer_memory_barriers`] is a pointer to an array of [`BufferMemoryBarrier`] structures.
- [`image_memory_barrier_count`] is the length of the [`p_image_memory_barriers`] array.
- [`p_image_memory_barriers`] is a pointer to an array of [`ImageMemoryBarrier`] structures.

# Description
[`cmd_wait_events`] is largely similar to [`cmd_wait_events2`], but  **can** 
only wait on signal operations defined by [`cmd_set_event`].
As [`cmd_set_event`] does not define any access scopes,
[`cmd_wait_events`] defines the first access scope for each event signal
operation in addition to its own access scopes.When [`cmd_wait_events`] is submitted to a queue, it defines a memory
dependency between prior event signal operations on the same queue or the
host, and subsequent commands.
[`cmd_wait_events`] **must**  not be used to wait on event signal operations
occurring on other queues.The first synchronization scope only includes event signal operations that
operate on members of [`p_events`], and the operations that happened-before
the event signal operations.
Event signal operations performed by [`cmd_set_event`] that occur earlier
in [submission order](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-submission-order) are included in the
first synchronization scope, if the [logically latest](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-pipeline-stages-order) pipeline stage in their `stageMask` parameter is
[logically earlier](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-pipeline-stages-order) than or equal
to the [logically latest](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-pipeline-stages-order) pipeline
stage in [`src_stage_mask`].
Event signal operations performed by [`set_event`] are only included in
the first synchronization scope if `VK_PIPELINE_STAGE_HOST_BIT` is
included in [`src_stage_mask`].The second [synchronization scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-scopes)
includes all commands that occur later in
[submission order](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-submission-order).
The second synchronization scope is limited to operations on the pipeline
stages determined by the [destination stage mask](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-pipeline-stages-masks) specified by [`dst_stage_mask`].The first [access scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-access-scopes) is
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
no accesses.
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
-    Any pipeline stage included in [`src_stage_mask`] **must**  be supported by the capabilities of the queue family specified by the `queueFamilyIndex` member of the [`CommandPoolCreateInfo`] structure that was used to create the [`CommandPool`] that [`command_buffer`] was allocated from, as specified in the [table of supported pipeline stages](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-pipeline-stages-supported)
-    Any pipeline stage included in [`dst_stage_mask`] **must**  be supported by the capabilities of the queue family specified by the `queueFamilyIndex` member of the [`CommandPoolCreateInfo`] structure that was used to create the [`CommandPool`] that [`command_buffer`] was allocated from, as specified in the [table of supported pipeline stages](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-pipeline-stages-supported)
-  [`src_stage_mask`] **must**  be the bitwise OR of the `stageMask` parameter used in previous calls to [`cmd_set_event`] with any of the elements of [`p_events`] and `VK_PIPELINE_STAGE_HOST_BIT` if any of the elements of [`p_events`] was set using [`set_event`]
-    If [`p_events`] includes one or more events that will be signaled by [`set_event`] after [`command_buffer`] has been submitted to a queue, then [`cmd_wait_events`] **must**  not be called inside a render pass instance
-    The `srcQueueFamilyIndex` and `dstQueueFamilyIndex` members of any element of [`p_buffer_memory_barriers`] or [`p_image_memory_barriers`] **must**  be equal
-  [`command_buffer`]’s current device mask  **must**  include exactly one physical device
-    Elements of [`p_events`] **must**  not have been signaled by [`cmd_set_event2`]

## Valid Usage (Implicit)
-  [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
-  [`p_events`] **must**  be a valid pointer to an array of [`event_count`] valid [`Event`] handles
-  [`src_stage_mask`] **must**  be a valid combination of [`PipelineStageFlagBits`] values
-  [`dst_stage_mask`] **must**  be a valid combination of [`PipelineStageFlagBits`] values
-    If [`memory_barrier_count`] is not `0`, [`p_memory_barriers`] **must**  be a valid pointer to an array of [`memory_barrier_count`] valid [`MemoryBarrier`] structures
-    If [`buffer_memory_barrier_count`] is not `0`, [`p_buffer_memory_barriers`] **must**  be a valid pointer to an array of [`buffer_memory_barrier_count`] valid [`BufferMemoryBarrier`] structures
-    If [`image_memory_barrier_count`] is not `0`, [`p_image_memory_barriers`] **must**  be a valid pointer to an array of [`image_memory_barrier_count`] valid [`ImageMemoryBarrier`] structures
-  [`command_buffer`] **must**  be in the [recording state]()
-    The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics, or compute operations
-  [`event_count`] **must**  be greater than `0`
-    Both of [`command_buffer`], and the elements of [`p_events`] **must**  have been created, allocated, or retrieved from the same [`Device`]

## Host Synchronization
- Host access to [`command_buffer`] **must**  be externally synchronized
- Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be externally synchronized

## Command Properties

# Related
- [`crate::vulkan1_0`]
- [`BufferMemoryBarrier`]
- [`CommandBuffer`]
- [`Event`]
- [`ImageMemoryBarrier`]
- [`MemoryBarrier`]
- [VkPipelineStageFlags]()

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        