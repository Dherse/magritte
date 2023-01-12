[VkCoarseSampleLocationNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCoarseSampleLocationNV.html) - Structure specifying parameters controlling shading rate image usage

# C Specifications
The [`CoarseSampleLocationNV`] structure identifies a specific pixel and
[sample index](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-multisampling-coverage-mask) for one of the
coverage samples in a fragment that is larger than one pixel.
This structure is defined as:
```c
// Provided by VK_NV_shading_rate_image
typedef struct VkCoarseSampleLocationNV {
    uint32_t    pixelX;
    uint32_t    pixelY;
    uint32_t    sample;
} VkCoarseSampleLocationNV;
```

# Members
- [`pixel_x`] is added to the x coordinate of the upper-leftmost pixel of each fragment to identify the pixel containing the coverage sample.
- [`pixel_y`] is added to the y coordinate of the upper-leftmost pixel of each fragment to identify the pixel containing the coverage sample.
- [`sample`] is the number of the coverage sample in the pixel identified by [`pixel_x`] and [`pixel_y`].

# Description
## Valid Usage
-  [`pixel_x`] **must**  be less than the width (in pixels) of the fragment
-  [`pixel_y`] **must**  be less than the height (in pixels) of the fragment
-  [`sample`] **must**  be less than the number of coverage samples in each pixel belonging to the fragment

# Related
- [`nv_shading_rate_image`]
- [`CoarseSampleOrderCustomNV`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        