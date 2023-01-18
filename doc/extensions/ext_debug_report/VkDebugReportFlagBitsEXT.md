[VkDebugReportFlagBitsEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDebugReportFlagBitsEXT.html) - Bitmask specifying events which cause a debug report callback

# C Specifications
Bits which  **can**  be set in
[`DebugReportCallbackCreateInfoEXT::flags`], specifying events
which cause a debug report, are:
```c
// Provided by VK_EXT_debug_report
typedef enum VkDebugReportFlagBitsEXT {
    VK_DEBUG_REPORT_INFORMATION_BIT_EXT = 0x00000001,
    VK_DEBUG_REPORT_WARNING_BIT_EXT = 0x00000002,
    VK_DEBUG_REPORT_PERFORMANCE_WARNING_BIT_EXT = 0x00000004,
    VK_DEBUG_REPORT_ERROR_BIT_EXT = 0x00000008,
    VK_DEBUG_REPORT_DEBUG_BIT_EXT = 0x00000010,
} VkDebugReportFlagBitsEXT;
```

# Description
- [`ERROR`] specifies that the application has violated a valid usage condition of the specification.
- [`WARNING`] specifies use of Vulkan that  **may**  expose an app bug. Such cases may not be immediately harmful, such as a fragment shader outputting to a location with no attachment. Other cases  **may**  point to behavior that is almost certainly bad when unintended such as using an image whose memory has not been filled. In general if you see a warning but you know that the behavior is intended/desired, then simply ignore the warning.
- [`PERFORMANCE_WARNING`] specifies a potentially non-optimal use of Vulkan, e.g. using [`cmd_clear_color_image`] when setting [`AttachmentDescription::load_op`] to `VK_ATTACHMENT_LOAD_OP_CLEAR` would have worked.
- [`INFORMATION`] specifies an informational message such as resource details that may be handy when debugging an application.
- [`DEBUG`] specifies diagnostic information from the implementation and layers.

# Related
- [`VK_EXT_debug_report`]
- [`DebugReportFlagsEXT`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        