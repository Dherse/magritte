[VkExternalMemoryFeatureFlagBits](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExternalMemoryFeatureFlagBits.html) - Bitmask specifying features of an external memory handle type

# C Specifications
Bits which  **may**  be set in
[`ExternalMemoryProperties::external_memory_features`], specifying
features of an external memory handle type, are:
```c
// Provided by VK_VERSION_1_1
typedef enum VkExternalMemoryFeatureFlagBits {
    VK_EXTERNAL_MEMORY_FEATURE_DEDICATED_ONLY_BIT = 0x00000001,
    VK_EXTERNAL_MEMORY_FEATURE_EXPORTABLE_BIT = 0x00000002,
    VK_EXTERNAL_MEMORY_FEATURE_IMPORTABLE_BIT = 0x00000004,
  // Provided by VK_KHR_external_memory_capabilities
    VK_EXTERNAL_MEMORY_FEATURE_DEDICATED_ONLY_BIT_KHR = VK_EXTERNAL_MEMORY_FEATURE_DEDICATED_ONLY_BIT,
  // Provided by VK_KHR_external_memory_capabilities
    VK_EXTERNAL_MEMORY_FEATURE_EXPORTABLE_BIT_KHR = VK_EXTERNAL_MEMORY_FEATURE_EXPORTABLE_BIT,
  // Provided by VK_KHR_external_memory_capabilities
    VK_EXTERNAL_MEMORY_FEATURE_IMPORTABLE_BIT_KHR = VK_EXTERNAL_MEMORY_FEATURE_IMPORTABLE_BIT,
} VkExternalMemoryFeatureFlagBits;
```
or the equivalent
```c
// Provided by VK_KHR_external_memory_capabilities
typedef VkExternalMemoryFeatureFlagBits VkExternalMemoryFeatureFlagBitsKHR;
```

# Description
- [`VK_EXTERNAL_MEMORY_FEATURE_FLAG_BITS`] specifies that images or buffers created with the specified parameters and handle type  **must**  use the mechanisms defined by [`MemoryDedicatedRequirements`] and [`MemoryDedicatedAllocateInfo`] to create (or import) a dedicated allocation for the image or buffer.
- [`VK_EXTERNAL_MEMORY_FEATURE_FLAG_BITS`] specifies that handles of this type  **can**  be exported from Vulkan memory objects.
- [`VK_EXTERNAL_MEMORY_FEATURE_FLAG_BITS`] specifies that handles of this type  **can**  be imported as Vulkan memory objects.
Because their semantics in external APIs roughly align with that of an image
or buffer with a dedicated allocation in Vulkan, implementations are
 **required**  to report [`VK_EXTERNAL_MEMORY_FEATURE_FLAG_BITS`] for
the following external handle types:
- `VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_BIT`
- `VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_KMT_BIT`
- `VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_RESOURCE_BIT`
- `VK_EXTERNAL_MEMORY_HANDLE_TYPE_ANDROID_HARDWARE_BUFFER_BIT_ANDROID` for images only
Implementations  **must**  not report
[`VK_EXTERNAL_MEMORY_FEATURE_FLAG_BITS`] for buffers with
external handle type
`VK_EXTERNAL_MEMORY_HANDLE_TYPE_ANDROID_HARDWARE_BUFFER_BIT_ANDROID`.
Implementations  **must**  not report
[`VK_EXTERNAL_MEMORY_FEATURE_FLAG_BITS`] for images or buffers
with external handle type
`VK_EXTERNAL_MEMORY_HANDLE_TYPE_HOST_ALLOCATION_BIT_EXT`, or
`VK_EXTERNAL_MEMORY_HANDLE_TYPE_HOST_MAPPED_FOREIGN_MEMORY_BIT_EXT`.

# Related
- [`crate::vulkan1_1`]
- [VkExternalMemoryFeatureFlags]()

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        