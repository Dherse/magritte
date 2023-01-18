[VkVideoEncodeFlagBitsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeFlagBitsKHR.html) - Video Encode Command Flags

# C Specifications
The [`cmd_encode_video_khr`] flags are defined with the following
enumeration:
```c
// Provided by VK_KHR_video_encode_queue
typedef enum VkVideoEncodeFlagBitsKHR {
    VK_VIDEO_ENCODE_DEFAULT_KHR = 0,
    VK_VIDEO_ENCODE_RESERVED_0_BIT_KHR = 0x00000001,
} VkVideoEncodeFlagBitsKHR;
```

# Description
- [`RESERVED0`] The current version of the specification has reserved this value for future use.

# Related
- [`VK_KHR_video_encode_queue`]
- [`VideoEncodeFlagsKHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        