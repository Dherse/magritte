[VkImportSemaphoreFdInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImportSemaphoreFdInfoKHR.html) - Structure specifying POSIX file descriptor to import to a semaphore

# C Specifications
The [`ImportSemaphoreFdInfoKHR`] structure is defined as:
```c
// Provided by VK_KHR_external_semaphore_fd
typedef struct VkImportSemaphoreFdInfoKHR {
    VkStructureType                          sType;
    const void*                              pNext;
    VkSemaphore                              semaphore;
    VkSemaphoreImportFlags                   flags;
    VkExternalSemaphoreHandleTypeFlagBits    handleType;
    int                                      fd;
} VkImportSemaphoreFdInfoKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`semaphore`] is the semaphore into which the payload will be imported.
- [`flags`] is a bitmask of [`SemaphoreImportFlagBits`] specifying additional parameters for the semaphore payload import operation.
- [`handle_type`] is a [`ExternalSemaphoreHandleTypeFlagBits`] value specifying the type of [`fd`].
- [`fd`] is the external handle to import.

# Description
The handle types supported by [`handle_type`] are:
## Valid Usage
-  [`handle_type`] **must**  be a value included in the [Handle Types Supported by [`ImportSemaphoreFdInfoKHR`]](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-semaphore-handletypes-fd) table
-  [`fd`] **must**  obey any requirements listed for [`handle_type`] in [external semaphore handle types compatibility](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#external-semaphore-handle-types-compatibility)
-    If [`handle_type`] is `VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_FD_BIT`, the [`SemaphoreCreateInfo`]::[`flags`] field  **must**  match that of the semaphore from which [`fd`] was exported
-    If [`handle_type`] is `VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_FD_BIT`, the [`SemaphoreTypeCreateInfo::semaphore_type`] field  **must**  match that of the semaphore from which [`fd`] was exported
-    If [`flags`] contains `VK_SEMAPHORE_IMPORT_TEMPORARY_BIT`, the [`SemaphoreTypeCreateInfo::semaphore_type`] field of the semaphore from which [`fd`] was exported  **must**  not be `VK_SEMAPHORE_TYPE_TIMELINE`
If [`handle_type`] is `VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_SYNC_FD_BIT`,
the special value `-1` for [`fd`] is treated like a valid sync file
descriptor referring to an object that has already signaled.
The import operation will succeed and the [`Semaphore`] will have a
temporarily imported payload as if a valid file descriptor had been
provided.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_IMPORT_SEMAPHORE_FD_INFO_KHR`
-  [`p_next`] **must**  be `NULL`
-  [`semaphore`] **must**  be a valid [`Semaphore`] handle
-  [`flags`] **must**  be a valid combination of [`SemaphoreImportFlagBits`] values
-  [`handle_type`] **must**  be a valid [`ExternalSemaphoreHandleTypeFlagBits`] value

## Host Synchronization
- Host access to [`semaphore`] **must**  be externally synchronized

# Related
- [`VK_KHR_external_semaphore_fd`]
- [`ExternalSemaphoreHandleTypeFlagBits`]
- [`Semaphore`]
- [`SemaphoreImportFlags`]
- [`StructureType`]
- [`import_semaphore_fd_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        