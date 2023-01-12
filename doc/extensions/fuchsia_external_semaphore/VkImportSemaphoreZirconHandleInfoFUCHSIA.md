[VkImportSemaphoreZirconHandleInfoFUCHSIA](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImportSemaphoreZirconHandleInfoFUCHSIA.html) - Structure specifying Zircon event handle to import to a semaphore

# C Specifications
The [`ImportSemaphoreZirconHandleInfoFUCHSIA`] structure is defined as:
```c
// Provided by VK_FUCHSIA_external_semaphore
typedef struct VkImportSemaphoreZirconHandleInfoFUCHSIA {
    VkStructureType                          sType;
    const void*                              pNext;
    VkSemaphore                              semaphore;
    VkSemaphoreImportFlags                   flags;
    VkExternalSemaphoreHandleTypeFlagBits    handleType;
    zx_handle_t                              zirconHandle;
} VkImportSemaphoreZirconHandleInfoFUCHSIA;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`semaphore`] is the semaphore into which the payload will be imported.
- [`flags`] is a bitmask of [`SemaphoreImportFlagBits`] specifying additional parameters for the semaphore payload import operation.
- [`handle_type`] is a [`ExternalSemaphoreHandleTypeFlagBits`] value specifying the type of [`zircon_handle`].
- [`zircon_handle`] is the external handle to import.

# Description
The handle types supported by [`handle_type`] are:
## Valid Usage
-  [`handle_type`] **must**  be a value included in the [Handle Types Supported by [`ImportSemaphoreZirconHandleInfoFUCHSIA`]](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-semaphore-handletypes-fuchsia) table
-  [`zircon_handle`] **must**  obey any requirements listed for [`handle_type`] in [external semaphore handle types compatibility](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#external-semaphore-handle-types-compatibility)
-  [`zircon_handle`] **must**  have `ZX_RIGHTS_BASIC` and `ZX_RIGHTS_SIGNAL` rights
-    The [`SemaphoreTypeCreateInfo::semaphore_type`] field  **must**  not be `VK_SEMAPHORE_TYPE_TIMELINE`

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_IMPORT_SEMAPHORE_ZIRCON_HANDLE_INFO_FUCHSIA`
-  [`p_next`] **must**  be `NULL`
-  [`semaphore`] **must**  be a valid [`Semaphore`] handle
-  [`flags`] **must**  be a valid combination of [`SemaphoreImportFlagBits`] values
-  [`handle_type`] **must**  be a valid [`ExternalSemaphoreHandleTypeFlagBits`] value

## Host Synchronization
- Host access to [`semaphore`] **must**  be externally synchronized

# Related
- [`fuchsia_external_semaphore`]
- [`ExternalSemaphoreHandleTypeFlagBits`]
- [`Semaphore`]
- [VkSemaphoreImportFlags]()
- [`StructureType`]
- [`import_semaphore_zircon_handle_fuchsia`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        