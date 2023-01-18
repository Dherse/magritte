[VkMemoryHeapFlagBits](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryHeapFlagBits.html) - Bitmask specifying attribute flags for a heap

# C Specifications
Bits which  **may**  be set in [`MemoryHeap::flags`], indicating
attribute flags for the heap, are:
```c
// Provided by VK_VERSION_1_0
typedef enum VkMemoryHeapFlagBits {
    VK_MEMORY_HEAP_DEVICE_LOCAL_BIT = 0x00000001,
  // Provided by VK_VERSION_1_1
    VK_MEMORY_HEAP_MULTI_INSTANCE_BIT = 0x00000002,
  // Provided by VK_KHR_device_group_creation
    VK_MEMORY_HEAP_MULTI_INSTANCE_BIT_KHR = VK_MEMORY_HEAP_MULTI_INSTANCE_BIT,
} VkMemoryHeapFlagBits;
```

# Description
- [`DEVICE_LOCAL`] specifies that the heap corresponds to device-local memory. Device-local memory  **may**  have different performance characteristics than host-local memory, and  **may**  support different memory property flags.
- [`MULTI_INSTANCE`] specifies that in a logical device representing more than one physical device, there is a per-physical device instance of the heap memory. By default, an allocation from such a heap will be replicated to each physical device’s instance of the heap.

# Related
- [`crate::vulkan1_0`]
- [`MemoryHeapFlags`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        