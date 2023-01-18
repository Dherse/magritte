[VkDeviceMemoryReportEventTypeEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceMemoryReportEventTypeEXT.html) - Events that can occur on a device memory object

# C Specifications
Possible values of [`DeviceMemoryReportCallbackDataEXT::type_`],
specifying event types which cause the device driver to call the callback,
are:
```c
// Provided by VK_EXT_device_memory_report
typedef enum VkDeviceMemoryReportEventTypeEXT {
    VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_ALLOCATE_EXT = 0,
    VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_FREE_EXT = 1,
    VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_IMPORT_EXT = 2,
    VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_UNIMPORT_EXT = 3,
    VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_ALLOCATION_FAILED_EXT = 4,
} VkDeviceMemoryReportEventTypeEXT;
```

# Description
- [`ALLOCATE`] specifies this event corresponds to the allocation of an internal device memory object or a [`DeviceMemory`].
- [`FREE`] specifies this event corresponds to the deallocation of an internally-allocated device memory object or a [`DeviceMemory`].
- [`IMPORT`] specifies this event corresponds to the import of an external memory object.
- [`UNIMPORT`] specifies this event is the release of an imported external memory object.
- [`ALLOCATION_FAILED`] specifies this event corresponds to the failed allocation of an internal device memory object or a [`DeviceMemory`].

# Related
- [`VK_EXT_device_memory_report`]
- [`DeviceMemoryReportCallbackDataEXT`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        