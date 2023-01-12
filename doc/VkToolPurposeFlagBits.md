[VkToolPurposeFlagBits](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkToolPurposeFlagBits.html) - Bitmask specifying the purposes of an active tool

# C Specifications
Bits which  **can**  be set in
[`PhysicalDeviceToolProperties::purposes`], specifying the
purposes of an active tool, are:
```c
// Provided by VK_VERSION_1_3
typedef enum VkToolPurposeFlagBits {
    VK_TOOL_PURPOSE_VALIDATION_BIT = 0x00000001,
    VK_TOOL_PURPOSE_PROFILING_BIT = 0x00000002,
    VK_TOOL_PURPOSE_TRACING_BIT = 0x00000004,
    VK_TOOL_PURPOSE_ADDITIONAL_FEATURES_BIT = 0x00000008,
    VK_TOOL_PURPOSE_MODIFYING_FEATURES_BIT = 0x00000010,
  // Provided by VK_EXT_debug_report with VK_EXT_tooling_info, VK_EXT_debug_utils with VK_EXT_tooling_info
    VK_TOOL_PURPOSE_DEBUG_REPORTING_BIT_EXT = 0x00000020,
  // Provided by VK_EXT_debug_marker with VK_EXT_tooling_info, VK_EXT_debug_utils with VK_EXT_tooling_info
    VK_TOOL_PURPOSE_DEBUG_MARKERS_BIT_EXT = 0x00000040,
    VK_TOOL_PURPOSE_VALIDATION_BIT_EXT = VK_TOOL_PURPOSE_VALIDATION_BIT,
    VK_TOOL_PURPOSE_PROFILING_BIT_EXT = VK_TOOL_PURPOSE_PROFILING_BIT,
    VK_TOOL_PURPOSE_TRACING_BIT_EXT = VK_TOOL_PURPOSE_TRACING_BIT,
    VK_TOOL_PURPOSE_ADDITIONAL_FEATURES_BIT_EXT = VK_TOOL_PURPOSE_ADDITIONAL_FEATURES_BIT,
    VK_TOOL_PURPOSE_MODIFYING_FEATURES_BIT_EXT = VK_TOOL_PURPOSE_MODIFYING_FEATURES_BIT,
} VkToolPurposeFlagBits;
```
or the equivalent
```c
// Provided by VK_EXT_tooling_info
typedef VkToolPurposeFlagBits VkToolPurposeFlagBitsEXT;
```

# Description
- [`VK_TOOL_PURPOSE_FLAG_BITS`] specifies that the tool provides validation of API usage.
- [`VK_TOOL_PURPOSE_FLAG_BITS`] specifies that the tool provides profiling of API usage.
- [`VK_TOOL_PURPOSE_FLAG_BITS`] specifies that the tool is capturing data about the application’s API usage, including anything from simple logging to capturing data for later replay.
- [`VK_TOOL_PURPOSE_FLAG_BITS`] specifies that the tool provides additional API features/extensions on top of the underlying implementation.
- [`VK_TOOL_PURPOSE_FLAG_BITS`] specifies that the tool modifies the API features/limits/extensions presented to the application.
- [`DEBUG_REPORTING_EXT`] specifies that the tool reports additional information to the application via callbacks specified by [`create_debug_report_callback_ext`] or [`create_debug_utils_messenger_ext`]
- [`DEBUG_MARKERS_EXT`] specifies that the tool consumes [debug markers](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#debugging-debug-markers) or [object debug annotation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#debugging-object-debug-annotation), [queue labels](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#debugging-queue-labels), or [command buffer labels](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#debugging-command-buffer-labels)

# Related
- [`ext_tooling_info`]
- [`crate::vulkan1_3`]
- [VkToolPurposeFlags]()

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        