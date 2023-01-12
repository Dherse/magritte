[VkVideoEncodeCapabilityFlagBitsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeCapabilityFlagBitsKHR.html) - Video encode capability flags

# C Specifications
Bits which  **may**  be set in [`VideoEncodeCapabilitiesKHR::flags`],
indicating the encoding tools supported, are:
```c
// Provided by VK_KHR_video_encode_queue
typedef enum VkVideoEncodeCapabilityFlagBitsKHR {
    VK_VIDEO_ENCODE_CAPABILITY_DEFAULT_KHR = 0,
    VK_VIDEO_ENCODE_CAPABILITY_PRECEDING_EXTERNALLY_ENCODED_BYTES_BIT_KHR = 0x00000001,
} VkVideoEncodeCapabilityFlagBitsKHR;
```

# Description
- [`VK_VIDEO_ENCODE_CAPABILITY_FLAG_BITS_KHR`] reports that the implementation supports use of [`VideoEncodeInfoKHR::preceding_externally_encoded_bytes`].

# Related
- [`khr_video_encode_queue`]
- [VkVideoEncodeCapabilityFlagsKHR]()

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        