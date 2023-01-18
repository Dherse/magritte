[VkAccelerationStructureDeviceAddressInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureDeviceAddressInfoKHR.html) - Structure specifying the acceleration structure to query an address for

# C Specifications
The [`AccelerationStructureDeviceAddressInfoKHR`] structure is defined
as:
```c
// Provided by VK_KHR_acceleration_structure
typedef struct VkAccelerationStructureDeviceAddressInfoKHR {
    VkStructureType               sType;
    const void*                   pNext;
    VkAccelerationStructureKHR    accelerationStructure;
} VkAccelerationStructureDeviceAddressInfoKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`acceleration_structure`] specifies the acceleration structure whose address is being queried.

# Description
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_DEVICE_ADDRESS_INFO_KHR`
-  [`p_next`] **must**  be `NULL`
-  [`acceleration_structure`] **must**  be a valid [`AccelerationStructureKHR`] handle

# Related
- [`VK_KHR_acceleration_structure`]
- [`AccelerationStructureKHR`]
- [`StructureType`]
- [`get_acceleration_structure_device_address_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        