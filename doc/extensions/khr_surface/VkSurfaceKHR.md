[VkSurfaceKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSurfaceKHR.html) - Opaque handle to a surface object

# C Specifications
Native platform surface or window objects are abstracted by surface objects,
which are represented by [`SurfaceKHR`] handles:
```c
// Provided by VK_KHR_surface
VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkSurfaceKHR)
```

# Related
- [`VK_KHR_surface`]
- [`PhysicalDeviceSurfaceInfo2KHR`]
- [`SwapchainCreateInfoKHR`]
- [`create_android_surface_khr`]
- [`create_direct_fb_surface_ext`]
- [`create_display_plane_surface_khr`]
- [`create_headless_surface_ext`]
- [`create_ios_surface_mvk`]
- [`create_image_pipe_surface_fuchsia`]
- [`create_mac_os_surface_mvk`]
- [`create_metal_surface_ext`]
- [`create_screen_surface_qnx`]
- [`create_stream_descriptor_surface_ggp`]
- [`create_vi_surface_nn`]
- [`create_wayland_surface_khr`]
- [`create_win32_surface_khr`]
- [`create_xcb_surface_khr`]
- [`create_xlib_surface_khr`]
- [`destroy_surface_khr`]
- [`get_device_group_surface_present_modes_khr`]
- [`get_physical_device_present_rectangles_khr`]
- [`get_physical_device_surface_capabilities2_ext`]
- [`get_physical_device_surface_capabilities_khr`]
- [`get_physical_device_surface_formats_khr`]
- [`get_physical_device_surface_present_modes_khr`]
- [`get_physical_device_surface_support_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        