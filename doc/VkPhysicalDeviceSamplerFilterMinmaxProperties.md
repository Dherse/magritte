[VkPhysicalDeviceSamplerFilterMinmaxProperties](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceSamplerFilterMinmaxProperties.html) - Structure describing sampler filter minmax limits that can be supported by an implementation

# C Specifications
The [`PhysicalDeviceSamplerFilterMinmaxProperties`] structure is defined
as:
```c
// Provided by VK_VERSION_1_2
typedef struct VkPhysicalDeviceSamplerFilterMinmaxProperties {
    VkStructureType    sType;
    void*              pNext;
    VkBool32           filterMinmaxSingleComponentFormats;
    VkBool32           filterMinmaxImageComponentMapping;
} VkPhysicalDeviceSamplerFilterMinmaxProperties;
```
or the equivalent
```c
// Provided by VK_EXT_sampler_filter_minmax
typedef VkPhysicalDeviceSamplerFilterMinmaxProperties VkPhysicalDeviceSamplerFilterMinmaxPropertiesEXT;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.

# Description
- [`filter_minmax_single_component_formats`] is a boolean value indicating whether a minimum set of required formats support min/max filtering.
- [`filter_minmax_image_component_mapping`] is a boolean value indicating whether the implementation supports non-identity component mapping of the image when doing min/max filtering.
If the [`PhysicalDeviceSamplerFilterMinmaxProperties`] structure is included in the [`p_next`] chain of the
[`PhysicalDeviceProperties2`] structure passed to
[`get_physical_device_properties2`], it is filled in with each
corresponding implementation-dependent property.If [`filter_minmax_single_component_formats`] is [`TRUE`], the following
formats  **must**  support the
`VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_MINMAX_BIT` feature with
`VK_IMAGE_TILING_OPTIMAL`, if they support
`VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT`:
- `VK_FORMAT_R8_UNORM`
- `VK_FORMAT_R8_SNORM`
- `VK_FORMAT_R16_UNORM`
- `VK_FORMAT_R16_SNORM`
- `VK_FORMAT_R16_SFLOAT`
- `VK_FORMAT_R32_SFLOAT`
- `VK_FORMAT_D16_UNORM`
- `VK_FORMAT_X8_D24_UNORM_PACK32`
- `VK_FORMAT_D32_SFLOAT`
- `VK_FORMAT_D16_UNORM_S8_UINT`
- `VK_FORMAT_D24_UNORM_S8_UINT`
- `VK_FORMAT_D32_SFLOAT_S8_UINT`
If the format is a depth/stencil format, this bit only specifies that the
depth aspect (not the stencil aspect) of an image of this format supports
min/max filtering, and that min/max filtering of the depth aspect is
supported when depth compare is disabled in the sampler.If [`filter_minmax_image_component_mapping`] is [`FALSE`] the component
mapping of the image view used with min/max filtering  **must**  have been
created with the `r` component set to the
[identity swizzle](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#resources-image-views-identity-mappings).
Only the `r` component of the sampled image value is defined and the
other component values are undefined.
If [`filter_minmax_image_component_mapping`] is [`TRUE`] this restriction
does not apply and image component mapping works as normal.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLER_FILTER_MINMAX_PROPERTIES`

# Related
- [`VK_EXT_sampler_filter_minmax`]
- [`crate::vulkan1_2`]
- [`Bool32`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        