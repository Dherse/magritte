[VkCommandBuffer](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCommandBuffer.html) - Opaque handle to a command buffer object

# C Specifications
Command buffers are objects used to record commands which  **can**  be
subsequently submitted to a device queue for execution.
There are two levels of command buffers - *primary command buffers*, which
 **can**  execute secondary command buffers, and which are submitted to queues,
and *secondary command buffers*, which  **can**  be executed by primary command
buffers, and which are not directly submitted to queues.Command buffers are represented by [`CommandBuffer`] handles:
```c
// Provided by VK_VERSION_1_0
VK_DEFINE_HANDLE(VkCommandBuffer)
```

# Related
- [`crate::vulkan1_0`]
- [`CommandBufferSubmitInfo`]
- [`SubmitInfo`]
- [`allocate_command_buffers`]
- [`begin_command_buffer`]
- [`cmd_begin_conditional_rendering_ext`]
- [`cmd_begin_debug_utils_label_ext`]
- [`cmd_begin_query`]
- [`cmd_begin_query_indexed_ext`]
- [`cmd_begin_render_pass`]
- [`cmd_begin_render_pass2`]
- [`cmd_begin_render_pass2_khr`]
- [`cmd_begin_rendering`]
- [`cmd_begin_rendering_khr`]
- [`cmd_begin_transform_feedback_ext`]
- [`cmd_begin_video_coding_khr`]
- [`cmd_bind_descriptor_sets`]
- [`cmd_bind_index_buffer`]
- [`cmd_bind_invocation_mask_huawei`]
- [`cmd_bind_pipeline`]
- [`cmd_bind_pipeline_shader_group_nv`]
- [`cmd_bind_shading_rate_image_nv`]
- [`cmd_bind_transform_feedback_buffers_ext`]
- [`cmd_bind_vertex_buffers`]
- [`cmd_bind_vertex_buffers2`]
- [`cmd_bind_vertex_buffers2_ext`]
- [`cmd_blit_image`]
- [`cmd_blit_image2`]
- [`cmd_blit_image2_khr`]
- [`cmd_build_acceleration_structure_nv`]
- [`cmd_build_acceleration_structures_indirect_khr`]
- [`cmd_build_acceleration_structures_khr`]
- [`cmd_clear_attachments`]
- [`cmd_clear_color_image`]
- [`cmd_clear_depth_stencil_image`]
- [`cmd_control_video_coding_khr`]
- [`cmd_copy_acceleration_structure_khr`]
- [`cmd_copy_acceleration_structure_nv`]
- [`cmd_copy_acceleration_structure_to_memory_khr`]
- [`cmd_copy_buffer`]
- [`cmd_copy_buffer2`]
- [`cmd_copy_buffer2_khr`]
- [`cmd_copy_buffer_to_image`]
- [`cmd_copy_buffer_to_image2`]
- [`cmd_copy_buffer_to_image2_khr`]
- [`cmd_copy_image`]
- [`cmd_copy_image2`]
- [`cmd_copy_image2_khr`]
- [`cmd_copy_image_to_buffer`]
- [`cmd_copy_image_to_buffer2`]
- [`cmd_copy_image_to_buffer2_khr`]
- [`cmd_copy_memory_to_acceleration_structure_khr`]
- [`cmd_copy_query_pool_results`]
- [`cmd_cu_launch_kernel_nvx`]
- [`cmd_debug_marker_begin_ext`]
- [`cmd_debug_marker_end_ext`]
- [`cmd_debug_marker_insert_ext`]
- [`cmd_decode_video_khr`]
- [`cmd_dispatch`]
- [`cmd_dispatch_base`]
- [`cmd_dispatch_base_khr`]
- [`cmd_dispatch_indirect`]
- [`cmd_draw`]
- [`cmd_draw_indexed`]
- [`cmd_draw_indexed_indirect`]
- [`cmd_draw_indexed_indirect_count`]
- [`cmd_draw_indexed_indirect_count_amd`]
- [`cmd_draw_indexed_indirect_count_khr`]
- [`cmd_draw_indirect`]
- [`cmd_draw_indirect_byte_count_ext`]
- [`cmd_draw_indirect_count`]
- [`cmd_draw_indirect_count_amd`]
- [`cmd_draw_indirect_count_khr`]
- [`cmd_draw_mesh_tasks_indirect_count_nv`]
- [`cmd_draw_mesh_tasks_indirect_nv`]
- [`cmd_draw_mesh_tasks_nv`]
- [`cmd_draw_multi_ext`]
- [`cmd_draw_multi_indexed_ext`]
- [`cmd_encode_video_khr`]
- [`cmd_end_conditional_rendering_ext`]
- [`cmd_end_debug_utils_label_ext`]
- [`cmd_end_query`]
- [`cmd_end_query_indexed_ext`]
- [`cmd_end_render_pass`]
- [`cmd_end_render_pass2`]
- [`cmd_end_render_pass2_khr`]
- [`cmd_end_rendering`]
- [`cmd_end_rendering_khr`]
- [`cmd_end_transform_feedback_ext`]
- [`cmd_end_video_coding_khr`]
- [`cmd_execute_commands`]
- [`cmd_execute_generated_commands_nv`]
- [`cmd_fill_buffer`]
- [`cmd_insert_debug_utils_label_ext`]
- [`cmd_next_subpass`]
- [`cmd_next_subpass2`]
- [`cmd_next_subpass2_khr`]
- [`cmd_pipeline_barrier`]
- [`cmd_pipeline_barrier2`]
- [`cmd_pipeline_barrier2_khr`]
- [`cmd_preprocess_generated_commands_nv`]
- [`cmd_push_constants`]
- [`cmd_push_descriptor_set_khr`]
- [`cmd_push_descriptor_set_with_template_khr`]
- [`cmd_reset_event`]
- [`cmd_reset_event2`]
- [`cmd_reset_event2_khr`]
- [`cmd_reset_query_pool`]
- [`cmd_resolve_image`]
- [`cmd_resolve_image2`]
- [`cmd_resolve_image2_khr`]
- [`cmd_set_blend_constants`]
- [`cmd_set_checkpoint_nv`]
- [`cmd_set_coarse_sample_order_nv`]
- [`cmd_set_color_write_enable_ext`]
- [`cmd_set_cull_mode`]
- [`cmd_set_cull_mode_ext`]
- [`cmd_set_depth_bias`]
- [`cmd_set_depth_bias_enable`]
- [`cmd_set_depth_bias_enable_ext`]
- [`cmd_set_depth_bounds`]
- [`cmd_set_depth_bounds_test_enable`]
- [`cmd_set_depth_bounds_test_enable_ext`]
- [`cmd_set_depth_compare_op`]
- [`cmd_set_depth_compare_op_ext`]
- [`cmd_set_depth_test_enable`]
- [`cmd_set_depth_test_enable_ext`]
- [`cmd_set_depth_write_enable`]
- [`cmd_set_depth_write_enable_ext`]
- [`cmd_set_device_mask`]
- [`cmd_set_device_mask_khr`]
- [`cmd_set_discard_rectangle_ext`]
- [`cmd_set_event`]
- [`cmd_set_event2`]
- [`cmd_set_event2_khr`]
- [`cmd_set_exclusive_scissor_nv`]
- [`cmd_set_fragment_shading_rate_enum_nv`]
- [`cmd_set_fragment_shading_rate_khr`]
- [`cmd_set_front_face`]
- [`cmd_set_front_face_ext`]
- [`cmd_set_line_stipple_ext`]
- [`cmd_set_line_width`]
- [`cmd_set_logic_op_ext`]
- [`cmd_set_patch_control_points_ext`]
- [`cmd_set_performance_marker_intel`]
- [`cmd_set_performance_override_intel`]
- [`cmd_set_performance_stream_marker_intel`]
- [`cmd_set_primitive_restart_enable`]
- [`cmd_set_primitive_restart_enable_ext`]
- [`cmd_set_primitive_topology`]
- [`cmd_set_primitive_topology_ext`]
- [`cmd_set_rasterizer_discard_enable`]
- [`cmd_set_rasterizer_discard_enable_ext`]
- [`cmd_set_ray_tracing_pipeline_stack_size_khr`]
- [`cmd_set_sample_locations_ext`]
- [`cmd_set_scissor`]
- [`cmd_set_scissor_with_count`]
- [`cmd_set_scissor_with_count_ext`]
- [`cmd_set_stencil_compare_mask`]
- [`cmd_set_stencil_op`]
- [`cmd_set_stencil_op_ext`]
- [`cmd_set_stencil_reference`]
- [`cmd_set_stencil_test_enable`]
- [`cmd_set_stencil_test_enable_ext`]
- [`cmd_set_stencil_write_mask`]
- [`cmd_set_vertex_input_ext`]
- [`cmd_set_viewport`]
- [`cmd_set_viewport_shading_rate_palette_nv`]
- [`cmd_set_viewport_w_scaling_nv`]
- [`cmd_set_viewport_with_count`]
- [`cmd_set_viewport_with_count_ext`]
- [`cmd_subpass_shading_huawei`]
- [`cmd_trace_rays_indirect_khr`]
- [`cmd_trace_rays_khr`]
- [`cmd_trace_rays_nv`]
- [`cmd_update_buffer`]
- [`cmd_wait_events`]
- [`cmd_wait_events2`]
- [`cmd_wait_events2_khr`]
- [`cmd_write_acceleration_structures_properties_khr`]
- [`cmd_write_acceleration_structures_properties_nv`]
- [`cmd_write_buffer_marker2_amd`]
- [`cmd_write_buffer_marker_amd`]
- [`cmd_write_timestamp`]
- [`cmd_write_timestamp2`]
- [`cmd_write_timestamp2_khr`]
- [`end_command_buffer`]
- [`free_command_buffers`]
- [`reset_command_buffer`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        