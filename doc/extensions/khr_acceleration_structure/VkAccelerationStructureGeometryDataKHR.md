[VkAccelerationStructureGeometryDataKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureGeometryDataKHR.html) - Union specifying acceleration structure geometry data

# C Specifications
The [`AccelerationStructureGeometryDataKHR`] union is defined as:
```c
// Provided by VK_KHR_acceleration_structure
typedef union VkAccelerationStructureGeometryDataKHR {
    VkAccelerationStructureGeometryTrianglesDataKHR    triangles;
    VkAccelerationStructureGeometryAabbsDataKHR        aabbs;
    VkAccelerationStructureGeometryInstancesDataKHR    instances;
} VkAccelerationStructureGeometryDataKHR;
```

# Members
- [`triangles`] is a [`AccelerationStructureGeometryTrianglesDataKHR`] structure.
- [`aabbs`] is a [`AccelerationStructureGeometryAabbsDataKHR`] struture.
- [`instances`] is a [`AccelerationStructureGeometryInstancesDataKHR`] structure.

# Related
- [`VK_KHR_acceleration_structure`]
- [`AccelerationStructureGeometryAabbsDataKHR`]
- [`AccelerationStructureGeometryInstancesDataKHR`]
- [`AccelerationStructureGeometryKHR`]
- [`AccelerationStructureGeometryTrianglesDataKHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        