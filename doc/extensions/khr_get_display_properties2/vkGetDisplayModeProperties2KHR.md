[vkGetDisplayModeProperties2KHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDisplayModeProperties2KHR.html) - Query information about the available display modes.

# C Specifications
To query the properties of a device’s built-in display modes, call:
```c
// Provided by VK_KHR_get_display_properties2
VkResult vkGetDisplayModeProperties2KHR(
    VkPhysicalDevice                            physicalDevice,
    VkDisplayKHR                                display,
    uint32_t*                                   pPropertyCount,
    VkDisplayModeProperties2KHR*                pProperties);
```

# Parameters
- [`physical_device`] is the physical device associated with [`display`].
- [`display`] is the display to query.
- [`p_property_count`] is a pointer to an integer related to the number of display modes available or queried, as described below.
- [`p_properties`] is either `NULL` or a pointer to an array of [`DisplayModeProperties2KHR`] structures.

# Description
[`get_display_mode_properties2_khr`] behaves similarly to
[`get_display_mode_properties_khr`], with the ability to return extended
information via chained output structures.
## Valid Usage (Implicit)
-  [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
-  [`display`] **must**  be a valid [`DisplayKHR`] handle
-  [`p_property_count`] **must**  be a valid pointer to a `uint32_t` value
-    If the value referenced by [`p_property_count`] is not `0`, and [`p_properties`] is not `NULL`, [`p_properties`] **must**  be a valid pointer to an array of [`p_property_count`][`DisplayModeProperties2KHR`] structures
-  [`display`] **must**  have been created, allocated, or retrieved from [`physical_device`]

## Return Codes
*   - `VK_SUCCESS`  - `VK_INCOMPLETE` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`

# Related
- [`khr_get_display_properties2`]
- [`DisplayKHR`]
- [`DisplayModeProperties2KHR`]
- [`PhysicalDevice`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        