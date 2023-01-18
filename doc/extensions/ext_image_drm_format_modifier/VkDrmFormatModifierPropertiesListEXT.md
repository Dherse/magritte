[VkDrmFormatModifierPropertiesListEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDrmFormatModifierPropertiesListEXT.html) - Structure specifying the list of DRM format modifiers supported for a format

# C Specifications
To obtain the list of [Linux DRM format
modifiers](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#glossary-drm-format-modifier) compatible with a [`Format`], add a
[`DrmFormatModifierPropertiesListEXT`] structure to the [`p_next`]
chain of [`FormatProperties2`].The [`DrmFormatModifierPropertiesListEXT`] structure is defined as:
```c
// Provided by VK_EXT_image_drm_format_modifier
typedef struct VkDrmFormatModifierPropertiesListEXT {
    VkStructureType                      sType;
    void*                                pNext;
    uint32_t                             drmFormatModifierCount;
    VkDrmFormatModifierPropertiesEXT*    pDrmFormatModifierProperties;
} VkDrmFormatModifierPropertiesListEXT;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`drm_format_modifier_count`] is an inout parameter related to the number of modifiers compatible with the `format`, as described below.
- [`drm_format_modifier_properties`] is either `NULL` or a pointer to an array of [`DrmFormatModifierPropertiesEXT`] structures.

# Description
If [`drm_format_modifier_properties`] is `NULL`, then the function returns
in [`drm_format_modifier_count`] the number of modifiers compatible with the
queried `format`.
Otherwise, the application  **must**  set [`drm_format_modifier_count`] to the
length of the array [`drm_format_modifier_properties`]; the function will
write at most [`drm_format_modifier_count`] elements to the array, and will
return in [`drm_format_modifier_count`] the number of elements written.Among the elements in array [`drm_format_modifier_properties`], each
returned `drmFormatModifier` **must**  be unique.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DRM_FORMAT_MODIFIER_PROPERTIES_LIST_EXT`

# Related
- [`VK_EXT_image_drm_format_modifier`]
- [`DrmFormatModifierPropertiesEXT`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        