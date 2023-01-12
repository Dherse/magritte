[vkBindBufferMemory2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkBindBufferMemory2.html) - Bind device memory to buffer objects

# C Specifications
To attach memory to buffer objects for one or more buffers at a time, call:
```c
// Provided by VK_VERSION_1_1
VkResult vkBindBufferMemory2(
    VkDevice                                    device,
    uint32_t                                    bindInfoCount,
    const VkBindBufferMemoryInfo*               pBindInfos);
```
or the equivalent command
```c
// Provided by VK_KHR_bind_memory2
VkResult vkBindBufferMemory2KHR(
    VkDevice                                    device,
    uint32_t                                    bindInfoCount,
    const VkBindBufferMemoryInfo*               pBindInfos);
```

# Parameters
- [`device`] is the logical device that owns the buffers and memory.
- [`bind_info_count`] is the number of elements in [`p_bind_infos`].
- [`p_bind_infos`] is a pointer to an array of [`bind_info_count`][`BindBufferMemoryInfo`] structures describing buffers and memory to bind.

# Description
On some implementations, it  **may**  be more efficient to batch memory bindings
into a single command.
## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`p_bind_infos`] **must**  be a valid pointer to an array of [`bind_info_count`] valid [`BindBufferMemoryInfo`] structures
-  [`bind_info_count`] **must**  be greater than `0`

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  - `VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR`

# Related
- [`crate::vulkan1_1`]
- [`BindBufferMemoryInfo`]
- [`Device`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        