[VkSemaphoreImportFlagBits](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSemaphoreImportFlagBits.html) - Bitmask specifying additional parameters of semaphore payload import

# C Specifications
Bits which  **can**  be set in
- [`ImportSemaphoreWin32HandleInfoKHR::flags`]
- [`ImportSemaphoreFdInfoKHR::flags`]
- [`ImportSemaphoreZirconHandleInfoFUCHSIA::flags`]
specifying additional parameters of a semaphore import operation are:
```c
// Provided by VK_VERSION_1_1
typedef enum VkSemaphoreImportFlagBits {
    VK_SEMAPHORE_IMPORT_TEMPORARY_BIT = 0x00000001,
  // Provided by VK_KHR_external_semaphore
    VK_SEMAPHORE_IMPORT_TEMPORARY_BIT_KHR = VK_SEMAPHORE_IMPORT_TEMPORARY_BIT,
} VkSemaphoreImportFlagBits;
```
or the equivalent
```c
// Provided by VK_KHR_external_semaphore
typedef VkSemaphoreImportFlagBits VkSemaphoreImportFlagBitsKHR;
```

# Description
These bits have the following meanings:
- [`TEMPORARY`] specifies that the semaphore payload will be imported only temporarily, as described in [Importing Semaphore Payloads](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-semaphores-importing), regardless of the permanence of `handleType`.

# Related
- [`crate::vulkan1_1`]
- [`SemaphoreImportFlags`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        