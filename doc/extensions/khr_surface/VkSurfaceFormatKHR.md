[VkSurfaceFormatKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSurfaceFormatKHR.html) - Structure describing a supported swapchain format-color space pair

# C Specifications
The [`SurfaceFormatKHR`] structure is defined as:
```c
// Provided by VK_KHR_surface
typedef struct VkSurfaceFormatKHR {
    VkFormat           format;
    VkColorSpaceKHR    colorSpace;
} VkSurfaceFormatKHR;
```

# Members
- [`format`] is a [`Format`] that is compatible with the specified surface.
- [`color_space`] is a presentation [`ColorSpaceKHR`] that is compatible with the surface.

# Related
- [`VK_KHR_surface`]
- [`ColorSpaceKHR`]
- [`Format`]
- [`SurfaceFormat2KHR`]
- [`get_physical_device_surface_formats_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        