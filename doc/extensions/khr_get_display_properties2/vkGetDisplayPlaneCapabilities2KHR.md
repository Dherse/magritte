[vkGetDisplayPlaneCapabilities2KHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDisplayPlaneCapabilities2KHR.html) - Query capabilities of a mode and plane combination

# C Specifications
To query the capabilities of a given mode and plane combination, call:
```c
// Provided by VK_KHR_get_display_properties2
VkResult vkGetDisplayPlaneCapabilities2KHR(
    VkPhysicalDevice                            physicalDevice,
    const VkDisplayPlaneInfo2KHR*               pDisplayPlaneInfo,
    VkDisplayPlaneCapabilities2KHR*             pCapabilities);
```

# Parameters
- [`physical_device`] is the physical device associated with [`p_display_plane_info`].
- [`p_display_plane_info`] is a pointer to a [`DisplayPlaneInfo2KHR`] structure describing the plane and mode.
- [`p_capabilities`] is a pointer to a [`DisplayPlaneCapabilities2KHR`] structure in which the capabilities are returned.

# Description
[`get_display_plane_capabilities2_khr`] behaves similarly to
[`get_display_plane_capabilities_khr`], with the ability to specify extended
inputs via chained input structures, and to return extended information via
chained output structures.
## Valid Usage (Implicit)
-  [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
-  [`p_display_plane_info`] **must**  be a valid pointer to a valid [`DisplayPlaneInfo2KHR`] structure
-  [`p_capabilities`] **must**  be a valid pointer to a [`DisplayPlaneCapabilities2KHR`] structure

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`

# Related
- [`khr_get_display_properties2`]
- [`DisplayPlaneCapabilities2KHR`]
- [`DisplayPlaneInfo2KHR`]
- [`PhysicalDevice`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        