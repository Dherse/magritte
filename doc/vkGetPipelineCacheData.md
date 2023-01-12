[vkGetPipelineCacheData](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPipelineCacheData.html) - Get the data store from a pipeline cache

# C Specifications
Data  **can**  be retrieved from a pipeline cache object using the command:
```c
// Provided by VK_VERSION_1_0
VkResult vkGetPipelineCacheData(
    VkDevice                                    device,
    VkPipelineCache                             pipelineCache,
    size_t*                                     pDataSize,
    void*                                       pData);
```

# Parameters
- [`device`] is the logical device that owns the pipeline cache.
- [`pipeline_cache`] is the pipeline cache to retrieve data from.
- [`p_data_size`] is a pointer to a `size_t` value related to the amount of data in the pipeline cache, as described below.
- [`p_data`] is either `NULL` or a pointer to a buffer.

# Description
If [`p_data`] is `NULL`, then the maximum size of the data that  **can**  be
retrieved from the pipeline cache, in bytes, is returned in [`p_data_size`].
Otherwise, [`p_data_size`] **must**  point to a variable set by the user to the
size of the buffer, in bytes, pointed to by [`p_data`], and on return the
variable is overwritten with the amount of data actually written to
[`p_data`].
If [`p_data_size`] is less than the maximum size that  **can**  be retrieved by
the pipeline cache, at most [`p_data_size`] bytes will be written to
[`p_data`], and `VK_INCOMPLETE` will be returned instead of
`VK_SUCCESS`, to indicate that not all of the pipeline cache was
returned.Any data written to [`p_data`] is valid and  **can**  be provided as the
`pInitialData` member of the [`PipelineCacheCreateInfo`] structure
passed to [`create_pipeline_cache`].Two calls to [`get_pipeline_cache_data`] with the same parameters  **must** 
retrieve the same data unless a command that modifies the contents of the
cache is called between them.The initial bytes written to [`p_data`] **must**  be a header as described in
the [Pipeline Cache Header](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#pipelines-cache-header) section.If [`p_data_size`] is less than what is necessary to store this header,
nothing will be written to [`p_data`] and zero will be written to
[`p_data_size`].
## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`pipeline_cache`] **must**  be a valid [`PipelineCache`] handle
-  [`p_data_size`] **must**  be a valid pointer to a `size_t` value
-    If the value referenced by [`p_data_size`] is not `0`, and [`p_data`] is not `NULL`, [`p_data`] **must**  be a valid pointer to an array of [`p_data_size`] bytes
-  [`pipeline_cache`] **must**  have been created, allocated, or retrieved from [`device`]

## Return Codes
*   - `VK_SUCCESS`  - `VK_INCOMPLETE` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`

# Related
- [`crate::vulkan1_0`]
- [`Device`]
- [`PipelineCache`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        