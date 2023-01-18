[vkGetMemoryFdPropertiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetMemoryFdPropertiesKHR.html) - Get Properties of External Memory File Descriptors

# C Specifications
POSIX file descriptor memory handles compatible with Vulkan  **may**  also be
created by non-Vulkan APIs using methods beyond the scope of this
specification.
To determine the correct parameters to use when importing such handles,
call:
```c
// Provided by VK_KHR_external_memory_fd
VkResult vkGetMemoryFdPropertiesKHR(
    VkDevice                                    device,
    VkExternalMemoryHandleTypeFlagBits          handleType,
    int                                         fd,
    VkMemoryFdPropertiesKHR*                    pMemoryFdProperties);
```

# Parameters
- [`device`] is the logical device that will be importing [`fd`].
- [`handle_type`] is a [`ExternalMemoryHandleTypeFlagBits`] value specifying the type of the handle [`fd`].
- [`fd`] is the handle which will be imported.
- [`p_memory_fd_properties`] is a pointer to a [`MemoryFdPropertiesKHR`] structure in which the properties of the handle [`fd`] are returned.

# Description
## Valid Usage
-  [`fd`] **must**  be an external memory handle created outside of the Vulkan API
-  [`handle_type`] **must**  not be `VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_FD_BIT`

## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`handle_type`] **must**  be a valid [`ExternalMemoryHandleTypeFlagBits`] value
-  [`p_memory_fd_properties`] **must**  be a valid pointer to a [`MemoryFdPropertiesKHR`] structure

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_INVALID_EXTERNAL_HANDLE`

# Related
- [`VK_KHR_external_memory_fd`]
- [`Device`]
- [`ExternalMemoryHandleTypeFlagBits`]
- [`MemoryFdPropertiesKHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        