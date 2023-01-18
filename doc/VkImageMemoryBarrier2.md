[VkImageMemoryBarrier2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageMemoryBarrier2.html) - Structure specifying an image memory barrier

# C Specifications
The [`ImageMemoryBarrier2`] structure is defined as:
```c
// Provided by VK_VERSION_1_3
typedef struct VkImageMemoryBarrier2 {
    VkStructureType            sType;
    const void*                pNext;
    VkPipelineStageFlags2      srcStageMask;
    VkAccessFlags2             srcAccessMask;
    VkPipelineStageFlags2      dstStageMask;
    VkAccessFlags2             dstAccessMask;
    VkImageLayout              oldLayout;
    VkImageLayout              newLayout;
    uint32_t                   srcQueueFamilyIndex;
    uint32_t                   dstQueueFamilyIndex;
    VkImage                    image;
    VkImageSubresourceRange    subresourceRange;
} VkImageMemoryBarrier2;
```
or the equivalent
```c
// Provided by VK_KHR_synchronization2
typedef VkImageMemoryBarrier2 VkImageMemoryBarrier2KHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`src_stage_mask`] is a [`PipelineStageFlags2`] mask of pipeline stages to be included in the [first synchronization scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-scopes).
- [`src_access_mask`] is a [`AccessFlags2`] mask of access flags to be included in the [first access scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-access-scopes).
- [`dst_stage_mask`] is a [`PipelineStageFlags2`] mask of pipeline stages to be included in the [second synchronization scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-scopes).
- [`dst_access_mask`] is a [`AccessFlags2`] mask of access flags to be included in the [second access scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-access-scopes).
- [`old_layout`] is the old layout in an [image layout transition](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-image-layout-transitions).
- [`new_layout`] is the new layout in an [image layout transition](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-image-layout-transitions).
- [`src_queue_family_index`] is the source queue family for a [queue family ownership transfer](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-queue-transfers).
- [`dst_queue_family_index`] is the destination queue family for a [queue family ownership transfer](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-queue-transfers).
- [`image`] is a handle to the image affected by this barrier.
- [`subresource_range`] describes the [image subresource range](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#resources-image-views) within [`image`] that is affected by this barrier.

# Description
This structure defines a [memory
dependency](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-memory) limited to an image subresource range, and  **can**  define a
[queue family transfer operation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-queue-transfers) and
[image layout transition](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-image-layout-transitions) for
that subresource range.The first [synchronization scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-scopes) and
[access scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-access-scopes) described by
this structure include only operations and memory accesses specified by
[`src_stage_mask`] and [`src_access_mask`].The second [synchronization scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-scopes)
and [access scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-access-scopes) described
by this structure include only operations and memory accesses specified by
[`dst_stage_mask`] and [`dst_access_mask`].Both [access scopes](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-access-scopes) are
limited to only memory accesses to [`image`] in the subresource range
defined by [`subresource_range`].If [`image`] was created with `VK_SHARING_MODE_EXCLUSIVE`, and
[`src_queue_family_index`] is not equal to [`dst_queue_family_index`], this
memory barrier defines a [queue family
transfer operation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-queue-transfers).
When executed on a queue in the family identified by
[`src_queue_family_index`], this barrier defines a
[queue family release operation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-queue-transfers-release)
for the specified image subresource range, and the second synchronization
and access scopes do not synchronize operations on that queue.
When executed on a queue in the family identified by
[`dst_queue_family_index`], this barrier defines a
[queue family acquire operation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-queue-transfers-acquire)
for the specified image subresource range, and the first synchronization and
access scopes do not synchronize operations on that queue.A [queue family transfer operation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-queue-transfers) is
also defined if the values are not equal, and either is one of the special
queue family values reserved for external memory ownership transfers, as
described in [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-queue-transfers](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-queue-transfers).
A [queue family release
operation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-queue-transfers-release) is defined when [`dst_queue_family_index`] is one of those
values, and a [queue family
acquire operation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-queue-transfers-acquire) is defined when [`src_queue_family_index`] is one of
those values.If [`old_layout`] is not equal to [`new_layout`], then the memory barrier
defines an [image layout
transition](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-image-layout-transitions) for the specified image subresource range.
If this memory barrier defines a [queue
family transfer operation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-queue-transfers), the layout transition is only executed once
between the queues.If [`image`] has a multi-planar format and the image is *disjoint*, then
including `VK_IMAGE_ASPECT_COLOR_BIT` in the `aspectMask` member of
[`subresource_range`] is equivalent to including
`VK_IMAGE_ASPECT_PLANE_0_BIT`, `VK_IMAGE_ASPECT_PLANE_1_BIT`, and
(for three-plane formats only) `VK_IMAGE_ASPECT_PLANE_2_BIT`.
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

-  `subresourceRange.baseMipLevel` **must**  be less than the `mipLevels` specified in [`ImageCreateInfo`] when [`image`] was created
-    If `subresourceRange.levelCount` is not [`REMAINING_MIP_LEVELS`], `subresourceRange.baseMipLevel` +  `subresourceRange.levelCount` **must**  be less than or equal to the `mipLevels` specified in [`ImageCreateInfo`] when [`image`] was created
-  `subresourceRange.baseArrayLayer` **must**  be less than the `arrayLayers` specified in [`ImageCreateInfo`] when [`image`] was created
-    If `subresourceRange.layerCount` is not [`REMAINING_ARRAY_LAYERS`], `subresourceRange.baseArrayLayer` +  `subresourceRange.layerCount` **must**  be less than or equal to the `arrayLayers` specified in [`ImageCreateInfo`] when [`image`] was created
-    If [`image`] is non-sparse then it  **must**  be bound completely and contiguously to a single [`DeviceMemory`] object
-    If [`src_queue_family_index`] and [`dst_queue_family_index`] define a [queue family ownership transfer](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-queue-transfers) or [`old_layout`] and [`new_layout`] define an [image layout transition](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-image-layout-transitions), and [`old_layout`] or [`new_layout`] is `VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL` then [`image`] **must**  have been created with `VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT`
-    If [`src_queue_family_index`] and [`dst_queue_family_index`] define a [queue family ownership transfer](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-queue-transfers) or [`old_layout`] and [`new_layout`] define an [image layout transition](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-image-layout-transitions), and [`old_layout`] or [`new_layout`] is `VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL` then [`image`] **must**  have been created with `VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT`
-    If [`src_queue_family_index`] and [`dst_queue_family_index`] define a [queue family ownership transfer](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-queue-transfers) or [`old_layout`] and [`new_layout`] define an [image layout transition](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-image-layout-transitions), and [`old_layout`] or [`new_layout`] is `VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL` then [`image`] **must**  have been created with `VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT`
-    If [`src_queue_family_index`] and [`dst_queue_family_index`] define a [queue family ownership transfer](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-queue-transfers) or [`old_layout`] and [`new_layout`] define an [image layout transition](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-image-layout-transitions), and [`old_layout`] or [`new_layout`] is `VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL` then [`image`] **must**  have been created with `VK_IMAGE_USAGE_SAMPLED_BIT` or `VK_IMAGE_USAGE_INPUT_ATTACHMENT_BIT`
-    If [`src_queue_family_index`] and [`dst_queue_family_index`] define a [queue family ownership transfer](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-queue-transfers) or [`old_layout`] and [`new_layout`] define an [image layout transition](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-image-layout-transitions), and [`old_layout`] or [`new_layout`] is `VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL` then [`image`] **must**  have been created with `VK_IMAGE_USAGE_TRANSFER_SRC_BIT`
-    If [`src_queue_family_index`] and [`dst_queue_family_index`] define a [queue family ownership transfer](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-queue-transfers) or [`old_layout`] and [`new_layout`] define an [image layout transition](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-image-layout-transitions), and [`old_layout`] or [`new_layout`] is `VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL` then [`image`] **must**  have been created with `VK_IMAGE_USAGE_TRANSFER_DST_BIT`
-    If [`src_queue_family_index`] and [`dst_queue_family_index`] define a [queue family ownership transfer](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-queue-transfers) or [`old_layout`] and [`new_layout`] define an [image layout transition](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-image-layout-transitions), [`old_layout`] **must**  be `VK_IMAGE_LAYOUT_UNDEFINED` or the current layout of the image subresources affected by the barrier
-    If [`src_queue_family_index`] and [`dst_queue_family_index`] define a [queue family ownership transfer](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-queue-transfers) or [`old_layout`] and [`new_layout`] define an [image layout transition](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-image-layout-transitions), [`new_layout`] **must**  not be `VK_IMAGE_LAYOUT_UNDEFINED` or `VK_IMAGE_LAYOUT_PREINITIALIZED`
-    If [`src_queue_family_index`] and [`dst_queue_family_index`] define a [queue family ownership transfer](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-queue-transfers) or [`old_layout`] and [`new_layout`] define an [image layout transition](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-image-layout-transitions), and [`old_layout`] or [`new_layout`] is `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL` then [`image`] **must**  have been created with `VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT`
-    If [`src_queue_family_index`] and [`dst_queue_family_index`] define a [queue family ownership transfer](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-queue-transfers) or [`old_layout`] and [`new_layout`] define an [image layout transition](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-image-layout-transitions), and [`old_layout`] or [`new_layout`] is `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL` then [`image`] **must**  have been created with `VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT`
-    If [`src_queue_family_index`] and [`dst_queue_family_index`] define a [queue family ownership transfer](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-queue-transfers) or [`old_layout`] and [`new_layout`] define an [image layout transition](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-image-layout-transitions), and [`old_layout`] or [`new_layout`] is `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL` then [`image`] **must**  have been created with at least one of `VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT`, `VK_IMAGE_USAGE_SAMPLED_BIT`, or `VK_IMAGE_USAGE_INPUT_ATTACHMENT_BIT`
-    If [`src_queue_family_index`] and [`dst_queue_family_index`] define a [queue family ownership transfer](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-queue-transfers) or [`old_layout`] and [`new_layout`] define an [image layout transition](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-image-layout-transitions), and [`old_layout`] or [`new_layout`] is `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL` then [`image`] **must**  have been created with `VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT` set
-    If [`src_queue_family_index`] and [`dst_queue_family_index`] define a [queue family ownership transfer](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-queue-transfers) or [`old_layout`] and [`new_layout`] define an [image layout transition](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-image-layout-transitions), and [`old_layout`] or [`new_layout`] is `VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL` then [`image`] **must**  have been created with at least one of `VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT`, `VK_IMAGE_USAGE_SAMPLED_BIT`, or `VK_IMAGE_USAGE_INPUT_ATTACHMENT_BIT`
-    If [`src_queue_family_index`] and [`dst_queue_family_index`] define a [queue family ownership transfer](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-queue-transfers) or [`old_layout`] and [`new_layout`] define an [image layout transition](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-image-layout-transitions), and [`old_layout`] or [`new_layout`] is `VK_IMAGE_LAYOUT_STENCIL_ATTACHMENT_OPTIMAL` then [`image`] **must**  have been created with `VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT` set
-    If [`src_queue_family_index`] and [`dst_queue_family_index`] define a [queue family ownership transfer](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-queue-transfers) or [`old_layout`] and [`new_layout`] define an [image layout transition](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-image-layout-transitions), and [`old_layout`] or [`new_layout`] is `VK_IMAGE_LAYOUT_ATTACHMENT_OPTIMAL`, [`image`] **must**  have been created with `VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT` or `VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT`
-    If [`src_queue_family_index`] and [`dst_queue_family_index`] define a [queue family ownership transfer](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-queue-transfers) or [`old_layout`] and [`new_layout`] define an [image layout transition](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-image-layout-transitions), and [`old_layout`] or [`new_layout`] is `VK_IMAGE_LAYOUT_READ_ONLY_OPTIMAL`, [`image`] **must**  have been created with at least one of `VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT`, `VK_IMAGE_USAGE_SAMPLED_BIT`, or `VK_IMAGE_USAGE_INPUT_ATTACHMENT_BIT`
-    If [`src_queue_family_index`] and [`dst_queue_family_index`] define a [queue family ownership transfer](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-queue-transfers) or [`old_layout`] and [`new_layout`] define an [image layout transition](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-image-layout-transitions), and [`old_layout`] or [`new_layout`] is `VK_IMAGE_LAYOUT_FRAGMENT_SHADING_RATE_ATTACHMENT_OPTIMAL_KHR` then [`image`] **must**  have been created with `VK_IMAGE_USAGE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR` set
-    If [`image`] has a single-plane color format or is not *disjoint*, then the `aspectMask` member of [`subresource_range`] **must**  be `VK_IMAGE_ASPECT_COLOR_BIT`
-    If [`image`] has a multi-planar format and the image is *disjoint*, then the `aspectMask` member of [`subresource_range`] **must**  include either at least one of `VK_IMAGE_ASPECT_PLANE_0_BIT`, `VK_IMAGE_ASPECT_PLANE_1_BIT`, and `VK_IMAGE_ASPECT_PLANE_2_BIT`; or  **must**  include `VK_IMAGE_ASPECT_COLOR_BIT`
-    If [`image`] has a multi-planar format with only two planes, then the `aspectMask` member of [`subresource_range`] **must**  not include `VK_IMAGE_ASPECT_PLANE_2_BIT`
-    If [`image`] has a depth/stencil format with both depth and stencil and the [separateDepthStencilLayouts](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-separateDepthStencilLayouts) feature is enabled, then the `aspectMask` member of [`subresource_range`] **must**  include either or both `VK_IMAGE_ASPECT_DEPTH_BIT` and `VK_IMAGE_ASPECT_STENCIL_BIT`
-    If [`image`] has a depth/stencil format with both depth and stencil and the [separateDepthStencilLayouts](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-separateDepthStencilLayouts) feature is not enabled, then the `aspectMask` member of [`subresource_range`] **must**  include both `VK_IMAGE_ASPECT_DEPTH_BIT` and `VK_IMAGE_ASPECT_STENCIL_BIT`
-    If [`src_queue_family_index`] is not equal to [`dst_queue_family_index`], at least one  **must**  not be a special queue family reserved for external memory ownership transfers, as described in [[synchronization-queue-transfers]](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-queue-transfers)
-    If [`image`] was created with a sharing mode of `VK_SHARING_MODE_CONCURRENT`, [`src_queue_family_index`] and [`dst_queue_family_index`] are not equal, and one of [`src_queue_family_index`] and [`dst_queue_family_index`] is one of the special queue family values reserved for external memory transfers, the other  **must**  be [`QUEUE_FAMILY_IGNORED`]
-    If [`image`] was created with a sharing mode of `VK_SHARING_MODE_EXCLUSIVE`, and [`src_queue_family_index`] and [`dst_queue_family_index`] are not equal, [`src_queue_family_index`] and [`dst_queue_family_index`] **must**  both be valid queue families, or one of the special queue family values reserved for external memory transfers, as described in [[synchronization-queue-transfers]](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-queue-transfers)
-    If either [`src_stage_mask`] or [`dst_stage_mask`] includes `VK_PIPELINE_STAGE_2_HOST_BIT`, [`src_queue_family_index`] and [`dst_queue_family_index`] **must**  be equal
-    If [`src_stage_mask`] includes `VK_PIPELINE_STAGE_2_HOST_BIT`, and [`src_queue_family_index`] and [`dst_queue_family_index`] define a [queue family ownership transfer](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-queue-transfers) or [`old_layout`] and [`new_layout`] define an [image layout transition](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-image-layout-transitions), [`old_layout`] **must**  be one of `VK_IMAGE_LAYOUT_PREINITIALIZED`, `VK_IMAGE_LAYOUT_UNDEFINED`, or `VK_IMAGE_LAYOUT_GENERAL`

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER_2`
-  [`p_next`] **must**  be `NULL` or a pointer to a valid instance of [`SampleLocationsInfoEXT`]
-    The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique
-  [`src_stage_mask`] **must**  be a valid combination of [`PipelineStageFlagBits2`] values
-  [`src_access_mask`] **must**  be a valid combination of [`AccessFlagBits2`] values
-  [`dst_stage_mask`] **must**  be a valid combination of [`PipelineStageFlagBits2`] values
-  [`dst_access_mask`] **must**  be a valid combination of [`AccessFlagBits2`] values
-  [`old_layout`] **must**  be a valid [`ImageLayout`] value
-  [`new_layout`] **must**  be a valid [`ImageLayout`] value
-  [`image`] **must**  be a valid [`Image`] handle
-  [`subresource_range`] **must**  be a valid [`ImageSubresourceRange`] structure

# Related
- [`VK_KHR_synchronization2`]
- [`crate::vulkan1_3`]
- [`AccessFlags2`]
- [`DependencyInfo`]
- [`Image`]
- [`ImageLayout`]
- [`ImageSubresourceRange`]
- [`PipelineStageFlags2`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        