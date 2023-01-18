[vkAcquireXlibDisplayEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAcquireXlibDisplayEXT.html) - Acquire access to a VkDisplayKHR using Xlib

# C Specifications
To acquire permission to directly access a display in Vulkan from an X11
server, call:
```c
// Provided by VK_EXT_acquire_xlib_display
VkResult vkAcquireXlibDisplayEXT(
    VkPhysicalDevice                            physicalDevice,
    Display*                                    dpy,
    VkDisplayKHR                                display);
```

# Parameters
- [`physical_device`] The physical device the display is on.
- [`dpy`] A connection to the X11 server that currently owns [`display`].
- [`display`] The display the caller wishes to control in Vulkan.

# Description
All permissions necessary to control the display are granted to the Vulkan
instance associated with [`physical_device`] until the display is released
or the X11 connection specified by [`dpy`] is terminated.
Permission to access the display  **may**  be temporarily revoked during periods
when the X11 server from which control was acquired itself loses access to
[`display`].
During such periods, operations which require access to the display  **must** 
fail with an approriate error code.
If the X11 server associated with [`dpy`] does not own [`display`], or
if permission to access it has already been acquired by another entity, the
call  **must**  return the error code `VK_ERROR_INITIALIZATION_FAILED`.
## Valid Usage (Implicit)
-  [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
-  [`dpy`] **must**  be a valid pointer to a [`Display`] value
-  [`display`] **must**  be a valid [`DisplayKHR`] handle
-  [`display`] **must**  have been created, allocated, or retrieved from [`physical_device`]

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_INITIALIZATION_FAILED`

# Related
- [`VK_EXT_acquire_xlib_display`]
- [`DisplayKHR`]
- [`PhysicalDevice`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        