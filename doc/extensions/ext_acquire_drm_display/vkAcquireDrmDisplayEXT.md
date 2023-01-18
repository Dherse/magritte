[vkAcquireDrmDisplayEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAcquireDrmDisplayEXT.html) - Acquire access to a VkDisplayKHR using DRM

# C Specifications
To acquire permission to directly a display in Vulkan from the Direct
Rendering Manager (DRM) interface, call:
```c
// Provided by VK_EXT_acquire_drm_display
VkResult vkAcquireDrmDisplayEXT(
    VkPhysicalDevice                            physicalDevice,
    int32_t                                     drmFd,
    VkDisplayKHR                                display);
```

# Parameters
- [`physical_device`] The physical device the display is on.
- [`drm_fd`] DRM primary file descriptor.
- [`display`] The display the caller wishes Vulkan to control.

# Description
All permissions necessary to control the display are granted to the Vulkan
instance associated with the provided [`physical_device`] until the display
is either released or the connector is unplugged.
The provided [`drm_fd`] must correspond to the one owned by the
[`physical_device`].
If not, the error code `VK_ERROR_UNKNOWN` must be returned.
The DRM FD must have DRM master permissions.
If any error is encountered during the acquisition of the display, the call
must return the error code `VK_ERROR_INITIALIZATION_FAILED`.The provided DRM fd should not be closed before the display is released,
attempting to do it may result in undefined behaviour.
## Valid Usage (Implicit)
-  [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
-  [`display`] **must**  be a valid [`DisplayKHR`] handle
-  [`display`] **must**  have been created, allocated, or retrieved from [`physical_device`]

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_INITIALIZATION_FAILED`

# Related
- [`VK_EXT_acquire_drm_display`]
- [`DisplayKHR`]
- [`PhysicalDevice`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        