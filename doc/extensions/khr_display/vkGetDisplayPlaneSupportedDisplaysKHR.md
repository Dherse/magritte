[vkGetDisplayPlaneSupportedDisplaysKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDisplayPlaneSupportedDisplaysKHR.html) - Query the list of displays a plane supports

# C Specifications
To determine which displays a plane is usable with, call
```c
// Provided by VK_KHR_display
VkResult vkGetDisplayPlaneSupportedDisplaysKHR(
    VkPhysicalDevice                            physicalDevice,
    uint32_t                                    planeIndex,
    uint32_t*                                   pDisplayCount,
    VkDisplayKHR*                               pDisplays);
```

# Parameters
- [`physical_device`] is a physical device.
- [`plane_index`] is the plane which the application wishes to use, and  **must**  be in the range [0, physical device plane count - 1].
- [`p_display_count`] is a pointer to an integer related to the number of displays available or queried, as described below.
- [`p_displays`] is either `NULL` or a pointer to an array of [`DisplayKHR`] handles.

# Description
If [`p_displays`] is `NULL`, then the number of displays usable with the
specified [`plane_index`] for [`physical_device`] is returned in
[`p_display_count`].
Otherwise, [`p_display_count`] **must**  point to a variable set by the user to
the number of elements in the [`p_displays`] array, and on return the
variable is overwritten with the number of handles actually written to
[`p_displays`].
If the value of [`p_display_count`] is less than the number of usable
display-plane pairs for [`physical_device`], at most [`p_display_count`]
handles will be written, and `VK_INCOMPLETE` will be returned instead of
`VK_SUCCESS`, to indicate that not all the available pairs were
returned.
## Valid Usage
-  [`plane_index`] **must**  be less than the number of display planes supported by the device as determined by calling [`get_physical_device_display_plane_properties_khr`]

## Valid Usage (Implicit)
-  [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
-  [`p_display_count`] **must**  be a valid pointer to a `uint32_t` value
-    If the value referenced by [`p_display_count`] is not `0`, and [`p_displays`] is not `NULL`, [`p_displays`] **must**  be a valid pointer to an array of [`p_display_count`][`DisplayKHR`] handles

## Return Codes
*   - `VK_SUCCESS`  - `VK_INCOMPLETE` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`

# Related
- [`khr_display`]
- [`DisplayKHR`]
- [`PhysicalDevice`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        