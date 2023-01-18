[vkBindAccelerationStructureMemoryNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkBindAccelerationStructureMemoryNV.html) - Bind acceleration structure memory

# C Specifications
To attach memory to one or more acceleration structures at a time, call:
```c
// Provided by VK_NV_ray_tracing
VkResult vkBindAccelerationStructureMemoryNV(
    VkDevice                                    device,
    uint32_t                                    bindInfoCount,
    const VkBindAccelerationStructureMemoryInfoNV* pBindInfos);
```

# Parameters
- [`device`] is the logical device that owns the acceleration structures and memory.
- [`bind_info_count`] is the number of elements in [`p_bind_infos`].
- [`p_bind_infos`] is a pointer to an array of [`BindAccelerationStructureMemoryInfoNV`] structures describing acceleration structures and memory to bind.

# Description
## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`p_bind_infos`] **must**  be a valid pointer to an array of [`bind_info_count`] valid [`BindAccelerationStructureMemoryInfoNV`] structures
-  [`bind_info_count`] **must**  be greater than `0`

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`

# Related
- [`VK_NV_ray_tracing`]
- [`BindAccelerationStructureMemoryInfoNV`]
- [`Device`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        