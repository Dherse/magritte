[vkGetDeviceGroupPresentCapabilitiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceGroupPresentCapabilitiesKHR.html) - Query present capabilities from other physical devices

# C Specifications
A logical device that represents multiple physical devices  **may**  support
presenting from images on more than one physical device, or combining images
from multiple physical devices.To query these capabilities, call:
```c
// Provided by VK_VERSION_1_1 with VK_KHR_swapchain, VK_KHR_device_group with VK_KHR_surface
VkResult vkGetDeviceGroupPresentCapabilitiesKHR(
    VkDevice                                    device,
    VkDeviceGroupPresentCapabilitiesKHR*        pDeviceGroupPresentCapabilities);
```

# Parameters
- [`device`] is the logical device.
- [`p_device_group_present_capabilities`] is a pointer to a [`DeviceGroupPresentCapabilitiesKHR`] structure in which the device’s capabilities are returned.

# Description
## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`p_device_group_present_capabilities`] **must**  be a valid pointer to a [`DeviceGroupPresentCapabilitiesKHR`] structure

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`

# Related
- [`khr_device_group`]
- [`khr_surface`]
- [`khr_swapchain`]
- [`crate::vulkan1_1`]
- [`Device`]
- [`DeviceGroupPresentCapabilitiesKHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        