[vkBindImageMemory2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkBindImageMemory2.html) - Bind device memory to image objects

# C Specifications
To attach memory to image objects for one or more images at a time, call:
```c
// Provided by VK_VERSION_1_1
VkResult vkBindImageMemory2(
    VkDevice                                    device,
    uint32_t                                    bindInfoCount,
    const VkBindImageMemoryInfo*                pBindInfos);
```
or the equivalent command
```c
// Provided by VK_KHR_bind_memory2
VkResult vkBindImageMemory2KHR(
    VkDevice                                    device,
    uint32_t                                    bindInfoCount,
    const VkBindImageMemoryInfo*                pBindInfos);
```

# Parameters
- [`device`] is the logical device that owns the images and memory.
- [`bind_info_count`] is the number of elements in [`p_bind_infos`].
- [`p_bind_infos`] is a pointer to an array of [`BindImageMemoryInfo`] structures, describing images and memory to bind.

# Description
On some implementations, it  **may**  be more efficient to batch memory bindings
into a single command.
## Valid Usage
-    If any [`BindImageMemoryInfo::image`] was created with `VK_IMAGE_CREATE_DISJOINT_BIT` then all planes of [`BindImageMemoryInfo::image`] **must**  be bound individually in separate [`p_bind_infos`]
-  [`p_bind_infos`] **must**  not refer to the same image subresource more than once

## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`p_bind_infos`] **must**  be a valid pointer to an array of [`bind_info_count`] valid [`BindImageMemoryInfo`] structures
-  [`bind_info_count`] **must**  be greater than `0`

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`

# Related
- [`crate::vulkan1_1`]
- [`BindImageMemoryInfo`]
- [`Device`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        