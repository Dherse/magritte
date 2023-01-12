[VkMemoryHeap](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryHeap.html) - Structure specifying a memory heap

# C Specifications
The [`MemoryHeap`] structure is defined as:
```c
// Provided by VK_VERSION_1_0
typedef struct VkMemoryHeap {
    VkDeviceSize         size;
    VkMemoryHeapFlags    flags;
} VkMemoryHeap;
```

# Members
- [`size`] is the total memory size in bytes in the heap.
- [`flags`] is a bitmask of [`MemoryHeapFlagBits`] specifying attribute flags for the heap.

# Related
- [`crate::vulkan1_0`]
- [`DeviceSize`]
- [VkMemoryHeapFlags]()
- [`PhysicalDeviceMemoryProperties`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        