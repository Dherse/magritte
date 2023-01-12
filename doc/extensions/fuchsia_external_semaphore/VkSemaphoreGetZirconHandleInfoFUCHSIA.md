[VkSemaphoreGetZirconHandleInfoFUCHSIA](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSemaphoreGetZirconHandleInfoFUCHSIA.html) - Structure describing a Zircon event handle semaphore export operation

# C Specifications
The [`SemaphoreGetZirconHandleInfoFUCHSIA`] structure is defined as:
```c
// Provided by VK_FUCHSIA_external_semaphore
typedef struct VkSemaphoreGetZirconHandleInfoFUCHSIA {
    VkStructureType                          sType;
    const void*                              pNext;
    VkSemaphore                              semaphore;
    VkExternalSemaphoreHandleTypeFlagBits    handleType;
} VkSemaphoreGetZirconHandleInfoFUCHSIA;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`semaphore`] is the semaphore from which state will be exported.
- [`handle_type`] is a [`ExternalSemaphoreHandleTypeFlagBits`] value specifying the type of handle requested.

# Description
The properties of the Zircon event handle returned depend on the value of
[`handle_type`].
See [`ExternalSemaphoreHandleTypeFlagBits`] for a description of the
properties of the defined external semaphore handle types.
## Valid Usage
-  [`handle_type`] **must**  have been included in [`ExportSemaphoreCreateInfo::handle_types`] when [`semaphore`]’s current payload was created
-  [`semaphore`] **must**  not currently have its payload replaced by an imported payload as described below in [Importing Semaphore Payloads](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-semaphores-importing) unless that imported payload’s handle type was included in [`ExternalSemaphoreProperties::export_from_imported_handle_types`] for [`handle_type`]
-    If [`handle_type`] refers to a handle type with copy payload transference semantics, as defined below in [Importing Semaphore Payloads](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-semaphores-importing), there  **must**  be no queue waiting on [`semaphore`]
-    If [`handle_type`] refers to a handle type with copy payload transference semantics, [`semaphore`] **must**  be signaled, or have an associated [semaphore signal operation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-semaphores-signaling) pending execution
-  [`handle_type`] **must**  be defined as a Zircon event handle
-  [`semaphore`] **must**  have been created with a [`SemaphoreType`] of `VK_SEMAPHORE_TYPE_BINARY`

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_SEMAPHORE_GET_ZIRCON_HANDLE_INFO_FUCHSIA`
-  [`p_next`] **must**  be `NULL`
-  [`semaphore`] **must**  be a valid [`Semaphore`] handle
-  [`handle_type`] **must**  be a valid [`ExternalSemaphoreHandleTypeFlagBits`] value

# Related
- [`fuchsia_external_semaphore`]
- [`ExternalSemaphoreHandleTypeFlagBits`]
- [`Semaphore`]
- [`StructureType`]
- [`get_semaphore_zircon_handle_fuchsia`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        