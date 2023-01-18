[vkDestroyAccelerationStructureKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyAccelerationStructureKHR.html) - Destroy an acceleration structure object

# C Specifications
To destroy an acceleration structure, call:
```c
// Provided by VK_KHR_acceleration_structure
void vkDestroyAccelerationStructureKHR(
    VkDevice                                    device,
    VkAccelerationStructureKHR                  accelerationStructure,
    const VkAllocationCallbacks*                pAllocator);
```

# Parameters
- [`device`] is the logical device that destroys the acceleration structure.
- [`acceleration_structure`] is the acceleration structure to destroy.
- [`p_allocator`] controls host memory allocation as described in the [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation) chapter.

# Description
## Valid Usage
-    All submitted commands that refer to [`acceleration_structure`] **must**  have completed execution
-    If [`AllocationCallbacks`] were provided when [`acceleration_structure`] was created, a compatible set of callbacks  **must**  be provided here
-    If no [`AllocationCallbacks`] were provided when [`acceleration_structure`] was created, [`p_allocator`] **must**  be `NULL`

## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-    If [`acceleration_structure`] is not [`crate::Handle::null`], [`acceleration_structure`] **must**  be a valid [`AccelerationStructureKHR`] handle
-    If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid [`AllocationCallbacks`] structure
-    If [`acceleration_structure`] is a valid handle, it  **must**  have been created, allocated, or retrieved from [`device`]

## Host Synchronization
- Host access to [`acceleration_structure`] **must**  be externally synchronized

# Related
- [`VK_KHR_acceleration_structure`]
- [`AccelerationStructureKHR`]
- [`AllocationCallbacks`]
- [`Device`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        