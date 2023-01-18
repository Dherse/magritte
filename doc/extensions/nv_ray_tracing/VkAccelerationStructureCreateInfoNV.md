[VkAccelerationStructureCreateInfoNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureCreateInfoNV.html) - Structure specifying the parameters of a newly created acceleration structure object

# C Specifications
The [`AccelerationStructureCreateInfoNV`] structure is defined as:
```c
// Provided by VK_NV_ray_tracing
typedef struct VkAccelerationStructureCreateInfoNV {
    VkStructureType                  sType;
    const void*                      pNext;
    VkDeviceSize                     compactedSize;
    VkAccelerationStructureInfoNV    info;
} VkAccelerationStructureCreateInfoNV;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`compacted_size`] is the size from the result of [`cmd_write_acceleration_structures_properties_nv`] if this acceleration structure is going to be the target of a compacting copy.
- [`info`] is the [`AccelerationStructureInfoNV`] structure specifying further parameters of the created acceleration structure.

# Description
## Valid Usage
-    If [`compacted_size`] is not `0` then both `info.geometryCount` and `info.instanceCount` **must**  be `0`

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_CREATE_INFO_NV`
-  [`p_next`] **must**  be `NULL`
-  [`info`] **must**  be a valid [`AccelerationStructureInfoNV`] structure

# Related
- [`VK_NV_ray_tracing`]
- [`AccelerationStructureInfoNV`]
- [`DeviceSize`]
- [`StructureType`]
- [`create_acceleration_structure_nv`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        