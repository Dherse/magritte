[VkDedicatedAllocationMemoryAllocateInfoNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDedicatedAllocationMemoryAllocateInfoNV.html) - Specify a dedicated memory allocation resource

# C Specifications
If the [`p_next`] chain includes a
[`DedicatedAllocationMemoryAllocateInfoNV`] structure, then that
structure includes a handle of the sole buffer or image resource that the
memory  **can**  be bound to.The [`DedicatedAllocationMemoryAllocateInfoNV`] structure is defined as:
```c
// Provided by VK_NV_dedicated_allocation
typedef struct VkDedicatedAllocationMemoryAllocateInfoNV {
    VkStructureType    sType;
    const void*        pNext;
    VkImage            image;
    VkBuffer           buffer;
} VkDedicatedAllocationMemoryAllocateInfoNV;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`image`] is [`crate::Handle::null`] or a handle of an image which this memory will be bound to.
- [`buffer`] is [`crate::Handle::null`] or a handle of a buffer which this memory will be bound to.

# Description
## Valid Usage
-    At least one of [`image`] and [`buffer`] **must**  be [`crate::Handle::null`]
-    If [`image`] is not [`crate::Handle::null`], the image  **must**  have been created with [`DedicatedAllocationImageCreateInfoNV::dedicated_allocation`] equal to [`TRUE`]
-    If [`buffer`] is not [`crate::Handle::null`], the buffer  **must**  have been created with [`DedicatedAllocationBufferCreateInfoNV::dedicated_allocation`] equal to [`TRUE`]
-    If [`image`] is not [`crate::Handle::null`], [`MemoryAllocateInfo::allocation_size`] **must**  equal the [`MemoryRequirements::size`] of the image
-    If [`buffer`] is not [`crate::Handle::null`], [`MemoryAllocateInfo::allocation_size`] **must**  equal the [`MemoryRequirements::size`] of the buffer
-    If [`image`] is not [`crate::Handle::null`] and [`MemoryAllocateInfo`] defines a memory import operation, the memory being imported  **must**  also be a dedicated image allocation and [`image`] **must**  be identical to the image associated with the imported memory
-    If [`buffer`] is not [`crate::Handle::null`] and [`MemoryAllocateInfo`] defines a memory import operation, the memory being imported  **must**  also be a dedicated buffer allocation and [`buffer`] **must**  be identical to the buffer associated with the imported memory

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DEDICATED_ALLOCATION_MEMORY_ALLOCATE_INFO_NV`
-    If [`image`] is not [`crate::Handle::null`], [`image`] **must**  be a valid [`Image`] handle
-    If [`buffer`] is not [`crate::Handle::null`], [`buffer`] **must**  be a valid [`Buffer`] handle
-    Both of [`buffer`], and [`image`] that are valid handles of non-ignored parameters  **must**  have been created, allocated, or retrieved from the same [`Device`]

# Related
- [`VK_NV_dedicated_allocation`]
- [`Buffer`]
- [`Image`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        