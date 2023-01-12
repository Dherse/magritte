[VkVideoComponentBitDepthFlagBitsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoComponentBitDepthFlagBitsKHR.html) - Video component bit depth

# C Specifications
The video format component bit depth is defined with the following enums:
```c
// Provided by VK_KHR_video_queue
typedef enum VkVideoComponentBitDepthFlagBitsKHR {
    VK_VIDEO_COMPONENT_BIT_DEPTH_INVALID_KHR = 0,
    VK_VIDEO_COMPONENT_BIT_DEPTH_8_BIT_KHR = 0x00000001,
    VK_VIDEO_COMPONENT_BIT_DEPTH_10_BIT_KHR = 0x00000004,
    VK_VIDEO_COMPONENT_BIT_DEPTH_12_BIT_KHR = 0x00000010,
} VkVideoComponentBitDepthFlagBitsKHR;
```

# Description
- [`VK_VIDEO_COMPONENT_BIT_DEPTH_FLAG_BITS_KHR`] - the format component bit depth is 8 bits.
- [`VK_VIDEO_COMPONENT_BIT_DEPTH_FLAG_BITS_KHR`] - the format component bit depth is 10 bits.
- [`VK_VIDEO_COMPONENT_BIT_DEPTH_FLAG_BITS_KHR`] - the format component bit depth is 12 bits.

# Related
- [`khr_video_queue`]
- [VkVideoComponentBitDepthFlagsKHR]()

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        