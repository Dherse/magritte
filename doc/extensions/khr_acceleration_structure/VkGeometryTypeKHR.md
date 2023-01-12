[VkGeometryTypeKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkGeometryTypeKHR.html) - Enum specifying which type of geometry is provided

# C Specifications
Geometry types are specified by [`GeometryTypeKHR`], which takes values:
```c
// Provided by VK_KHR_acceleration_structure
typedef enum VkGeometryTypeKHR {
    VK_GEOMETRY_TYPE_TRIANGLES_KHR = 0,
    VK_GEOMETRY_TYPE_AABBS_KHR = 1,
    VK_GEOMETRY_TYPE_INSTANCES_KHR = 2,
  // Provided by VK_NV_ray_tracing
    VK_GEOMETRY_TYPE_TRIANGLES_NV = VK_GEOMETRY_TYPE_TRIANGLES_KHR,
  // Provided by VK_NV_ray_tracing
    VK_GEOMETRY_TYPE_AABBS_NV = VK_GEOMETRY_TYPE_AABBS_KHR,
} VkGeometryTypeKHR;
```
or the equivalent
```c
// Provided by VK_NV_ray_tracing
typedef VkGeometryTypeKHR VkGeometryTypeNV;
```

# Description
- [`VK_GEOMETRY_TYPE_KHR`] specifies a geometry type consisting of triangles.
- [`VK_GEOMETRY_TYPE_KHR`] specifies a geometry type consisting of axis-aligned bounding boxes.
- [`VK_GEOMETRY_TYPE_KHR`] specifies a geometry type consisting of acceleration structure instances.

# Related
- [`khr_acceleration_structure`]
- [`nv_ray_tracing`]
- [`AccelerationStructureGeometryKHR`]
- [`GeometryNV`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        