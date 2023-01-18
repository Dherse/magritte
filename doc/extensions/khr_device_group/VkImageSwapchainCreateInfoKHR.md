[VkImageSwapchainCreateInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageSwapchainCreateInfoKHR.html) - Specify that an image will be bound to swapchain memory

# C Specifications
If the [`p_next`] chain of [`ImageCreateInfo`] includes a
[`ImageSwapchainCreateInfoKHR`] structure, then that structure includes
a swapchain handle indicating that the image will be bound to memory from
that swapchain.The [`ImageSwapchainCreateInfoKHR`] structure is defined as:
```c
// Provided by VK_VERSION_1_1 with VK_KHR_swapchain, VK_KHR_device_group with VK_KHR_swapchain
typedef struct VkImageSwapchainCreateInfoKHR {
    VkStructureType    sType;
    const void*        pNext;
    VkSwapchainKHR     swapchain;
} VkImageSwapchainCreateInfoKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`swapchain`] is [`crate::Handle::null`] or a handle of a swapchain that the image will be bound to.

# Description
## Valid Usage
-    If [`swapchain`] is not [`crate::Handle::null`], the fields of [`ImageCreateInfo`] **must**  match the [implied image creation parameters](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#swapchain-wsi-image-create-info) of the swapchain

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_IMAGE_SWAPCHAIN_CREATE_INFO_KHR`
-    If [`swapchain`] is not [`crate::Handle::null`], [`swapchain`] **must**  be a valid [`SwapchainKHR`] handle

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
        