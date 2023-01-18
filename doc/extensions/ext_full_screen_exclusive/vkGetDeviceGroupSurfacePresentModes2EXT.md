[vkGetDeviceGroupSurfacePresentModes2EXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceGroupSurfacePresentModes2EXT.html) - Query device group present capabilities for a surface

# C Specifications
Alternatively, to query the supported device group presentation modes for a
surface combined with select other fixed swapchain creation parameters,
call:
```c
// Provided by VK_VERSION_1_1 with VK_EXT_full_screen_exclusive, VK_KHR_device_group with VK_EXT_full_screen_exclusive
VkResult vkGetDeviceGroupSurfacePresentModes2EXT(
    VkDevice                                    device,
    const VkPhysicalDeviceSurfaceInfo2KHR*      pSurfaceInfo,
    VkDeviceGroupPresentModeFlagsKHR*           pModes);
```

# Parameters
- [`device`] is the logical device.
- [`p_surface_info`] is a pointer to a [`PhysicalDeviceSurfaceInfo2KHR`] structure describing the surface and other fixed parameters that would be consumed by [`create_swapchain_khr`].
- [`p_modes`] is a pointer to a [`DeviceGroupPresentModeFlagsKHR`] in which the supported device group present modes for the surface are returned.

# Description
[`get_device_group_surface_present_modes2_ext`] behaves similarly to
[`get_device_group_surface_present_modes_khr`], with the ability to specify
extended inputs via chained input structures.
## Valid Usage
-  `pSurfaceInfo->surface` **must**  be supported by all physical devices associated with [`device`], as reported by [`get_physical_device_surface_support_khr`] or an equivalent platform-specific mechanism

## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`p_surface_info`] **must**  be a valid pointer to a valid [`PhysicalDeviceSurfaceInfo2KHR`] structure
-  [`p_modes`] **must**  be a valid pointer to a [`DeviceGroupPresentModeFlagsKHR`] value

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  - `VK_ERROR_SURFACE_LOST_KHR`

# Related
- [`VK_EXT_full_screen_exclusive`]
- [`VK_KHR_device_group`]
- [`crate::vulkan1_1`]
- [`Device`]
- [`DeviceGroupPresentModeFlagsKHR`]
- [`PhysicalDeviceSurfaceInfo2KHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        