[VkSparseBufferMemoryBindInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSparseBufferMemoryBindInfo.html) - Structure specifying a sparse buffer memory bind operation

# C Specifications
Memory is bound to [`Buffer`] objects created with the
`VK_BUFFER_CREATE_SPARSE_BINDING_BIT` flag using the following
structure:
```c
// Provided by VK_VERSION_1_0
typedef struct VkSparseBufferMemoryBindInfo {
    VkBuffer                     buffer;
    uint32_t                     bindCount;
    const VkSparseMemoryBind*    pBinds;
} VkSparseBufferMemoryBindInfo;
```

# Members
- [`buffer`] is the [`Buffer`] object to be bound.
- [`bind_count`] is the number of [`SparseMemoryBind`] structures in the [`binds`] array.
- [`binds`] is a pointer to an array of [`SparseMemoryBind`] structures.

# Description
## Valid Usage (Implicit)
-  [`buffer`] **must**  be a valid [`Buffer`] handle
-  [`binds`] **must**  be a valid pointer to an array of [`bind_count`] valid [`SparseMemoryBind`] structures
-  [`bind_count`] **must**  be greater than `0`

# Related
- [`crate::vulkan1_0`]
- [`BindSparseInfo`]
- [`Buffer`]
- [`SparseMemoryBind`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        