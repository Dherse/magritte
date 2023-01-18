[VkDrmFormatModifierProperties2EXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDrmFormatModifierProperties2EXT.html) - Structure specifying properties of a format when combined with a DRM format modifier

# C Specifications
The [`DrmFormatModifierProperties2EXT`] structure describes properties
of a [`Format`] when that format is combined with a
[Linux DRM format modifier](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#glossary-drm-format-modifier).
These properties, like those of [`FormatProperties2`], are independent
of any particular image.The [`DrmFormatModifierPropertiesEXT`] structure is defined as:
```c
// Provided by VK_KHR_format_feature_flags2 with VK_EXT_image_drm_format_modifier
typedef struct VkDrmFormatModifierProperties2EXT {
    uint64_t                 drmFormatModifier;
    uint32_t                 drmFormatModifierPlaneCount;
    VkFormatFeatureFlags2    drmFormatModifierTilingFeatures;
} VkDrmFormatModifierProperties2EXT;
```

# Members
- [`drm_format_modifier`] is a *Linux DRM format modifier*.
- [`drm_format_modifier_plane_count`] is the number of *memory planes* in any image created with `format` and [`drm_format_modifier`]. An image’s *memory planecount* is distinct from its *format planecount*, as explained below.
- [`drm_format_modifier_tiling_features`] is a bitmask of [`FormatFeatureFlagBits2`] that are supported by any image created with `format` and [`drm_format_modifier`].

# Related
- [`VK_EXT_image_drm_format_modifier`]
- [`VK_KHR_format_feature_flags2`]
- [`DrmFormatModifierPropertiesList2EXT`]
- [`FormatFeatureFlags2`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        