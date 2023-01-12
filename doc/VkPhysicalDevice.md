[VkPhysicalDevice](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevice.html) - Opaque handle to a physical device object

# C Specifications
Vulkan separates the concept of *physical* and *logical* devices.
A physical device usually represents a single complete implementation of
Vulkan (excluding instance-level functionality) available to the host, of
which there are a finite number.
A logical device represents an instance of that implementation with its own
state and resources independent of other logical devices.Physical devices are represented by [`PhysicalDevice`] handles:
```c
// Provided by VK_VERSION_1_0
VK_DEFINE_HANDLE(VkPhysicalDevice)
```

# Related
- [`crate::vulkan1_0`]
- [`DeviceGroupDeviceCreateInfo`]
- [`PhysicalDeviceGroupProperties`]
- [`acquire_drm_display_ext`]
- [`acquire_winrt_display_nv`]
- [`acquire_xlib_display_ext`]
- [`create_device`]
- [`create_display_mode_khr`]
- [`enumerate_device_extension_properties`]
- [`enumerate_device_layer_properties`]
- [`enumerate_physical_device_queue_family_performance_query_counters_khr`]
- [`enumerate_physical_devices`]
- [`get_display_mode_properties2_khr`]
- [`get_display_mode_properties_khr`]
- [`get_display_plane_capabilities2_khr`]
- [`get_display_plane_capabilities_khr`]
- [`get_display_plane_supported_displays_khr`]
- [`get_drm_display_ext`]
- [`get_physical_device_calibrateable_time_domains_ext`]
- [`get_physical_device_cooperative_matrix_properties_nv`]
- [`get_physical_device_direct_fb_presentation_support_ext`]
- [`get_physical_device_display_plane_properties2_khr`]
- [`get_physical_device_display_plane_properties_khr`]
- [`get_physical_device_display_properties2_khr`]
- [`get_physical_device_display_properties_khr`]
- [`get_physical_device_external_buffer_properties`]
- [`get_physical_device_external_buffer_properties_khr`]
- [`get_physical_device_external_fence_properties`]
- [`get_physical_device_external_fence_properties_khr`]
- [`get_physical_device_external_image_format_properties_nv`]
- [`get_physical_device_external_semaphore_properties`]
- [`get_physical_device_external_semaphore_properties_khr`]
- [`get_physical_device_features`]
- [`get_physical_device_features2`]
- [`get_physical_device_features2_khr`]
- [`get_physical_device_format_properties`]
- [`get_physical_device_format_properties2`]
- [`get_physical_device_format_properties2_khr`]
- [`get_physical_device_fragment_shading_rates_khr`]
- [`get_physical_device_image_format_properties`]
- [`get_physical_device_image_format_properties2`]
- [`get_physical_device_image_format_properties2_khr`]
- [`get_physical_device_memory_properties`]
- [`get_physical_device_memory_properties2`]
- [`get_physical_device_memory_properties2_khr`]
- [`get_physical_device_multisample_properties_ext`]
- [`get_physical_device_present_rectangles_khr`]
- [`get_physical_device_properties`]
- [`get_physical_device_properties2`]
- [`get_physical_device_properties2_khr`]
- [`get_physical_device_queue_family_performance_query_passes_khr`]
- [`get_physical_device_queue_family_properties`]
- [`get_physical_device_queue_family_properties2`]
- [`get_physical_device_queue_family_properties2_khr`]
- [`get_physical_device_screen_presentation_support_qnx`]
- [`get_physical_device_sparse_image_format_properties`]
- [`get_physical_device_sparse_image_format_properties2`]
- [`get_physical_device_sparse_image_format_properties2_khr`]
- [`get_physical_device_supported_framebuffer_mixed_samples_combinations_nv`]
- [`get_physical_device_surface_capabilities2_ext`]
- [`get_physical_device_surface_capabilities2_khr`]
- [`get_physical_device_surface_capabilities_khr`]
- [`get_physical_device_surface_formats2_khr`]
- [`get_physical_device_surface_formats_khr`]
- [`get_physical_device_surface_present_modes2_ext`]
- [`get_physical_device_surface_present_modes_khr`]
- [`get_physical_device_surface_support_khr`]
- [`get_physical_device_tool_properties`]
- [`get_physical_device_tool_properties_ext`]
- [`get_physical_device_video_capabilities_khr`]
- [`get_physical_device_video_format_properties_khr`]
- [`get_physical_device_wayland_presentation_support_khr`]
- [`get_physical_device_win32_presentation_support_khr`]
- [`get_physical_device_xcb_presentation_support_khr`]
- [`get_physical_device_xlib_presentation_support_khr`]
- [`get_rand_r_output_display_ext`]
- [`get_winrt_display_nv`]
- [`release_display_ext`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        