[VkExportMemoryAllocateInfoNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExportMemoryAllocateInfoNV.html) - Specify memory handle types that may be exported

# C Specifications
The [`ExportMemoryAllocateInfoNV`] structure is defined as:
```c
// Provided by VK_NV_external_memory
typedef struct VkExportMemoryAllocateInfoNV {
    VkStructureType                      sType;
    const void*                          pNext;
    VkExternalMemoryHandleTypeFlagsNV    handleTypes;
} VkExportMemoryAllocateInfoNV;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`handle_types`] is a bitmask of [`ExternalMemoryHandleTypeFlagBitsNV`] specifying one or more memory handle types that  **may**  be exported. Multiple handle types  **may**  be requested for the same allocation as long as they are compatible, as reported by [`get_physical_device_external_image_format_properties_nv`].

# Description
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_EXPORT_MEMORY_ALLOCATE_INFO_NV`
-  [`handle_types`] **must**  be a valid combination of [`ExternalMemoryHandleTypeFlagBitsNV`] values

# Related
- [`VK_NV_external_memory`]
- [`ExternalMemoryHandleTypeFlagsNV`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        