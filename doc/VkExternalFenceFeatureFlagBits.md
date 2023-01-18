[VkExternalFenceFeatureFlagBits](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExternalFenceFeatureFlagBits.html) - Bitfield describing features of an external fence handle type

# C Specifications
Bits which  **may**  be set in
[`ExternalFenceProperties::external_fence_features`], indicating
features of a fence external handle type, are:
```c
// Provided by VK_VERSION_1_1
typedef enum VkExternalFenceFeatureFlagBits {
    VK_EXTERNAL_FENCE_FEATURE_EXPORTABLE_BIT = 0x00000001,
    VK_EXTERNAL_FENCE_FEATURE_IMPORTABLE_BIT = 0x00000002,
  // Provided by VK_KHR_external_fence_capabilities
    VK_EXTERNAL_FENCE_FEATURE_EXPORTABLE_BIT_KHR = VK_EXTERNAL_FENCE_FEATURE_EXPORTABLE_BIT,
  // Provided by VK_KHR_external_fence_capabilities
    VK_EXTERNAL_FENCE_FEATURE_IMPORTABLE_BIT_KHR = VK_EXTERNAL_FENCE_FEATURE_IMPORTABLE_BIT,
} VkExternalFenceFeatureFlagBits;
```
or the equivalent
```c
// Provided by VK_KHR_external_fence_capabilities
typedef VkExternalFenceFeatureFlagBits VkExternalFenceFeatureFlagBitsKHR;
```

# Description
- [`EXPORTABLE`] specifies handles of this type  **can**  be exported from Vulkan fence objects.
- [`IMPORTABLE`] specifies handles of this type  **can**  be imported to Vulkan fence objects.

# Related
- [`crate::vulkan1_1`]
- [`ExternalFenceFeatureFlags`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        