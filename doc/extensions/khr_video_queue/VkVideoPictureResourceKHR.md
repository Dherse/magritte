[VkVideoPictureResourceKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoPictureResourceKHR.html) - Structure specifying the picture resources

# C Specifications
The [`VideoPictureResourceKHR`] structure is defined as:
```c
// Provided by VK_KHR_video_queue
typedef struct VkVideoPictureResourceKHR {
    VkStructureType    sType;
    const void*        pNext;
    VkOffset2D         codedOffset;
    VkExtent2D         codedExtent;
    uint32_t           baseArrayLayer;
    VkImageView        imageViewBinding;
} VkVideoPictureResourceKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`coded_offset`] is the offset to be used for the picture resource.
- [`coded_extent`] is the extent to be used for the picture resource.
- [`base_array_layer`] is the first array layer to be accessed for the Decode or Encode Operations.
- [`image_view_binding`] is a [`ImageView`] image view representing this picture resource.

# Description
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_VIDEO_PICTURE_RESOURCE_KHR`
-  [`p_next`] **must**  be `NULL`
-  [`image_view_binding`] **must**  be a valid [`ImageView`] handle

# Related
- [`khr_video_queue`]
- [`Extent2D`]
- [`ImageView`]
- [`Offset2D`]
- [`StructureType`]
- [`VideoDecodeInfoKHR`]
- [`VideoEncodeInfoKHR`]
- [`VideoReferenceSlotKHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        