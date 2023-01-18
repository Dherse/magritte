[VkExternalMemoryProperties](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExternalMemoryProperties.html) - Structure specifying external memory handle type capabilities

# C Specifications
The [`ExternalMemoryProperties`] structure is defined as:
```c
// Provided by VK_VERSION_1_1
typedef struct VkExternalMemoryProperties {
    VkExternalMemoryFeatureFlags       externalMemoryFeatures;
    VkExternalMemoryHandleTypeFlags    exportFromImportedHandleTypes;
    VkExternalMemoryHandleTypeFlags    compatibleHandleTypes;
} VkExternalMemoryProperties;
```
or the equivalent
```c
// Provided by VK_KHR_external_memory_capabilities
typedef VkExternalMemoryProperties VkExternalMemoryPropertiesKHR;
```

# Members
- [`external_memory_features`] is a bitmask of [`ExternalMemoryFeatureFlagBits`] specifying the features of `handleType`.
- [`export_from_imported_handle_types`] is a bitmask of [`ExternalMemoryHandleTypeFlagBits`] specifying which types of imported handle `handleType` **can**  be exported from.
- [`compatible_handle_types`] is a bitmask of [`ExternalMemoryHandleTypeFlagBits`] specifying handle types which  **can**  be specified at the same time as `handleType` when creating an image compatible with external memory.

# Description
[`compatible_handle_types`] **must**  include at least `handleType`.
Inclusion of a handle type in [`compatible_handle_types`] does not imply the
values returned in [`ImageFormatProperties2`] will be the same when
[`PhysicalDeviceExternalImageFormatInfo::handle_type`] is set to
that type.
The application is responsible for querying the capabilities of all handle
types intended for concurrent use in a single image and intersecting them to
obtain the compatible set of capabilities.

# Related
- [`crate::vulkan1_1`]
- [`ExternalBufferProperties`]
- [`ExternalImageFormatProperties`]
- [`ExternalMemoryFeatureFlags`]
- [`ExternalMemoryHandleTypeFlags`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        