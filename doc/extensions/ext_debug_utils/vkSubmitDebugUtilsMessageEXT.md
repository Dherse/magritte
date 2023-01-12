[vkSubmitDebugUtilsMessageEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSubmitDebugUtilsMessageEXT.html) - Inject a message into a debug stream

# C Specifications
There may be times that a user wishes to intentionally submit a debug
message.
To do this, call:
```c
// Provided by VK_EXT_debug_utils
void vkSubmitDebugUtilsMessageEXT(
    VkInstance                                  instance,
    VkDebugUtilsMessageSeverityFlagBitsEXT      messageSeverity,
    VkDebugUtilsMessageTypeFlagsEXT             messageTypes,
    const VkDebugUtilsMessengerCallbackDataEXT* pCallbackData);
```

# Parameters
- [`instance`] is the debug stream’s [`Instance`].
- [`message_severity`] is a [`DebugUtilsMessageSeverityFlagBitsEXT`] value specifying the severity of this event/message.
- [`message_types`] is a bitmask of [`DebugUtilsMessageTypeFlagBitsEXT`] specifying which type of event(s) to identify with this message.
- [`p_callback_data`] contains all the callback related data in the [`DebugUtilsMessengerCallbackDataEXT`] structure.

# Description
The call will propagate through the layers and generate callback(s) as
indicated by the message’s flags.
The parameters are passed on to the callback in addition to the
`pUserData` value that was defined at the time the messenger was
registered.
## Valid Usage
-    The `objectType` member of each element of `pCallbackData->pObjects` **must**  not be `VK_OBJECT_TYPE_UNKNOWN`

## Valid Usage (Implicit)
-  [`instance`] **must**  be a valid [`Instance`] handle
-  [`message_severity`] **must**  be a valid [`DebugUtilsMessageSeverityFlagBitsEXT`] value
-  [`message_types`] **must**  be a valid combination of [`DebugUtilsMessageTypeFlagBitsEXT`] values
-  [`message_types`] **must**  not be `0`
-  [`p_callback_data`] **must**  be a valid pointer to a valid [`DebugUtilsMessengerCallbackDataEXT`] structure

# Related
- [`ext_debug_utils`]
- [`DebugUtilsMessageSeverityFlagBitsEXT`]
- [VkDebugUtilsMessageTypeFlagsEXT]()
- [`DebugUtilsMessengerCallbackDataEXT`]
- [`Instance`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        