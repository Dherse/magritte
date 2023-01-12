[VkFormatProperties](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFormatProperties.html) - Structure specifying image format properties

# C Specifications
The [`FormatProperties`] structure is defined as:
```c
// Provided by VK_VERSION_1_0
typedef struct VkFormatProperties {
    VkFormatFeatureFlags    linearTilingFeatures;
    VkFormatFeatureFlags    optimalTilingFeatures;
    VkFormatFeatureFlags    bufferFeatures;
} VkFormatProperties;
```

# Members
- [`linear_tiling_features`] is a bitmask of [`FormatFeatureFlagBits`] specifying features supported by images created with a `tiling` parameter of `VK_IMAGE_TILING_LINEAR`.
- [`optimal_tiling_features`] is a bitmask of [`FormatFeatureFlagBits`] specifying features supported by images created with a `tiling` parameter of `VK_IMAGE_TILING_OPTIMAL`.
- [`buffer_features`] is a bitmask of [`FormatFeatureFlagBits`] specifying features supported by buffers.

# Description
If `format` is a block-compressed format, then [`buffer_features`] **must**  not support any features for the format.If `format` is not a multi-plane format then [`linear_tiling_features`]
and [`optimal_tiling_features`] **must**  not contain
`VK_FORMAT_FEATURE_DISJOINT_BIT`.

# Related
- [`crate::vulkan1_0`]
- [VkFormatFeatureFlags]()
- [`FormatProperties2`]
- [`get_physical_device_format_properties`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        