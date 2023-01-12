[VkInstance](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkInstance.html) - Opaque handle to an instance object

# C Specifications
There is no global state in Vulkan and all per-application state is stored
in a [`Instance`] object.
Creating a [`Instance`] object initializes the Vulkan library and allows
the application to pass information about itself to the implementation.Instances are represented by [`Instance`] handles:
```c
// Provided by VK_VERSION_1_0
VK_DEFINE_HANDLE(VkInstance)
```

# Related
- [`crate::vulkan1_0`]
- [`create_android_surface_khr`]
- [`create_debug_report_callback_ext`]
- [`create_debug_utils_messenger_ext`]
- [`create_direct_fb_surface_ext`]
- [`create_display_plane_surface_khr`]
- [`create_headless_surface_ext`]
- [`create_ios_surface_mvk`]
- [`create_image_pipe_surface_fuchsia`]
- [`create_instance`]
- [`create_mac_os_surface_mvk`]
- [`create_metal_surface_ext`]
- [`create_screen_surface_qnx`]
- [`create_stream_descriptor_surface_ggp`]
- [`create_vi_surface_nn`]
- [`create_wayland_surface_khr`]
- [`create_win32_surface_khr`]
- [`create_xcb_surface_khr`]
- [`create_xlib_surface_khr`]
- [`debug_report_message_ext`]
- [`destroy_debug_report_callback_ext`]
- [`destroy_debug_utils_messenger_ext`]
- [`destroy_instance`]
- [`destroy_surface_khr`]
- [`enumerate_physical_device_groups`]
- [`enumerate_physical_device_groups_khr`]
- [`enumerate_physical_devices`]
- [`get_instance_proc_addr`]
- [`submit_debug_utils_message_ext`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        