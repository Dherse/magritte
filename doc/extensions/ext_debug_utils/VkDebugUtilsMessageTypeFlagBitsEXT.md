[VkDebugUtilsMessageTypeFlagBitsEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDebugUtilsMessageTypeFlagBitsEXT.html) - Bitmask specifying which types of events cause a debug messenger callback

# C Specifications
Bits which  **can**  be set in
[`DebugUtilsMessengerCreateInfoEXT::message_type`], specifying
event types which cause a debug messenger to call the callback, are:
```c
// Provided by VK_EXT_debug_utils
typedef enum VkDebugUtilsMessageTypeFlagBitsEXT {
    VK_DEBUG_UTILS_MESSAGE_TYPE_GENERAL_BIT_EXT = 0x00000001,
    VK_DEBUG_UTILS_MESSAGE_TYPE_VALIDATION_BIT_EXT = 0x00000002,
    VK_DEBUG_UTILS_MESSAGE_TYPE_PERFORMANCE_BIT_EXT = 0x00000004,
} VkDebugUtilsMessageTypeFlagBitsEXT;
```

# Description
- [`VK_DEBUG_UTILS_MESSAGE_TYPE_FLAG_BITS_EXT`] specifies that some general event has occurred. This is typically a non-specification, non-performance event.
- [`VK_DEBUG_UTILS_MESSAGE_TYPE_FLAG_BITS_EXT`] specifies that something has occurred during validation against the Vulkan specification that may indicate invalid behavior.
- [`VK_DEBUG_UTILS_MESSAGE_TYPE_FLAG_BITS_EXT`] specifies a potentially non-optimal use of Vulkan, e.g. using [`cmd_clear_color_image`] when setting [`AttachmentDescription::load_op`] to `VK_ATTACHMENT_LOAD_OP_CLEAR` would have worked.

# Related
- [`ext_debug_utils`]
- [VkDebugUtilsMessageTypeFlagsEXT]()

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        