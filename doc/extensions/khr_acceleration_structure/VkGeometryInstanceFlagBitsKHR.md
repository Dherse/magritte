[VkGeometryInstanceFlagBitsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkGeometryInstanceFlagBitsKHR.html) - Instance flag bits

# C Specifications
Possible values of `flags` in the instance modifying the behavior of
that instance are:
```c
// Provided by VK_KHR_acceleration_structure
typedef enum VkGeometryInstanceFlagBitsKHR {
    VK_GEOMETRY_INSTANCE_TRIANGLE_FACING_CULL_DISABLE_BIT_KHR = 0x00000001,
    VK_GEOMETRY_INSTANCE_TRIANGLE_FLIP_FACING_BIT_KHR = 0x00000002,
    VK_GEOMETRY_INSTANCE_FORCE_OPAQUE_BIT_KHR = 0x00000004,
    VK_GEOMETRY_INSTANCE_FORCE_NO_OPAQUE_BIT_KHR = 0x00000008,
    VK_GEOMETRY_INSTANCE_TRIANGLE_FRONT_COUNTERCLOCKWISE_BIT_KHR = VK_GEOMETRY_INSTANCE_TRIANGLE_FLIP_FACING_BIT_KHR,
  // Provided by VK_NV_ray_tracing
    VK_GEOMETRY_INSTANCE_TRIANGLE_CULL_DISABLE_BIT_NV = VK_GEOMETRY_INSTANCE_TRIANGLE_FACING_CULL_DISABLE_BIT_KHR,
  // Provided by VK_NV_ray_tracing
    VK_GEOMETRY_INSTANCE_TRIANGLE_FRONT_COUNTERCLOCKWISE_BIT_NV = VK_GEOMETRY_INSTANCE_TRIANGLE_FRONT_COUNTERCLOCKWISE_BIT_KHR,
  // Provided by VK_NV_ray_tracing
    VK_GEOMETRY_INSTANCE_FORCE_OPAQUE_BIT_NV = VK_GEOMETRY_INSTANCE_FORCE_OPAQUE_BIT_KHR,
  // Provided by VK_NV_ray_tracing
    VK_GEOMETRY_INSTANCE_FORCE_NO_OPAQUE_BIT_NV = VK_GEOMETRY_INSTANCE_FORCE_NO_OPAQUE_BIT_KHR,
} VkGeometryInstanceFlagBitsKHR;
```
or the equivalent
```c
// Provided by VK_NV_ray_tracing
typedef VkGeometryInstanceFlagBitsKHR VkGeometryInstanceFlagBitsNV;
```

# Description
- [`VK_GEOMETRY_INSTANCE_FLAG_BITS_KHR`] disables face culling for this instance.
- [`VK_GEOMETRY_INSTANCE_FLAG_BITS_KHR`] indicates that the [facing determination](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#ray-traversal-culling-face) for geometry in this instance is inverted. Because the facing is determined in object space, an instance transform does not change the winding, but a geometry transform does.
- [`VK_GEOMETRY_INSTANCE_FLAG_BITS_KHR`] causes this instance to act as though `VK_GEOMETRY_OPAQUE_BIT_KHR` were specified on all geometries referenced by this instance. This behavior  **can**  be overridden by the SPIR-V `NoOpaqueKHR` ray flag.
- [`VK_GEOMETRY_INSTANCE_FLAG_BITS_KHR`] causes this instance to act as though `VK_GEOMETRY_OPAQUE_BIT_KHR` were not specified on all geometries referenced by this instance. This behavior  **can**  be overridden by the SPIR-V `OpaqueKHR` ray flag.
[`VK_GEOMETRY_INSTANCE_FLAG_BITS_KHR`] and
[`VK_GEOMETRY_INSTANCE_FLAG_BITS_KHR`] **must**  not be used in the
same flag.

# Related
- [`khr_acceleration_structure`]
- [`nv_ray_tracing`]
- [VkGeometryInstanceFlagsKHR]()

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        