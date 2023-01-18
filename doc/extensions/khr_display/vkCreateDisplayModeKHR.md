[vkCreateDisplayModeKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateDisplayModeKHR.html) - Create a display mode

# C Specifications
Additional modes  **may**  also be created by calling:
```c
// Provided by VK_KHR_display
VkResult vkCreateDisplayModeKHR(
    VkPhysicalDevice                            physicalDevice,
    VkDisplayKHR                                display,
    const VkDisplayModeCreateInfoKHR*           pCreateInfo,
    const VkAllocationCallbacks*                pAllocator,
    VkDisplayModeKHR*                           pMode);
```

# Parameters
- [`physical_device`] is the physical device associated with [`display`].
- [`display`] is the display to create an additional mode for.
- [`p_create_info`] is a pointer to a [`DisplayModeCreateInfoKHR`] structure describing the new mode to create.
- [`p_allocator`] is the allocator used for host memory allocated for the display mode object when there is no more specific allocator available (see [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation)).
- [`p_mode`] is a pointer to a [`DisplayModeKHR`] handle in which the mode created is returned.

# Description
## Valid Usage (Implicit)
-  [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
-  [`display`] **must**  be a valid [`DisplayKHR`] handle
-  [`p_create_info`] **must**  be a valid pointer to a valid [`DisplayModeCreateInfoKHR`] structure
-    If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid [`AllocationCallbacks`] structure
-  [`p_mode`] **must**  be a valid pointer to a [`DisplayModeKHR`] handle
-  [`display`] **must**  have been created, allocated, or retrieved from [`physical_device`]

## Host Synchronization
- Host access to [`display`] **must**  be externally synchronized

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  - `VK_ERROR_INITIALIZATION_FAILED`

# Related
- [`VK_KHR_display`]
- [`AllocationCallbacks`]
- [`DisplayKHR`]
- [`DisplayModeCreateInfoKHR`]
- [`DisplayModeKHR`]
- [`PhysicalDevice`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        