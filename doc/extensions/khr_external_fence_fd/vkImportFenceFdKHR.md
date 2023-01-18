[vkImportFenceFdKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkImportFenceFdKHR.html) - Import a fence from a POSIX file descriptor

# C Specifications
To import a fence payload from a POSIX file descriptor, call:
```c
// Provided by VK_KHR_external_fence_fd
VkResult vkImportFenceFdKHR(
    VkDevice                                    device,
    const VkImportFenceFdInfoKHR*               pImportFenceFdInfo);
```

# Parameters
- [`device`] is the logical device that created the fence.
- [`p_import_fence_fd_info`] is a pointer to a [`ImportFenceFdInfoKHR`] structure specifying the fence and import parameters.

# Description
Importing a fence payload from a file descriptor transfers ownership of the
file descriptor from the application to the Vulkan implementation.
The application  **must**  not perform any operations on the file descriptor
after a successful import.Applications  **can**  import the same fence payload into multiple instances of
Vulkan, into the same instance from which it was exported, and multiple
times into a given Vulkan instance.
## Valid Usage
-  `fence` **must**  not be associated with any queue command that has not yet completed execution on that queue

## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`p_import_fence_fd_info`] **must**  be a valid pointer to a valid [`ImportFenceFdInfoKHR`] structure

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_INVALID_EXTERNAL_HANDLE`

# Related
- [`VK_KHR_external_fence_fd`]
- [`Device`]
- [`ImportFenceFdInfoKHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        