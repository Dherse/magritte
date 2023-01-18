[VkAccelerationStructureMotionInfoNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureMotionInfoNV.html) - Structure specifying the parameters of a newly created acceleration structure object

# C Specifications
The [`AccelerationStructureMotionInfoNV`] structure is defined as:
```c
// Provided by VK_NV_ray_tracing_motion_blur
typedef struct VkAccelerationStructureMotionInfoNV {
    VkStructureType                             sType;
    const void*                                 pNext;
    uint32_t                                    maxInstances;
    VkAccelerationStructureMotionInfoFlagsNV    flags;
} VkAccelerationStructureMotionInfoNV;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`max_instances`] is the maximum number of instances that  **may**  be used in the motion top-level acceleration structure.
- [`flags`] is 0 and reserved for future use.

# Description
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_MOTION_INFO_NV`
-  [`flags`] **must**  be `0`

# Related
- [`VK_NV_ray_tracing_motion_blur`]
- [`AccelerationStructureMotionInfoFlagsNV`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        