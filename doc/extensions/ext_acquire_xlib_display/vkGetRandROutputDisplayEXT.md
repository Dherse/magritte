[vkGetRandROutputDisplayEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetRandROutputDisplayEXT.html) - Query the VkDisplayKHR corresponding to an X11 RandR Output

# C Specifications
When acquiring displays from an X11 server, an application may also wish to
enumerate and identify them using a native handle rather than a
[`DisplayKHR`] handle.
To determine the [`DisplayKHR`] handle corresponding to an X11 RandR
Output, call:
```c
// Provided by VK_EXT_acquire_xlib_display
VkResult vkGetRandROutputDisplayEXT(
    VkPhysicalDevice                            physicalDevice,
    Display*                                    dpy,
    RROutput                                    rrOutput,
    VkDisplayKHR*                               pDisplay);
```

# Parameters
- [`physical_device`] The physical device to query the display handle on.
- [`dpy`] A connection to the X11 server from which [`rr_output`] was queried.
- [`rr_output`] An X11 RandR output ID.
- [`p_display`] The corresponding [`DisplayKHR`] handle will be returned here.

# Description
If there is no [`DisplayKHR`] corresponding to [`rr_output`] on
[`physical_device`], [`crate::Handle::null`] **must**  be returned in
[`p_display`].
## Valid Usage (Implicit)
-  [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
-  [`dpy`] **must**  be a valid pointer to a `Display` value
-  [`p_display`] **must**  be a valid pointer to a [`DisplayKHR`] handle

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`

# Related
- [`ext_acquire_xlib_display`]
- [`DisplayKHR`]
- [`PhysicalDevice`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        