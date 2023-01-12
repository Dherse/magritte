[VkImageSparseMemoryRequirementsInfo2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageSparseMemoryRequirementsInfo2.html) - (None)

# C Specifications
The [`ImageSparseMemoryRequirementsInfo2`] structure is defined as:
```c
// Provided by VK_VERSION_1_1
typedef struct VkImageSparseMemoryRequirementsInfo2 {
    VkStructureType    sType;
    const void*        pNext;
    VkImage            image;
} VkImageSparseMemoryRequirementsInfo2;
```
or the equivalent
```c
// Provided by VK_KHR_get_memory_requirements2
typedef VkImageSparseMemoryRequirementsInfo2 VkImageSparseMemoryRequirementsInfo2KHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`image`] is the image to query.

# Description
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2`
-  [`p_next`] **must**  be `NULL`
-  [`image`] **must**  be a valid [`Image`] handle

# Related
- [`crate::vulkan1_1`]
- [`Image`]
- [`StructureType`]
- [`get_image_sparse_memory_requirements2`]
- [`get_image_sparse_memory_requirements2_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        