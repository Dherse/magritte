[VkPresentInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPresentInfoKHR.html) - Structure describing parameters of a queue presentation

# C Specifications
The [`PresentInfoKHR`] structure is defined as:
```c
// Provided by VK_KHR_swapchain
typedef struct VkPresentInfoKHR {
    VkStructureType          sType;
    const void*              pNext;
    uint32_t                 waitSemaphoreCount;
    const VkSemaphore*       pWaitSemaphores;
    uint32_t                 swapchainCount;
    const VkSwapchainKHR*    pSwapchains;
    const uint32_t*          pImageIndices;
    VkResult*                pResults;
} VkPresentInfoKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`wait_semaphore_count`] is the number of semaphores to wait for before issuing the present request. The number  **may**  be zero.
- [`wait_semaphores`] is `NULL` or a pointer to an array of [`Semaphore`] objects with [`wait_semaphore_count`] entries, and specifies the semaphores to wait for before issuing the present request.
- [`swapchain_count`] is the number of swapchains being presented to by this command.
- [`swapchains`] is a pointer to an array of [`SwapchainKHR`] objects with [`swapchain_count`] entries. A given swapchain  **must**  not appear in this list more than once.
- [`image_indices`] is a pointer to an array of indices into the array of each swapchain’s presentable images, with [`swapchain_count`] entries. Each entry in this array identifies the image to present on the corresponding entry in the [`swapchains`] array.
- [`results`] is a pointer to an array of [`VulkanResultCodes`] typed elements with [`swapchain_count`] entries. Applications that do not need per-swapchain results  **can**  use `NULL` for [`results`]. If non-`NULL`, each entry in [`results`] will be set to the [`VulkanResultCodes`] for presenting the swapchain corresponding to the same index in [`swapchains`].

# Description
Before an application  **can**  present an image, the image’s layout  **must**  be
transitioned to the `VK_IMAGE_LAYOUT_PRESENT_SRC_KHR`
layout, or for a shared presentable image the
`VK_IMAGE_LAYOUT_SHARED_PRESENT_KHR`
layout.
## Valid Usage
-    Each element of [`image_indices`] **must**  be the index of a presentable image acquired from the swapchain specified by the corresponding element of the [`swapchains`] array, and the presented image subresource  **must**  be in the `VK_IMAGE_LAYOUT_PRESENT_SRC_KHR` or `VK_IMAGE_LAYOUT_SHARED_PRESENT_KHR` layout at the time the operation is executed on a [`Device`]
-    If a [`PresentIdKHR`] structure is included in the [`p_next`] chain, and the [`presentId`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-presentId) feature is not enabled, each `presentIds` entry in that structure  **must**  be NULL

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PRESENT_INFO_KHR`
-    Each [`p_next`] member of any structure (including this one) in the [`p_next`] chain  **must**  be either `NULL` or a pointer to a valid instance of [`DeviceGroupPresentInfoKHR`], [`DisplayPresentInfoKHR`], [`PresentFrameTokenGGP`], [`PresentIdKHR`], [`PresentRegionsKHR`], or [`PresentTimesInfoGOOGLE`]
-    The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique
-    If [`wait_semaphore_count`] is not `0`, [`wait_semaphores`] **must**  be a valid pointer to an array of [`wait_semaphore_count`] valid [`Semaphore`] handles
-  [`swapchains`] **must**  be a valid pointer to an array of [`swapchain_count`] valid [`SwapchainKHR`] handles
-  [`image_indices`] **must**  be a valid pointer to an array of [`swapchain_count`]`uint32_t` values
-    If [`results`] is not `NULL`, [`results`] **must**  be a valid pointer to an array of [`swapchain_count`][`VulkanResultCodes`] values
-  [`swapchain_count`] **must**  be greater than `0`
-    Both of the elements of [`swapchains`], and the elements of [`wait_semaphores`] that are valid handles of non-ignored parameters  **must**  have been created, allocated, or retrieved from the same [`Instance`]

# Related
- [`khr_swapchain`]
- [`VulkanResultCodes`]
- [`Semaphore`]
- [`StructureType`]
- [`SwapchainKHR`]
- [`queue_present_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        