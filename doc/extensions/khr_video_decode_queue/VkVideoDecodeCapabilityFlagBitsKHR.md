[VkVideoDecodeCapabilityFlagBitsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoDecodeCapabilityFlagBitsKHR.html) - Video decode capability flags

# C Specifications
Bits which  **may**  be set in [`VideoDecodeCapabilitiesKHR::flags`],
indicating the decoding features supported, are:
```c
// Provided by VK_KHR_video_decode_queue
typedef enum VkVideoDecodeCapabilityFlagBitsKHR {
    VK_VIDEO_DECODE_CAPABILITY_DEFAULT_KHR = 0,
    VK_VIDEO_DECODE_CAPABILITY_DPB_AND_OUTPUT_COINCIDE_BIT_KHR = 0x00000001,
    VK_VIDEO_DECODE_CAPABILITY_DPB_AND_OUTPUT_DISTINCT_BIT_KHR = 0x00000002,
} VkVideoDecodeCapabilityFlagBitsKHR;
```

# Description
- [`VK_VIDEO_DECODE_CAPABILITY_FLAG_BITS_KHR`] - reports the implementation supports using the same [Video Picture Resource](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#video-picture-resources) for decode DPB and decode output.
- [`VK_VIDEO_DECODE_CAPABILITY_FLAG_BITS_KHR`] - reports the implementation supports using distinct [Video Picture Resources](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#video-picture-resources) for decode DPB and decode output.
An implementation  **must**  report at least one of
[`VK_VIDEO_DECODE_CAPABILITY_FLAG_BITS_KHR`] or
[`VK_VIDEO_DECODE_CAPABILITY_FLAG_BITS_KHR`] as
supported.

# Related
- [`khr_video_decode_queue`]
- [VkVideoDecodeCapabilityFlagsKHR]()

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        