[vkGetMemoryZirconHandleFUCHSIA](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetMemoryZirconHandleFUCHSIA.html) - Get a Zircon handle for an external memory object

# C Specifications
To export device memory as a Zircon handle that can be used by another
instance, device, or process, the handle to the [`DeviceMemory`] must be
retrieved using [`get_memory_zircon_handle_fuchsia`]:
```c
// Provided by VK_FUCHSIA_external_memory
VkResult vkGetMemoryZirconHandleFUCHSIA(
    VkDevice                                    device,
    const VkMemoryGetZirconHandleInfoFUCHSIA*   pGetZirconHandleInfo,
    zx_handle_t*                                pZirconHandle);
```

# Parameters
- [`device`] is the [`Device`].
- [`p_get_zircon_handle_info`] is a pointer to a [`MemoryGetZirconHandleInfoFUCHSIA`] structure.
- [`p_zircon_handle`] is a pointer to a `zx_handle_t` which holds the resulting Zircon handle.

# Description
## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`p_get_zircon_handle_info`] **must**  be a valid pointer to a valid [`MemoryGetZirconHandleInfoFUCHSIA`] structure
-  [`p_zircon_handle`] **must**  be a valid pointer to a `zx_handle_t` value

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_TOO_MANY_OBJECTS`  - `VK_ERROR_OUT_OF_HOST_MEMORY`

# Related
- [`fuchsia_external_memory`]
- [`Device`]
- [`MemoryGetZirconHandleInfoFUCHSIA`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        