[VkSparseImageMemoryRequirements2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSparseImageMemoryRequirements2.html) - (None)

# C Specifications
The [`SparseImageMemoryRequirements2`] structure is defined as:
```c
// Provided by VK_VERSION_1_1
typedef struct VkSparseImageMemoryRequirements2 {
    VkStructureType                    sType;
    void*                              pNext;
    VkSparseImageMemoryRequirements    memoryRequirements;
} VkSparseImageMemoryRequirements2;
```
or the equivalent
```c
// Provided by VK_KHR_get_memory_requirements2
typedef VkSparseImageMemoryRequirements2 VkSparseImageMemoryRequirements2KHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`memory_requirements`] is a [`SparseImageMemoryRequirements`] structure describing the memory requirements of the sparse image.

# Description
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_SPARSE_IMAGE_MEMORY_REQUIREMENTS_2`
-  [`p_next`] **must**  be `NULL`

# Related
- [`crate::vulkan1_1`]
- [`SparseImageMemoryRequirements`]
- [`StructureType`]
- [`get_device_image_sparse_memory_requirements`]
- [`get_device_image_sparse_memory_requirements_khr`]
- [`get_image_sparse_memory_requirements2`]
- [`get_image_sparse_memory_requirements2_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        