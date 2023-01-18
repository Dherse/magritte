[VkPipelineCacheCreateFlagBits](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineCacheCreateFlagBits.html) - Bitmask specifying the behavior of the pipeline cache

# C Specifications
Bits which  **can**  be set in [`PipelineCacheCreateInfo::flags`],
specifying behavior of the pipeline cache, are:
```c
// Provided by VK_EXT_pipeline_creation_cache_control
typedef enum VkPipelineCacheCreateFlagBits {
  // Provided by VK_VERSION_1_3
    VK_PIPELINE_CACHE_CREATE_EXTERNALLY_SYNCHRONIZED_BIT = 0x00000001,
  // Provided by VK_EXT_pipeline_creation_cache_control
    VK_PIPELINE_CACHE_CREATE_EXTERNALLY_SYNCHRONIZED_BIT_EXT = VK_PIPELINE_CACHE_CREATE_EXTERNALLY_SYNCHRONIZED_BIT,
} VkPipelineCacheCreateFlagBits;
```

# Description
- [`EXTERNALLY_SYNCHRONIZED`] specifies that all commands that modify the created [`PipelineCache`] will be [externally synchronized](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#fundamentals-threadingbehavior). When set, the implementation  **may**  skip any unnecessary processing needed to support simultaneous modification from multiple threads where allowed.

# Related
- [`VK_EXT_pipeline_creation_cache_control`]
- [`PipelineCacheCreateFlags`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        