[VkMemoryBarrier2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryBarrier2.html) - Structure specifying a global memory barrier

# C Specifications
The [`MemoryBarrier2`] structure is defined as:
```c
// Provided by VK_VERSION_1_3
typedef struct VkMemoryBarrier2 {
    VkStructureType          sType;
    const void*              pNext;
    VkPipelineStageFlags2    srcStageMask;
    VkAccessFlags2           srcAccessMask;
    VkPipelineStageFlags2    dstStageMask;
    VkAccessFlags2           dstAccessMask;
} VkMemoryBarrier2;
```
or the equivalent
```c
// Provided by VK_KHR_synchronization2
typedef VkMemoryBarrier2 VkMemoryBarrier2KHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`src_stage_mask`] is a [`PipelineStageFlags2`] mask of pipeline stages to be included in the [first synchronization scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-scopes).
- [`src_access_mask`] is a [`AccessFlags2`] mask of access flags to be included in the [first access scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-access-scopes).
- [`dst_stage_mask`] is a [`PipelineStageFlags2`] mask of pipeline stages to be included in the [second synchronization scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-scopes).
- [`dst_access_mask`] is a [`AccessFlags2`] mask of access flags to be included in the [second access scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-access-scopes).

# Description
This structure defines a [memory
dependency](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-memory) affecting all device memory.The first [synchronization scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-scopes) and
[access scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-access-scopes) described by
this structure include only operations and memory accesses specified by
[`src_stage_mask`] and [`src_access_mask`].The second [synchronization scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-scopes)
and [access scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-access-scopes) described
by this structure include only operations and memory accesses specified by
[`dst_stage_mask`] and [`dst_access_mask`].
## Valid Usage
-    If the [geometry shaders](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-geometryShader) feature is not enabled, [`src_stage_mask`] **must**  not contain `VK_PIPELINE_STAGE_2_GEOMETRY_SHADER_BIT`
-    If the [tessellation shaders](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-tessellationShader) feature is not enabled, [`src_stage_mask`] **must**  not contain `VK_PIPELINE_STAGE_2_TESSELLATION_CONTROL_SHADER_BIT` or `VK_PIPELINE_STAGE_2_TESSELLATION_EVALUATION_SHADER_BIT`
-    If the [conditional rendering](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-conditionalRendering) feature is not enabled, [`src_stage_mask`] **must**  not contain `VK_PIPELINE_STAGE_2_CONDITIONAL_RENDERING_BIT_EXT`
-    If the [fragment density map](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-fragmentDensityMap) feature is not enabled, [`src_stage_mask`] **must**  not contain `VK_PIPELINE_STAGE_2_FRAGMENT_DENSITY_PROCESS_BIT_EXT`
-    If the [transform feedback](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-transformFeedback) feature is not enabled, [`src_stage_mask`] **must**  not contain `VK_PIPELINE_STAGE_2_TRANSFORM_FEEDBACK_BIT_EXT`
-    If the [mesh shaders](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-meshShader) feature is not enabled, [`src_stage_mask`] **must**  not contain `VK_PIPELINE_STAGE_2_MESH_SHADER_BIT_NV`
-    If the [task shaders](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-taskShader) feature is not enabled, [`src_stage_mask`] **must**  not contain `VK_PIPELINE_STAGE_2_TASK_SHADER_BIT_NV`
-    If the [shading rate image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-shadingRateImage) feature is not enabled, [`src_stage_mask`] **must**  not contain `VK_PIPELINE_STAGE_2_SHADING_RATE_IMAGE_BIT_NV`
-    If the [subpass shading](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-subpassShading) feature is not enabled, [`src_stage_mask`] **must**  not contain `VK_PIPELINE_STAGE_2_SUBPASS_SHADING_BIT_HUAWEI`
-    If the [invocation mask image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-invocationMask) feature is not enabled, [`src_stage_mask`] **must**  not contain `VK_PIPELINE_STAGE_2_INVOCATION_MASK_BIT_HUAWEI`
-    If [`src_access_mask`] includes `VK_ACCESS_2_INDIRECT_COMMAND_READ_BIT`, [`src_stage_mask`] **must**  include `VK_PIPELINE_STAGE_2_DRAW_INDIRECT_BIT`, `VK_PIPELINE_STAGE_2_ACCELERATION_STRUCTURE_BUILD_BIT_KHR`, `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
-    If [`src_access_mask`] includes `VK_ACCESS_2_INDEX_READ_BIT`, [`src_stage_mask`] **must**  include `VK_PIPELINE_STAGE_2_INDEX_INPUT_BIT`, `VK_PIPELINE_STAGE_2_VERTEX_INPUT_BIT`, `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
-    If [`src_access_mask`] includes `VK_ACCESS_2_VERTEX_ATTRIBUTE_READ_BIT`, [`src_stage_mask`] **must**  include `VK_PIPELINE_STAGE_2_VERTEX_ATTRIBUTE_INPUT_BIT`, `VK_PIPELINE_STAGE_2_VERTEX_INPUT_BIT`, `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
-    If [`src_access_mask`] includes `VK_ACCESS_2_INPUT_ATTACHMENT_READ_BIT`, [`src_stage_mask`] **must**  include `VK_PIPELINE_STAGE_2_FRAGMENT_SHADER_BIT`, `VK_PIPELINE_STAGE_2_SUBPASS_SHADING_BIT_HUAWEI`, `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
-    If [`src_access_mask`] includes `VK_ACCESS_2_UNIFORM_READ_BIT`, [`src_stage_mask`] **must**  include `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`, or one of the `VK_PIPELINE_STAGE_*_SHADER_BIT` stages
-    If [`src_access_mask`] includes `VK_ACCESS_2_SHADER_SAMPLED_READ_BIT`, [`src_stage_mask`] **must**  include `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`, or one of the `VK_PIPELINE_STAGE_*_SHADER_BIT` stages
-    If [`src_access_mask`] includes `VK_ACCESS_2_SHADER_STORAGE_READ_BIT`, [`src_stage_mask`] **must**  include `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`, or one of the `VK_PIPELINE_STAGE_*_SHADER_BIT` stages
-    If [`src_access_mask`] includes `VK_ACCESS_2_SHADER_STORAGE_WRITE_BIT`, [`src_stage_mask`] **must**  include `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`, or one of the `VK_PIPELINE_STAGE_*_SHADER_BIT` stages
-    If [`src_access_mask`] includes `VK_ACCESS_2_SHADER_READ_BIT`, [`src_stage_mask`] **must**  include `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`, `VK_PIPELINE_STAGE_2_ACCELERATION_STRUCTURE_BUILD_BIT_KHR`, or one of the `VK_PIPELINE_STAGE_*_SHADER_BIT` stages
-    If [`src_access_mask`] includes `VK_ACCESS_2_SHADER_WRITE_BIT`, [`src_stage_mask`] **must**  include `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`, or one of the `VK_PIPELINE_STAGE_*_SHADER_BIT` stages
-    If [`src_access_mask`] includes `VK_ACCESS_2_COLOR_ATTACHMENT_READ_BIT`, [`src_stage_mask`] **must**  include `VK_PIPELINE_STAGE_2_COLOR_ATTACHMENT_OUTPUT_BIT``VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
-    If [`src_access_mask`] includes `VK_ACCESS_2_COLOR_ATTACHMENT_WRITE_BIT`, [`src_stage_mask`] **must**  include `VK_PIPELINE_STAGE_2_COLOR_ATTACHMENT_OUTPUT_BIT``VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
-    If [`src_access_mask`] includes `VK_ACCESS_2_DEPTH_STENCIL_ATTACHMENT_READ_BIT`, [`src_stage_mask`] **must**  include `VK_PIPELINE_STAGE_2_EARLY_FRAGMENT_TESTS_BIT`, `VK_PIPELINE_STAGE_2_LATE_FRAGMENT_TESTS_BIT`, `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
-    If [`src_access_mask`] includes `VK_ACCESS_2_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT`, [`src_stage_mask`] **must**  include `VK_PIPELINE_STAGE_2_EARLY_FRAGMENT_TESTS_BIT`, `VK_PIPELINE_STAGE_2_LATE_FRAGMENT_TESTS_BIT`, `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
-    If [`src_access_mask`] includes `VK_ACCESS_2_TRANSFER_READ_BIT`, [`src_stage_mask`] **must**  include `VK_PIPELINE_STAGE_2_COPY_BIT`, `VK_PIPELINE_STAGE_2_BLIT_BIT`, `VK_PIPELINE_STAGE_2_RESOLVE_BIT`, `VK_PIPELINE_STAGE_2_ALL_TRANSFER_BIT`, `VK_PIPELINE_STAGE_2_ACCELERATION_STRUCTURE_BUILD_BIT_KHR`, or `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
-    If [`src_access_mask`] includes `VK_ACCESS_2_TRANSFER_WRITE_BIT`, [`src_stage_mask`] **must**  include `VK_PIPELINE_STAGE_2_COPY_BIT`, `VK_PIPELINE_STAGE_2_BLIT_BIT`, `VK_PIPELINE_STAGE_2_RESOLVE_BIT`, `VK_PIPELINE_STAGE_2_CLEAR_BIT`, `VK_PIPELINE_STAGE_2_ALL_TRANSFER_BIT`, `VK_PIPELINE_STAGE_2_ACCELERATION_STRUCTURE_BUILD_BIT_KHR`, or `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
-    If [`src_access_mask`] includes `VK_ACCESS_2_HOST_READ_BIT`, [`src_stage_mask`] **must**  include `VK_PIPELINE_STAGE_2_HOST_BIT`
-    If [`src_access_mask`] includes `VK_ACCESS_2_HOST_WRITE_BIT`, [`src_stage_mask`] **must**  include `VK_PIPELINE_STAGE_2_HOST_BIT`
-    If [`src_access_mask`] includes `VK_ACCESS_2_CONDITIONAL_RENDERING_READ_BIT_EXT`, [`src_stage_mask`] **must**  include `VK_PIPELINE_STAGE_2_CONDITIONAL_RENDERING_BIT_EXT`, `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
-    If [`src_access_mask`] includes `VK_ACCESS_2_FRAGMENT_DENSITY_MAP_READ_BIT_EXT`, [`src_stage_mask`] **must**  include `VK_PIPELINE_STAGE_2_FRAGMENT_DENSITY_PROCESS_BIT_EXT`, `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
-    If [`src_access_mask`] includes `VK_ACCESS_2_TRANSFORM_FEEDBACK_WRITE_BIT_EXT`, [`src_stage_mask`] **must**  include `VK_PIPELINE_STAGE_2_TRANSFORM_FEEDBACK_BIT_EXT`, `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
-    If [`src_access_mask`] includes `VK_ACCESS_2_TRANSFORM_FEEDBACK_COUNTER_READ_BIT_EXT`, [`src_stage_mask`] **must**  include `VK_PIPELINE_STAGE_2_DRAW_INDIRECT_BIT`, `VK_PIPELINE_STAGE_2_TRANSFORM_FEEDBACK_BIT_EXT`, `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
-    If [`src_access_mask`] includes `VK_ACCESS_2_TRANSFORM_FEEDBACK_COUNTER_WRITE_BIT_EXT`, [`src_stage_mask`] **must**  include `VK_PIPELINE_STAGE_2_TRANSFORM_FEEDBACK_BIT_EXT`, `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
-    If [`src_access_mask`] includes `VK_ACCESS_2_SHADING_RATE_IMAGE_READ_BIT_NV`, [`src_stage_mask`] **must**  include `VK_PIPELINE_STAGE_2_SHADING_RATE_IMAGE_BIT_NV`, `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
-    If [`src_access_mask`] includes `VK_ACCESS_2_INVOCATION_MASK_READ_BIT_HUAWEI`, [`src_stage_mask`] **must**  include `VK_PIPELINE_STAGE_2_INVOCATION_MASK_BIT_HUAWEI`
-    If [`src_access_mask`] includes `VK_ACCESS_2_COMMAND_PREPROCESS_READ_BIT_NV`, [`src_stage_mask`] **must**  include `VK_PIPELINE_STAGE_2_COMMAND_PREPROCESS_BIT_NV` or `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
-    If [`src_access_mask`] includes `VK_ACCESS_2_COMMAND_PREPROCESS_WRITE_BIT_NV`, [`src_stage_mask`] **must**  include `VK_PIPELINE_STAGE_2_COMMAND_PREPROCESS_BIT_NV` or `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
-    If [`src_access_mask`] includes `VK_ACCESS_2_COLOR_ATTACHMENT_READ_NONCOHERENT_BIT_EXT`, [`src_stage_mask`] **must**  include `VK_PIPELINE_STAGE_2_COLOR_ATTACHMENT_OUTPUT_BIT``VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
-    If [`src_access_mask`] includes `VK_ACCESS_2_ACCELERATION_STRUCTURE_READ_BIT_KHR`, [`src_stage_mask`] **must**  include `VK_PIPELINE_STAGE_2_ACCELERATION_STRUCTURE_BUILD_BIT_KHR`, `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`, or one of the `VK_PIPELINE_STAGE_*_SHADER_BIT` stages
-    If [`src_access_mask`] includes `VK_ACCESS_2_ACCELERATION_STRUCTURE_WRITE_BIT_KHR`, [`src_stage_mask`] **must**  include `VK_PIPELINE_STAGE_2_ACCELERATION_STRUCTURE_BUILD_BIT_KHR` or `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
-    If [`rayQuery`](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-rayQuery) is not enabled and [`src_access_mask`] includes `VK_ACCESS_2_ACCELERATION_STRUCTURE_READ_BIT_KHR`, [`src_stage_mask`] **must**  not include any of the `VK_PIPELINE_STAGE_*_SHADER_BIT` stages except `VK_PIPELINE_STAGE_2_RAY_TRACING_SHADER_BIT_KHR`
-    If [`src_access_mask`] includes `VK_ACCESS_2_VIDEO_DECODE_READ_BIT_KHR`, [`src_stage_mask`] **must**  include `VK_PIPELINE_STAGE_2_VIDEO_DECODE_BIT_KHR`
-    If [`src_access_mask`] includes `VK_ACCESS_2_VIDEO_DECODE_WRITE_BIT_KHR`, [`src_stage_mask`] **must**  include `VK_PIPELINE_STAGE_2_VIDEO_DECODE_BIT_KHR`
-    If [`src_access_mask`] includes `VK_ACCESS_2_VIDEO_ENCODE_READ_BIT_KHR`, [`src_stage_mask`] **must**  include `VK_PIPELINE_STAGE_2_VIDEO_ENCODE_BIT_KHR`
-    If [`src_access_mask`] includes `VK_ACCESS_2_VIDEO_ENCODE_WRITE_BIT_KHR`, [`src_stage_mask`] **must**  include `VK_PIPELINE_STAGE_2_VIDEO_ENCODE_BIT_KHR`

-    If the [geometry shaders](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-geometryShader) feature is not enabled, [`dst_stage_mask`] **must**  not contain `VK_PIPELINE_STAGE_2_GEOMETRY_SHADER_BIT`
-    If the [tessellation shaders](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-tessellationShader) feature is not enabled, [`dst_stage_mask`] **must**  not contain `VK_PIPELINE_STAGE_2_TESSELLATION_CONTROL_SHADER_BIT` or `VK_PIPELINE_STAGE_2_TESSELLATION_EVALUATION_SHADER_BIT`
-    If the [conditional rendering](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-conditionalRendering) feature is not enabled, [`dst_stage_mask`] **must**  not contain `VK_PIPELINE_STAGE_2_CONDITIONAL_RENDERING_BIT_EXT`
-    If the [fragment density map](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-fragmentDensityMap) feature is not enabled, [`dst_stage_mask`] **must**  not contain `VK_PIPELINE_STAGE_2_FRAGMENT_DENSITY_PROCESS_BIT_EXT`
-    If the [transform feedback](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-transformFeedback) feature is not enabled, [`dst_stage_mask`] **must**  not contain `VK_PIPELINE_STAGE_2_TRANSFORM_FEEDBACK_BIT_EXT`
-    If the [mesh shaders](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-meshShader) feature is not enabled, [`dst_stage_mask`] **must**  not contain `VK_PIPELINE_STAGE_2_MESH_SHADER_BIT_NV`
-    If the [task shaders](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-taskShader) feature is not enabled, [`dst_stage_mask`] **must**  not contain `VK_PIPELINE_STAGE_2_TASK_SHADER_BIT_NV`
-    If the [shading rate image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-shadingRateImage) feature is not enabled, [`dst_stage_mask`] **must**  not contain `VK_PIPELINE_STAGE_2_SHADING_RATE_IMAGE_BIT_NV`
-    If the [subpass shading](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-subpassShading) feature is not enabled, [`dst_stage_mask`] **must**  not contain `VK_PIPELINE_STAGE_2_SUBPASS_SHADING_BIT_HUAWEI`
-    If the [invocation mask image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-invocationMask) feature is not enabled, [`dst_stage_mask`] **must**  not contain `VK_PIPELINE_STAGE_2_INVOCATION_MASK_BIT_HUAWEI`
-    If [`dst_access_mask`] includes `VK_ACCESS_2_INDIRECT_COMMAND_READ_BIT`, [`dst_stage_mask`] **must**  include `VK_PIPELINE_STAGE_2_DRAW_INDIRECT_BIT`, `VK_PIPELINE_STAGE_2_ACCELERATION_STRUCTURE_BUILD_BIT_KHR`, `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
-    If [`dst_access_mask`] includes `VK_ACCESS_2_INDEX_READ_BIT`, [`dst_stage_mask`] **must**  include `VK_PIPELINE_STAGE_2_INDEX_INPUT_BIT`, `VK_PIPELINE_STAGE_2_VERTEX_INPUT_BIT`, `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
-    If [`dst_access_mask`] includes `VK_ACCESS_2_VERTEX_ATTRIBUTE_READ_BIT`, [`dst_stage_mask`] **must**  include `VK_PIPELINE_STAGE_2_VERTEX_ATTRIBUTE_INPUT_BIT`, `VK_PIPELINE_STAGE_2_VERTEX_INPUT_BIT`, `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
-    If [`dst_access_mask`] includes `VK_ACCESS_2_INPUT_ATTACHMENT_READ_BIT`, [`dst_stage_mask`] **must**  include `VK_PIPELINE_STAGE_2_FRAGMENT_SHADER_BIT`, `VK_PIPELINE_STAGE_2_SUBPASS_SHADING_BIT_HUAWEI`, `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
-    If [`dst_access_mask`] includes `VK_ACCESS_2_UNIFORM_READ_BIT`, [`dst_stage_mask`] **must**  include `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`, or one of the `VK_PIPELINE_STAGE_*_SHADER_BIT` stages
-    If [`dst_access_mask`] includes `VK_ACCESS_2_SHADER_SAMPLED_READ_BIT`, [`dst_stage_mask`] **must**  include `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`, or one of the `VK_PIPELINE_STAGE_*_SHADER_BIT` stages
-    If [`dst_access_mask`] includes `VK_ACCESS_2_SHADER_STORAGE_READ_BIT`, [`dst_stage_mask`] **must**  include `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`, or one of the `VK_PIPELINE_STAGE_*_SHADER_BIT` stages
-    If [`dst_access_mask`] includes `VK_ACCESS_2_SHADER_STORAGE_WRITE_BIT`, [`dst_stage_mask`] **must**  include `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`, or one of the `VK_PIPELINE_STAGE_*_SHADER_BIT` stages
-    If [`dst_access_mask`] includes `VK_ACCESS_2_SHADER_READ_BIT`, [`dst_stage_mask`] **must**  include `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`, `VK_PIPELINE_STAGE_2_ACCELERATION_STRUCTURE_BUILD_BIT_KHR`, or one of the `VK_PIPELINE_STAGE_*_SHADER_BIT` stages
-    If [`dst_access_mask`] includes `VK_ACCESS_2_SHADER_WRITE_BIT`, [`dst_stage_mask`] **must**  include `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`, or one of the `VK_PIPELINE_STAGE_*_SHADER_BIT` stages
-    If [`dst_access_mask`] includes `VK_ACCESS_2_COLOR_ATTACHMENT_READ_BIT`, [`dst_stage_mask`] **must**  include `VK_PIPELINE_STAGE_2_COLOR_ATTACHMENT_OUTPUT_BIT``VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
-    If [`dst_access_mask`] includes `VK_ACCESS_2_COLOR_ATTACHMENT_WRITE_BIT`, [`dst_stage_mask`] **must**  include `VK_PIPELINE_STAGE_2_COLOR_ATTACHMENT_OUTPUT_BIT``VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
-    If [`dst_access_mask`] includes `VK_ACCESS_2_DEPTH_STENCIL_ATTACHMENT_READ_BIT`, [`dst_stage_mask`] **must**  include `VK_PIPELINE_STAGE_2_EARLY_FRAGMENT_TESTS_BIT`, `VK_PIPELINE_STAGE_2_LATE_FRAGMENT_TESTS_BIT`, `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
-    If [`dst_access_mask`] includes `VK_ACCESS_2_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT`, [`dst_stage_mask`] **must**  include `VK_PIPELINE_STAGE_2_EARLY_FRAGMENT_TESTS_BIT`, `VK_PIPELINE_STAGE_2_LATE_FRAGMENT_TESTS_BIT`, `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
-    If [`dst_access_mask`] includes `VK_ACCESS_2_TRANSFER_READ_BIT`, [`dst_stage_mask`] **must**  include `VK_PIPELINE_STAGE_2_COPY_BIT`, `VK_PIPELINE_STAGE_2_BLIT_BIT`, `VK_PIPELINE_STAGE_2_RESOLVE_BIT`, `VK_PIPELINE_STAGE_2_ALL_TRANSFER_BIT`, `VK_PIPELINE_STAGE_2_ACCELERATION_STRUCTURE_BUILD_BIT_KHR`, or `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
-    If [`dst_access_mask`] includes `VK_ACCESS_2_TRANSFER_WRITE_BIT`, [`dst_stage_mask`] **must**  include `VK_PIPELINE_STAGE_2_COPY_BIT`, `VK_PIPELINE_STAGE_2_BLIT_BIT`, `VK_PIPELINE_STAGE_2_RESOLVE_BIT`, `VK_PIPELINE_STAGE_2_CLEAR_BIT`, `VK_PIPELINE_STAGE_2_ALL_TRANSFER_BIT`, `VK_PIPELINE_STAGE_2_ACCELERATION_STRUCTURE_BUILD_BIT_KHR`, or `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
-    If [`dst_access_mask`] includes `VK_ACCESS_2_HOST_READ_BIT`, [`dst_stage_mask`] **must**  include `VK_PIPELINE_STAGE_2_HOST_BIT`
-    If [`dst_access_mask`] includes `VK_ACCESS_2_HOST_WRITE_BIT`, [`dst_stage_mask`] **must**  include `VK_PIPELINE_STAGE_2_HOST_BIT`
-    If [`dst_access_mask`] includes `VK_ACCESS_2_CONDITIONAL_RENDERING_READ_BIT_EXT`, [`dst_stage_mask`] **must**  include `VK_PIPELINE_STAGE_2_CONDITIONAL_RENDERING_BIT_EXT`, `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
-    If [`dst_access_mask`] includes `VK_ACCESS_2_FRAGMENT_DENSITY_MAP_READ_BIT_EXT`, [`dst_stage_mask`] **must**  include `VK_PIPELINE_STAGE_2_FRAGMENT_DENSITY_PROCESS_BIT_EXT`, `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
-    If [`dst_access_mask`] includes `VK_ACCESS_2_TRANSFORM_FEEDBACK_WRITE_BIT_EXT`, [`dst_stage_mask`] **must**  include `VK_PIPELINE_STAGE_2_TRANSFORM_FEEDBACK_BIT_EXT`, `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
-    If [`dst_access_mask`] includes `VK_ACCESS_2_TRANSFORM_FEEDBACK_COUNTER_READ_BIT_EXT`, [`dst_stage_mask`] **must**  include `VK_PIPELINE_STAGE_2_DRAW_INDIRECT_BIT`, `VK_PIPELINE_STAGE_2_TRANSFORM_FEEDBACK_BIT_EXT`, `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
-    If [`dst_access_mask`] includes `VK_ACCESS_2_TRANSFORM_FEEDBACK_COUNTER_WRITE_BIT_EXT`, [`dst_stage_mask`] **must**  include `VK_PIPELINE_STAGE_2_TRANSFORM_FEEDBACK_BIT_EXT`, `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
-    If [`dst_access_mask`] includes `VK_ACCESS_2_SHADING_RATE_IMAGE_READ_BIT_NV`, [`dst_stage_mask`] **must**  include `VK_PIPELINE_STAGE_2_SHADING_RATE_IMAGE_BIT_NV`, `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
-    If [`dst_access_mask`] includes `VK_ACCESS_2_INVOCATION_MASK_READ_BIT_HUAWEI`, [`dst_stage_mask`] **must**  include `VK_PIPELINE_STAGE_2_INVOCATION_MASK_BIT_HUAWEI`
-    If [`dst_access_mask`] includes `VK_ACCESS_2_COMMAND_PREPROCESS_READ_BIT_NV`, [`dst_stage_mask`] **must**  include `VK_PIPELINE_STAGE_2_COMMAND_PREPROCESS_BIT_NV` or `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
-    If [`dst_access_mask`] includes `VK_ACCESS_2_COMMAND_PREPROCESS_WRITE_BIT_NV`, [`dst_stage_mask`] **must**  include `VK_PIPELINE_STAGE_2_COMMAND_PREPROCESS_BIT_NV` or `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
-    If [`dst_access_mask`] includes `VK_ACCESS_2_COLOR_ATTACHMENT_READ_NONCOHERENT_BIT_EXT`, [`dst_stage_mask`] **must**  include `VK_PIPELINE_STAGE_2_COLOR_ATTACHMENT_OUTPUT_BIT``VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
-    If [`dst_access_mask`] includes `VK_ACCESS_2_ACCELERATION_STRUCTURE_READ_BIT_KHR`, [`dst_stage_mask`] **must**  include `VK_PIPELINE_STAGE_2_ACCELERATION_STRUCTURE_BUILD_BIT_KHR`, `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`, or one of the `VK_PIPELINE_STAGE_*_SHADER_BIT` stages
-    If [`dst_access_mask`] includes `VK_ACCESS_2_ACCELERATION_STRUCTURE_WRITE_BIT_KHR`, [`dst_stage_mask`] **must**  include `VK_PIPELINE_STAGE_2_ACCELERATION_STRUCTURE_BUILD_BIT_KHR` or `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
-    If [`rayQuery`](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-rayQuery) is not enabled and [`dst_access_mask`] includes `VK_ACCESS_2_ACCELERATION_STRUCTURE_READ_BIT_KHR`, [`dst_stage_mask`] **must**  not include any of the `VK_PIPELINE_STAGE_*_SHADER_BIT` stages except `VK_PIPELINE_STAGE_2_RAY_TRACING_SHADER_BIT_KHR`
-    If [`dst_access_mask`] includes `VK_ACCESS_2_VIDEO_DECODE_READ_BIT_KHR`, [`dst_stage_mask`] **must**  include `VK_PIPELINE_STAGE_2_VIDEO_DECODE_BIT_KHR`
-    If [`dst_access_mask`] includes `VK_ACCESS_2_VIDEO_DECODE_WRITE_BIT_KHR`, [`dst_stage_mask`] **must**  include `VK_PIPELINE_STAGE_2_VIDEO_DECODE_BIT_KHR`
-    If [`dst_access_mask`] includes `VK_ACCESS_2_VIDEO_ENCODE_READ_BIT_KHR`, [`dst_stage_mask`] **must**  include `VK_PIPELINE_STAGE_2_VIDEO_ENCODE_BIT_KHR`
-    If [`dst_access_mask`] includes `VK_ACCESS_2_VIDEO_ENCODE_WRITE_BIT_KHR`, [`dst_stage_mask`] **must**  include `VK_PIPELINE_STAGE_2_VIDEO_ENCODE_BIT_KHR`

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_MEMORY_BARRIER_2`
-  [`src_stage_mask`] **must**  be a valid combination of [`PipelineStageFlagBits2`] values
-  [`src_access_mask`] **must**  be a valid combination of [`AccessFlagBits2`] values
-  [`dst_stage_mask`] **must**  be a valid combination of [`PipelineStageFlagBits2`] values
-  [`dst_access_mask`] **must**  be a valid combination of [`AccessFlagBits2`] values

# Related
- [`khr_synchronization2`]
- [`crate::vulkan1_3`]
- [`AccessFlags2`]
- [`DependencyInfo`]
- [`PipelineStageFlags2`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        