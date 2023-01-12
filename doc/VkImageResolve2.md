[VkImageResolve2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageResolve2.html) - Structure specifying an image resolve operation

# C Specifications
The [`ImageResolve2`] structure is defined as:
```c
// Provided by VK_VERSION_1_3
typedef struct VkImageResolve2 {
    VkStructureType             sType;
    const void*                 pNext;
    VkImageSubresourceLayers    srcSubresource;
    VkOffset3D                  srcOffset;
    VkImageSubresourceLayers    dstSubresource;
    VkOffset3D                  dstOffset;
    VkExtent3D                  extent;
} VkImageResolve2;
```
or the equivalent
```c
// Provided by VK_KHR_copy_commands2
typedef VkImageResolve2 VkImageResolve2KHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`src_subresource`] and [`dst_subresource`] are [`ImageSubresourceLayers`] structures specifying the image subresources of the images used for the source and destination image data, respectively. Resolve of depth/stencil images is not supported.
- [`src_offset`] and [`dst_offset`] select the initial `x`, `y`, and `z` offsets in texels of the sub-regions of the source and destination image data.
- [`extent`] is the size in texels of the source image to resolve in `width`, `height` and `depth`.

# Description
## Valid Usage
-    The `aspectMask` member of [`src_subresource`] and [`dst_subresource`] **must**  only contain `VK_IMAGE_ASPECT_COLOR_BIT`
-    The `layerCount` member of [`src_subresource`] and [`dst_subresource`] **must**  match

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_IMAGE_RESOLVE_2`
-  [`p_next`] **must**  be `NULL`
-  [`src_subresource`] **must**  be a valid [`ImageSubresourceLayers`] structure
-  [`dst_subresource`] **must**  be a valid [`ImageSubresourceLayers`] structure

# Related
- [`khr_copy_commands2`]
- [`crate::vulkan1_3`]
- [`Extent3D`]
- [`ImageSubresourceLayers`]
- [`Offset3D`]
- [`ResolveImageInfo2`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        