[VkDeviceEventInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceEventInfoEXT.html) - Describe a device event to create

# C Specifications
The [`DeviceEventInfoEXT`] structure is defined as:
```c
// Provided by VK_EXT_display_control
typedef struct VkDeviceEventInfoEXT {
    VkStructureType         sType;
    const void*             pNext;
    VkDeviceEventTypeEXT    deviceEvent;
} VkDeviceEventInfoEXT;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- `device` is a [`DeviceEventTypeEXT`] value specifying when the fence will be signaled.

# Description
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DEVICE_EVENT_INFO_EXT`
-  [`p_next`] **must**  be `NULL`
-  [`device_event`] **must**  be a valid [`DeviceEventTypeEXT`] value

# Related
- [`VK_EXT_display_control`]
- [`DeviceEventTypeEXT`]
- [`StructureType`]
- [`register_device_event_ext`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        