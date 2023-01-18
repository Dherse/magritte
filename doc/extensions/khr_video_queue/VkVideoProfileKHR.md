[VkVideoProfileKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoProfileKHR.html) - Structure specifying the codec video profile

# C Specifications
A video profile is defined by [`VideoProfileKHR`] structure as:
```c
// Provided by VK_KHR_video_queue
typedef struct VkVideoProfileKHR {
    VkStructureType                     sType;
    void*                               pNext;
    VkVideoCodecOperationFlagBitsKHR    videoCodecOperation;
    VkVideoChromaSubsamplingFlagsKHR    chromaSubsampling;
    VkVideoComponentBitDepthFlagsKHR    lumaBitDepth;
    VkVideoComponentBitDepthFlagsKHR    chromaBitDepth;
} VkVideoProfileKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`video_codec_operation`] is a [`VideoCodecOperationFlagBitsKHR`] value specifying a video codec operation.
- [`chroma_subsampling`] is a bitmask of [`VideoChromaSubsamplingFlagBitsKHR`] specifying video chroma subsampling information.
- [`luma_bit_depth`] is a bitmask of [`VideoComponentBitDepthFlagBitsKHR`] specifying video luma bit depth information.
- [`chroma_bit_depth`] is a bitmask of [`VideoComponentBitDepthFlagBitsKHR`] specifying video chroma bit depth information.

# Description
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_VIDEO_PROFILE_KHR`
-  [`video_codec_operation`] **must**  be a valid [`VideoCodecOperationFlagBitsKHR`] value
-  [`chroma_subsampling`] **must**  be a valid combination of [`VideoChromaSubsamplingFlagBitsKHR`] values
-  [`chroma_subsampling`] **must**  not be `0`
-  [`luma_bit_depth`] **must**  be a valid combination of [`VideoComponentBitDepthFlagBitsKHR`] values
-  [`luma_bit_depth`] **must**  not be `0`
-  [`chroma_bit_depth`] **must**  be a valid combination of [`VideoComponentBitDepthFlagBitsKHR`] values
-  [`chroma_bit_depth`] **must**  not be `0`

# Related
- [`VK_KHR_video_queue`]
- [`StructureType`]
- [`VideoChromaSubsamplingFlagsKHR`]
- [`VideoCodecOperationFlagBitsKHR`]
- [`VideoComponentBitDepthFlagsKHR`]
- [`VideoProfilesKHR`]
- [`VideoSessionCreateInfoKHR`]
- [`get_physical_device_video_capabilities_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        