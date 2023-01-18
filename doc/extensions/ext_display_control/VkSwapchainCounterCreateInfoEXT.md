[VkSwapchainCounterCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSwapchainCounterCreateInfoEXT.html) - Specify the surface counters desired

# C Specifications
To enable surface counters when creating a swapchain, add a
[`SwapchainCounterCreateInfoEXT`] structure to the [`p_next`] chain of
[`SwapchainCreateInfoKHR`].
[`SwapchainCounterCreateInfoEXT`] is defined as:
```c
// Provided by VK_EXT_display_control
typedef struct VkSwapchainCounterCreateInfoEXT {
    VkStructureType             sType;
    const void*                 pNext;
    VkSurfaceCounterFlagsEXT    surfaceCounters;
} VkSwapchainCounterCreateInfoEXT;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`surface_counters`] is a bitmask of [`SurfaceCounterFlagBitsEXT`] specifying surface counters to enable for the swapchain.

# Description
## Valid Usage
-    The bits in [`surface_counters`] **must**  be supported by [`SwapchainCreateInfoKHR::surface`], as reported by [`get_physical_device_surface_capabilities2_ext`]

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_SWAPCHAIN_COUNTER_CREATE_INFO_EXT`
-  [`surface_counters`] **must**  be a valid combination of [`SurfaceCounterFlagBitsEXT`] values

# Related
- [`VK_EXT_display_control`]
- [`StructureType`]
- [`SurfaceCounterFlagsEXT`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        