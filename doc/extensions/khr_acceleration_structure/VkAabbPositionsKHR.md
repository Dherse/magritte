[VkAabbPositionsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAabbPositionsKHR.html) - Structure specifying two opposing corners of an axis-aligned bounding box

# C Specifications
The [`AabbPositionsKHR`] structure is defined as:
```c
// Provided by VK_KHR_acceleration_structure
typedef struct VkAabbPositionsKHR {
    float    minX;
    float    minY;
    float    minZ;
    float    maxX;
    float    maxY;
    float    maxZ;
} VkAabbPositionsKHR;
```
or the equivalent
```c
// Provided by VK_NV_ray_tracing
typedef VkAabbPositionsKHR VkAabbPositionsNV;
```

# Members
- [`min_x`] is the x position of one opposing corner of a bounding box.
- [`min_y`] is the y position of one opposing corner of a bounding box.
- [`min_z`] is the z position of one opposing corner of a bounding box.
- [`max_x`] is the x position of the other opposing corner of a bounding box.
- [`max_y`] is the y position of the other opposing corner of a bounding box.
- [`max_z`] is the z position of the other opposing corner of a bounding box.

# Description
## Valid Usage
-  [`min_x`] **must**  be less than or equal to [`max_x`]
-  [`min_y`] **must**  be less than or equal to [`max_y`]
-  [`min_z`] **must**  be less than or equal to [`max_z`]

# Related
- [`khr_acceleration_structure`]
- [`nv_ray_tracing`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        