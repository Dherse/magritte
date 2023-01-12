[VkDrmFormatModifierPropertiesList2EXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDrmFormatModifierPropertiesList2EXT.html) - Structure specifying the list of DRM format modifiers supported for a format

# C Specifications
The list of [Linux DRM format modifiers](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#glossary-drm-format-modifier)
compatible with a [`Format`] **can**  be obtained by adding a
[`DrmFormatModifierPropertiesList2EXT`] structure to the [`p_next`]
chain of [`FormatProperties2`].The [`DrmFormatModifierPropertiesList2EXT`] structure is defined as:
```c
// Provided by VK_KHR_format_feature_flags2 with VK_EXT_image_drm_format_modifier
typedef struct VkDrmFormatModifierPropertiesList2EXT {
    VkStructureType                       sType;
    void*                                 pNext;
    uint32_t                              drmFormatModifierCount;
    VkDrmFormatModifierProperties2EXT*    pDrmFormatModifierProperties;
} VkDrmFormatModifierPropertiesList2EXT;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`drm_format_modifier_count`] is an inout parameter related to the number of modifiers compatible with the `format`, as described below.
- [`drm_format_modifier_properties`] is either `NULL` or a pointer to an array of [`DrmFormatModifierProperties2EXT`] structures.

# Description
If [`drm_format_modifier_properties`] is `NULL`, the number of modifiers
compatible with the queried `format` is returned in
[`drm_format_modifier_count`].
Otherwise, the application  **must**  set [`drm_format_modifier_count`] to the
length of the array [`drm_format_modifier_properties`]; the function will
write at most [`drm_format_modifier_count`] elements to the array, and will
return in [`drm_format_modifier_count`] the number of elements written.Among the elements in array [`drm_format_modifier_properties`], each
returned `drmFormatModifier` **must**  be unique.Among the elements in array [`drm_format_modifier_properties`], the bits
reported in `drmFormatModifierTilingFeatures` **must**  include the bits
reported in the corresponding element of
[`DrmFormatModifierPropertiesListEXT`]::[`drm_format_modifier_properties`].
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DRM_FORMAT_MODIFIER_PROPERTIES_LIST_2_EXT`

# Related
- [`ext_image_drm_format_modifier`]
- [`khr_format_feature_flags2`]
- [`DrmFormatModifierProperties2EXT`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        