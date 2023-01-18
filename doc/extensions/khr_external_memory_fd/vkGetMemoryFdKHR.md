[vkGetMemoryFdKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetMemoryFdKHR.html) - Get a POSIX file descriptor for a memory object

# C Specifications
To export a POSIX file descriptor referencing the payload of a Vulkan device
memory object, call:
```c
// Provided by VK_KHR_external_memory_fd
VkResult vkGetMemoryFdKHR(
    VkDevice                                    device,
    const VkMemoryGetFdInfoKHR*                 pGetFdInfo,
    int*                                        pFd);
```

# Parameters
- [`device`] is the logical device that created the device memory being exported.
- [`p_get_fd_info`] is a pointer to a [`MemoryGetFdInfoKHR`] structure containing parameters of the export operation.
- [`p_fd`] will return a file descriptor referencing the payload of the device memory object.

# Description
Each call to [`get_memory_fd_khr`] **must**  create a new file descriptor
holding a reference to the memory object’s payload and transfer ownership of
the file descriptor to the application.
To avoid leaking resources, the application  **must**  release ownership of the
file descriptor using the `close` system call when it is no longer
needed, or by importing a Vulkan memory object from it.
Where supported by the operating system, the implementation  **must**  set the
file descriptor to be closed automatically when an `execve` system call
is made.
## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`p_get_fd_info`] **must**  be a valid pointer to a valid [`MemoryGetFdInfoKHR`] structure
-  [`p_fd`] **must**  be a valid pointer to an `int` value

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_TOO_MANY_OBJECTS`  - `VK_ERROR_OUT_OF_HOST_MEMORY`

# Related
- [`VK_KHR_external_memory_fd`]
- [`Device`]
- [`MemoryGetFdInfoKHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        