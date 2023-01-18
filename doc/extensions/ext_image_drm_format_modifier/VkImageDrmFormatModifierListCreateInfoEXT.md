[VkImageDrmFormatModifierListCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageDrmFormatModifierListCreateInfoEXT.html) - Specify that an image must be created with a DRM format modifier from the provided list

# C Specifications
If the [`p_next`] chain of [`ImageCreateInfo`] includes a
[`ImageDrmFormatModifierListCreateInfoEXT`] structure, then the image
will be created with one of the [Linux DRM
format modifiers](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#glossary-drm-format-modifier) listed in the structure.
The choice of modifier is implementation-dependent.The [`ImageDrmFormatModifierListCreateInfoEXT`] structure is defined as:
```c
// Provided by VK_EXT_image_drm_format_modifier
typedef struct VkImageDrmFormatModifierListCreateInfoEXT {
    VkStructureType    sType;
    const void*        pNext;
    uint32_t           drmFormatModifierCount;
    const uint64_t*    pDrmFormatModifiers;
} VkImageDrmFormatModifierListCreateInfoEXT;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`drm_format_modifier_count`] is the length of the [`drm_format_modifiers`] array.
- [`drm_format_modifiers`] is a pointer to an array of *Linux DRM format modifiers*.

# Description
## Valid Usage
-    Each *modifier* in [`drm_format_modifiers`] **must**  be compatible with the parameters in [`ImageCreateInfo`] and its [`p_next`] chain, as determined by querying [`PhysicalDeviceImageFormatInfo2`] extended with [`PhysicalDeviceImageDrmFormatModifierInfoEXT`]

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_IMAGE_DRM_FORMAT_MODIFIER_LIST_CREATE_INFO_EXT`
-  [`drm_format_modifiers`] **must**  be a valid pointer to an array of [`drm_format_modifier_count`]`uint64_t` values
-  [`drm_format_modifier_count`] **must**  be greater than `0`

# Related
- [`VK_EXT_image_drm_format_modifier`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        