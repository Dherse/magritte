[VkSamplerBorderColorComponentMappingCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSamplerBorderColorComponentMappingCreateInfoEXT.html) - Structure specifying the component mapping of the border color

# C Specifications
If the sampler is created with `VK_BORDER_COLOR_FLOAT_OPAQUE_BLACK`,
`VK_BORDER_COLOR_INT_OPAQUE_BLACK`,
`VK_BORDER_COLOR_FLOAT_CUSTOM_EXT`, or
`VK_BORDER_COLOR_INT_CUSTOM_EXT``borderColor`, and that sampler
will be combined with an image view that does not have an
[identity swizzle](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#resources-image-views-identity-mappings), and
[`PhysicalDeviceBorderColorSwizzleFeaturesEXT::border_color_swizzle_from_image`]
is not enabled, then it is necessary to specify the component mapping of the
border color, by including the
[`SamplerBorderColorComponentMappingCreateInfoEXT`] structure in the
[`SamplerCreateInfo`]::[`p_next`] chain, to get defined results.The [`SamplerBorderColorComponentMappingCreateInfoEXT`] structure is
defined as:
```c
// Provided by VK_EXT_border_color_swizzle
typedef struct VkSamplerBorderColorComponentMappingCreateInfoEXT {
    VkStructureType       sType;
    const void*           pNext;
    VkComponentMapping    components;
    VkBool32              srgb;
} VkSamplerBorderColorComponentMappingCreateInfoEXT;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`components`] is a [`ComponentMapping`] structure specifying a remapping of the border color components.
- [`srgb`] indicates that the sampler will be combined with an image view that has an image format which is sRGB encoded.

# Description
The [`ComponentMapping`][`components`] member describes a remapping
from components of the border color to components of the vector returned by
shader image instructions when the border color is used.
## Valid Usage
-    The [`borderColorSwizzle`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-borderColorSwizzle) feature  **must**  be enabled.

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_SAMPLER_BORDER_COLOR_COMPONENT_MAPPING_CREATE_INFO_EXT`
-  [`components`] **must**  be a valid [`ComponentMapping`] structure

# Related
- [`ext_border_color_swizzle`]
- [`Bool32`]
- [`ComponentMapping`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        