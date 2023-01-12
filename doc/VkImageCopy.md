[VkImageCopy](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageCopy.html) - Structure specifying an image copy operation

# C Specifications
The [`ImageCopy`] structure is defined as:
```c
// Provided by VK_VERSION_1_0
typedef struct VkImageCopy {
    VkImageSubresourceLayers    srcSubresource;
    VkOffset3D                  srcOffset;
    VkImageSubresourceLayers    dstSubresource;
    VkOffset3D                  dstOffset;
    VkExtent3D                  extent;
} VkImageCopy;
```

# Members
- [`src_subresource`] and [`dst_subresource`] are [`ImageSubresourceLayers`] structures specifying the image subresources of the images used for the source and destination image data, respectively.
- [`src_offset`] and [`dst_offset`] select the initial `x`, `y`, and `z` offsets in texels of the sub-regions of the source and destination image data.
- [`extent`] is the size in texels of the image to copy in `width`, `height` and `depth`.

# Description
For `VK_IMAGE_TYPE_3D` images, copies are performed slice by slice
starting with the `z` member of the [`src_offset`] or [`dst_offset`],
and copying `depth` slices.
For images with multiple layers, copies are performed layer by layer
starting with the `baseArrayLayer` member of the [`src_subresource`] or
[`dst_subresource`] and copying `layerCount` layers.
Image data  **can**  be copied between images with different image types.
If one image is `VK_IMAGE_TYPE_3D` and the other image is
`VK_IMAGE_TYPE_2D` with multiple layers, then each slice is copied to or
from a different layer.Copies involving a [multi-planar image format](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#formats-requiring-sampler-ycbcr-conversion) specify the region to be copied in terms of the
*plane* to be copied, not the coordinates of the multi-planar image.
This means that copies accessing the R/B planes of “`_422`” format
images  **must**  fit the copied region within half the `width` of the parent
image, and that copies accessing the R/B planes of “`_420`” format
images  **must**  fit the copied region within half the `width` and
`height` of the parent image.
## Valid Usage
-    The number of slices of the [`extent`] (for 3D) or layers of the [`src_subresource`] (for non-3D)  **must**  match the number of slices of the [`extent`] (for 3D) or layers of the [`dst_subresource`] (for non-3D)
-  `extent.width` **must**  not be 0
-  `extent.height` **must**  not be 0
-  `extent.depth` **must**  not be 0

## Valid Usage (Implicit)
-  [`src_subresource`] **must**  be a valid [`ImageSubresourceLayers`] structure
-  [`dst_subresource`] **must**  be a valid [`ImageSubresourceLayers`] structure

# Related
- [`crate::vulkan1_0`]
- [`Extent3D`]
- [`ImageSubresourceLayers`]
- [`Offset3D`]
- [`cmd_copy_image`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        