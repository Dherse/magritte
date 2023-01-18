[VkSemaphoreGetWin32HandleInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSemaphoreGetWin32HandleInfoKHR.html) - Structure describing a Win32 handle semaphore export operation

# C Specifications
The [`SemaphoreGetWin32HandleInfoKHR`] structure is defined as:
```c
// Provided by VK_KHR_external_semaphore_win32
typedef struct VkSemaphoreGetWin32HandleInfoKHR {
    VkStructureType                          sType;
    const void*                              pNext;
    VkSemaphore                              semaphore;
    VkExternalSemaphoreHandleTypeFlagBits    handleType;
} VkSemaphoreGetWin32HandleInfoKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`semaphore`] is the semaphore from which state will be exported.
- [`handle_type`] is a [`ExternalSemaphoreHandleTypeFlagBits`] value specifying the type of handle requested.

# Description
The properties of the handle returned depend on the value of
[`handle_type`].
See [`ExternalSemaphoreHandleTypeFlagBits`] for a description of the
properties of the defined external semaphore handle types.
## Valid Usage
-  [`handle_type`] **must**  have been included in [`ExportSemaphoreCreateInfo::handle_types`] when the [`semaphore`]’s current payload was created
-    If [`handle_type`] is defined as an NT handle, [`get_semaphore_win32_handle_khr`] **must**  be called no more than once for each valid unique combination of [`semaphore`] and [`handle_type`]
-  [`semaphore`] **must**  not currently have its payload replaced by an imported payload as described below in [Importing Semaphore Payloads](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-semaphores-importing) unless that imported payload’s handle type was included in [`ExternalSemaphoreProperties::export_from_imported_handle_types`] for [`handle_type`]
-    If [`handle_type`] refers to a handle type with copy payload transference semantics, as defined below in [Importing Semaphore Payloads](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-semaphores-importing), there  **must**  be no queue waiting on [`semaphore`]
-    If [`handle_type`] refers to a handle type with copy payload transference semantics, [`semaphore`] **must**  be signaled, or have an associated [semaphore signal operation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-semaphores-signaling) pending execution
-  [`handle_type`] **must**  be defined as an NT handle or a global share handle

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_SEMAPHORE_GET_WIN32_HANDLE_INFO_KHR`
-  [`p_next`] **must**  be `NULL`
-  [`semaphore`] **must**  be a valid [`Semaphore`] handle
-  [`handle_type`] **must**  be a valid [`ExternalSemaphoreHandleTypeFlagBits`] value

# Related
- [`VK_KHR_external_semaphore_win32`]
- [`ExternalSemaphoreHandleTypeFlagBits`]
- [`Semaphore`]
- [`StructureType`]
- [`get_semaphore_win32_handle_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        