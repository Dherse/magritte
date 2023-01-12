[VkSubmitFlagBits](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSubmitFlagBits.html) - Bitmask specifying behavior of a submission

# C Specifications
Bits which  **can**  be set in [`SubmitInfo2::flags`], specifying
submission behavior, are:
```c
// Provided by VK_VERSION_1_3
typedef enum VkSubmitFlagBits {
    VK_SUBMIT_PROTECTED_BIT = 0x00000001,
    VK_SUBMIT_PROTECTED_BIT_KHR = VK_SUBMIT_PROTECTED_BIT,
} VkSubmitFlagBits;
```
or the equivalent
```c
// Provided by VK_KHR_synchronization2
typedef VkSubmitFlagBits VkSubmitFlagBitsKHR;
```

# Description
- [`VK_SUBMIT_FLAG_BITS`] specifies that this batch is a protected submission.

# Related
- [`khr_synchronization2`]
- [`crate::vulkan1_3`]
- [VkSubmitFlags]()

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        