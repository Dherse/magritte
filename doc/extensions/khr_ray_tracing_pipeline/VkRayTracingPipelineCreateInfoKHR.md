[VkRayTracingPipelineCreateInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRayTracingPipelineCreateInfoKHR.html) - Structure specifying parameters of a newly created ray tracing pipeline

# C Specifications
The [`RayTracingPipelineCreateInfoKHR`] structure is defined as:
```c
// Provided by VK_KHR_ray_tracing_pipeline
typedef struct VkRayTracingPipelineCreateInfoKHR {
    VkStructureType                                      sType;
    const void*                                          pNext;
    VkPipelineCreateFlags                                flags;
    uint32_t                                             stageCount;
    const VkPipelineShaderStageCreateInfo*               pStages;
    uint32_t                                             groupCount;
    const VkRayTracingShaderGroupCreateInfoKHR*          pGroups;
    uint32_t                                             maxPipelineRayRecursionDepth;
    const VkPipelineLibraryCreateInfoKHR*                pLibraryInfo;
    const VkRayTracingPipelineInterfaceCreateInfoKHR*    pLibraryInterface;
    const VkPipelineDynamicStateCreateInfo*              pDynamicState;
    VkPipelineLayout                                     layout;
    VkPipeline                                           basePipelineHandle;
    int32_t                                              basePipelineIndex;
} VkRayTracingPipelineCreateInfoKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`flags`] is a bitmask of [`PipelineCreateFlagBits`] specifying how the pipeline will be generated.
- [`stage_count`] is the number of entries in the [`stages`] array.
- [`stages`] is a pointer to an array of [`stage_count`][`PipelineShaderStageCreateInfo`] structures describing the set of the shader stages to be included in the ray tracing pipeline.
- [`group_count`] is the number of entries in the [`groups`] array.
- [`groups`] is a pointer to an array of [`group_count`][`RayTracingShaderGroupCreateInfoKHR`] structures describing the set of the shader stages to be included in each shader group in the ray tracing pipeline.
- [`max_pipeline_ray_recursion_depth`] is the [maximum recursion depth](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#ray-tracing-recursion-depth) of shaders executed by this pipeline.
- [`library_info`] is a pointer to a [`PipelineLibraryCreateInfoKHR`] structure defining pipeline libraries to include.
- [`library_interface`] is a pointer to a [`RayTracingPipelineInterfaceCreateInfoKHR`] structure defining additional information when using pipeline libraries.
- [`dynamic_state`] is a pointer to a [`PipelineDynamicStateCreateInfo`] structure, and is used to indicate which properties of the pipeline state object are dynamic and  **can**  be changed independently of the pipeline state. This  **can**  be `NULL`, which means no state in the pipeline is considered dynamic.
- [`layout`] is the description of binding locations used by both the pipeline and descriptor sets used with the pipeline.
- [`base_pipeline_handle`] is a pipeline to derive from.
- [`base_pipeline_index`] is an index into the `pCreateInfos` parameter to use as a pipeline to derive from.

# Description
The parameters [`base_pipeline_handle`] and [`base_pipeline_index`] are
described in more detail in [Pipeline
Derivatives](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#pipelines-pipeline-derivatives).When `VK_PIPELINE_CREATE_LIBRARY_BIT_KHR` is specified, this pipeline
defines a *pipeline library* which  **cannot**  be bound as a ray tracing
pipeline directly.
Instead, pipeline libraries define common shaders and shader groups which
 **can**  be included in future pipeline creation.If pipeline libraries are included in [`library_info`], shaders defined in
those libraries are treated as if they were defined as additional entries in
[`stages`], appended in the order they appear in the `pLibraries`
array and in the [`stages`] array when those libraries were defined.When referencing shader groups in order to obtain a shader group handle,
groups defined in those libraries are treated as if they were defined as
additional entries in [`groups`], appended in the order they appear in
the `pLibraries` array and in the [`groups`] array when those
libraries were defined.
The shaders these groups reference are set when the pipeline library is
created, referencing those specified in the pipeline library, not in the
pipeline that includes it.The default stack size for a pipeline if
`VK_DYNAMIC_STATE_RAY_TRACING_PIPELINE_STACK_SIZE_KHR` is not provided
is computed as described in [Ray Tracing
Pipeline Stack](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#ray-tracing-pipeline-stack).
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
-    If [`flags`] does not include `VK_PIPELINE_CREATE_LIBRARY_BIT_KHR`, the `stage` member of at least one element of [`stages`], including those implicitly added by [`library_info`],  **must**  be `VK_SHADER_STAGE_RAYGEN_BIT_KHR`
-  [`max_pipeline_ray_recursion_depth`] **must**  be less than or equal to [`PhysicalDeviceRayTracingPipelinePropertiesKHR::max_ray_recursion_depth`]
-    If [`flags`] includes `VK_PIPELINE_CREATE_LIBRARY_BIT_KHR`, [`library_interface`] **must**  not be `NULL`
-    If [`library_info`] is not `NULL` and its `libraryCount` member is greater than `0`, its [`library_interface`] member  **must**  not be `NULL`
-    Each element of `pLibraryInfo->pLibraries` **must**  have been created with the value of [`max_pipeline_ray_recursion_depth`] equal to that in this pipeline
-    If [`library_info`] is not `NULL`, each element of its `pLibraries` member  **must**  have been created with a [`layout`] that is compatible with the [`layout`] in this pipeline
-    If [`library_info`] is not `NULL`, each element of its `pLibraries` member  **must**  have been created with values of the `maxPipelineRayPayloadSize` and `maxPipelineRayHitAttributeSize` members of [`library_interface`] equal to those in this pipeline
-    If [`flags`] includes `VK_PIPELINE_CREATE_RAY_TRACING_SHADER_GROUP_HANDLE_CAPTURE_REPLAY_BIT_KHR`, each element of `pLibraryInfo->pLibraries` **must**  have been created with the `VK_PIPELINE_CREATE_RAY_TRACING_SHADER_GROUP_HANDLE_CAPTURE_REPLAY_BIT_KHR` bit set
-    If [`flags`] includes `VK_PIPELINE_CREATE_RAY_TRACING_SKIP_AABBS_BIT_KHR`, each element of `pLibraryInfo->pLibraries` **must**  have been created with the `VK_PIPELINE_CREATE_RAY_TRACING_SKIP_AABBS_BIT_KHR` bit set
-    If [`flags`] includes `VK_PIPELINE_CREATE_RAY_TRACING_SKIP_TRIANGLES_BIT_KHR`, each element of `pLibraryInfo->pLibraries` **must**  have been created with the `VK_PIPELINE_CREATE_RAY_TRACING_SKIP_TRIANGLES_BIT_KHR` bit set
-    If [`flags`] includes `VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_ANY_HIT_SHADERS_BIT_KHR`, each element of `pLibraryInfo->pLibraries` **must**  have been created with the `VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_ANY_HIT_SHADERS_BIT_KHR` bit set
-    If [`flags`] includes `VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_CLOSEST_HIT_SHADERS_BIT_KHR`, each element of `pLibraryInfo->pLibraries` **must**  have been created with the `VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_CLOSEST_HIT_SHADERS_BIT_KHR` bit set
-    If [`flags`] includes `VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_INTERSECTION_SHADERS_BIT_KHR`, each element of `pLibraryInfo->pLibraries` **must**  have been created with the `VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_INTERSECTION_SHADERS_BIT_KHR` bit set
-    If [`flags`] includes `VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_MISS_SHADERS_BIT_KHR`, each element of `pLibraryInfo->pLibraries` **must**  have been created with the `VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_MISS_SHADERS_BIT_KHR` bit set
-    If the `[`VK_KHR_pipeline_library`]` extension is not enabled, [`library_info`] and [`library_interface`] **must**  be `NULL`
-    If [`flags`] includes `VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_ANY_HIT_SHADERS_BIT_KHR`, for any element of [`groups`] with a `type` of `VK_RAY_TRACING_SHADER_GROUP_TYPE_TRIANGLES_HIT_GROUP_KHR` or `VK_RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP_KHR`, the `anyHitShader` of that element  **must**  not be [`SHADER_UNUSED_KHR`]
-    If [`flags`] includes `VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_CLOSEST_HIT_SHADERS_BIT_KHR`, for any element of [`groups`] with a `type` of `VK_RAY_TRACING_SHADER_GROUP_TYPE_TRIANGLES_HIT_GROUP_KHR` or `VK_RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP_KHR`, the `closestHitShader` of that element  **must**  not be [`SHADER_UNUSED_KHR`]
-    If the [`rayTraversalPrimitiveCulling`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-rayTraversalPrimitiveCulling) feature is not enabled, [`flags`] **must**  not include `VK_PIPELINE_CREATE_RAY_TRACING_SKIP_AABBS_BIT_KHR`
-    If the [`rayTraversalPrimitiveCulling`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-rayTraversalPrimitiveCulling) feature is not enabled, [`flags`] **must**  not include `VK_PIPELINE_CREATE_RAY_TRACING_SKIP_TRIANGLES_BIT_KHR`
-  [`flags`] **must**  not include both `VK_PIPELINE_CREATE_RAY_TRACING_SKIP_TRIANGLES_BIT_KHR` and `VK_PIPELINE_CREATE_RAY_TRACING_SKIP_AABBS_BIT_KHR`
-    If [`flags`] includes `VK_PIPELINE_CREATE_RAY_TRACING_SHADER_GROUP_HANDLE_CAPTURE_REPLAY_BIT_KHR`, [`rayTracingPipelineShaderGroupHandleCaptureReplay`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-rayTracingPipelineShaderGroupHandleCaptureReplay) **must**  be enabled
-    If [`PhysicalDeviceRayTracingPipelineFeaturesKHR::ray_tracing_pipeline_shader_group_handle_capture_replay`] is [`TRUE`] and the `pShaderGroupCaptureReplayHandle` member of any element of [`groups`] is not `NULL`, [`flags`] **must**  include `VK_PIPELINE_CREATE_RAY_TRACING_SHADER_GROUP_HANDLE_CAPTURE_REPLAY_BIT_KHR`
-    If [`library_info`] is not `NULL` and its `libraryCount` is `0`, [`stage_count`] **must**  not be `0`
-    If [`library_info`] is not `NULL` and its `libraryCount` is `0`, [`group_count`] **must**  not be `0`
-    Any element of the `pDynamicStates` member of [`dynamic_state`] **must**  be `VK_DYNAMIC_STATE_RAY_TRACING_PIPELINE_STACK_SIZE_KHR`

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_RAY_TRACING_PIPELINE_CREATE_INFO_KHR`
-  [`p_next`] **must**  be `NULL` or a pointer to a valid instance of [`PipelineCreationFeedbackCreateInfo`]
-    The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique
-  [`flags`] **must**  be a valid combination of [`PipelineCreateFlagBits`] values
-    If [`stage_count`] is not `0`, [`stages`] **must**  be a valid pointer to an array of [`stage_count`] valid [`PipelineShaderStageCreateInfo`] structures
-    If [`group_count`] is not `0`, [`groups`] **must**  be a valid pointer to an array of [`group_count`] valid [`RayTracingShaderGroupCreateInfoKHR`] structures
-    If [`library_info`] is not `NULL`, [`library_info`] **must**  be a valid pointer to a valid [`PipelineLibraryCreateInfoKHR`] structure
-    If [`library_interface`] is not `NULL`, [`library_interface`] **must**  be a valid pointer to a valid [`RayTracingPipelineInterfaceCreateInfoKHR`] structure
-    If [`dynamic_state`] is not `NULL`, [`dynamic_state`] **must**  be a valid pointer to a valid [`PipelineDynamicStateCreateInfo`] structure
-  [`layout`] **must**  be a valid [`PipelineLayout`] handle
-    Both of [`base_pipeline_handle`], and [`layout`] that are valid handles of non-ignored parameters  **must**  have been created, allocated, or retrieved from the same [`Device`]

# Related
- [`VK_KHR_ray_tracing_pipeline`]
- [`Pipeline`]
- [`PipelineCreateFlags`]
- [`PipelineDynamicStateCreateInfo`]
- [`PipelineLayout`]
- [`PipelineLibraryCreateInfoKHR`]
- [`PipelineShaderStageCreateInfo`]
- [`RayTracingPipelineInterfaceCreateInfoKHR`]
- [`RayTracingShaderGroupCreateInfoKHR`]
- [`StructureType`]
- [`create_ray_tracing_pipelines_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        