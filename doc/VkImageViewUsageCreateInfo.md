[VkImageViewUsageCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageViewUsageCreateInfo.html) - Specify the intended usage of an image view

# C Specifications
The set of usages for the created image view  **can**  be restricted compared to
the parent image’s [`usage`] flags by adding a
[`ImageViewUsageCreateInfo`] structure to the [`p_next`] chain of
[`ImageViewCreateInfo`].The [`ImageViewUsageCreateInfo`] structure is defined as:
```c
// Provided by VK_VERSION_1_1
typedef struct VkImageViewUsageCreateInfo {
    VkStructureType      sType;
    const void*          pNext;
    VkImageUsageFlags    usage;
} VkImageViewUsageCreateInfo;
```
or the equivalent
```c
// Provided by VK_KHR_maintenance2
typedef VkImageViewUsageCreateInfo VkImageViewUsageCreateInfoKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`usage`] is a bitmask of [`ImageUsageFlagBits`] specifying allowed usages of the image view.

# Description
When this structure is chained to [`ImageViewCreateInfo`] the
[`usage`] field overrides the implicit [`usage`] parameter inherited
from image creation time and its value is used instead for the purposes of
determining the valid usage conditions of [`ImageViewCreateInfo`].
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_IMAGE_VIEW_USAGE_CREATE_INFO`
-  [`usage`] **must**  be a valid combination of [`ImageUsageFlagBits`] values
-  [`usage`] **must**  not be `0`

# Related
- [`crate::vulkan1_1`]
- [`ImageUsageFlags`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        