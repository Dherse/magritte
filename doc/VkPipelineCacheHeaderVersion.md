[VkPipelineCacheHeaderVersion](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineCacheHeaderVersion.html) - Encode pipeline cache version

# C Specifications
Possible values of the `headerVersion` value of the pipeline cache
header are:
```c
// Provided by VK_VERSION_1_0
typedef enum VkPipelineCacheHeaderVersion {
    VK_PIPELINE_CACHE_HEADER_VERSION_ONE = 1,
} VkPipelineCacheHeaderVersion;
```

# Description
- [`ONE`] specifies version one of the pipeline cache.

# Related
- [`crate::vulkan1_0`]
- [`PipelineCacheHeaderVersionOne`]
- [`create_pipeline_cache`]
- [`get_pipeline_cache_data`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        