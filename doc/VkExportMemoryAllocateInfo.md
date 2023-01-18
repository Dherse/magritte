[VkExportMemoryAllocateInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExportMemoryAllocateInfo.html) - Specify exportable handle types for a device memory object

# C Specifications
When allocating memory whose payload  **may**  be exported to another process or
Vulkan instance, add a [`ExportMemoryAllocateInfo`] structure to the
[`p_next`] chain of the [`MemoryAllocateInfo`] structure, specifying
the handle types that  **may**  be exported.The [`ExportMemoryAllocateInfo`] structure is defined as:
```c
// Provided by VK_VERSION_1_1
typedef struct VkExportMemoryAllocateInfo {
    VkStructureType                    sType;
    const void*                        pNext;
    VkExternalMemoryHandleTypeFlags    handleTypes;
} VkExportMemoryAllocateInfo;
```
or the equivalent
```c
// Provided by VK_KHR_external_memory
typedef VkExportMemoryAllocateInfo VkExportMemoryAllocateInfoKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`handle_types`] is a bitmask of [`ExternalMemoryHandleTypeFlagBits`] specifying one or more memory handle types the application  **can**  export from the resulting allocation. The application  **can**  request multiple handle types for the same allocation.

# Description
## Valid Usage
-    The bits in [`handle_types`] **must**  be supported and compatible, as reported by [`ExternalImageFormatProperties`] or [`ExternalBufferProperties`]

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_EXPORT_MEMORY_ALLOCATE_INFO`
-  [`handle_types`] **must**  be a valid combination of [`ExternalMemoryHandleTypeFlagBits`] values

# Related
- [`crate::vulkan1_1`]
- [`ExternalMemoryHandleTypeFlags`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        