[vkImportSemaphoreWin32HandleKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkImportSemaphoreWin32HandleKHR.html) - Import a semaphore from a Windows HANDLE

# C Specifications
To import a semaphore payload from a Windows handle, call:
```c
// Provided by VK_KHR_external_semaphore_win32
VkResult vkImportSemaphoreWin32HandleKHR(
    VkDevice                                    device,
    const VkImportSemaphoreWin32HandleInfoKHR*  pImportSemaphoreWin32HandleInfo);
```

# Parameters
- [`device`] is the logical device that created the semaphore.
- [`p_import_semaphore_win32_handle_info`] is a pointer to a [`ImportSemaphoreWin32HandleInfoKHR`] structure specifying the semaphore and import parameters.

# Description
Importing a semaphore payload from Windows handles does not transfer
ownership of the handle to the Vulkan implementation.
For handle types defined as NT handles, the application  **must**  release
ownership using the `CloseHandle` system call when the handle is no
longer needed.Applications  **can**  import the same semaphore payload into multiple instances
of Vulkan, into the same instance from which it was exported, and multiple
times into a given Vulkan instance.
## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`p_import_semaphore_win32_handle_info`] **must**  be a valid pointer to a valid [`ImportSemaphoreWin32HandleInfoKHR`] structure

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_INVALID_EXTERNAL_HANDLE`

# Related
- [`khr_external_semaphore_win32`]
- [`Device`]
- [`ImportSemaphoreWin32HandleInfoKHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        