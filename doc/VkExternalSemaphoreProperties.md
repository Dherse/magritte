[VkExternalSemaphoreProperties](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExternalSemaphoreProperties.html) - Structure describing supported external semaphore handle features

# C Specifications
The [`ExternalSemaphoreProperties`] structure is defined as:
```c
// Provided by VK_VERSION_1_1
typedef struct VkExternalSemaphoreProperties {
    VkStructureType                       sType;
    void*                                 pNext;
    VkExternalSemaphoreHandleTypeFlags    exportFromImportedHandleTypes;
    VkExternalSemaphoreHandleTypeFlags    compatibleHandleTypes;
    VkExternalSemaphoreFeatureFlags       externalSemaphoreFeatures;
} VkExternalSemaphoreProperties;
```
or the equivalent
```c
// Provided by VK_KHR_external_semaphore_capabilities
typedef VkExternalSemaphoreProperties VkExternalSemaphorePropertiesKHR;
```

# Members
- [`s_type`] is the type of this structure
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`export_from_imported_handle_types`] is a bitmask of [`ExternalSemaphoreHandleTypeFlagBits`] specifying which types of imported handle `handleType` **can**  be exported from.
- [`compatible_handle_types`] is a bitmask of [`ExternalSemaphoreHandleTypeFlagBits`] specifying handle types which  **can**  be specified at the same time as `handleType` when creating a semaphore.
- [`external_semaphore_features`] is a bitmask of [`ExternalSemaphoreFeatureFlagBits`] describing the features of `handleType`.

# Description
If `handleType` is not supported by the implementation, then
[`ExternalSemaphoreProperties`]::[`external_semaphore_features`] will be
set to zero.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_EXTERNAL_SEMAPHORE_PROPERTIES`
-  [`p_next`] **must**  be `NULL`

# Related
- [`crate::vulkan1_1`]
- [VkExternalSemaphoreFeatureFlags]()
- [VkExternalSemaphoreHandleTypeFlags]()
- [`StructureType`]
- [`get_physical_device_external_semaphore_properties`]
- [`get_physical_device_external_semaphore_properties_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        