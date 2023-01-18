[VkQueryControlFlagBits](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueryControlFlagBits.html) - Bitmask specifying constraints on a query

# C Specifications
Bits which  **can**  be set in [`cmd_begin_query`]`::flags`, specifying
constraints on the types of queries that  **can**  be performed, are:
```c
// Provided by VK_VERSION_1_0
typedef enum VkQueryControlFlagBits {
    VK_QUERY_CONTROL_PRECISE_BIT = 0x00000001,
} VkQueryControlFlagBits;
```

# Description
- [`PRECISE`] specifies the precision of [occlusion queries](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#queries-occlusion).

# Related
- [`crate::vulkan1_0`]
- [`QueryControlFlags`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        