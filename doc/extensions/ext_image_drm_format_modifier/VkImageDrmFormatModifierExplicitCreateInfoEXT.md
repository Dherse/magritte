[VkImageDrmFormatModifierExplicitCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageDrmFormatModifierExplicitCreateInfoEXT.html) - Specify that an image be created with the provided DRM format modifier and explicit memory layout

# C Specifications
If the [`p_next`] chain of [`ImageCreateInfo`] includes a
[`ImageDrmFormatModifierExplicitCreateInfoEXT`] structure, then the
image will be created with the [Linux DRM
format modifier](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#glossary-drm-format-modifier) and memory layout defined by the structure.The [`ImageDrmFormatModifierExplicitCreateInfoEXT`] structure is defined
as:
```c
// Provided by VK_EXT_image_drm_format_modifier
typedef struct VkImageDrmFormatModifierExplicitCreateInfoEXT {
    VkStructureType               sType;
    const void*                   pNext;
    uint64_t                      drmFormatModifier;
    uint32_t                      drmFormatModifierPlaneCount;
    const VkSubresourceLayout*    pPlaneLayouts;
} VkImageDrmFormatModifierExplicitCreateInfoEXT;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`drm_format_modifier`] is the *Linux DRM format modifier* with which the image will be created.
- [`drm_format_modifier_plane_count`] is the number of *memory planes* in the image (as reported by [`DrmFormatModifierPropertiesEXT`]) as well as the length of the [`plane_layouts`] array.
- [`plane_layouts`] is a pointer to an array of [`SubresourceLayout`] structures describing the image’s *memory planes*.

# Description
The `i`<sup>th</sup> member of [`plane_layouts`] describes the layout of the
image’s `i`<sup>th</sup>*memory plane* (that is,
`VK_IMAGE_ASPECT_MEMORY_PLANE*_i_*BIT_EXT`).
In each element of [`plane_layouts`], the implementation  **must**  ignore
`size`.
The implementation calculates the size of each plane, which the application
 **can**  query with [`get_image_subresource_layout`].When creating an image with
[`ImageDrmFormatModifierExplicitCreateInfoEXT`], it is the application’s
responsibility to satisfy all valid usage requirements.
However, the implementation  **must**  validate that the provided
[`plane_layouts`], when combined with the provided [`drm_format_modifier`]
and other creation parameters in [`ImageCreateInfo`] and its [`p_next`]
chain, produce a valid image.
(This validation is necessarily implementation-dependent and outside the
scope of Vulkan, and therefore not described by valid usage requirements).
If this validation fails, then [`create_image`] returns
`VK_ERROR_INVALID_DRM_FORMAT_MODIFIER_PLANE_LAYOUT_EXT`.
## Valid Usage
-  [`drm_format_modifier`] **must**  be compatible with the parameters in [`ImageCreateInfo`] and its [`p_next`] chain, as determined by querying [`PhysicalDeviceImageFormatInfo2`] extended with [`PhysicalDeviceImageDrmFormatModifierInfoEXT`]
-  [`drm_format_modifier_plane_count`] **must**  be equal to the [`DrmFormatModifierPropertiesEXT`]::[`drm_format_modifier_plane_count`] associated with [`ImageCreateInfo::format`] and [`drm_format_modifier`], as found by querying [`DrmFormatModifierPropertiesListEXT`]
-    For each element of [`plane_layouts`], `size` **must**  be 0
-    For each element of [`plane_layouts`], `arrayPitch` **must**  be 0 if [`ImageCreateInfo::array_layers`] is 1
-    For each element of [`plane_layouts`], `depthPitch` **must**  be 0 if [`ImageCreateInfo`]::`extent.depth` is 1

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_IMAGE_DRM_FORMAT_MODIFIER_EXPLICIT_CREATE_INFO_EXT`
-    If [`drm_format_modifier_plane_count`] is not `0`, [`plane_layouts`] **must**  be a valid pointer to an array of [`drm_format_modifier_plane_count`][`SubresourceLayout`] structures

# Related
- [`ext_image_drm_format_modifier`]
- [`StructureType`]
- [`SubresourceLayout`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        