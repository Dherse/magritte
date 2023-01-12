[VkExportMemoryWin32HandleInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExportMemoryWin32HandleInfoKHR.html) - Structure specifying additional attributes of Windows handles exported from a memory

# C Specifications
To specify additional attributes of NT handles exported from a memory
object, add a [`ExportMemoryWin32HandleInfoKHR`] structure to the
[`p_next`] chain of the [`MemoryAllocateInfo`] structure.
The [`ExportMemoryWin32HandleInfoKHR`] structure is defined as:
```c
// Provided by VK_KHR_external_memory_win32
typedef struct VkExportMemoryWin32HandleInfoKHR {
    VkStructureType               sType;
    const void*                   pNext;
    const SECURITY_ATTRIBUTES*    pAttributes;
    DWORD                         dwAccess;
    LPCWSTR                       name;
} VkExportMemoryWin32HandleInfoKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`attributes`] is a pointer to a Windows `SECURITY_ATTRIBUTES` structure specifying security attributes of the handle.
- [`dw_access`] is a `DWORD` specifying access rights of the handle.
- [`name`] is a null-terminated UTF-16 string to associate with the payload referenced by NT handles exported from the created memory.

# Description
If [`ExportMemoryAllocateInfo`] is not included in the same [`p_next`]
chain, this structure is ignored.If [`ExportMemoryAllocateInfo`] is included in the [`p_next`] chain of
[`MemoryAllocateInfo`] with a Windows `handleType`, but either
[`ExportMemoryWin32HandleInfoKHR`] is not included in the [`p_next`]
chain, or if it is but [`attributes`] is set to `NULL`, default security
descriptor values will be used, and child processes created by the
application will not inherit the handle, as described in the MSDN
documentation for “Synchronization Object Security and Access Rights”<sup>1</sup>.
Further, if the structure is not present, the access rights used depend on
the handle type.For handles of the following types:
- `VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT`
The implementation  **must**  ensure the access rights allow read and write
access to the memory.
* [https://docs.microsoft.com/en-us/windows/win32/sync/synchronization-object-security-and-access-rights](https://docs.microsoft.com/en-us/windows/win32/sync/synchronization-object-security-and-access-rights)

## Valid Usage
-    If [`ExportMemoryAllocateInfo::handle_types`] does not include `VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT`, a [`ExportMemoryWin32HandleInfoKHR`] structure  **must**  not be included in the [`p_next`] chain of [`MemoryAllocateInfo`]

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_EXPORT_MEMORY_WIN32_HANDLE_INFO_KHR`
-    If [`attributes`] is not `NULL`, [`attributes`] **must**  be a valid pointer to a valid `SECURITY_ATTRIBUTES` value

# Related
- [`khr_external_memory_win32`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        