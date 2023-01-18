[VkPresentTimesInfoGOOGLE](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPresentTimesInfoGOOGLE.html) - The earliest time each image should be presented

# C Specifications
When the `[`VK_GOOGLE_display_timing`]` extension is enabled, additional
fields  **can**  be specified that allow an application to specify the earliest
time that an image should be displayed.
This allows an application to avoid stutter that is caused by an image being
displayed earlier than planned.
Such stuttering can occur with both fixed and variable-refresh-rate
displays, because stuttering occurs when the geometry is not correctly
positioned for when the image is displayed.
An application  **can**  instruct the presentation engine that an image should
not be displayed earlier than a specified time by adding a
[`PresentTimesInfoGOOGLE`] structure to the [`p_next`] chain of the
[`PresentInfoKHR`] structure.The [`PresentTimesInfoGOOGLE`] structure is defined as:
```c
// Provided by VK_GOOGLE_display_timing
typedef struct VkPresentTimesInfoGOOGLE {
    VkStructureType               sType;
    const void*                   pNext;
    uint32_t                      swapchainCount;
    const VkPresentTimeGOOGLE*    pTimes;
} VkPresentTimesInfoGOOGLE;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`swapchain_count`] is the number of swapchains being presented to by this command.
- [`times`] is `NULL` or a pointer to an array of [`PresentTimeGOOGLE`] elements with [`swapchain_count`] entries. If not `NULL`, each element of [`times`] contains the earliest time to present the image corresponding to the entry in the [`PresentInfoKHR::image_indices`] array.

# Description
## Valid Usage
-  [`swapchain_count`] **must**  be the same value as [`PresentInfoKHR`]::[`swapchain_count`], where [`PresentInfoKHR`] is included in the [`p_next`] chain of this [`PresentTimesInfoGOOGLE`] structure

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PRESENT_TIMES_INFO_GOOGLE`
-    If [`times`] is not `NULL`, [`times`] **must**  be a valid pointer to an array of [`swapchain_count`][`PresentTimeGOOGLE`] structures
-  [`swapchain_count`] **must**  be greater than `0`

# Related
- [`VK_GOOGLE_display_timing`]
- [`PresentTimeGOOGLE`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        