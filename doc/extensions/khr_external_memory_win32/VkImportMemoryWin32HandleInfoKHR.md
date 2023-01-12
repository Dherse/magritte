[VkImportMemoryWin32HandleInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImportMemoryWin32HandleInfoKHR.html) - Import Win32 memory created on the same physical device

# C Specifications
To import memory from a Windows handle, add a
[`ImportMemoryWin32HandleInfoKHR`] structure to the [`p_next`] chain of
the [`MemoryAllocateInfo`] structure.The [`ImportMemoryWin32HandleInfoKHR`] structure is defined as:
```c
// Provided by VK_KHR_external_memory_win32
typedef struct VkImportMemoryWin32HandleInfoKHR {
    VkStructureType                       sType;
    const void*                           pNext;
    VkExternalMemoryHandleTypeFlagBits    handleType;
    HANDLE                                handle;
    LPCWSTR                               name;
} VkImportMemoryWin32HandleInfoKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`handle_type`] is a [`ExternalMemoryHandleTypeFlagBits`] value specifying the type of [`handle`] or [`name`].
- [`handle`] is `NULL` or the external handle to import.
- [`name`] is `NULL` or a null-terminated UTF-16 string naming the payload to import.

# Description
Importing memory object payloads from Windows handles does not transfer
ownership of the handle to the Vulkan implementation.
For handle types defined as NT handles, the application  **must**  release handle
ownership using the `CloseHandle` system call when the handle is no
longer needed.
For handle types defined as NT handles, the imported memory object holds a
reference to its payload.Applications  **can**  import the same payload into multiple instances of Vulkan,
into the same instance from which it was exported, and multiple times into a
given Vulkan instance.
In all cases, each import operation  **must**  create a distinct
[`DeviceMemory`] object.
## Valid Usage
-    If [`handle_type`] is not `0`, it  **must**  be supported for import, as reported by [`ExternalImageFormatProperties`] or [`ExternalBufferProperties`]
-    The memory from which [`handle`] was exported, or the memory named by [`name`] **must**  have been created on the same underlying physical device as `device`
-    If [`handle_type`] is not `0`, it  **must**  be defined as an NT handle or a global share handle
-    If [`handle_type`] is not `VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT`, `VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_BIT`, `VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_HEAP_BIT`, or `VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_RESOURCE_BIT`, [`name`] **must**  be `NULL`
-    If [`handle_type`] is not `0` and [`handle`] is `NULL`, [`name`] **must**  name a valid memory resource of the type specified by [`handle_type`]
-    If [`handle_type`] is not `0` and [`name`] is `NULL`, [`handle`] **must**  be a valid handle of the type specified by [`handle_type`]
-    if [`handle`] is not `NULL`, [`name`] **must**  be `NULL`
-    If [`handle`] is not `NULL`, it  **must**  obey any requirements listed for [`handle_type`] in [external memory handle types compatibility](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#external-memory-handle-types-compatibility)
-    If [`name`] is not `NULL`, it  **must**  obey any requirements listed for [`handle_type`] in [external memory handle types compatibility](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#external-memory-handle-types-compatibility)

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_IMPORT_MEMORY_WIN32_HANDLE_INFO_KHR`
-    If [`handle_type`] is not `0`, [`handle_type`] **must**  be a valid [`ExternalMemoryHandleTypeFlagBits`] value

# Related
- [`khr_external_memory_win32`]
- [`ExternalMemoryHandleTypeFlagBits`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        