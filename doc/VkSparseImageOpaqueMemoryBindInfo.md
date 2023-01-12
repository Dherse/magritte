[VkSparseImageOpaqueMemoryBindInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSparseImageOpaqueMemoryBindInfo.html) - Structure specifying sparse image opaque memory bind information

# C Specifications
Memory is bound to opaque regions of [`Image`] objects created with the
`VK_IMAGE_CREATE_SPARSE_BINDING_BIT` flag using the following structure:
```c
// Provided by VK_VERSION_1_0
typedef struct VkSparseImageOpaqueMemoryBindInfo {
    VkImage                      image;
    uint32_t                     bindCount;
    const VkSparseMemoryBind*    pBinds;
} VkSparseImageOpaqueMemoryBindInfo;
```

# Members
- [`image`] is the [`Image`] object to be bound.
- [`bind_count`] is the number of [`SparseMemoryBind`] structures in the [`binds`] array.
- [`binds`] is a pointer to an array of [`SparseMemoryBind`] structures.

# Description
## Valid Usage
-    If the `flags` member of any element of [`binds`] contains `VK_SPARSE_MEMORY_BIND_METADATA_BIT`, the binding range defined  **must**  be within the mip tail region of the metadata aspect of [`image`]

## Valid Usage (Implicit)
-  [`image`] **must**  be a valid [`Image`] handle
-  [`binds`] **must**  be a valid pointer to an array of [`bind_count`] valid [`SparseMemoryBind`] structures
-  [`bind_count`] **must**  be greater than `0`

# Related
- [`crate::vulkan1_0`]
- [`BindSparseInfo`]
- [`Image`]
- [`SparseMemoryBind`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        