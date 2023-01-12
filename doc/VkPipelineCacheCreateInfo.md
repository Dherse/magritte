[VkPipelineCacheCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineCacheCreateInfo.html) - Structure specifying parameters of a newly created pipeline cache

# C Specifications
The [`PipelineCacheCreateInfo`] structure is defined as:
```c
// Provided by VK_VERSION_1_0
typedef struct VkPipelineCacheCreateInfo {
    VkStructureType               sType;
    const void*                   pNext;
    VkPipelineCacheCreateFlags    flags;
    size_t                        initialDataSize;
    const void*                   pInitialData;
} VkPipelineCacheCreateInfo;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`flags`] is a bitmask of [`PipelineCacheCreateFlagBits`] specifying the behavior of the pipeline cache.
- [`initial_data_size`] is the number of bytes in [`initial_data`]. If [`initial_data_size`] is zero, the pipeline cache will initially be empty.
- [`initial_data`] is a pointer to previously retrieved pipeline cache data. If the pipeline cache data is incompatible (as defined below) with the device, the pipeline cache will be initially empty. If [`initial_data_size`] is zero, [`initial_data`] is ignored.

# Description
## Valid Usage
-    If [`initial_data_size`] is not `0`, it  **must**  be equal to the size of [`initial_data`], as returned by [`get_pipeline_cache_data`] when [`initial_data`] was originally retrieved
-    If [`initial_data_size`] is not `0`, [`initial_data`] **must**  have been retrieved from a previous call to [`get_pipeline_cache_data`]
-    If the [`pipelineCreationCacheControl`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-pipelineCreationCacheControl) feature is not enabled, [`flags`] **must**  not include `VK_PIPELINE_CACHE_CREATE_EXTERNALLY_SYNCHRONIZED_BIT`

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PIPELINE_CACHE_CREATE_INFO`
-  [`p_next`] **must**  be `NULL`
-  [`flags`] **must**  be a valid combination of [`PipelineCacheCreateFlagBits`] values
-    If [`initial_data_size`] is not `0`, [`initial_data`] **must**  be a valid pointer to an array of [`initial_data_size`] bytes

# Related
- [`crate::vulkan1_0`]
- [VkPipelineCacheCreateFlags]()
- [`StructureType`]
- [`create_pipeline_cache`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        