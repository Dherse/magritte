[VkDebugUtilsMessageSeverityFlagBitsEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDebugUtilsMessageSeverityFlagBitsEXT.html) - Bitmask specifying which severities of events cause a debug messenger callback

# C Specifications
Bits which  **can**  be set in
[`DebugUtilsMessengerCreateInfoEXT::message_severity`], specifying
event severities which cause a debug messenger to call the callback, are:
```c
// Provided by VK_EXT_debug_utils
typedef enum VkDebugUtilsMessageSeverityFlagBitsEXT {
    VK_DEBUG_UTILS_MESSAGE_SEVERITY_VERBOSE_BIT_EXT = 0x00000001,
    VK_DEBUG_UTILS_MESSAGE_SEVERITY_INFO_BIT_EXT = 0x00000010,
    VK_DEBUG_UTILS_MESSAGE_SEVERITY_WARNING_BIT_EXT = 0x00000100,
    VK_DEBUG_UTILS_MESSAGE_SEVERITY_ERROR_BIT_EXT = 0x00001000,
} VkDebugUtilsMessageSeverityFlagBitsEXT;
```

# Description
- [`VK_DEBUG_UTILS_MESSAGE_SEVERITY_FLAG_BITS_EXT`] specifies the most verbose output indicating all diagnostic messages from the Vulkan loader, layers, and drivers should be captured.
- [`VK_DEBUG_UTILS_MESSAGE_SEVERITY_FLAG_BITS_EXT`] specifies an informational message such as resource details that may be handy when debugging an application.
- [`VK_DEBUG_UTILS_MESSAGE_SEVERITY_FLAG_BITS_EXT`] specifies use of Vulkan that  **may**  expose an app bug. Such cases may not be immediately harmful, such as a fragment shader outputting to a location with no attachment. Other cases  **may**  point to behavior that is almost certainly bad when unintended such as using an image whose memory has not been filled. In general if you see a warning but you know that the behavior is intended/desired, then simply ignore the warning.
- [`VK_DEBUG_UTILS_MESSAGE_SEVERITY_FLAG_BITS_EXT`] specifies that the application has violated a valid usage condition of the specification.

# Related
- [`ext_debug_utils`]
- [VkDebugUtilsMessageSeverityFlagsEXT]()
- [`submit_debug_utils_message_ext`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        