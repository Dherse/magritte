[vkImportSemaphoreZirconHandleFUCHSIA](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkImportSemaphoreZirconHandleFUCHSIA.html) - Import a semaphore from a Zircon event handle

# C Specifications
To import a semaphore payload from a Zircon event handle, call:
```c
// Provided by VK_FUCHSIA_external_semaphore
VkResult vkImportSemaphoreZirconHandleFUCHSIA(
    VkDevice                                    device,
    const VkImportSemaphoreZirconHandleInfoFUCHSIA* pImportSemaphoreZirconHandleInfo);
```

# Parameters
- [`device`] is the logical device that created the semaphore.
- [`p_import_semaphore_zircon_handle_info`] is a pointer to a [`ImportSemaphoreZirconHandleInfoFUCHSIA`] structure specifying the semaphore and import parameters.

# Description
Importing a semaphore payload from a Zircon event handle transfers ownership
of the handle from the application to the Vulkan implementation.
The application  **must**  not perform any operations on the handle after a
successful import.Applications  **can**  import the same semaphore payload into multiple instances
of Vulkan, into the same instance from which it was exported, and multiple
times into a given Vulkan instance.
## Valid Usage
-  `semaphore` **must**  not be associated with any queue command that has not yet completed execution on that queue

## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`p_import_semaphore_zircon_handle_info`] **must**  be a valid pointer to a valid [`ImportSemaphoreZirconHandleInfoFUCHSIA`] structure

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_INVALID_EXTERNAL_HANDLE`

# Related
- [`fuchsia_external_semaphore`]
- [`Device`]
- [`ImportSemaphoreZirconHandleInfoFUCHSIA`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        