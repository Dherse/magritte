[VkExternalFenceProperties](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExternalFenceProperties.html) - Structure describing supported external fence handle features

# C Specifications
The [`ExternalFenceProperties`] structure is defined as:
```c
// Provided by VK_VERSION_1_1
typedef struct VkExternalFenceProperties {
    VkStructureType                   sType;
    void*                             pNext;
    VkExternalFenceHandleTypeFlags    exportFromImportedHandleTypes;
    VkExternalFenceHandleTypeFlags    compatibleHandleTypes;
    VkExternalFenceFeatureFlags       externalFenceFeatures;
} VkExternalFenceProperties;
```
or the equivalent
```c
// Provided by VK_KHR_external_fence_capabilities
typedef VkExternalFenceProperties VkExternalFencePropertiesKHR;
```

# Members
- [`export_from_imported_handle_types`] is a bitmask of [`ExternalFenceHandleTypeFlagBits`] indicating which types of imported handle `handleType` **can**  be exported from.
- [`compatible_handle_types`] is a bitmask of [`ExternalFenceHandleTypeFlagBits`] specifying handle types which  **can**  be specified at the same time as `handleType` when creating a fence.
- [`external_fence_features`] is a bitmask of [`ExternalFenceFeatureFlagBits`] indicating the features of `handleType`.

# Description
If `handleType` is not supported by the implementation, then
[`ExternalFenceProperties`]::[`external_fence_features`] will be set to
zero.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_EXTERNAL_FENCE_PROPERTIES`
-  [`p_next`] **must**  be `NULL`

# Related
- [`crate::vulkan1_1`]
- [`ExternalFenceFeatureFlags`]
- [`ExternalFenceHandleTypeFlags`]
- [`StructureType`]
- [`get_physical_device_external_fence_properties`]
- [`get_physical_device_external_fence_properties_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        