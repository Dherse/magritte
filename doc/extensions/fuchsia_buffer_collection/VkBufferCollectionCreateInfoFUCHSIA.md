[VkBufferCollectionCreateInfoFUCHSIA](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBufferCollectionCreateInfoFUCHSIA.html) - Structure specifying desired parameters to create the buffer collection

# C Specifications
The [`BufferCollectionCreateInfoFUCHSIA`] structure is defined as:
```c
// Provided by VK_FUCHSIA_buffer_collection
typedef struct VkBufferCollectionCreateInfoFUCHSIA {
    VkStructureType    sType;
    const void*        pNext;
    zx_handle_t        collectionToken;
} VkBufferCollectionCreateInfoFUCHSIA;
```

# Members
- [`s_type`] is the type of this structure
- [`p_next`] is `NULL` or a pointer to a structure extending this structure
- [`collection_token`] is a `zx_handle_t` containing the Sysmem client’s buffer collection token

# Description
## Valid Usage
-  [`collection_token`] **must**  be a valid `zx_handle_t` to a Zircon channel allocated from Sysmem (`fuchsia.sysmem.Allocator`/AllocateSharedCollection) with `ZX_DEFAULT_CHANNEL_RIGHTS` rights

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_BUFFER_COLLECTION_CREATE_INFO_FUCHSIA`
-  [`p_next`] **must**  be `NULL`

# Related
- [`fuchsia_buffer_collection`]
- [`StructureType`]
- [`create_buffer_collection_fuchsia`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        