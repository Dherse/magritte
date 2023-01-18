[vkCmdWriteTimestamp2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdWriteTimestamp2.html) - Write a device timestamp into a query object

# C Specifications
To request a timestamp, call:
```c
// Provided by VK_VERSION_1_3
void vkCmdWriteTimestamp2(
    VkCommandBuffer                             commandBuffer,
    VkPipelineStageFlags2                       stage,
    VkQueryPool                                 queryPool,
    uint32_t                                    query);
```
or the equivalent command
```c
// Provided by VK_KHR_synchronization2
void vkCmdWriteTimestamp2KHR(
    VkCommandBuffer                             commandBuffer,
    VkPipelineStageFlags2                       stage,
    VkQueryPool                                 queryPool,
    uint32_t                                    query);
```

# Parameters
- [`command_buffer`] is the command buffer into which the command will be recorded.
- [`stage`] specifies a stage of the pipeline.
- [`query_pool`] is the query pool that will manage the timestamp.
- [`query`] is the query within the query pool that will contain the timestamp.

# Description
When [`cmd_write_timestamp2`] is submitted to a queue, it defines an
execution dependency on commands that were submitted before it, and writes a
timestamp to a query pool.The first [synchronization scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-scopes)
includes all commands that occur earlier in
[submission order](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-submission-order).
The synchronization scope is limited to operations on the pipeline stage
specified by [`stage`].The second [synchronization scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-scopes)
includes only the timestamp write operation.When the timestamp value is written, the availability status of the query is
set to available.Comparisons between timestamps are not meaningful if the timestamps are
written by commands submitted to different queues.If [`cmd_write_timestamp2`] is called while executing a render pass
instance that has multiview enabled, the timestamp uses N consecutive
query indices in the query pool (starting at [`query`]) where N is
the number of bits set in the view mask of the subpass the command is
executed in.
The resulting query values are determined by an implementation-dependent
choice of one of the following behaviors:
- The first query is a timestamp value and (if more than one bit is set in the view mask) zero is written to the remaining queries. If two timestamps are written in the same subpass, the sum of the execution time of all views between those commands is the difference between the first query written by each command.
- All N queries are timestamp values. If two timestamps are written in the same subpass, the sum of the execution time of all views between those commands is the sum of the difference between corresponding queries written by each command. The difference between corresponding queries  **may**  be the execution time of a single view.
In either case, the application  **can**  sum the differences between all N
queries to determine the total execution time.
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
-  [`stage`] **must**  only include a single pipeline stage
-  [`stage`] **must**  only include stages valid for the queue family that was used to create the command pool that [`command_buffer`] was allocated from
-  [`query_pool`] **must**  have been created with a `queryType` of `VK_QUERY_TYPE_TIMESTAMP`
-    The query identified by [`query_pool`] and [`query`] **must**  be *unavailable*
-    The command pool’s queue family  **must**  support a non-zero `timestampValidBits`
-  [`query`] **must**  be less than the number of queries in [`query_pool`]
-    All queries used by the command  **must**  be unavailable
-    If [`cmd_write_timestamp2`] is called within a render pass instance, the sum of [`query`] and the number of bits set in the current subpass’s view mask  **must**  be less than or equal to the number of queries in [`query_pool`]

## Valid Usage (Implicit)
-  [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
-  [`stage`] **must**  be a valid combination of [`PipelineStageFlagBits2`] values
-  [`query_pool`] **must**  be a valid [`QueryPool`] handle
-  [`command_buffer`] **must**  be in the [recording state]()
-    The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support transfer, graphics, compute, decode, or encode operations
-    Both of [`command_buffer`], and [`query_pool`] **must**  have been created, allocated, or retrieved from the same [`Device`]

## Host Synchronization
- Host access to [`command_buffer`] **must**  be externally synchronized
- Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be externally synchronized

## Command Properties

# Related
- [`VK_KHR_synchronization2`]
- [`crate::vulkan1_3`]
- [`CommandBuffer`]
- [`PipelineStageFlags2`]
- [`QueryPool`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        