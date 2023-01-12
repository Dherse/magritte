[VkRenderingFragmentDensityMapAttachmentInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRenderingFragmentDensityMapAttachmentInfoEXT.html) - Structure specifying fragment shading rate attachment information

# C Specifications
The [`RenderingFragmentDensityMapAttachmentInfoEXT`] structure is
defined as:
```c
// Provided by VK_KHR_dynamic_rendering with VK_EXT_fragment_density_map
typedef struct VkRenderingFragmentDensityMapAttachmentInfoEXT {
    VkStructureType    sType;
    const void*        pNext;
    VkImageView        imageView;
    VkImageLayout      imageLayout;
} VkRenderingFragmentDensityMapAttachmentInfoEXT;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`image_view`] is the image view that will be used as a fragment shading rate attachment.
- [`image_layout`] is the layout that [`image_view`] will be in during rendering.

# Description
This structure can be included in the [`p_next`] chain of
[`RenderingInfo`] to define a fragment density map.
If this structure is not included in the [`p_next`] chain, [`image_view`]
is treated as [`crate::Handle::null`].
## Valid Usage
-    If [`image_view`] is not [`crate::Handle::null`], `layout` **must**  be `VK_IMAGE_LAYOUT_GENERAL` or `VK_IMAGE_LAYOUT_FRAGMENT_DENSITY_MAP_OPTIMAL_EXT`
-    If [`image_view`] is not [`crate::Handle::null`], it  **must**  have been created with `VK_IMAGE_USAGE_FRAGMENT_DENSITY_MAP_BIT_EXT`
-    If [`image_view`] is not [`crate::Handle::null`], it  **must**  not have been created with `VK_IMAGE_CREATE_SUBSAMPLED_BIT_EXT`

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_INFO_EXT`
-  [`image_view`] **must**  be a valid [`ImageView`] handle
-  [`image_layout`] **must**  be a valid [`ImageLayout`] value

# Related
- [`ext_fragment_density_map`]
- [`khr_dynamic_rendering`]
- [`ImageLayout`]
- [`ImageView`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        