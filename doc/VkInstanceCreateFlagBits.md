[VkInstanceCreateFlagBits](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkInstanceCreateFlagBits.html) - Bitmask specifying behavior of the instance

# C Specifications
```c
// Provided by VK_VERSION_1_0
typedef enum VkInstanceCreateFlagBits {
  // Provided by VK_KHR_portability_enumeration
    VK_INSTANCE_CREATE_ENUMERATE_PORTABILITY_BIT_KHR = 0x00000001,
} VkInstanceCreateFlagBits;
```

# Description
- [`ENUMERATE_PORTABILITY_KHR`] specifies that the instance will enumerate available Vulkan Portability-compliant physical devices and groups in addition to the Vulkan physical devices and groups that are enumerated by default.

# Related
- [`crate::vulkan1_0`]
- [VkInstanceCreateFlags]()

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        