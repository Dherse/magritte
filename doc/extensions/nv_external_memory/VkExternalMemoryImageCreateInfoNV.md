[VkExternalMemoryImageCreateInfoNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExternalMemoryImageCreateInfoNV.html) - Specify that an image may be backed by external memory

# C Specifications
If the [`p_next`] chain includes a [`ExternalMemoryImageCreateInfoNV`]
structure, then that structure defines a set of external memory handle types
that  **may**  be used as backing store for the image.The [`ExternalMemoryImageCreateInfoNV`] structure is defined as:
```c
// Provided by VK_NV_external_memory
typedef struct VkExternalMemoryImageCreateInfoNV {
    VkStructureType                      sType;
    const void*                          pNext;
    VkExternalMemoryHandleTypeFlagsNV    handleTypes;
} VkExternalMemoryImageCreateInfoNV;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`handle_types`] is zero, or a bitmask of [`ExternalMemoryHandleTypeFlagBitsNV`] specifying one or more external memory handle types.

# Description
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_IMAGE_CREATE_INFO_NV`
-  [`handle_types`] **must**  be a valid combination of [`ExternalMemoryHandleTypeFlagBitsNV`] values

# Related
- [`nv_external_memory`]
- [VkExternalMemoryHandleTypeFlagsNV]()
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        