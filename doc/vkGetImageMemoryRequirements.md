[vkGetImageMemoryRequirements](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetImageMemoryRequirements.html) - Returns the memory requirements for specified Vulkan object

# C Specifications
To determine the memory requirements for an image resource which is not
created with the `VK_IMAGE_CREATE_DISJOINT_BIT` flag set, call:
```c
// Provided by VK_VERSION_1_0
void vkGetImageMemoryRequirements(
    VkDevice                                    device,
    VkImage                                     image,
    VkMemoryRequirements*                       pMemoryRequirements);
```

# Parameters
- [`device`] is the logical device that owns the image.
- [`image`] is the image to query.
- [`p_memory_requirements`] is a pointer to a [`MemoryRequirements`] structure in which the memory requirements of the image object are returned.

# Description
## Valid Usage
-  [`image`] **must**  not have been created with the `VK_IMAGE_CREATE_DISJOINT_BIT` flag set
-    If [`image`] was created with the `VK_EXTERNAL_MEMORY_HANDLE_TYPE_ANDROID_HARDWARE_BUFFER_BIT_ANDROID` external memory handle type, then [`image`] **must**  be bound to memory

## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`image`] **must**  be a valid [`Image`] handle
-  [`p_memory_requirements`] **must**  be a valid pointer to a [`MemoryRequirements`] structure
-  [`image`] **must**  have been created, allocated, or retrieved from [`device`]

# Related
- [`crate::vulkan1_0`]
- [`Device`]
- [`Image`]
- [`MemoryRequirements`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        