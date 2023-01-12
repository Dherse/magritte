[vkGetPhysicalDeviceDisplayPlanePropertiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceDisplayPlanePropertiesKHR.html) - Query the plane properties

# C Specifications
Images are presented to individual planes on a display.
Devices  **must**  support at least one plane on each display.
Planes  **can**  be stacked and blended to composite multiple images on one
display.
Devices  **may**  support only a fixed stacking order and fixed mapping between
planes and displays, or they  **may**  allow arbitrary application specified
stacking orders and mappings between planes and displays.
To query the properties of device display planes, call:
```c
// Provided by VK_KHR_display
VkResult vkGetPhysicalDeviceDisplayPlanePropertiesKHR(
    VkPhysicalDevice                            physicalDevice,
    uint32_t*                                   pPropertyCount,
    VkDisplayPlanePropertiesKHR*                pProperties);
```

# Parameters
- [`physical_device`] is a physical device.
- [`p_property_count`] is a pointer to an integer related to the number of display planes available or queried, as described below.
- [`p_properties`] is either `NULL` or a pointer to an array of [`DisplayPlanePropertiesKHR`] structures.

# Description
If [`p_properties`] is `NULL`, then the number of display planes available
for [`physical_device`] is returned in [`p_property_count`].
Otherwise, [`p_property_count`] **must**  point to a variable set by the user to
the number of elements in the [`p_properties`] array, and on return the
variable is overwritten with the number of structures actually written to
[`p_properties`].
If the value of [`p_property_count`] is less than the number of display
planes for [`physical_device`], at most [`p_property_count`] structures
will be written.
## Valid Usage (Implicit)
-  [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
-  [`p_property_count`] **must**  be a valid pointer to a `uint32_t` value
-    If the value referenced by [`p_property_count`] is not `0`, and [`p_properties`] is not `NULL`, [`p_properties`] **must**  be a valid pointer to an array of [`p_property_count`][`DisplayPlanePropertiesKHR`] structures

## Return Codes
*   - `VK_SUCCESS`  - `VK_INCOMPLETE` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`

# Related
- [`khr_display`]
- [`DisplayPlanePropertiesKHR`]
- [`PhysicalDevice`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        