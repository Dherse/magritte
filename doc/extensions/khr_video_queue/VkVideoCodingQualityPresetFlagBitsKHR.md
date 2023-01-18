[VkVideoCodingQualityPresetFlagBitsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoCodingQualityPresetFlagBitsKHR.html) - Video codec profile types

# C Specifications
The decode preset types are defined with the following:
```c
// Provided by VK_KHR_video_queue
typedef enum VkVideoCodingQualityPresetFlagBitsKHR {
    VK_VIDEO_CODING_QUALITY_PRESET_NORMAL_BIT_KHR = 0x00000001,
    VK_VIDEO_CODING_QUALITY_PRESET_POWER_BIT_KHR = 0x00000002,
    VK_VIDEO_CODING_QUALITY_PRESET_QUALITY_BIT_KHR = 0x00000004,
} VkVideoCodingQualityPresetFlagBitsKHR;
```

# Description
- [`NORMAL`] defines normal decode case.
- [`POWER`] defines power efficient case.
- [`QUALITY`] defines quality focus case.

# Related
- [`VK_KHR_video_queue`]
- [`VideoCodingQualityPresetFlagsKHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        