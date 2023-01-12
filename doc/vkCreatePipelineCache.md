[vkCreatePipelineCache](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreatePipelineCache.html) - Creates a new pipeline cache

# C Specifications
To create pipeline cache objects, call:
```c
// Provided by VK_VERSION_1_0
VkResult vkCreatePipelineCache(
    VkDevice                                    device,
    const VkPipelineCacheCreateInfo*            pCreateInfo,
    const VkAllocationCallbacks*                pAllocator,
    VkPipelineCache*                            pPipelineCache);
```

# Parameters
- [`device`] is the logical device that creates the pipeline cache object.
- [`p_create_info`] is a pointer to a [`PipelineCacheCreateInfo`] structure containing initial parameters for the pipeline cache object.
- [`p_allocator`] controls host memory allocation as described in the [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation) chapter.
- [`p_pipeline_cache`] is a pointer to a [`PipelineCache`] handle in which the resulting pipeline cache object is returned.

# Description
Once created, a pipeline cache  **can**  be passed to the
[`create_graphics_pipelines`][`create_ray_tracing_pipelines_khr`],
[`create_ray_tracing_pipelines_nv`],
and [`create_compute_pipelines`] commands.
If the pipeline cache passed into these commands is not
[`crate::Handle::null`], the implementation will query it for possible reuse
opportunities and update it with new content.
The use of the pipeline cache object in these commands is internally
synchronized, and the same pipeline cache object  **can**  be used in multiple
threads simultaneously.If `flags` of [`p_create_info`] includes
`VK_PIPELINE_CACHE_CREATE_EXTERNALLY_SYNCHRONIZED_BIT`, all commands
that modify the returned pipeline cache object  **must**  be
[externally synchronized](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#fundamentals-threadingbehavior).
## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`p_create_info`] **must**  be a valid pointer to a valid [`PipelineCacheCreateInfo`] structure
-    If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid [`AllocationCallbacks`] structure
-  [`p_pipeline_cache`] **must**  be a valid pointer to a [`PipelineCache`] handle

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`

# Related
- [`crate::vulkan1_0`]
- [`AllocationCallbacks`]
- [`Device`]
- [`PipelineCache`]
- [`PipelineCacheCreateInfo`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        