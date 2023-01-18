[VkPhysicalDeviceImageDrmFormatModifierInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceImageDrmFormatModifierInfoEXT.html) - Structure specifying a DRM format modifier as image creation parameter

# C Specifications
To query the image capabilities that are compatible with a
[Linux DRM format modifier](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#glossary-drm-format-modifier), set
[`PhysicalDeviceImageFormatInfo2::tiling`] to
`VK_IMAGE_TILING_DRM_FORMAT_MODIFIER_EXT` and add a
[`PhysicalDeviceImageDrmFormatModifierInfoEXT`] structure to the
[`p_next`] chain of [`PhysicalDeviceImageFormatInfo2`].The [`PhysicalDeviceImageDrmFormatModifierInfoEXT`] structure is defined
as:
```c
// Provided by VK_EXT_image_drm_format_modifier
typedef struct VkPhysicalDeviceImageDrmFormatModifierInfoEXT {
    VkStructureType    sType;
    const void*        pNext;
    uint64_t           drmFormatModifier;
    VkSharingMode      sharingMode;
    uint32_t           queueFamilyIndexCount;
    const uint32_t*    pQueueFamilyIndices;
} VkPhysicalDeviceImageDrmFormatModifierInfoEXT;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`drm_format_modifier`] is the image’s *Linux DRM format modifier*, corresponding to [`ImageDrmFormatModifierExplicitCreateInfoEXT`]`::modifier` or to [`ImageDrmFormatModifierListCreateInfoEXT`]`::pModifiers`.
- [`sharing_mode`] specifies how the image will be accessed by multiple queue families.
- [`queue_family_index_count`] is the number of entries in the [`queue_family_indices`] array.
- [`queue_family_indices`] is a pointer to an array of queue families that will access the image. It is ignored if [`sharing_mode`] is not `VK_SHARING_MODE_CONCURRENT`.

# Description
If the [`drm_format_modifier`] is incompatible with the parameters specified
in [`PhysicalDeviceImageFormatInfo2`] and its [`p_next`] chain, then
[`get_physical_device_image_format_properties2`] returns
`VK_ERROR_FORMAT_NOT_SUPPORTED`.
The implementation  **must**  support the query of any [`drm_format_modifier`],
including unknown and invalid modifier values.
## Valid Usage
-    If [`sharing_mode`] is `VK_SHARING_MODE_CONCURRENT`, then [`queue_family_indices`] **must**  be a valid pointer to an array of [`queue_family_index_count`]`uint32_t` values
-    If [`sharing_mode`] is `VK_SHARING_MODE_CONCURRENT`, then [`queue_family_index_count`] **must**  be greater than `1`
-    If [`sharing_mode`] is `VK_SHARING_MODE_CONCURRENT`, each element of [`queue_family_indices`] **must**  be unique and  **must**  be less than the `pQueueFamilyPropertyCount` returned by [`get_physical_device_queue_family_properties2`] for the `physicalDevice` that was used to create `device`

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_DRM_FORMAT_MODIFIER_INFO_EXT`
-  [`sharing_mode`] **must**  be a valid [`SharingMode`] value

# Related
- [`VK_EXT_image_drm_format_modifier`]
- [`SharingMode`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        