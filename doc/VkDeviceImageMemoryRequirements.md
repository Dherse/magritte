[VkDeviceImageMemoryRequirements](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceImageMemoryRequirements.html) - (None)

# C Specifications
The [`DeviceImageMemoryRequirements`] structure is defined as:
```c
// Provided by VK_VERSION_1_3
typedef struct VkDeviceImageMemoryRequirements {
    VkStructureType             sType;
    const void*                 pNext;
    const VkImageCreateInfo*    pCreateInfo;
    VkImageAspectFlagBits       planeAspect;
} VkDeviceImageMemoryRequirements;
```
or the equivalent
```c
// Provided by VK_KHR_maintenance4
typedef VkDeviceImageMemoryRequirements VkDeviceImageMemoryRequirementsKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`create_info`] is a pointer to a [`ImageCreateInfo`] structure containing parameters affecting creation of the image to query.
- [`plane_aspect`] is a [`ImageAspectFlagBits`] value specifying the aspect corresponding to the image plane to query. This parameter is ignored unless [`create_info`]`::tiling` is `VK_IMAGE_TILING_DRM_FORMAT_MODIFIER_EXT`, or [`create_info`]`::flags` has `VK_IMAGE_CREATE_DISJOINT_BIT` set.

# Description
## Valid Usage
-    The [`create_info`]::[`p_next`] chain  **must**  not contain a [`ImageSwapchainCreateInfoKHR`] structure
-    If [`create_info`]`::format` specifies a *multi-planar* format and [`create_info`]`::flags` has `VK_IMAGE_CREATE_DISJOINT_BIT` set then [`plane_aspect`] **must**  not be `VK_IMAGE_ASPECT_NONE_KHR`
-    If [`create_info`]`::flags` has `VK_IMAGE_CREATE_DISJOINT_BIT` set and if the [`create_info`]`::tiling` is `VK_IMAGE_TILING_LINEAR` or `VK_IMAGE_TILING_OPTIMAL`, then [`plane_aspect`] **must**  be a single valid *format plane* for the image (that is, for a two-plane image [`plane_aspect`] **must**  be `VK_IMAGE_ASPECT_PLANE_0_BIT` or `VK_IMAGE_ASPECT_PLANE_1_BIT`, and for a three-plane image [`plane_aspect`] **must**  be `VK_IMAGE_ASPECT_PLANE_0_BIT`, `VK_IMAGE_ASPECT_PLANE_1_BIT` or `VK_IMAGE_ASPECT_PLANE_2_BIT`)
-    If [`create_info`]`::tiling` is `VK_IMAGE_TILING_DRM_FORMAT_MODIFIER_EXT`, then [`plane_aspect`] **must**  be a single valid *memory plane* for the image (that is, `aspectMask` **must**  specify a plane index that is less than the [`DrmFormatModifierPropertiesEXT::drm_format_modifier_plane_count`] associated with the image’s `format` and [`ImageDrmFormatModifierPropertiesEXT::drm_format_modifier`])

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DEVICE_IMAGE_MEMORY_REQUIREMENTS`
-  [`p_next`] **must**  be `NULL`
-  [`create_info`] **must**  be a valid pointer to a valid [`ImageCreateInfo`] structure
-    If [`plane_aspect`] is not `0`, [`plane_aspect`] **must**  be a valid [`ImageAspectFlagBits`] value

# Related
- [`VK_KHR_maintenance4`]
- [`crate::vulkan1_3`]
- [`ImageAspectFlagBits`]
- [`ImageCreateInfo`]
- [`StructureType`]
- [`get_device_image_memory_requirements`]
- [`get_device_image_memory_requirements_khr`]
- [`get_device_image_sparse_memory_requirements`]
- [`get_device_image_sparse_memory_requirements_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        