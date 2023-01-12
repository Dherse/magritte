[VkAccelerationStructureMemoryRequirementsInfoNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureMemoryRequirementsInfoNV.html) - Structure specifying acceleration to query for memory requirements

# C Specifications
The [`AccelerationStructureMemoryRequirementsInfoNV`] structure is
defined as:
```c
// Provided by VK_NV_ray_tracing
typedef struct VkAccelerationStructureMemoryRequirementsInfoNV {
    VkStructureType                                    sType;
    const void*                                        pNext;
    VkAccelerationStructureMemoryRequirementsTypeNV    type;
    VkAccelerationStructureNV                          accelerationStructure;
} VkAccelerationStructureMemoryRequirementsInfoNV;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`type_`] selects the type of memory requirement being queried. `VK_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_OBJECT_NV` returns the memory requirements for the object itself. `VK_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_BUILD_SCRATCH_NV` returns the memory requirements for the scratch memory when doing a build. `VK_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_UPDATE_SCRATCH_NV` returns the memory requirements for the scratch memory when doing an update.
- [`acceleration_structure`] is the acceleration structure to be queried for memory requirements.

# Description
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_INFO_NV`
-  [`p_next`] **must**  be `NULL`
-  [`type_`] **must**  be a valid [`AccelerationStructureMemoryRequirementsTypeNV`] value
-  [`acceleration_structure`] **must**  be a valid [`AccelerationStructureNV`] handle

# Related
- [`nv_ray_tracing`]
- [`AccelerationStructureMemoryRequirementsTypeNV`]
- [`AccelerationStructureNV`]
- [`StructureType`]
- [`get_acceleration_structure_memory_requirements_nv`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        