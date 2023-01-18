[VkAccelerationStructureMatrixMotionInstanceNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureMatrixMotionInstanceNV.html) - Structure specifying a single acceleration structure matrix motion instance for building into an acceleration structure geometry

# C Specifications
An acceleration structure matrix motion instance is defined by the
structure:
```c
// Provided by VK_NV_ray_tracing_motion_blur
typedef struct VkAccelerationStructureMatrixMotionInstanceNV {
    VkTransformMatrixKHR          transformT0;
    VkTransformMatrixKHR          transformT1;
    uint32_t                      instanceCustomIndex:24;
    uint32_t                      mask:8;
    uint32_t                      instanceShaderBindingTableRecordOffset:24;
    VkGeometryInstanceFlagsKHR    flags:8;
    uint64_t                      accelerationStructureReference;
} VkAccelerationStructureMatrixMotionInstanceNV;
```

# Members
- [`transform_t0`] is a [`TransformMatrixKHR`] structure describing a transformation to be applied to the acceleration structure at time 0.
- [`transform_t1`] is a [`TransformMatrixKHR`] structure describing a transformation to be applied to the acceleration structure at time 1.
- [`instance_custom_index`] is a 24-bit user-specified index value accessible to ray shaders in the `InstanceCustomIndexKHR` built-in.
- [`mask`] is an 8-bit visibility mask for the geometry. The instance  **may**  only be hit if `Cull Mask & instance.mask != 0`
- [`instance_shader_binding_table_record_offset`] is a 24-bit offset used in calculating the hit shader binding table index.
- [`flags`] is an 8-bit mask of [`GeometryInstanceFlagBitsKHR`] values to apply to this instance.
- [`acceleration_structure_reference`] is either:  - a device address containing the value obtained from [`get_acceleration_structure_device_address_khr`] or [`get_acceleration_structure_handle_nv`]      (used by device operations which reference acceleration structures) or,  - a [`AccelerationStructureKHR`] object (used by host operations which reference acceleration structures).

# Description
The C language specification does not define the ordering of bit-fields, but
in practice, this struct produces the correct layout with existing
compilers.
The intended bit pattern is for the following:
- [`instance_custom_index`] and [`mask`] occupy the same memory as if a single `uint32_t` was specified in their place  - [`instance_custom_index`] occupies the 24 least significant bits of that memory  - [`mask`] occupies the 8 most significant bits of that memory 
- [`instance_shader_binding_table_record_offset`] and [`flags`] occupy the same memory as if a single `uint32_t` was specified in their place  - [`instance_shader_binding_table_record_offset`] occupies the 24 least significant bits of that memory  - [`flags`] occupies the 8 most significant bits of that memory 
If a compiler produces code that diverges from that pattern, applications
 **must**  employ another method to set values according to the correct bit
pattern.The transform for a matrix motion instance at a point in time is derived by
component-wise linear interpolation of the two transforms.
That is, for a `time` in [0,1] the resulting transform is
* [`transform_t0`] × (1 - `time`) +  [`transform_t1`] × `time`

## Valid Usage (Implicit)
-  [`flags`] **must**  be a valid combination of [`GeometryInstanceFlagBitsKHR`] values

# Related
- [`VK_NV_ray_tracing_motion_blur`]
- [`AccelerationStructureMotionInstanceDataNV`]
- [`GeometryInstanceFlagsKHR`]
- [`TransformMatrixKHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        