[vkGetImageDrmFormatModifierPropertiesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetImageDrmFormatModifierPropertiesEXT.html) - Returns an image's DRM format modifier

# C Specifications
If an image was created with `VK_IMAGE_TILING_DRM_FORMAT_MODIFIER_EXT`,
then the image has a [Linux DRM format
modifier](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#glossary-drm-format-modifier).
To query the *modifier*, call:
```c
// Provided by VK_EXT_image_drm_format_modifier
VkResult vkGetImageDrmFormatModifierPropertiesEXT(
    VkDevice                                    device,
    VkImage                                     image,
    VkImageDrmFormatModifierPropertiesEXT*      pProperties);
```

# Parameters
- [`device`] is the logical device that owns the image.
- [`image`] is the queried image.
- [`p_properties`] is a pointer to a [`ImageDrmFormatModifierPropertiesEXT`] structure in which properties of the image’s *DRM format modifier* are returned.

# Description
## Valid Usage
-  [`image`] **must**  have been created with [`ImageCreateInfo`] equal to `VK_IMAGE_TILING_DRM_FORMAT_MODIFIER_EXT`

## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`image`] **must**  be a valid [`Image`] handle
-  [`p_properties`] **must**  be a valid pointer to a [`ImageDrmFormatModifierPropertiesEXT`] structure
-  [`image`] **must**  have been created, allocated, or retrieved from [`device`]

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`

# Related
- [`VK_EXT_image_drm_format_modifier`]
- [`Device`]
- [`Image`]
- [`ImageDrmFormatModifierPropertiesEXT`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        