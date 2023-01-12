[VkDeviceDeviceMemoryReportCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceDeviceMemoryReportCreateInfoEXT.html) - Register device memory report callbacks for a Vulkan device

# C Specifications
To register callbacks for underlying device memory events of type
[`DeviceMemoryReportEventTypeEXT`], add one or multiple
[`DeviceDeviceMemoryReportCreateInfoEXT`] structures to the [`p_next`]
chain of the [`DeviceCreateInfo`] structure.
```c
// Provided by VK_EXT_device_memory_report
typedef struct VkDeviceDeviceMemoryReportCreateInfoEXT {
    VkStructureType                        sType;
    const void*                            pNext;
    VkDeviceMemoryReportFlagsEXT           flags;
    PFN_vkDeviceMemoryReportCallbackEXT    pfnUserCallback;
    void*                                  pUserData;
} VkDeviceDeviceMemoryReportCreateInfoEXT;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`flags`] is 0 and reserved for future use.
- [`pfn_user_callback`] is the application callback function to call.
- [`user_data`] is user data to be passed to the callback.

# Description
The callback  **may**  be called from multiple threads simultaneously.The callback  **must**  be called only once by the implementation when a
[`DeviceMemoryReportEventTypeEXT`] event occurs.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DEVICE_DEVICE_MEMORY_REPORT_CREATE_INFO_EXT`
-  [`flags`] **must**  be `0`
-  [`pfn_user_callback`] **must**  be a valid [PFN_vkDeviceMemoryReportCallbackEXT]() value
-  [`user_data`] **must**  be a pointer value

# Related
- [PFN_vkDeviceMemoryReportCallbackEXT]()
- [`ext_device_memory_report`]
- [`DeviceMemoryReportFlagsEXT`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        