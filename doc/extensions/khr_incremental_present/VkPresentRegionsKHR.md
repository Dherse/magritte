[VkPresentRegionsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPresentRegionsKHR.html) - Structure hint of rectangular regions changed by vkQueuePresentKHR

# C Specifications
When the [`VK_KHR_incremental_present`] extension is enabled, additional
fields  **can**  be specified that allow an application to specify that only
certain rectangular regions of the presentable images of a swapchain are
changed.
This is an optimization hint that a presentation engine  **may**  use to only
update the region of a surface that is actually changing.
The application still  **must**  ensure that all pixels of a presented image
contain the desired values, in case the presentation engine ignores this
hint.
An application  **can**  provide this hint by adding a [`PresentRegionsKHR`]
structure to the [`p_next`] chain of the [`PresentInfoKHR`] structure.The [`PresentRegionsKHR`] structure is defined as:
```c
// Provided by VK_KHR_incremental_present
typedef struct VkPresentRegionsKHR {
    VkStructureType              sType;
    const void*                  pNext;
    uint32_t                     swapchainCount;
    const VkPresentRegionKHR*    pRegions;
} VkPresentRegionsKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`swapchain_count`] is the number of swapchains being presented to by this command.
- [`regions`] is `NULL` or a pointer to an array of [`PresentRegionKHR`] elements with [`swapchain_count`] entries. If not `NULL`, each element of [`regions`] contains the region that has changed since the last present to the swapchain in the corresponding entry in the [`PresentInfoKHR::swapchains`] array.

# Description
## Valid Usage
-  [`swapchain_count`] **must**  be the same value as [`PresentInfoKHR`]::[`swapchain_count`], where [`PresentInfoKHR`] is included in the [`p_next`] chain of this [`PresentRegionsKHR`] structure

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PRESENT_REGIONS_KHR`
-    If [`regions`] is not `NULL`, [`regions`] **must**  be a valid pointer to an array of [`swapchain_count`] valid [`PresentRegionKHR`] structures
-  [`swapchain_count`] **must**  be greater than `0`

# Related
- [`VK_KHR_incremental_present`]
- [`PresentRegionKHR`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        