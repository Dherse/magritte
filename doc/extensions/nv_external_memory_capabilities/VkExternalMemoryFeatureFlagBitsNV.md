[VkExternalMemoryFeatureFlagBitsNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExternalMemoryFeatureFlagBitsNV.html) - Bitmask specifying external memory features

# C Specifications
Bits which  **can**  be set in
[`ExternalImageFormatPropertiesNV::external_memory_features`],
indicating properties of the external memory handle type, are:
```c
// Provided by VK_NV_external_memory_capabilities
typedef enum VkExternalMemoryFeatureFlagBitsNV {
    VK_EXTERNAL_MEMORY_FEATURE_DEDICATED_ONLY_BIT_NV = 0x00000001,
    VK_EXTERNAL_MEMORY_FEATURE_EXPORTABLE_BIT_NV = 0x00000002,
    VK_EXTERNAL_MEMORY_FEATURE_IMPORTABLE_BIT_NV = 0x00000004,
} VkExternalMemoryFeatureFlagBitsNV;
```

# Description
- [`VK_EXTERNAL_MEMORY_FEATURE_FLAG_BITS_NV`] specifies that external memory of the specified type  **must**  be created as a dedicated allocation when used in the manner specified.
- [`VK_EXTERNAL_MEMORY_FEATURE_FLAG_BITS_NV`] specifies that the implementation supports exporting handles of the specified type.
- [`VK_EXTERNAL_MEMORY_FEATURE_FLAG_BITS_NV`] specifies that the implementation supports importing handles of the specified type.

# Related
- [`nv_external_memory_capabilities`]
- [`ExternalImageFormatPropertiesNV`]
- [VkExternalMemoryFeatureFlagsNV]()
- [`get_physical_device_external_image_format_properties_nv`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        