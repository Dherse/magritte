[VkVideoSessionCreateFlagBitsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoSessionCreateFlagBitsKHR.html) - Video decode or encode video session creation flags

# C Specifications
The decode or encode session creation flags defined with the following
enums:
```c
// Provided by VK_KHR_video_queue
typedef enum VkVideoSessionCreateFlagBitsKHR {
    VK_VIDEO_SESSION_CREATE_DEFAULT_KHR = 0,
    VK_VIDEO_SESSION_CREATE_PROTECTED_CONTENT_BIT_KHR = 0x00000001,
} VkVideoSessionCreateFlagBitsKHR;
```

# Description
- [`PROTECTED_CONTENT`] - create the video session for use with protected video content

# Related
- [`VK_KHR_video_queue`]
- [`VideoSessionCreateFlagsKHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        