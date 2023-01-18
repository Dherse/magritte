[VkAccelerationStructureMotionInstanceTypeNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureMotionInstanceTypeNV.html) - Enum specifying a type of acceleration structure motion instance data for building into an acceleration structure geometry

# C Specifications
The [`AccelerationStructureMotionInstanceTypeNV`] enumeration is defined
as:
```c
// Provided by VK_NV_ray_tracing_motion_blur
typedef enum VkAccelerationStructureMotionInstanceTypeNV {
    VK_ACCELERATION_STRUCTURE_MOTION_INSTANCE_TYPE_STATIC_NV = 0,
    VK_ACCELERATION_STRUCTURE_MOTION_INSTANCE_TYPE_MATRIX_MOTION_NV = 1,
    VK_ACCELERATION_STRUCTURE_MOTION_INSTANCE_TYPE_SRT_MOTION_NV = 2,
} VkAccelerationStructureMotionInstanceTypeNV;
```

# Description
- [`STATIC`] specifies that the instance is a static instance with no instance motion.
- [`MATRIX_MOTION`] specifies that the instance is a motion instance with motion specified by interpolation between two matrices.
- [`SRT_MOTION`] specifies that the instance is a motion instance with motion specified by interpolation in the SRT decomposition.

# Related
- [`VK_NV_ray_tracing_motion_blur`]
- [`AccelerationStructureMotionInstanceNV`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        