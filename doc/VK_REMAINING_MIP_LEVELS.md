[VK_REMAINING_MIP_LEVELS](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_REMAINING_MIP_LEVELS.html) - Sentinel for all remaining mipmap levels

# C Specifications
[`REMAINING_MIP_LEVELS`] is a special constant value used for image
views to indicate that all remaining mipmap levels in an image after the
base level should be included in the view.
```c
#define VK_REMAINING_MIP_LEVELS           (~0U)
```

# Related
- [`crate::vulkan1_0`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        