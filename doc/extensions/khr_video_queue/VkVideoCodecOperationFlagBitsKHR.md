[VkVideoCodecOperationFlagBitsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoCodecOperationFlagBitsKHR.html) - Video codec operation types

# C Specifications
The codec operations are defined with the
[`VideoCodecOperationFlagBitsKHR`] enum:
```c
// Provided by VK_KHR_video_queue
typedef enum VkVideoCodecOperationFlagBitsKHR {
    VK_VIDEO_CODEC_OPERATION_INVALID_BIT_KHR = 0,
#ifdef VK_ENABLE_BETA_EXTENSIONS
  // Provided by VK_EXT_video_encode_h264
    VK_VIDEO_CODEC_OPERATION_ENCODE_H264_BIT_EXT = 0x00010000,
#endif
#ifdef VK_ENABLE_BETA_EXTENSIONS
  // Provided by VK_EXT_video_encode_h265
    VK_VIDEO_CODEC_OPERATION_ENCODE_H265_BIT_EXT = 0x00020000,
#endif
#ifdef VK_ENABLE_BETA_EXTENSIONS
  // Provided by VK_EXT_video_decode_h264
    VK_VIDEO_CODEC_OPERATION_DECODE_H264_BIT_EXT = 0x00000001,
#endif
#ifdef VK_ENABLE_BETA_EXTENSIONS
  // Provided by VK_EXT_video_decode_h265
    VK_VIDEO_CODEC_OPERATION_DECODE_H265_BIT_EXT = 0x00000002,
#endif
} VkVideoCodecOperationFlagBitsKHR;
```

# Description
Each decode or encode codec-specific extension extends this enumeration with
the appropriate bit corresponding to the extension’s codec operation:
- [`INVALID`] - No video operations are supported for this queue family.
- [`ENCODE_H264_EXT`] - H.264 video encode operations are supported by this queue family.
- [`DECODE_H264_EXT`] - H.264 video decode operations are supported by this queue family.
- [`DECODE_H265_EXT`] - H.265 video decode operations are supported by this queue family.

# Related
- [`VK_KHR_video_queue`]
- [`VideoCodecOperationFlagsKHR`]
- [`VideoProfileKHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        