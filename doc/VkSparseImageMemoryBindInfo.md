[VkSparseImageMemoryBindInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSparseImageMemoryBindInfo.html) - Structure specifying sparse image memory bind information

# C Specifications
Memory  **can**  be bound to sparse image blocks of [`Image`] objects created
with the `VK_IMAGE_CREATE_SPARSE_RESIDENCY_BIT` flag using the following
structure:
```c
// Provided by VK_VERSION_1_0
typedef struct VkSparseImageMemoryBindInfo {
    VkImage                           image;
    uint32_t                          bindCount;
    const VkSparseImageMemoryBind*    pBinds;
} VkSparseImageMemoryBindInfo;
```

# Members
- [`image`] is the [`Image`] object to be bound
- [`bind_count`] is the number of [`SparseImageMemoryBind`] structures in [`binds`] array
- [`binds`] is a pointer to an array of [`SparseImageMemoryBind`] structures

# Description
## Valid Usage
-    The `subresource.mipLevel` member of each element of [`binds`] **must**  be less than the `mipLevels` specified in [`ImageCreateInfo`] when [`image`] was created
-    The `subresource.arrayLayer` member of each element of [`binds`] **must**  be less than the `arrayLayers` specified in [`ImageCreateInfo`] when [`image`] was created
-  [`image`] **must**  have been created with `VK_IMAGE_CREATE_SPARSE_RESIDENCY_BIT` set

## Valid Usage (Implicit)
-  [`image`] **must**  be a valid [`Image`] handle
-  [`binds`] **must**  be a valid pointer to an array of [`bind_count`] valid [`SparseImageMemoryBind`] structures
-  [`bind_count`] **must**  be greater than `0`

# Related
- [`crate::vulkan1_0`]
- [`BindSparseInfo`]
- [`Image`]
- [`SparseImageMemoryBind`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        