[vkGetBufferMemoryRequirements](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetBufferMemoryRequirements.html) - Returns the memory requirements for specified Vulkan object

# C Specifications
To determine the memory requirements for a buffer resource, call:
```c
// Provided by VK_VERSION_1_0
void vkGetBufferMemoryRequirements(
    VkDevice                                    device,
    VkBuffer                                    buffer,
    VkMemoryRequirements*                       pMemoryRequirements);
```

# Parameters
- [`device`] is the logical device that owns the buffer.
- [`buffer`] is the buffer to query.
- [`p_memory_requirements`] is a pointer to a [`MemoryRequirements`] structure in which the memory requirements of the buffer object are returned.

# Description
## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`buffer`] **must**  be a valid [`Buffer`] handle
-  [`p_memory_requirements`] **must**  be a valid pointer to a [`MemoryRequirements`] structure
-  [`buffer`] **must**  have been created, allocated, or retrieved from [`device`]

# Related
- [`crate::vulkan1_0`]
- [`Buffer`]
- [`Device`]
- [`MemoryRequirements`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        