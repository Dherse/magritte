[vkGetDeviceImageSparseMemoryRequirements](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceImageSparseMemoryRequirements.html) - Query the memory requirements for a sparse image

# C Specifications
To determine the sparse memory requirements for an image resource without
creating an object, call:
```c
// Provided by VK_VERSION_1_3
void vkGetDeviceImageSparseMemoryRequirements(
    VkDevice                                    device,
    const VkDeviceImageMemoryRequirements*      pInfo,
    uint32_t*                                   pSparseMemoryRequirementCount,
    VkSparseImageMemoryRequirements2*           pSparseMemoryRequirements);
```
or the equivalent command
```c
// Provided by VK_KHR_maintenance4
void vkGetDeviceImageSparseMemoryRequirementsKHR(
    VkDevice                                    device,
    const VkDeviceImageMemoryRequirements*      pInfo,
    uint32_t*                                   pSparseMemoryRequirementCount,
    VkSparseImageMemoryRequirements2*           pSparseMemoryRequirements);
```

# Parameters
- [`device`] is the logical device intended to own the image.
- [`p_info`] is a pointer to a [`DeviceImageMemoryRequirements`] structure containing parameters required for the memory requirements query.
- [`p_sparse_memory_requirement_count`] is a pointer to an integer related to the number of sparse memory requirements available or queried, as described below.
- [`p_sparse_memory_requirements`] is either `NULL` or a pointer to an array of [`SparseImageMemoryRequirements2`] structures.

# Description
## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`p_info`] **must**  be a valid pointer to a valid [`DeviceImageMemoryRequirements`] structure
-  [`p_sparse_memory_requirement_count`] **must**  be a valid pointer to a `uint32_t` value
-    If the value referenced by [`p_sparse_memory_requirement_count`] is not `0`, and [`p_sparse_memory_requirements`] is not `NULL`, [`p_sparse_memory_requirements`] **must**  be a valid pointer to an array of [`p_sparse_memory_requirement_count`][`SparseImageMemoryRequirements2`] structures

# Related
- [`VK_KHR_maintenance4`]
- [`crate::vulkan1_3`]
- [`Device`]
- [`DeviceImageMemoryRequirements`]
- [`SparseImageMemoryRequirements2`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        