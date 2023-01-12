[VkImagePlaneMemoryRequirementsInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImagePlaneMemoryRequirementsInfo.html) - Structure specifying image plane for memory requirements

# C Specifications
To determine the memory requirements for a plane of a disjoint image, add a
[`ImagePlaneMemoryRequirementsInfo`] structure to the [`p_next`] chain
of the [`ImageMemoryRequirementsInfo2`] structure.The [`ImagePlaneMemoryRequirementsInfo`] structure is defined as:
```c
// Provided by VK_VERSION_1_1
typedef struct VkImagePlaneMemoryRequirementsInfo {
    VkStructureType          sType;
    const void*              pNext;
    VkImageAspectFlagBits    planeAspect;
} VkImagePlaneMemoryRequirementsInfo;
```
or the equivalent
```c
// Provided by VK_KHR_sampler_ycbcr_conversion
typedef VkImagePlaneMemoryRequirementsInfo VkImagePlaneMemoryRequirementsInfoKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`plane_aspect`] is a [`ImageAspectFlagBits`] value specifying the aspect corresponding to the image plane to query.

# Description
## Valid Usage
-    If the image’s `tiling` is `VK_IMAGE_TILING_LINEAR` or `VK_IMAGE_TILING_OPTIMAL`, then [`plane_aspect`] **must**  be a single valid *format plane* for the image (that is, for a two-plane image [`plane_aspect`] **must**  be `VK_IMAGE_ASPECT_PLANE_0_BIT` or `VK_IMAGE_ASPECT_PLANE_1_BIT`, and for a three-plane image [`plane_aspect`] **must**  be `VK_IMAGE_ASPECT_PLANE_0_BIT`, `VK_IMAGE_ASPECT_PLANE_1_BIT` or `VK_IMAGE_ASPECT_PLANE_2_BIT`)
-    If the image’s `tiling` is `VK_IMAGE_TILING_DRM_FORMAT_MODIFIER_EXT`, then [`plane_aspect`] **must**  be a single valid *memory plane* for the image (that is, `aspectMask` **must**  specify a plane index that is less than the [`DrmFormatModifierPropertiesEXT::drm_format_modifier_plane_count`] associated with the image’s `format` and [`ImageDrmFormatModifierPropertiesEXT::drm_format_modifier`])

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO`
-  [`plane_aspect`] **must**  be a valid [`ImageAspectFlagBits`] value

# Related
- [`crate::vulkan1_1`]
- [`ImageAspectFlagBits`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        