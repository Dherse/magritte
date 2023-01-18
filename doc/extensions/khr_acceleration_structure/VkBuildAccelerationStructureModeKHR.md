[VkBuildAccelerationStructureModeKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBuildAccelerationStructureModeKHR.html) - Enum specifying the type of build operation to perform

# C Specifications
The [`BuildAccelerationStructureModeKHR`] enumeration is defined as:
```c
// Provided by VK_KHR_acceleration_structure
typedef enum VkBuildAccelerationStructureModeKHR {
    VK_BUILD_ACCELERATION_STRUCTURE_MODE_BUILD_KHR = 0,
    VK_BUILD_ACCELERATION_STRUCTURE_MODE_UPDATE_KHR = 1,
} VkBuildAccelerationStructureModeKHR;
```

# Description
- [`BUILD`] specifies that the destination acceleration structure will be built using the specified geometries.
- [`UPDATE`] specifies that the destination acceleration structure will be built using data in a source acceleration structure, updated by the specified geometries.

# Related
- [`VK_KHR_acceleration_structure`]
- [`AccelerationStructureBuildGeometryInfoKHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        