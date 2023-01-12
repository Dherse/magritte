[VkVideoDecodeInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoDecodeInfoKHR.html) - Structure specifying parameters of decoding a frame

# C Specifications
The [`VideoDecodeInfoKHR`] structure is defined as:
```c
// Provided by VK_KHR_video_decode_queue
typedef struct VkVideoDecodeInfoKHR {
    VkStructureType                   sType;
    const void*                       pNext;
    VkVideoDecodeFlagsKHR             flags;
    VkOffset2D                        codedOffset;
    VkExtent2D                        codedExtent;
    VkBuffer                          srcBuffer;
    VkDeviceSize                      srcBufferOffset;
    VkDeviceSize                      srcBufferRange;
    VkVideoPictureResourceKHR         dstPictureResource;
    const VkVideoReferenceSlotKHR*    pSetupReferenceSlot;
    uint32_t                          referenceSlotCount;
    const VkVideoReferenceSlotKHR*    pReferenceSlots;
} VkVideoDecodeInfoKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure. All the codec specific structures related to each frame(picture parameters, quantization matrix, etc.)  **must**  be chained here and pass to decode session with the function call [`cmd_decode_video_khr`].
- [`flags`] is a bitmask of [`VideoDecodeFlagBitsKHR`] specifying decode flags, reserved for future versions of this specification.
- [`coded_offset`] is the coded offset of the decode operations. The purpose of this field is interpreted based on the codec extension. When decoding content in H.264 field mode, the [`coded_offset`] specifies the line or picture field’s offset within the image.
- [`coded_extent`] is the coded size of the decode operations.
- [`src_buffer`] is the source buffer that holds the encoded bitstream.
- [`src_buffer_offset`] is the buffer offset where the valid encoded bitstream starts in srcBuffer. It  **must**  meet the alignment requirement `minBitstreamBufferOffsetAlignment` within [`VideoCapabilitiesKHR`] queried with the [`get_physical_device_video_capabilities_khr`] function.
- [`src_buffer_range`] is the size of the srcBuffer with valid encoded bitstream, starting from [`src_buffer_offset`]. It  **must**  meet the alignment requirement `minBitstreamBufferSizeAlignment` within [`VideoCapabilitiesKHR`] queried with the [`get_physical_device_video_capabilities_khr`] function.
- [`dst_picture_resource`] is the destination [Decoded Output Picture](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#decoded-output-picture) Resource.
- [`setup_reference_slot`] is `NULL` or a pointer to a [`VideoReferenceSlotKHR`] structure used for generating a DPB reference slot and Picture Resource. `pSetupReferenceSlot->slotIndex` specifies the slot index number to use as a target for producing the DPB data. `slotIndex` **must**  reference a valid entry as specified in [`VideoBeginCodingInfoKHR`] via the [`reference_slots`] within the [`cmd_begin_video_coding_khr`] command that established the Vulkan Video Decode Context for this command.
- [`reference_slot_count`] is the number of the DPB Reference Pictures that will be used when this decoding operation is executing.
- [`reference_slots`] is a pointer to an array of [`VideoReferenceSlotKHR`] structures specifying the DPB Reference pictures that will be used when this decoding operation is executing.

# Description
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_VIDEO_DECODE_INFO_KHR`
-    Each [`p_next`] member of any structure (including this one) in the [`p_next`] chain  **must**  be either `NULL` or a pointer to a valid instance of [`VideoDecodeH264PictureInfoEXT`] or [`VideoDecodeH265PictureInfoEXT`]
-    The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique
-  [`flags`] **must**  be a valid combination of [`VideoDecodeFlagBitsKHR`] values
-  [`src_buffer`] **must**  be a valid [`Buffer`] handle
-  [`dst_picture_resource`] **must**  be a valid [`VideoPictureResourceKHR`] structure
-  [`setup_reference_slot`] **must**  be a valid pointer to a valid [`VideoReferenceSlotKHR`] structure
-    If [`reference_slot_count`] is not `0`, [`reference_slots`] **must**  be a valid pointer to an array of [`reference_slot_count`] valid [`VideoReferenceSlotKHR`] structures

# Related
- [`khr_video_decode_queue`]
- [`Buffer`]
- [`DeviceSize`]
- [`Extent2D`]
- [`Offset2D`]
- [`StructureType`]
- [VkVideoDecodeFlagsKHR]()
- [`VideoPictureResourceKHR`]
- [`VideoReferenceSlotKHR`]
- [`cmd_decode_video_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        