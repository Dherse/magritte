[VkVideoReferenceSlotKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoReferenceSlotKHR.html) - Structure specifying the reference picture slot

# C Specifications
The [`VideoReferenceSlotKHR`] structure is defined as:
```c
// Provided by VK_KHR_video_queue
typedef struct VkVideoReferenceSlotKHR {
    VkStructureType                     sType;
    const void*                         pNext;
    int8_t                              slotIndex;
    const VkVideoPictureResourceKHR*    pPictureResource;
} VkVideoReferenceSlotKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`slot_index`] is the unique reference slot index used for the encode or decode operation.
- [`picture_resource`] is a pointer to a [`VideoPictureResourceKHR`] structure describing the picture resource bound to this slot index.

# Description
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_VIDEO_REFERENCE_SLOT_KHR`
-    Each [`p_next`] member of any structure (including this one) in the [`p_next`] chain  **must**  be either `NULL` or a pointer to a valid instance of [`VideoDecodeH264DpbSlotInfoEXT`] or [`VideoDecodeH265DpbSlotInfoEXT`]
-    The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique
-  [`picture_resource`] **must**  be a valid pointer to a valid [`VideoPictureResourceKHR`] structure

# Related
- [`VK_KHR_video_queue`]
- [`StructureType`]
- [`VideoBeginCodingInfoKHR`]
- [`VideoDecodeInfoKHR`]
- [`VideoEncodeInfoKHR`]
- [`VideoPictureResourceKHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        