[VkImageMemoryRequirementsInfo2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageMemoryRequirementsInfo2.html) - (None)

# C Specifications
The [`ImageMemoryRequirementsInfo2`] structure is defined as:
```c
// Provided by VK_VERSION_1_1
typedef struct VkImageMemoryRequirementsInfo2 {
    VkStructureType    sType;
    const void*        pNext;
    VkImage            image;
} VkImageMemoryRequirementsInfo2;
```
or the equivalent
```c
// Provided by VK_KHR_get_memory_requirements2
typedef VkImageMemoryRequirementsInfo2 VkImageMemoryRequirementsInfo2KHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`image`] is the image to query.

# Description
## Valid Usage
-    If [`image`] was created with a *multi-planar* format and the `VK_IMAGE_CREATE_DISJOINT_BIT` flag, there  **must**  be a [`ImagePlaneMemoryRequirementsInfo`] included in the [`p_next`] chain of the [`ImageMemoryRequirementsInfo2`] structure
-    If [`image`] was created with `VK_IMAGE_CREATE_DISJOINT_BIT` and with `VK_IMAGE_TILING_DRM_FORMAT_MODIFIER_EXT`, then there  **must**  be a [`ImagePlaneMemoryRequirementsInfo`] included in the [`p_next`] chain of the [`ImageMemoryRequirementsInfo2`] structure
-    If [`image`] was not created with the `VK_IMAGE_CREATE_DISJOINT_BIT` flag, there  **must**  not be a [`ImagePlaneMemoryRequirementsInfo`] included in the [`p_next`] chain of the [`ImageMemoryRequirementsInfo2`] structure
-    If [`image`] was created with a single-plane format and with any `tiling` other than `VK_IMAGE_TILING_DRM_FORMAT_MODIFIER_EXT`, then there  **must**  not be a [`ImagePlaneMemoryRequirementsInfo`] included in the [`p_next`] chain of the [`ImageMemoryRequirementsInfo2`] structure
-    If [`image`] was created with the `VK_EXTERNAL_MEMORY_HANDLE_TYPE_ANDROID_HARDWARE_BUFFER_BIT_ANDROID` external memory handle type, then [`image`] **must**  be bound to memory

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_IMAGE_MEMORY_REQUIREMENTS_INFO_2`
-  [`p_next`] **must**  be `NULL` or a pointer to a valid instance of [`ImagePlaneMemoryRequirementsInfo`]
-    The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique
-  [`image`] **must**  be a valid [`Image`] handle

# Related
- [`crate::vulkan1_1`]
- [`Image`]
- [`StructureType`]
- [`get_image_memory_requirements2`]
- [`get_image_memory_requirements2_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        