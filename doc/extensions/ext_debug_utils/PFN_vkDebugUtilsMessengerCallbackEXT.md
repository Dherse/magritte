[PFN_vkDebugUtilsMessengerCallbackEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/PFN_vkDebugUtilsMessengerCallbackEXT.html) - Application-defined debug messenger callback function

# C Specifications
The prototype for the
[`DebugUtilsMessengerCreateInfoEXT::pfn_user_callback`] function
implemented by the application is:
```c
// Provided by VK_EXT_debug_utils
typedef VkBool32 (VKAPI_PTR *PFN_vkDebugUtilsMessengerCallbackEXT)(
    VkDebugUtilsMessageSeverityFlagBitsEXT           messageSeverity,
    VkDebugUtilsMessageTypeFlagsEXT                  messageTypes,
    const VkDebugUtilsMessengerCallbackDataEXT*      pCallbackData,
    void*                                            pUserData);
```

# Parameters
- [`message_severity`] specifies the [`DebugUtilsMessageSeverityFlagBitsEXT`] that triggered this callback.
- [`message_types`] is a bitmask of [`DebugUtilsMessageTypeFlagBitsEXT`] specifying which type of event(s) triggered this callback.
- [`p_callback_data`] contains all the callback related data in the [`DebugUtilsMessengerCallbackDataEXT`] structure.
- [`p_user_data`] is the user data provided when the [`DebugUtilsMessengerEXT`] was created.

# Description
The callback returns a [`Bool32`], which is interpreted in a
layer-specified manner.
The application  **should**  always return [`FALSE`].
The [`TRUE`] value is reserved for use in layer development.
## Valid Usage
-    The callback  **must**  not make calls to any Vulkan commands

# Related
- [`VK_EXT_debug_utils`]
- [`DebugUtilsMessengerCreateInfoEXT`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        