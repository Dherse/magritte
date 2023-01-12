[VkCullModeFlagBits](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCullModeFlagBits.html) - Bitmask controlling triangle culling

# C Specifications
Once the orientation of triangles is determined, they are culled according
to the [`PipelineRasterizationStateCreateInfo::cull_mode`] property
of the currently active pipeline.
Possible values are:
```c
// Provided by VK_VERSION_1_0
typedef enum VkCullModeFlagBits {
    VK_CULL_MODE_NONE = 0,
    VK_CULL_MODE_FRONT_BIT = 0x00000001,
    VK_CULL_MODE_BACK_BIT = 0x00000002,
    VK_CULL_MODE_FRONT_AND_BACK = 0x00000003,
} VkCullModeFlagBits;
```

# Description
- [`VK_CULL_MODE_FLAG_BITS`] specifies that no triangles are discarded
- [`VK_CULL_MODE_FLAG_BITS`] specifies that front-facing triangles are discarded
- [`VK_CULL_MODE_FLAG_BITS`] specifies that back-facing triangles are discarded
- [`VK_CULL_MODE_FLAG_BITS`] specifies that all triangles are discarded.
Following culling, fragments are produced for any triangles which have not
been discarded.

# Related
- [`crate::vulkan1_0`]
- [VkCullModeFlags]()

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        