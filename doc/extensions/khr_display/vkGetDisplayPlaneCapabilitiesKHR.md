[vkGetDisplayPlaneCapabilitiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDisplayPlaneCapabilitiesKHR.html) - Query capabilities of a mode and plane combination

# C Specifications
Applications that wish to present directly to a display  **must**  select which
layer, or “plane” of the display they wish to target, and a mode to use
with the display.
Each display supports at least one plane.
The capabilities of a given mode and plane combination are determined by
calling:
```c
// Provided by VK_KHR_display
VkResult vkGetDisplayPlaneCapabilitiesKHR(
    VkPhysicalDevice                            physicalDevice,
    VkDisplayModeKHR                            mode,
    uint32_t                                    planeIndex,
    VkDisplayPlaneCapabilitiesKHR*              pCapabilities);
```

# Parameters
- [`physical_device`] is the physical device associated with the display specified by [`mode`]
- [`mode`] is the display mode the application intends to program when using the specified plane. Note this parameter also implicitly specifies a display.
- [`plane_index`] is the plane which the application intends to use with the display, and is less than the number of display planes supported by the device.
- [`p_capabilities`] is a pointer to a [`DisplayPlaneCapabilitiesKHR`] structure in which the capabilities are returned.

# Description
## Valid Usage (Implicit)
-  [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
-  [`mode`] **must**  be a valid [`DisplayModeKHR`] handle
-  [`p_capabilities`] **must**  be a valid pointer to a [`DisplayPlaneCapabilitiesKHR`] structure

## Host Synchronization
- Host access to [`mode`] **must**  be externally synchronized

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`

# Related
- [`khr_display`]
- [`DisplayModeKHR`]
- [`DisplayPlaneCapabilitiesKHR`]
- [`PhysicalDevice`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        