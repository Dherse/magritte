[VkImageStencilUsageCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageStencilUsageCreateInfo.html) - Specify separate usage flags for the stencil aspect of a depth-stencil image

# C Specifications
The [`ImageStencilUsageCreateInfo`] structure is defined as:
```c
// Provided by VK_VERSION_1_2
typedef struct VkImageStencilUsageCreateInfo {
    VkStructureType      sType;
    const void*          pNext;
    VkImageUsageFlags    stencilUsage;
} VkImageStencilUsageCreateInfo;
```
or the equivalent
```c
// Provided by VK_EXT_separate_stencil_usage
typedef VkImageStencilUsageCreateInfo VkImageStencilUsageCreateInfoEXT;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`stencil_usage`] is a bitmask of [`ImageUsageFlagBits`] describing the intended usage of the stencil aspect of the image.

# Description
If the [`p_next`] chain of [`ImageCreateInfo`] includes a
[`ImageStencilUsageCreateInfo`] structure, then that structure includes
the usage flags specific to the stencil aspect of the image for an image
with a depth-stencil format.This structure specifies image usages which only apply to the stencil aspect
of a depth/stencil format image.
When this structure is included in the [`p_next`] chain of
[`ImageCreateInfo`], the stencil aspect of the image  **must**  only be used
as specified by [`stencil_usage`].
When this structure is not included in the [`p_next`] chain of
[`ImageCreateInfo`], the stencil aspect of an image  **must**  only be used
as specified by [`ImageCreateInfo::usage`].
Use of other aspects of an image are unaffected by this structure.This structure  **can**  also be included in the [`p_next`] chain of
[`PhysicalDeviceImageFormatInfo2`] to query additional capabilities
specific to image creation parameter combinations including a separate set
of usage flags for the stencil aspect of the image using
[`get_physical_device_image_format_properties2`].
When this structure is not included in the [`p_next`] chain of
[`PhysicalDeviceImageFormatInfo2`] then the implicit value of
[`stencil_usage`] matches that of
[`PhysicalDeviceImageFormatInfo2::usage`].
## Valid Usage
-    If [`stencil_usage`] includes `VK_IMAGE_USAGE_TRANSIENT_ATTACHMENT_BIT`, it  **must**  not include bits other than `VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT` or `VK_IMAGE_USAGE_INPUT_ATTACHMENT_BIT`

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_IMAGE_STENCIL_USAGE_CREATE_INFO`
-  [`stencil_usage`] **must**  be a valid combination of [`ImageUsageFlagBits`] values
-  [`stencil_usage`] **must**  not be `0`

# Related
- [`VK_EXT_separate_stencil_usage`]
- [`crate::vulkan1_2`]
- [`ImageUsageFlags`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        