[VkImageViewMinLodCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageViewMinLodCreateInfoEXT.html) - Structure describing the minimum lod of an image view

# C Specifications
If the [`p_next`] chain includes a [`ImageViewMinLodCreateInfoEXT`]
structure, then that structure includes a parameter specifying a value to
clamp the minimum LOD value during [Image
Level(s) Selection](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#textures-image-level-selection) and [Integer
Texel Coordinate Operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#textures-integer-coordinate-operations).The [`ImageViewMinLodCreateInfoEXT`] structure is defined as:
```c
// Provided by VK_EXT_image_view_min_lod
typedef struct VkImageViewMinLodCreateInfoEXT {
    VkStructureType    sType;
    const void*        pNext;
    float              minLod;
} VkImageViewMinLodCreateInfoEXT;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`min_lod`] is the value to clamp the minimum LOD accessible by this [`ImageView`].

# Description
## Valid Usage
-    If the [[`min_lod`]](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-minLod) feature is not enabled, [`min_lod`] **must**  be `0.0`.
-  [`min_lod`] **must**  be less or equal to the index of the last mipmap level accessible to the view.

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_IMAGE_VIEW_MIN_LOD_CREATE_INFO_EXT`

# Related
- [`ext_image_view_min_lod`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        