[vkGetDeviceGroupSurfacePresentModesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceGroupSurfacePresentModesKHR.html) - Query present capabilities for a surface

# C Specifications
Some surfaces  **may**  not be capable of using all the device group present
modes.To query the supported device group present modes for a particular surface,
call:
```c
// Provided by VK_VERSION_1_1 with VK_KHR_swapchain, VK_KHR_device_group with VK_KHR_surface
VkResult vkGetDeviceGroupSurfacePresentModesKHR(
    VkDevice                                    device,
    VkSurfaceKHR                                surface,
    VkDeviceGroupPresentModeFlagsKHR*           pModes);
```

# Parameters
- [`device`] is the logical device.
- [`surface`] is the surface.
- [`p_modes`] is a pointer to a [`DeviceGroupPresentModeFlagsKHR`] in which the supported device group present modes for the surface are returned.

# Description
The modes returned by this command are not invariant, and  **may**  change in
response to the surface being moved, resized, or occluded.
These modes  **must**  be a subset of the modes returned by
[`get_device_group_present_capabilities_khr`].
## Valid Usage
-  [`surface`] **must**  be supported by all physical devices associated with [`device`], as reported by [`get_physical_device_surface_support_khr`] or an equivalent platform-specific mechanism

## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`surface`] **must**  be a valid [`SurfaceKHR`] handle
-  [`p_modes`] **must**  be a valid pointer to a [`DeviceGroupPresentModeFlagsKHR`] value
-    Both of [`device`], and [`surface`] **must**  have been created, allocated, or retrieved from the same [`Instance`]

## Host Synchronization
- Host access to [`surface`] **must**  be externally synchronized

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  - `VK_ERROR_SURFACE_LOST_KHR`

# Related
- [`VK_KHR_device_group`]
- [`VK_KHR_surface`]
- [`VK_KHR_swapchain`]
- [`crate::vulkan1_1`]
- [`Device`]
- [`DeviceGroupPresentModeFlagsKHR`]
- [`SurfaceKHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        