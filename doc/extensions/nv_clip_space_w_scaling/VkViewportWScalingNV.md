[VkViewportWScalingNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkViewportWScalingNV.html) - Structure specifying a viewport

# C Specifications
The [`ViewportWScalingNV`] structure is defined as:
```c
// Provided by VK_NV_clip_space_w_scaling
typedef struct VkViewportWScalingNV {
    float    xcoeff;
    float    ycoeff;
} VkViewportWScalingNV;
```

# Members
- [`xcoeff`] and [`ycoeff`] are the viewport’s W scaling factor for x and y respectively.

# Related
- [`VK_NV_clip_space_w_scaling`]
- [`PipelineViewportWScalingStateCreateInfoNV`]
- [`cmd_set_viewport_w_scaling_nv`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        