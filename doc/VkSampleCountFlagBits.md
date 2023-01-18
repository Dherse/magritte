[VkSampleCountFlagBits](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSampleCountFlagBits.html) - Bitmask specifying sample counts supported for an image used for storage operations

# C Specifications
Bits which  **may**  be set in the sample count limits returned by
[`PhysicalDeviceLimits`], as well as in other queries and structures
representing image sample counts, are:
```c
// Provided by VK_VERSION_1_0
typedef enum VkSampleCountFlagBits {
    VK_SAMPLE_COUNT_1_BIT = 0x00000001,
    VK_SAMPLE_COUNT_2_BIT = 0x00000002,
    VK_SAMPLE_COUNT_4_BIT = 0x00000004,
    VK_SAMPLE_COUNT_8_BIT = 0x00000008,
    VK_SAMPLE_COUNT_16_BIT = 0x00000010,
    VK_SAMPLE_COUNT_32_BIT = 0x00000020,
    VK_SAMPLE_COUNT_64_BIT = 0x00000040,
} VkSampleCountFlagBits;
```

# Description
- [`1`] specifies an image with one sample per pixel.
- [`2`] specifies an image with 2 samples per pixel.
- [`4`] specifies an image with 4 samples per pixel.
- [`8`] specifies an image with 8 samples per pixel.
- [`16`] specifies an image with 16 samples per pixel.
- [`32`] specifies an image with 32 samples per pixel.
- [`64`] specifies an image with 64 samples per pixel.

# Related
- [`crate::vulkan1_0`]
- [`AttachmentDescription`]
- [`AttachmentDescription2`]
- [`AttachmentSampleCountInfoAMD`]
- [`CommandBufferInheritanceRenderingInfo`]
- [`FramebufferMixedSamplesCombinationNV`]
- [`ImageCreateInfo`]
- [`PhysicalDeviceFragmentShadingRateEnumsPropertiesNV`]
- [`PhysicalDeviceFragmentShadingRatePropertiesKHR`]
- [`PhysicalDeviceSparseImageFormatInfo2`]
- [`PipelineMultisampleStateCreateInfo`]
- [`SampleCountFlags`]
- [`SampleLocationsInfoEXT`]
- [`get_physical_device_multisample_properties_ext`]
- [`get_physical_device_sparse_image_format_properties`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        