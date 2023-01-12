[VkSparseMemoryBindFlagBits](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSparseMemoryBindFlagBits.html) - Bitmask specifying usage of a sparse memory binding operation

# C Specifications
Bits which  **can**  be set in [`SparseMemoryBind::flags`], specifying
usage of a sparse memory binding operation, are:
```c
// Provided by VK_VERSION_1_0
typedef enum VkSparseMemoryBindFlagBits {
    VK_SPARSE_MEMORY_BIND_METADATA_BIT = 0x00000001,
} VkSparseMemoryBindFlagBits;
```

# Description
- [`VK_SPARSE_MEMORY_BIND_FLAG_BITS`] specifies that the memory being bound is only for the metadata aspect.

# Related
- [`crate::vulkan1_0`]
- [VkSparseMemoryBindFlags]()

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        