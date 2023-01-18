[VK_REMAINING_ARRAY_LAYERS](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_REMAINING_ARRAY_LAYERS.html) - Sentinel for all remaining array layers

# C Specifications
[`REMAINING_ARRAY_LAYERS`] is a special constant value used for image
views to indicate that all remaining array layers in an image after the base
layer should be included in the view.
```c
#define VK_REMAINING_ARRAY_LAYERS         (~0U)
```

# Related
- [`crate::vulkan1_0`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        