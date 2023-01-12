[VkSurfaceFormat2KHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSurfaceFormat2KHR.html) - Structure describing a supported swapchain format tuple

# C Specifications
The [`SurfaceFormat2KHR`] structure is defined as:
```c
// Provided by VK_KHR_get_surface_capabilities2
typedef struct VkSurfaceFormat2KHR {
    VkStructureType       sType;
    void*                 pNext;
    VkSurfaceFormatKHR    surfaceFormat;
} VkSurfaceFormat2KHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`surface_format`] is a [`SurfaceFormatKHR`] structure describing a format-color space pair that is compatible with the specified surface.

# Description
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_SURFACE_FORMAT_2_KHR`
-  [`p_next`] **must**  be `NULL`

# Related
- [`khr_get_surface_capabilities2`]
- [`StructureType`]
- [`SurfaceFormatKHR`]
- [`get_physical_device_surface_formats2_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        