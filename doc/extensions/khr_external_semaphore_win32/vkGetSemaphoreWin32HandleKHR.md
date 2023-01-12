[vkGetSemaphoreWin32HandleKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetSemaphoreWin32HandleKHR.html) - Get a Windows HANDLE for a semaphore

# C Specifications
To export a Windows handle representing the payload of a semaphore, call:
```c
// Provided by VK_KHR_external_semaphore_win32
VkResult vkGetSemaphoreWin32HandleKHR(
    VkDevice                                    device,
    const VkSemaphoreGetWin32HandleInfoKHR*     pGetWin32HandleInfo,
    HANDLE*                                     pHandle);
```

# Parameters
- [`device`] is the logical device that created the semaphore being exported.
- [`p_get_win32_handle_info`] is a pointer to a [`SemaphoreGetWin32HandleInfoKHR`] structure containing parameters of the export operation.
- [`p_handle`] will return the Windows handle representing the semaphore state.

# Description
For handle types defined as NT handles, the handles returned by
[`get_semaphore_win32_handle_khr`] are owned by the application.
To avoid leaking resources, the application  **must**  release ownership of them
using the `CloseHandle` system call when they are no longer needed.Exporting a Windows handle from a semaphore  **may**  have side effects depending
on the transference of the specified handle type, as described in
[Importing Semaphore Payloads](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-semaphores-importing).
## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`p_get_win32_handle_info`] **must**  be a valid pointer to a valid [`SemaphoreGetWin32HandleInfoKHR`] structure
-  [`p_handle`] **must**  be a valid pointer to a `HANDLE` value

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_TOO_MANY_OBJECTS`  - `VK_ERROR_OUT_OF_HOST_MEMORY`

# Related
- [`khr_external_semaphore_win32`]
- [`Device`]
- [`SemaphoreGetWin32HandleInfoKHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        