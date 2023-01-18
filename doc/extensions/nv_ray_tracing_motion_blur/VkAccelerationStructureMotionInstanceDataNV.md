[VkAccelerationStructureMotionInstanceDataNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureMotionInstanceDataNV.html) - Union specifying a acceleration structure motion instance data for building into an acceleration structure geometry

# C Specifications
Acceleration structure motion instance is defined by the union:
```c
// Provided by VK_NV_ray_tracing_motion_blur
typedef union VkAccelerationStructureMotionInstanceDataNV {
    VkAccelerationStructureInstanceKHR               staticInstance;
    VkAccelerationStructureMatrixMotionInstanceNV    matrixMotionInstance;
    VkAccelerationStructureSRTMotionInstanceNV       srtMotionInstance;
} VkAccelerationStructureMotionInstanceDataNV;
```

# Members
- [`static_instance`] is a [`AccelerationStructureInstanceKHR`] structure containing data for a static instance.
- [`matrix_motion_instance`] is a [`AccelerationStructureMatrixMotionInstanceNV`] structure containing data for a matrix motion instance.
- [`srt_motion_instance`] is a [`AccelerationStructureSrtMotionInstanceNV`] structure containing data for an SRT motion instance.

# Related
- [`VK_NV_ray_tracing_motion_blur`]
- [`AccelerationStructureInstanceKHR`]
- [`AccelerationStructureMatrixMotionInstanceNV`]
- [`AccelerationStructureMotionInstanceNV`]
- [`AccelerationStructureSrtMotionInstanceNV`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        