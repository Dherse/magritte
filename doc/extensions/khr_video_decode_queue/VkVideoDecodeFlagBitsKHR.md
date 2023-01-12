[VkVideoDecodeFlagBitsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoDecodeFlagBitsKHR.html) - Video Decode Command Flags

# C Specifications
The [`cmd_decode_video_khr`] flags are defined with the following
enumeration:
```c
// Provided by VK_KHR_video_decode_queue
typedef enum VkVideoDecodeFlagBitsKHR {
    VK_VIDEO_DECODE_DEFAULT_KHR = 0,
    VK_VIDEO_DECODE_RESERVED_0_BIT_KHR = 0x00000001,
} VkVideoDecodeFlagBitsKHR;
```

# Description
- [`VK_VIDEO_DECODE_FLAG_BITS_KHR`] The current version of the specification has reserved this value for future use.

# Related
- [`khr_video_decode_queue`]
- [VkVideoDecodeFlagsKHR]()

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        