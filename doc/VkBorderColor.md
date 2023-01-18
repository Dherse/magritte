[VkBorderColor](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBorderColor.html) - Specify border color used for texture lookups

# C Specifications
Possible values of [`SamplerCreateInfo::border_color`], specifying
the border color used for texture lookups, are:
```c
// Provided by VK_VERSION_1_0
typedef enum VkBorderColor {
    VK_BORDER_COLOR_FLOAT_TRANSPARENT_BLACK = 0,
    VK_BORDER_COLOR_INT_TRANSPARENT_BLACK = 1,
    VK_BORDER_COLOR_FLOAT_OPAQUE_BLACK = 2,
    VK_BORDER_COLOR_INT_OPAQUE_BLACK = 3,
    VK_BORDER_COLOR_FLOAT_OPAQUE_WHITE = 4,
    VK_BORDER_COLOR_INT_OPAQUE_WHITE = 5,
  // Provided by VK_EXT_custom_border_color
    VK_BORDER_COLOR_FLOAT_CUSTOM_EXT = 1000287003,
  // Provided by VK_EXT_custom_border_color
    VK_BORDER_COLOR_INT_CUSTOM_EXT = 1000287004,
} VkBorderColor;
```

# Description
- [`FLOAT_TRANSPARENT_BLACK`] specifies a transparent, floating-point format, black color.
- [`INT_TRANSPARENT_BLACK`] specifies a transparent, integer format, black color.
- [`FLOAT_OPAQUE_BLACK`] specifies an opaque, floating-point format, black color.
- [`INT_OPAQUE_BLACK`] specifies an opaque, integer format, black color.
- [`FLOAT_OPAQUE_WHITE`] specifies an opaque, floating-point format, white color.
- [`INT_OPAQUE_WHITE`] specifies an opaque, integer format, white color.
- [`FLOAT_CUSTOM_EXT`] indicates that a [`SamplerCustomBorderColorCreateInfoEXT`] structure is included in the [`SamplerCreateInfo::p_next`] chain containing the color data in floating-point format.
- [`INT_CUSTOM_EXT`] indicates that a [`SamplerCustomBorderColorCreateInfoEXT`] structure is included in the [`SamplerCreateInfo::p_next`] chain containing the color data in integer format.
These colors are described in detail in [Texel
Replacement](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#textures-texel-replacement).

# Related
- [`crate::vulkan1_0`]
- [`SamplerCreateInfo`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        