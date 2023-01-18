[VkPhysicalDeviceMaintenance4Properties](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMaintenance4Properties.html) - Structure describing various implementation-defined properties introduced with VK_KHR_maintenance4

# C Specifications
The [`PhysicalDeviceMaintenance4Properties`] structure is defined as:
```c
// Provided by VK_VERSION_1_3
typedef struct VkPhysicalDeviceMaintenance4Properties {
    VkStructureType    sType;
    void*              pNext;
    VkDeviceSize       maxBufferSize;
} VkPhysicalDeviceMaintenance4Properties;
```
or the equivalent
```c
// Provided by VK_KHR_maintenance4
typedef VkPhysicalDeviceMaintenance4Properties VkPhysicalDeviceMaintenance4PropertiesKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.

# Description
- [`max_buffer_size`] is the maximum size [`Buffer`] that  **can**  be created.
If the [`PhysicalDeviceMaintenance4Properties`] structure is included in the [`p_next`] chain of the
[`PhysicalDeviceProperties2`] structure passed to
[`get_physical_device_properties2`], it is filled in with each
corresponding implementation-dependent property.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_4_PROPERTIES`

# Related
- [`VK_KHR_maintenance4`]
- [`crate::vulkan1_3`]
- [`DeviceSize`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        