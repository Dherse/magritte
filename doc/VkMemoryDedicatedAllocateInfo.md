[VkMemoryDedicatedAllocateInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryDedicatedAllocateInfo.html) - Specify a dedicated memory allocation resource

# C Specifications
If the [`p_next`] chain includes a [`MemoryDedicatedAllocateInfo`]
structure, then that structure includes a handle of the sole buffer or image
resource that the memory  **can**  be bound to.The [`MemoryDedicatedAllocateInfo`] structure is defined as:
```c
// Provided by VK_VERSION_1_1
typedef struct VkMemoryDedicatedAllocateInfo {
    VkStructureType    sType;
    const void*        pNext;
    VkImage            image;
    VkBuffer           buffer;
} VkMemoryDedicatedAllocateInfo;
```
or the equivalent
```c
// Provided by VK_KHR_dedicated_allocation
typedef VkMemoryDedicatedAllocateInfo VkMemoryDedicatedAllocateInfoKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`image`] is [`crate::Handle::null`] or a handle of an image which this memory will be bound to.
- [`buffer`] is [`crate::Handle::null`] or a handle of a buffer which this memory will be bound to.

# Description
## Valid Usage
-    At least one of [`image`] and [`buffer`] **must**  be [`crate::Handle::null`]
-    If [`image`] is not [`crate::Handle::null`] and the memory is not an imported Android Hardware Buffer, [`MemoryAllocateInfo::allocation_size`] **must**  equal the [`MemoryRequirements::size`] of the image
-    If [`image`] is not [`crate::Handle::null`], [`image`] **must**  have been created without `VK_IMAGE_CREATE_SPARSE_BINDING_BIT` set in [`ImageCreateInfo::flags`]
-    If [`buffer`] is not [`crate::Handle::null`] and the memory is not an imported Android Hardware Buffer, [`MemoryAllocateInfo::allocation_size`] **must**  equal the [`MemoryRequirements::size`] of the buffer
-    If [`buffer`] is not [`crate::Handle::null`], [`buffer`] **must**  have been created without `VK_BUFFER_CREATE_SPARSE_BINDING_BIT` set in [`BufferCreateInfo::flags`]
-    If [`image`] is not [`crate::Handle::null`] and [`MemoryAllocateInfo`] defines a memory import operation with handle type `VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT`, `VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT`, `VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_BIT`, `VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_KMT_BIT`, `VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_HEAP_BIT`, or `VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_RESOURCE_BIT`, and the external handle was created by the Vulkan API, then the memory being imported  **must**  also be a dedicated image allocation and [`image`] must be identical to the image associated with the imported memory
-    If [`buffer`] is not [`crate::Handle::null`] and [`MemoryAllocateInfo`] defines a memory import operation with handle type `VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT`, `VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT`, `VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_BIT`, `VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_KMT_BIT`, `VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_HEAP_BIT`, or `VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_RESOURCE_BIT`, and the external handle was created by the Vulkan API, then the memory being imported  **must**  also be a dedicated buffer allocation and [`buffer`] **must**  be identical to the buffer associated with the imported memory
-    If [`image`] is not [`crate::Handle::null`] and [`MemoryAllocateInfo`] defines a memory import operation with handle type `VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_FD_BIT`, the memory being imported  **must**  also be a dedicated image allocation and [`image`] **must**  be identical to the image associated with the imported memory
-    If [`buffer`] is not [`crate::Handle::null`] and [`MemoryAllocateInfo`] defines a memory import operation with handle type `VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_FD_BIT`, the memory being imported  **must**  also be a dedicated buffer allocation and [`buffer`] **must**  be identical to the buffer associated with the imported memory
-    If [`image`] is not [`crate::Handle::null`], [`image`] **must**  not have been created with `VK_IMAGE_CREATE_DISJOINT_BIT` set in [`ImageCreateInfo::flags`]
-    If [`image`] is not [`crate::Handle::null`] and [`MemoryAllocateInfo`] defines a memory import operation with handle type `VK_EXTERNAL_MEMORY_HANDLE_TYPE_ZIRCON_VMO_BIT_FUCHSIA`, the memory being imported  **must**  also be a dedicated image allocation and [`image`] **must**  be identical to the image associated with the imported memory
-    If [`buffer`] is not [`crate::Handle::null`] and [`MemoryAllocateInfo`] defines a memory import operation with handle type `VK_EXTERNAL_MEMORY_HANDLE_TYPE_ZIRCON_VMO_BIT_FUCHSIA`, the memory being imported  **must**  also be a dedicated buffer allocation and [`buffer`] **must**  be identical to the buffer associated with the imported memory

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_MEMORY_DEDICATED_ALLOCATE_INFO`
-    If [`image`] is not [`crate::Handle::null`], [`image`] **must**  be a valid [`Image`] handle
-    If [`buffer`] is not [`crate::Handle::null`], [`buffer`] **must**  be a valid [`Buffer`] handle
-    Both of [`buffer`], and [`image`] that are valid handles of non-ignored parameters  **must**  have been created, allocated, or retrieved from the same [`Device`]

# Related
- [`crate::vulkan1_1`]
- [`Buffer`]
- [`Image`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        