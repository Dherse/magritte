[vkCreateAccelerationStructureNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateAccelerationStructureNV.html) - Create a new acceleration structure object

# C Specifications
To create acceleration structures, call:
```c
// Provided by VK_NV_ray_tracing
VkResult vkCreateAccelerationStructureNV(
    VkDevice                                    device,
    const VkAccelerationStructureCreateInfoNV*  pCreateInfo,
    const VkAllocationCallbacks*                pAllocator,
    VkAccelerationStructureNV*                  pAccelerationStructure);
```

# Parameters
- [`device`] is the logical device that creates the buffer object.
- [`p_create_info`] is a pointer to a [`AccelerationStructureCreateInfoNV`] structure containing parameters affecting creation of the acceleration structure.
- [`p_allocator`] controls host memory allocation as described in the [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation) chapter.
- [`p_acceleration_structure`] is a pointer to a [`AccelerationStructureNV`] handle in which the resulting acceleration structure object is returned.

# Description
Similarly to other objects in Vulkan, the acceleration structure creation
merely creates an object with a specific “shape” as specified by the
information in [`AccelerationStructureInfoNV`] and `compactedSize`
in [`p_create_info`].
Populating the data in the object after allocating and binding memory is
done with [`cmd_build_acceleration_structure_nv`] and
[`cmd_copy_acceleration_structure_nv`].Acceleration structure creation uses the count and type information from the
geometries, but does not use the data references in the structures.
## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`p_create_info`] **must**  be a valid pointer to a valid [`AccelerationStructureCreateInfoNV`] structure
-    If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid [`AllocationCallbacks`] structure
-  [`p_acceleration_structure`] **must**  be a valid pointer to a [`AccelerationStructureNV`] handle

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`

# Related
- [`VK_NV_ray_tracing`]
- [`AccelerationStructureCreateInfoNV`]
- [`AccelerationStructureNV`]
- [`AllocationCallbacks`]
- [`Device`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        