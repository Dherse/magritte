[VkVideoEncodeRateControlModeFlagBitsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeRateControlModeFlagBitsKHR.html) - Video encode rate control modes

# C Specifications
The rate control modes are defined with the following enums:
```c
// Provided by VK_KHR_video_encode_queue
typedef enum VkVideoEncodeRateControlModeFlagBitsKHR {
    VK_VIDEO_ENCODE_RATE_CONTROL_MODE_NONE_BIT_KHR = 0,
    VK_VIDEO_ENCODE_RATE_CONTROL_MODE_CBR_BIT_KHR = 1,
    VK_VIDEO_ENCODE_RATE_CONTROL_MODE_VBR_BIT_KHR = 2,
} VkVideoEncodeRateControlModeFlagBitsKHR;
```

# Description
- [`VK_VIDEO_ENCODE_RATE_CONTROL_MODE_FLAG_BITS_KHR`] for disabling rate control.
- [`VK_VIDEO_ENCODE_RATE_CONTROL_MODE_FLAG_BITS_KHR`] for constant bitrate rate control mode.
- [`VK_VIDEO_ENCODE_RATE_CONTROL_MODE_FLAG_BITS_KHR`] for variable bitrate rate control mode.

# Related
- [`khr_video_encode_queue`]
- [`VideoEncodeRateControlInfoKHR`]
- [VkVideoEncodeRateControlModeFlagsKHR]()

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        