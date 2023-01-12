[VkExportMemoryWin32HandleInfoNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExportMemoryWin32HandleInfoNV.html) - Specify security attributes and access rights for Win32 memory handles

# C Specifications
When [`ExportMemoryAllocateInfoNV::handle_types`] includes
`VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT_NV`, add a
[`ExportMemoryWin32HandleInfoNV`] structure to the [`p_next`] chain of
the [`ExportMemoryAllocateInfoNV`] structure to specify security
attributes and access rights for the memory object’s external handle.The [`ExportMemoryWin32HandleInfoNV`] structure is defined as:
```c
// Provided by VK_NV_external_memory_win32
typedef struct VkExportMemoryWin32HandleInfoNV {
    VkStructureType               sType;
    const void*                   pNext;
    const SECURITY_ATTRIBUTES*    pAttributes;
    DWORD                         dwAccess;
} VkExportMemoryWin32HandleInfoNV;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`attributes`] is a pointer to a Windows `SECURITY_ATTRIBUTES` structure specifying security attributes of the handle.
- [`dw_access`] is a `DWORD` specifying access rights of the handle.

# Description
If this structure is not present, or if [`attributes`] is set to `NULL`,
default security descriptor values will be used, and child processes created
by the application will not inherit the handle, as described in the MSDN
documentation for “Synchronization Object Security and Access Rights”<sup>1</sup>.
Further, if the structure is not present, the access rights will be`DXGI_SHARED_RESOURCE_READ` | `DXGI_SHARED_RESOURCE_WRITE`
* [https://docs.microsoft.com/en-us/windows/win32/sync/synchronization-object-security-and-access-rights](https://docs.microsoft.com/en-us/windows/win32/sync/synchronization-object-security-and-access-rights)

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_EXPORT_MEMORY_WIN32_HANDLE_INFO_NV`
-    If [`attributes`] is not `NULL`, [`attributes`] **must**  be a valid pointer to a valid `SECURITY_ATTRIBUTES` value

# Related
- [`nv_external_memory_win32`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        