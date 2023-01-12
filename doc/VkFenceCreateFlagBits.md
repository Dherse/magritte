[VkFenceCreateFlagBits](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFenceCreateFlagBits.html) - Bitmask specifying initial state and behavior of a fence

# C Specifications
```c
// Provided by VK_VERSION_1_0
typedef enum VkFenceCreateFlagBits {
    VK_FENCE_CREATE_SIGNALED_BIT = 0x00000001,
} VkFenceCreateFlagBits;
```

# Description
- [`VK_FENCE_CREATE_FLAG_BITS`] specifies that the fence object is created in the signaled state. Otherwise, it is created in the unsignaled state.

# Related
- [`crate::vulkan1_0`]
- [VkFenceCreateFlags]()

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        