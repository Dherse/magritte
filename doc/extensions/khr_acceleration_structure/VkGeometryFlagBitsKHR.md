[VkGeometryFlagBitsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkGeometryFlagBitsKHR.html) - Bitmask specifying additional parameters for a geometry

# C Specifications
Bits specifying additional parameters for geometries in acceleration
structure builds, are:
```c
// Provided by VK_KHR_acceleration_structure
typedef enum VkGeometryFlagBitsKHR {
    VK_GEOMETRY_OPAQUE_BIT_KHR = 0x00000001,
    VK_GEOMETRY_NO_DUPLICATE_ANY_HIT_INVOCATION_BIT_KHR = 0x00000002,
  // Provided by VK_NV_ray_tracing
    VK_GEOMETRY_OPAQUE_BIT_NV = VK_GEOMETRY_OPAQUE_BIT_KHR,
  // Provided by VK_NV_ray_tracing
    VK_GEOMETRY_NO_DUPLICATE_ANY_HIT_INVOCATION_BIT_NV = VK_GEOMETRY_NO_DUPLICATE_ANY_HIT_INVOCATION_BIT_KHR,
} VkGeometryFlagBitsKHR;
```
or the equivalent
```c
// Provided by VK_NV_ray_tracing
typedef VkGeometryFlagBitsKHR VkGeometryFlagBitsNV;
```

# Description
- [`VK_GEOMETRY_FLAG_BITS_KHR`] indicates that this geometry does not invoke the any-hit shaders even if present in a hit group.
- [`VK_GEOMETRY_FLAG_BITS_KHR`] indicates that the implementation  **must**  only call the any-hit shader a single time for each primitive in this geometry. If this bit is absent an implementation  **may**  invoke the any-hit shader more than once for this geometry.

# Related
- [`khr_acceleration_structure`]
- [`nv_ray_tracing`]
- [VkGeometryFlagsKHR]()

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        