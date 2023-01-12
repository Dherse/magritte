[VkAccelerationStructureBuildSizesInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureBuildSizesInfoKHR.html) - Structure specifying build sizes for an acceleration structure

# C Specifications
The [`AccelerationStructureBuildSizesInfoKHR`] structure describes the
required build sizes for an acceleration structure and scratch buffers and
is defined as:
```c
// Provided by VK_KHR_acceleration_structure
typedef struct VkAccelerationStructureBuildSizesInfoKHR {
    VkStructureType    sType;
    const void*        pNext;
    VkDeviceSize       accelerationStructureSize;
    VkDeviceSize       updateScratchSize;
    VkDeviceSize       buildScratchSize;
} VkAccelerationStructureBuildSizesInfoKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`acceleration_structure_size`] is the size in bytes required in a [`AccelerationStructureKHR`] for a build or update operation.
- [`update_scratch_size`] is the size in bytes required in a scratch buffer for an update operation.
- [`build_scratch_size`] is the size in bytes required in a scratch buffer for a build operation.

# Description
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_BUILD_SIZES_INFO_KHR`
-  [`p_next`] **must**  be `NULL`

# Related
- [`khr_acceleration_structure`]
- [`DeviceSize`]
- [`StructureType`]
- [`get_acceleration_structure_build_sizes_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        