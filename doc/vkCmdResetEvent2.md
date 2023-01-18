[vkCmdResetEvent2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdResetEvent2.html) - Reset an event object to non-signaled state

# C Specifications
To unsignal the event from a device, call:
```c
// Provided by VK_VERSION_1_3
void vkCmdResetEvent2(
    VkCommandBuffer                             commandBuffer,
    VkEvent                                     event,
    VkPipelineStageFlags2                       stageMask);
```
or the equivalent command
```c
// Provided by VK_KHR_synchronization2
void vkCmdResetEvent2KHR(
    VkCommandBuffer                             commandBuffer,
    VkEvent                                     event,
    VkPipelineStageFlags2                       stageMask);
```

# Parameters
- [`command_buffer`] is the command buffer into which the command is recorded.
- [`event`] is the event that will be unsignaled.
- [`stage_mask`] is a [`PipelineStageFlags2`] mask of pipeline stages used to determine the first [synchronization scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-scopes).

# Description
When [`cmd_reset_event2`] is submitted to a queue, it defines an execution
dependency on commands that were submitted before it, and defines an event
unsignal operation which resets the event to the unsignaled state.The first [synchronization scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-scopes)
includes all commands that occur earlier in
[submission order](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-submission-order).
The synchronization scope is limited to operations by [`stage_mask`] or
stages that are [logically earlier](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-pipeline-stages-order)
than [`stage_mask`].The second [synchronization scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-scopes)
includes only the event unsignal operation.If [`event`] is already in the unsignaled state when
[`cmd_reset_event2`] is executed on the device, then this command has no
effect, no event unsignal operation occurs, and no execution dependency is
generated.
## Valid Usage
-    If the [geometry shaders](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-geometryShader) feature is not enabled, [`stage_mask`] **must**  not contain `VK_PIPELINE_STAGE_2_GEOMETRY_SHADER_BIT`
-    If the [tessellation shaders](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-tessellationShader) feature is not enabled, [`stage_mask`] **must**  not contain `VK_PIPELINE_STAGE_2_TESSELLATION_CONTROL_SHADER_BIT` or `VK_PIPELINE_STAGE_2_TESSELLATION_EVALUATION_SHADER_BIT`
-    If the [conditional rendering](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-conditionalRendering) feature is not enabled, [`stage_mask`] **must**  not contain `VK_PIPELINE_STAGE_2_CONDITIONAL_RENDERING_BIT_EXT`
-    If the [fragment density map](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-fragmentDensityMap) feature is not enabled, [`stage_mask`] **must**  not contain `VK_PIPELINE_STAGE_2_FRAGMENT_DENSITY_PROCESS_BIT_EXT`
-    If the [transform feedback](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-transformFeedback) feature is not enabled, [`stage_mask`] **must**  not contain `VK_PIPELINE_STAGE_2_TRANSFORM_FEEDBACK_BIT_EXT`
-    If the [mesh shaders](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-meshShader) feature is not enabled, [`stage_mask`] **must**  not contain `VK_PIPELINE_STAGE_2_MESH_SHADER_BIT_NV`
-    If the [task shaders](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-taskShader) feature is not enabled, [`stage_mask`] **must**  not contain `VK_PIPELINE_STAGE_2_TASK_SHADER_BIT_NV`
-    If the [shading rate image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-shadingRateImage) feature is not enabled, [`stage_mask`] **must**  not contain `VK_PIPELINE_STAGE_2_SHADING_RATE_IMAGE_BIT_NV`
-    If the [subpass shading](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-subpassShading) feature is not enabled, [`stage_mask`] **must**  not contain `VK_PIPELINE_STAGE_2_SUBPASS_SHADING_BIT_HUAWEI`
-    If the [invocation mask image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-invocationMask) feature is not enabled, [`stage_mask`] **must**  not contain `VK_PIPELINE_STAGE_2_INVOCATION_MASK_BIT_HUAWEI`
-    The [`synchronization2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-synchronization2) feature  **must**  be enabled
-  [`stage_mask`] **must**  not include `VK_PIPELINE_STAGE_2_HOST_BIT`
-    There  **must**  be an execution dependency between [`cmd_reset_event2`] and the execution of any [`cmd_wait_events`] that includes [`event`] in its `pEvents` parameter
-    There  **must**  be an execution dependency between [`cmd_reset_event2`] and the execution of any [`cmd_wait_events2`] that includes [`event`] in its `pEvents` parameter
-  [`command_buffer`]’s current device mask  **must**  include exactly one physical device

## Valid Usage (Implicit)
-  [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
-  [`event`] **must**  be a valid [`Event`] handle
-  [`stage_mask`] **must**  be a valid combination of [`PipelineStageFlagBits2`] values
-  [`command_buffer`] **must**  be in the [recording state]()
-    The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics, or compute operations
-    This command  **must**  only be called outside of a render pass instance
-    Both of [`command_buffer`], and [`event`] **must**  have been created, allocated, or retrieved from the same [`Device`]

## Host Synchronization
- Host access to [`command_buffer`] **must**  be externally synchronized
- Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be externally synchronized

## Command Properties

# Related
- [`VK_KHR_synchronization2`]
- [`crate::vulkan1_3`]
- [`CommandBuffer`]
- [`Event`]
- [`PipelineStageFlags2`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        