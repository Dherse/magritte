[VkFormatProperties3](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFormatProperties3.html) - Structure specifying image format properties

# C Specifications
To query supported format extended features which are properties of the
physical device, add [`FormatProperties3`] structure to the [`p_next`]
chain of [`FormatProperties2`].The [`FormatProperties3`] structure is defined as:
```c
// Provided by VK_VERSION_1_3
typedef struct VkFormatProperties3 {
    VkStructureType          sType;
    void*                    pNext;
    VkFormatFeatureFlags2    linearTilingFeatures;
    VkFormatFeatureFlags2    optimalTilingFeatures;
    VkFormatFeatureFlags2    bufferFeatures;
} VkFormatProperties3;
```
or the equivalent
```c
// Provided by VK_KHR_format_feature_flags2
typedef VkFormatProperties3 VkFormatProperties3KHR;
```

# Members
- [`linear_tiling_features`] is a bitmask of [`FormatFeatureFlagBits2`] specifying features supported by images created with a `tiling` parameter of `VK_IMAGE_TILING_LINEAR`.
- [`optimal_tiling_features`] is a bitmask of [`FormatFeatureFlagBits2`] specifying features supported by images created with a `tiling` parameter of `VK_IMAGE_TILING_OPTIMAL`.
- [`buffer_features`] is a bitmask of [`FormatFeatureFlagBits2`] specifying features supported by buffers.

# Description
The bits reported in [`linear_tiling_features`], [`optimal_tiling_features`]
and [`buffer_features`] **must**  include the bits reported in the
corresponding fields of [`FormatProperties2::format_properties`].
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_FORMAT_PROPERTIES_3`

# Related
- [`khr_format_feature_flags2`]
- [`crate::vulkan1_3`]
- [`FormatFeatureFlags2`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        