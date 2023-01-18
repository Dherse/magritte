[PFN_vkDeviceMemoryReportCallbackEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/PFN_vkDeviceMemoryReportCallbackEXT.html) - Application-defined device memory report callback function

# C Specifications
The prototype for the
[`DeviceDeviceMemoryReportCreateInfoEXT::pfn_user_callback`]
function implemented by the application is:
```c
// Provided by VK_EXT_device_memory_report
typedef void (VKAPI_PTR *PFN_vkDeviceMemoryReportCallbackEXT)(
    const VkDeviceMemoryReportCallbackDataEXT*  pCallbackData,
    void*                                       pUserData);
```

# Parameters
- [`p_callback_data`] contains all the callback related data in the [`DeviceMemoryReportCallbackDataEXT`] structure.
- [`p_user_data`] is the user data provided when the [`DeviceDeviceMemoryReportCreateInfoEXT`] was created.

# Description
The callback  **must**  not make calls to any Vulkan commands.

# Related
- [`VK_EXT_device_memory_report`]
- [`DeviceDeviceMemoryReportCreateInfoEXT`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        