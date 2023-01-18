[VkBindImageMemorySwapchainInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBindImageMemorySwapchainInfoKHR.html) - Structure specifying swapchain image memory to bind to

# C Specifications
If the [`p_next`] chain of [`BindImageMemoryInfo`] includes a
[`BindImageMemorySwapchainInfoKHR`] structure, then that structure
includes a swapchain handle and image index indicating that the image will
be bound to memory from that swapchain.The [`BindImageMemorySwapchainInfoKHR`] structure is defined as:
```c
// Provided by VK_VERSION_1_1 with VK_KHR_swapchain, VK_KHR_device_group with VK_KHR_swapchain
typedef struct VkBindImageMemorySwapchainInfoKHR {
    VkStructureType    sType;
    const void*        pNext;
    VkSwapchainKHR     swapchain;
    uint32_t           imageIndex;
} VkBindImageMemorySwapchainInfoKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`swapchain`] is [`crate::Handle::null`] or a swapchain handle.
- [`image_index`] is an image index within [`swapchain`].

# Description
If [`swapchain`] is not `NULL`, the [`swapchain`] and [`image_index`]
are used to determine the memory that the image is bound to, instead of
`memory` and `memoryOffset`.Memory  **can**  be bound to a swapchain and use the `pDeviceIndices` or
`pSplitInstanceBindRegions` members of
[`BindImageMemoryDeviceGroupInfo`].
## Valid Usage
-  [`image_index`] **must**  be less than the number of images in [`swapchain`]

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_SWAPCHAIN_INFO_KHR`
-  [`swapchain`] **must**  be a valid [`SwapchainKHR`] handle

## Host Synchronization
- Host access to [`swapchain`] **must**  be externally synchronized

# Related
- [`VK_KHR_device_group`]
- [`VK_KHR_swapchain`]
- [`crate::vulkan1_1`]
- [`StructureType`]
- [`SwapchainKHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        