[PFN_vkDebugReportCallbackEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/PFN_vkDebugReportCallbackEXT.html) - Application-defined debug report callback function

# C Specifications
The prototype for the
[`DebugReportCallbackCreateInfoEXT::pfn_callback`] function
implemented by the application is:
```c
// Provided by VK_EXT_debug_report
typedef VkBool32 (VKAPI_PTR *PFN_vkDebugReportCallbackEXT)(
    VkDebugReportFlagsEXT                       flags,
    VkDebugReportObjectTypeEXT                  objectType,
    uint64_t                                    object,
    size_t                                      location,
    int32_t                                     messageCode,
    const char*                                 pLayerPrefix,
    const char*                                 pMessage,
    void*                                       pUserData);
```

# Parameters
- [`flags`] specifies the [`DebugReportFlagBitsEXT`] that triggered this callback.
- [`object_type`] is a [`DebugReportObjectTypeEXT`] value specifying the type of object being used or created at the time the event was triggered.
- [`object`] is the object where the issue was detected. If [`object_type`] is `VK_DEBUG_REPORT_OBJECT_TYPE_UNKNOWN_EXT`, [`object`] is undefined.
- [`location`] is a component (layer, driver, loader) defined value specifying the *location* of the trigger. This is an  **optional**  value.
- [`message_code`] is a layer-defined value indicating what test triggered this callback.
- [`p_layer_prefix`] is a null-terminated string that is an abbreviation of the name of the component making the callback. [`p_layer_prefix`] is only valid for the duration of the callback.
- [`p_message`] is a null-terminated string detailing the trigger conditions. [`p_message`] is only valid for the duration of the callback.
- [`p_user_data`] is the user data given when the [`DebugReportCallbackEXT`] was created.

# Description
The callback  **must**  not call [`destroy_debug_report_callback_ext`].The callback returns a [`Bool32`], which is interpreted in a
layer-specified manner.
The application  **should**  always return [`FALSE`].
The [`TRUE`] value is reserved for use in layer development.[`object`] **must**  be a Vulkan object or [`crate::Handle::null`].
If [`object_type`] is not `VK_DEBUG_REPORT_OBJECT_TYPE_UNKNOWN_EXT` and
[`object`] is not [`crate::Handle::null`], [`object`] **must**  be a Vulkan
object of the corresponding type associated with [`object_type`] as defined
in [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#debug-report-object-types](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#debug-report-object-types).

# Related
- [`VK_EXT_debug_report`]
- [`DebugReportCallbackCreateInfoEXT`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        