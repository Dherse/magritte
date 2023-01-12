[VkFenceImportFlagBits](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFenceImportFlagBits.html) - Bitmask specifying additional parameters of fence payload import

# C Specifications
Bits which  **can**  be set in
- [`ImportFenceWin32HandleInfoKHR::flags`]
- [`ImportFenceFdInfoKHR::flags`]
specifying additional parameters of a fence import operation are:
```c
// Provided by VK_VERSION_1_1
typedef enum VkFenceImportFlagBits {
    VK_FENCE_IMPORT_TEMPORARY_BIT = 0x00000001,
  // Provided by VK_KHR_external_fence
    VK_FENCE_IMPORT_TEMPORARY_BIT_KHR = VK_FENCE_IMPORT_TEMPORARY_BIT,
} VkFenceImportFlagBits;
```
or the equivalent
```c
// Provided by VK_KHR_external_fence
typedef VkFenceImportFlagBits VkFenceImportFlagBitsKHR;
```

# Description
- [`VK_FENCE_IMPORT_FLAG_BITS`] specifies that the fence payload will be imported only temporarily, as described in [Importing Fence Payloads](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-fences-importing), regardless of the permanence of `handleType`.

# Related
- [`crate::vulkan1_1`]
- [VkFenceImportFlags]()

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        