[VkDebugUtilsMessengerCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDebugUtilsMessengerCreateInfoEXT.html) - Structure specifying parameters of a newly created debug messenger

# C Specifications
The definition of [`DebugUtilsMessengerCreateInfoEXT`] is:
```c
// Provided by VK_EXT_debug_utils
typedef struct VkDebugUtilsMessengerCreateInfoEXT {
    VkStructureType                         sType;
    const void*                             pNext;
    VkDebugUtilsMessengerCreateFlagsEXT     flags;
    VkDebugUtilsMessageSeverityFlagsEXT     messageSeverity;
    VkDebugUtilsMessageTypeFlagsEXT         messageType;
    PFN_vkDebugUtilsMessengerCallbackEXT    pfnUserCallback;
    void*                                   pUserData;
} VkDebugUtilsMessengerCreateInfoEXT;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`flags`] is `0` and is reserved for future use.
- [`message_severity`] is a bitmask of [`DebugUtilsMessageSeverityFlagBitsEXT`] specifying which severity of event(s) will cause this callback to be called.
- [`message_type`] is a bitmask of [`DebugUtilsMessageTypeFlagBitsEXT`] specifying which type of event(s) will cause this callback to be called.
- [`pfn_user_callback`] is the application callback function to call.
- [`user_data`] is user data to be passed to the callback.

# Description
For each [`DebugUtilsMessengerEXT`] that is created the
[`DebugUtilsMessengerCreateInfoEXT`]::[`message_severity`] and
[`DebugUtilsMessengerCreateInfoEXT`]::[`message_type`] determine when
that [`DebugUtilsMessengerCreateInfoEXT`]::[`pfn_user_callback`] is
called.
The process to determine if the user’s [`pfn_user_callback`] is triggered
when an event occurs is as follows:
0. The implementation will perform a bitwise AND of the event’s [`DebugUtilsMessageSeverityFlagBitsEXT`] with the [`message_severity`] provided during creation of the [`DebugUtilsMessengerEXT`] object.  0. If the value is 0, the message is skipped. 
2. The implementation will perform bitwise AND of the event’s [`DebugUtilsMessageTypeFlagBitsEXT`] with the [`message_type`] provided during the creation of the [`DebugUtilsMessengerEXT`] object.  0. If the value is 0, the message is skipped. 
4. The callback will trigger a debug message for the current event
The callback will come directly from the component that detected the event,
unless some other layer intercepts the calls for its own purposes (filter
them in a different way, log to a system error log, etc.).An application  **can**  receive multiple callbacks if multiple
[`DebugUtilsMessengerEXT`] objects are created.
A callback will always be executed in the same thread as the originating
Vulkan call.A callback  **can**  be called from multiple threads simultaneously (if the
application is making Vulkan calls from multiple threads).
## Valid Usage
-  [`pfn_user_callback`] **must**  be a valid [`PFNDebugUtilsMessengerCallbackEXT`]

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT`
-  [`flags`] **must**  be `0`
-  [`message_severity`] **must**  be a valid combination of [`DebugUtilsMessageSeverityFlagBitsEXT`] values
-  [`message_severity`] **must**  not be `0`
-  [`message_type`] **must**  be a valid combination of [`DebugUtilsMessageTypeFlagBitsEXT`] values
-  [`message_type`] **must**  not be `0`
-  [`pfn_user_callback`] **must**  be a valid [`PFNDebugUtilsMessengerCallbackEXT`] value

# Related
- [`PFNDebugUtilsMessengerCallbackEXT`]
- [`VK_EXT_debug_utils`]
- [`DebugUtilsMessageSeverityFlagsEXT`]
- [`DebugUtilsMessageTypeFlagsEXT`]
- [`DebugUtilsMessengerCreateFlagsEXT`]
- [`StructureType`]
- [`create_debug_utils_messenger_ext`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        