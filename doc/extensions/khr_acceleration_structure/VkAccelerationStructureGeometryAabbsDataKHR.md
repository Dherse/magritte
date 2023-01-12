[VkAccelerationStructureGeometryAabbsDataKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureGeometryAabbsDataKHR.html) - Structure specifying axis-aligned bounding box geometry in a bottom-level acceleration structure

# C Specifications
The [`AccelerationStructureGeometryAabbsDataKHR`] structure is defined
as:
```c
// Provided by VK_KHR_acceleration_structure
typedef struct VkAccelerationStructureGeometryAabbsDataKHR {
    VkStructureType                  sType;
    const void*                      pNext;
    VkDeviceOrHostAddressConstKHR    data;
    VkDeviceSize                     stride;
} VkAccelerationStructureGeometryAabbsDataKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`data`] is a device or host address to memory containing [`AabbPositionsKHR`] structures containing position data for each axis-aligned bounding box in the geometry.
- [`stride`] is the stride in bytes between each entry in [`data`]. The stride  **must**  be a multiple of `8`.

# Description
## Valid Usage
-  [`stride`] **must**  be a multiple of `8`
-  [`stride`] **must**  be less than or equal to 2<sup>32</sup>-1

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_GEOMETRY_AABBS_DATA_KHR`
-  [`p_next`] **must**  be `NULL`

# Related
- [`khr_acceleration_structure`]
- [`AccelerationStructureGeometryDataKHR`]
- [`DeviceOrHostAddressConstKHR`]
- [`DeviceSize`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        