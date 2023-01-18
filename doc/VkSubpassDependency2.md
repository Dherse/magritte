[VkSubpassDependency2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSubpassDependency2.html) - Structure specifying a subpass dependency

# C Specifications
The [`SubpassDependency2`] structure is defined as:
```c
// Provided by VK_VERSION_1_2
typedef struct VkSubpassDependency2 {
    VkStructureType         sType;
    const void*             pNext;
    uint32_t                srcSubpass;
    uint32_t                dstSubpass;
    VkPipelineStageFlags    srcStageMask;
    VkPipelineStageFlags    dstStageMask;
    VkAccessFlags           srcAccessMask;
    VkAccessFlags           dstAccessMask;
    VkDependencyFlags       dependencyFlags;
    int32_t                 viewOffset;
} VkSubpassDependency2;
```
or the equivalent
```c
// Provided by VK_KHR_create_renderpass2
typedef VkSubpassDependency2 VkSubpassDependency2KHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`src_subpass`] is the subpass index of the first subpass in the dependency, or [`SUBPASS_EXTERNAL`].
- [`dst_subpass`] is the subpass index of the second subpass in the dependency, or [`SUBPASS_EXTERNAL`].
- [`src_stage_mask`] is a bitmask of [`PipelineStageFlagBits`] specifying the [source stage mask](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-pipeline-stages-masks).
- [`dst_stage_mask`] is a bitmask of [`PipelineStageFlagBits`] specifying the [destination stage mask](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-pipeline-stages-masks)
- [`src_access_mask`] is a bitmask of [`AccessFlagBits`] specifying a [source access mask](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-access-masks).
- [`dst_access_mask`] is a bitmask of [`AccessFlagBits`] specifying a [destination access mask](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-access-masks).
- [`dependency_flags`] is a bitmask of [`DependencyFlagBits`].
- [`view_offset`] controls which views in the source subpass the views in the destination subpass depend on.

# Description
Parameters defined by this structure with the same name as those in
[`SubpassDependency`] have the identical effect to those parameters.[`view_offset`] has the same effect for the described subpass dependency as
[`RenderPassMultiviewCreateInfo::view_offsets`] has on each
corresponding subpass dependency.If a [`MemoryBarrier2`] is included in the [`p_next`] chain,
[`src_stage_mask`], [`dst_stage_mask`], [`src_access_mask`], and
[`dst_access_mask`] parameters are ignored.
The synchronization and access scopes instead are defined by the parameters
of [`MemoryBarrier2`].
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
-  [`src_subpass`] **must**  be less than or equal to [`dst_subpass`], unless one of them is [`SUBPASS_EXTERNAL`], to avoid cyclic dependencies and ensure a valid execution order
-  [`src_subpass`] and [`dst_subpass`] **must**  not both be equal to [`SUBPASS_EXTERNAL`]
-    If [`src_subpass`] is equal to [`dst_subpass`] and not all of the stages in [`src_stage_mask`] and [`dst_stage_mask`] are [framebuffer-space stages](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-framebuffer-regions), the [logically latest](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-pipeline-stages-order) pipeline stage in [`src_stage_mask`] **must**  be [logically earlier](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-pipeline-stages-order) than or equal to the [logically earliest](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-pipeline-stages-order) pipeline stage in [`dst_stage_mask`]
-    Any access flag included in [`src_access_mask`] **must**  be supported by one of the pipeline stages in [`src_stage_mask`], as specified in the [table of supported access types](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-access-types-supported)
-    Any access flag included in [`dst_access_mask`] **must**  be supported by one of the pipeline stages in [`dst_stage_mask`], as specified in the [table of supported access types](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-access-types-supported)
-    If [`dependency_flags`] includes `VK_DEPENDENCY_VIEW_LOCAL_BIT`, [`src_subpass`] **must**  not be equal to [`SUBPASS_EXTERNAL`]
-    If [`dependency_flags`] includes `VK_DEPENDENCY_VIEW_LOCAL_BIT`, [`dst_subpass`] **must**  not be equal to [`SUBPASS_EXTERNAL`]
-    If [`src_subpass`] equals [`dst_subpass`], and [`src_stage_mask`] and [`dst_stage_mask`] both include a [framebuffer-space stage](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-framebuffer-regions), then [`dependency_flags`] **must**  include `VK_DEPENDENCY_BY_REGION_BIT`
-    If [`view_offset`] is not equal to `0`, [`src_subpass`] **must**  not be equal to [`dst_subpass`]
-    If [`dependency_flags`] does not include `VK_DEPENDENCY_VIEW_LOCAL_BIT`, [`view_offset`] **must**  be `0`

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_SUBPASS_DEPENDENCY_2`
-  [`p_next`] **must**  be `NULL` or a pointer to a valid instance of [`MemoryBarrier2`]
-    The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique
-  [`src_stage_mask`] **must**  be a valid combination of [`PipelineStageFlagBits`] values
-  [`dst_stage_mask`] **must**  be a valid combination of [`PipelineStageFlagBits`] values
-  [`src_access_mask`] **must**  be a valid combination of [`AccessFlagBits`] values
-  [`dst_access_mask`] **must**  be a valid combination of [`AccessFlagBits`] values
-  [`dependency_flags`] **must**  be a valid combination of [`DependencyFlagBits`] values

# Related
- [`VK_KHR_create_renderpass2`]
- [`crate::vulkan1_2`]
- [`AccessFlags`]
- [`DependencyFlags`]
- [`PipelineStageFlags`]
- [`RenderPassCreateInfo2`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        