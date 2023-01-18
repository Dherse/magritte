[VkMemoryType](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryType.html) - Structure specifying memory type

# C Specifications
The [`MemoryType`] structure is defined as:
```c
// Provided by VK_VERSION_1_0
typedef struct VkMemoryType {
    VkMemoryPropertyFlags    propertyFlags;
    uint32_t                 heapIndex;
} VkMemoryType;
```

# Members
- [`heap_index`] describes which memory heap this memory type corresponds to, and  **must**  be less than `memoryHeapCount` from the [`PhysicalDeviceMemoryProperties`] structure.
- [`property_flags`] is a bitmask of [`MemoryPropertyFlagBits`] of properties for this memory type.

# Related
- [`crate::vulkan1_0`]
- [`MemoryPropertyFlags`]
- [`PhysicalDeviceMemoryProperties`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        