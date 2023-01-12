[VkImportMemoryWin32HandleInfoNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImportMemoryWin32HandleInfoNV.html) - Import Win32 memory created on the same physical device

# C Specifications
To import memory created on the same physical device but outside of the
current Vulkan instance, add a [`ImportMemoryWin32HandleInfoNV`]
structure to the [`p_next`] chain of the [`MemoryAllocateInfo`]
structure, specifying a handle to and the type of the memory.The [`ImportMemoryWin32HandleInfoNV`] structure is defined as:
```c
// Provided by VK_NV_external_memory_win32
typedef struct VkImportMemoryWin32HandleInfoNV {
    VkStructureType                      sType;
    const void*                          pNext;
    VkExternalMemoryHandleTypeFlagsNV    handleType;
    HANDLE                               handle;
} VkImportMemoryWin32HandleInfoNV;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`handle_type`] is `0` or a [`ExternalMemoryHandleTypeFlagBitsNV`] value specifying the type of memory handle in [`handle`].
- [`handle`] is a Windows `HANDLE` referring to the memory.

# Description
If [`handle_type`] is `0`, this structure is ignored by consumers of the
[`MemoryAllocateInfo`] structure it is chained from.
## Valid Usage
-  [`handle_type`] **must**  not have more than one bit set
-  [`handle`] **must**  be a valid handle to memory, obtained as specified by [`handle_type`]

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_IMPORT_MEMORY_WIN32_HANDLE_INFO_NV`
-  [`handle_type`] **must**  be a valid combination of [`ExternalMemoryHandleTypeFlagBitsNV`] values

# Related
- [`nv_external_memory_win32`]
- [VkExternalMemoryHandleTypeFlagsNV]()
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        