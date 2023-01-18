[VkImageFormatListCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageFormatListCreateInfo.html) - Specify that an image can: be used with a particular set of formats

# C Specifications
If the [`p_next`] chain of [`ImageCreateInfo`] includes a
[`ImageFormatListCreateInfo`] structure, then that structure contains a
list of all formats that  **can**  be used when creating views of this image.The [`ImageFormatListCreateInfo`] structure is defined as:
```c
// Provided by VK_VERSION_1_2
typedef struct VkImageFormatListCreateInfo {
    VkStructureType    sType;
    const void*        pNext;
    uint32_t           viewFormatCount;
    const VkFormat*    pViewFormats;
} VkImageFormatListCreateInfo;
```
or the equivalent
```c
// Provided by VK_KHR_image_format_list
typedef VkImageFormatListCreateInfo VkImageFormatListCreateInfoKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`view_format_count`] is the number of entries in the [`view_formats`] array.
- [`view_formats`] is a pointer to an array of [`Format`] values specifying all formats which  **can**  be used when creating views of this image.

# Description
If [`view_format_count`] is zero, [`view_formats`] is ignored and the
image is created as if the [`ImageFormatListCreateInfo`] structure were
not included in the [`p_next`] chain of [`ImageCreateInfo`].
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_IMAGE_FORMAT_LIST_CREATE_INFO`
-    If [`view_format_count`] is not `0`, [`view_formats`] **must**  be a valid pointer to an array of [`view_format_count`] valid [`Format`] values

# Related
- [`VK_KHR_image_format_list`]
- [`crate::vulkan1_2`]
- [`Format`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        