[VkImageCopy2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageCopy2.html) - Structure specifying an image copy operation

# C Specifications
The [`ImageCopy2`] structure is defined as:
```c
// Provided by VK_VERSION_1_3
typedef struct VkImageCopy2 {
    VkStructureType             sType;
    const void*                 pNext;
    VkImageSubresourceLayers    srcSubresource;
    VkOffset3D                  srcOffset;
    VkImageSubresourceLayers    dstSubresource;
    VkOffset3D                  dstOffset;
    VkExtent3D                  extent;
} VkImageCopy2;
```
or the equivalent
```c
// Provided by VK_KHR_copy_commands2
typedef VkImageCopy2 VkImageCopy2KHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`src_subresource`] and [`dst_subresource`] are [`ImageSubresourceLayers`] structures specifying the image subresources of the images used for the source and destination image data, respectively.
- [`src_offset`] and [`dst_offset`] select the initial `x`, `y`, and `z` offsets in texels of the sub-regions of the source and destination image data.
- [`extent`] is the size in texels of the image to copy in `width`, `height` and `depth`.

# Description
## Valid Usage
-    The number of slices of the [`extent`] (for 3D) or layers of the [`src_subresource`] (for non-3D)  **must**  match the number of slices of the [`extent`] (for 3D) or layers of the [`dst_subresource`] (for non-3D)
-  `extent.width` **must**  not be 0
-  `extent.height` **must**  not be 0
-  `extent.depth` **must**  not be 0

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_IMAGE_COPY_2`
-  [`p_next`] **must**  be `NULL`
-  [`src_subresource`] **must**  be a valid [`ImageSubresourceLayers`] structure
-  [`dst_subresource`] **must**  be a valid [`ImageSubresourceLayers`] structure

# Related
- [`VK_KHR_copy_commands2`]
- [`crate::vulkan1_3`]
- [`CopyImageInfo2`]
- [`Extent3D`]
- [`ImageSubresourceLayers`]
- [`Offset3D`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        