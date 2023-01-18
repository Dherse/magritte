[VkSamplerCustomBorderColorCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSamplerCustomBorderColorCreateInfoEXT.html) - Structure specifying custom border color

# C Specifications
In addition to the predefined border color values, applications  **can**  provide
a custom border color value by including the
[`SamplerCustomBorderColorCreateInfoEXT`] structure in the
[`SamplerCreateInfo`]::[`p_next`] chain.The [`SamplerCustomBorderColorCreateInfoEXT`] structure is defined as:
```c
// Provided by VK_EXT_custom_border_color
typedef struct VkSamplerCustomBorderColorCreateInfoEXT {
    VkStructureType      sType;
    const void*          pNext;
    VkClearColorValue    customBorderColor;
    VkFormat             format;
} VkSamplerCustomBorderColorCreateInfoEXT;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`custom_border_color`] is a [`ClearColorValue`] representing the desired custom sampler border color.
- [`format`] is a [`Format`] representing the format of the sampled image view(s). This field may be `VK_FORMAT_UNDEFINED` if the [customBorderColorWithoutFormat](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-customBorderColorWithoutFormat) feature is enabled.

# Description
## Valid Usage
-    If provided [`format`] is not `VK_FORMAT_UNDEFINED` then the [`SamplerCreateInfo::border_color`] type  **must**  match the sampled type of the provided [`format`], as shown in the *SPIR-V Sampled Type* column of the [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#formats-numericformat](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#formats-numericformat) table
-    If the [customBorderColorWithoutFormat](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-customBorderColorWithoutFormat) feature is not enabled then [`format`] **must**  not be `VK_FORMAT_UNDEFINED`
-    If the sampler is used to sample an image view of `VK_FORMAT_B4G4R4A4_UNORM_PACK16`, `VK_FORMAT_B5G6R5_UNORM_PACK16`, or `VK_FORMAT_B5G5R5A1_UNORM_PACK16` format then [`format`] **must**  not be `VK_FORMAT_UNDEFINED`

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_SAMPLER_CUSTOM_BORDER_COLOR_CREATE_INFO_EXT`
-  [`format`] **must**  be a valid [`Format`] value

# Related
- [`VK_EXT_custom_border_color`]
- [`ClearColorValue`]
- [`Format`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        