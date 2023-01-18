[VkDeviceGroupSwapchainCreateInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceGroupSwapchainCreateInfoKHR.html) - Structure specifying parameters of a newly created swapchain object

# C Specifications
If the [`p_next`] chain of [`SwapchainCreateInfoKHR`] includes a
[`DeviceGroupSwapchainCreateInfoKHR`] structure, then that structure
includes a set of device group present modes that the swapchain  **can**  be used
with.The [`DeviceGroupSwapchainCreateInfoKHR`] structure is defined as:
```c
// Provided by VK_VERSION_1_1 with VK_KHR_swapchain, VK_KHR_device_group with VK_KHR_swapchain
typedef struct VkDeviceGroupSwapchainCreateInfoKHR {
    VkStructureType                     sType;
    const void*                         pNext;
    VkDeviceGroupPresentModeFlagsKHR    modes;
} VkDeviceGroupSwapchainCreateInfoKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`modes`] is a bitfield of modes that the swapchain  **can**  be used with.

# Description
If this structure is not present, [`modes`] is considered to be
`VK_DEVICE_GROUP_PRESENT_MODE_LOCAL_BIT_KHR`.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DEVICE_GROUP_SWAPCHAIN_CREATE_INFO_KHR`
-  [`modes`] **must**  be a valid combination of [`DeviceGroupPresentModeFlagBitsKHR`] values
-  [`modes`] **must**  not be `0`

# Related
- [`VK_KHR_device_group`]
- [`VK_KHR_swapchain`]
- [`crate::vulkan1_1`]
- [`DeviceGroupPresentModeFlagsKHR`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        