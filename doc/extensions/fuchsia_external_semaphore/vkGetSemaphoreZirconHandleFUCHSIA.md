[vkGetSemaphoreZirconHandleFUCHSIA](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetSemaphoreZirconHandleFUCHSIA.html) - Get a Zircon event handle for a semaphore

# C Specifications
To export a Zircon event handle representing the payload of a semaphore,
call:
```c
// Provided by VK_FUCHSIA_external_semaphore
VkResult vkGetSemaphoreZirconHandleFUCHSIA(
    VkDevice                                    device,
    const VkSemaphoreGetZirconHandleInfoFUCHSIA* pGetZirconHandleInfo,
    zx_handle_t*                                pZirconHandle);
```

# Parameters
- [`device`] is the logical device that created the semaphore being exported.
- [`p_get_zircon_handle_info`] is a pointer to a [`SemaphoreGetZirconHandleInfoFUCHSIA`] structure containing parameters of the export operation.
- [`p_zircon_handle`] will return the Zircon event handle representing the semaphore payload.

# Description
Each call to [`get_semaphore_zircon_handle_fuchsia`] **must**  create a Zircon
event handle and transfer ownership of it to the application.
To avoid leaking resources, the application  **must**  release ownership of the
Zircon event handle when it is no longer needed.Exporting a Zircon event handle from a semaphore  **may**  have side effects
depending on the transference of the specified handle type, as described in
[Importing Semaphore State](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-semaphores-importing).
## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`p_get_zircon_handle_info`] **must**  be a valid pointer to a valid [`SemaphoreGetZirconHandleInfoFUCHSIA`] structure
-  [`p_zircon_handle`] **must**  be a valid pointer to a `zx_handle_t` value

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_TOO_MANY_OBJECTS`  - `VK_ERROR_OUT_OF_HOST_MEMORY`

# Related
- [`fuchsia_external_semaphore`]
- [`Device`]
- [`SemaphoreGetZirconHandleInfoFUCHSIA`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        