[VkAccelerationStructureMemoryRequirementsTypeNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureMemoryRequirementsTypeNV.html) - Acceleration structure memory requirement type

# C Specifications
Possible values of `type` in
[`AccelerationStructureMemoryRequirementsInfoNV`] are:,
```c
// Provided by VK_NV_ray_tracing
typedef enum VkAccelerationStructureMemoryRequirementsTypeNV {
    VK_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_OBJECT_NV = 0,
    VK_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_BUILD_SCRATCH_NV = 1,
    VK_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_UPDATE_SCRATCH_NV = 2,
} VkAccelerationStructureMemoryRequirementsTypeNV;
```

# Description
- [`VK_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_NV`] requests the memory requirement for the [`AccelerationStructureNV`] backing store.
- [`VK_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_NV`] requests the memory requirement for scratch space during the initial build.
- [`VK_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_NV`] requests the memory requirement for scratch space during an update.

# Related
- [`nv_ray_tracing`]
- [`AccelerationStructureMemoryRequirementsInfoNV`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        