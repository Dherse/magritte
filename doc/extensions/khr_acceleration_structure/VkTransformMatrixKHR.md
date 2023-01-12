[VkTransformMatrixKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkTransformMatrixKHR.html) - Structure specifying a 3x4 affine transformation matrix

# C Specifications
The [`TransformMatrixKHR`] structure is defined as:
```c
// Provided by VK_KHR_acceleration_structure
typedef struct VkTransformMatrixKHR {
    float    matrix[3][4];
} VkTransformMatrixKHR;
```
or the equivalent
```c
// Provided by VK_NV_ray_tracing
typedef VkTransformMatrixKHR VkTransformMatrixNV;
```

# Members
- [`matrix`] is a 3x4 row-major affine transformation matrix.

# Description
## Valid Usage
-    The first three columns of [`matrix`] **must**  define an invertible 3x3 matrix

# Related
- [`khr_acceleration_structure`]
- [`nv_ray_tracing`]
- [`AccelerationStructureInstanceKHR`]
- [`AccelerationStructureMatrixMotionInstanceNV`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        