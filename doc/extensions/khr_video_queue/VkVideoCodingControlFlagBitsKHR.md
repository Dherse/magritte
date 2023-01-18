[VkVideoCodingControlFlagBitsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoCodingControlFlagBitsKHR.html) - Video Coding Control Command Flags

# C Specifications
The [`cmd_control_video_coding_khr`] flags are defined with the following
enumeration:
```c
// Provided by VK_KHR_video_queue
typedef enum VkVideoCodingControlFlagBitsKHR {
    VK_VIDEO_CODING_CONTROL_DEFAULT_KHR = 0,
    VK_VIDEO_CODING_CONTROL_RESET_BIT_KHR = 0x00000001,
} VkVideoCodingControlFlagBitsKHR;
```

# Description
- [`DEFAULT`] indicates a request for the coding control paramaters to be applied to the current state of the bound video session.
- [`RESET`] indicates a request for the bound video session device context to be reset before the coding control parameters are applied.
A newly created video session  **must**  be reset before use for video decode or
encode operations.
The reset operation returns all session DPB slots to the unused state (see
[DPB Slot States](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#video-session-dpb-slot-states)).
For encode sessions, the reset operation returns rate control configuration
to implementation default settings.
After decode or encode operations are performed on a session, the reset
operation  **may**  be used to return the video session device context to the
same initial state as after the reset of a newly created video session.
This  **may**  be used when different video sequences are processed with the same
session.

# Related
- [`VK_KHR_video_queue`]
- [`VideoCodingControlFlagsKHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        