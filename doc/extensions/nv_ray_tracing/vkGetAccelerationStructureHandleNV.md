[vkGetAccelerationStructureHandleNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetAccelerationStructureHandleNV.html) - Get opaque acceleration structure handle

# C Specifications
To allow constructing geometry instances with device code if desired, we
need to be able to query a opaque handle for an acceleration structure.
This handle is a value of 8 bytes.
To get this handle, call:
```c
// Provided by VK_NV_ray_tracing
VkResult vkGetAccelerationStructureHandleNV(
    VkDevice                                    device,
    VkAccelerationStructureNV                   accelerationStructure,
    size_t                                      dataSize,
    void*                                       pData);
```

# Parameters
- [`device`] is the logical device that owns the acceleration structures.
- [`acceleration_structure`] is the acceleration structure.
- [`data_size`] is the size in bytes of the buffer pointed to by [`p_data`].
- [`p_data`] is a pointer to a user-allocated buffer where the results will be written.

# Description
## Valid Usage
-  [`data_size`] **must**  be large enough to contain the result of the query, as described above
-  [`acceleration_structure`] **must**  be bound completely and contiguously to a single [`DeviceMemory`] object via [`bind_acceleration_structure_memory_nv`]

## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`acceleration_structure`] **must**  be a valid [`AccelerationStructureNV`] handle
-  [`p_data`] **must**  be a valid pointer to an array of [`data_size`] bytes
-  [`data_size`] **must**  be greater than `0`
-  [`acceleration_structure`] **must**  have been created, allocated, or retrieved from [`device`]

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`

# Related
- [`VK_NV_ray_tracing`]
- [`AccelerationStructureNV`]
- [`Device`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        