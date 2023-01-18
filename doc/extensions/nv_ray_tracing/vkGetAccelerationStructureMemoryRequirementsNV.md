[vkGetAccelerationStructureMemoryRequirementsNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetAccelerationStructureMemoryRequirementsNV.html) - Get acceleration structure memory requirements

# C Specifications
An acceleration structure has memory requirements for the structure object
itself, scratch space for the build, and scratch space for the update.Scratch space is allocated as a [`Buffer`], so for
`VK_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_BUILD_SCRATCH_NV`
and
`VK_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_UPDATE_SCRATCH_NV`
the `pMemoryRequirements->alignment` and
`pMemoryRequirements->memoryTypeBits` values returned by this call  **must** 
be filled with zero, and  **should**  be ignored by the application.To query the memory requirements, call:
```c
// Provided by VK_NV_ray_tracing
void vkGetAccelerationStructureMemoryRequirementsNV(
    VkDevice                                    device,
    const VkAccelerationStructureMemoryRequirementsInfoNV* pInfo,
    VkMemoryRequirements2KHR*                   pMemoryRequirements);
```

# Parameters
- [`device`] is the logical device on which the acceleration structure was created.
- [`p_info`] is a pointer to a [`AccelerationStructureMemoryRequirementsInfoNV`] structure specifying the acceleration structure to get memory requirements for.
- [`p_memory_requirements`] is a pointer to a [`MemoryRequirements2KHR`] structure in which the requested acceleration structure memory requirements are returned.

# Description
## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`p_info`] **must**  be a valid pointer to a valid [`AccelerationStructureMemoryRequirementsInfoNV`] structure
-  [`p_memory_requirements`] **must**  be a valid pointer to a [`MemoryRequirements2KHR`] structure

# Related
- [`VK_NV_ray_tracing`]
- [`AccelerationStructureMemoryRequirementsInfoNV`]
- [`Device`]
- [`MemoryRequirements2KHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        