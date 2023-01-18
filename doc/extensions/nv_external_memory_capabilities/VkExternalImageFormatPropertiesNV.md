[VkExternalImageFormatPropertiesNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExternalImageFormatPropertiesNV.html) - Structure specifying external image format properties

# C Specifications
The [`ExternalImageFormatPropertiesNV`] structure is defined as:
```c
// Provided by VK_NV_external_memory_capabilities
typedef struct VkExternalImageFormatPropertiesNV {
    VkImageFormatProperties              imageFormatProperties;
    VkExternalMemoryFeatureFlagsNV       externalMemoryFeatures;
    VkExternalMemoryHandleTypeFlagsNV    exportFromImportedHandleTypes;
    VkExternalMemoryHandleTypeFlagsNV    compatibleHandleTypes;
} VkExternalImageFormatPropertiesNV;
```

# Members
- [`image_format_properties`] will be filled in as when calling [`get_physical_device_image_format_properties`], but the values returned  **may**  vary depending on the external handle type requested.
- [`external_memory_features`] is a bitmask of [`ExternalMemoryFeatureFlagBitsNV`], indicating properties of the external memory handle type ([`get_physical_device_external_image_format_properties_nv`]`::externalHandleType`) being queried, or 0 if the external memory handle type is 0.
- [`export_from_imported_handle_types`] is a bitmask of [`ExternalMemoryHandleTypeFlagBitsNV`] containing a bit set for every external handle type that  **may**  be used to create memory from which the handles of the type specified in [`get_physical_device_external_image_format_properties_nv`]`::externalHandleType` **can**  be exported, or 0 if the external memory handle type is 0.
- [`compatible_handle_types`] is a bitmask of [`ExternalMemoryHandleTypeFlagBitsNV`] containing a bit set for every external handle type that  **may**  be specified simultaneously with the handle type specified by [`get_physical_device_external_image_format_properties_nv`]`::externalHandleType` when calling [`allocate_memory`], or 0 if the external memory handle type is 0. [`compatible_handle_types`] will always contain [`get_physical_device_external_image_format_properties_nv`]`::externalHandleType`

# Related
- [`VK_NV_external_memory_capabilities`]
- [`ExternalMemoryFeatureFlagsNV`]
- [`ExternalMemoryHandleTypeFlagsNV`]
- [`ImageFormatProperties`]
- [`get_physical_device_external_image_format_properties_nv`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        