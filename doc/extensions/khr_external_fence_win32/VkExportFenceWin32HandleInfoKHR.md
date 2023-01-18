[VkExportFenceWin32HandleInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExportFenceWin32HandleInfoKHR.html) - Structure specifying additional attributes of Windows handles exported from a fence

# C Specifications
To specify additional attributes of NT handles exported from a fence, add a
[`ExportFenceWin32HandleInfoKHR`] structure to the [`p_next`] chain of
the [`FenceCreateInfo`] structure.
The [`ExportFenceWin32HandleInfoKHR`] structure is defined as:
```c
// Provided by VK_KHR_external_fence_win32
typedef struct VkExportFenceWin32HandleInfoKHR {
    VkStructureType               sType;
    const void*                   pNext;
    const SECURITY_ATTRIBUTES*    pAttributes;
    DWORD                         dwAccess;
    LPCWSTR                       name;
} VkExportFenceWin32HandleInfoKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`attributes`] is a pointer to a Windows [`SECURITY_ATTRIBUTES`] structure specifying security attributes of the handle.
- [`dw_access`] is a [`DWORD`] specifying access rights of the handle.
- [`name`] is a null-terminated UTF-16 string to associate with the underlying synchronization primitive referenced by NT handles exported from the created fence.

# Description
If [`ExportFenceCreateInfo`] is not inluded in the same [`p_next`]
chain, this structure is ignored.If [`ExportFenceCreateInfo`] is included in the [`p_next`] chain of
[`FenceCreateInfo`] with a Windows `handleType`, but either
[`ExportFenceWin32HandleInfoKHR`] is not included in the [`p_next`]
chain, or if it is but [`attributes`] is set to `NULL`, default security
descriptor values will be used, and child processes created by the
application will not inherit the handle, as described in the MSDN
documentation for “Synchronization Object Security and Access Rights”<sup>1</sup>.
Further, if the structure is not present, the access rights will be`DXGI_SHARED_RESOURCE_READ` | `DXGI_SHARED_RESOURCE_WRITE`for handles of the following types:`VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_BIT`
* [https://docs.microsoft.com/en-us/windows/win32/sync/synchronization-object-security-and-access-rights](https://docs.microsoft.com/en-us/windows/win32/sync/synchronization-object-security-and-access-rights)

## Valid Usage
-    If [`ExportFenceCreateInfo::handle_types`] does not include `VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_BIT`, a [`ExportFenceWin32HandleInfoKHR`] structure  **must**  not be included in the [`p_next`] chain of [`FenceCreateInfo`]

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_EXPORT_FENCE_WIN32_HANDLE_INFO_KHR`
-    If [`attributes`] is not `NULL`, [`attributes`] **must**  be a valid pointer to a valid [`SECURITY_ATTRIBUTES`] value

# Related
- [`VK_KHR_external_fence_win32`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        