[VkVideoEncodeCapabilitiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeCapabilitiesKHR.html) - Structure specifying encode capabilities

# C Specifications
When calling [`get_physical_device_video_capabilities_khr`] with
`pVideoProfile->videoCodecOperation` specified as one of the encode
operation bits, the [`VideoEncodeCapabilitiesKHR`] structure  **must**  be
included in the [`p_next`] chain of the [`VideoCapabilitiesKHR`]
structure to retrieve capabilities specific to video encoding.The [`VideoEncodeCapabilitiesKHR`] structure is defined as:
```c
// Provided by VK_KHR_video_encode_queue
typedef struct VkVideoEncodeCapabilitiesKHR {
    VkStructureType                         sType;
    const void*                             pNext;
    VkVideoEncodeCapabilityFlagsKHR         flags;
    VkVideoEncodeRateControlModeFlagsKHR    rateControlModes;
    uint8_t                                 rateControlLayerCount;
    uint8_t                                 qualityLevelCount;
    VkExtent2D                              inputImageDataFillAlignment;
} VkVideoEncodeCapabilitiesKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`flags`] is a bitmask of [`VideoEncodeCapabilityFlagBitsKHR`] describing supported encoding features.
- [`rate_control_modes`] is a bitmask of [`VideoEncodeRateControlModeFlagBitsKHR`] describing supported rate control modes. All implementations  **must**  support `VK_VIDEO_ENCODE_RATE_CONTROL_MODE_NONE_BIT_KHR`.
- [`rate_control_layer_count`] reports the maximum number of rate control layers supported. Implementations  **must**  report at least 1.
- [`quality_level_count`] is the number of discrete quality levels supported. Implementations  **must**  report at least 1.
- [`input_image_data_fill_alignment`] reports alignment of data that should be filled in the input image horizontally and vertically in pixels before encode operations are performed on the input image.

# Description
The input content and encode resolution (specified in
[`VideoEncodeInfoKHR::coded_extent`]) may not be aligned with the
codec-specific coding block size.
For example, the input content may be 1920x1080 and the coding block size
may be 16x16 pixel blocks.
In this example, the content is horizontally aligned with the coding block
size, but not vertically aligned with the coding block size.
Encoding of the last row of blocks may be impacted by contents of the input
image in pixel rows 1081 to 1088 (the next vertical alignment with the
coding block size).
In general, to ensure efficient encoding for the last row/column of blocks,
and/or to ensure consistent encoding results between repeated encoding of
the same input content, these extra pixel rows/columns should be filled to
known values up to the coding block size alignment before encoding
operations are performed.
Some implementations support performing auto-fill of unaligned pixels beyond
a specific alignment, which is reported in
[`input_image_data_fill_alignment`].
For example, if an implementation reports 1x1 in
[`input_image_data_fill_alignment`], then the implementation will perform
auto-fill for any unaligned pixels beyond the encode resolution up to the
next coding block size.
For a coding block size of 16x16, if the implementation reports 16x16 in
[`input_image_data_fill_alignment`], then it is the application’s
responsibility to fill any unaligned pixels, if desired.
If not, it may impact the encoding efficiency, but it will not affect the
validity of the generated bitstream.
If the implementation reports 8x8 in [`input_image_data_fill_alignment`], then
for the 1920x1080 example, since the content is aligned to 8 pixels
vertically, the implementation will auto-fill pixel rows 1081 to 1088 (up to
the 16x16 coding block size in the example).
The auto-fill value(s) are implementation-specific.
The auto-fill value(s) are not written to the input image memory, but are
used as part of the encoding operation on the input image.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_VIDEO_ENCODE_CAPABILITIES_KHR`
-  [`rate_control_modes`] **must**  be a valid combination of [`VideoEncodeRateControlModeFlagBitsKHR`] values
-  [`rate_control_modes`] **must**  not be `0`

# Related
- [`khr_video_encode_queue`]
- [`Extent2D`]
- [`StructureType`]
- [VkVideoEncodeCapabilityFlagsKHR]()
- [VkVideoEncodeRateControlModeFlagsKHR]()

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        