[VkImageDrmFormatModifierPropertiesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageDrmFormatModifierPropertiesEXT.html) - Properties of an image's Linux DRM format modifier

# C Specifications
The [`ImageDrmFormatModifierPropertiesEXT`] structure is defined as:
```c
// Provided by VK_EXT_image_drm_format_modifier
typedef struct VkImageDrmFormatModifierPropertiesEXT {
    VkStructureType    sType;
    void*              pNext;
    uint64_t           drmFormatModifier;
} VkImageDrmFormatModifierPropertiesEXT;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`drm_format_modifier`] returns the image’s [Linux DRM format modifier](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#glossary-drm-format-modifier).

# Description
If the `image` was created with
[`ImageDrmFormatModifierListCreateInfoEXT`], then the returned
[`drm_format_modifier`] **must**  belong to the list of modifiers provided at
time of image creation in
[`ImageDrmFormatModifierListCreateInfoEXT::drm_format_modifiers`].
If the `image` was created with
[`ImageDrmFormatModifierExplicitCreateInfoEXT`], then the returned
[`drm_format_modifier`] **must**  be the modifier provided at time of image
creation in
[`ImageDrmFormatModifierExplicitCreateInfoEXT`]::[`drm_format_modifier`].
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_IMAGE_DRM_FORMAT_MODIFIER_PROPERTIES_EXT`
-  [`p_next`] **must**  be `NULL`

# Related
- [`VK_EXT_image_drm_format_modifier`]
- [`StructureType`]
- [`get_image_drm_format_modifier_properties_ext`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        