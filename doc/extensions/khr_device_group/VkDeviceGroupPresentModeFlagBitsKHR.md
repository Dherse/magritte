[VkDeviceGroupPresentModeFlagBitsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceGroupPresentModeFlagBitsKHR.html) - Bitmask specifying supported device group present modes

# C Specifications
Bits which  **may**  be set in
[`DeviceGroupPresentCapabilitiesKHR::modes`], indicating which
device group presentation modes are supported, are:
```c
// Provided by VK_VERSION_1_1 with VK_KHR_swapchain, VK_KHR_device_group with VK_KHR_surface
typedef enum VkDeviceGroupPresentModeFlagBitsKHR {
    VK_DEVICE_GROUP_PRESENT_MODE_LOCAL_BIT_KHR = 0x00000001,
    VK_DEVICE_GROUP_PRESENT_MODE_REMOTE_BIT_KHR = 0x00000002,
    VK_DEVICE_GROUP_PRESENT_MODE_SUM_BIT_KHR = 0x00000004,
    VK_DEVICE_GROUP_PRESENT_MODE_LOCAL_MULTI_DEVICE_BIT_KHR = 0x00000008,
} VkDeviceGroupPresentModeFlagBitsKHR;
```

# Description
- [`LOCAL`] specifies that any physical device with a presentation engine  **can**  present its own swapchain images.
- [`REMOTE`] specifies that any physical device with a presentation engine  **can**  present swapchain images from any physical device in its `presentMask`.
- [`SUM`] specifies that any physical device with a presentation engine  **can**  present the sum of swapchain images from any physical devices in its `presentMask`.
- [`LOCAL_MULTI_DEVICE`] specifies that multiple physical devices with a presentation engine  **can**  each present their own swapchain images.

# Related
- [`VK_KHR_device_group`]
- [`VK_KHR_surface`]
- [`VK_KHR_swapchain`]
- [`crate::vulkan1_1`]
- [`DeviceGroupPresentInfoKHR`]
- [`DeviceGroupPresentModeFlagsKHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        