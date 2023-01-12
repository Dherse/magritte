[vkCreateAccelerationStructureKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateAccelerationStructureKHR.html) - Create a new acceleration structure object

# C Specifications
To create an acceleration structure, call:
```c
// Provided by VK_KHR_acceleration_structure
VkResult vkCreateAccelerationStructureKHR(
    VkDevice                                    device,
    const VkAccelerationStructureCreateInfoKHR* pCreateInfo,
    const VkAllocationCallbacks*                pAllocator,
    VkAccelerationStructureKHR*                 pAccelerationStructure);
```

# Parameters
- [`device`] is the logical device that creates the acceleration structure object.
- [`p_create_info`] is a pointer to a [`AccelerationStructureCreateInfoKHR`] structure containing parameters affecting creation of the acceleration structure.
- [`p_allocator`] controls host memory allocation as described in the [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation) chapter.
- [`p_acceleration_structure`] is a pointer to a [`AccelerationStructureKHR`] handle in which the resulting acceleration structure object is returned.

# Description
Similar to other objects in Vulkan, the acceleration structure creation
merely creates an object with a specific “shape”.
The type and quantity of geometry that can be built into an acceleration
structure is determined by the parameters of
[`AccelerationStructureCreateInfoKHR`].Populating the data in the object after allocating and binding memory is
done with commands such as [`cmd_build_acceleration_structures_khr`],
[`build_acceleration_structures_khr`],
[`cmd_copy_acceleration_structure_khr`], and
[`copy_acceleration_structure_khr`].The input buffers passed to acceleration structure build commands will be
referenced by the implementation for the duration of the command.
After the command completes, the acceleration structure  **may**  hold a
reference to any acceleration structure specified by an active instance
contained therein.
Apart from this referencing, acceleration structures  **must**  be fully
self-contained.
The application  **may**  re-use or free any memory which was used by the command
as an input or as scratch without affecting the results of ray traversal.
## Valid Usage
-    The [`accelerationStructure`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-accelerationStructure) feature  **must**  be enabled
-    If [`AccelerationStructureCreateInfoKHR::device_address`] is not zero, the [`accelerationStructureCaptureReplay`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-accelerationStructureCaptureReplay) feature  **must**  be enabled
-    If [`device`] was created with multiple physical devices, then the [bufferDeviceAddressMultiDevice](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-bufferDeviceAddressMultiDevice) feature  **must**  be enabled

## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`p_create_info`] **must**  be a valid pointer to a valid [`AccelerationStructureCreateInfoKHR`] structure
-    If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid [`AllocationCallbacks`] structure
-  [`p_acceleration_structure`] **must**  be a valid pointer to a [`AccelerationStructureKHR`] handle

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR`

# Related
- [`khr_acceleration_structure`]
- [`AccelerationStructureCreateInfoKHR`]
- [`AccelerationStructureKHR`]
- [`AllocationCallbacks`]
- [`Device`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        