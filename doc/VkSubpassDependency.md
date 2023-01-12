[VkSubpassDependency](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSubpassDependency.html) - Structure specifying a subpass dependency

# C Specifications
The [`SubpassDependency`] structure is defined as:
```c
// Provided by VK_VERSION_1_0
typedef struct VkSubpassDependency {
    uint32_t                srcSubpass;
    uint32_t                dstSubpass;
    VkPipelineStageFlags    srcStageMask;
    VkPipelineStageFlags    dstStageMask;
    VkAccessFlags           srcAccessMask;
    VkAccessFlags           dstAccessMask;
    VkDependencyFlags       dependencyFlags;
} VkSubpassDependency;
```

# Members
- [`src_subpass`] is the subpass index of the first subpass in the dependency, or `VK_SUBPASS_EXTERNAL`.
- [`dst_subpass`] is the subpass index of the second subpass in the dependency, or `VK_SUBPASS_EXTERNAL`.
- [`src_stage_mask`] is a bitmask of [`PipelineStageFlagBits`] specifying the [source stage mask](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-pipeline-stages-masks).
- [`dst_stage_mask`] is a bitmask of [`PipelineStageFlagBits`] specifying the [destination stage mask](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-pipeline-stages-masks)
- [`src_access_mask`] is a bitmask of [`AccessFlagBits`] specifying a [source access mask](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-access-masks).
- [`dst_access_mask`] is a bitmask of [`AccessFlagBits`] specifying a [destination access mask](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-access-masks).
- [`dependency_flags`] is a bitmask of [`DependencyFlagBits`].

# Description
If [`src_subpass`] is equal to [`dst_subpass`] then the
[`SubpassDependency`] describes a
[subpass
self-dependency](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-pipeline-barriers-subpass-self-dependencies), and only constrains the pipeline barriers allowed within
a subpass instance.
Otherwise, when a render pass instance which includes a subpass dependency
is submitted to a queue, it defines a memory dependency between the
subpasses identified by [`src_subpass`] and [`dst_subpass`].If [`src_subpass`] is equal to `VK_SUBPASS_EXTERNAL`, the first
[synchronization scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-scopes) includes
commands that occur earlier in [submission
order](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-submission-order) than the [`cmd_begin_render_pass`] used to begin the render pass
instance.
Otherwise, the first set of commands includes all commands submitted as part
of the subpass instance identified by [`src_subpass`] and any load, store
or multisample resolve operations on attachments used in [`src_subpass`].
In either case, the first synchronization scope is limited to operations on
the pipeline stages determined by the
[source stage mask](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-pipeline-stages-masks) specified by
[`src_stage_mask`].If [`dst_subpass`] is equal to `VK_SUBPASS_EXTERNAL`, the second
[synchronization scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-scopes) includes
commands that occur later in [submission
order](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-submission-order) than the [`cmd_end_render_pass`] used to end the render pass
instance.
Otherwise, the second set of commands includes all commands submitted as
part of the subpass instance identified by [`dst_subpass`] and any load,
store or multisample resolve operations on attachments used in
[`dst_subpass`].
In either case, the second synchronization scope is limited to operations on
the pipeline stages determined by the
[destination stage mask](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-pipeline-stages-masks) specified
by [`dst_stage_mask`].The first [access scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-access-scopes) is
limited to accesses in the pipeline stages determined by the
[source stage mask](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-pipeline-stages-masks) specified by
[`src_stage_mask`].
It is also limited to access types in the [source access mask](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-access-masks) specified by [`src_access_mask`].The second [access scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-access-scopes) is
limited to accesses in the pipeline stages determined by the
[destination stage mask](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-pipeline-stages-masks) specified
by [`dst_stage_mask`].
It is also limited to access types in the [destination access mask](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-access-masks) specified by [`dst_access_mask`].The [availability and
visibility operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-available-and-visible) defined by a subpass dependency affect the execution
of [image layout transitions](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#renderpass-layout-transitions) within the
render pass.
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
-  [`src_subpass`] **must**  be less than or equal to [`dst_subpass`], unless one of them is `VK_SUBPASS_EXTERNAL`, to avoid cyclic dependencies and ensure a valid execution order
-  [`src_subpass`] and [`dst_subpass`] **must**  not both be equal to `VK_SUBPASS_EXTERNAL`
-    If [`src_subpass`] is equal to [`dst_subpass`] and not all of the stages in [`src_stage_mask`] and [`dst_stage_mask`] are [framebuffer-space stages](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-framebuffer-regions), the [logically latest](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-pipeline-stages-order) pipeline stage in [`src_stage_mask`] **must**  be [logically earlier](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-pipeline-stages-order) than or equal to the [logically earliest](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-pipeline-stages-order) pipeline stage in [`dst_stage_mask`]
-    Any access flag included in [`src_access_mask`] **must**  be supported by one of the pipeline stages in [`src_stage_mask`], as specified in the [table of supported access types](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-access-types-supported)
-    Any access flag included in [`dst_access_mask`] **must**  be supported by one of the pipeline stages in [`dst_stage_mask`], as specified in the [table of supported access types](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-access-types-supported)
-    If [`src_subpass`] equals [`dst_subpass`], and [`src_stage_mask`] and [`dst_stage_mask`] both include a [framebuffer-space stage](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-framebuffer-regions), then [`dependency_flags`] **must**  include `VK_DEPENDENCY_BY_REGION_BIT`
-    If [`dependency_flags`] includes `VK_DEPENDENCY_VIEW_LOCAL_BIT`, [`src_subpass`] **must**  not be equal to `VK_SUBPASS_EXTERNAL`
-    If [`dependency_flags`] includes `VK_DEPENDENCY_VIEW_LOCAL_BIT`, [`dst_subpass`] **must**  not be equal to `VK_SUBPASS_EXTERNAL`
-    If [`src_subpass`] equals [`dst_subpass`] and that subpass has more than one bit set in the view mask, then [`dependency_flags`] **must**  include `VK_DEPENDENCY_VIEW_LOCAL_BIT`

## Valid Usage (Implicit)
-  [`src_stage_mask`] **must**  be a valid combination of [`PipelineStageFlagBits`] values
-  [`dst_stage_mask`] **must**  be a valid combination of [`PipelineStageFlagBits`] values
-  [`src_access_mask`] **must**  be a valid combination of [`AccessFlagBits`] values
-  [`dst_access_mask`] **must**  be a valid combination of [`AccessFlagBits`] values
-  [`dependency_flags`] **must**  be a valid combination of [`DependencyFlagBits`] values

# Related
- [`crate::vulkan1_0`]
- [VkAccessFlags]()
- [VkDependencyFlags]()
- [VkPipelineStageFlags]()
- [`RenderPassCreateInfo`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        