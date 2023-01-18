[VkQueueFlagBits](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueueFlagBits.html) - Bitmask specifying capabilities of queues in a queue family

# C Specifications
Bits which  **may**  be set in [`QueueFamilyProperties::queue_flags`],
indicating capabilities of queues in a queue family are:
```c
// Provided by VK_VERSION_1_0
typedef enum VkQueueFlagBits {
    VK_QUEUE_GRAPHICS_BIT = 0x00000001,
    VK_QUEUE_COMPUTE_BIT = 0x00000002,
    VK_QUEUE_TRANSFER_BIT = 0x00000004,
    VK_QUEUE_SPARSE_BINDING_BIT = 0x00000008,
  // Provided by VK_VERSION_1_1
    VK_QUEUE_PROTECTED_BIT = 0x00000010,
#ifdef VK_ENABLE_BETA_EXTENSIONS
  // Provided by VK_KHR_video_decode_queue
    VK_QUEUE_VIDEO_DECODE_BIT_KHR = 0x00000020,
#endif
#ifdef VK_ENABLE_BETA_EXTENSIONS
  // Provided by VK_KHR_video_encode_queue
    VK_QUEUE_VIDEO_ENCODE_BIT_KHR = 0x00000040,
#endif
} VkQueueFlagBits;
```

# Description
- [`GRAPHICS`] specifies that queues in this queue family support graphics operations.
- [`COMPUTE`] specifies that queues in this queue family support compute operations.
- [`TRANSFER`] specifies that queues in this queue family support transfer operations.
- [`SPARSE_BINDING`] specifies that queues in this queue family support sparse memory management operations (see [Sparse Resources](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#sparsememory)). If any of the sparse resource features are enabled, then at least one queue family  **must**  support this bit.
- [`VIDEO_DECODE_KHR`] specifies that queues in this queue family support Video Decode operations.
- [`VIDEO_ENCODE_KHR`] specifies that queues in this queue family support Video Encode operations.
- [`PROTECTED`] specifies that queues in this queue family support the `VK_DEVICE_QUEUE_CREATE_PROTECTED_BIT` bit. (see [Protected Memory](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-protected-memory)). If the physical device supports the `protectedMemory` feature, at least one of its queue families  **must**  support this bit.
If an implementation exposes any queue family that supports graphics
operations, at least one queue family of at least one physical device
exposed by the implementation  **must**  support both graphics and compute
operations.Furthermore, if the protected memory physical device feature is supported,
then at least one queue family of at least one physical device exposed by
the implementation  **must**  support graphics operations, compute operations,
and protected memory operations.For further details see [Queues](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#devsandqueues-queues).

# Related
- [`crate::vulkan1_0`]
- [`QueueFlags`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        