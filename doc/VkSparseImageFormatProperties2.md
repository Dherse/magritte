[VkSparseImageFormatProperties2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSparseImageFormatProperties2.html) - Structure specifying sparse image format properties

# C Specifications
The [`SparseImageFormatProperties2`] structure is defined as:
```c
// Provided by VK_VERSION_1_1
typedef struct VkSparseImageFormatProperties2 {
    VkStructureType                  sType;
    void*                            pNext;
    VkSparseImageFormatProperties    properties;
} VkSparseImageFormatProperties2;
```
or the equivalent
```c
// Provided by VK_KHR_get_physical_device_properties2
typedef VkSparseImageFormatProperties2 VkSparseImageFormatProperties2KHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`properties`] is a [`SparseImageFormatProperties`] structure which is populated with the same values as in [`get_physical_device_sparse_image_format_properties`].

# Description
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_SPARSE_IMAGE_FORMAT_PROPERTIES_2`
-  [`p_next`] **must**  be `NULL`

# Related
- [`crate::vulkan1_1`]
- [`SparseImageFormatProperties`]
- [`StructureType`]
- [`get_physical_device_sparse_image_format_properties2`]
- [`get_physical_device_sparse_image_format_properties2_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        