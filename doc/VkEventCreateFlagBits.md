[VkEventCreateFlagBits](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkEventCreateFlagBits.html) - Event creation flag bits

# C Specifications
```c
// Provided by VK_VERSION_1_0
typedef enum VkEventCreateFlagBits {
  // Provided by VK_VERSION_1_3
    VK_EVENT_CREATE_DEVICE_ONLY_BIT = 0x00000001,
  // Provided by VK_KHR_synchronization2
    VK_EVENT_CREATE_DEVICE_ONLY_BIT_KHR = VK_EVENT_CREATE_DEVICE_ONLY_BIT,
} VkEventCreateFlagBits;
```

# Description
- [`DEVICE_ONLY`] specifies that host event commands will not be used with this event.

# Related
- [`crate::vulkan1_0`]
- [`EventCreateFlags`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        