[VkVideoCapabilitiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoCapabilitiesKHR.html) - Structure specifying parameters of video capabilities

# C Specifications
The [`VideoCapabilitiesKHR`] structure is defined as:
```c
// Provided by VK_KHR_video_queue
typedef struct VkVideoCapabilitiesKHR {
    VkStructureType              sType;
    void*                        pNext;
    VkVideoCapabilityFlagsKHR    capabilityFlags;
    VkDeviceSize                 minBitstreamBufferOffsetAlignment;
    VkDeviceSize                 minBitstreamBufferSizeAlignment;
    VkExtent2D                   videoPictureExtentGranularity;
    VkExtent2D                   minExtent;
    VkExtent2D                   maxExtent;
    uint32_t                     maxReferencePicturesSlotsCount;
    uint32_t                     maxReferencePicturesActiveCount;
    VkExtensionProperties        stdHeaderVersion;
} VkVideoCapabilitiesKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`capability_flags`] is a bitmask of [`VideoCapabilityFlagBitsKHR`] specifying capability flags.
- [`min_bitstream_buffer_offset_alignment`] is the minimum alignment for the input or output bitstream buffer offset.
- [`min_bitstream_buffer_size_alignment`] is the minimum alignment for the input or output bitstream buffer size
- [`video_picture_extent_granularity`] is the minimum size alignment of the extent with the required padding for the decoded or encoded video images.
- [`min_extent`] is the minimum width and height of the decoded or encoded video.
- [`max_extent`] is the maximum width and height of the decoded or encoded video.
- [`max_reference_pictures_slots_count`] is the maximum number of DPB Slots supported by the implementation for a single video session instance.
- [`max_reference_pictures_active_count`] is the maximum slots that can be used as [Reference Pictures](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#reference-picture) with a single decode or encode operation.
- [`std_header_version`] is a [`ExtensionProperties`] structure reporting the Video Std header version supported for the `codecOperation` requested in [`get_physical_device_video_capabilities_khr`]`::pVideoProfile`.

# Description
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_VIDEO_CAPABILITIES_KHR`
-    Each [`p_next`] member of any structure (including this one) in the [`p_next`] chain  **must**  be either `NULL` or a pointer to a valid instance of [`VideoDecodeCapabilitiesKHR`] or [`VideoEncodeCapabilitiesKHR`]
-    The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique

# Related
- [`khr_video_queue`]
- [`DeviceSize`]
- [`ExtensionProperties`]
- [`Extent2D`]
- [`StructureType`]
- [VkVideoCapabilityFlagsKHR]()
- [`get_physical_device_video_capabilities_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        