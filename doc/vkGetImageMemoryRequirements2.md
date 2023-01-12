[vkGetImageMemoryRequirements2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetImageMemoryRequirements2.html) - Returns the memory requirements for specified Vulkan object

# C Specifications
To determine the memory requirements for an image resource, call:
```c
// Provided by VK_VERSION_1_1
void vkGetImageMemoryRequirements2(
    VkDevice                                    device,
    const VkImageMemoryRequirementsInfo2*       pInfo,
    VkMemoryRequirements2*                      pMemoryRequirements);
```
or the equivalent command
```c
// Provided by VK_KHR_get_memory_requirements2
void vkGetImageMemoryRequirements2KHR(
    VkDevice                                    device,
    const VkImageMemoryRequirementsInfo2*       pInfo,
    VkMemoryRequirements2*                      pMemoryRequirements);
```

# Parameters
- [`device`] is the logical device that owns the image.
- [`p_info`] is a pointer to a [`ImageMemoryRequirementsInfo2`] structure containing parameters required for the memory requirements query.
- [`p_memory_requirements`] is a pointer to a [`MemoryRequirements2`] structure in which the memory requirements of the image object are returned.

# Description
## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`p_info`] **must**  be a valid pointer to a valid [`ImageMemoryRequirementsInfo2`] structure
-  [`p_memory_requirements`] **must**  be a valid pointer to a [`MemoryRequirements2`] structure

# Related
- [`crate::vulkan1_1`]
- [`Device`]
- [`ImageMemoryRequirementsInfo2`]
- [`MemoryRequirements2`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        