[vkGetMemoryWin32HandleKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetMemoryWin32HandleKHR.html) - Get a Windows HANDLE for a memory object

# C Specifications
To export a Windows handle representing the payload of a Vulkan device
memory object, call:
```c
// Provided by VK_KHR_external_memory_win32
VkResult vkGetMemoryWin32HandleKHR(
    VkDevice                                    device,
    const VkMemoryGetWin32HandleInfoKHR*        pGetWin32HandleInfo,
    HANDLE*                                     pHandle);
```

# Parameters
- [`device`] is the logical device that created the device memory being exported.
- [`p_get_win32_handle_info`] is a pointer to a [`MemoryGetWin32HandleInfoKHR`] structure containing parameters of the export operation.
- [`p_handle`] will return the Windows handle representing the payload of the device memory object.

# Description
For handle types defined as NT handles, the handles returned by
[`get_memory_win32_handle_khr`] are owned by the application and hold a
reference to their payload.
To avoid leaking resources, the application  **must**  release ownership of them
using the `CloseHandle` system call when they are no longer needed.
## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`p_get_win32_handle_info`] **must**  be a valid pointer to a valid [`MemoryGetWin32HandleInfoKHR`] structure
-  [`p_handle`] **must**  be a valid pointer to a `HANDLE` value

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_TOO_MANY_OBJECTS`  - `VK_ERROR_OUT_OF_HOST_MEMORY`

# Related
- [`khr_external_memory_win32`]
- [`Device`]
- [`MemoryGetWin32HandleInfoKHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        