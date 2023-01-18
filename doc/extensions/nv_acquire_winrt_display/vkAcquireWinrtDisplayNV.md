[vkAcquireWinrtDisplayNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAcquireWinrtDisplayNV.html) - Acquire access to a VkDisplayKHR

# C Specifications
To acquire permission to directly access a display in Vulkan on Windows 10,
call:
```c
// Provided by VK_NV_acquire_winrt_display
VkResult vkAcquireWinrtDisplayNV(
    VkPhysicalDevice                            physicalDevice,
    VkDisplayKHR                                display);
```

# Parameters
- [`physical_device`] The physical device the display is on.
- [`display`] The display the caller wishes to control in Vulkan.

# Description
All permissions necessary to control the display are granted to the Vulkan
instance associated with [`physical_device`] until the display is released
or the application is terminated.
Permission to access the display  **may**  be revoked by events that cause
Windows 10 itself to lose access to [`display`].
If this has happened, operations which require access to the display  **must** 
fail with an appropriate error code.
If permission to access [`display`] has already been acquired by another
entity, the call  **must**  return the error code
`VK_ERROR_INITIALIZATION_FAILED`.
## Valid Usage (Implicit)
-  [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
-  [`display`] **must**  be a valid [`DisplayKHR`] handle
-  [`display`] **must**  have been created, allocated, or retrieved from [`physical_device`]

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_DEVICE_LOST`  - `VK_ERROR_INITIALIZATION_FAILED`

# Related
- [`VK_NV_acquire_winrt_display`]
- [`DisplayKHR`]
- [`PhysicalDevice`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        