[vkGetDisplayModePropertiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDisplayModePropertiesKHR.html) - Query the set of mode properties supported by the display

# C Specifications
Each display has one or more supported modes associated with it by default.
These built-in modes are queried by calling:
```c
// Provided by VK_KHR_display
VkResult vkGetDisplayModePropertiesKHR(
    VkPhysicalDevice                            physicalDevice,
    VkDisplayKHR                                display,
    uint32_t*                                   pPropertyCount,
    VkDisplayModePropertiesKHR*                 pProperties);
```

# Parameters
- [`physical_device`] is the physical device associated with [`display`].
- [`display`] is the display to query.
- [`p_property_count`] is a pointer to an integer related to the number of display modes available or queried, as described below.
- [`p_properties`] is either `NULL` or a pointer to an array of [`DisplayModePropertiesKHR`] structures.

# Description
If [`p_properties`] is `NULL`, then the number of display modes available
on the specified [`display`] for [`physical_device`] is returned in
[`p_property_count`].
Otherwise, [`p_property_count`] **must**  point to a variable set by the user to
the number of elements in the [`p_properties`] array, and on return the
variable is overwritten with the number of structures actually written to
[`p_properties`].
If the value of [`p_property_count`] is less than the number of display
modes for [`physical_device`], at most [`p_property_count`] structures will
be written, and `VK_INCOMPLETE` will be returned instead of
`VK_SUCCESS`, to indicate that not all the available display modes were
returned.
## Valid Usage (Implicit)
-  [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
-  [`display`] **must**  be a valid [`DisplayKHR`] handle
-  [`p_property_count`] **must**  be a valid pointer to a `uint32_t` value
-    If the value referenced by [`p_property_count`] is not `0`, and [`p_properties`] is not `NULL`, [`p_properties`] **must**  be a valid pointer to an array of [`p_property_count`][`DisplayModePropertiesKHR`] structures
-  [`display`] **must**  have been created, allocated, or retrieved from [`physical_device`]

## Return Codes
*   - `VK_SUCCESS`  - `VK_INCOMPLETE` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`

# Related
- [`VK_KHR_display`]
- [`DisplayKHR`]
- [`DisplayModePropertiesKHR`]
- [`PhysicalDevice`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        