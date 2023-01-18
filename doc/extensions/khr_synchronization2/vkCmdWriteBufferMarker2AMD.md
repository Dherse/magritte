[vkCmdWriteBufferMarker2AMD](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdWriteBufferMarker2AMD.html) - Execute a pipelined write of a marker value into a buffer

# C Specifications
To write a 32-bit marker value into a buffer as a pipelined operation, call:
```c
// Provided by VK_KHR_synchronization2 with VK_AMD_buffer_marker
void vkCmdWriteBufferMarker2AMD(
    VkCommandBuffer                             commandBuffer,
    VkPipelineStageFlags2                       stage,
    VkBuffer                                    dstBuffer,
    VkDeviceSize                                dstOffset,
    uint32_t                                    marker);
```

# Parameters
- [`command_buffer`] is the command buffer into which the command will be recorded.
- [`stage`] specifies the pipeline stage whose completion triggers the marker write.
- [`dst_buffer`] is the buffer where the marker will be written.
- [`dst_offset`] is the byte offset into the buffer where the marker will be written.
- [`marker`] is the 32-bit value of the marker.

# Description
The command will write the 32-bit marker value into the buffer only after
all preceding commands have finished executing up to at least the specified
pipeline stage.
This includes the completion of other preceding
[`cmd_write_buffer_marker2_amd`] commands so long as their specified
pipeline stages occur either at the same time or earlier than this command’s
specified [`stage`].While consecutive buffer marker writes with the same [`stage`] parameter
implicitly complete in submission order, memory and execution dependencies
between buffer marker writes and other operations  **must**  still be explicitly
ordered using synchronization commands.
The access scope for buffer marker writes falls under the
`VK_ACCESS_TRANSFER_WRITE_BIT`, and the pipeline stages for identifying
the synchronization scope  **must**  include both [`stage`] and
`VK_PIPELINE_STAGE_TRANSFER_BIT`.
## Valid Usage
-    If the [geometry shaders](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-geometryShader) feature is not enabled, [`stage`] **must**  not contain `VK_PIPELINE_STAGE_2_GEOMETRY_SHADER_BIT`
-    If the [tessellation shaders](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-tessellationShader) feature is not enabled, [`stage`] **must**  not contain `VK_PIPELINE_STAGE_2_TESSELLATION_CONTROL_SHADER_BIT` or `VK_PIPELINE_STAGE_2_TESSELLATION_EVALUATION_SHADER_BIT`
-    If the [conditional rendering](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-conditionalRendering) feature is not enabled, [`stage`] **must**  not contain `VK_PIPELINE_STAGE_2_CONDITIONAL_RENDERING_BIT_EXT`
-    If the [fragment density map](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-fragmentDensityMap) feature is not enabled, [`stage`] **must**  not contain `VK_PIPELINE_STAGE_2_FRAGMENT_DENSITY_PROCESS_BIT_EXT`
-    If the [transform feedback](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-transformFeedback) feature is not enabled, [`stage`] **must**  not contain `VK_PIPELINE_STAGE_2_TRANSFORM_FEEDBACK_BIT_EXT`
-    If the [mesh shaders](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-meshShader) feature is not enabled, [`stage`] **must**  not contain `VK_PIPELINE_STAGE_2_MESH_SHADER_BIT_NV`
-    If the [task shaders](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-taskShader) feature is not enabled, [`stage`] **must**  not contain `VK_PIPELINE_STAGE_2_TASK_SHADER_BIT_NV`
-    If the [shading rate image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-shadingRateImage) feature is not enabled, [`stage`] **must**  not contain `VK_PIPELINE_STAGE_2_SHADING_RATE_IMAGE_BIT_NV`
-    If the [subpass shading](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-subpassShading) feature is not enabled, [`stage`] **must**  not contain `VK_PIPELINE_STAGE_2_SUBPASS_SHADING_BIT_HUAWEI`
-    If the [invocation mask image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-invocationMask) feature is not enabled, [`stage`] **must**  not contain `VK_PIPELINE_STAGE_2_INVOCATION_MASK_BIT_HUAWEI`
-    The [`synchronization2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-synchronization2) feature  **must**  be enabled
-  [`stage`] **must**  include only a single pipeline stage
-  [`stage`] **must**  include only stages that are valid for the queue family that was used to create the command pool that [`command_buffer`] was allocated from
-  [`dst_offset`] **must**  be less than or equal to the size of [`dst_buffer`] minus `4`
-  [`dst_buffer`] **must**  have been created with the `VK_BUFFER_USAGE_TRANSFER_DST_BIT` usage flag
-    If [`dst_buffer`] is non-sparse then it  **must**  be bound completely and contiguously to a single [`DeviceMemory`] object
-  [`dst_offset`] **must**  be a multiple of `4`

## Valid Usage (Implicit)
-  [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
-  [`stage`] **must**  be a valid combination of [`PipelineStageFlagBits2`] values
-  [`dst_buffer`] **must**  be a valid [`Buffer`] handle
-  [`command_buffer`] **must**  be in the [recording state]()
-    The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support transfer, graphics, or compute operations
-    Both of [`command_buffer`], and [`dst_buffer`] **must**  have been created, allocated, or retrieved from the same [`Device`]

## Host Synchronization
- Host access to [`command_buffer`] **must**  be externally synchronized
- Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be externally synchronized

## Command Properties

# Related
- [`VK_AMD_buffer_marker`]
- [`VK_KHR_synchronization2`]
- [`Buffer`]
- [`CommandBuffer`]
- [`DeviceSize`]
- [`PipelineStageFlags2`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        