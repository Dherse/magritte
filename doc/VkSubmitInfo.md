[VkSubmitInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSubmitInfo.html) - Structure specifying a queue submit operation

# C Specifications
The [`SubmitInfo`] structure is defined as:
```c
// Provided by VK_VERSION_1_0
typedef struct VkSubmitInfo {
    VkStructureType                sType;
    const void*                    pNext;
    uint32_t                       waitSemaphoreCount;
    const VkSemaphore*             pWaitSemaphores;
    const VkPipelineStageFlags*    pWaitDstStageMask;
    uint32_t                       commandBufferCount;
    const VkCommandBuffer*         pCommandBuffers;
    uint32_t                       signalSemaphoreCount;
    const VkSemaphore*             pSignalSemaphores;
} VkSubmitInfo;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`wait_semaphore_count`] is the number of semaphores upon which to wait before executing the command buffers for the batch.
- [`wait_semaphores`] is a pointer to an array of [`Semaphore`] handles upon which to wait before the command buffers for this batch begin execution. If semaphores to wait on are provided, they define a [semaphore wait operation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-semaphores-waiting).
- [`wait_dst_stage_mask`] is a pointer to an array of pipeline stages at which each corresponding semaphore wait will occur.
- [`command_buffer_count`] is the number of command buffers to execute in the batch.
- [`command_buffers`] is a pointer to an array of [`CommandBuffer`] handles to execute in the batch.
- [`signal_semaphore_count`] is the number of semaphores to be signaled once the commands specified in [`command_buffers`] have completed execution.
- [`signal_semaphores`] is a pointer to an array of [`Semaphore`] handles which will be signaled when the command buffers for this batch have completed execution. If semaphores to be signaled are provided, they define a [semaphore signal operation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-semaphores-signaling).

# Description
The order that command buffers appear in [`command_buffers`] is used to
determine [submission order](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-submission-order), and thus
all the [implicit ordering guarantees](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-implicit) that
respect it.
Other than these implicit ordering guarantees and any [explicit synchronization primitives](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization), these command buffers  **may**  overlap or
otherwise execute out of order.
## Valid Usage
-    If the [geometry shaders](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-geometryShader) feature is not enabled, [`wait_dst_stage_mask`] **must**  not contain `VK_PIPELINE_STAGE_GEOMETRY_SHADER_BIT`
-    If the [tessellation shaders](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-tessellationShader) feature is not enabled, [`wait_dst_stage_mask`] **must**  not contain `VK_PIPELINE_STAGE_TESSELLATION_CONTROL_SHADER_BIT` or `VK_PIPELINE_STAGE_TESSELLATION_EVALUATION_SHADER_BIT`
-    If the [conditional rendering](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-conditionalRendering) feature is not enabled, [`wait_dst_stage_mask`] **must**  not contain `VK_PIPELINE_STAGE_CONDITIONAL_RENDERING_BIT_EXT`
-    If the [fragment density map](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-fragmentDensityMap) feature is not enabled, [`wait_dst_stage_mask`] **must**  not contain `VK_PIPELINE_STAGE_FRAGMENT_DENSITY_PROCESS_BIT_EXT`
-    If the [transform feedback](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-transformFeedback) feature is not enabled, [`wait_dst_stage_mask`] **must**  not contain `VK_PIPELINE_STAGE_TRANSFORM_FEEDBACK_BIT_EXT`
-    If the [mesh shaders](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-meshShader) feature is not enabled, [`wait_dst_stage_mask`] **must**  not contain `VK_PIPELINE_STAGE_MESH_SHADER_BIT_NV`
-    If the [task shaders](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-taskShader) feature is not enabled, [`wait_dst_stage_mask`] **must**  not contain `VK_PIPELINE_STAGE_TASK_SHADER_BIT_NV`
-    If the [shading rate image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-shadingRateImage) feature is not enabled, [`wait_dst_stage_mask`] **must**  not contain `VK_PIPELINE_STAGE_SHADING_RATE_IMAGE_BIT_NV`
-    If the [`synchronization2`](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-synchronization2) feature is not enabled, [`wait_dst_stage_mask`] **must**  not be `0`
-    Each element of [`command_buffers`] **must**  not have been allocated with `VK_COMMAND_BUFFER_LEVEL_SECONDARY`
-    Each element of [`wait_dst_stage_mask`] **must**  not include `VK_PIPELINE_STAGE_HOST_BIT`
-    If any element of [`wait_semaphores`] or [`signal_semaphores`] was created with a [`SemaphoreType`] of `VK_SEMAPHORE_TYPE_TIMELINE`, then the [`p_next`] chain  **must**  include a [`TimelineSemaphoreSubmitInfo`] structure
-    If the [`p_next`] chain of this structure includes a [`TimelineSemaphoreSubmitInfo`] structure and any element of [`wait_semaphores`] was created with a [`SemaphoreType`] of `VK_SEMAPHORE_TYPE_TIMELINE`, then its `waitSemaphoreValueCount` member  **must**  equal [`wait_semaphore_count`]
-    If the [`p_next`] chain of this structure includes a [`TimelineSemaphoreSubmitInfo`] structure and any element of [`signal_semaphores`] was created with a [`SemaphoreType`] of `VK_SEMAPHORE_TYPE_TIMELINE`, then its `signalSemaphoreValueCount` member  **must**  equal [`signal_semaphore_count`]
-    For each element of [`signal_semaphores`] created with a [`SemaphoreType`] of `VK_SEMAPHORE_TYPE_TIMELINE` the corresponding element of [`TimelineSemaphoreSubmitInfo::signal_semaphore_values`] **must**  have a value greater than the current value of the semaphore when the [semaphore signal operation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-semaphores-signaling) is executed
-    For each element of [`wait_semaphores`] created with a [`SemaphoreType`] of `VK_SEMAPHORE_TYPE_TIMELINE` the corresponding element of [`TimelineSemaphoreSubmitInfo::wait_semaphore_values`] **must**  have a value which does not differ from the current value of the semaphore or the value of any outstanding semaphore wait or signal operation on that semaphore by more than [`maxTimelineSemaphoreValueDifference`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-maxTimelineSemaphoreValueDifference)
-    For each element of [`signal_semaphores`] created with a [`SemaphoreType`] of `VK_SEMAPHORE_TYPE_TIMELINE` the corresponding element of [`TimelineSemaphoreSubmitInfo::signal_semaphore_values`] **must**  have a value which does not differ from the current value of the semaphore or the value of any outstanding semaphore wait or signal operation on that semaphore by more than [`maxTimelineSemaphoreValueDifference`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-maxTimelineSemaphoreValueDifference)
-    If the [`p_next`] chain of this structure does not include a [`ProtectedSubmitInfo`] structure with `protectedSubmit` set to [`TRUE`], then each element of the [`command_buffers`] array  **must**  be an unprotected command buffer
-    If the [`p_next`] chain of this structure includes a [`ProtectedSubmitInfo`] structure with `protectedSubmit` set to [`TRUE`], then each element of the [`command_buffers`] array  **must**  be a protected command buffer
-    If [`command_buffers`] contains any [resumed render pass instances](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#renderpass-suspension), they  **must**  be suspended by a render pass instance earlier in submission order within [`command_buffers`]
-    If [`command_buffers`] contains any [suspended render pass instances](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#renderpass-suspension), they  **must**  be resumed by a render pass instance later in submission order within [`command_buffers`]
-    If [`command_buffers`] contains any [suspended render pass instances](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#renderpass-suspension), there  **must**  be no action or synchronization commands between that render pass instance and the render pass instance that resumes it
-    If [`command_buffers`] contains any [suspended render pass instances](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#renderpass-suspension), there  **must**  be no render pass instances between that render pass instance and the render pass instance that resumes it
-    If the [`variableSampleLocations`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-variableSampleLocations) limit is not supported, and any element of [`command_buffers`] contains any [suspended render pass instances](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#renderpass-suspension), where a graphics pipeline has been bound, any pipelines bound in the render pass instance that resumes it, or any subsequent render pass instances that resume from that one and so on,  **must**  use the same sample locations

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_SUBMIT_INFO`
-    Each [`p_next`] member of any structure (including this one) in the [`p_next`] chain  **must**  be either `NULL` or a pointer to a valid instance of [`D3d12FenceSubmitInfoKHR`], [`DeviceGroupSubmitInfo`], [`PerformanceQuerySubmitInfoKHR`], [`ProtectedSubmitInfo`], [`TimelineSemaphoreSubmitInfo`], [`Win32KeyedMutexAcquireReleaseInfoKHR`], or [`Win32KeyedMutexAcquireReleaseInfoNV`]
-    The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique
-    If [`wait_semaphore_count`] is not `0`, [`wait_semaphores`] **must**  be a valid pointer to an array of [`wait_semaphore_count`] valid [`Semaphore`] handles
-    If [`wait_semaphore_count`] is not `0`, [`wait_dst_stage_mask`] **must**  be a valid pointer to an array of [`wait_semaphore_count`] valid combinations of [`PipelineStageFlagBits`] values
-    Each element of [`wait_dst_stage_mask`] **must**  not be `0`
-    If [`command_buffer_count`] is not `0`, [`command_buffers`] **must**  be a valid pointer to an array of [`command_buffer_count`] valid [`CommandBuffer`] handles
-    If [`signal_semaphore_count`] is not `0`, [`signal_semaphores`] **must**  be a valid pointer to an array of [`signal_semaphore_count`] valid [`Semaphore`] handles
-    Each of the elements of [`command_buffers`], the elements of [`signal_semaphores`], and the elements of [`wait_semaphores`] that are valid handles of non-ignored parameters  **must**  have been created, allocated, or retrieved from the same [`Device`]

# Related
- [`crate::vulkan1_0`]
- [`CommandBuffer`]
- [`PipelineStageFlags`]
- [`Semaphore`]
- [`StructureType`]
- [`queue_submit`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        