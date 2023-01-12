[VkExternalSemaphoreFeatureFlagBits](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExternalSemaphoreFeatureFlagBits.html) - Bitfield describing features of an external semaphore handle type

# C Specifications
Bits which  **may**  be set in
[`ExternalSemaphoreProperties::external_semaphore_features`],
specifying the features of an external semaphore handle type, are:
```c
// Provided by VK_VERSION_1_1
typedef enum VkExternalSemaphoreFeatureFlagBits {
    VK_EXTERNAL_SEMAPHORE_FEATURE_EXPORTABLE_BIT = 0x00000001,
    VK_EXTERNAL_SEMAPHORE_FEATURE_IMPORTABLE_BIT = 0x00000002,
  // Provided by VK_KHR_external_semaphore_capabilities
    VK_EXTERNAL_SEMAPHORE_FEATURE_EXPORTABLE_BIT_KHR = VK_EXTERNAL_SEMAPHORE_FEATURE_EXPORTABLE_BIT,
  // Provided by VK_KHR_external_semaphore_capabilities
    VK_EXTERNAL_SEMAPHORE_FEATURE_IMPORTABLE_BIT_KHR = VK_EXTERNAL_SEMAPHORE_FEATURE_IMPORTABLE_BIT,
} VkExternalSemaphoreFeatureFlagBits;
```
or the equivalent
```c
// Provided by VK_KHR_external_semaphore_capabilities
typedef VkExternalSemaphoreFeatureFlagBits VkExternalSemaphoreFeatureFlagBitsKHR;
```

# Description
- [`VK_EXTERNAL_SEMAPHORE_FEATURE_FLAG_BITS`] specifies that handles of this type  **can**  be exported from Vulkan semaphore objects.
- [`VK_EXTERNAL_SEMAPHORE_FEATURE_FLAG_BITS`] specifies that handles of this type  **can**  be imported as Vulkan semaphore objects.

# Related
- [`crate::vulkan1_1`]
- [VkExternalSemaphoreFeatureFlags]()

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        