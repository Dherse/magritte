[VkDiscardRectangleModeEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDiscardRectangleModeEXT.html) - Specify the discard rectangle mode

# C Specifications
[`DiscardRectangleModeEXT`] values are:
```c
// Provided by VK_EXT_discard_rectangles
typedef enum VkDiscardRectangleModeEXT {
    VK_DISCARD_RECTANGLE_MODE_INCLUSIVE_EXT = 0,
    VK_DISCARD_RECTANGLE_MODE_EXCLUSIVE_EXT = 1,
} VkDiscardRectangleModeEXT;
```

# Description
- [`INCLUSIVE`] specifies that the discard rectangle test is inclusive.
- [`EXCLUSIVE`] specifies that the discard rectangle test is exclusive.

# Related
- [`VK_EXT_discard_rectangles`]
- [`PipelineDiscardRectangleStateCreateInfoEXT`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        