[vkGetSemaphoreFdKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetSemaphoreFdKHR.html) - Get a POSIX file descriptor handle for a semaphore

# C Specifications
To export a POSIX file descriptor representing the payload of a semaphore,
call:
```c
// Provided by VK_KHR_external_semaphore_fd
VkResult vkGetSemaphoreFdKHR(
    VkDevice                                    device,
    const VkSemaphoreGetFdInfoKHR*              pGetFdInfo,
    int*                                        pFd);
```

# Parameters
- [`device`] is the logical device that created the semaphore being exported.
- [`p_get_fd_info`] is a pointer to a [`SemaphoreGetFdInfoKHR`] structure containing parameters of the export operation.
- [`p_fd`] will return the file descriptor representing the semaphore payload.

# Description
Each call to [`get_semaphore_fd_khr`] **must**  create a new file descriptor
and transfer ownership of it to the application.
To avoid leaking resources, the application  **must**  release ownership of the
file descriptor when it is no longer needed.Where supported by the operating system, the implementation  **must**  set the
file descriptor to be closed automatically when an `execve` system call
is made.Exporting a file descriptor from a semaphore  **may**  have side effects
depending on the transference of the specified handle type, as described in
[Importing Semaphore State](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-semaphores-importing).
## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`p_get_fd_info`] **must**  be a valid pointer to a valid [`SemaphoreGetFdInfoKHR`] structure
-  [`p_fd`] **must**  be a valid pointer to an `int` value

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_TOO_MANY_OBJECTS`  - `VK_ERROR_OUT_OF_HOST_MEMORY`

# Related
- [`VK_KHR_external_semaphore_fd`]
- [`Device`]
- [`SemaphoreGetFdInfoKHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        