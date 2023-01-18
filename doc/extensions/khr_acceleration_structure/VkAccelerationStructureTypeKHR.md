[VkAccelerationStructureTypeKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureTypeKHR.html) - Type of acceleration structure

# C Specifications
Values which  **can**  be set in
[`AccelerationStructureCreateInfoKHR::type_`]
or
[`AccelerationStructureInfoNV::type_`]
specifying the type of acceleration structure, are:
```c
// Provided by VK_KHR_acceleration_structure
typedef enum VkAccelerationStructureTypeKHR {
    VK_ACCELERATION_STRUCTURE_TYPE_TOP_LEVEL_KHR = 0,
    VK_ACCELERATION_STRUCTURE_TYPE_BOTTOM_LEVEL_KHR = 1,
    VK_ACCELERATION_STRUCTURE_TYPE_GENERIC_KHR = 2,
  // Provided by VK_NV_ray_tracing
    VK_ACCELERATION_STRUCTURE_TYPE_TOP_LEVEL_NV = VK_ACCELERATION_STRUCTURE_TYPE_TOP_LEVEL_KHR,
  // Provided by VK_NV_ray_tracing
    VK_ACCELERATION_STRUCTURE_TYPE_BOTTOM_LEVEL_NV = VK_ACCELERATION_STRUCTURE_TYPE_BOTTOM_LEVEL_KHR,
} VkAccelerationStructureTypeKHR;
```
or the equivalent
```c
// Provided by VK_NV_ray_tracing
typedef VkAccelerationStructureTypeKHR VkAccelerationStructureTypeNV;
```

# Description
- [`TOP_LEVEL`] is a top-level acceleration structure containing instance data referring to bottom-level acceleration structures.
- [`BOTTOM_LEVEL`] is a bottom-level acceleration structure containing the AABBs or geometry to be intersected.
- [`GENERIC`] is an acceleration structure whose type is determined at build time used for special circumstances.

# Related
- [`VK_KHR_acceleration_structure`]
- [`VK_NV_ray_tracing`]
- [`AccelerationStructureBuildGeometryInfoKHR`]
- [`AccelerationStructureCreateInfoKHR`]
- [`AccelerationStructureInfoNV`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        