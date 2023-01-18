[VkDeviceQueueCreateFlagBits](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceQueueCreateFlagBits.html) - Bitmask specifying behavior of the queue

# C Specifications
Bits which  **can**  be set in [`DeviceQueueCreateInfo::flags`],
specifying usage behavior of a queue, are:
```c
// Provided by VK_VERSION_1_1
typedef enum VkDeviceQueueCreateFlagBits {
  // Provided by VK_VERSION_1_1
    VK_DEVICE_QUEUE_CREATE_PROTECTED_BIT = 0x00000001,
} VkDeviceQueueCreateFlagBits;
```

# Description
- [`PROTECTED`] specifies that the device queue is a protected-capable queue.

# Related
- [`crate::vulkan1_1`]
- [`DeviceQueueCreateFlags`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        