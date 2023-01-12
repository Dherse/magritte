[VkRayTracingPipelineCreateInfoNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRayTracingPipelineCreateInfoNV.html) - Structure specifying parameters of a newly created ray tracing pipeline

# C Specifications
The [`RayTracingPipelineCreateInfoNV`] structure is defined as:
```c
// Provided by VK_NV_ray_tracing
typedef struct VkRayTracingPipelineCreateInfoNV {
    VkStructureType                               sType;
    const void*                                   pNext;
    VkPipelineCreateFlags                         flags;
    uint32_t                                      stageCount;
    const VkPipelineShaderStageCreateInfo*        pStages;
    uint32_t                                      groupCount;
    const VkRayTracingShaderGroupCreateInfoNV*    pGroups;
    uint32_t                                      maxRecursionDepth;
    VkPipelineLayout                              layout;
    VkPipeline                                    basePipelineHandle;
    int32_t                                       basePipelineIndex;
} VkRayTracingPipelineCreateInfoNV;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`flags`] is a bitmask of [`PipelineCreateFlagBits`] specifying how the pipeline will be generated.
- [`stage_count`] is the number of entries in the [`stages`] array.
- [`stages`] is a pointer to an array of [`PipelineShaderStageCreateInfo`] structures specifying the set of the shader stages to be included in the ray tracing pipeline.
- [`group_count`] is the number of entries in the [`groups`] array.
- [`groups`] is a pointer to an array of [`RayTracingShaderGroupCreateInfoNV`] structures describing the set of the shader stages to be included in each shader group in the ray tracing pipeline.
- [`max_recursion_depth`] is the [maximum recursion depth](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#ray-tracing-recursion-depth) of shaders executed by this pipeline.
- [`layout`] is the description of binding locations used by both the pipeline and descriptor sets used with the pipeline.
- [`base_pipeline_handle`] is a pipeline to derive from.
- [`base_pipeline_index`] is an index into the `pCreateInfos` parameter to use as a pipeline to derive from.

# Description
The parameters [`base_pipeline_handle`] and [`base_pipeline_index`] are
described in more detail in [Pipeline
Derivatives](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#pipelines-pipeline-derivatives).
## Valid Usage
-    If [`flags`] contains the `VK_PIPELINE_CREATE_DERIVATIVE_BIT` flag, and [`base_pipeline_index`] is `-1`, [`base_pipeline_handle`] **must**  be a valid handle to a ray tracing [`Pipeline`]
-    If [`flags`] contains the `VK_PIPELINE_CREATE_DERIVATIVE_BIT` flag, and [`base_pipeline_handle`] is [`crate::Handle::null`], [`base_pipeline_index`] **must**  be a valid index into the calling command’s `pCreateInfos` parameter
-    If [`flags`] contains the `VK_PIPELINE_CREATE_DERIVATIVE_BIT` flag, and [`base_pipeline_index`] is not `-1`, [`base_pipeline_handle`] **must**  be [`crate::Handle::null`]
-    If [`flags`] contains the `VK_PIPELINE_CREATE_DERIVATIVE_BIT` flag, and [`base_pipeline_handle`] is not [`crate::Handle::null`], [`base_pipeline_index`] **must**  be `-1`
-    The shader code for the entry points identified by [`stages`], and the rest of the state identified by this structure  **must**  adhere to the pipeline linking rules described in the [Shader Interfaces](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#interfaces) chapter
-  [`layout`] **must**  be [consistent](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#descriptorsets-pipelinelayout-consistency) with all shaders specified in [`stages`]
-    The number of resources in [`layout`] accessible to each shader stage that is used by the pipeline  **must**  be less than or equal to [`PhysicalDeviceLimits::max_per_stage_resources`]
-  [`flags`] **must**  not include `VK_PIPELINE_CREATE_INDIRECT_BINDABLE_BIT_NV`
-    If the [`pipelineCreationCacheControl`](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-pipelineCreationCacheControl) feature is not enabled, [`flags`] **must**  not include `VK_PIPELINE_CREATE_FAIL_ON_PIPELINE_COMPILE_REQUIRED_BIT` or `VK_PIPELINE_CREATE_EARLY_RETURN_ON_FAILURE_BIT`
-    The `stage` member of at least one element of [`stages`] **must**  be `VK_SHADER_STAGE_RAYGEN_BIT_KHR`
-  [`flags`] **must**  not include `VK_PIPELINE_CREATE_LIBRARY_BIT_KHR`
-  [`max_recursion_depth`] **must**  be less than or equal to [`PhysicalDeviceRayTracingPropertiesNV`]::[`max_recursion_depth`]
-  [`flags`] **must**  not include `VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_ANY_HIT_SHADERS_BIT_KHR`
-  [`flags`] **must**  not include `VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_CLOSEST_HIT_SHADERS_BIT_KHR`
-  [`flags`] **must**  not include `VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_MISS_SHADERS_BIT_KHR`
-  [`flags`] **must**  not include `VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_INTERSECTION_SHADERS_BIT_KHR`
-  [`flags`] **must**  not include `VK_PIPELINE_CREATE_RAY_TRACING_SKIP_AABBS_BIT_KHR`
-  [`flags`] **must**  not include `VK_PIPELINE_CREATE_RAY_TRACING_SKIP_TRIANGLES_BIT_KHR`
-  [`flags`] **must**  not include `VK_PIPELINE_CREATE_RAY_TRACING_SHADER_GROUP_HANDLE_CAPTURE_REPLAY_BIT_KHR`
-  [`flags`] **must**  not include `VK_PIPELINE_CREATE_RAY_TRACING_ALLOW_MOTION_BIT_NV`
-  [`flags`] **must**  not include both `VK_PIPELINE_CREATE_DEFER_COMPILE_BIT_NV` and `VK_PIPELINE_CREATE_FAIL_ON_PIPELINE_COMPILE_REQUIRED_BIT` at the same time

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_RAY_TRACING_PIPELINE_CREATE_INFO_NV`
-  [`p_next`] **must**  be `NULL` or a pointer to a valid instance of [`PipelineCreationFeedbackCreateInfo`]
-    The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique
-  [`flags`] **must**  be a valid combination of [`PipelineCreateFlagBits`] values
-  [`stages`] **must**  be a valid pointer to an array of [`stage_count`] valid [`PipelineShaderStageCreateInfo`] structures
-  [`groups`] **must**  be a valid pointer to an array of [`group_count`] valid [`RayTracingShaderGroupCreateInfoNV`] structures
-  [`layout`] **must**  be a valid [`PipelineLayout`] handle
-  [`stage_count`] **must**  be greater than `0`
-  [`group_count`] **must**  be greater than `0`
-    Both of [`base_pipeline_handle`], and [`layout`] that are valid handles of non-ignored parameters  **must**  have been created, allocated, or retrieved from the same [`Device`]

# Related
- [`nv_ray_tracing`]
- [`Pipeline`]
- [VkPipelineCreateFlags]()
- [`PipelineLayout`]
- [`PipelineShaderStageCreateInfo`]
- [`RayTracingShaderGroupCreateInfoNV`]
- [`StructureType`]
- [`create_ray_tracing_pipelines_nv`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        