[vkGetMemoryWin32HandlePropertiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetMemoryWin32HandlePropertiesKHR.html) - Get Properties of External Memory Win32 Handles

# C Specifications
Windows memory handles compatible with Vulkan  **may**  also be created by
non-Vulkan APIs using methods beyond the scope of this specification.
To determine the correct parameters to use when importing such handles,
call:
```c
// Provided by VK_KHR_external_memory_win32
VkResult vkGetMemoryWin32HandlePropertiesKHR(
    VkDevice                                    device,
    VkExternalMemoryHandleTypeFlagBits          handleType,
    HANDLE                                      handle,
    VkMemoryWin32HandlePropertiesKHR*           pMemoryWin32HandleProperties);
```

# Parameters
- [`device`] is the logical device that will be importing [`handle`].
- [`handle_type`] is a [`ExternalMemoryHandleTypeFlagBits`] value specifying the type of the handle [`handle`].
- [`handle`] is the handle which will be imported.
- [`p_memory_win32_handle_properties`] is a pointer to a [`MemoryWin32HandlePropertiesKHR`] structure in which properties of [`handle`] are returned.

# Description
## Valid Usage
-  [`handle`] **must**  be an external memory handle created outside of the Vulkan API
-  [`handle_type`] **must**  not be one of the handle types defined as opaque

## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`handle_type`] **must**  be a valid [`ExternalMemoryHandleTypeFlagBits`] value
-  [`p_memory_win32_handle_properties`] **must**  be a valid pointer to a [`MemoryWin32HandlePropertiesKHR`] structure

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_INVALID_EXTERNAL_HANDLE`

# Related
- [`khr_external_memory_win32`]
- [`Device`]
- [`ExternalMemoryHandleTypeFlagBits`]
- [`MemoryWin32HandlePropertiesKHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        