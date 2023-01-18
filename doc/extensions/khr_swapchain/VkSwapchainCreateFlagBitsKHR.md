[VkSwapchainCreateFlagBitsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSwapchainCreateFlagBitsKHR.html) - Bitmask controlling swapchain creation

# C Specifications
Bits which  **can**  be set in [`SwapchainCreateInfoKHR::flags`],
specifying parameters of swapchain creation, are:
```c
// Provided by VK_KHR_swapchain
typedef enum VkSwapchainCreateFlagBitsKHR {
  // Provided by VK_VERSION_1_1 with VK_KHR_swapchain, VK_KHR_device_group with VK_KHR_swapchain
    VK_SWAPCHAIN_CREATE_SPLIT_INSTANCE_BIND_REGIONS_BIT_KHR = 0x00000001,
  // Provided by VK_VERSION_1_1 with VK_KHR_swapchain
    VK_SWAPCHAIN_CREATE_PROTECTED_BIT_KHR = 0x00000002,
  // Provided by VK_KHR_swapchain_mutable_format
    VK_SWAPCHAIN_CREATE_MUTABLE_FORMAT_BIT_KHR = 0x00000004,
} VkSwapchainCreateFlagBitsKHR;
```

# Description
- [`SPLIT_INSTANCE_BIND_REGIONS`] specifies that images created from the swapchain (i.e. with the `swapchain` member of [`ImageSwapchainCreateInfoKHR`] set to this swapchain’s handle)  **must**  use `VK_IMAGE_CREATE_SPLIT_INSTANCE_BIND_REGIONS_BIT`.
- [`PROTECTED`] specifies that images created from the swapchain are protected images.
- [`MUTABLE_FORMAT`] specifies that the images of the swapchain  **can**  be used to create a [`ImageView`] with a different format than what the swapchain was created with. The list of allowed image view formats is specified by adding a [`ImageFormatListCreateInfo`] structure to the `pNext` chain of [`SwapchainCreateInfoKHR`]. In addition, this flag also specifies that the swapchain  **can**  be created with usage flags that are not supported for the format the swapchain is created with but are supported for at least one of the allowed image view formats.

# Related
- [`VK_KHR_swapchain`]
- [`SwapchainCreateFlagsKHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        