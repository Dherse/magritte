[VkBufferImageCopy](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBufferImageCopy.html) - Structure specifying a buffer image copy operation

# C Specifications
For both [`cmd_copy_buffer_to_image`] and [`cmd_copy_image_to_buffer`], each
element of `pRegions` is a structure defined as:
```c
// Provided by VK_VERSION_1_0
typedef struct VkBufferImageCopy {
    VkDeviceSize                bufferOffset;
    uint32_t                    bufferRowLength;
    uint32_t                    bufferImageHeight;
    VkImageSubresourceLayers    imageSubresource;
    VkOffset3D                  imageOffset;
    VkExtent3D                  imageExtent;
} VkBufferImageCopy;
```

# Members
- [`buffer_offset`] is the offset in bytes from the start of the buffer object where the image data is copied from or to.
- [`buffer_row_length`] and [`buffer_image_height`] specify in texels a subregion of a larger two- or three-dimensional image in buffer memory, and control the addressing calculations. If either of these values is zero, that aspect of the buffer memory is considered to be tightly packed according to the [`image_extent`].
- [`image_subresource`] is a [`ImageSubresourceLayers`] used to specify the specific image subresources of the image used for the source or destination image data.
- [`image_offset`] selects the initial `x`, `y`, `z` offsets in texels of the sub-region of the source or destination image data.
- [`image_extent`] is the size in texels of the image to copy in `width`, `height` and `depth`.

# Description
When copying to or from a depth or stencil aspect, the data in buffer memory
uses a layout that is a (mostly) tightly packed representation of the depth
or stencil data.
Specifically:
- data copied to or from the stencil aspect of any depth/stencil format is tightly packed with one `VK_FORMAT_S8_UINT` value per texel.
- data copied to or from the depth aspect of a `VK_FORMAT_D16_UNORM` or `VK_FORMAT_D16_UNORM_S8_UINT` format is tightly packed with one `VK_FORMAT_D16_UNORM` value per texel.
- data copied to or from the depth aspect of a `VK_FORMAT_D32_SFLOAT` or `VK_FORMAT_D32_SFLOAT_S8_UINT` format is tightly packed with one `VK_FORMAT_D32_SFLOAT` value per texel.
- data copied to or from the depth aspect of a `VK_FORMAT_X8_D24_UNORM_PACK32` or `VK_FORMAT_D24_UNORM_S8_UINT` format is packed with one 32-bit word per texel with the D24 value in the LSBs of the word, and undefined values in the eight MSBs.
Because depth or stencil aspect buffer to image copies  **may**  require format
conversions on some implementations, they are not supported on queues that
do not support graphics.When copying to a depth aspect,
and the `[`ext_depth_range_unrestricted`]` extension is not enabled,
the data in buffer memory  **must**  be in the range [0,1], or the
resulting values are undefined.Copies are done layer by layer starting with image layer
`baseArrayLayer` member of [`image_subresource`].
`layerCount` layers are copied from the source image or to the
destination image.For purpose of valid usage statements here and in related copy commands, a
*blocked image* is defined as:
- an image with a *single-plane*, “`_422`” format, which is treated as a format with a 2 × 1 compressed texel block, or
- a compressed image.

## Valid Usage
-  [`buffer_row_length`] **must**  be `0`, or greater than or equal to the `width` member of [`image_extent`]
-  [`buffer_image_height`] **must**  be `0`, or greater than or equal to the `height` member of [`image_extent`]
-    The `aspectMask` member of [`image_subresource`] **must**  only have a single bit set
-  `imageExtent.width` **must**  not be 0
-  `imageExtent.height` **must**  not be 0
-  `imageExtent.depth` **must**  not be 0

## Valid Usage (Implicit)
-  [`image_subresource`] **must**  be a valid [`ImageSubresourceLayers`] structure

# Related
- [`crate::vulkan1_0`]
- [`DeviceSize`]
- [`Extent3D`]
- [`ImageSubresourceLayers`]
- [`Offset3D`]
- [`cmd_copy_buffer_to_image`]
- [`cmd_copy_image_to_buffer`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        