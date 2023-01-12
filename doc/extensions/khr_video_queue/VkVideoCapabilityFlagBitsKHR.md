[VkVideoCapabilityFlagBitsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoCapabilityFlagBitsKHR.html) - Video Decode and Encode Capability Flags

# C Specifications
The [`VideoCapabilitiesKHR`] flags are defined with the following
enumeration:
```c
// Provided by VK_KHR_video_queue
typedef enum VkVideoCapabilityFlagBitsKHR {
    VK_VIDEO_CAPABILITY_PROTECTED_CONTENT_BIT_KHR = 0x00000001,
    VK_VIDEO_CAPABILITY_SEPARATE_REFERENCE_IMAGES_BIT_KHR = 0x00000002,
} VkVideoCapabilityFlagBitsKHR;
```

# Description
- [`VK_VIDEO_CAPABILITY_FLAG_BITS_KHR`] - the decode or encode session supports protected content.
- [`VK_VIDEO_CAPABILITY_FLAG_BITS_KHR`] - the DPB or Reconstructed Video Picture Resources for the video session  **may**  be created as a separate [`Image`] for each DPB picture. If not supported, the DPB  **must**  be created as single multi-layered image where each layer represents one of the DPB Video Picture Resources.

# Related
- [`khr_video_queue`]
- [VkVideoCapabilityFlagsKHR]()

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        