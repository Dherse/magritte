[VkSemaphoreWaitFlagBits](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSemaphoreWaitFlagBits.html) - Bitmask specifying additional parameters of a semaphore wait operation

# C Specifications
Bits which  **can**  be set in [`SemaphoreWaitInfo::flags`], specifying
additional parameters of a semaphore wait operation, are:
```c
// Provided by VK_VERSION_1_2
typedef enum VkSemaphoreWaitFlagBits {
    VK_SEMAPHORE_WAIT_ANY_BIT = 0x00000001,
  // Provided by VK_KHR_timeline_semaphore
    VK_SEMAPHORE_WAIT_ANY_BIT_KHR = VK_SEMAPHORE_WAIT_ANY_BIT,
} VkSemaphoreWaitFlagBits;
```
or the equivalent
```c
// Provided by VK_KHR_timeline_semaphore
typedef VkSemaphoreWaitFlagBits VkSemaphoreWaitFlagBitsKHR;
```

# Description
- [`VK_SEMAPHORE_WAIT_FLAG_BITS`] specifies that the semaphore wait condition is that at least one of the semaphores in [`SemaphoreWaitInfo::semaphores`] has reached the value specified by the corresponding element of [`SemaphoreWaitInfo::values`]. If [`VK_SEMAPHORE_WAIT_FLAG_BITS`] is not set, the semaphore wait condition is that all of the semaphores in [`SemaphoreWaitInfo::semaphores`] have reached the value specified by the corresponding element of [`SemaphoreWaitInfo::values`].

# Related
- [`khr_timeline_semaphore`]
- [`crate::vulkan1_2`]
- [VkSemaphoreWaitFlags]()

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        