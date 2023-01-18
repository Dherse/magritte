[VkBufferCollectionImageCreateInfoFUCHSIA](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBufferCollectionImageCreateInfoFUCHSIA.html) - Create a VkBufferCollectionFUCHSIA-compatible VkImage

# C Specifications
The [`BufferCollectionImageCreateInfoFUCHSIA`] structure is defined as:
```c
// Provided by VK_FUCHSIA_buffer_collection
typedef struct VkBufferCollectionImageCreateInfoFUCHSIA {
    VkStructureType              sType;
    const void*                  pNext;
    VkBufferCollectionFUCHSIA    collection;
    uint32_t                     index;
} VkBufferCollectionImageCreateInfoFUCHSIA;
```

# Members
- [`s_type`] is the type of this structure
- [`p_next`] is `NULL` or a pointer to a structure extending this structure
- [`collection`] is the [`BufferCollectionFUCHSIA`] handle
- [`index`] is the index of the buffer in the buffer collection from which the memory will be imported

# Description
## Valid Usage
-  [`index`] **must**  be less than [`BufferCollectionPropertiesFUCHSIA::buffer_count`]

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_BUFFER_COLLECTION_IMAGE_CREATE_INFO_FUCHSIA`
-  [`collection`] **must**  be a valid [`BufferCollectionFUCHSIA`] handle

# Related
- [`VK_FUCHSIA_buffer_collection`]
- [`BufferCollectionFUCHSIA`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        