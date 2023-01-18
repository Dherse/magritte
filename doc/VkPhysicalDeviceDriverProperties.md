[VkPhysicalDeviceDriverProperties](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDriverProperties.html) - Structure containing driver identification information

# C Specifications
The [`PhysicalDeviceDriverProperties`] structure is defined as:
```c
// Provided by VK_VERSION_1_2
typedef struct VkPhysicalDeviceDriverProperties {
    VkStructureType         sType;
    void*                   pNext;
    VkDriverId              driverID;
    char                    driverName[VK_MAX_DRIVER_NAME_SIZE];
    char                    driverInfo[VK_MAX_DRIVER_INFO_SIZE];
    VkConformanceVersion    conformanceVersion;
} VkPhysicalDeviceDriverProperties;
```
or the equivalent
```c
// Provided by VK_KHR_driver_properties
typedef VkPhysicalDeviceDriverProperties VkPhysicalDeviceDriverPropertiesKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.

# Description
- [`driver_id`] is a unique identifier for the driver of the physical device.
- [`driver_name`] is an array of [`MAX_DRIVER_NAME_SIZE`]`char` containing a null-terminated UTF-8 string which is the name of the driver.
- [`driver_info`] is an array of [`MAX_DRIVER_INFO_SIZE`]`char` containing a null-terminated UTF-8 string with additional information about the driver.
- [`conformance_version`] is the version of the Vulkan conformance test this driver is conformant against (see [`ConformanceVersion`]).
If the [`PhysicalDeviceDriverProperties`] structure is included in the [`p_next`] chain of the
[`PhysicalDeviceProperties2`] structure passed to
[`get_physical_device_properties2`], it is filled in with each
corresponding implementation-dependent property.These are properties of the driver corresponding to a physical device.[`driver_id`] **must**  be immutable for a given driver across instances,
processes, driver versions, and system reboots.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DRIVER_PROPERTIES`

# Related
- [`VK_KHR_driver_properties`]
- [`crate::vulkan1_2`]
- [`ConformanceVersion`]
- [`DriverId`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        