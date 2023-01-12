[VkImportMemoryZirconHandleInfoFUCHSIA](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImportMemoryZirconHandleInfoFUCHSIA.html) - Structure specifying import parameters for Zircon handle to external memory

# C Specifications
The [`ImportMemoryZirconHandleInfoFUCHSIA`] structure is defined as:
```c
// Provided by VK_FUCHSIA_external_memory
typedef struct VkImportMemoryZirconHandleInfoFUCHSIA {
    VkStructureType                       sType;
    const void*                           pNext;
    VkExternalMemoryHandleTypeFlagBits    handleType;
    zx_handle_t                           handle;
} VkImportMemoryZirconHandleInfoFUCHSIA;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`handle_type`] is a [`ExternalMemoryHandleTypeFlagBits`] value specifying the type of [`handle`].
- [`handle`] is a `zx_handle_t` (Zircon) handle to the external memory.

# Description
## Valid Usage
-  [`handle_type`] **must**  be `VK_EXTERNAL_MEMORY_HANDLE_TYPE_ZIRCON_VMO_BIT_FUCHSIA`
-  [`handle`] must be a valid VMO handle

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_IMPORT_MEMORY_ZIRCON_HANDLE_INFO_FUCHSIA`
-    If [`handle_type`] is not `0`, [`handle_type`] **must**  be a valid [`ExternalMemoryHandleTypeFlagBits`] value

# Related
- [`fuchsia_external_memory`]
- [`ExternalMemoryHandleTypeFlagBits`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        