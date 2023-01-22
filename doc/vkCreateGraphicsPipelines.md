[vkCreateGraphicsPipelines](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateGraphicsPipelines.html) - Create graphics pipelines

# C Specifications
To create graphics pipelines, call:
```c
// Provided by VK_VERSION_1_0
VkResult vkCreateGraphicsPipelines(
    VkDevice                                    device,
    VkPipelineCache                             pipelineCache,
    uint32_t                                    createInfoCount,
    const VkGraphicsPipelineCreateInfo*         pCreateInfos,
    const VkAllocationCallbacks*                pAllocator,
    VkPipeline*                                 pPipelines);
```

# Parameters
- [`device`] is the logical device that creates the graphics pipelines.
- [`pipeline_cache`] is either [`crate::Handle::null`], indicating that pipeline caching is disabled; or the handle of a valid [pipeline cache](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#pipelines-cache) object, in which case use of that cache is enabled for the duration of the command.
- [`create_info_count`] is the length of the [`p_create_infos`] and [`p_pipelines`] arrays.
- [`p_create_infos`] is a pointer to an array of [`GraphicsPipelineCreateInfo`] structures.
- [`p_allocator`] controls host memory allocation as described in the [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation) chapter.
- [`p_pipelines`] is a pointer to an array of [`Pipeline`] handles in which the resulting graphics pipeline objects are returned.

# Description
The [`GraphicsPipelineCreateInfo`] structure includes an array of
[`PipelineShaderStageCreateInfo`] structures for each of the desired
active shader stages, as well as creation information for all relevant
fixed-function stages, and a pipeline layout.
## Valid Usage
-    If the `flags` member of any element of [`p_create_infos`] contains the `VK_PIPELINE_CREATE_DERIVATIVE_BIT` flag, and the `basePipelineIndex` member of that same element is not `-1`, `basePipelineIndex` **must**  be less than the index into [`p_create_infos`] that corresponds to that element
-    If the `flags` member of any element of [`p_create_infos`] contains the `VK_PIPELINE_CREATE_DERIVATIVE_BIT` flag, the base pipeline  **must**  have been created with the `VK_PIPELINE_CREATE_ALLOW_DERIVATIVES_BIT` flag set
-    If [`pipeline_cache`] was created with `VK_PIPELINE_CACHE_CREATE_EXTERNALLY_SYNCHRONIZED_BIT`, host access to [`pipeline_cache`] **must**  be [externally synchronized](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#fundamentals-threadingbehavior)

## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-    If [`pipeline_cache`] is not [`crate::Handle::null`], [`pipeline_cache`] **must**  be a valid [`PipelineCache`] handle
-  [`p_create_infos`] **must**  be a valid pointer to an array of [`create_info_count`] valid [`GraphicsPipelineCreateInfo`] structures
-    If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid [`AllocationCallbacks`] structure
-  [`p_pipelines`] **must**  be a valid pointer to an array of [`create_info_count`][`Pipeline`] handles
-  [`create_info_count`] **must**  be greater than `0`
-    If [`pipeline_cache`] is a valid handle, it  **must**  have been created, allocated, or retrieved from [`device`]

## Return Codes
*   - `VK_SUCCESS`  - `VK_PIPELINE_COMPILE_REQUIRED_EXT` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  - `VK_ERROR_INVALID_SHADER_NV`

# Related
- [`crate::vulkan1_0`]
- [`AllocationCallbacks`]
- [`Device`]
- [`GraphicsPipelineCreateInfo`]
- [`Pipeline`]
- [`PipelineCache`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        