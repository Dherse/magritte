[VkAccelerationStructureGeometryInstancesDataKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureGeometryInstancesDataKHR.html) - Structure specifying a geometry consisting of instances of other acceleration structures

# C Specifications
The [`AccelerationStructureGeometryInstancesDataKHR`] structure is
defined as:
```c
// Provided by VK_KHR_acceleration_structure
typedef struct VkAccelerationStructureGeometryInstancesDataKHR {
    VkStructureType                  sType;
    const void*                      pNext;
    VkBool32                         arrayOfPointers;
    VkDeviceOrHostAddressConstKHR    data;
} VkAccelerationStructureGeometryInstancesDataKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`array_of_pointers`] specifies whether [`data`] is used as an array of addresses or just an array.
- [`data`] is either the address of an array of device or host addresses referencing individual [`AccelerationStructureInstanceKHR`] structures or packed motion instance information as described in [motion instances](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#acceleration-structure-motion-instances) if [`array_of_pointers`] is `VK_TRUE`, or the address of an array of [`AccelerationStructureInstanceKHR`] or [`AccelerationStructureMotionInstanceNV`] structures. Addresses and [`AccelerationStructureInstanceKHR`] structures are tightly packed. [`AccelerationStructureMotionInstanceNV`] structures have a stride of 160 bytes.

# Description
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_GEOMETRY_INSTANCES_DATA_KHR`
-  [`p_next`] **must**  be `NULL`

# Related
- [`khr_acceleration_structure`]
- [`AccelerationStructureGeometryDataKHR`]
- [`Bool32`]
- [`DeviceOrHostAddressConstKHR`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        