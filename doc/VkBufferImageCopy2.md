[VkBufferImageCopy2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBufferImageCopy2.html) - Structure specifying a buffer image copy operation

# C Specifications
For both [`cmd_copy_buffer_to_image2`] and [`cmd_copy_image_to_buffer2`],
each element of `pRegions` is a structure defined as:
```c
// Provided by VK_VERSION_1_3
typedef struct VkBufferImageCopy2 {
    VkStructureType             sType;
    const void*                 pNext;
    VkDeviceSize                bufferOffset;
    uint32_t                    bufferRowLength;
    uint32_t                    bufferImageHeight;
    VkImageSubresourceLayers    imageSubresource;
    VkOffset3D                  imageOffset;
    VkExtent3D                  imageExtent;
} VkBufferImageCopy2;
```
or the equivalent
```c
// Provided by VK_KHR_copy_commands2
typedef VkBufferImageCopy2 VkBufferImageCopy2KHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`buffer_offset`] is the offset in bytes from the start of the buffer object where the image data is copied from or to.
- [`buffer_row_length`] and [`buffer_image_height`] specify in texels a subregion of a larger two- or three-dimensional image in buffer memory, and control the addressing calculations. If either of these values is zero, that aspect of the buffer memory is considered to be tightly packed according to the [`image_extent`].
- [`image_subresource`] is a [`ImageSubresourceLayers`] used to specify the specific image subresources of the image used for the source or destination image data.
- [`image_offset`] selects the initial `x`, `y`, `z` offsets in texels of the sub-region of the source or destination image data.
- [`image_extent`] is the size in texels of the image to copy in `width`, `height` and `depth`.

# Description
This structure is functionally identical to [`BufferImageCopy`], but
adds [`s_type`] and [`p_next`] parameters, allowing it to be more easily
extended.
## Valid Usage
-  [`buffer_row_length`] **must**  be `0`, or greater than or equal to the `width` member of [`image_extent`]
-  [`buffer_image_height`] **must**  be `0`, or greater than or equal to the `height` member of [`image_extent`]
-    The `aspectMask` member of [`image_subresource`] **must**  only have a single bit set
-  `imageExtent.width` **must**  not be 0
-  `imageExtent.height` **must**  not be 0
-  `imageExtent.depth` **must**  not be 0

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_BUFFER_IMAGE_COPY_2`
-  [`p_next`] **must**  be `NULL` or a pointer to a valid instance of [`CopyCommandTransformInfoQCOM`]
-    The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique
-  [`image_subresource`] **must**  be a valid [`ImageSubresourceLayers`] structure

# Related
- [`VK_KHR_copy_commands2`]
- [`crate::vulkan1_3`]
- [`CopyBufferToImageInfo2`]
- [`CopyImageToBufferInfo2`]
- [`DeviceSize`]
- [`Extent3D`]
- [`ImageSubresourceLayers`]
- [`Offset3D`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        