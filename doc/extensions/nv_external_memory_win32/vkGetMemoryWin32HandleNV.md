[vkGetMemoryWin32HandleNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetMemoryWin32HandleNV.html) - Retrieve Win32 handle to a device memory object

# C Specifications
To retrieve the handle corresponding to a device memory object created with
[`ExportMemoryAllocateInfoNV::handle_types`] set to include
`VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT_NV` or
`VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_NV`, call:
```c
// Provided by VK_NV_external_memory_win32
VkResult vkGetMemoryWin32HandleNV(
    VkDevice                                    device,
    VkDeviceMemory                              memory,
    VkExternalMemoryHandleTypeFlagsNV           handleType,
    HANDLE*                                     pHandle);
```

# Parameters
- [`device`] is the logical device that owns the memory.
- [`memory`] is the [`DeviceMemory`] object.
- [`handle_type`] is a bitmask of [`ExternalMemoryHandleTypeFlagBitsNV`] containing a single bit specifying the type of handle requested.
- `handle` is a pointer to a Windows `HANDLE` in which the handle is returned.

# Description
## Valid Usage
-  [`handle_type`] **must**  be a flag specified in [`ExportMemoryAllocateInfoNV::handle_types`] when allocating [`memory`]

## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`memory`] **must**  be a valid [`DeviceMemory`] handle
-  [`handle_type`] **must**  be a valid combination of [`ExternalMemoryHandleTypeFlagBitsNV`] values
-  [`handle_type`] **must**  not be `0`
-  [`p_handle`] **must**  be a valid pointer to a `HANDLE` value
-  [`memory`] **must**  have been created, allocated, or retrieved from [`device`]

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_TOO_MANY_OBJECTS`  - `VK_ERROR_OUT_OF_HOST_MEMORY`

# Related
- [`nv_external_memory_win32`]
- [`Device`]
- [`DeviceMemory`]
- [VkExternalMemoryHandleTypeFlagsNV]()

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        