[VkImportMemoryBufferCollectionFUCHSIA](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImportMemoryBufferCollectionFUCHSIA.html) - Structure to specify the Sysmem buffer to import

# C Specifications
The [`ImportMemoryBufferCollectionFUCHSIA`] structure is defined as:
```c
// Provided by VK_FUCHSIA_buffer_collection
typedef struct VkImportMemoryBufferCollectionFUCHSIA {
    VkStructureType              sType;
    const void*                  pNext;
    VkBufferCollectionFUCHSIA    collection;
    uint32_t                     index;
} VkImportMemoryBufferCollectionFUCHSIA;
```

# Members
- [`s_type`] is the type of this structure
- [`p_next`] is `NULL` or a pointer to a structure extending this structure
- [`collection`] is the [`BufferCollectionFUCHSIA`] handle
- [`index`] the index of the buffer to import from [`collection`]

# Description
## Valid Usage
-  [`index`] **must**  be less than the value retrieved as [`BufferCollectionPropertiesFUCHSIA`]:bufferCount

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_IMPORT_MEMORY_BUFFER_COLLECTION_FUCHSIA`
-  [`collection`] **must**  be a valid [`BufferCollectionFUCHSIA`] handle

# Related
- [`fuchsia_buffer_collection`]
- [`BufferCollectionFUCHSIA`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        