[VkImportSemaphoreWin32HandleInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImportSemaphoreWin32HandleInfoKHR.html) - Structure specifying Windows handle to import to a semaphore

# C Specifications
The [`ImportSemaphoreWin32HandleInfoKHR`] structure is defined as:
```c
// Provided by VK_KHR_external_semaphore_win32
typedef struct VkImportSemaphoreWin32HandleInfoKHR {
    VkStructureType                          sType;
    const void*                              pNext;
    VkSemaphore                              semaphore;
    VkSemaphoreImportFlags                   flags;
    VkExternalSemaphoreHandleTypeFlagBits    handleType;
    HANDLE                                   handle;
    LPCWSTR                                  name;
} VkImportSemaphoreWin32HandleInfoKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`semaphore`] is the semaphore into which the payload will be imported.
- [`flags`] is a bitmask of [`SemaphoreImportFlagBits`] specifying additional parameters for the semaphore payload import operation.
- [`handle_type`] is a [`ExternalSemaphoreHandleTypeFlagBits`] value specifying the type of [`handle`].
- [`handle`] is `NULL` or the external handle to import.
- [`name`] is `NULL` or a null-terminated UTF-16 string naming the underlying synchronization primitive to import.

# Description
The handle types supported by [`handle_type`] are:
## Valid Usage
-  [`handle_type`] **must**  be a value included in the [Handle Types Supported by [`ImportSemaphoreWin32HandleInfoKHR`]](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-semaphore-handletypes-win32) table
-    If [`handle_type`] is not `VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_BIT` or `VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE_BIT`, [`name`] **must**  be `NULL`
-    If [`handle`] is `NULL`, [`name`] **must**  name a valid synchronization primitive of the type specified by [`handle_type`]
-    If [`name`] is `NULL`, [`handle`] **must**  be a valid handle of the type specified by [`handle_type`]
-    If [`handle`] is not `NULL`, [`name`] **must**  be `NULL`
-    If [`handle`] is not `NULL`, it  **must**  obey any requirements listed for [`handle_type`] in [external semaphore handle types compatibility](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#external-semaphore-handle-types-compatibility)
-    If [`name`] is not `NULL`, it  **must**  obey any requirements listed for [`handle_type`] in [external semaphore handle types compatibility](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#external-semaphore-handle-types-compatibility)
-    If [`handle_type`] is `VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_BIT` or `VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT`, the [`SemaphoreCreateInfo`]::[`flags`] field  **must**  match that of the semaphore from which [`handle`] or [`name`] was exported
-    If [`handle_type`] is `VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_BIT` or `VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT`, the [`SemaphoreTypeCreateInfo::semaphore_type`] field  **must**  match that of the semaphore from which [`handle`] or [`name`] was exported
-    If [`flags`] contains `VK_SEMAPHORE_IMPORT_TEMPORARY_BIT`, the [`SemaphoreTypeCreateInfo::semaphore_type`] field of the semaphore from which [`handle`] or [`name`] was exported  **must**  not be `VK_SEMAPHORE_TYPE_TIMELINE`

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_IMPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR`
-  [`p_next`] **must**  be `NULL`
-  [`semaphore`] **must**  be a valid [`Semaphore`] handle
-  [`flags`] **must**  be a valid combination of [`SemaphoreImportFlagBits`] values

## Host Synchronization
- Host access to [`semaphore`] **must**  be externally synchronized

# Related
- [`VK_KHR_external_semaphore_win32`]
- [`ExternalSemaphoreHandleTypeFlagBits`]
- [`Semaphore`]
- [`SemaphoreImportFlags`]
- [`StructureType`]
- [`import_semaphore_win32_handle_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        