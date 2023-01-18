[vkDebugReportMessageEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDebugReportMessageEXT.html) - Inject a message into a debug stream

# C Specifications
To inject its own messages into the debug stream, call:
```c
// Provided by VK_EXT_debug_report
void vkDebugReportMessageEXT(
    VkInstance                                  instance,
    VkDebugReportFlagsEXT                       flags,
    VkDebugReportObjectTypeEXT                  objectType,
    uint64_t                                    object,
    size_t                                      location,
    int32_t                                     messageCode,
    const char*                                 pLayerPrefix,
    const char*                                 pMessage);
```

# Parameters
- [`instance`] is the debug stream’s [`Instance`].
- [`flags`] specifies the [`DebugReportFlagBitsEXT`] classification of this event/message.
- [`object_type`] is a [`DebugReportObjectTypeEXT`] specifying the type of object being used or created at the time the event was triggered.
- [`object`] is the object where the issue was detected. [`object`] **can**  be [`crate::Handle::null`] if there is no object associated with the event.
- [`location`] is an application defined value.
- [`message_code`] is an application defined value.
- [`p_layer_prefix`] is the abbreviation of the component making this event/message.
- [`p_message`] is a null-terminated string detailing the trigger conditions.

# Description
The call will propagate through the layers and generate callback(s) as
indicated by the message’s flags.
The parameters are passed on to the callback in addition to the
`pUserData` value that was defined at the time the callback was
registered.
## Valid Usage
-  [`object`] **must**  be a Vulkan object or [`crate::Handle::null`]
-    If [`object_type`] is not `VK_DEBUG_REPORT_OBJECT_TYPE_UNKNOWN_EXT` and [`object`] is not [`crate::Handle::null`], [`object`] **must**  be a Vulkan object of the corresponding type associated with [`object_type`] as defined in [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#debug-report-object-types](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#debug-report-object-types)

## Valid Usage (Implicit)
-  [`instance`] **must**  be a valid [`Instance`] handle
-  [`flags`] **must**  be a valid combination of [`DebugReportFlagBitsEXT`] values
-  [`flags`] **must**  not be `0`
-  [`object_type`] **must**  be a valid [`DebugReportObjectTypeEXT`] value
-  [`p_layer_prefix`] **must**  be a null-terminated UTF-8 string
-  [`p_message`] **must**  be a null-terminated UTF-8 string

# Related
- [`VK_EXT_debug_report`]
- [`DebugReportFlagsEXT`]
- [`DebugReportObjectTypeEXT`]
- [`Instance`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        