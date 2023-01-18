[vkImportFenceWin32HandleKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkImportFenceWin32HandleKHR.html) - Import a fence from a Windows HANDLE

# C Specifications
To import a fence payload from a Windows handle, call:
```c
// Provided by VK_KHR_external_fence_win32
VkResult vkImportFenceWin32HandleKHR(
    VkDevice                                    device,
    const VkImportFenceWin32HandleInfoKHR*      pImportFenceWin32HandleInfo);
```

# Parameters
- [`device`] is the logical device that created the fence.
- [`p_import_fence_win32_handle_info`] is a pointer to a [`ImportFenceWin32HandleInfoKHR`] structure specifying the fence and import parameters.

# Description
Importing a fence payload from Windows handles does not transfer ownership
of the handle to the Vulkan implementation.
For handle types defined as NT handles, the application  **must**  release
ownership using the `CloseHandle` system call when the handle is no
longer needed.Applications  **can**  import the same fence payload into multiple instances of
Vulkan, into the same instance from which it was exported, and multiple
times into a given Vulkan instance.
## Valid Usage
-  `fence` **must**  not be associated with any queue command that has not yet completed execution on that queue

## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`p_import_fence_win32_handle_info`] **must**  be a valid pointer to a valid [`ImportFenceWin32HandleInfoKHR`] structure

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_INVALID_EXTERNAL_HANDLE`

# Related
- [`VK_KHR_external_fence_win32`]
- [`Device`]
- [`ImportFenceWin32HandleInfoKHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        