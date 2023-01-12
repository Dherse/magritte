[VkAllocationCallbacks](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAllocationCallbacks.html) - Structure containing callback function pointers for memory allocation

# C Specifications
Allocators are provided by the application as a pointer to a
[`AllocationCallbacks`] structure:
```c
// Provided by VK_VERSION_1_0
typedef struct VkAllocationCallbacks {
    void*                                   pUserData;
    PFN_vkAllocationFunction                pfnAllocation;
    PFN_vkReallocationFunction              pfnReallocation;
    PFN_vkFreeFunction                      pfnFree;
    PFN_vkInternalAllocationNotification    pfnInternalAllocation;
    PFN_vkInternalFreeNotification          pfnInternalFree;
} VkAllocationCallbacks;
```

# Members
- [`user_data`] is a value to be interpreted by the implementation of the callbacks. When any of the callbacks in [`AllocationCallbacks`] are called, the Vulkan implementation will pass this value as the first parameter to the callback. This value  **can**  vary each time an allocator is passed into a command, even when the same object takes an allocator in multiple commands.
- [`pfn_allocation`] is a [`PFNAllocationFunction`] pointer to an application-defined memory allocation function.
- [`pfn_reallocation`] is a [`PFNReallocationFunction`] pointer to an application-defined memory reallocation function.
- [`pfn_free`] is a [`PFNFreeFunction`] pointer to an application-defined memory free function.
- [`pfn_internal_allocation`] is a [`PFNInternalAllocationNotification`] pointer to an application-defined function that is called by the implementation when the implementation makes internal allocations.
- [`pfn_internal_free`] is a [`PFNInternalFreeNotification`] pointer to an application-defined function that is called by the implementation when the implementation frees internal allocations.

# Description
## Valid Usage
-  [`pfn_allocation`] **must**  be a valid pointer to a valid user-defined [`PFNAllocationFunction`]
-  [`pfn_reallocation`] **must**  be a valid pointer to a valid user-defined [`PFNReallocationFunction`]
-  [`pfn_free`] **must**  be a valid pointer to a valid user-defined [`PFNFreeFunction`]
-    If either of [`pfn_internal_allocation`] or [`pfn_internal_free`] is not `NULL`, both  **must**  be valid callbacks

# Related
- [`PFNAllocationFunction`]
- [`PFNFreeFunction`]
- [`PFNInternalAllocationNotification`]
- [`PFNInternalFreeNotification`]
- [`PFNReallocationFunction`]
- [`crate::vulkan1_0`]
- [`allocate_memory`]
- [`create_acceleration_structure_khr`]
- [`create_acceleration_structure_nv`]
- [`create_android_surface_khr`]
- [`create_buffer`]
- [`create_buffer_collection_fuchsia`]
- [`create_buffer_view`]
- [`create_command_pool`]
- [`create_compute_pipelines`]
- [`create_cu_function_nvx`]
- [`create_cu_module_nvx`]
- [`create_debug_report_callback_ext`]
- [`create_debug_utils_messenger_ext`]
- [`create_deferred_operation_khr`]
- [`create_descriptor_pool`]
- [`create_descriptor_set_layout`]
- [`create_descriptor_update_template`]
- [`create_descriptor_update_template_khr`]
- [`create_device`]
- [`create_direct_fb_surface_ext`]
- [`create_display_mode_khr`]
- [`create_display_plane_surface_khr`]
- [`create_event`]
- [`create_fence`]
- [`create_framebuffer`]
- [`create_graphics_pipelines`]
- [`create_headless_surface_ext`]
- [`create_ios_surface_mvk`]
- [`create_image`]
- [`create_image_pipe_surface_fuchsia`]
- [`create_image_view`]
- [`create_indirect_commands_layout_nv`]
- [`create_instance`]
- [`create_mac_os_surface_mvk`]
- [`create_metal_surface_ext`]
- [`create_pipeline_cache`]
- [`create_pipeline_layout`]
- [`create_private_data_slot`]
- [`create_private_data_slot_ext`]
- [`create_query_pool`]
- [`create_ray_tracing_pipelines_khr`]
- [`create_ray_tracing_pipelines_nv`]
- [`create_render_pass`]
- [`create_render_pass2`]
- [`create_render_pass2_khr`]
- [`create_sampler`]
- [`create_sampler_ycbcr_conversion`]
- [`create_sampler_ycbcr_conversion_khr`]
- [`create_screen_surface_qnx`]
- [`create_semaphore`]
- [`create_shader_module`]
- [`create_shared_swapchains_khr`]
- [`create_stream_descriptor_surface_ggp`]
- [`create_swapchain_khr`]
- [`create_validation_cache_ext`]
- [`create_vi_surface_nn`]
- [`create_video_session_khr`]
- [`create_video_session_parameters_khr`]
- [`create_wayland_surface_khr`]
- [`create_win32_surface_khr`]
- [`create_xcb_surface_khr`]
- [`create_xlib_surface_khr`]
- [`destroy_acceleration_structure_khr`]
- [`destroy_acceleration_structure_nv`]
- [`destroy_buffer`]
- [`destroy_buffer_collection_fuchsia`]
- [`destroy_buffer_view`]
- [`destroy_command_pool`]
- [`destroy_cu_function_nvx`]
- [`destroy_cu_module_nvx`]
- [`destroy_debug_report_callback_ext`]
- [`destroy_debug_utils_messenger_ext`]
- [`destroy_deferred_operation_khr`]
- [`destroy_descriptor_pool`]
- [`destroy_descriptor_set_layout`]
- [`destroy_descriptor_update_template`]
- [`destroy_descriptor_update_template_khr`]
- [`destroy_device`]
- [`destroy_event`]
- [`destroy_fence`]
- [`destroy_framebuffer`]
- [`destroy_image`]
- [`destroy_image_view`]
- [`destroy_indirect_commands_layout_nv`]
- [`destroy_instance`]
- [`destroy_pipeline`]
- [`destroy_pipeline_cache`]
- [`destroy_pipeline_layout`]
- [`destroy_private_data_slot`]
- [`destroy_private_data_slot_ext`]
- [`destroy_query_pool`]
- [`destroy_render_pass`]
- [`destroy_sampler`]
- [`destroy_sampler_ycbcr_conversion`]
- [`destroy_sampler_ycbcr_conversion_khr`]
- [`destroy_semaphore`]
- [`destroy_shader_module`]
- [`destroy_surface_khr`]
- [`destroy_swapchain_khr`]
- [`destroy_validation_cache_ext`]
- [`destroy_video_session_khr`]
- [`destroy_video_session_parameters_khr`]
- [`free_memory`]
- [`register_device_event_ext`]
- [`register_display_event_ext`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        