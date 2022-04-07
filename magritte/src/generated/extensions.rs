
//!# Extensions
//!This module contains all of the registered extensions gated by relevant feature gates.

use crate::cstr_ptr;
use std::os::raw::c_char;
#[cfg(feature = "VK_AMD_buffer_marker")]
pub mod amd_buffer_marker;
#[cfg(feature = "VK_AMD_device_coherent_memory")]
pub mod amd_device_coherent_memory;
#[cfg(feature = "VK_AMD_display_native_hdr")]
pub mod amd_display_native_hdr;
#[cfg(feature = "VK_AMD_draw_indirect_count")]
pub mod amd_draw_indirect_count;
#[cfg(feature = "VK_AMD_gcn_shader")]
pub mod amd_gcn_shader;
#[cfg(feature = "VK_AMD_gpu_shader_half_float")]
pub mod amd_gpu_shader_half_float;
#[cfg(feature = "VK_AMD_gpu_shader_int16")]
pub mod amd_gpu_shader_int_16;
#[cfg(feature = "VK_AMD_memory_overallocation_behavior")]
pub mod amd_memory_overallocation_behavior;
#[cfg(feature = "VK_AMD_mixed_attachment_samples")]
pub mod amd_mixed_attachment_samples;
#[cfg(feature = "VK_AMD_negative_viewport_height")]
pub mod amd_negative_viewport_height;
#[cfg(feature = "VK_AMD_pipeline_compiler_control")]
pub mod amd_pipeline_compiler_control;
#[cfg(feature = "VK_AMD_rasterization_order")]
pub mod amd_rasterization_order;
#[cfg(feature = "VK_AMD_shader_ballot")]
pub mod amd_shader_ballot;
#[cfg(feature = "VK_AMD_shader_core_properties")]
pub mod amd_shader_core_properties;
#[cfg(feature = "VK_AMD_shader_core_properties2")]
pub mod amd_shader_core_properties_2;
#[cfg(feature = "VK_AMD_shader_explicit_vertex_parameter")]
pub mod amd_shader_explicit_vertex_parameter;
#[cfg(feature = "VK_AMD_shader_fragment_mask")]
pub mod amd_shader_fragment_mask;
#[cfg(feature = "VK_AMD_shader_image_load_store_lod")]
pub mod amd_shader_image_load_store_lod;
#[cfg(feature = "VK_AMD_shader_info")]
pub mod amd_shader_info;
#[cfg(feature = "VK_AMD_shader_trinary_minmax")]
pub mod amd_shader_trinary_minmax;
#[cfg(feature = "VK_AMD_texture_gather_bias_lod")]
pub mod amd_texture_gather_bias_lod;
#[cfg(feature = "VK_ANDROID_external_memory_android_hardware_buffer")]
pub mod android_external_memory_android_hardware_buffer;
#[cfg(feature = "VK_ARM_rasterization_order_attachment_access")]
pub mod arm_rasterization_order_attachment_access;
#[cfg(feature = "VK_EXT_4444_formats")]
pub mod ext_4444_formats;
#[cfg(feature = "VK_EXT_acquire_drm_display")]
pub mod ext_acquire_drm_display;
#[cfg(feature = "VK_EXT_acquire_xlib_display")]
pub mod ext_acquire_xlib_display;
#[cfg(feature = "VK_EXT_astc_decode_mode")]
pub mod ext_astc_decode_mode;
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
pub mod ext_blend_operation_advanced;
#[cfg(feature = "VK_EXT_border_color_swizzle")]
pub mod ext_border_color_swizzle;
#[cfg(feature = "VK_EXT_buffer_device_address")]
pub mod ext_buffer_device_address;
#[cfg(feature = "VK_EXT_calibrated_timestamps")]
pub mod ext_calibrated_timestamps;
#[cfg(feature = "VK_EXT_color_write_enable")]
pub mod ext_color_write_enable;
#[cfg(feature = "VK_EXT_conditional_rendering")]
pub mod ext_conditional_rendering;
#[cfg(feature = "VK_EXT_conservative_rasterization")]
pub mod ext_conservative_rasterization;
#[cfg(feature = "VK_EXT_custom_border_color")]
pub mod ext_custom_border_color;
#[cfg(feature = "VK_EXT_debug_marker")]
pub mod ext_debug_marker;
#[cfg(feature = "VK_EXT_debug_report")]
pub mod ext_debug_report;
#[cfg(feature = "VK_EXT_debug_utils")]
pub mod ext_debug_utils;
#[cfg(feature = "VK_EXT_depth_clip_control")]
pub mod ext_depth_clip_control;
#[cfg(feature = "VK_EXT_depth_clip_enable")]
pub mod ext_depth_clip_enable;
#[cfg(feature = "VK_EXT_depth_range_unrestricted")]
pub mod ext_depth_range_unrestricted;
#[cfg(feature = "VK_EXT_descriptor_indexing")]
pub mod ext_descriptor_indexing;
#[cfg(feature = "VK_EXT_device_memory_report")]
pub mod ext_device_memory_report;
#[cfg(feature = "VK_EXT_direct_mode_display")]
pub mod ext_direct_mode_display;
#[cfg(feature = "VK_EXT_directfb_surface")]
pub mod ext_directfb_surface;
#[cfg(feature = "VK_EXT_discard_rectangles")]
pub mod ext_discard_rectangles;
#[cfg(feature = "VK_EXT_display_control")]
pub mod ext_display_control;
#[cfg(feature = "VK_EXT_display_surface_counter")]
pub mod ext_display_surface_counter;
#[cfg(feature = "VK_EXT_extended_dynamic_state")]
pub mod ext_extended_dynamic_state;
#[cfg(feature = "VK_EXT_extended_dynamic_state2")]
pub mod ext_extended_dynamic_state_2;
#[cfg(feature = "VK_EXT_external_memory_dma_buf")]
pub mod ext_external_memory_dma_buf;
#[cfg(feature = "VK_EXT_external_memory_host")]
pub mod ext_external_memory_host;
#[cfg(feature = "VK_EXT_filter_cubic")]
pub mod ext_filter_cubic;
#[cfg(feature = "VK_EXT_fragment_density_map")]
pub mod ext_fragment_density_map;
#[cfg(feature = "VK_EXT_fragment_density_map2")]
pub mod ext_fragment_density_map_2;
#[cfg(feature = "VK_EXT_fragment_shader_interlock")]
pub mod ext_fragment_shader_interlock;
#[cfg(feature = "VK_EXT_full_screen_exclusive")]
pub mod ext_full_screen_exclusive;
#[cfg(feature = "VK_EXT_global_priority")]
pub mod ext_global_priority;
#[cfg(feature = "VK_EXT_global_priority_query")]
pub mod ext_global_priority_query;
#[cfg(feature = "VK_EXT_hdr_metadata")]
pub mod ext_hdr_metadata;
#[cfg(feature = "VK_EXT_headless_surface")]
pub mod ext_headless_surface;
#[cfg(feature = "VK_EXT_host_query_reset")]
pub mod ext_host_query_reset;
#[cfg(feature = "VK_EXT_image_drm_format_modifier")]
pub mod ext_image_drm_format_modifier;
#[cfg(feature = "VK_EXT_image_robustness")]
pub mod ext_image_robustness;
#[cfg(feature = "VK_EXT_image_view_min_lod")]
pub mod ext_image_view_min_lod;
#[cfg(feature = "VK_EXT_index_type_uint8")]
pub mod ext_index_type_uint_8;
#[cfg(feature = "VK_EXT_inline_uniform_block")]
pub mod ext_inline_uniform_block;
#[cfg(feature = "VK_EXT_line_rasterization")]
pub mod ext_line_rasterization;
#[cfg(feature = "VK_EXT_load_store_op_none")]
pub mod ext_load_store_op_none;
#[cfg(feature = "VK_EXT_memory_budget")]
pub mod ext_memory_budget;
#[cfg(feature = "VK_EXT_memory_priority")]
pub mod ext_memory_priority;
#[cfg(feature = "VK_EXT_metal_surface")]
pub mod ext_metal_surface;
#[cfg(feature = "VK_EXT_multi_draw")]
pub mod ext_multi_draw;
#[cfg(feature = "VK_EXT_pageable_device_local_memory")]
pub mod ext_pageable_device_local_memory;
#[cfg(feature = "VK_EXT_pci_bus_info")]
pub mod ext_pci_bus_info;
#[cfg(feature = "VK_EXT_physical_device_drm")]
pub mod ext_physical_device_drm;
#[cfg(feature = "VK_EXT_pipeline_creation_cache_control")]
pub mod ext_pipeline_creation_cache_control;
#[cfg(feature = "VK_EXT_pipeline_creation_feedback")]
pub mod ext_pipeline_creation_feedback;
#[cfg(feature = "VK_EXT_post_depth_coverage")]
pub mod ext_post_depth_coverage;
#[cfg(feature = "VK_EXT_primitive_topology_list_restart")]
pub mod ext_primitive_topology_list_restart;
#[cfg(feature = "VK_EXT_private_data")]
pub mod ext_private_data;
#[cfg(feature = "VK_EXT_provoking_vertex")]
pub mod ext_provoking_vertex;
#[cfg(feature = "VK_EXT_queue_family_foreign")]
pub mod ext_queue_family_foreign;
#[cfg(feature = "VK_EXT_rgba10x6_formats")]
pub mod ext_rgba_10_x_6_formats;
#[cfg(feature = "VK_EXT_robustness2")]
pub mod ext_robustness_2;
#[cfg(feature = "VK_EXT_sample_locations")]
pub mod ext_sample_locations;
#[cfg(feature = "VK_EXT_sampler_filter_minmax")]
pub mod ext_sampler_filter_minmax;
#[cfg(feature = "VK_EXT_scalar_block_layout")]
pub mod ext_scalar_block_layout;
#[cfg(feature = "VK_EXT_separate_stencil_usage")]
pub mod ext_separate_stencil_usage;
#[cfg(feature = "VK_EXT_shader_atomic_float")]
pub mod ext_shader_atomic_float;
#[cfg(feature = "VK_EXT_shader_atomic_float2")]
pub mod ext_shader_atomic_float_2;
#[cfg(feature = "VK_EXT_shader_demote_to_helper_invocation")]
pub mod ext_shader_demote_to_helper_invocation;
#[cfg(feature = "VK_EXT_shader_image_atomic_int64")]
pub mod ext_shader_image_atomic_int_64;
#[cfg(feature = "VK_EXT_shader_stencil_export")]
pub mod ext_shader_stencil_export;
#[cfg(feature = "VK_EXT_shader_subgroup_ballot")]
pub mod ext_shader_subgroup_ballot;
#[cfg(feature = "VK_EXT_shader_subgroup_vote")]
pub mod ext_shader_subgroup_vote;
#[cfg(feature = "VK_EXT_shader_viewport_index_layer")]
pub mod ext_shader_viewport_index_layer;
#[cfg(feature = "VK_EXT_subgroup_size_control")]
pub mod ext_subgroup_size_control;
#[cfg(feature = "VK_EXT_swapchain_colorspace")]
pub mod ext_swapchain_colorspace;
#[cfg(feature = "VK_EXT_texel_buffer_alignment")]
pub mod ext_texel_buffer_alignment;
#[cfg(feature = "VK_EXT_texture_compression_astc_hdr")]
pub mod ext_texture_compression_astc_hdr;
#[cfg(feature = "VK_EXT_tooling_info")]
pub mod ext_tooling_info;
#[cfg(feature = "VK_EXT_transform_feedback")]
pub mod ext_transform_feedback;
#[cfg(feature = "VK_EXT_validation_cache")]
pub mod ext_validation_cache;
#[cfg(feature = "VK_EXT_validation_features")]
pub mod ext_validation_features;
#[cfg(feature = "VK_EXT_validation_flags")]
pub mod ext_validation_flags;
#[cfg(feature = "VK_EXT_vertex_attribute_divisor")]
pub mod ext_vertex_attribute_divisor;
#[cfg(feature = "VK_EXT_vertex_input_dynamic_state")]
pub mod ext_vertex_input_dynamic_state;
#[cfg(feature = "VK_EXT_video_decode_h264")]
pub mod ext_video_decode_h_264;
#[cfg(feature = "VK_EXT_video_decode_h265")]
pub mod ext_video_decode_h_265;
#[cfg(feature = "VK_EXT_video_encode_h264")]
pub mod ext_video_encode_h_264;
#[cfg(feature = "VK_EXT_video_encode_h265")]
pub mod ext_video_encode_h_265;
#[cfg(feature = "VK_EXT_ycbcr_2plane_444_formats")]
pub mod ext_ycbcr_2_plane_444_formats;
#[cfg(feature = "VK_EXT_ycbcr_image_arrays")]
pub mod ext_ycbcr_image_arrays;
#[cfg(feature = "VK_FUCHSIA_buffer_collection")]
pub mod fuchsia_buffer_collection;
#[cfg(feature = "VK_FUCHSIA_external_memory")]
pub mod fuchsia_external_memory;
#[cfg(feature = "VK_FUCHSIA_external_semaphore")]
pub mod fuchsia_external_semaphore;
#[cfg(feature = "VK_FUCHSIA_imagepipe_surface")]
pub mod fuchsia_imagepipe_surface;
#[cfg(feature = "VK_GGP_frame_token")]
pub mod ggp_frame_token;
#[cfg(feature = "VK_GGP_stream_descriptor_surface")]
pub mod ggp_stream_descriptor_surface;
#[cfg(feature = "VK_GOOGLE_decorate_string")]
pub mod google_decorate_string;
#[cfg(feature = "VK_GOOGLE_display_timing")]
pub mod google_display_timing;
#[cfg(feature = "VK_GOOGLE_hlsl_functionality1")]
pub mod google_hlsl_functionality_1;
#[cfg(feature = "VK_GOOGLE_surfaceless_query")]
pub mod google_surfaceless_query;
#[cfg(feature = "VK_GOOGLE_user_type")]
pub mod google_user_type;
#[cfg(feature = "VK_HUAWEI_invocation_mask")]
pub mod huawei_invocation_mask;
#[cfg(feature = "VK_HUAWEI_subpass_shading")]
pub mod huawei_subpass_shading;
#[cfg(feature = "VK_IMG_filter_cubic")]
pub mod img_filter_cubic;
#[cfg(feature = "VK_IMG_format_pvrtc")]
pub mod img_format_pvrtc;
#[cfg(feature = "VK_INTEL_performance_query")]
pub mod intel_performance_query;
#[cfg(feature = "VK_INTEL_shader_integer_functions2")]
pub mod intel_shader_integer_functions_2;
#[cfg(feature = "VK_KHR_16bit_storage")]
pub mod khr_16_bit_storage;
#[cfg(feature = "VK_KHR_8bit_storage")]
pub mod khr_8_bit_storage;
#[cfg(feature = "VK_KHR_acceleration_structure")]
pub mod khr_acceleration_structure;
#[cfg(feature = "VK_KHR_android_surface")]
pub mod khr_android_surface;
#[cfg(feature = "VK_KHR_bind_memory2")]
pub mod khr_bind_memory_2;
#[cfg(feature = "VK_KHR_buffer_device_address")]
pub mod khr_buffer_device_address;
#[cfg(feature = "VK_KHR_copy_commands2")]
pub mod khr_copy_commands_2;
#[cfg(feature = "VK_KHR_create_renderpass2")]
pub mod khr_create_renderpass_2;
#[cfg(feature = "VK_KHR_dedicated_allocation")]
pub mod khr_dedicated_allocation;
#[cfg(feature = "VK_KHR_deferred_host_operations")]
pub mod khr_deferred_host_operations;
#[cfg(feature = "VK_KHR_depth_stencil_resolve")]
pub mod khr_depth_stencil_resolve;
#[cfg(feature = "VK_KHR_descriptor_update_template")]
pub mod khr_descriptor_update_template;
#[cfg(feature = "VK_KHR_device_group")]
pub mod khr_device_group;
#[cfg(feature = "VK_KHR_device_group_creation")]
pub mod khr_device_group_creation;
#[cfg(feature = "VK_KHR_display")]
pub mod khr_display;
#[cfg(feature = "VK_KHR_display_swapchain")]
pub mod khr_display_swapchain;
#[cfg(feature = "VK_KHR_draw_indirect_count")]
pub mod khr_draw_indirect_count;
#[cfg(feature = "VK_KHR_driver_properties")]
pub mod khr_driver_properties;
#[cfg(feature = "VK_KHR_dynamic_rendering")]
pub mod khr_dynamic_rendering;
#[cfg(feature = "VK_KHR_external_fence")]
pub mod khr_external_fence;
#[cfg(feature = "VK_KHR_external_fence_capabilities")]
pub mod khr_external_fence_capabilities;
#[cfg(feature = "VK_KHR_external_fence_fd")]
pub mod khr_external_fence_fd;
#[cfg(feature = "VK_KHR_external_fence_win32")]
pub mod khr_external_fence_win_32;
#[cfg(feature = "VK_KHR_external_memory")]
pub mod khr_external_memory;
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
pub mod khr_external_memory_capabilities;
#[cfg(feature = "VK_KHR_external_memory_fd")]
pub mod khr_external_memory_fd;
#[cfg(feature = "VK_KHR_external_memory_win32")]
pub mod khr_external_memory_win_32;
#[cfg(feature = "VK_KHR_external_semaphore")]
pub mod khr_external_semaphore;
#[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
pub mod khr_external_semaphore_capabilities;
#[cfg(feature = "VK_KHR_external_semaphore_fd")]
pub mod khr_external_semaphore_fd;
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
pub mod khr_external_semaphore_win_32;
#[cfg(feature = "VK_KHR_format_feature_flags2")]
pub mod khr_format_feature_flags_2;
#[cfg(feature = "VK_KHR_fragment_shading_rate")]
pub mod khr_fragment_shading_rate;
#[cfg(feature = "VK_KHR_get_display_properties2")]
pub mod khr_get_display_properties_2;
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
pub mod khr_get_memory_requirements_2;
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
pub mod khr_get_physical_device_properties_2;
#[cfg(feature = "VK_KHR_get_surface_capabilities2")]
pub mod khr_get_surface_capabilities_2;
#[cfg(feature = "VK_KHR_global_priority")]
pub mod khr_global_priority;
#[cfg(feature = "VK_KHR_image_format_list")]
pub mod khr_image_format_list;
#[cfg(feature = "VK_KHR_imageless_framebuffer")]
pub mod khr_imageless_framebuffer;
#[cfg(feature = "VK_KHR_incremental_present")]
pub mod khr_incremental_present;
#[cfg(feature = "VK_KHR_maintenance1")]
pub mod khr_maintenance_1;
#[cfg(feature = "VK_KHR_maintenance2")]
pub mod khr_maintenance_2;
#[cfg(feature = "VK_KHR_maintenance3")]
pub mod khr_maintenance_3;
#[cfg(feature = "VK_KHR_maintenance4")]
pub mod khr_maintenance_4;
#[cfg(feature = "VK_KHR_multiview")]
pub mod khr_multiview;
#[cfg(feature = "VK_KHR_performance_query")]
pub mod khr_performance_query;
#[cfg(feature = "VK_KHR_pipeline_executable_properties")]
pub mod khr_pipeline_executable_properties;
#[cfg(feature = "VK_KHR_pipeline_library")]
pub mod khr_pipeline_library;
#[cfg(feature = "VK_KHR_portability_enumeration")]
pub mod khr_portability_enumeration;
#[cfg(feature = "VK_KHR_portability_subset")]
pub mod khr_portability_subset;
#[cfg(feature = "VK_KHR_present_id")]
pub mod khr_present_id;
#[cfg(feature = "VK_KHR_present_wait")]
pub mod khr_present_wait;
#[cfg(feature = "VK_KHR_push_descriptor")]
pub mod khr_push_descriptor;
#[cfg(feature = "VK_KHR_ray_query")]
pub mod khr_ray_query;
#[cfg(feature = "VK_KHR_ray_tracing_pipeline")]
pub mod khr_ray_tracing_pipeline;
#[cfg(feature = "VK_KHR_relaxed_block_layout")]
pub mod khr_relaxed_block_layout;
#[cfg(feature = "VK_KHR_sampler_mirror_clamp_to_edge")]
pub mod khr_sampler_mirror_clamp_to_edge;
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
pub mod khr_sampler_ycbcr_conversion;
#[cfg(feature = "VK_KHR_separate_depth_stencil_layouts")]
pub mod khr_separate_depth_stencil_layouts;
#[cfg(feature = "VK_KHR_shader_atomic_int64")]
pub mod khr_shader_atomic_int_64;
#[cfg(feature = "VK_KHR_shader_clock")]
pub mod khr_shader_clock;
#[cfg(feature = "VK_KHR_shader_draw_parameters")]
pub mod khr_shader_draw_parameters;
#[cfg(feature = "VK_KHR_shader_float16_int8")]
pub mod khr_shader_float_16_int_8;
#[cfg(feature = "VK_KHR_shader_float_controls")]
pub mod khr_shader_float_controls;
#[cfg(feature = "VK_KHR_shader_integer_dot_product")]
pub mod khr_shader_integer_dot_product;
#[cfg(feature = "VK_KHR_shader_non_semantic_info")]
pub mod khr_shader_non_semantic_info;
#[cfg(feature = "VK_KHR_shader_subgroup_extended_types")]
pub mod khr_shader_subgroup_extended_types;
#[cfg(feature = "VK_KHR_shader_subgroup_uniform_control_flow")]
pub mod khr_shader_subgroup_uniform_control_flow;
#[cfg(feature = "VK_KHR_shader_terminate_invocation")]
pub mod khr_shader_terminate_invocation;
#[cfg(feature = "VK_KHR_shared_presentable_image")]
pub mod khr_shared_presentable_image;
#[cfg(feature = "VK_KHR_spirv_1_4")]
pub mod khr_spirv_1_4;
#[cfg(feature = "VK_KHR_storage_buffer_storage_class")]
pub mod khr_storage_buffer_storage_class;
#[cfg(feature = "VK_KHR_surface")]
pub mod khr_surface;
#[cfg(feature = "VK_KHR_surface_protected_capabilities")]
pub mod khr_surface_protected_capabilities;
#[cfg(feature = "VK_KHR_swapchain")]
pub mod khr_swapchain;
#[cfg(feature = "VK_KHR_swapchain_mutable_format")]
pub mod khr_swapchain_mutable_format;
#[cfg(feature = "VK_KHR_synchronization2")]
pub mod khr_synchronization_2;
#[cfg(feature = "VK_KHR_timeline_semaphore")]
pub mod khr_timeline_semaphore;
#[cfg(feature = "VK_KHR_uniform_buffer_standard_layout")]
pub mod khr_uniform_buffer_standard_layout;
#[cfg(feature = "VK_KHR_variable_pointers")]
pub mod khr_variable_pointers;
#[cfg(feature = "VK_KHR_video_decode_queue")]
pub mod khr_video_decode_queue;
#[cfg(feature = "VK_KHR_video_encode_queue")]
pub mod khr_video_encode_queue;
#[cfg(feature = "VK_KHR_video_queue")]
pub mod khr_video_queue;
#[cfg(feature = "VK_KHR_vulkan_memory_model")]
pub mod khr_vulkan_memory_model;
#[cfg(feature = "VK_KHR_wayland_surface")]
pub mod khr_wayland_surface;
#[cfg(feature = "VK_KHR_win32_keyed_mutex")]
pub mod khr_win_32_keyed_mutex;
#[cfg(feature = "VK_KHR_win32_surface")]
pub mod khr_win_32_surface;
#[cfg(feature = "VK_KHR_workgroup_memory_explicit_layout")]
pub mod khr_workgroup_memory_explicit_layout;
#[cfg(feature = "VK_KHR_xcb_surface")]
pub mod khr_xcb_surface;
#[cfg(feature = "VK_KHR_xlib_surface")]
pub mod khr_xlib_surface;
#[cfg(feature = "VK_KHR_zero_initialize_workgroup_memory")]
pub mod khr_zero_initialize_workgroup_memory;
#[cfg(feature = "VK_MVK_ios_surface")]
pub mod mvk_ios_surface;
#[cfg(feature = "VK_MVK_macos_surface")]
pub mod mvk_macos_surface;
#[cfg(feature = "VK_NN_vi_surface")]
pub mod nn_vi_surface;
#[cfg(feature = "VK_NV_acquire_winrt_display")]
pub mod nv_acquire_winrt_display;
#[cfg(feature = "VK_NV_clip_space_w_scaling")]
pub mod nv_clip_space_w_scaling;
#[cfg(feature = "VK_NV_compute_shader_derivatives")]
pub mod nv_compute_shader_derivatives;
#[cfg(feature = "VK_NV_cooperative_matrix")]
pub mod nv_cooperative_matrix;
#[cfg(feature = "VK_NV_corner_sampled_image")]
pub mod nv_corner_sampled_image;
#[cfg(feature = "VK_NV_coverage_reduction_mode")]
pub mod nv_coverage_reduction_mode;
#[cfg(feature = "VK_NV_dedicated_allocation")]
pub mod nv_dedicated_allocation;
#[cfg(feature = "VK_NV_dedicated_allocation_image_aliasing")]
pub mod nv_dedicated_allocation_image_aliasing;
#[cfg(feature = "VK_NV_device_diagnostic_checkpoints")]
pub mod nv_device_diagnostic_checkpoints;
#[cfg(feature = "VK_NV_device_diagnostics_config")]
pub mod nv_device_diagnostics_config;
#[cfg(feature = "VK_NV_device_generated_commands")]
pub mod nv_device_generated_commands;
#[cfg(feature = "VK_NV_external_memory")]
pub mod nv_external_memory;
#[cfg(feature = "VK_NV_external_memory_capabilities")]
pub mod nv_external_memory_capabilities;
#[cfg(feature = "VK_NV_external_memory_rdma")]
pub mod nv_external_memory_rdma;
#[cfg(feature = "VK_NV_external_memory_win32")]
pub mod nv_external_memory_win_32;
#[cfg(feature = "VK_NV_fill_rectangle")]
pub mod nv_fill_rectangle;
#[cfg(feature = "VK_NV_fragment_coverage_to_color")]
pub mod nv_fragment_coverage_to_color;
#[cfg(feature = "VK_NV_fragment_shader_barycentric")]
pub mod nv_fragment_shader_barycentric;
#[cfg(feature = "VK_NV_fragment_shading_rate_enums")]
pub mod nv_fragment_shading_rate_enums;
#[cfg(feature = "VK_NV_framebuffer_mixed_samples")]
pub mod nv_framebuffer_mixed_samples;
#[cfg(feature = "VK_NV_geometry_shader_passthrough")]
pub mod nv_geometry_shader_passthrough;
#[cfg(feature = "VK_NV_glsl_shader")]
pub mod nv_glsl_shader;
#[cfg(feature = "VK_NV_inherited_viewport_scissor")]
pub mod nv_inherited_viewport_scissor;
#[cfg(feature = "VK_NV_linear_color_attachment")]
pub mod nv_linear_color_attachment;
#[cfg(feature = "VK_NV_mesh_shader")]
pub mod nv_mesh_shader;
#[cfg(feature = "VK_NV_ray_tracing")]
pub mod nv_ray_tracing;
#[cfg(feature = "VK_NV_ray_tracing_motion_blur")]
pub mod nv_ray_tracing_motion_blur;
#[cfg(feature = "VK_NV_representative_fragment_test")]
pub mod nv_representative_fragment_test;
#[cfg(feature = "VK_NV_sample_mask_override_coverage")]
pub mod nv_sample_mask_override_coverage;
#[cfg(feature = "VK_NV_scissor_exclusive")]
pub mod nv_scissor_exclusive;
#[cfg(feature = "VK_NV_shader_image_footprint")]
pub mod nv_shader_image_footprint;
#[cfg(feature = "VK_NV_shader_sm_builtins")]
pub mod nv_shader_sm_builtins;
#[cfg(feature = "VK_NV_shader_subgroup_partitioned")]
pub mod nv_shader_subgroup_partitioned;
#[cfg(feature = "VK_NV_shading_rate_image")]
pub mod nv_shading_rate_image;
#[cfg(feature = "VK_NV_viewport_array2")]
pub mod nv_viewport_array_2;
#[cfg(feature = "VK_NV_viewport_swizzle")]
pub mod nv_viewport_swizzle;
#[cfg(feature = "VK_NV_win32_keyed_mutex")]
pub mod nv_win_32_keyed_mutex;
#[cfg(feature = "VK_NVX_binary_import")]
pub mod nvx_binary_import;
#[cfg(feature = "VK_NVX_image_view_handle")]
pub mod nvx_image_view_handle;
#[cfg(feature = "VK_NVX_multiview_per_view_attributes")]
pub mod nvx_multiview_per_view_attributes;
#[cfg(feature = "VK_QCOM_fragment_density_map_offset")]
pub mod qcom_fragment_density_map_offset;
#[cfg(feature = "VK_QCOM_render_pass_shader_resolve")]
pub mod qcom_render_pass_shader_resolve;
#[cfg(feature = "VK_QCOM_render_pass_store_ops")]
pub mod qcom_render_pass_store_ops;
#[cfg(feature = "VK_QCOM_render_pass_transform")]
pub mod qcom_render_pass_transform;
#[cfg(feature = "VK_QCOM_rotated_copy_commands")]
pub mod qcom_rotated_copy_commands;
#[cfg(feature = "VK_QNX_screen_surface")]
pub mod qnx_screen_surface;
#[cfg(feature = "VK_VALVE_descriptor_set_host_mapping")]
pub mod valve_descriptor_set_host_mapping;
#[cfg(feature = "VK_VALVE_mutable_descriptor_type")]
pub mod valve_mutable_descriptor_type;
use crate::Version;
///A list of Vulkan extensions
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Extensions {
    version: Version,
    #[cfg(feature = "VK_AMD_negative_viewport_height")]
    pub amd_negative_viewport_height: bool,
    #[cfg(feature = "VK_EXT_image_view_min_lod")]
    pub ext_image_view_min_lod: bool,
    #[cfg(feature = "VK_MVK_ios_surface")]
    pub mvk_ios_surface: bool,
    #[cfg(feature = "VK_KHR_external_memory")]
    pub khr_external_memory: bool,
    #[cfg(feature = "VK_KHR_bind_memory2")]
    pub khr_bind_memory_2: bool,
    #[cfg(feature = "VK_KHR_present_id")]
    pub khr_present_id: bool,
    #[cfg(feature = "VK_KHR_xcb_surface")]
    pub khr_xcb_surface: bool,
    #[cfg(feature = "VK_KHR_pipeline_library")]
    pub khr_pipeline_library: bool,
    #[cfg(feature = "VK_KHR_get_display_properties2")]
    pub khr_get_display_properties_2: bool,
    #[cfg(feature = "VK_AMD_shader_image_load_store_lod")]
    pub amd_shader_image_load_store_lod: bool,
    #[cfg(feature = "VK_NN_vi_surface")]
    pub nn_vi_surface: bool,
    #[cfg(feature = "VK_KHR_video_queue")]
    pub khr_video_queue: bool,
    #[cfg(feature = "VK_EXT_tooling_info")]
    pub ext_tooling_info: bool,
    #[cfg(feature = "VK_KHR_swapchain")]
    pub khr_swapchain: bool,
    #[cfg(feature = "VK_EXT_shader_subgroup_ballot")]
    pub ext_shader_subgroup_ballot: bool,
    #[cfg(feature = "VK_KHR_push_descriptor")]
    pub khr_push_descriptor: bool,
    #[cfg(feature = "VK_KHR_incremental_present")]
    pub khr_incremental_present: bool,
    #[cfg(feature = "VK_EXT_external_memory_host")]
    pub ext_external_memory_host: bool,
    #[cfg(feature = "VK_NV_device_generated_commands")]
    pub nv_device_generated_commands: bool,
    #[cfg(feature = "VK_EXT_robustness2")]
    pub ext_robustness_2: bool,
    #[cfg(feature = "VK_NV_fill_rectangle")]
    pub nv_fill_rectangle: bool,
    #[cfg(feature = "VK_EXT_metal_surface")]
    pub ext_metal_surface: bool,
    #[cfg(feature = "VK_MVK_macos_surface")]
    pub mvk_macos_surface: bool,
    #[cfg(feature = "VK_KHR_maintenance3")]
    pub khr_maintenance_3: bool,
    #[cfg(feature = "VK_KHR_android_surface")]
    pub khr_android_surface: bool,
    #[cfg(feature = "VK_EXT_pci_bus_info")]
    pub ext_pci_bus_info: bool,
    #[cfg(feature = "VK_NV_external_memory_capabilities")]
    pub nv_external_memory_capabilities: bool,
    #[cfg(feature = "VK_NV_external_memory_win32")]
    pub nv_external_memory_win_32: bool,
    #[cfg(feature = "VK_EXT_sampler_filter_minmax")]
    pub ext_sampler_filter_minmax: bool,
    #[cfg(feature = "VK_AMD_display_native_hdr")]
    pub amd_display_native_hdr: bool,
    #[cfg(feature = "VK_EXT_fragment_density_map")]
    pub ext_fragment_density_map: bool,
    #[cfg(feature = "VK_NV_cooperative_matrix")]
    pub nv_cooperative_matrix: bool,
    #[cfg(feature = "VK_EXT_multi_draw")]
    pub ext_multi_draw: bool,
    #[cfg(feature = "VK_KHR_vulkan_memory_model")]
    pub khr_vulkan_memory_model: bool,
    #[cfg(feature = "VK_NV_linear_color_attachment")]
    pub nv_linear_color_attachment: bool,
    #[cfg(feature = "VK_AMD_shader_info")]
    pub amd_shader_info: bool,
    #[cfg(feature = "VK_EXT_global_priority_query")]
    pub ext_global_priority_query: bool,
    #[cfg(feature = "VK_KHR_external_semaphore_win32")]
    pub khr_external_semaphore_win_32: bool,
    #[cfg(feature = "VK_NV_dedicated_allocation_image_aliasing")]
    pub nv_dedicated_allocation_image_aliasing: bool,
    #[cfg(feature = "VK_KHR_shared_presentable_image")]
    pub khr_shared_presentable_image: bool,
    #[cfg(feature = "VK_KHR_external_fence_capabilities")]
    pub khr_external_fence_capabilities: bool,
    #[cfg(feature = "VK_NV_sample_mask_override_coverage")]
    pub nv_sample_mask_override_coverage: bool,
    #[cfg(feature = "VK_FUCHSIA_buffer_collection")]
    pub fuchsia_buffer_collection: bool,
    #[cfg(feature = "VK_NV_shader_image_footprint")]
    pub nv_shader_image_footprint: bool,
    #[cfg(feature = "VK_EXT_discard_rectangles")]
    pub ext_discard_rectangles: bool,
    #[cfg(feature = "VK_EXT_shader_atomic_float")]
    pub ext_shader_atomic_float: bool,
    #[cfg(feature = "VK_EXT_debug_report")]
    pub ext_debug_report: bool,
    #[cfg(feature = "VK_KHR_win32_keyed_mutex")]
    pub khr_win_32_keyed_mutex: bool,
    #[cfg(feature = "VK_NVX_multiview_per_view_attributes")]
    pub nvx_multiview_per_view_attributes: bool,
    #[cfg(feature = "VK_KHR_external_memory_fd")]
    pub khr_external_memory_fd: bool,
    #[cfg(feature = "VK_EXT_scalar_block_layout")]
    pub ext_scalar_block_layout: bool,
    #[cfg(feature = "VK_AMD_rasterization_order")]
    pub amd_rasterization_order: bool,
    #[cfg(feature = "VK_EXT_image_robustness")]
    pub ext_image_robustness: bool,
    #[cfg(feature = "VK_NVX_image_view_handle")]
    pub nvx_image_view_handle: bool,
    #[cfg(feature = "VK_NV_fragment_shader_barycentric")]
    pub nv_fragment_shader_barycentric: bool,
    #[cfg(feature = "VK_EXT_depth_clip_enable")]
    pub ext_depth_clip_enable: bool,
    #[cfg(feature = "VK_VALVE_descriptor_set_host_mapping")]
    pub valve_descriptor_set_host_mapping: bool,
    #[cfg(feature = "VK_EXT_custom_border_color")]
    pub ext_custom_border_color: bool,
    #[cfg(feature = "VK_KHR_global_priority")]
    pub khr_global_priority: bool,
    #[cfg(feature = "VK_NV_dedicated_allocation")]
    pub nv_dedicated_allocation: bool,
    #[cfg(feature = "VK_EXT_conditional_rendering")]
    pub ext_conditional_rendering: bool,
    #[cfg(feature = "VK_EXT_pageable_device_local_memory")]
    pub ext_pageable_device_local_memory: bool,
    #[cfg(feature = "VK_KHR_create_renderpass2")]
    pub khr_create_renderpass_2: bool,
    #[cfg(feature = "VK_EXT_validation_features")]
    pub ext_validation_features: bool,
    #[cfg(feature = "VK_QCOM_render_pass_transform")]
    pub qcom_render_pass_transform: bool,
    #[cfg(feature = "VK_GGP_stream_descriptor_surface")]
    pub ggp_stream_descriptor_surface: bool,
    #[cfg(feature = "VK_EXT_depth_clip_control")]
    pub ext_depth_clip_control: bool,
    #[cfg(feature = "VK_KHR_present_wait")]
    pub khr_present_wait: bool,
    #[cfg(feature = "VK_KHR_maintenance2")]
    pub khr_maintenance_2: bool,
    #[cfg(feature = "VK_KHR_relaxed_block_layout")]
    pub khr_relaxed_block_layout: bool,
    #[cfg(feature = "VK_KHR_external_memory_win32")]
    pub khr_external_memory_win_32: bool,
    #[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
    pub khr_external_semaphore_capabilities: bool,
    #[cfg(feature = "VK_EXT_subgroup_size_control")]
    pub ext_subgroup_size_control: bool,
    #[cfg(feature = "VK_EXT_shader_demote_to_helper_invocation")]
    pub ext_shader_demote_to_helper_invocation: bool,
    #[cfg(feature = "VK_KHR_external_fence")]
    pub khr_external_fence: bool,
    #[cfg(feature = "VK_VALVE_mutable_descriptor_type")]
    pub valve_mutable_descriptor_type: bool,
    #[cfg(feature = "VK_EXT_debug_utils")]
    pub ext_debug_utils: bool,
    #[cfg(feature = "VK_NV_corner_sampled_image")]
    pub nv_corner_sampled_image: bool,
    #[cfg(feature = "VK_EXT_validation_cache")]
    pub ext_validation_cache: bool,
    #[cfg(feature = "VK_KHR_depth_stencil_resolve")]
    pub khr_depth_stencil_resolve: bool,
    #[cfg(feature = "VK_NV_shader_sm_builtins")]
    pub nv_shader_sm_builtins: bool,
    #[cfg(feature = "VK_EXT_provoking_vertex")]
    pub ext_provoking_vertex: bool,
    #[cfg(feature = "VK_KHR_performance_query")]
    pub khr_performance_query: bool,
    #[cfg(feature = "VK_KHR_imageless_framebuffer")]
    pub khr_imageless_framebuffer: bool,
    #[cfg(feature = "VK_KHR_descriptor_update_template")]
    pub khr_descriptor_update_template: bool,
    #[cfg(feature = "VK_NV_clip_space_w_scaling")]
    pub nv_clip_space_w_scaling: bool,
    #[cfg(feature = "VK_KHR_storage_buffer_storage_class")]
    pub khr_storage_buffer_storage_class: bool,
    #[cfg(feature = "VK_KHR_get_memory_requirements2")]
    pub khr_get_memory_requirements_2: bool,
    #[cfg(feature = "VK_KHR_external_fence_fd")]
    pub khr_external_fence_fd: bool,
    #[cfg(feature = "VK_EXT_shader_viewport_index_layer")]
    pub ext_shader_viewport_index_layer: bool,
    #[cfg(feature = "VK_NV_representative_fragment_test")]
    pub nv_representative_fragment_test: bool,
    #[cfg(feature = "VK_KHR_wayland_surface")]
    pub khr_wayland_surface: bool,
    #[cfg(feature = "VK_KHR_shader_terminate_invocation")]
    pub khr_shader_terminate_invocation: bool,
    #[cfg(feature = "VK_NV_viewport_swizzle")]
    pub nv_viewport_swizzle: bool,
    #[cfg(feature = "VK_KHR_fragment_shading_rate")]
    pub khr_fragment_shading_rate: bool,
    #[cfg(feature = "VK_EXT_memory_budget")]
    pub ext_memory_budget: bool,
    #[cfg(feature = "VK_EXT_device_memory_report")]
    pub ext_device_memory_report: bool,
    #[cfg(feature = "VK_EXT_direct_mode_display")]
    pub ext_direct_mode_display: bool,
    #[cfg(feature = "VK_EXT_external_memory_dma_buf")]
    pub ext_external_memory_dma_buf: bool,
    #[cfg(feature = "VK_KHR_copy_commands2")]
    pub khr_copy_commands_2: bool,
    #[cfg(feature = "VK_EXT_video_decode_h265")]
    pub ext_video_decode_h_265: bool,
    #[cfg(feature = "VK_KHR_portability_enumeration")]
    pub khr_portability_enumeration: bool,
    #[cfg(feature = "VK_EXT_line_rasterization")]
    pub ext_line_rasterization: bool,
    #[cfg(feature = "VK_NV_scissor_exclusive")]
    pub nv_scissor_exclusive: bool,
    #[cfg(feature = "VK_EXT_queue_family_foreign")]
    pub ext_queue_family_foreign: bool,
    #[cfg(feature = "VK_EXT_video_encode_h265")]
    pub ext_video_encode_h_265: bool,
    #[cfg(feature = "VK_KHR_portability_subset")]
    pub khr_portability_subset: bool,
    #[cfg(feature = "VK_AMD_gpu_shader_int16")]
    pub amd_gpu_shader_int_16: bool,
    #[cfg(feature = "VK_KHR_zero_initialize_workgroup_memory")]
    pub khr_zero_initialize_workgroup_memory: bool,
    #[cfg(feature = "VK_INTEL_performance_query")]
    pub intel_performance_query: bool,
    #[cfg(feature = "VK_EXT_video_encode_h264")]
    pub ext_video_encode_h_264: bool,
    #[cfg(feature = "VK_AMD_shader_fragment_mask")]
    pub amd_shader_fragment_mask: bool,
    #[cfg(feature = "VK_AMD_shader_core_properties")]
    pub amd_shader_core_properties: bool,
    #[cfg(feature = "VK_KHR_ray_query")]
    pub khr_ray_query: bool,
    #[cfg(feature = "VK_NV_framebuffer_mixed_samples")]
    pub nv_framebuffer_mixed_samples: bool,
    #[cfg(feature = "VK_KHR_format_feature_flags2")]
    pub khr_format_feature_flags_2: bool,
    #[cfg(feature = "VK_EXT_pipeline_creation_cache_control")]
    pub ext_pipeline_creation_cache_control: bool,
    #[cfg(feature = "VK_HUAWEI_invocation_mask")]
    pub huawei_invocation_mask: bool,
    #[cfg(feature = "VK_KHR_timeline_semaphore")]
    pub khr_timeline_semaphore: bool,
    #[cfg(feature = "VK_EXT_global_priority")]
    pub ext_global_priority: bool,
    #[cfg(feature = "VK_KHR_pipeline_executable_properties")]
    pub khr_pipeline_executable_properties: bool,
    #[cfg(feature = "VK_KHR_external_memory_capabilities")]
    pub khr_external_memory_capabilities: bool,
    #[cfg(feature = "VK_INTEL_shader_integer_functions2")]
    pub intel_shader_integer_functions_2: bool,
    #[cfg(feature = "VK_HUAWEI_subpass_shading")]
    pub huawei_subpass_shading: bool,
    #[cfg(feature = "VK_NV_acquire_winrt_display")]
    pub nv_acquire_winrt_display: bool,
    #[cfg(feature = "VK_KHR_surface")]
    pub khr_surface: bool,
    #[cfg(feature = "VK_FUCHSIA_external_semaphore")]
    pub fuchsia_external_semaphore: bool,
    #[cfg(feature = "VK_KHR_acceleration_structure")]
    pub khr_acceleration_structure: bool,
    #[cfg(feature = "VK_KHR_win32_surface")]
    pub khr_win_32_surface: bool,
    #[cfg(feature = "VK_AMD_shader_trinary_minmax")]
    pub amd_shader_trinary_minmax: bool,
    #[cfg(feature = "VK_EXT_separate_stencil_usage")]
    pub ext_separate_stencil_usage: bool,
    #[cfg(feature = "VK_NV_ray_tracing")]
    pub nv_ray_tracing: bool,
    #[cfg(feature = "VK_EXT_post_depth_coverage")]
    pub ext_post_depth_coverage: bool,
    #[cfg(feature = "VK_KHR_16bit_storage")]
    pub khr_16_bit_storage: bool,
    #[cfg(feature = "VK_EXT_vertex_attribute_divisor")]
    pub ext_vertex_attribute_divisor: bool,
    #[cfg(feature = "VK_KHR_shader_integer_dot_product")]
    pub khr_shader_integer_dot_product: bool,
    #[cfg(feature = "VK_NV_fragment_shading_rate_enums")]
    pub nv_fragment_shading_rate_enums: bool,
    #[cfg(feature = "VK_KHR_image_format_list")]
    pub khr_image_format_list: bool,
    #[cfg(feature = "VK_KHR_maintenance4")]
    pub khr_maintenance_4: bool,
    #[cfg(feature = "VK_EXT_display_control")]
    pub ext_display_control: bool,
    #[cfg(feature = "VK_EXT_video_decode_h264")]
    pub ext_video_decode_h_264: bool,
    #[cfg(feature = "VK_EXT_acquire_xlib_display")]
    pub ext_acquire_xlib_display: bool,
    #[cfg(feature = "VK_EXT_full_screen_exclusive")]
    pub ext_full_screen_exclusive: bool,
    #[cfg(feature = "VK_GOOGLE_surfaceless_query")]
    pub google_surfaceless_query: bool,
    #[cfg(feature = "VK_AMD_shader_ballot")]
    pub amd_shader_ballot: bool,
    #[cfg(feature = "VK_KHR_shader_float_controls")]
    pub khr_shader_float_controls: bool,
    #[cfg(feature = "VK_EXT_acquire_drm_display")]
    pub ext_acquire_drm_display: bool,
    #[cfg(feature = "VK_EXT_sample_locations")]
    pub ext_sample_locations: bool,
    #[cfg(feature = "VK_KHR_variable_pointers")]
    pub khr_variable_pointers: bool,
    #[cfg(feature = "VK_EXT_extended_dynamic_state2")]
    pub ext_extended_dynamic_state_2: bool,
    #[cfg(feature = "VK_EXT_shader_atomic_float2")]
    pub ext_shader_atomic_float_2: bool,
    #[cfg(feature = "VK_QNX_screen_surface")]
    pub qnx_screen_surface: bool,
    #[cfg(feature = "VK_FUCHSIA_imagepipe_surface")]
    pub fuchsia_imagepipe_surface: bool,
    #[cfg(feature = "VK_KHR_separate_depth_stencil_layouts")]
    pub khr_separate_depth_stencil_layouts: bool,
    #[cfg(feature = "VK_KHR_device_group_creation")]
    pub khr_device_group_creation: bool,
    #[cfg(feature = "VK_NV_fragment_coverage_to_color")]
    pub nv_fragment_coverage_to_color: bool,
    #[cfg(feature = "VK_GOOGLE_decorate_string")]
    pub google_decorate_string: bool,
    #[cfg(feature = "VK_NV_geometry_shader_passthrough")]
    pub nv_geometry_shader_passthrough: bool,
    #[cfg(feature = "VK_NVX_binary_import")]
    pub nvx_binary_import: bool,
    #[cfg(feature = "VK_KHR_shader_subgroup_extended_types")]
    pub khr_shader_subgroup_extended_types: bool,
    #[cfg(feature = "VK_EXT_transform_feedback")]
    pub ext_transform_feedback: bool,
    #[cfg(feature = "VK_EXT_rgba10x6_formats")]
    pub ext_rgba_10_x_6_formats: bool,
    #[cfg(feature = "VK_KHR_draw_indirect_count")]
    pub khr_draw_indirect_count: bool,
    #[cfg(feature = "VK_GOOGLE_user_type")]
    pub google_user_type: bool,
    #[cfg(feature = "VK_KHR_shader_subgroup_uniform_control_flow")]
    pub khr_shader_subgroup_uniform_control_flow: bool,
    #[cfg(feature = "VK_ANDROID_external_memory_android_hardware_buffer")]
    pub android_external_memory_android_hardware_buffer: bool,
    #[cfg(feature = "VK_NV_shading_rate_image")]
    pub nv_shading_rate_image: bool,
    #[cfg(feature = "VK_EXT_astc_decode_mode")]
    pub ext_astc_decode_mode: bool,
    #[cfg(feature = "VK_KHR_get_surface_capabilities2")]
    pub khr_get_surface_capabilities_2: bool,
    #[cfg(feature = "VK_AMD_shader_explicit_vertex_parameter")]
    pub amd_shader_explicit_vertex_parameter: bool,
    #[cfg(feature = "VK_KHR_device_group")]
    pub khr_device_group: bool,
    #[cfg(feature = "VK_KHR_dedicated_allocation")]
    pub khr_dedicated_allocation: bool,
    #[cfg(feature = "VK_IMG_format_pvrtc")]
    pub img_format_pvrtc: bool,
    #[cfg(feature = "VK_KHR_ray_tracing_pipeline")]
    pub khr_ray_tracing_pipeline: bool,
    #[cfg(feature = "VK_QCOM_render_pass_shader_resolve")]
    pub qcom_render_pass_shader_resolve: bool,
    #[cfg(feature = "VK_KHR_workgroup_memory_explicit_layout")]
    pub khr_workgroup_memory_explicit_layout: bool,
    #[cfg(feature = "VK_EXT_vertex_input_dynamic_state")]
    pub ext_vertex_input_dynamic_state: bool,
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    pub khr_sampler_ycbcr_conversion: bool,
    #[cfg(feature = "VK_KHR_shader_atomic_int64")]
    pub khr_shader_atomic_int_64: bool,
    #[cfg(feature = "VK_KHR_display_swapchain")]
    pub khr_display_swapchain: bool,
    #[cfg(feature = "VK_IMG_filter_cubic")]
    pub img_filter_cubic: bool,
    #[cfg(feature = "VK_EXT_index_type_uint8")]
    pub ext_index_type_uint_8: bool,
    #[cfg(feature = "VK_AMD_memory_overallocation_behavior")]
    pub amd_memory_overallocation_behavior: bool,
    #[cfg(feature = "VK_EXT_shader_image_atomic_int64")]
    pub ext_shader_image_atomic_int_64: bool,
    #[cfg(feature = "VK_KHR_multiview")]
    pub khr_multiview: bool,
    #[cfg(feature = "VK_AMD_mixed_attachment_samples")]
    pub amd_mixed_attachment_samples: bool,
    #[cfg(feature = "VK_KHR_uniform_buffer_standard_layout")]
    pub khr_uniform_buffer_standard_layout: bool,
    #[cfg(feature = "VK_EXT_ycbcr_2plane_444_formats")]
    pub ext_ycbcr_2_plane_444_formats: bool,
    #[cfg(feature = "VK_EXT_display_surface_counter")]
    pub ext_display_surface_counter: bool,
    #[cfg(feature = "VK_KHR_shader_float16_int8")]
    pub khr_shader_float_16_int_8: bool,
    #[cfg(feature = "VK_FUCHSIA_external_memory")]
    pub fuchsia_external_memory: bool,
    #[cfg(feature = "VK_KHR_display")]
    pub khr_display: bool,
    #[cfg(feature = "VK_KHR_driver_properties")]
    pub khr_driver_properties: bool,
    #[cfg(feature = "VK_EXT_filter_cubic")]
    pub ext_filter_cubic: bool,
    #[cfg(feature = "VK_KHR_maintenance1")]
    pub khr_maintenance_1: bool,
    #[cfg(feature = "VK_KHR_spirv_1_4")]
    pub khr_spirv_1_4: bool,
    #[cfg(feature = "VK_AMD_texture_gather_bias_lod")]
    pub amd_texture_gather_bias_lod: bool,
    #[cfg(feature = "VK_EXT_extended_dynamic_state")]
    pub ext_extended_dynamic_state: bool,
    #[cfg(feature = "VK_EXT_conservative_rasterization")]
    pub ext_conservative_rasterization: bool,
    #[cfg(feature = "VK_KHR_surface_protected_capabilities")]
    pub khr_surface_protected_capabilities: bool,
    #[cfg(feature = "VK_EXT_primitive_topology_list_restart")]
    pub ext_primitive_topology_list_restart: bool,
    #[cfg(feature = "VK_KHR_8bit_storage")]
    pub khr_8_bit_storage: bool,
    #[cfg(feature = "VK_EXT_border_color_swizzle")]
    pub ext_border_color_swizzle: bool,
    #[cfg(feature = "VK_EXT_calibrated_timestamps")]
    pub ext_calibrated_timestamps: bool,
    #[cfg(feature = "VK_EXT_depth_range_unrestricted")]
    pub ext_depth_range_unrestricted: bool,
    #[cfg(feature = "VK_NV_shader_subgroup_partitioned")]
    pub nv_shader_subgroup_partitioned: bool,
    #[cfg(feature = "VK_GOOGLE_hlsl_functionality1")]
    pub google_hlsl_functionality_1: bool,
    #[cfg(feature = "VK_EXT_memory_priority")]
    pub ext_memory_priority: bool,
    #[cfg(feature = "VK_KHR_external_semaphore_fd")]
    pub khr_external_semaphore_fd: bool,
    #[cfg(feature = "VK_KHR_get_physical_device_properties2")]
    pub khr_get_physical_device_properties_2: bool,
    #[cfg(feature = "VK_EXT_swapchain_colorspace")]
    pub ext_swapchain_colorspace: bool,
    #[cfg(feature = "VK_GOOGLE_display_timing")]
    pub google_display_timing: bool,
    #[cfg(feature = "VK_NV_mesh_shader")]
    pub nv_mesh_shader: bool,
    #[cfg(feature = "VK_KHR_synchronization2")]
    pub khr_synchronization_2: bool,
    #[cfg(feature = "VK_EXT_shader_subgroup_vote")]
    pub ext_shader_subgroup_vote: bool,
    #[cfg(feature = "VK_EXT_fragment_shader_interlock")]
    pub ext_fragment_shader_interlock: bool,
    #[cfg(feature = "VK_EXT_directfb_surface")]
    pub ext_directfb_surface: bool,
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    pub ext_blend_operation_advanced: bool,
    #[cfg(feature = "VK_EXT_descriptor_indexing")]
    pub ext_descriptor_indexing: bool,
    #[cfg(feature = "VK_EXT_inline_uniform_block")]
    pub ext_inline_uniform_block: bool,
    #[cfg(feature = "VK_KHR_video_encode_queue")]
    pub khr_video_encode_queue: bool,
    #[cfg(feature = "VK_EXT_fragment_density_map2")]
    pub ext_fragment_density_map_2: bool,
    #[cfg(feature = "VK_EXT_debug_marker")]
    pub ext_debug_marker: bool,
    #[cfg(feature = "VK_EXT_physical_device_drm")]
    pub ext_physical_device_drm: bool,
    #[cfg(feature = "VK_NV_coverage_reduction_mode")]
    pub nv_coverage_reduction_mode: bool,
    #[cfg(feature = "VK_KHR_shader_non_semantic_info")]
    pub khr_shader_non_semantic_info: bool,
    #[cfg(feature = "VK_ARM_rasterization_order_attachment_access")]
    pub arm_rasterization_order_attachment_access: bool,
    #[cfg(feature = "VK_KHR_shader_draw_parameters")]
    pub khr_shader_draw_parameters: bool,
    #[cfg(feature = "VK_EXT_texture_compression_astc_hdr")]
    pub ext_texture_compression_astc_hdr: bool,
    #[cfg(feature = "VK_AMD_gpu_shader_half_float")]
    pub amd_gpu_shader_half_float: bool,
    #[cfg(feature = "VK_NV_device_diagnostic_checkpoints")]
    pub nv_device_diagnostic_checkpoints: bool,
    #[cfg(feature = "VK_KHR_video_decode_queue")]
    pub khr_video_decode_queue: bool,
    #[cfg(feature = "VK_KHR_deferred_host_operations")]
    pub khr_deferred_host_operations: bool,
    #[cfg(feature = "VK_KHR_sampler_mirror_clamp_to_edge")]
    pub khr_sampler_mirror_clamp_to_edge: bool,
    #[cfg(feature = "VK_KHR_external_semaphore")]
    pub khr_external_semaphore: bool,
    #[cfg(feature = "VK_NV_viewport_array2")]
    pub nv_viewport_array_2: bool,
    #[cfg(feature = "VK_KHR_external_fence_win32")]
    pub khr_external_fence_win_32: bool,
    #[cfg(feature = "VK_EXT_ycbcr_image_arrays")]
    pub ext_ycbcr_image_arrays: bool,
    #[cfg(feature = "VK_KHR_buffer_device_address")]
    pub khr_buffer_device_address: bool,
    #[cfg(feature = "VK_AMD_pipeline_compiler_control")]
    pub amd_pipeline_compiler_control: bool,
    #[cfg(feature = "VK_KHR_swapchain_mutable_format")]
    pub khr_swapchain_mutable_format: bool,
    #[cfg(feature = "VK_QCOM_rotated_copy_commands")]
    pub qcom_rotated_copy_commands: bool,
    #[cfg(feature = "VK_EXT_color_write_enable")]
    pub ext_color_write_enable: bool,
    #[cfg(feature = "VK_EXT_image_drm_format_modifier")]
    pub ext_image_drm_format_modifier: bool,
    #[cfg(feature = "VK_EXT_headless_surface")]
    pub ext_headless_surface: bool,
    #[cfg(feature = "VK_GGP_frame_token")]
    pub ggp_frame_token: bool,
    #[cfg(feature = "VK_NV_external_memory")]
    pub nv_external_memory: bool,
    #[cfg(feature = "VK_EXT_shader_stencil_export")]
    pub ext_shader_stencil_export: bool,
    #[cfg(feature = "VK_KHR_xlib_surface")]
    pub khr_xlib_surface: bool,
    #[cfg(feature = "VK_AMD_gcn_shader")]
    pub amd_gcn_shader: bool,
    #[cfg(feature = "VK_EXT_4444_formats")]
    pub ext_4444_formats: bool,
    #[cfg(feature = "VK_EXT_texel_buffer_alignment")]
    pub ext_texel_buffer_alignment: bool,
    #[cfg(feature = "VK_EXT_validation_flags")]
    pub ext_validation_flags: bool,
    #[cfg(feature = "VK_KHR_shader_clock")]
    pub khr_shader_clock: bool,
    #[cfg(feature = "VK_QCOM_fragment_density_map_offset")]
    pub qcom_fragment_density_map_offset: bool,
    #[cfg(feature = "VK_EXT_hdr_metadata")]
    pub ext_hdr_metadata: bool,
    #[cfg(feature = "VK_EXT_pipeline_creation_feedback")]
    pub ext_pipeline_creation_feedback: bool,
    #[cfg(feature = "VK_NV_inherited_viewport_scissor")]
    pub nv_inherited_viewport_scissor: bool,
    #[cfg(feature = "VK_AMD_draw_indirect_count")]
    pub amd_draw_indirect_count: bool,
    #[cfg(feature = "VK_NV_device_diagnostics_config")]
    pub nv_device_diagnostics_config: bool,
    #[cfg(feature = "VK_KHR_dynamic_rendering")]
    pub khr_dynamic_rendering: bool,
    #[cfg(feature = "VK_AMD_device_coherent_memory")]
    pub amd_device_coherent_memory: bool,
    #[cfg(feature = "VK_AMD_buffer_marker")]
    pub amd_buffer_marker: bool,
    #[cfg(feature = "VK_NV_compute_shader_derivatives")]
    pub nv_compute_shader_derivatives: bool,
    #[cfg(feature = "VK_NV_external_memory_rdma")]
    pub nv_external_memory_rdma: bool,
    #[cfg(feature = "VK_EXT_private_data")]
    pub ext_private_data: bool,
    #[cfg(feature = "VK_NV_ray_tracing_motion_blur")]
    pub nv_ray_tracing_motion_blur: bool,
    #[cfg(feature = "VK_NV_glsl_shader")]
    pub nv_glsl_shader: bool,
    #[cfg(feature = "VK_EXT_buffer_device_address")]
    pub ext_buffer_device_address: bool,
    #[cfg(feature = "VK_EXT_host_query_reset")]
    pub ext_host_query_reset: bool,
    #[cfg(feature = "VK_EXT_load_store_op_none")]
    pub ext_load_store_op_none: bool,
    #[cfg(feature = "VK_AMD_shader_core_properties2")]
    pub amd_shader_core_properties_2: bool,
    #[cfg(feature = "VK_QCOM_render_pass_store_ops")]
    pub qcom_render_pass_store_ops: bool,
    #[cfg(feature = "VK_NV_win32_keyed_mutex")]
    pub nv_win_32_keyed_mutex: bool,
}
impl const Default for Extensions {
    fn default() -> Self {
        Self {
            version: Version::VULKAN1_0,
            #[cfg(feature = "VK_AMD_negative_viewport_height")]
            amd_negative_viewport_height: false,
            #[cfg(feature = "VK_EXT_image_view_min_lod")]
            ext_image_view_min_lod: false,
            #[cfg(feature = "VK_MVK_ios_surface")]
            mvk_ios_surface: false,
            #[cfg(feature = "VK_KHR_external_memory")]
            khr_external_memory: false,
            #[cfg(feature = "VK_KHR_bind_memory2")]
            khr_bind_memory_2: false,
            #[cfg(feature = "VK_KHR_present_id")]
            khr_present_id: false,
            #[cfg(feature = "VK_KHR_xcb_surface")]
            khr_xcb_surface: false,
            #[cfg(feature = "VK_KHR_pipeline_library")]
            khr_pipeline_library: false,
            #[cfg(feature = "VK_KHR_get_display_properties2")]
            khr_get_display_properties_2: false,
            #[cfg(feature = "VK_AMD_shader_image_load_store_lod")]
            amd_shader_image_load_store_lod: false,
            #[cfg(feature = "VK_NN_vi_surface")]
            nn_vi_surface: false,
            #[cfg(feature = "VK_KHR_video_queue")]
            khr_video_queue: false,
            #[cfg(feature = "VK_EXT_tooling_info")]
            ext_tooling_info: false,
            #[cfg(feature = "VK_KHR_swapchain")]
            khr_swapchain: false,
            #[cfg(feature = "VK_EXT_shader_subgroup_ballot")]
            ext_shader_subgroup_ballot: false,
            #[cfg(feature = "VK_KHR_push_descriptor")]
            khr_push_descriptor: false,
            #[cfg(feature = "VK_KHR_incremental_present")]
            khr_incremental_present: false,
            #[cfg(feature = "VK_EXT_external_memory_host")]
            ext_external_memory_host: false,
            #[cfg(feature = "VK_NV_device_generated_commands")]
            nv_device_generated_commands: false,
            #[cfg(feature = "VK_EXT_robustness2")]
            ext_robustness_2: false,
            #[cfg(feature = "VK_NV_fill_rectangle")]
            nv_fill_rectangle: false,
            #[cfg(feature = "VK_EXT_metal_surface")]
            ext_metal_surface: false,
            #[cfg(feature = "VK_MVK_macos_surface")]
            mvk_macos_surface: false,
            #[cfg(feature = "VK_KHR_maintenance3")]
            khr_maintenance_3: false,
            #[cfg(feature = "VK_KHR_android_surface")]
            khr_android_surface: false,
            #[cfg(feature = "VK_EXT_pci_bus_info")]
            ext_pci_bus_info: false,
            #[cfg(feature = "VK_NV_external_memory_capabilities")]
            nv_external_memory_capabilities: false,
            #[cfg(feature = "VK_NV_external_memory_win32")]
            nv_external_memory_win_32: false,
            #[cfg(feature = "VK_EXT_sampler_filter_minmax")]
            ext_sampler_filter_minmax: false,
            #[cfg(feature = "VK_AMD_display_native_hdr")]
            amd_display_native_hdr: false,
            #[cfg(feature = "VK_EXT_fragment_density_map")]
            ext_fragment_density_map: false,
            #[cfg(feature = "VK_NV_cooperative_matrix")]
            nv_cooperative_matrix: false,
            #[cfg(feature = "VK_EXT_multi_draw")]
            ext_multi_draw: false,
            #[cfg(feature = "VK_KHR_vulkan_memory_model")]
            khr_vulkan_memory_model: false,
            #[cfg(feature = "VK_NV_linear_color_attachment")]
            nv_linear_color_attachment: false,
            #[cfg(feature = "VK_AMD_shader_info")]
            amd_shader_info: false,
            #[cfg(feature = "VK_EXT_global_priority_query")]
            ext_global_priority_query: false,
            #[cfg(feature = "VK_KHR_external_semaphore_win32")]
            khr_external_semaphore_win_32: false,
            #[cfg(feature = "VK_NV_dedicated_allocation_image_aliasing")]
            nv_dedicated_allocation_image_aliasing: false,
            #[cfg(feature = "VK_KHR_shared_presentable_image")]
            khr_shared_presentable_image: false,
            #[cfg(feature = "VK_KHR_external_fence_capabilities")]
            khr_external_fence_capabilities: false,
            #[cfg(feature = "VK_NV_sample_mask_override_coverage")]
            nv_sample_mask_override_coverage: false,
            #[cfg(feature = "VK_FUCHSIA_buffer_collection")]
            fuchsia_buffer_collection: false,
            #[cfg(feature = "VK_NV_shader_image_footprint")]
            nv_shader_image_footprint: false,
            #[cfg(feature = "VK_EXT_discard_rectangles")]
            ext_discard_rectangles: false,
            #[cfg(feature = "VK_EXT_shader_atomic_float")]
            ext_shader_atomic_float: false,
            #[cfg(feature = "VK_EXT_debug_report")]
            ext_debug_report: false,
            #[cfg(feature = "VK_KHR_win32_keyed_mutex")]
            khr_win_32_keyed_mutex: false,
            #[cfg(feature = "VK_NVX_multiview_per_view_attributes")]
            nvx_multiview_per_view_attributes: false,
            #[cfg(feature = "VK_KHR_external_memory_fd")]
            khr_external_memory_fd: false,
            #[cfg(feature = "VK_EXT_scalar_block_layout")]
            ext_scalar_block_layout: false,
            #[cfg(feature = "VK_AMD_rasterization_order")]
            amd_rasterization_order: false,
            #[cfg(feature = "VK_EXT_image_robustness")]
            ext_image_robustness: false,
            #[cfg(feature = "VK_NVX_image_view_handle")]
            nvx_image_view_handle: false,
            #[cfg(feature = "VK_NV_fragment_shader_barycentric")]
            nv_fragment_shader_barycentric: false,
            #[cfg(feature = "VK_EXT_depth_clip_enable")]
            ext_depth_clip_enable: false,
            #[cfg(feature = "VK_VALVE_descriptor_set_host_mapping")]
            valve_descriptor_set_host_mapping: false,
            #[cfg(feature = "VK_EXT_custom_border_color")]
            ext_custom_border_color: false,
            #[cfg(feature = "VK_KHR_global_priority")]
            khr_global_priority: false,
            #[cfg(feature = "VK_NV_dedicated_allocation")]
            nv_dedicated_allocation: false,
            #[cfg(feature = "VK_EXT_conditional_rendering")]
            ext_conditional_rendering: false,
            #[cfg(feature = "VK_EXT_pageable_device_local_memory")]
            ext_pageable_device_local_memory: false,
            #[cfg(feature = "VK_KHR_create_renderpass2")]
            khr_create_renderpass_2: false,
            #[cfg(feature = "VK_EXT_validation_features")]
            ext_validation_features: false,
            #[cfg(feature = "VK_QCOM_render_pass_transform")]
            qcom_render_pass_transform: false,
            #[cfg(feature = "VK_GGP_stream_descriptor_surface")]
            ggp_stream_descriptor_surface: false,
            #[cfg(feature = "VK_EXT_depth_clip_control")]
            ext_depth_clip_control: false,
            #[cfg(feature = "VK_KHR_present_wait")]
            khr_present_wait: false,
            #[cfg(feature = "VK_KHR_maintenance2")]
            khr_maintenance_2: false,
            #[cfg(feature = "VK_KHR_relaxed_block_layout")]
            khr_relaxed_block_layout: false,
            #[cfg(feature = "VK_KHR_external_memory_win32")]
            khr_external_memory_win_32: false,
            #[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
            khr_external_semaphore_capabilities: false,
            #[cfg(feature = "VK_EXT_subgroup_size_control")]
            ext_subgroup_size_control: false,
            #[cfg(feature = "VK_EXT_shader_demote_to_helper_invocation")]
            ext_shader_demote_to_helper_invocation: false,
            #[cfg(feature = "VK_KHR_external_fence")]
            khr_external_fence: false,
            #[cfg(feature = "VK_VALVE_mutable_descriptor_type")]
            valve_mutable_descriptor_type: false,
            #[cfg(feature = "VK_EXT_debug_utils")]
            ext_debug_utils: false,
            #[cfg(feature = "VK_NV_corner_sampled_image")]
            nv_corner_sampled_image: false,
            #[cfg(feature = "VK_EXT_validation_cache")]
            ext_validation_cache: false,
            #[cfg(feature = "VK_KHR_depth_stencil_resolve")]
            khr_depth_stencil_resolve: false,
            #[cfg(feature = "VK_NV_shader_sm_builtins")]
            nv_shader_sm_builtins: false,
            #[cfg(feature = "VK_EXT_provoking_vertex")]
            ext_provoking_vertex: false,
            #[cfg(feature = "VK_KHR_performance_query")]
            khr_performance_query: false,
            #[cfg(feature = "VK_KHR_imageless_framebuffer")]
            khr_imageless_framebuffer: false,
            #[cfg(feature = "VK_KHR_descriptor_update_template")]
            khr_descriptor_update_template: false,
            #[cfg(feature = "VK_NV_clip_space_w_scaling")]
            nv_clip_space_w_scaling: false,
            #[cfg(feature = "VK_KHR_storage_buffer_storage_class")]
            khr_storage_buffer_storage_class: false,
            #[cfg(feature = "VK_KHR_get_memory_requirements2")]
            khr_get_memory_requirements_2: false,
            #[cfg(feature = "VK_KHR_external_fence_fd")]
            khr_external_fence_fd: false,
            #[cfg(feature = "VK_EXT_shader_viewport_index_layer")]
            ext_shader_viewport_index_layer: false,
            #[cfg(feature = "VK_NV_representative_fragment_test")]
            nv_representative_fragment_test: false,
            #[cfg(feature = "VK_KHR_wayland_surface")]
            khr_wayland_surface: false,
            #[cfg(feature = "VK_KHR_shader_terminate_invocation")]
            khr_shader_terminate_invocation: false,
            #[cfg(feature = "VK_NV_viewport_swizzle")]
            nv_viewport_swizzle: false,
            #[cfg(feature = "VK_KHR_fragment_shading_rate")]
            khr_fragment_shading_rate: false,
            #[cfg(feature = "VK_EXT_memory_budget")]
            ext_memory_budget: false,
            #[cfg(feature = "VK_EXT_device_memory_report")]
            ext_device_memory_report: false,
            #[cfg(feature = "VK_EXT_direct_mode_display")]
            ext_direct_mode_display: false,
            #[cfg(feature = "VK_EXT_external_memory_dma_buf")]
            ext_external_memory_dma_buf: false,
            #[cfg(feature = "VK_KHR_copy_commands2")]
            khr_copy_commands_2: false,
            #[cfg(feature = "VK_EXT_video_decode_h265")]
            ext_video_decode_h_265: false,
            #[cfg(feature = "VK_KHR_portability_enumeration")]
            khr_portability_enumeration: false,
            #[cfg(feature = "VK_EXT_line_rasterization")]
            ext_line_rasterization: false,
            #[cfg(feature = "VK_NV_scissor_exclusive")]
            nv_scissor_exclusive: false,
            #[cfg(feature = "VK_EXT_queue_family_foreign")]
            ext_queue_family_foreign: false,
            #[cfg(feature = "VK_EXT_video_encode_h265")]
            ext_video_encode_h_265: false,
            #[cfg(feature = "VK_KHR_portability_subset")]
            khr_portability_subset: false,
            #[cfg(feature = "VK_AMD_gpu_shader_int16")]
            amd_gpu_shader_int_16: false,
            #[cfg(feature = "VK_KHR_zero_initialize_workgroup_memory")]
            khr_zero_initialize_workgroup_memory: false,
            #[cfg(feature = "VK_INTEL_performance_query")]
            intel_performance_query: false,
            #[cfg(feature = "VK_EXT_video_encode_h264")]
            ext_video_encode_h_264: false,
            #[cfg(feature = "VK_AMD_shader_fragment_mask")]
            amd_shader_fragment_mask: false,
            #[cfg(feature = "VK_AMD_shader_core_properties")]
            amd_shader_core_properties: false,
            #[cfg(feature = "VK_KHR_ray_query")]
            khr_ray_query: false,
            #[cfg(feature = "VK_NV_framebuffer_mixed_samples")]
            nv_framebuffer_mixed_samples: false,
            #[cfg(feature = "VK_KHR_format_feature_flags2")]
            khr_format_feature_flags_2: false,
            #[cfg(feature = "VK_EXT_pipeline_creation_cache_control")]
            ext_pipeline_creation_cache_control: false,
            #[cfg(feature = "VK_HUAWEI_invocation_mask")]
            huawei_invocation_mask: false,
            #[cfg(feature = "VK_KHR_timeline_semaphore")]
            khr_timeline_semaphore: false,
            #[cfg(feature = "VK_EXT_global_priority")]
            ext_global_priority: false,
            #[cfg(feature = "VK_KHR_pipeline_executable_properties")]
            khr_pipeline_executable_properties: false,
            #[cfg(feature = "VK_KHR_external_memory_capabilities")]
            khr_external_memory_capabilities: false,
            #[cfg(feature = "VK_INTEL_shader_integer_functions2")]
            intel_shader_integer_functions_2: false,
            #[cfg(feature = "VK_HUAWEI_subpass_shading")]
            huawei_subpass_shading: false,
            #[cfg(feature = "VK_NV_acquire_winrt_display")]
            nv_acquire_winrt_display: false,
            #[cfg(feature = "VK_KHR_surface")]
            khr_surface: false,
            #[cfg(feature = "VK_FUCHSIA_external_semaphore")]
            fuchsia_external_semaphore: false,
            #[cfg(feature = "VK_KHR_acceleration_structure")]
            khr_acceleration_structure: false,
            #[cfg(feature = "VK_KHR_win32_surface")]
            khr_win_32_surface: false,
            #[cfg(feature = "VK_AMD_shader_trinary_minmax")]
            amd_shader_trinary_minmax: false,
            #[cfg(feature = "VK_EXT_separate_stencil_usage")]
            ext_separate_stencil_usage: false,
            #[cfg(feature = "VK_NV_ray_tracing")]
            nv_ray_tracing: false,
            #[cfg(feature = "VK_EXT_post_depth_coverage")]
            ext_post_depth_coverage: false,
            #[cfg(feature = "VK_KHR_16bit_storage")]
            khr_16_bit_storage: false,
            #[cfg(feature = "VK_EXT_vertex_attribute_divisor")]
            ext_vertex_attribute_divisor: false,
            #[cfg(feature = "VK_KHR_shader_integer_dot_product")]
            khr_shader_integer_dot_product: false,
            #[cfg(feature = "VK_NV_fragment_shading_rate_enums")]
            nv_fragment_shading_rate_enums: false,
            #[cfg(feature = "VK_KHR_image_format_list")]
            khr_image_format_list: false,
            #[cfg(feature = "VK_KHR_maintenance4")]
            khr_maintenance_4: false,
            #[cfg(feature = "VK_EXT_display_control")]
            ext_display_control: false,
            #[cfg(feature = "VK_EXT_video_decode_h264")]
            ext_video_decode_h_264: false,
            #[cfg(feature = "VK_EXT_acquire_xlib_display")]
            ext_acquire_xlib_display: false,
            #[cfg(feature = "VK_EXT_full_screen_exclusive")]
            ext_full_screen_exclusive: false,
            #[cfg(feature = "VK_GOOGLE_surfaceless_query")]
            google_surfaceless_query: false,
            #[cfg(feature = "VK_AMD_shader_ballot")]
            amd_shader_ballot: false,
            #[cfg(feature = "VK_KHR_shader_float_controls")]
            khr_shader_float_controls: false,
            #[cfg(feature = "VK_EXT_acquire_drm_display")]
            ext_acquire_drm_display: false,
            #[cfg(feature = "VK_EXT_sample_locations")]
            ext_sample_locations: false,
            #[cfg(feature = "VK_KHR_variable_pointers")]
            khr_variable_pointers: false,
            #[cfg(feature = "VK_EXT_extended_dynamic_state2")]
            ext_extended_dynamic_state_2: false,
            #[cfg(feature = "VK_EXT_shader_atomic_float2")]
            ext_shader_atomic_float_2: false,
            #[cfg(feature = "VK_QNX_screen_surface")]
            qnx_screen_surface: false,
            #[cfg(feature = "VK_FUCHSIA_imagepipe_surface")]
            fuchsia_imagepipe_surface: false,
            #[cfg(feature = "VK_KHR_separate_depth_stencil_layouts")]
            khr_separate_depth_stencil_layouts: false,
            #[cfg(feature = "VK_KHR_device_group_creation")]
            khr_device_group_creation: false,
            #[cfg(feature = "VK_NV_fragment_coverage_to_color")]
            nv_fragment_coverage_to_color: false,
            #[cfg(feature = "VK_GOOGLE_decorate_string")]
            google_decorate_string: false,
            #[cfg(feature = "VK_NV_geometry_shader_passthrough")]
            nv_geometry_shader_passthrough: false,
            #[cfg(feature = "VK_NVX_binary_import")]
            nvx_binary_import: false,
            #[cfg(feature = "VK_KHR_shader_subgroup_extended_types")]
            khr_shader_subgroup_extended_types: false,
            #[cfg(feature = "VK_EXT_transform_feedback")]
            ext_transform_feedback: false,
            #[cfg(feature = "VK_EXT_rgba10x6_formats")]
            ext_rgba_10_x_6_formats: false,
            #[cfg(feature = "VK_KHR_draw_indirect_count")]
            khr_draw_indirect_count: false,
            #[cfg(feature = "VK_GOOGLE_user_type")]
            google_user_type: false,
            #[cfg(feature = "VK_KHR_shader_subgroup_uniform_control_flow")]
            khr_shader_subgroup_uniform_control_flow: false,
            #[cfg(feature = "VK_ANDROID_external_memory_android_hardware_buffer")]
            android_external_memory_android_hardware_buffer: false,
            #[cfg(feature = "VK_NV_shading_rate_image")]
            nv_shading_rate_image: false,
            #[cfg(feature = "VK_EXT_astc_decode_mode")]
            ext_astc_decode_mode: false,
            #[cfg(feature = "VK_KHR_get_surface_capabilities2")]
            khr_get_surface_capabilities_2: false,
            #[cfg(feature = "VK_AMD_shader_explicit_vertex_parameter")]
            amd_shader_explicit_vertex_parameter: false,
            #[cfg(feature = "VK_KHR_device_group")]
            khr_device_group: false,
            #[cfg(feature = "VK_KHR_dedicated_allocation")]
            khr_dedicated_allocation: false,
            #[cfg(feature = "VK_IMG_format_pvrtc")]
            img_format_pvrtc: false,
            #[cfg(feature = "VK_KHR_ray_tracing_pipeline")]
            khr_ray_tracing_pipeline: false,
            #[cfg(feature = "VK_QCOM_render_pass_shader_resolve")]
            qcom_render_pass_shader_resolve: false,
            #[cfg(feature = "VK_KHR_workgroup_memory_explicit_layout")]
            khr_workgroup_memory_explicit_layout: false,
            #[cfg(feature = "VK_EXT_vertex_input_dynamic_state")]
            ext_vertex_input_dynamic_state: false,
            #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
            khr_sampler_ycbcr_conversion: false,
            #[cfg(feature = "VK_KHR_shader_atomic_int64")]
            khr_shader_atomic_int_64: false,
            #[cfg(feature = "VK_KHR_display_swapchain")]
            khr_display_swapchain: false,
            #[cfg(feature = "VK_IMG_filter_cubic")]
            img_filter_cubic: false,
            #[cfg(feature = "VK_EXT_index_type_uint8")]
            ext_index_type_uint_8: false,
            #[cfg(feature = "VK_AMD_memory_overallocation_behavior")]
            amd_memory_overallocation_behavior: false,
            #[cfg(feature = "VK_EXT_shader_image_atomic_int64")]
            ext_shader_image_atomic_int_64: false,
            #[cfg(feature = "VK_KHR_multiview")]
            khr_multiview: false,
            #[cfg(feature = "VK_AMD_mixed_attachment_samples")]
            amd_mixed_attachment_samples: false,
            #[cfg(feature = "VK_KHR_uniform_buffer_standard_layout")]
            khr_uniform_buffer_standard_layout: false,
            #[cfg(feature = "VK_EXT_ycbcr_2plane_444_formats")]
            ext_ycbcr_2_plane_444_formats: false,
            #[cfg(feature = "VK_EXT_display_surface_counter")]
            ext_display_surface_counter: false,
            #[cfg(feature = "VK_KHR_shader_float16_int8")]
            khr_shader_float_16_int_8: false,
            #[cfg(feature = "VK_FUCHSIA_external_memory")]
            fuchsia_external_memory: false,
            #[cfg(feature = "VK_KHR_display")]
            khr_display: false,
            #[cfg(feature = "VK_KHR_driver_properties")]
            khr_driver_properties: false,
            #[cfg(feature = "VK_EXT_filter_cubic")]
            ext_filter_cubic: false,
            #[cfg(feature = "VK_KHR_maintenance1")]
            khr_maintenance_1: false,
            #[cfg(feature = "VK_KHR_spirv_1_4")]
            khr_spirv_1_4: false,
            #[cfg(feature = "VK_AMD_texture_gather_bias_lod")]
            amd_texture_gather_bias_lod: false,
            #[cfg(feature = "VK_EXT_extended_dynamic_state")]
            ext_extended_dynamic_state: false,
            #[cfg(feature = "VK_EXT_conservative_rasterization")]
            ext_conservative_rasterization: false,
            #[cfg(feature = "VK_KHR_surface_protected_capabilities")]
            khr_surface_protected_capabilities: false,
            #[cfg(feature = "VK_EXT_primitive_topology_list_restart")]
            ext_primitive_topology_list_restart: false,
            #[cfg(feature = "VK_KHR_8bit_storage")]
            khr_8_bit_storage: false,
            #[cfg(feature = "VK_EXT_border_color_swizzle")]
            ext_border_color_swizzle: false,
            #[cfg(feature = "VK_EXT_calibrated_timestamps")]
            ext_calibrated_timestamps: false,
            #[cfg(feature = "VK_EXT_depth_range_unrestricted")]
            ext_depth_range_unrestricted: false,
            #[cfg(feature = "VK_NV_shader_subgroup_partitioned")]
            nv_shader_subgroup_partitioned: false,
            #[cfg(feature = "VK_GOOGLE_hlsl_functionality1")]
            google_hlsl_functionality_1: false,
            #[cfg(feature = "VK_EXT_memory_priority")]
            ext_memory_priority: false,
            #[cfg(feature = "VK_KHR_external_semaphore_fd")]
            khr_external_semaphore_fd: false,
            #[cfg(feature = "VK_KHR_get_physical_device_properties2")]
            khr_get_physical_device_properties_2: false,
            #[cfg(feature = "VK_EXT_swapchain_colorspace")]
            ext_swapchain_colorspace: false,
            #[cfg(feature = "VK_GOOGLE_display_timing")]
            google_display_timing: false,
            #[cfg(feature = "VK_NV_mesh_shader")]
            nv_mesh_shader: false,
            #[cfg(feature = "VK_KHR_synchronization2")]
            khr_synchronization_2: false,
            #[cfg(feature = "VK_EXT_shader_subgroup_vote")]
            ext_shader_subgroup_vote: false,
            #[cfg(feature = "VK_EXT_fragment_shader_interlock")]
            ext_fragment_shader_interlock: false,
            #[cfg(feature = "VK_EXT_directfb_surface")]
            ext_directfb_surface: false,
            #[cfg(feature = "VK_EXT_blend_operation_advanced")]
            ext_blend_operation_advanced: false,
            #[cfg(feature = "VK_EXT_descriptor_indexing")]
            ext_descriptor_indexing: false,
            #[cfg(feature = "VK_EXT_inline_uniform_block")]
            ext_inline_uniform_block: false,
            #[cfg(feature = "VK_KHR_video_encode_queue")]
            khr_video_encode_queue: false,
            #[cfg(feature = "VK_EXT_fragment_density_map2")]
            ext_fragment_density_map_2: false,
            #[cfg(feature = "VK_EXT_debug_marker")]
            ext_debug_marker: false,
            #[cfg(feature = "VK_EXT_physical_device_drm")]
            ext_physical_device_drm: false,
            #[cfg(feature = "VK_NV_coverage_reduction_mode")]
            nv_coverage_reduction_mode: false,
            #[cfg(feature = "VK_KHR_shader_non_semantic_info")]
            khr_shader_non_semantic_info: false,
            #[cfg(feature = "VK_ARM_rasterization_order_attachment_access")]
            arm_rasterization_order_attachment_access: false,
            #[cfg(feature = "VK_KHR_shader_draw_parameters")]
            khr_shader_draw_parameters: false,
            #[cfg(feature = "VK_EXT_texture_compression_astc_hdr")]
            ext_texture_compression_astc_hdr: false,
            #[cfg(feature = "VK_AMD_gpu_shader_half_float")]
            amd_gpu_shader_half_float: false,
            #[cfg(feature = "VK_NV_device_diagnostic_checkpoints")]
            nv_device_diagnostic_checkpoints: false,
            #[cfg(feature = "VK_KHR_video_decode_queue")]
            khr_video_decode_queue: false,
            #[cfg(feature = "VK_KHR_deferred_host_operations")]
            khr_deferred_host_operations: false,
            #[cfg(feature = "VK_KHR_sampler_mirror_clamp_to_edge")]
            khr_sampler_mirror_clamp_to_edge: false,
            #[cfg(feature = "VK_KHR_external_semaphore")]
            khr_external_semaphore: false,
            #[cfg(feature = "VK_NV_viewport_array2")]
            nv_viewport_array_2: false,
            #[cfg(feature = "VK_KHR_external_fence_win32")]
            khr_external_fence_win_32: false,
            #[cfg(feature = "VK_EXT_ycbcr_image_arrays")]
            ext_ycbcr_image_arrays: false,
            #[cfg(feature = "VK_KHR_buffer_device_address")]
            khr_buffer_device_address: false,
            #[cfg(feature = "VK_AMD_pipeline_compiler_control")]
            amd_pipeline_compiler_control: false,
            #[cfg(feature = "VK_KHR_swapchain_mutable_format")]
            khr_swapchain_mutable_format: false,
            #[cfg(feature = "VK_QCOM_rotated_copy_commands")]
            qcom_rotated_copy_commands: false,
            #[cfg(feature = "VK_EXT_color_write_enable")]
            ext_color_write_enable: false,
            #[cfg(feature = "VK_EXT_image_drm_format_modifier")]
            ext_image_drm_format_modifier: false,
            #[cfg(feature = "VK_EXT_headless_surface")]
            ext_headless_surface: false,
            #[cfg(feature = "VK_GGP_frame_token")]
            ggp_frame_token: false,
            #[cfg(feature = "VK_NV_external_memory")]
            nv_external_memory: false,
            #[cfg(feature = "VK_EXT_shader_stencil_export")]
            ext_shader_stencil_export: false,
            #[cfg(feature = "VK_KHR_xlib_surface")]
            khr_xlib_surface: false,
            #[cfg(feature = "VK_AMD_gcn_shader")]
            amd_gcn_shader: false,
            #[cfg(feature = "VK_EXT_4444_formats")]
            ext_4444_formats: false,
            #[cfg(feature = "VK_EXT_texel_buffer_alignment")]
            ext_texel_buffer_alignment: false,
            #[cfg(feature = "VK_EXT_validation_flags")]
            ext_validation_flags: false,
            #[cfg(feature = "VK_KHR_shader_clock")]
            khr_shader_clock: false,
            #[cfg(feature = "VK_QCOM_fragment_density_map_offset")]
            qcom_fragment_density_map_offset: false,
            #[cfg(feature = "VK_EXT_hdr_metadata")]
            ext_hdr_metadata: false,
            #[cfg(feature = "VK_EXT_pipeline_creation_feedback")]
            ext_pipeline_creation_feedback: false,
            #[cfg(feature = "VK_NV_inherited_viewport_scissor")]
            nv_inherited_viewport_scissor: false,
            #[cfg(feature = "VK_AMD_draw_indirect_count")]
            amd_draw_indirect_count: false,
            #[cfg(feature = "VK_NV_device_diagnostics_config")]
            nv_device_diagnostics_config: false,
            #[cfg(feature = "VK_KHR_dynamic_rendering")]
            khr_dynamic_rendering: false,
            #[cfg(feature = "VK_AMD_device_coherent_memory")]
            amd_device_coherent_memory: false,
            #[cfg(feature = "VK_AMD_buffer_marker")]
            amd_buffer_marker: false,
            #[cfg(feature = "VK_NV_compute_shader_derivatives")]
            nv_compute_shader_derivatives: false,
            #[cfg(feature = "VK_NV_external_memory_rdma")]
            nv_external_memory_rdma: false,
            #[cfg(feature = "VK_EXT_private_data")]
            ext_private_data: false,
            #[cfg(feature = "VK_NV_ray_tracing_motion_blur")]
            nv_ray_tracing_motion_blur: false,
            #[cfg(feature = "VK_NV_glsl_shader")]
            nv_glsl_shader: false,
            #[cfg(feature = "VK_EXT_buffer_device_address")]
            ext_buffer_device_address: false,
            #[cfg(feature = "VK_EXT_host_query_reset")]
            ext_host_query_reset: false,
            #[cfg(feature = "VK_EXT_load_store_op_none")]
            ext_load_store_op_none: false,
            #[cfg(feature = "VK_AMD_shader_core_properties2")]
            amd_shader_core_properties_2: false,
            #[cfg(feature = "VK_QCOM_render_pass_store_ops")]
            qcom_render_pass_store_ops: false,
            #[cfg(feature = "VK_NV_win32_keyed_mutex")]
            nv_win_32_keyed_mutex: false,
        }
    }
}
impl Extensions {
    ///Creates an empty set of extensions with Vulkan v1.0
    pub const fn new() -> Self {
        Self::default()
    }
    ///Creates an empty set of extensions with a vulkan version
    pub const fn from_version(version: Version) -> Self {
        Self {
            version,
            ..Default::default()
        }
    }
    ///Creates an empty set of extensions with Vulkan v1.0
    pub const fn vulkan1_0() -> Self {
        Self::default()
    }
    ///Creates an empty set of extensions with Vulkan v1.1
    pub const fn vulkan1_1() -> Self {
        Self {
            version: Version::VULKAN1_1,
            ..Default::default()
        }
    }
    ///Creates an empty set of extensions with Vulkan v1.2
    pub const fn vulkan1_2() -> Self {
        Self {
            version: Version::VULKAN1_2,
            ..Default::default()
        }
    }
    ///Creates an empty set of extensions with Vulkan v1.3
    pub const fn vulkan1_3() -> Self {
        Self {
            version: Version::VULKAN1_3,
            ..Default::default()
        }
    }
    ///Gets the version of this extension set.
    #[inline(always)]
    pub const fn version(&self) -> Version {
        self.version
    }
    ///Enables `VK_AMD_negative_viewport_height` and it dependencies
    #[cfg(feature = "VK_AMD_negative_viewport_height")]
    pub const fn enable_amd_negative_viewport_height(mut self) -> Self {
        self = self;
        self.amd_negative_viewport_height = true;
        self
    }
    ///Checks if `VK_AMD_negative_viewport_height` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_AMD_negative_viewport_height")]
    pub const fn amd_negative_viewport_height(&self) -> bool {
        self.amd_negative_viewport_height
    }
    ///Enables `VK_EXT_image_view_min_lod` and it dependencies
    #[cfg(feature = "VK_EXT_image_view_min_lod")]
    pub const fn enable_ext_image_view_min_lod(mut self) -> Self {
        self = self.enable_khr_get_physical_device_properties_2();
        self.ext_image_view_min_lod = true;
        self
    }
    ///Checks if `VK_EXT_image_view_min_lod` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_EXT_image_view_min_lod")]
    pub const fn ext_image_view_min_lod(&self) -> bool {
        self.ext_image_view_min_lod
    }
    ///Enables `VK_MVK_ios_surface` and it dependencies
    #[cfg(feature = "VK_MVK_ios_surface")]
    pub const fn enable_mvk_ios_surface(mut self) -> Self {
        self = self.enable_khr_surface();
        self.mvk_ios_surface = true;
        self
    }
    ///Checks if `VK_MVK_ios_surface` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_MVK_ios_surface")]
    pub const fn mvk_ios_surface(&self) -> bool {
        self.mvk_ios_surface
    }
    ///Enables `VK_KHR_external_memory` and it dependencies
    #[cfg(feature = "VK_KHR_external_memory")]
    pub const fn enable_khr_external_memory(mut self) -> Self {
        self = self.enable_khr_external_memory_capabilities();
        self.khr_external_memory = true;
        self
    }
    ///Checks if `VK_KHR_external_memory` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_KHR_external_memory")]
    pub const fn khr_external_memory(&self) -> bool {
        self.khr_external_memory
    }
    ///Enables `VK_KHR_bind_memory2` and it dependencies
    #[cfg(feature = "VK_KHR_bind_memory2")]
    pub const fn enable_khr_bind_memory_2(mut self) -> Self {
        self = self;
        self.khr_bind_memory_2 = true;
        self
    }
    ///Checks if `VK_KHR_bind_memory2` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_KHR_bind_memory2")]
    pub const fn khr_bind_memory_2(&self) -> bool {
        self.khr_bind_memory_2
    }
    ///Enables `VK_KHR_present_id` and it dependencies
    #[cfg(feature = "VK_KHR_present_id")]
    pub const fn enable_khr_present_id(mut self) -> Self {
        self = self.enable_khr_swapchain();
        self.khr_present_id = true;
        self
    }
    ///Checks if `VK_KHR_present_id` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_KHR_present_id")]
    pub const fn khr_present_id(&self) -> bool {
        self.khr_present_id
    }
    ///Enables `VK_KHR_xcb_surface` and it dependencies
    #[cfg(feature = "VK_KHR_xcb_surface")]
    pub const fn enable_khr_xcb_surface(mut self) -> Self {
        self = self.enable_khr_surface();
        self.khr_xcb_surface = true;
        self
    }
    ///Checks if `VK_KHR_xcb_surface` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_KHR_xcb_surface")]
    pub const fn khr_xcb_surface(&self) -> bool {
        self.khr_xcb_surface
    }
    ///Enables `VK_KHR_pipeline_library` and it dependencies
    #[cfg(feature = "VK_KHR_pipeline_library")]
    pub const fn enable_khr_pipeline_library(mut self) -> Self {
        self = self;
        self.khr_pipeline_library = true;
        self
    }
    ///Checks if `VK_KHR_pipeline_library` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_KHR_pipeline_library")]
    pub const fn khr_pipeline_library(&self) -> bool {
        self.khr_pipeline_library
    }
    ///Enables `VK_KHR_get_display_properties2` and it dependencies
    #[cfg(feature = "VK_KHR_get_display_properties2")]
    pub const fn enable_khr_get_display_properties_2(mut self) -> Self {
        self = self.enable_khr_display();
        self.khr_get_display_properties_2 = true;
        self
    }
    ///Checks if `VK_KHR_get_display_properties2` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_KHR_get_display_properties2")]
    pub const fn khr_get_display_properties_2(&self) -> bool {
        self.khr_get_display_properties_2
    }
    ///Enables `VK_AMD_shader_image_load_store_lod` and it dependencies
    #[cfg(feature = "VK_AMD_shader_image_load_store_lod")]
    pub const fn enable_amd_shader_image_load_store_lod(mut self) -> Self {
        self = self;
        self.amd_shader_image_load_store_lod = true;
        self
    }
    ///Checks if `VK_AMD_shader_image_load_store_lod` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_AMD_shader_image_load_store_lod")]
    pub const fn amd_shader_image_load_store_lod(&self) -> bool {
        self.amd_shader_image_load_store_lod
    }
    ///Enables `VK_NN_vi_surface` and it dependencies
    #[cfg(feature = "VK_NN_vi_surface")]
    pub const fn enable_nn_vi_surface(mut self) -> Self {
        self = self.enable_khr_surface();
        self.nn_vi_surface = true;
        self
    }
    ///Checks if `VK_NN_vi_surface` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_NN_vi_surface")]
    pub const fn nn_vi_surface(&self) -> bool {
        self.nn_vi_surface
    }
    ///Enables `VK_KHR_video_queue` and it dependencies
    #[cfg(feature = "VK_KHR_video_queue")]
    pub const fn enable_khr_video_queue(mut self) -> Self {
        self = self
            .enable_khr_get_physical_device_properties_2()
            .enable_khr_sampler_ycbcr_conversion();
        self.khr_video_queue = true;
        self
    }
    ///Checks if `VK_KHR_video_queue` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_KHR_video_queue")]
    pub const fn khr_video_queue(&self) -> bool {
        self.khr_video_queue
    }
    ///Enables `VK_EXT_tooling_info` and it dependencies
    #[cfg(feature = "VK_EXT_tooling_info")]
    pub const fn enable_ext_tooling_info(mut self) -> Self {
        self = self;
        self.ext_tooling_info = true;
        self
    }
    ///Checks if `VK_EXT_tooling_info` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_EXT_tooling_info")]
    pub const fn ext_tooling_info(&self) -> bool {
        self.ext_tooling_info
    }
    ///Enables `VK_KHR_swapchain` and it dependencies
    #[cfg(feature = "VK_KHR_swapchain")]
    pub const fn enable_khr_swapchain(mut self) -> Self {
        self = self.enable_khr_surface();
        self.khr_swapchain = true;
        self
    }
    ///Checks if `VK_KHR_swapchain` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_KHR_swapchain")]
    pub const fn khr_swapchain(&self) -> bool {
        self.khr_swapchain
    }
    ///Enables `VK_EXT_shader_subgroup_ballot` and it dependencies
    #[cfg(feature = "VK_EXT_shader_subgroup_ballot")]
    pub const fn enable_ext_shader_subgroup_ballot(mut self) -> Self {
        self = self;
        self.ext_shader_subgroup_ballot = true;
        self
    }
    ///Checks if `VK_EXT_shader_subgroup_ballot` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_EXT_shader_subgroup_ballot")]
    pub const fn ext_shader_subgroup_ballot(&self) -> bool {
        self.ext_shader_subgroup_ballot
    }
    ///Enables `VK_KHR_push_descriptor` and it dependencies
    #[cfg(feature = "VK_KHR_push_descriptor")]
    pub const fn enable_khr_push_descriptor(mut self) -> Self {
        self = self.enable_khr_get_physical_device_properties_2();
        self.khr_push_descriptor = true;
        self
    }
    ///Checks if `VK_KHR_push_descriptor` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_KHR_push_descriptor")]
    pub const fn khr_push_descriptor(&self) -> bool {
        self.khr_push_descriptor
    }
    ///Enables `VK_KHR_incremental_present` and it dependencies
    #[cfg(feature = "VK_KHR_incremental_present")]
    pub const fn enable_khr_incremental_present(mut self) -> Self {
        self = self.enable_khr_swapchain();
        self.khr_incremental_present = true;
        self
    }
    ///Checks if `VK_KHR_incremental_present` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_KHR_incremental_present")]
    pub const fn khr_incremental_present(&self) -> bool {
        self.khr_incremental_present
    }
    ///Enables `VK_EXT_external_memory_host` and it dependencies
    #[cfg(feature = "VK_EXT_external_memory_host")]
    pub const fn enable_ext_external_memory_host(mut self) -> Self {
        self = self.enable_khr_external_memory();
        self.ext_external_memory_host = true;
        self
    }
    ///Checks if `VK_EXT_external_memory_host` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_EXT_external_memory_host")]
    pub const fn ext_external_memory_host(&self) -> bool {
        self.ext_external_memory_host
    }
    ///Enables `VK_NV_device_generated_commands` and it dependencies
    #[cfg(feature = "VK_NV_device_generated_commands")]
    pub const fn enable_nv_device_generated_commands(mut self) -> Self {
        self = self.enable_khr_buffer_device_address();
        self.nv_device_generated_commands = true;
        self
    }
    ///Checks if `VK_NV_device_generated_commands` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_NV_device_generated_commands")]
    pub const fn nv_device_generated_commands(&self) -> bool {
        self.nv_device_generated_commands
    }
    ///Enables `VK_EXT_robustness2` and it dependencies
    #[cfg(feature = "VK_EXT_robustness2")]
    pub const fn enable_ext_robustness_2(mut self) -> Self {
        self = self;
        self.ext_robustness_2 = true;
        self
    }
    ///Checks if `VK_EXT_robustness2` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_EXT_robustness2")]
    pub const fn ext_robustness_2(&self) -> bool {
        self.ext_robustness_2
    }
    ///Enables `VK_NV_fill_rectangle` and it dependencies
    #[cfg(feature = "VK_NV_fill_rectangle")]
    pub const fn enable_nv_fill_rectangle(mut self) -> Self {
        self = self;
        self.nv_fill_rectangle = true;
        self
    }
    ///Checks if `VK_NV_fill_rectangle` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_NV_fill_rectangle")]
    pub const fn nv_fill_rectangle(&self) -> bool {
        self.nv_fill_rectangle
    }
    ///Enables `VK_EXT_metal_surface` and it dependencies
    #[cfg(feature = "VK_EXT_metal_surface")]
    pub const fn enable_ext_metal_surface(mut self) -> Self {
        self = self.enable_khr_surface();
        self.ext_metal_surface = true;
        self
    }
    ///Checks if `VK_EXT_metal_surface` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_EXT_metal_surface")]
    pub const fn ext_metal_surface(&self) -> bool {
        self.ext_metal_surface
    }
    ///Enables `VK_MVK_macos_surface` and it dependencies
    #[cfg(feature = "VK_MVK_macos_surface")]
    pub const fn enable_mvk_macos_surface(mut self) -> Self {
        self = self.enable_khr_surface();
        self.mvk_macos_surface = true;
        self
    }
    ///Checks if `VK_MVK_macos_surface` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_MVK_macos_surface")]
    pub const fn mvk_macos_surface(&self) -> bool {
        self.mvk_macos_surface
    }
    ///Enables `VK_KHR_maintenance3` and it dependencies
    #[cfg(feature = "VK_KHR_maintenance3")]
    pub const fn enable_khr_maintenance_3(mut self) -> Self {
        self = self.enable_khr_get_physical_device_properties_2();
        self.khr_maintenance_3 = true;
        self
    }
    ///Checks if `VK_KHR_maintenance3` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_KHR_maintenance3")]
    pub const fn khr_maintenance_3(&self) -> bool {
        self.khr_maintenance_3
    }
    ///Enables `VK_KHR_android_surface` and it dependencies
    #[cfg(feature = "VK_KHR_android_surface")]
    pub const fn enable_khr_android_surface(mut self) -> Self {
        self = self.enable_khr_surface();
        self.khr_android_surface = true;
        self
    }
    ///Checks if `VK_KHR_android_surface` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_KHR_android_surface")]
    pub const fn khr_android_surface(&self) -> bool {
        self.khr_android_surface
    }
    ///Enables `VK_EXT_pci_bus_info` and it dependencies
    #[cfg(feature = "VK_EXT_pci_bus_info")]
    pub const fn enable_ext_pci_bus_info(mut self) -> Self {
        self = self.enable_khr_get_physical_device_properties_2();
        self.ext_pci_bus_info = true;
        self
    }
    ///Checks if `VK_EXT_pci_bus_info` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_EXT_pci_bus_info")]
    pub const fn ext_pci_bus_info(&self) -> bool {
        self.ext_pci_bus_info
    }
    ///Enables `VK_NV_external_memory_capabilities` and it dependencies
    #[cfg(feature = "VK_NV_external_memory_capabilities")]
    pub const fn enable_nv_external_memory_capabilities(mut self) -> Self {
        self = self;
        self.nv_external_memory_capabilities = true;
        self
    }
    ///Checks if `VK_NV_external_memory_capabilities` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_NV_external_memory_capabilities")]
    pub const fn nv_external_memory_capabilities(&self) -> bool {
        self.nv_external_memory_capabilities
    }
    ///Enables `VK_NV_external_memory_win32` and it dependencies
    #[cfg(feature = "VK_NV_external_memory_win32")]
    pub const fn enable_nv_external_memory_win_32(mut self) -> Self {
        self = self.enable_nv_external_memory();
        self.nv_external_memory_win_32 = true;
        self
    }
    ///Checks if `VK_NV_external_memory_win32` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_NV_external_memory_win32")]
    pub const fn nv_external_memory_win_32(&self) -> bool {
        self.nv_external_memory_win_32
    }
    ///Enables `VK_EXT_sampler_filter_minmax` and it dependencies
    #[cfg(feature = "VK_EXT_sampler_filter_minmax")]
    pub const fn enable_ext_sampler_filter_minmax(mut self) -> Self {
        self = self.enable_khr_get_physical_device_properties_2();
        self.ext_sampler_filter_minmax = true;
        self
    }
    ///Checks if `VK_EXT_sampler_filter_minmax` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_EXT_sampler_filter_minmax")]
    pub const fn ext_sampler_filter_minmax(&self) -> bool {
        self.ext_sampler_filter_minmax
    }
    ///Enables `VK_AMD_display_native_hdr` and it dependencies
    #[cfg(feature = "VK_AMD_display_native_hdr")]
    pub const fn enable_amd_display_native_hdr(mut self) -> Self {
        self = self
            .enable_khr_get_physical_device_properties_2()
            .enable_khr_get_surface_capabilities_2()
            .enable_khr_swapchain();
        self.amd_display_native_hdr = true;
        self
    }
    ///Checks if `VK_AMD_display_native_hdr` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_AMD_display_native_hdr")]
    pub const fn amd_display_native_hdr(&self) -> bool {
        self.amd_display_native_hdr
    }
    ///Enables `VK_EXT_fragment_density_map` and it dependencies
    #[cfg(feature = "VK_EXT_fragment_density_map")]
    pub const fn enable_ext_fragment_density_map(mut self) -> Self {
        self = self.enable_khr_get_physical_device_properties_2();
        self.ext_fragment_density_map = true;
        self
    }
    ///Checks if `VK_EXT_fragment_density_map` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_EXT_fragment_density_map")]
    pub const fn ext_fragment_density_map(&self) -> bool {
        self.ext_fragment_density_map
    }
    ///Enables `VK_NV_cooperative_matrix` and it dependencies
    #[cfg(feature = "VK_NV_cooperative_matrix")]
    pub const fn enable_nv_cooperative_matrix(mut self) -> Self {
        self = self.enable_khr_get_physical_device_properties_2();
        self.nv_cooperative_matrix = true;
        self
    }
    ///Checks if `VK_NV_cooperative_matrix` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_NV_cooperative_matrix")]
    pub const fn nv_cooperative_matrix(&self) -> bool {
        self.nv_cooperative_matrix
    }
    ///Enables `VK_EXT_multi_draw` and it dependencies
    #[cfg(feature = "VK_EXT_multi_draw")]
    pub const fn enable_ext_multi_draw(mut self) -> Self {
        self = self;
        self.ext_multi_draw = true;
        self
    }
    ///Checks if `VK_EXT_multi_draw` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_EXT_multi_draw")]
    pub const fn ext_multi_draw(&self) -> bool {
        self.ext_multi_draw
    }
    ///Enables `VK_KHR_vulkan_memory_model` and it dependencies
    #[cfg(feature = "VK_KHR_vulkan_memory_model")]
    pub const fn enable_khr_vulkan_memory_model(mut self) -> Self {
        self = self;
        self.khr_vulkan_memory_model = true;
        self
    }
    ///Checks if `VK_KHR_vulkan_memory_model` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_KHR_vulkan_memory_model")]
    pub const fn khr_vulkan_memory_model(&self) -> bool {
        self.khr_vulkan_memory_model
    }
    ///Enables `VK_NV_linear_color_attachment` and it dependencies
    #[cfg(feature = "VK_NV_linear_color_attachment")]
    pub const fn enable_nv_linear_color_attachment(mut self) -> Self {
        self = self;
        self.nv_linear_color_attachment = true;
        self
    }
    ///Checks if `VK_NV_linear_color_attachment` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_NV_linear_color_attachment")]
    pub const fn nv_linear_color_attachment(&self) -> bool {
        self.nv_linear_color_attachment
    }
    ///Enables `VK_AMD_shader_info` and it dependencies
    #[cfg(feature = "VK_AMD_shader_info")]
    pub const fn enable_amd_shader_info(mut self) -> Self {
        self = self;
        self.amd_shader_info = true;
        self
    }
    ///Checks if `VK_AMD_shader_info` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_AMD_shader_info")]
    pub const fn amd_shader_info(&self) -> bool {
        self.amd_shader_info
    }
    ///Enables `VK_EXT_global_priority_query` and it dependencies
    #[cfg(feature = "VK_EXT_global_priority_query")]
    pub const fn enable_ext_global_priority_query(mut self) -> Self {
        self = self
            .enable_ext_global_priority()
            .enable_khr_get_physical_device_properties_2();
        self.ext_global_priority_query = true;
        self
    }
    ///Checks if `VK_EXT_global_priority_query` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_EXT_global_priority_query")]
    pub const fn ext_global_priority_query(&self) -> bool {
        self.ext_global_priority_query
    }
    ///Enables `VK_KHR_external_semaphore_win32` and it dependencies
    #[cfg(feature = "VK_KHR_external_semaphore_win32")]
    pub const fn enable_khr_external_semaphore_win_32(mut self) -> Self {
        self = self.enable_khr_external_semaphore();
        self.khr_external_semaphore_win_32 = true;
        self
    }
    ///Checks if `VK_KHR_external_semaphore_win32` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_KHR_external_semaphore_win32")]
    pub const fn khr_external_semaphore_win_32(&self) -> bool {
        self.khr_external_semaphore_win_32
    }
    ///Enables `VK_NV_dedicated_allocation_image_aliasing` and it dependencies
    #[cfg(feature = "VK_NV_dedicated_allocation_image_aliasing")]
    pub const fn enable_nv_dedicated_allocation_image_aliasing(mut self) -> Self {
        self = self.enable_khr_dedicated_allocation();
        self.nv_dedicated_allocation_image_aliasing = true;
        self
    }
    ///Checks if `VK_NV_dedicated_allocation_image_aliasing` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_NV_dedicated_allocation_image_aliasing")]
    pub const fn nv_dedicated_allocation_image_aliasing(&self) -> bool {
        self.nv_dedicated_allocation_image_aliasing
    }
    ///Enables `VK_KHR_shared_presentable_image` and it dependencies
    #[cfg(feature = "VK_KHR_shared_presentable_image")]
    pub const fn enable_khr_shared_presentable_image(mut self) -> Self {
        self = self
            .enable_khr_swapchain()
            .enable_khr_get_physical_device_properties_2()
            .enable_khr_get_surface_capabilities_2();
        self.khr_shared_presentable_image = true;
        self
    }
    ///Checks if `VK_KHR_shared_presentable_image` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_KHR_shared_presentable_image")]
    pub const fn khr_shared_presentable_image(&self) -> bool {
        self.khr_shared_presentable_image
    }
    ///Enables `VK_KHR_external_fence_capabilities` and it dependencies
    #[cfg(feature = "VK_KHR_external_fence_capabilities")]
    pub const fn enable_khr_external_fence_capabilities(mut self) -> Self {
        self = self.enable_khr_get_physical_device_properties_2();
        self.khr_external_fence_capabilities = true;
        self
    }
    ///Checks if `VK_KHR_external_fence_capabilities` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_KHR_external_fence_capabilities")]
    pub const fn khr_external_fence_capabilities(&self) -> bool {
        self.khr_external_fence_capabilities
    }
    ///Enables `VK_NV_sample_mask_override_coverage` and it dependencies
    #[cfg(feature = "VK_NV_sample_mask_override_coverage")]
    pub const fn enable_nv_sample_mask_override_coverage(mut self) -> Self {
        self = self;
        self.nv_sample_mask_override_coverage = true;
        self
    }
    ///Checks if `VK_NV_sample_mask_override_coverage` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_NV_sample_mask_override_coverage")]
    pub const fn nv_sample_mask_override_coverage(&self) -> bool {
        self.nv_sample_mask_override_coverage
    }
    ///Enables `VK_FUCHSIA_buffer_collection` and it dependencies
    #[cfg(feature = "VK_FUCHSIA_buffer_collection")]
    pub const fn enable_fuchsia_buffer_collection(mut self) -> Self {
        self = self
            .enable_fuchsia_external_memory()
            .enable_khr_sampler_ycbcr_conversion();
        self.fuchsia_buffer_collection = true;
        self
    }
    ///Checks if `VK_FUCHSIA_buffer_collection` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_FUCHSIA_buffer_collection")]
    pub const fn fuchsia_buffer_collection(&self) -> bool {
        self.fuchsia_buffer_collection
    }
    ///Enables `VK_NV_shader_image_footprint` and it dependencies
    #[cfg(feature = "VK_NV_shader_image_footprint")]
    pub const fn enable_nv_shader_image_footprint(mut self) -> Self {
        self = self.enable_khr_get_physical_device_properties_2();
        self.nv_shader_image_footprint = true;
        self
    }
    ///Checks if `VK_NV_shader_image_footprint` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_NV_shader_image_footprint")]
    pub const fn nv_shader_image_footprint(&self) -> bool {
        self.nv_shader_image_footprint
    }
    ///Enables `VK_EXT_discard_rectangles` and it dependencies
    #[cfg(feature = "VK_EXT_discard_rectangles")]
    pub const fn enable_ext_discard_rectangles(mut self) -> Self {
        self = self.enable_khr_get_physical_device_properties_2();
        self.ext_discard_rectangles = true;
        self
    }
    ///Checks if `VK_EXT_discard_rectangles` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_EXT_discard_rectangles")]
    pub const fn ext_discard_rectangles(&self) -> bool {
        self.ext_discard_rectangles
    }
    ///Enables `VK_EXT_shader_atomic_float` and it dependencies
    #[cfg(feature = "VK_EXT_shader_atomic_float")]
    pub const fn enable_ext_shader_atomic_float(mut self) -> Self {
        self = self.enable_khr_get_physical_device_properties_2();
        self.ext_shader_atomic_float = true;
        self
    }
    ///Checks if `VK_EXT_shader_atomic_float` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_EXT_shader_atomic_float")]
    pub const fn ext_shader_atomic_float(&self) -> bool {
        self.ext_shader_atomic_float
    }
    ///Enables `VK_EXT_debug_report` and it dependencies
    #[cfg(feature = "VK_EXT_debug_report")]
    pub const fn enable_ext_debug_report(mut self) -> Self {
        self = self;
        self.ext_debug_report = true;
        self
    }
    ///Checks if `VK_EXT_debug_report` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_EXT_debug_report")]
    pub const fn ext_debug_report(&self) -> bool {
        self.ext_debug_report
    }
    ///Enables `VK_KHR_win32_keyed_mutex` and it dependencies
    #[cfg(feature = "VK_KHR_win32_keyed_mutex")]
    pub const fn enable_khr_win_32_keyed_mutex(mut self) -> Self {
        self = self.enable_khr_external_memory_win_32();
        self.khr_win_32_keyed_mutex = true;
        self
    }
    ///Checks if `VK_KHR_win32_keyed_mutex` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_KHR_win32_keyed_mutex")]
    pub const fn khr_win_32_keyed_mutex(&self) -> bool {
        self.khr_win_32_keyed_mutex
    }
    ///Enables `VK_NVX_multiview_per_view_attributes` and it dependencies
    #[cfg(feature = "VK_NVX_multiview_per_view_attributes")]
    pub const fn enable_nvx_multiview_per_view_attributes(mut self) -> Self {
        self = self.enable_khr_multiview();
        self.nvx_multiview_per_view_attributes = true;
        self
    }
    ///Checks if `VK_NVX_multiview_per_view_attributes` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_NVX_multiview_per_view_attributes")]
    pub const fn nvx_multiview_per_view_attributes(&self) -> bool {
        self.nvx_multiview_per_view_attributes
    }
    ///Enables `VK_KHR_external_memory_fd` and it dependencies
    #[cfg(feature = "VK_KHR_external_memory_fd")]
    pub const fn enable_khr_external_memory_fd(mut self) -> Self {
        self = self.enable_khr_external_memory();
        self.khr_external_memory_fd = true;
        self
    }
    ///Checks if `VK_KHR_external_memory_fd` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_KHR_external_memory_fd")]
    pub const fn khr_external_memory_fd(&self) -> bool {
        self.khr_external_memory_fd
    }
    ///Enables `VK_EXT_scalar_block_layout` and it dependencies
    #[cfg(feature = "VK_EXT_scalar_block_layout")]
    pub const fn enable_ext_scalar_block_layout(mut self) -> Self {
        self = self.enable_khr_get_physical_device_properties_2();
        self.ext_scalar_block_layout = true;
        self
    }
    ///Checks if `VK_EXT_scalar_block_layout` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_EXT_scalar_block_layout")]
    pub const fn ext_scalar_block_layout(&self) -> bool {
        self.ext_scalar_block_layout
    }
    ///Enables `VK_AMD_rasterization_order` and it dependencies
    #[cfg(feature = "VK_AMD_rasterization_order")]
    pub const fn enable_amd_rasterization_order(mut self) -> Self {
        self = self;
        self.amd_rasterization_order = true;
        self
    }
    ///Checks if `VK_AMD_rasterization_order` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_AMD_rasterization_order")]
    pub const fn amd_rasterization_order(&self) -> bool {
        self.amd_rasterization_order
    }
    ///Enables `VK_EXT_image_robustness` and it dependencies
    #[cfg(feature = "VK_EXT_image_robustness")]
    pub const fn enable_ext_image_robustness(mut self) -> Self {
        self = self.enable_khr_get_physical_device_properties_2();
        self.ext_image_robustness = true;
        self
    }
    ///Checks if `VK_EXT_image_robustness` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_EXT_image_robustness")]
    pub const fn ext_image_robustness(&self) -> bool {
        self.ext_image_robustness
    }
    ///Enables `VK_NVX_image_view_handle` and it dependencies
    #[cfg(feature = "VK_NVX_image_view_handle")]
    pub const fn enable_nvx_image_view_handle(mut self) -> Self {
        self = self;
        self.nvx_image_view_handle = true;
        self
    }
    ///Checks if `VK_NVX_image_view_handle` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_NVX_image_view_handle")]
    pub const fn nvx_image_view_handle(&self) -> bool {
        self.nvx_image_view_handle
    }
    ///Enables `VK_NV_fragment_shader_barycentric` and it dependencies
    #[cfg(feature = "VK_NV_fragment_shader_barycentric")]
    pub const fn enable_nv_fragment_shader_barycentric(mut self) -> Self {
        self = self.enable_khr_get_physical_device_properties_2();
        self.nv_fragment_shader_barycentric = true;
        self
    }
    ///Checks if `VK_NV_fragment_shader_barycentric` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_NV_fragment_shader_barycentric")]
    pub const fn nv_fragment_shader_barycentric(&self) -> bool {
        self.nv_fragment_shader_barycentric
    }
    ///Enables `VK_EXT_depth_clip_enable` and it dependencies
    #[cfg(feature = "VK_EXT_depth_clip_enable")]
    pub const fn enable_ext_depth_clip_enable(mut self) -> Self {
        self = self;
        self.ext_depth_clip_enable = true;
        self
    }
    ///Checks if `VK_EXT_depth_clip_enable` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_EXT_depth_clip_enable")]
    pub const fn ext_depth_clip_enable(&self) -> bool {
        self.ext_depth_clip_enable
    }
    ///Enables `VK_VALVE_descriptor_set_host_mapping` and it dependencies
    #[cfg(feature = "VK_VALVE_descriptor_set_host_mapping")]
    pub const fn enable_valve_descriptor_set_host_mapping(mut self) -> Self {
        self = self;
        self.valve_descriptor_set_host_mapping = true;
        self
    }
    ///Checks if `VK_VALVE_descriptor_set_host_mapping` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_VALVE_descriptor_set_host_mapping")]
    pub const fn valve_descriptor_set_host_mapping(&self) -> bool {
        self.valve_descriptor_set_host_mapping
    }
    ///Enables `VK_EXT_custom_border_color` and it dependencies
    #[cfg(feature = "VK_EXT_custom_border_color")]
    pub const fn enable_ext_custom_border_color(mut self) -> Self {
        self = self;
        self.ext_custom_border_color = true;
        self
    }
    ///Checks if `VK_EXT_custom_border_color` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_EXT_custom_border_color")]
    pub const fn ext_custom_border_color(&self) -> bool {
        self.ext_custom_border_color
    }
    ///Enables `VK_KHR_global_priority` and it dependencies
    #[cfg(feature = "VK_KHR_global_priority")]
    pub const fn enable_khr_global_priority(mut self) -> Self {
        self = self;
        self.khr_global_priority = true;
        self
    }
    ///Checks if `VK_KHR_global_priority` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_KHR_global_priority")]
    pub const fn khr_global_priority(&self) -> bool {
        self.khr_global_priority
    }
    ///Enables `VK_NV_dedicated_allocation` and it dependencies
    #[cfg(feature = "VK_NV_dedicated_allocation")]
    pub const fn enable_nv_dedicated_allocation(mut self) -> Self {
        self = self;
        self.nv_dedicated_allocation = true;
        self
    }
    ///Checks if `VK_NV_dedicated_allocation` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_NV_dedicated_allocation")]
    pub const fn nv_dedicated_allocation(&self) -> bool {
        self.nv_dedicated_allocation
    }
    ///Enables `VK_EXT_conditional_rendering` and it dependencies
    #[cfg(feature = "VK_EXT_conditional_rendering")]
    pub const fn enable_ext_conditional_rendering(mut self) -> Self {
        self = self;
        self.ext_conditional_rendering = true;
        self
    }
    ///Checks if `VK_EXT_conditional_rendering` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_EXT_conditional_rendering")]
    pub const fn ext_conditional_rendering(&self) -> bool {
        self.ext_conditional_rendering
    }
    ///Enables `VK_EXT_pageable_device_local_memory` and it dependencies
    #[cfg(feature = "VK_EXT_pageable_device_local_memory")]
    pub const fn enable_ext_pageable_device_local_memory(mut self) -> Self {
        self = self.enable_ext_memory_priority();
        self.ext_pageable_device_local_memory = true;
        self
    }
    ///Checks if `VK_EXT_pageable_device_local_memory` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_EXT_pageable_device_local_memory")]
    pub const fn ext_pageable_device_local_memory(&self) -> bool {
        self.ext_pageable_device_local_memory
    }
    ///Enables `VK_KHR_create_renderpass2` and it dependencies
    #[cfg(feature = "VK_KHR_create_renderpass2")]
    pub const fn enable_khr_create_renderpass_2(mut self) -> Self {
        self = self.enable_khr_multiview().enable_khr_maintenance_2();
        self.khr_create_renderpass_2 = true;
        self
    }
    ///Checks if `VK_KHR_create_renderpass2` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_KHR_create_renderpass2")]
    pub const fn khr_create_renderpass_2(&self) -> bool {
        self.khr_create_renderpass_2
    }
    ///Enables `VK_EXT_validation_features` and it dependencies
    #[cfg(feature = "VK_EXT_validation_features")]
    pub const fn enable_ext_validation_features(mut self) -> Self {
        self = self;
        self.ext_validation_features = true;
        self
    }
    ///Checks if `VK_EXT_validation_features` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_EXT_validation_features")]
    pub const fn ext_validation_features(&self) -> bool {
        self.ext_validation_features
    }
    ///Enables `VK_QCOM_render_pass_transform` and it dependencies
    #[cfg(feature = "VK_QCOM_render_pass_transform")]
    pub const fn enable_qcom_render_pass_transform(mut self) -> Self {
        self = self.enable_khr_swapchain().enable_khr_surface();
        self.qcom_render_pass_transform = true;
        self
    }
    ///Checks if `VK_QCOM_render_pass_transform` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_QCOM_render_pass_transform")]
    pub const fn qcom_render_pass_transform(&self) -> bool {
        self.qcom_render_pass_transform
    }
    ///Enables `VK_GGP_stream_descriptor_surface` and it dependencies
    #[cfg(feature = "VK_GGP_stream_descriptor_surface")]
    pub const fn enable_ggp_stream_descriptor_surface(mut self) -> Self {
        self = self.enable_khr_surface();
        self.ggp_stream_descriptor_surface = true;
        self
    }
    ///Checks if `VK_GGP_stream_descriptor_surface` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_GGP_stream_descriptor_surface")]
    pub const fn ggp_stream_descriptor_surface(&self) -> bool {
        self.ggp_stream_descriptor_surface
    }
    ///Enables `VK_EXT_depth_clip_control` and it dependencies
    #[cfg(feature = "VK_EXT_depth_clip_control")]
    pub const fn enable_ext_depth_clip_control(mut self) -> Self {
        self = self.enable_khr_get_physical_device_properties_2();
        self.ext_depth_clip_control = true;
        self
    }
    ///Checks if `VK_EXT_depth_clip_control` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_EXT_depth_clip_control")]
    pub const fn ext_depth_clip_control(&self) -> bool {
        self.ext_depth_clip_control
    }
    ///Enables `VK_KHR_present_wait` and it dependencies
    #[cfg(feature = "VK_KHR_present_wait")]
    pub const fn enable_khr_present_wait(mut self) -> Self {
        self = self.enable_khr_swapchain().enable_khr_present_id();
        self.khr_present_wait = true;
        self
    }
    ///Checks if `VK_KHR_present_wait` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_KHR_present_wait")]
    pub const fn khr_present_wait(&self) -> bool {
        self.khr_present_wait
    }
    ///Enables `VK_KHR_maintenance2` and it dependencies
    #[cfg(feature = "VK_KHR_maintenance2")]
    pub const fn enable_khr_maintenance_2(mut self) -> Self {
        self = self;
        self.khr_maintenance_2 = true;
        self
    }
    ///Checks if `VK_KHR_maintenance2` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_KHR_maintenance2")]
    pub const fn khr_maintenance_2(&self) -> bool {
        self.khr_maintenance_2
    }
    ///Enables `VK_KHR_relaxed_block_layout` and it dependencies
    #[cfg(feature = "VK_KHR_relaxed_block_layout")]
    pub const fn enable_khr_relaxed_block_layout(mut self) -> Self {
        self = self;
        self.khr_relaxed_block_layout = true;
        self
    }
    ///Checks if `VK_KHR_relaxed_block_layout` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_KHR_relaxed_block_layout")]
    pub const fn khr_relaxed_block_layout(&self) -> bool {
        self.khr_relaxed_block_layout
    }
    ///Enables `VK_KHR_external_memory_win32` and it dependencies
    #[cfg(feature = "VK_KHR_external_memory_win32")]
    pub const fn enable_khr_external_memory_win_32(mut self) -> Self {
        self = self.enable_khr_external_memory();
        self.khr_external_memory_win_32 = true;
        self
    }
    ///Checks if `VK_KHR_external_memory_win32` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_KHR_external_memory_win32")]
    pub const fn khr_external_memory_win_32(&self) -> bool {
        self.khr_external_memory_win_32
    }
    ///Enables `VK_KHR_external_semaphore_capabilities` and it dependencies
    #[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
    pub const fn enable_khr_external_semaphore_capabilities(mut self) -> Self {
        self = self.enable_khr_get_physical_device_properties_2();
        self.khr_external_semaphore_capabilities = true;
        self
    }
    ///Checks if `VK_KHR_external_semaphore_capabilities` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
    pub const fn khr_external_semaphore_capabilities(&self) -> bool {
        self.khr_external_semaphore_capabilities
    }
    ///Enables `VK_EXT_subgroup_size_control` and it dependencies
    #[cfg(feature = "VK_EXT_subgroup_size_control")]
    pub const fn enable_ext_subgroup_size_control(mut self) -> Self {
        self = self;
        self.ext_subgroup_size_control = true;
        self
    }
    ///Checks if `VK_EXT_subgroup_size_control` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_EXT_subgroup_size_control")]
    pub const fn ext_subgroup_size_control(&self) -> bool {
        self.ext_subgroup_size_control
    }
    ///Enables `VK_EXT_shader_demote_to_helper_invocation` and it dependencies
    #[cfg(feature = "VK_EXT_shader_demote_to_helper_invocation")]
    pub const fn enable_ext_shader_demote_to_helper_invocation(mut self) -> Self {
        self = self.enable_khr_get_physical_device_properties_2();
        self.ext_shader_demote_to_helper_invocation = true;
        self
    }
    ///Checks if `VK_EXT_shader_demote_to_helper_invocation` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_EXT_shader_demote_to_helper_invocation")]
    pub const fn ext_shader_demote_to_helper_invocation(&self) -> bool {
        self.ext_shader_demote_to_helper_invocation
    }
    ///Enables `VK_KHR_external_fence` and it dependencies
    #[cfg(feature = "VK_KHR_external_fence")]
    pub const fn enable_khr_external_fence(mut self) -> Self {
        self = self.enable_khr_external_fence_capabilities();
        self.khr_external_fence = true;
        self
    }
    ///Checks if `VK_KHR_external_fence` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_KHR_external_fence")]
    pub const fn khr_external_fence(&self) -> bool {
        self.khr_external_fence
    }
    ///Enables `VK_VALVE_mutable_descriptor_type` and it dependencies
    #[cfg(feature = "VK_VALVE_mutable_descriptor_type")]
    pub const fn enable_valve_mutable_descriptor_type(mut self) -> Self {
        self = self.enable_khr_maintenance_3();
        self.valve_mutable_descriptor_type = true;
        self
    }
    ///Checks if `VK_VALVE_mutable_descriptor_type` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_VALVE_mutable_descriptor_type")]
    pub const fn valve_mutable_descriptor_type(&self) -> bool {
        self.valve_mutable_descriptor_type
    }
    ///Enables `VK_EXT_debug_utils` and it dependencies
    #[cfg(feature = "VK_EXT_debug_utils")]
    pub const fn enable_ext_debug_utils(mut self) -> Self {
        self = self;
        self.ext_debug_utils = true;
        self
    }
    ///Checks if `VK_EXT_debug_utils` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_EXT_debug_utils")]
    pub const fn ext_debug_utils(&self) -> bool {
        self.ext_debug_utils
    }
    ///Enables `VK_NV_corner_sampled_image` and it dependencies
    #[cfg(feature = "VK_NV_corner_sampled_image")]
    pub const fn enable_nv_corner_sampled_image(mut self) -> Self {
        self = self.enable_khr_get_physical_device_properties_2();
        self.nv_corner_sampled_image = true;
        self
    }
    ///Checks if `VK_NV_corner_sampled_image` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_NV_corner_sampled_image")]
    pub const fn nv_corner_sampled_image(&self) -> bool {
        self.nv_corner_sampled_image
    }
    ///Enables `VK_EXT_validation_cache` and it dependencies
    #[cfg(feature = "VK_EXT_validation_cache")]
    pub const fn enable_ext_validation_cache(mut self) -> Self {
        self = self;
        self.ext_validation_cache = true;
        self
    }
    ///Checks if `VK_EXT_validation_cache` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_EXT_validation_cache")]
    pub const fn ext_validation_cache(&self) -> bool {
        self.ext_validation_cache
    }
    ///Enables `VK_KHR_depth_stencil_resolve` and it dependencies
    #[cfg(feature = "VK_KHR_depth_stencil_resolve")]
    pub const fn enable_khr_depth_stencil_resolve(mut self) -> Self {
        self = self.enable_khr_create_renderpass_2();
        self.khr_depth_stencil_resolve = true;
        self
    }
    ///Checks if `VK_KHR_depth_stencil_resolve` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_KHR_depth_stencil_resolve")]
    pub const fn khr_depth_stencil_resolve(&self) -> bool {
        self.khr_depth_stencil_resolve
    }
    ///Enables `VK_NV_shader_sm_builtins` and it dependencies
    #[cfg(feature = "VK_NV_shader_sm_builtins")]
    pub const fn enable_nv_shader_sm_builtins(mut self) -> Self {
        self = self;
        self.nv_shader_sm_builtins = true;
        self
    }
    ///Checks if `VK_NV_shader_sm_builtins` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_NV_shader_sm_builtins")]
    pub const fn nv_shader_sm_builtins(&self) -> bool {
        self.nv_shader_sm_builtins
    }
    ///Enables `VK_EXT_provoking_vertex` and it dependencies
    #[cfg(feature = "VK_EXT_provoking_vertex")]
    pub const fn enable_ext_provoking_vertex(mut self) -> Self {
        self = self.enable_khr_get_physical_device_properties_2();
        self.ext_provoking_vertex = true;
        self
    }
    ///Checks if `VK_EXT_provoking_vertex` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_EXT_provoking_vertex")]
    pub const fn ext_provoking_vertex(&self) -> bool {
        self.ext_provoking_vertex
    }
    ///Enables `VK_KHR_performance_query` and it dependencies
    #[cfg(feature = "VK_KHR_performance_query")]
    pub const fn enable_khr_performance_query(mut self) -> Self {
        self = self.enable_khr_get_physical_device_properties_2();
        self.khr_performance_query = true;
        self
    }
    ///Checks if `VK_KHR_performance_query` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_KHR_performance_query")]
    pub const fn khr_performance_query(&self) -> bool {
        self.khr_performance_query
    }
    ///Enables `VK_KHR_imageless_framebuffer` and it dependencies
    #[cfg(feature = "VK_KHR_imageless_framebuffer")]
    pub const fn enable_khr_imageless_framebuffer(mut self) -> Self {
        self = self.enable_khr_maintenance_2().enable_khr_image_format_list();
        self.khr_imageless_framebuffer = true;
        self
    }
    ///Checks if `VK_KHR_imageless_framebuffer` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_KHR_imageless_framebuffer")]
    pub const fn khr_imageless_framebuffer(&self) -> bool {
        self.khr_imageless_framebuffer
    }
    ///Enables `VK_KHR_descriptor_update_template` and it dependencies
    #[cfg(feature = "VK_KHR_descriptor_update_template")]
    pub const fn enable_khr_descriptor_update_template(mut self) -> Self {
        self = self;
        self.khr_descriptor_update_template = true;
        self
    }
    ///Checks if `VK_KHR_descriptor_update_template` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_KHR_descriptor_update_template")]
    pub const fn khr_descriptor_update_template(&self) -> bool {
        self.khr_descriptor_update_template
    }
    ///Enables `VK_NV_clip_space_w_scaling` and it dependencies
    #[cfg(feature = "VK_NV_clip_space_w_scaling")]
    pub const fn enable_nv_clip_space_w_scaling(mut self) -> Self {
        self = self;
        self.nv_clip_space_w_scaling = true;
        self
    }
    ///Checks if `VK_NV_clip_space_w_scaling` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_NV_clip_space_w_scaling")]
    pub const fn nv_clip_space_w_scaling(&self) -> bool {
        self.nv_clip_space_w_scaling
    }
    ///Enables `VK_KHR_storage_buffer_storage_class` and it dependencies
    #[cfg(feature = "VK_KHR_storage_buffer_storage_class")]
    pub const fn enable_khr_storage_buffer_storage_class(mut self) -> Self {
        self = self;
        self.khr_storage_buffer_storage_class = true;
        self
    }
    ///Checks if `VK_KHR_storage_buffer_storage_class` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_KHR_storage_buffer_storage_class")]
    pub const fn khr_storage_buffer_storage_class(&self) -> bool {
        self.khr_storage_buffer_storage_class
    }
    ///Enables `VK_KHR_get_memory_requirements2` and it dependencies
    #[cfg(feature = "VK_KHR_get_memory_requirements2")]
    pub const fn enable_khr_get_memory_requirements_2(mut self) -> Self {
        self = self;
        self.khr_get_memory_requirements_2 = true;
        self
    }
    ///Checks if `VK_KHR_get_memory_requirements2` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_KHR_get_memory_requirements2")]
    pub const fn khr_get_memory_requirements_2(&self) -> bool {
        self.khr_get_memory_requirements_2
    }
    ///Enables `VK_KHR_external_fence_fd` and it dependencies
    #[cfg(feature = "VK_KHR_external_fence_fd")]
    pub const fn enable_khr_external_fence_fd(mut self) -> Self {
        self = self.enable_khr_external_fence();
        self.khr_external_fence_fd = true;
        self
    }
    ///Checks if `VK_KHR_external_fence_fd` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_KHR_external_fence_fd")]
    pub const fn khr_external_fence_fd(&self) -> bool {
        self.khr_external_fence_fd
    }
    ///Enables `VK_EXT_shader_viewport_index_layer` and it dependencies
    #[cfg(feature = "VK_EXT_shader_viewport_index_layer")]
    pub const fn enable_ext_shader_viewport_index_layer(mut self) -> Self {
        self = self;
        self.ext_shader_viewport_index_layer = true;
        self
    }
    ///Checks if `VK_EXT_shader_viewport_index_layer` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_EXT_shader_viewport_index_layer")]
    pub const fn ext_shader_viewport_index_layer(&self) -> bool {
        self.ext_shader_viewport_index_layer
    }
    ///Enables `VK_NV_representative_fragment_test` and it dependencies
    #[cfg(feature = "VK_NV_representative_fragment_test")]
    pub const fn enable_nv_representative_fragment_test(mut self) -> Self {
        self = self;
        self.nv_representative_fragment_test = true;
        self
    }
    ///Checks if `VK_NV_representative_fragment_test` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_NV_representative_fragment_test")]
    pub const fn nv_representative_fragment_test(&self) -> bool {
        self.nv_representative_fragment_test
    }
    ///Enables `VK_KHR_wayland_surface` and it dependencies
    #[cfg(feature = "VK_KHR_wayland_surface")]
    pub const fn enable_khr_wayland_surface(mut self) -> Self {
        self = self.enable_khr_surface();
        self.khr_wayland_surface = true;
        self
    }
    ///Checks if `VK_KHR_wayland_surface` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_KHR_wayland_surface")]
    pub const fn khr_wayland_surface(&self) -> bool {
        self.khr_wayland_surface
    }
    ///Enables `VK_KHR_shader_terminate_invocation` and it dependencies
    #[cfg(feature = "VK_KHR_shader_terminate_invocation")]
    pub const fn enable_khr_shader_terminate_invocation(mut self) -> Self {
        self = self.enable_khr_get_physical_device_properties_2();
        self.khr_shader_terminate_invocation = true;
        self
    }
    ///Checks if `VK_KHR_shader_terminate_invocation` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_KHR_shader_terminate_invocation")]
    pub const fn khr_shader_terminate_invocation(&self) -> bool {
        self.khr_shader_terminate_invocation
    }
    ///Enables `VK_NV_viewport_swizzle` and it dependencies
    #[cfg(feature = "VK_NV_viewport_swizzle")]
    pub const fn enable_nv_viewport_swizzle(mut self) -> Self {
        self = self;
        self.nv_viewport_swizzle = true;
        self
    }
    ///Checks if `VK_NV_viewport_swizzle` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_NV_viewport_swizzle")]
    pub const fn nv_viewport_swizzle(&self) -> bool {
        self.nv_viewport_swizzle
    }
    ///Enables `VK_KHR_fragment_shading_rate` and it dependencies
    #[cfg(feature = "VK_KHR_fragment_shading_rate")]
    pub const fn enable_khr_fragment_shading_rate(mut self) -> Self {
        self = self
            .enable_khr_create_renderpass_2()
            .enable_khr_get_physical_device_properties_2();
        self.khr_fragment_shading_rate = true;
        self
    }
    ///Checks if `VK_KHR_fragment_shading_rate` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_KHR_fragment_shading_rate")]
    pub const fn khr_fragment_shading_rate(&self) -> bool {
        self.khr_fragment_shading_rate
    }
    ///Enables `VK_EXT_memory_budget` and it dependencies
    #[cfg(feature = "VK_EXT_memory_budget")]
    pub const fn enable_ext_memory_budget(mut self) -> Self {
        self = self.enable_khr_get_physical_device_properties_2();
        self.ext_memory_budget = true;
        self
    }
    ///Checks if `VK_EXT_memory_budget` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_EXT_memory_budget")]
    pub const fn ext_memory_budget(&self) -> bool {
        self.ext_memory_budget
    }
    ///Enables `VK_EXT_device_memory_report` and it dependencies
    #[cfg(feature = "VK_EXT_device_memory_report")]
    pub const fn enable_ext_device_memory_report(mut self) -> Self {
        self = self.enable_khr_get_physical_device_properties_2();
        self.ext_device_memory_report = true;
        self
    }
    ///Checks if `VK_EXT_device_memory_report` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_EXT_device_memory_report")]
    pub const fn ext_device_memory_report(&self) -> bool {
        self.ext_device_memory_report
    }
    ///Enables `VK_EXT_direct_mode_display` and it dependencies
    #[cfg(feature = "VK_EXT_direct_mode_display")]
    pub const fn enable_ext_direct_mode_display(mut self) -> Self {
        self = self.enable_khr_display();
        self.ext_direct_mode_display = true;
        self
    }
    ///Checks if `VK_EXT_direct_mode_display` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_EXT_direct_mode_display")]
    pub const fn ext_direct_mode_display(&self) -> bool {
        self.ext_direct_mode_display
    }
    ///Enables `VK_EXT_external_memory_dma_buf` and it dependencies
    #[cfg(feature = "VK_EXT_external_memory_dma_buf")]
    pub const fn enable_ext_external_memory_dma_buf(mut self) -> Self {
        self = self.enable_khr_external_memory_fd();
        self.ext_external_memory_dma_buf = true;
        self
    }
    ///Checks if `VK_EXT_external_memory_dma_buf` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_EXT_external_memory_dma_buf")]
    pub const fn ext_external_memory_dma_buf(&self) -> bool {
        self.ext_external_memory_dma_buf
    }
    ///Enables `VK_KHR_copy_commands2` and it dependencies
    #[cfg(feature = "VK_KHR_copy_commands2")]
    pub const fn enable_khr_copy_commands_2(mut self) -> Self {
        self = self;
        self.khr_copy_commands_2 = true;
        self
    }
    ///Checks if `VK_KHR_copy_commands2` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_KHR_copy_commands2")]
    pub const fn khr_copy_commands_2(&self) -> bool {
        self.khr_copy_commands_2
    }
    ///Enables `VK_EXT_video_decode_h265` and it dependencies
    #[cfg(feature = "VK_EXT_video_decode_h265")]
    pub const fn enable_ext_video_decode_h_265(mut self) -> Self {
        self = self.enable_khr_video_decode_queue();
        self.ext_video_decode_h_265 = true;
        self
    }
    ///Checks if `VK_EXT_video_decode_h265` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_EXT_video_decode_h265")]
    pub const fn ext_video_decode_h_265(&self) -> bool {
        self.ext_video_decode_h_265
    }
    ///Enables `VK_KHR_portability_enumeration` and it dependencies
    #[cfg(feature = "VK_KHR_portability_enumeration")]
    pub const fn enable_khr_portability_enumeration(mut self) -> Self {
        self = self;
        self.khr_portability_enumeration = true;
        self
    }
    ///Checks if `VK_KHR_portability_enumeration` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_KHR_portability_enumeration")]
    pub const fn khr_portability_enumeration(&self) -> bool {
        self.khr_portability_enumeration
    }
    ///Enables `VK_EXT_line_rasterization` and it dependencies
    #[cfg(feature = "VK_EXT_line_rasterization")]
    pub const fn enable_ext_line_rasterization(mut self) -> Self {
        self = self.enable_khr_get_physical_device_properties_2();
        self.ext_line_rasterization = true;
        self
    }
    ///Checks if `VK_EXT_line_rasterization` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_EXT_line_rasterization")]
    pub const fn ext_line_rasterization(&self) -> bool {
        self.ext_line_rasterization
    }
    ///Enables `VK_NV_scissor_exclusive` and it dependencies
    #[cfg(feature = "VK_NV_scissor_exclusive")]
    pub const fn enable_nv_scissor_exclusive(mut self) -> Self {
        self = self.enable_khr_get_physical_device_properties_2();
        self.nv_scissor_exclusive = true;
        self
    }
    ///Checks if `VK_NV_scissor_exclusive` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_NV_scissor_exclusive")]
    pub const fn nv_scissor_exclusive(&self) -> bool {
        self.nv_scissor_exclusive
    }
    ///Enables `VK_EXT_queue_family_foreign` and it dependencies
    #[cfg(feature = "VK_EXT_queue_family_foreign")]
    pub const fn enable_ext_queue_family_foreign(mut self) -> Self {
        self = self.enable_khr_external_memory();
        self.ext_queue_family_foreign = true;
        self
    }
    ///Checks if `VK_EXT_queue_family_foreign` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_EXT_queue_family_foreign")]
    pub const fn ext_queue_family_foreign(&self) -> bool {
        self.ext_queue_family_foreign
    }
    ///Enables `VK_EXT_video_encode_h265` and it dependencies
    #[cfg(feature = "VK_EXT_video_encode_h265")]
    pub const fn enable_ext_video_encode_h_265(mut self) -> Self {
        self = self.enable_khr_video_encode_queue();
        self.ext_video_encode_h_265 = true;
        self
    }
    ///Checks if `VK_EXT_video_encode_h265` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_EXT_video_encode_h265")]
    pub const fn ext_video_encode_h_265(&self) -> bool {
        self.ext_video_encode_h_265
    }
    ///Enables `VK_KHR_portability_subset` and it dependencies
    #[cfg(feature = "VK_KHR_portability_subset")]
    pub const fn enable_khr_portability_subset(mut self) -> Self {
        self = self.enable_khr_get_physical_device_properties_2();
        self.khr_portability_subset = true;
        self
    }
    ///Checks if `VK_KHR_portability_subset` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_KHR_portability_subset")]
    pub const fn khr_portability_subset(&self) -> bool {
        self.khr_portability_subset
    }
    ///Enables `VK_AMD_gpu_shader_int16` and it dependencies
    #[cfg(feature = "VK_AMD_gpu_shader_int16")]
    pub const fn enable_amd_gpu_shader_int_16(mut self) -> Self {
        self = self;
        self.amd_gpu_shader_int_16 = true;
        self
    }
    ///Checks if `VK_AMD_gpu_shader_int16` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_AMD_gpu_shader_int16")]
    pub const fn amd_gpu_shader_int_16(&self) -> bool {
        self.amd_gpu_shader_int_16
    }
    ///Enables `VK_KHR_zero_initialize_workgroup_memory` and it dependencies
    #[cfg(feature = "VK_KHR_zero_initialize_workgroup_memory")]
    pub const fn enable_khr_zero_initialize_workgroup_memory(mut self) -> Self {
        self = self.enable_khr_get_physical_device_properties_2();
        self.khr_zero_initialize_workgroup_memory = true;
        self
    }
    ///Checks if `VK_KHR_zero_initialize_workgroup_memory` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_KHR_zero_initialize_workgroup_memory")]
    pub const fn khr_zero_initialize_workgroup_memory(&self) -> bool {
        self.khr_zero_initialize_workgroup_memory
    }
    ///Enables `VK_INTEL_performance_query` and it dependencies
    #[cfg(feature = "VK_INTEL_performance_query")]
    pub const fn enable_intel_performance_query(mut self) -> Self {
        self = self;
        self.intel_performance_query = true;
        self
    }
    ///Checks if `VK_INTEL_performance_query` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_INTEL_performance_query")]
    pub const fn intel_performance_query(&self) -> bool {
        self.intel_performance_query
    }
    ///Enables `VK_EXT_video_encode_h264` and it dependencies
    #[cfg(feature = "VK_EXT_video_encode_h264")]
    pub const fn enable_ext_video_encode_h_264(mut self) -> Self {
        self = self.enable_khr_video_encode_queue();
        self.ext_video_encode_h_264 = true;
        self
    }
    ///Checks if `VK_EXT_video_encode_h264` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_EXT_video_encode_h264")]
    pub const fn ext_video_encode_h_264(&self) -> bool {
        self.ext_video_encode_h_264
    }
    ///Enables `VK_AMD_shader_fragment_mask` and it dependencies
    #[cfg(feature = "VK_AMD_shader_fragment_mask")]
    pub const fn enable_amd_shader_fragment_mask(mut self) -> Self {
        self = self;
        self.amd_shader_fragment_mask = true;
        self
    }
    ///Checks if `VK_AMD_shader_fragment_mask` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_AMD_shader_fragment_mask")]
    pub const fn amd_shader_fragment_mask(&self) -> bool {
        self.amd_shader_fragment_mask
    }
    ///Enables `VK_AMD_shader_core_properties` and it dependencies
    #[cfg(feature = "VK_AMD_shader_core_properties")]
    pub const fn enable_amd_shader_core_properties(mut self) -> Self {
        self = self.enable_khr_get_physical_device_properties_2();
        self.amd_shader_core_properties = true;
        self
    }
    ///Checks if `VK_AMD_shader_core_properties` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_AMD_shader_core_properties")]
    pub const fn amd_shader_core_properties(&self) -> bool {
        self.amd_shader_core_properties
    }
    ///Enables `VK_KHR_ray_query` and it dependencies
    #[cfg(feature = "VK_KHR_ray_query")]
    pub const fn enable_khr_ray_query(mut self) -> Self {
        self = self.enable_khr_spirv_1_4().enable_khr_acceleration_structure();
        self.khr_ray_query = true;
        self
    }
    ///Checks if `VK_KHR_ray_query` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_KHR_ray_query")]
    pub const fn khr_ray_query(&self) -> bool {
        self.khr_ray_query
    }
    ///Enables `VK_NV_framebuffer_mixed_samples` and it dependencies
    #[cfg(feature = "VK_NV_framebuffer_mixed_samples")]
    pub const fn enable_nv_framebuffer_mixed_samples(mut self) -> Self {
        self = self;
        self.nv_framebuffer_mixed_samples = true;
        self
    }
    ///Checks if `VK_NV_framebuffer_mixed_samples` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_NV_framebuffer_mixed_samples")]
    pub const fn nv_framebuffer_mixed_samples(&self) -> bool {
        self.nv_framebuffer_mixed_samples
    }
    ///Enables `VK_KHR_format_feature_flags2` and it dependencies
    #[cfg(feature = "VK_KHR_format_feature_flags2")]
    pub const fn enable_khr_format_feature_flags_2(mut self) -> Self {
        self = self.enable_khr_get_physical_device_properties_2();
        self.khr_format_feature_flags_2 = true;
        self
    }
    ///Checks if `VK_KHR_format_feature_flags2` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_KHR_format_feature_flags2")]
    pub const fn khr_format_feature_flags_2(&self) -> bool {
        self.khr_format_feature_flags_2
    }
    ///Enables `VK_EXT_pipeline_creation_cache_control` and it dependencies
    #[cfg(feature = "VK_EXT_pipeline_creation_cache_control")]
    pub const fn enable_ext_pipeline_creation_cache_control(mut self) -> Self {
        self = self;
        self.ext_pipeline_creation_cache_control = true;
        self
    }
    ///Checks if `VK_EXT_pipeline_creation_cache_control` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_EXT_pipeline_creation_cache_control")]
    pub const fn ext_pipeline_creation_cache_control(&self) -> bool {
        self.ext_pipeline_creation_cache_control
    }
    ///Enables `VK_HUAWEI_invocation_mask` and it dependencies
    #[cfg(feature = "VK_HUAWEI_invocation_mask")]
    pub const fn enable_huawei_invocation_mask(mut self) -> Self {
        self = self.enable_khr_ray_tracing_pipeline().enable_khr_synchronization_2();
        self.huawei_invocation_mask = true;
        self
    }
    ///Checks if `VK_HUAWEI_invocation_mask` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_HUAWEI_invocation_mask")]
    pub const fn huawei_invocation_mask(&self) -> bool {
        self.huawei_invocation_mask
    }
    ///Enables `VK_KHR_timeline_semaphore` and it dependencies
    #[cfg(feature = "VK_KHR_timeline_semaphore")]
    pub const fn enable_khr_timeline_semaphore(mut self) -> Self {
        self = self.enable_khr_get_physical_device_properties_2();
        self.khr_timeline_semaphore = true;
        self
    }
    ///Checks if `VK_KHR_timeline_semaphore` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_KHR_timeline_semaphore")]
    pub const fn khr_timeline_semaphore(&self) -> bool {
        self.khr_timeline_semaphore
    }
    ///Enables `VK_EXT_global_priority` and it dependencies
    #[cfg(feature = "VK_EXT_global_priority")]
    pub const fn enable_ext_global_priority(mut self) -> Self {
        self = self;
        self.ext_global_priority = true;
        self
    }
    ///Checks if `VK_EXT_global_priority` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_EXT_global_priority")]
    pub const fn ext_global_priority(&self) -> bool {
        self.ext_global_priority
    }
    ///Enables `VK_KHR_pipeline_executable_properties` and it dependencies
    #[cfg(feature = "VK_KHR_pipeline_executable_properties")]
    pub const fn enable_khr_pipeline_executable_properties(mut self) -> Self {
        self = self.enable_khr_get_physical_device_properties_2();
        self.khr_pipeline_executable_properties = true;
        self
    }
    ///Checks if `VK_KHR_pipeline_executable_properties` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_KHR_pipeline_executable_properties")]
    pub const fn khr_pipeline_executable_properties(&self) -> bool {
        self.khr_pipeline_executable_properties
    }
    ///Enables `VK_KHR_external_memory_capabilities` and it dependencies
    #[cfg(feature = "VK_KHR_external_memory_capabilities")]
    pub const fn enable_khr_external_memory_capabilities(mut self) -> Self {
        self = self.enable_khr_get_physical_device_properties_2();
        self.khr_external_memory_capabilities = true;
        self
    }
    ///Checks if `VK_KHR_external_memory_capabilities` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_KHR_external_memory_capabilities")]
    pub const fn khr_external_memory_capabilities(&self) -> bool {
        self.khr_external_memory_capabilities
    }
    ///Enables `VK_INTEL_shader_integer_functions2` and it dependencies
    #[cfg(feature = "VK_INTEL_shader_integer_functions2")]
    pub const fn enable_intel_shader_integer_functions_2(mut self) -> Self {
        self = self.enable_khr_get_physical_device_properties_2();
        self.intel_shader_integer_functions_2 = true;
        self
    }
    ///Checks if `VK_INTEL_shader_integer_functions2` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_INTEL_shader_integer_functions2")]
    pub const fn intel_shader_integer_functions_2(&self) -> bool {
        self.intel_shader_integer_functions_2
    }
    ///Enables `VK_HUAWEI_subpass_shading` and it dependencies
    #[cfg(feature = "VK_HUAWEI_subpass_shading")]
    pub const fn enable_huawei_subpass_shading(mut self) -> Self {
        self = self.enable_khr_create_renderpass_2().enable_khr_synchronization_2();
        self.huawei_subpass_shading = true;
        self
    }
    ///Checks if `VK_HUAWEI_subpass_shading` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_HUAWEI_subpass_shading")]
    pub const fn huawei_subpass_shading(&self) -> bool {
        self.huawei_subpass_shading
    }
    ///Enables `VK_NV_acquire_winrt_display` and it dependencies
    #[cfg(feature = "VK_NV_acquire_winrt_display")]
    pub const fn enable_nv_acquire_winrt_display(mut self) -> Self {
        self = self.enable_ext_direct_mode_display();
        self.nv_acquire_winrt_display = true;
        self
    }
    ///Checks if `VK_NV_acquire_winrt_display` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_NV_acquire_winrt_display")]
    pub const fn nv_acquire_winrt_display(&self) -> bool {
        self.nv_acquire_winrt_display
    }
    ///Enables `VK_KHR_surface` and it dependencies
    #[cfg(feature = "VK_KHR_surface")]
    pub const fn enable_khr_surface(mut self) -> Self {
        self = self;
        self.khr_surface = true;
        self
    }
    ///Checks if `VK_KHR_surface` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_KHR_surface")]
    pub const fn khr_surface(&self) -> bool {
        self.khr_surface
    }
    ///Enables `VK_FUCHSIA_external_semaphore` and it dependencies
    #[cfg(feature = "VK_FUCHSIA_external_semaphore")]
    pub const fn enable_fuchsia_external_semaphore(mut self) -> Self {
        self = self
            .enable_khr_external_semaphore_capabilities()
            .enable_khr_external_semaphore();
        self.fuchsia_external_semaphore = true;
        self
    }
    ///Checks if `VK_FUCHSIA_external_semaphore` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_FUCHSIA_external_semaphore")]
    pub const fn fuchsia_external_semaphore(&self) -> bool {
        self.fuchsia_external_semaphore
    }
    ///Enables `VK_KHR_acceleration_structure` and it dependencies
    #[cfg(feature = "VK_KHR_acceleration_structure")]
    pub const fn enable_khr_acceleration_structure(mut self) -> Self {
        self = self
            .enable_ext_descriptor_indexing()
            .enable_khr_buffer_device_address()
            .enable_khr_deferred_host_operations();
        self.khr_acceleration_structure = true;
        self
    }
    ///Checks if `VK_KHR_acceleration_structure` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_KHR_acceleration_structure")]
    pub const fn khr_acceleration_structure(&self) -> bool {
        self.khr_acceleration_structure
    }
    ///Enables `VK_KHR_win32_surface` and it dependencies
    #[cfg(feature = "VK_KHR_win32_surface")]
    pub const fn enable_khr_win_32_surface(mut self) -> Self {
        self = self.enable_khr_surface();
        self.khr_win_32_surface = true;
        self
    }
    ///Checks if `VK_KHR_win32_surface` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_KHR_win32_surface")]
    pub const fn khr_win_32_surface(&self) -> bool {
        self.khr_win_32_surface
    }
    ///Enables `VK_AMD_shader_trinary_minmax` and it dependencies
    #[cfg(feature = "VK_AMD_shader_trinary_minmax")]
    pub const fn enable_amd_shader_trinary_minmax(mut self) -> Self {
        self = self;
        self.amd_shader_trinary_minmax = true;
        self
    }
    ///Checks if `VK_AMD_shader_trinary_minmax` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_AMD_shader_trinary_minmax")]
    pub const fn amd_shader_trinary_minmax(&self) -> bool {
        self.amd_shader_trinary_minmax
    }
    ///Enables `VK_EXT_separate_stencil_usage` and it dependencies
    #[cfg(feature = "VK_EXT_separate_stencil_usage")]
    pub const fn enable_ext_separate_stencil_usage(mut self) -> Self {
        self = self;
        self.ext_separate_stencil_usage = true;
        self
    }
    ///Checks if `VK_EXT_separate_stencil_usage` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_EXT_separate_stencil_usage")]
    pub const fn ext_separate_stencil_usage(&self) -> bool {
        self.ext_separate_stencil_usage
    }
    ///Enables `VK_NV_ray_tracing` and it dependencies
    #[cfg(feature = "VK_NV_ray_tracing")]
    pub const fn enable_nv_ray_tracing(mut self) -> Self {
        self = self
            .enable_khr_get_physical_device_properties_2()
            .enable_khr_get_memory_requirements_2();
        self.nv_ray_tracing = true;
        self
    }
    ///Checks if `VK_NV_ray_tracing` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_NV_ray_tracing")]
    pub const fn nv_ray_tracing(&self) -> bool {
        self.nv_ray_tracing
    }
    ///Enables `VK_EXT_post_depth_coverage` and it dependencies
    #[cfg(feature = "VK_EXT_post_depth_coverage")]
    pub const fn enable_ext_post_depth_coverage(mut self) -> Self {
        self = self;
        self.ext_post_depth_coverage = true;
        self
    }
    ///Checks if `VK_EXT_post_depth_coverage` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_EXT_post_depth_coverage")]
    pub const fn ext_post_depth_coverage(&self) -> bool {
        self.ext_post_depth_coverage
    }
    ///Enables `VK_KHR_16bit_storage` and it dependencies
    #[cfg(feature = "VK_KHR_16bit_storage")]
    pub const fn enable_khr_16_bit_storage(mut self) -> Self {
        self = self
            .enable_khr_get_physical_device_properties_2()
            .enable_khr_storage_buffer_storage_class();
        self.khr_16_bit_storage = true;
        self
    }
    ///Checks if `VK_KHR_16bit_storage` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_KHR_16bit_storage")]
    pub const fn khr_16_bit_storage(&self) -> bool {
        self.khr_16_bit_storage
    }
    ///Enables `VK_EXT_vertex_attribute_divisor` and it dependencies
    #[cfg(feature = "VK_EXT_vertex_attribute_divisor")]
    pub const fn enable_ext_vertex_attribute_divisor(mut self) -> Self {
        self = self.enable_khr_get_physical_device_properties_2();
        self.ext_vertex_attribute_divisor = true;
        self
    }
    ///Checks if `VK_EXT_vertex_attribute_divisor` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_EXT_vertex_attribute_divisor")]
    pub const fn ext_vertex_attribute_divisor(&self) -> bool {
        self.ext_vertex_attribute_divisor
    }
    ///Enables `VK_KHR_shader_integer_dot_product` and it dependencies
    #[cfg(feature = "VK_KHR_shader_integer_dot_product")]
    pub const fn enable_khr_shader_integer_dot_product(mut self) -> Self {
        self = self.enable_khr_get_physical_device_properties_2();
        self.khr_shader_integer_dot_product = true;
        self
    }
    ///Checks if `VK_KHR_shader_integer_dot_product` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_KHR_shader_integer_dot_product")]
    pub const fn khr_shader_integer_dot_product(&self) -> bool {
        self.khr_shader_integer_dot_product
    }
    ///Enables `VK_NV_fragment_shading_rate_enums` and it dependencies
    #[cfg(feature = "VK_NV_fragment_shading_rate_enums")]
    pub const fn enable_nv_fragment_shading_rate_enums(mut self) -> Self {
        self = self.enable_khr_fragment_shading_rate();
        self.nv_fragment_shading_rate_enums = true;
        self
    }
    ///Checks if `VK_NV_fragment_shading_rate_enums` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_NV_fragment_shading_rate_enums")]
    pub const fn nv_fragment_shading_rate_enums(&self) -> bool {
        self.nv_fragment_shading_rate_enums
    }
    ///Enables `VK_KHR_image_format_list` and it dependencies
    #[cfg(feature = "VK_KHR_image_format_list")]
    pub const fn enable_khr_image_format_list(mut self) -> Self {
        self = self;
        self.khr_image_format_list = true;
        self
    }
    ///Checks if `VK_KHR_image_format_list` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_KHR_image_format_list")]
    pub const fn khr_image_format_list(&self) -> bool {
        self.khr_image_format_list
    }
    ///Enables `VK_KHR_maintenance4` and it dependencies
    #[cfg(feature = "VK_KHR_maintenance4")]
    pub const fn enable_khr_maintenance_4(mut self) -> Self {
        self = self;
        self.khr_maintenance_4 = true;
        self
    }
    ///Checks if `VK_KHR_maintenance4` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_KHR_maintenance4")]
    pub const fn khr_maintenance_4(&self) -> bool {
        self.khr_maintenance_4
    }
    ///Enables `VK_EXT_display_control` and it dependencies
    #[cfg(feature = "VK_EXT_display_control")]
    pub const fn enable_ext_display_control(mut self) -> Self {
        self = self.enable_ext_display_surface_counter().enable_khr_swapchain();
        self.ext_display_control = true;
        self
    }
    ///Checks if `VK_EXT_display_control` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_EXT_display_control")]
    pub const fn ext_display_control(&self) -> bool {
        self.ext_display_control
    }
    ///Enables `VK_EXT_video_decode_h264` and it dependencies
    #[cfg(feature = "VK_EXT_video_decode_h264")]
    pub const fn enable_ext_video_decode_h_264(mut self) -> Self {
        self = self.enable_khr_video_decode_queue();
        self.ext_video_decode_h_264 = true;
        self
    }
    ///Checks if `VK_EXT_video_decode_h264` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_EXT_video_decode_h264")]
    pub const fn ext_video_decode_h_264(&self) -> bool {
        self.ext_video_decode_h_264
    }
    ///Enables `VK_EXT_acquire_xlib_display` and it dependencies
    #[cfg(feature = "VK_EXT_acquire_xlib_display")]
    pub const fn enable_ext_acquire_xlib_display(mut self) -> Self {
        self = self.enable_ext_direct_mode_display();
        self.ext_acquire_xlib_display = true;
        self
    }
    ///Checks if `VK_EXT_acquire_xlib_display` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_EXT_acquire_xlib_display")]
    pub const fn ext_acquire_xlib_display(&self) -> bool {
        self.ext_acquire_xlib_display
    }
    ///Enables `VK_EXT_full_screen_exclusive` and it dependencies
    #[cfg(feature = "VK_EXT_full_screen_exclusive")]
    pub const fn enable_ext_full_screen_exclusive(mut self) -> Self {
        self = self
            .enable_khr_get_physical_device_properties_2()
            .enable_khr_surface()
            .enable_khr_get_surface_capabilities_2()
            .enable_khr_swapchain();
        self.ext_full_screen_exclusive = true;
        self
    }
    ///Checks if `VK_EXT_full_screen_exclusive` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_EXT_full_screen_exclusive")]
    pub const fn ext_full_screen_exclusive(&self) -> bool {
        self.ext_full_screen_exclusive
    }
    ///Enables `VK_GOOGLE_surfaceless_query` and it dependencies
    #[cfg(feature = "VK_GOOGLE_surfaceless_query")]
    pub const fn enable_google_surfaceless_query(mut self) -> Self {
        self = self.enable_khr_surface();
        self.google_surfaceless_query = true;
        self
    }
    ///Checks if `VK_GOOGLE_surfaceless_query` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_GOOGLE_surfaceless_query")]
    pub const fn google_surfaceless_query(&self) -> bool {
        self.google_surfaceless_query
    }
    ///Enables `VK_AMD_shader_ballot` and it dependencies
    #[cfg(feature = "VK_AMD_shader_ballot")]
    pub const fn enable_amd_shader_ballot(mut self) -> Self {
        self = self;
        self.amd_shader_ballot = true;
        self
    }
    ///Checks if `VK_AMD_shader_ballot` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_AMD_shader_ballot")]
    pub const fn amd_shader_ballot(&self) -> bool {
        self.amd_shader_ballot
    }
    ///Enables `VK_KHR_shader_float_controls` and it dependencies
    #[cfg(feature = "VK_KHR_shader_float_controls")]
    pub const fn enable_khr_shader_float_controls(mut self) -> Self {
        self = self.enable_khr_get_physical_device_properties_2();
        self.khr_shader_float_controls = true;
        self
    }
    ///Checks if `VK_KHR_shader_float_controls` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_KHR_shader_float_controls")]
    pub const fn khr_shader_float_controls(&self) -> bool {
        self.khr_shader_float_controls
    }
    ///Enables `VK_EXT_acquire_drm_display` and it dependencies
    #[cfg(feature = "VK_EXT_acquire_drm_display")]
    pub const fn enable_ext_acquire_drm_display(mut self) -> Self {
        self = self.enable_ext_direct_mode_display();
        self.ext_acquire_drm_display = true;
        self
    }
    ///Checks if `VK_EXT_acquire_drm_display` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_EXT_acquire_drm_display")]
    pub const fn ext_acquire_drm_display(&self) -> bool {
        self.ext_acquire_drm_display
    }
    ///Enables `VK_EXT_sample_locations` and it dependencies
    #[cfg(feature = "VK_EXT_sample_locations")]
    pub const fn enable_ext_sample_locations(mut self) -> Self {
        self = self.enable_khr_get_physical_device_properties_2();
        self.ext_sample_locations = true;
        self
    }
    ///Checks if `VK_EXT_sample_locations` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_EXT_sample_locations")]
    pub const fn ext_sample_locations(&self) -> bool {
        self.ext_sample_locations
    }
    ///Enables `VK_KHR_variable_pointers` and it dependencies
    #[cfg(feature = "VK_KHR_variable_pointers")]
    pub const fn enable_khr_variable_pointers(mut self) -> Self {
        self = self
            .enable_khr_get_physical_device_properties_2()
            .enable_khr_storage_buffer_storage_class();
        self.khr_variable_pointers = true;
        self
    }
    ///Checks if `VK_KHR_variable_pointers` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_KHR_variable_pointers")]
    pub const fn khr_variable_pointers(&self) -> bool {
        self.khr_variable_pointers
    }
    ///Enables `VK_EXT_extended_dynamic_state2` and it dependencies
    #[cfg(feature = "VK_EXT_extended_dynamic_state2")]
    pub const fn enable_ext_extended_dynamic_state_2(mut self) -> Self {
        self = self.enable_khr_get_physical_device_properties_2();
        self.ext_extended_dynamic_state_2 = true;
        self
    }
    ///Checks if `VK_EXT_extended_dynamic_state2` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_EXT_extended_dynamic_state2")]
    pub const fn ext_extended_dynamic_state_2(&self) -> bool {
        self.ext_extended_dynamic_state_2
    }
    ///Enables `VK_EXT_shader_atomic_float2` and it dependencies
    #[cfg(feature = "VK_EXT_shader_atomic_float2")]
    pub const fn enable_ext_shader_atomic_float_2(mut self) -> Self {
        self = self.enable_ext_shader_atomic_float();
        self.ext_shader_atomic_float_2 = true;
        self
    }
    ///Checks if `VK_EXT_shader_atomic_float2` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_EXT_shader_atomic_float2")]
    pub const fn ext_shader_atomic_float_2(&self) -> bool {
        self.ext_shader_atomic_float_2
    }
    ///Enables `VK_QNX_screen_surface` and it dependencies
    #[cfg(feature = "VK_QNX_screen_surface")]
    pub const fn enable_qnx_screen_surface(mut self) -> Self {
        self = self.enable_khr_surface();
        self.qnx_screen_surface = true;
        self
    }
    ///Checks if `VK_QNX_screen_surface` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_QNX_screen_surface")]
    pub const fn qnx_screen_surface(&self) -> bool {
        self.qnx_screen_surface
    }
    ///Enables `VK_FUCHSIA_imagepipe_surface` and it dependencies
    #[cfg(feature = "VK_FUCHSIA_imagepipe_surface")]
    pub const fn enable_fuchsia_imagepipe_surface(mut self) -> Self {
        self = self.enable_khr_surface();
        self.fuchsia_imagepipe_surface = true;
        self
    }
    ///Checks if `VK_FUCHSIA_imagepipe_surface` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_FUCHSIA_imagepipe_surface")]
    pub const fn fuchsia_imagepipe_surface(&self) -> bool {
        self.fuchsia_imagepipe_surface
    }
    ///Enables `VK_KHR_separate_depth_stencil_layouts` and it dependencies
    #[cfg(feature = "VK_KHR_separate_depth_stencil_layouts")]
    pub const fn enable_khr_separate_depth_stencil_layouts(mut self) -> Self {
        self = self
            .enable_khr_get_physical_device_properties_2()
            .enable_khr_create_renderpass_2();
        self.khr_separate_depth_stencil_layouts = true;
        self
    }
    ///Checks if `VK_KHR_separate_depth_stencil_layouts` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_KHR_separate_depth_stencil_layouts")]
    pub const fn khr_separate_depth_stencil_layouts(&self) -> bool {
        self.khr_separate_depth_stencil_layouts
    }
    ///Enables `VK_KHR_device_group_creation` and it dependencies
    #[cfg(feature = "VK_KHR_device_group_creation")]
    pub const fn enable_khr_device_group_creation(mut self) -> Self {
        self = self;
        self.khr_device_group_creation = true;
        self
    }
    ///Checks if `VK_KHR_device_group_creation` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_KHR_device_group_creation")]
    pub const fn khr_device_group_creation(&self) -> bool {
        self.khr_device_group_creation
    }
    ///Enables `VK_NV_fragment_coverage_to_color` and it dependencies
    #[cfg(feature = "VK_NV_fragment_coverage_to_color")]
    pub const fn enable_nv_fragment_coverage_to_color(mut self) -> Self {
        self = self;
        self.nv_fragment_coverage_to_color = true;
        self
    }
    ///Checks if `VK_NV_fragment_coverage_to_color` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_NV_fragment_coverage_to_color")]
    pub const fn nv_fragment_coverage_to_color(&self) -> bool {
        self.nv_fragment_coverage_to_color
    }
    ///Enables `VK_GOOGLE_decorate_string` and it dependencies
    #[cfg(feature = "VK_GOOGLE_decorate_string")]
    pub const fn enable_google_decorate_string(mut self) -> Self {
        self = self;
        self.google_decorate_string = true;
        self
    }
    ///Checks if `VK_GOOGLE_decorate_string` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_GOOGLE_decorate_string")]
    pub const fn google_decorate_string(&self) -> bool {
        self.google_decorate_string
    }
    ///Enables `VK_NV_geometry_shader_passthrough` and it dependencies
    #[cfg(feature = "VK_NV_geometry_shader_passthrough")]
    pub const fn enable_nv_geometry_shader_passthrough(mut self) -> Self {
        self = self;
        self.nv_geometry_shader_passthrough = true;
        self
    }
    ///Checks if `VK_NV_geometry_shader_passthrough` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_NV_geometry_shader_passthrough")]
    pub const fn nv_geometry_shader_passthrough(&self) -> bool {
        self.nv_geometry_shader_passthrough
    }
    ///Enables `VK_NVX_binary_import` and it dependencies
    #[cfg(feature = "VK_NVX_binary_import")]
    pub const fn enable_nvx_binary_import(mut self) -> Self {
        self = self;
        self.nvx_binary_import = true;
        self
    }
    ///Checks if `VK_NVX_binary_import` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_NVX_binary_import")]
    pub const fn nvx_binary_import(&self) -> bool {
        self.nvx_binary_import
    }
    ///Enables `VK_KHR_shader_subgroup_extended_types` and it dependencies
    #[cfg(feature = "VK_KHR_shader_subgroup_extended_types")]
    pub const fn enable_khr_shader_subgroup_extended_types(mut self) -> Self {
        self = self;
        self.khr_shader_subgroup_extended_types = true;
        self
    }
    ///Checks if `VK_KHR_shader_subgroup_extended_types` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_KHR_shader_subgroup_extended_types")]
    pub const fn khr_shader_subgroup_extended_types(&self) -> bool {
        self.khr_shader_subgroup_extended_types
    }
    ///Enables `VK_EXT_transform_feedback` and it dependencies
    #[cfg(feature = "VK_EXT_transform_feedback")]
    pub const fn enable_ext_transform_feedback(mut self) -> Self {
        self = self.enable_khr_get_physical_device_properties_2();
        self.ext_transform_feedback = true;
        self
    }
    ///Checks if `VK_EXT_transform_feedback` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_EXT_transform_feedback")]
    pub const fn ext_transform_feedback(&self) -> bool {
        self.ext_transform_feedback
    }
    ///Enables `VK_EXT_rgba10x6_formats` and it dependencies
    #[cfg(feature = "VK_EXT_rgba10x6_formats")]
    pub const fn enable_ext_rgba_10_x_6_formats(mut self) -> Self {
        self = self.enable_khr_sampler_ycbcr_conversion();
        self.ext_rgba_10_x_6_formats = true;
        self
    }
    ///Checks if `VK_EXT_rgba10x6_formats` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_EXT_rgba10x6_formats")]
    pub const fn ext_rgba_10_x_6_formats(&self) -> bool {
        self.ext_rgba_10_x_6_formats
    }
    ///Enables `VK_KHR_draw_indirect_count` and it dependencies
    #[cfg(feature = "VK_KHR_draw_indirect_count")]
    pub const fn enable_khr_draw_indirect_count(mut self) -> Self {
        self = self;
        self.khr_draw_indirect_count = true;
        self
    }
    ///Checks if `VK_KHR_draw_indirect_count` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_KHR_draw_indirect_count")]
    pub const fn khr_draw_indirect_count(&self) -> bool {
        self.khr_draw_indirect_count
    }
    ///Enables `VK_GOOGLE_user_type` and it dependencies
    #[cfg(feature = "VK_GOOGLE_user_type")]
    pub const fn enable_google_user_type(mut self) -> Self {
        self = self;
        self.google_user_type = true;
        self
    }
    ///Checks if `VK_GOOGLE_user_type` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_GOOGLE_user_type")]
    pub const fn google_user_type(&self) -> bool {
        self.google_user_type
    }
    ///Enables `VK_KHR_shader_subgroup_uniform_control_flow` and it dependencies
    #[cfg(feature = "VK_KHR_shader_subgroup_uniform_control_flow")]
    pub const fn enable_khr_shader_subgroup_uniform_control_flow(mut self) -> Self {
        self = self;
        self.khr_shader_subgroup_uniform_control_flow = true;
        self
    }
    ///Checks if `VK_KHR_shader_subgroup_uniform_control_flow` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_KHR_shader_subgroup_uniform_control_flow")]
    pub const fn khr_shader_subgroup_uniform_control_flow(&self) -> bool {
        self.khr_shader_subgroup_uniform_control_flow
    }
    ///Enables `VK_ANDROID_external_memory_android_hardware_buffer` and it dependencies
    #[cfg(feature = "VK_ANDROID_external_memory_android_hardware_buffer")]
    pub const fn enable_android_external_memory_android_hardware_buffer(mut self) -> Self {
        self = self
            .enable_khr_sampler_ycbcr_conversion()
            .enable_khr_external_memory()
            .enable_ext_queue_family_foreign()
            .enable_khr_dedicated_allocation();
        self.android_external_memory_android_hardware_buffer = true;
        self
    }
    ///Checks if `VK_ANDROID_external_memory_android_hardware_buffer` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_ANDROID_external_memory_android_hardware_buffer")]
    pub const fn android_external_memory_android_hardware_buffer(&self) -> bool {
        self.android_external_memory_android_hardware_buffer
    }
    ///Enables `VK_NV_shading_rate_image` and it dependencies
    #[cfg(feature = "VK_NV_shading_rate_image")]
    pub const fn enable_nv_shading_rate_image(mut self) -> Self {
        self = self.enable_khr_get_physical_device_properties_2();
        self.nv_shading_rate_image = true;
        self
    }
    ///Checks if `VK_NV_shading_rate_image` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_NV_shading_rate_image")]
    pub const fn nv_shading_rate_image(&self) -> bool {
        self.nv_shading_rate_image
    }
    ///Enables `VK_EXT_astc_decode_mode` and it dependencies
    #[cfg(feature = "VK_EXT_astc_decode_mode")]
    pub const fn enable_ext_astc_decode_mode(mut self) -> Self {
        self = self.enable_khr_get_physical_device_properties_2();
        self.ext_astc_decode_mode = true;
        self
    }
    ///Checks if `VK_EXT_astc_decode_mode` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_EXT_astc_decode_mode")]
    pub const fn ext_astc_decode_mode(&self) -> bool {
        self.ext_astc_decode_mode
    }
    ///Enables `VK_KHR_get_surface_capabilities2` and it dependencies
    #[cfg(feature = "VK_KHR_get_surface_capabilities2")]
    pub const fn enable_khr_get_surface_capabilities_2(mut self) -> Self {
        self = self.enable_khr_surface();
        self.khr_get_surface_capabilities_2 = true;
        self
    }
    ///Checks if `VK_KHR_get_surface_capabilities2` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_KHR_get_surface_capabilities2")]
    pub const fn khr_get_surface_capabilities_2(&self) -> bool {
        self.khr_get_surface_capabilities_2
    }
    ///Enables `VK_AMD_shader_explicit_vertex_parameter` and it dependencies
    #[cfg(feature = "VK_AMD_shader_explicit_vertex_parameter")]
    pub const fn enable_amd_shader_explicit_vertex_parameter(mut self) -> Self {
        self = self;
        self.amd_shader_explicit_vertex_parameter = true;
        self
    }
    ///Checks if `VK_AMD_shader_explicit_vertex_parameter` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_AMD_shader_explicit_vertex_parameter")]
    pub const fn amd_shader_explicit_vertex_parameter(&self) -> bool {
        self.amd_shader_explicit_vertex_parameter
    }
    ///Enables `VK_KHR_device_group` and it dependencies
    #[cfg(feature = "VK_KHR_device_group")]
    pub const fn enable_khr_device_group(mut self) -> Self {
        self = self.enable_khr_device_group_creation();
        self.khr_device_group = true;
        self
    }
    ///Checks if `VK_KHR_device_group` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_KHR_device_group")]
    pub const fn khr_device_group(&self) -> bool {
        self.khr_device_group
    }
    ///Enables `VK_KHR_dedicated_allocation` and it dependencies
    #[cfg(feature = "VK_KHR_dedicated_allocation")]
    pub const fn enable_khr_dedicated_allocation(mut self) -> Self {
        self = self.enable_khr_get_memory_requirements_2();
        self.khr_dedicated_allocation = true;
        self
    }
    ///Checks if `VK_KHR_dedicated_allocation` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_KHR_dedicated_allocation")]
    pub const fn khr_dedicated_allocation(&self) -> bool {
        self.khr_dedicated_allocation
    }
    ///Enables `VK_IMG_format_pvrtc` and it dependencies
    #[cfg(feature = "VK_IMG_format_pvrtc")]
    pub const fn enable_img_format_pvrtc(mut self) -> Self {
        self = self;
        self.img_format_pvrtc = true;
        self
    }
    ///Checks if `VK_IMG_format_pvrtc` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_IMG_format_pvrtc")]
    pub const fn img_format_pvrtc(&self) -> bool {
        self.img_format_pvrtc
    }
    ///Enables `VK_KHR_ray_tracing_pipeline` and it dependencies
    #[cfg(feature = "VK_KHR_ray_tracing_pipeline")]
    pub const fn enable_khr_ray_tracing_pipeline(mut self) -> Self {
        self = self.enable_khr_spirv_1_4().enable_khr_acceleration_structure();
        self.khr_ray_tracing_pipeline = true;
        self
    }
    ///Checks if `VK_KHR_ray_tracing_pipeline` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_KHR_ray_tracing_pipeline")]
    pub const fn khr_ray_tracing_pipeline(&self) -> bool {
        self.khr_ray_tracing_pipeline
    }
    ///Enables `VK_QCOM_render_pass_shader_resolve` and it dependencies
    #[cfg(feature = "VK_QCOM_render_pass_shader_resolve")]
    pub const fn enable_qcom_render_pass_shader_resolve(mut self) -> Self {
        self = self;
        self.qcom_render_pass_shader_resolve = true;
        self
    }
    ///Checks if `VK_QCOM_render_pass_shader_resolve` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_QCOM_render_pass_shader_resolve")]
    pub const fn qcom_render_pass_shader_resolve(&self) -> bool {
        self.qcom_render_pass_shader_resolve
    }
    ///Enables `VK_KHR_workgroup_memory_explicit_layout` and it dependencies
    #[cfg(feature = "VK_KHR_workgroup_memory_explicit_layout")]
    pub const fn enable_khr_workgroup_memory_explicit_layout(mut self) -> Self {
        self = self.enable_khr_get_physical_device_properties_2();
        self.khr_workgroup_memory_explicit_layout = true;
        self
    }
    ///Checks if `VK_KHR_workgroup_memory_explicit_layout` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_KHR_workgroup_memory_explicit_layout")]
    pub const fn khr_workgroup_memory_explicit_layout(&self) -> bool {
        self.khr_workgroup_memory_explicit_layout
    }
    ///Enables `VK_EXT_vertex_input_dynamic_state` and it dependencies
    #[cfg(feature = "VK_EXT_vertex_input_dynamic_state")]
    pub const fn enable_ext_vertex_input_dynamic_state(mut self) -> Self {
        self = self.enable_khr_get_physical_device_properties_2();
        self.ext_vertex_input_dynamic_state = true;
        self
    }
    ///Checks if `VK_EXT_vertex_input_dynamic_state` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_EXT_vertex_input_dynamic_state")]
    pub const fn ext_vertex_input_dynamic_state(&self) -> bool {
        self.ext_vertex_input_dynamic_state
    }
    ///Enables `VK_KHR_sampler_ycbcr_conversion` and it dependencies
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    pub const fn enable_khr_sampler_ycbcr_conversion(mut self) -> Self {
        self = self
            .enable_khr_maintenance_1()
            .enable_khr_bind_memory_2()
            .enable_khr_get_memory_requirements_2()
            .enable_khr_get_physical_device_properties_2();
        self.khr_sampler_ycbcr_conversion = true;
        self
    }
    ///Checks if `VK_KHR_sampler_ycbcr_conversion` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    pub const fn khr_sampler_ycbcr_conversion(&self) -> bool {
        self.khr_sampler_ycbcr_conversion
    }
    ///Enables `VK_KHR_shader_atomic_int64` and it dependencies
    #[cfg(feature = "VK_KHR_shader_atomic_int64")]
    pub const fn enable_khr_shader_atomic_int_64(mut self) -> Self {
        self = self.enable_khr_get_physical_device_properties_2();
        self.khr_shader_atomic_int_64 = true;
        self
    }
    ///Checks if `VK_KHR_shader_atomic_int64` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_KHR_shader_atomic_int64")]
    pub const fn khr_shader_atomic_int_64(&self) -> bool {
        self.khr_shader_atomic_int_64
    }
    ///Enables `VK_KHR_display_swapchain` and it dependencies
    #[cfg(feature = "VK_KHR_display_swapchain")]
    pub const fn enable_khr_display_swapchain(mut self) -> Self {
        self = self.enable_khr_swapchain().enable_khr_display();
        self.khr_display_swapchain = true;
        self
    }
    ///Checks if `VK_KHR_display_swapchain` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_KHR_display_swapchain")]
    pub const fn khr_display_swapchain(&self) -> bool {
        self.khr_display_swapchain
    }
    ///Enables `VK_IMG_filter_cubic` and it dependencies
    #[cfg(feature = "VK_IMG_filter_cubic")]
    pub const fn enable_img_filter_cubic(mut self) -> Self {
        self = self;
        self.img_filter_cubic = true;
        self
    }
    ///Checks if `VK_IMG_filter_cubic` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_IMG_filter_cubic")]
    pub const fn img_filter_cubic(&self) -> bool {
        self.img_filter_cubic
    }
    ///Enables `VK_EXT_index_type_uint8` and it dependencies
    #[cfg(feature = "VK_EXT_index_type_uint8")]
    pub const fn enable_ext_index_type_uint_8(mut self) -> Self {
        self = self;
        self.ext_index_type_uint_8 = true;
        self
    }
    ///Checks if `VK_EXT_index_type_uint8` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_EXT_index_type_uint8")]
    pub const fn ext_index_type_uint_8(&self) -> bool {
        self.ext_index_type_uint_8
    }
    ///Enables `VK_AMD_memory_overallocation_behavior` and it dependencies
    #[cfg(feature = "VK_AMD_memory_overallocation_behavior")]
    pub const fn enable_amd_memory_overallocation_behavior(mut self) -> Self {
        self = self;
        self.amd_memory_overallocation_behavior = true;
        self
    }
    ///Checks if `VK_AMD_memory_overallocation_behavior` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_AMD_memory_overallocation_behavior")]
    pub const fn amd_memory_overallocation_behavior(&self) -> bool {
        self.amd_memory_overallocation_behavior
    }
    ///Enables `VK_EXT_shader_image_atomic_int64` and it dependencies
    #[cfg(feature = "VK_EXT_shader_image_atomic_int64")]
    pub const fn enable_ext_shader_image_atomic_int_64(mut self) -> Self {
        self = self.enable_khr_get_physical_device_properties_2();
        self.ext_shader_image_atomic_int_64 = true;
        self
    }
    ///Checks if `VK_EXT_shader_image_atomic_int64` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_EXT_shader_image_atomic_int64")]
    pub const fn ext_shader_image_atomic_int_64(&self) -> bool {
        self.ext_shader_image_atomic_int_64
    }
    ///Enables `VK_KHR_multiview` and it dependencies
    #[cfg(feature = "VK_KHR_multiview")]
    pub const fn enable_khr_multiview(mut self) -> Self {
        self = self.enable_khr_get_physical_device_properties_2();
        self.khr_multiview = true;
        self
    }
    ///Checks if `VK_KHR_multiview` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_KHR_multiview")]
    pub const fn khr_multiview(&self) -> bool {
        self.khr_multiview
    }
    ///Enables `VK_AMD_mixed_attachment_samples` and it dependencies
    #[cfg(feature = "VK_AMD_mixed_attachment_samples")]
    pub const fn enable_amd_mixed_attachment_samples(mut self) -> Self {
        self = self;
        self.amd_mixed_attachment_samples = true;
        self
    }
    ///Checks if `VK_AMD_mixed_attachment_samples` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_AMD_mixed_attachment_samples")]
    pub const fn amd_mixed_attachment_samples(&self) -> bool {
        self.amd_mixed_attachment_samples
    }
    ///Enables `VK_KHR_uniform_buffer_standard_layout` and it dependencies
    #[cfg(feature = "VK_KHR_uniform_buffer_standard_layout")]
    pub const fn enable_khr_uniform_buffer_standard_layout(mut self) -> Self {
        self = self.enable_khr_get_physical_device_properties_2();
        self.khr_uniform_buffer_standard_layout = true;
        self
    }
    ///Checks if `VK_KHR_uniform_buffer_standard_layout` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_KHR_uniform_buffer_standard_layout")]
    pub const fn khr_uniform_buffer_standard_layout(&self) -> bool {
        self.khr_uniform_buffer_standard_layout
    }
    ///Enables `VK_EXT_ycbcr_2plane_444_formats` and it dependencies
    #[cfg(feature = "VK_EXT_ycbcr_2plane_444_formats")]
    pub const fn enable_ext_ycbcr_2_plane_444_formats(mut self) -> Self {
        self = self.enable_khr_sampler_ycbcr_conversion();
        self.ext_ycbcr_2_plane_444_formats = true;
        self
    }
    ///Checks if `VK_EXT_ycbcr_2plane_444_formats` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_EXT_ycbcr_2plane_444_formats")]
    pub const fn ext_ycbcr_2_plane_444_formats(&self) -> bool {
        self.ext_ycbcr_2_plane_444_formats
    }
    ///Enables `VK_EXT_display_surface_counter` and it dependencies
    #[cfg(feature = "VK_EXT_display_surface_counter")]
    pub const fn enable_ext_display_surface_counter(mut self) -> Self {
        self = self.enable_khr_display();
        self.ext_display_surface_counter = true;
        self
    }
    ///Checks if `VK_EXT_display_surface_counter` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_EXT_display_surface_counter")]
    pub const fn ext_display_surface_counter(&self) -> bool {
        self.ext_display_surface_counter
    }
    ///Enables `VK_KHR_shader_float16_int8` and it dependencies
    #[cfg(feature = "VK_KHR_shader_float16_int8")]
    pub const fn enable_khr_shader_float_16_int_8(mut self) -> Self {
        self = self.enable_khr_get_physical_device_properties_2();
        self.khr_shader_float_16_int_8 = true;
        self
    }
    ///Checks if `VK_KHR_shader_float16_int8` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_KHR_shader_float16_int8")]
    pub const fn khr_shader_float_16_int_8(&self) -> bool {
        self.khr_shader_float_16_int_8
    }
    ///Enables `VK_FUCHSIA_external_memory` and it dependencies
    #[cfg(feature = "VK_FUCHSIA_external_memory")]
    pub const fn enable_fuchsia_external_memory(mut self) -> Self {
        self = self
            .enable_khr_external_memory_capabilities()
            .enable_khr_external_memory();
        self.fuchsia_external_memory = true;
        self
    }
    ///Checks if `VK_FUCHSIA_external_memory` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_FUCHSIA_external_memory")]
    pub const fn fuchsia_external_memory(&self) -> bool {
        self.fuchsia_external_memory
    }
    ///Enables `VK_KHR_display` and it dependencies
    #[cfg(feature = "VK_KHR_display")]
    pub const fn enable_khr_display(mut self) -> Self {
        self = self.enable_khr_surface();
        self.khr_display = true;
        self
    }
    ///Checks if `VK_KHR_display` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_KHR_display")]
    pub const fn khr_display(&self) -> bool {
        self.khr_display
    }
    ///Enables `VK_KHR_driver_properties` and it dependencies
    #[cfg(feature = "VK_KHR_driver_properties")]
    pub const fn enable_khr_driver_properties(mut self) -> Self {
        self = self.enable_khr_get_physical_device_properties_2();
        self.khr_driver_properties = true;
        self
    }
    ///Checks if `VK_KHR_driver_properties` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_KHR_driver_properties")]
    pub const fn khr_driver_properties(&self) -> bool {
        self.khr_driver_properties
    }
    ///Enables `VK_EXT_filter_cubic` and it dependencies
    #[cfg(feature = "VK_EXT_filter_cubic")]
    pub const fn enable_ext_filter_cubic(mut self) -> Self {
        self = self;
        self.ext_filter_cubic = true;
        self
    }
    ///Checks if `VK_EXT_filter_cubic` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_EXT_filter_cubic")]
    pub const fn ext_filter_cubic(&self) -> bool {
        self.ext_filter_cubic
    }
    ///Enables `VK_KHR_maintenance1` and it dependencies
    #[cfg(feature = "VK_KHR_maintenance1")]
    pub const fn enable_khr_maintenance_1(mut self) -> Self {
        self = self;
        self.khr_maintenance_1 = true;
        self
    }
    ///Checks if `VK_KHR_maintenance1` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_KHR_maintenance1")]
    pub const fn khr_maintenance_1(&self) -> bool {
        self.khr_maintenance_1
    }
    ///Enables `VK_KHR_spirv_1_4` and it dependencies
    #[cfg(feature = "VK_KHR_spirv_1_4")]
    pub const fn enable_khr_spirv_1_4(mut self) -> Self {
        self = self.enable_khr_shader_float_controls();
        self.khr_spirv_1_4 = true;
        self
    }
    ///Checks if `VK_KHR_spirv_1_4` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_KHR_spirv_1_4")]
    pub const fn khr_spirv_1_4(&self) -> bool {
        self.khr_spirv_1_4
    }
    ///Enables `VK_AMD_texture_gather_bias_lod` and it dependencies
    #[cfg(feature = "VK_AMD_texture_gather_bias_lod")]
    pub const fn enable_amd_texture_gather_bias_lod(mut self) -> Self {
        self = self.enable_khr_get_physical_device_properties_2();
        self.amd_texture_gather_bias_lod = true;
        self
    }
    ///Checks if `VK_AMD_texture_gather_bias_lod` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_AMD_texture_gather_bias_lod")]
    pub const fn amd_texture_gather_bias_lod(&self) -> bool {
        self.amd_texture_gather_bias_lod
    }
    ///Enables `VK_EXT_extended_dynamic_state` and it dependencies
    #[cfg(feature = "VK_EXT_extended_dynamic_state")]
    pub const fn enable_ext_extended_dynamic_state(mut self) -> Self {
        self = self.enable_khr_get_physical_device_properties_2();
        self.ext_extended_dynamic_state = true;
        self
    }
    ///Checks if `VK_EXT_extended_dynamic_state` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_EXT_extended_dynamic_state")]
    pub const fn ext_extended_dynamic_state(&self) -> bool {
        self.ext_extended_dynamic_state
    }
    ///Enables `VK_EXT_conservative_rasterization` and it dependencies
    #[cfg(feature = "VK_EXT_conservative_rasterization")]
    pub const fn enable_ext_conservative_rasterization(mut self) -> Self {
        self = self.enable_khr_get_physical_device_properties_2();
        self.ext_conservative_rasterization = true;
        self
    }
    ///Checks if `VK_EXT_conservative_rasterization` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_EXT_conservative_rasterization")]
    pub const fn ext_conservative_rasterization(&self) -> bool {
        self.ext_conservative_rasterization
    }
    ///Enables `VK_KHR_surface_protected_capabilities` and it dependencies
    #[cfg(feature = "VK_KHR_surface_protected_capabilities")]
    pub const fn enable_khr_surface_protected_capabilities(mut self) -> Self {
        self = self.enable_khr_get_surface_capabilities_2();
        self.khr_surface_protected_capabilities = true;
        self
    }
    ///Checks if `VK_KHR_surface_protected_capabilities` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_KHR_surface_protected_capabilities")]
    pub const fn khr_surface_protected_capabilities(&self) -> bool {
        self.khr_surface_protected_capabilities
    }
    ///Enables `VK_EXT_primitive_topology_list_restart` and it dependencies
    #[cfg(feature = "VK_EXT_primitive_topology_list_restart")]
    pub const fn enable_ext_primitive_topology_list_restart(mut self) -> Self {
        self = self;
        self.ext_primitive_topology_list_restart = true;
        self
    }
    ///Checks if `VK_EXT_primitive_topology_list_restart` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_EXT_primitive_topology_list_restart")]
    pub const fn ext_primitive_topology_list_restart(&self) -> bool {
        self.ext_primitive_topology_list_restart
    }
    ///Enables `VK_KHR_8bit_storage` and it dependencies
    #[cfg(feature = "VK_KHR_8bit_storage")]
    pub const fn enable_khr_8_bit_storage(mut self) -> Self {
        self = self
            .enable_khr_get_physical_device_properties_2()
            .enable_khr_storage_buffer_storage_class();
        self.khr_8_bit_storage = true;
        self
    }
    ///Checks if `VK_KHR_8bit_storage` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_KHR_8bit_storage")]
    pub const fn khr_8_bit_storage(&self) -> bool {
        self.khr_8_bit_storage
    }
    ///Enables `VK_EXT_border_color_swizzle` and it dependencies
    #[cfg(feature = "VK_EXT_border_color_swizzle")]
    pub const fn enable_ext_border_color_swizzle(mut self) -> Self {
        self = self.enable_ext_custom_border_color();
        self.ext_border_color_swizzle = true;
        self
    }
    ///Checks if `VK_EXT_border_color_swizzle` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_EXT_border_color_swizzle")]
    pub const fn ext_border_color_swizzle(&self) -> bool {
        self.ext_border_color_swizzle
    }
    ///Enables `VK_EXT_calibrated_timestamps` and it dependencies
    #[cfg(feature = "VK_EXT_calibrated_timestamps")]
    pub const fn enable_ext_calibrated_timestamps(mut self) -> Self {
        self = self.enable_khr_get_physical_device_properties_2();
        self.ext_calibrated_timestamps = true;
        self
    }
    ///Checks if `VK_EXT_calibrated_timestamps` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_EXT_calibrated_timestamps")]
    pub const fn ext_calibrated_timestamps(&self) -> bool {
        self.ext_calibrated_timestamps
    }
    ///Enables `VK_EXT_depth_range_unrestricted` and it dependencies
    #[cfg(feature = "VK_EXT_depth_range_unrestricted")]
    pub const fn enable_ext_depth_range_unrestricted(mut self) -> Self {
        self = self;
        self.ext_depth_range_unrestricted = true;
        self
    }
    ///Checks if `VK_EXT_depth_range_unrestricted` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_EXT_depth_range_unrestricted")]
    pub const fn ext_depth_range_unrestricted(&self) -> bool {
        self.ext_depth_range_unrestricted
    }
    ///Enables `VK_NV_shader_subgroup_partitioned` and it dependencies
    #[cfg(feature = "VK_NV_shader_subgroup_partitioned")]
    pub const fn enable_nv_shader_subgroup_partitioned(mut self) -> Self {
        self = self;
        self.nv_shader_subgroup_partitioned = true;
        self
    }
    ///Checks if `VK_NV_shader_subgroup_partitioned` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_NV_shader_subgroup_partitioned")]
    pub const fn nv_shader_subgroup_partitioned(&self) -> bool {
        self.nv_shader_subgroup_partitioned
    }
    ///Enables `VK_GOOGLE_hlsl_functionality1` and it dependencies
    #[cfg(feature = "VK_GOOGLE_hlsl_functionality1")]
    pub const fn enable_google_hlsl_functionality_1(mut self) -> Self {
        self = self;
        self.google_hlsl_functionality_1 = true;
        self
    }
    ///Checks if `VK_GOOGLE_hlsl_functionality1` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_GOOGLE_hlsl_functionality1")]
    pub const fn google_hlsl_functionality_1(&self) -> bool {
        self.google_hlsl_functionality_1
    }
    ///Enables `VK_EXT_memory_priority` and it dependencies
    #[cfg(feature = "VK_EXT_memory_priority")]
    pub const fn enable_ext_memory_priority(mut self) -> Self {
        self = self.enable_khr_get_physical_device_properties_2();
        self.ext_memory_priority = true;
        self
    }
    ///Checks if `VK_EXT_memory_priority` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_EXT_memory_priority")]
    pub const fn ext_memory_priority(&self) -> bool {
        self.ext_memory_priority
    }
    ///Enables `VK_KHR_external_semaphore_fd` and it dependencies
    #[cfg(feature = "VK_KHR_external_semaphore_fd")]
    pub const fn enable_khr_external_semaphore_fd(mut self) -> Self {
        self = self.enable_khr_external_semaphore();
        self.khr_external_semaphore_fd = true;
        self
    }
    ///Checks if `VK_KHR_external_semaphore_fd` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_KHR_external_semaphore_fd")]
    pub const fn khr_external_semaphore_fd(&self) -> bool {
        self.khr_external_semaphore_fd
    }
    ///Enables `VK_KHR_get_physical_device_properties2` and it dependencies
    #[cfg(feature = "VK_KHR_get_physical_device_properties2")]
    pub const fn enable_khr_get_physical_device_properties_2(mut self) -> Self {
        self = self;
        self.khr_get_physical_device_properties_2 = true;
        self
    }
    ///Checks if `VK_KHR_get_physical_device_properties2` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_KHR_get_physical_device_properties2")]
    pub const fn khr_get_physical_device_properties_2(&self) -> bool {
        self.khr_get_physical_device_properties_2
    }
    ///Enables `VK_EXT_swapchain_colorspace` and it dependencies
    #[cfg(feature = "VK_EXT_swapchain_colorspace")]
    pub const fn enable_ext_swapchain_colorspace(mut self) -> Self {
        self = self.enable_khr_surface();
        self.ext_swapchain_colorspace = true;
        self
    }
    ///Checks if `VK_EXT_swapchain_colorspace` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_EXT_swapchain_colorspace")]
    pub const fn ext_swapchain_colorspace(&self) -> bool {
        self.ext_swapchain_colorspace
    }
    ///Enables `VK_GOOGLE_display_timing` and it dependencies
    #[cfg(feature = "VK_GOOGLE_display_timing")]
    pub const fn enable_google_display_timing(mut self) -> Self {
        self = self.enable_khr_swapchain();
        self.google_display_timing = true;
        self
    }
    ///Checks if `VK_GOOGLE_display_timing` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_GOOGLE_display_timing")]
    pub const fn google_display_timing(&self) -> bool {
        self.google_display_timing
    }
    ///Enables `VK_NV_mesh_shader` and it dependencies
    #[cfg(feature = "VK_NV_mesh_shader")]
    pub const fn enable_nv_mesh_shader(mut self) -> Self {
        self = self.enable_khr_get_physical_device_properties_2();
        self.nv_mesh_shader = true;
        self
    }
    ///Checks if `VK_NV_mesh_shader` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_NV_mesh_shader")]
    pub const fn nv_mesh_shader(&self) -> bool {
        self.nv_mesh_shader
    }
    ///Enables `VK_KHR_synchronization2` and it dependencies
    #[cfg(feature = "VK_KHR_synchronization2")]
    pub const fn enable_khr_synchronization_2(mut self) -> Self {
        self = self.enable_khr_get_physical_device_properties_2();
        self.khr_synchronization_2 = true;
        self
    }
    ///Checks if `VK_KHR_synchronization2` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_KHR_synchronization2")]
    pub const fn khr_synchronization_2(&self) -> bool {
        self.khr_synchronization_2
    }
    ///Enables `VK_EXT_shader_subgroup_vote` and it dependencies
    #[cfg(feature = "VK_EXT_shader_subgroup_vote")]
    pub const fn enable_ext_shader_subgroup_vote(mut self) -> Self {
        self = self;
        self.ext_shader_subgroup_vote = true;
        self
    }
    ///Checks if `VK_EXT_shader_subgroup_vote` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_EXT_shader_subgroup_vote")]
    pub const fn ext_shader_subgroup_vote(&self) -> bool {
        self.ext_shader_subgroup_vote
    }
    ///Enables `VK_EXT_fragment_shader_interlock` and it dependencies
    #[cfg(feature = "VK_EXT_fragment_shader_interlock")]
    pub const fn enable_ext_fragment_shader_interlock(mut self) -> Self {
        self = self.enable_khr_get_physical_device_properties_2();
        self.ext_fragment_shader_interlock = true;
        self
    }
    ///Checks if `VK_EXT_fragment_shader_interlock` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_EXT_fragment_shader_interlock")]
    pub const fn ext_fragment_shader_interlock(&self) -> bool {
        self.ext_fragment_shader_interlock
    }
    ///Enables `VK_EXT_directfb_surface` and it dependencies
    #[cfg(feature = "VK_EXT_directfb_surface")]
    pub const fn enable_ext_directfb_surface(mut self) -> Self {
        self = self.enable_khr_surface();
        self.ext_directfb_surface = true;
        self
    }
    ///Checks if `VK_EXT_directfb_surface` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_EXT_directfb_surface")]
    pub const fn ext_directfb_surface(&self) -> bool {
        self.ext_directfb_surface
    }
    ///Enables `VK_EXT_blend_operation_advanced` and it dependencies
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    pub const fn enable_ext_blend_operation_advanced(mut self) -> Self {
        self = self;
        self.ext_blend_operation_advanced = true;
        self
    }
    ///Checks if `VK_EXT_blend_operation_advanced` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    pub const fn ext_blend_operation_advanced(&self) -> bool {
        self.ext_blend_operation_advanced
    }
    ///Enables `VK_EXT_descriptor_indexing` and it dependencies
    #[cfg(feature = "VK_EXT_descriptor_indexing")]
    pub const fn enable_ext_descriptor_indexing(mut self) -> Self {
        self = self
            .enable_khr_get_physical_device_properties_2()
            .enable_khr_maintenance_3();
        self.ext_descriptor_indexing = true;
        self
    }
    ///Checks if `VK_EXT_descriptor_indexing` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_EXT_descriptor_indexing")]
    pub const fn ext_descriptor_indexing(&self) -> bool {
        self.ext_descriptor_indexing
    }
    ///Enables `VK_EXT_inline_uniform_block` and it dependencies
    #[cfg(feature = "VK_EXT_inline_uniform_block")]
    pub const fn enable_ext_inline_uniform_block(mut self) -> Self {
        self = self
            .enable_khr_get_physical_device_properties_2()
            .enable_khr_maintenance_1();
        self.ext_inline_uniform_block = true;
        self
    }
    ///Checks if `VK_EXT_inline_uniform_block` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_EXT_inline_uniform_block")]
    pub const fn ext_inline_uniform_block(&self) -> bool {
        self.ext_inline_uniform_block
    }
    ///Enables `VK_KHR_video_encode_queue` and it dependencies
    #[cfg(feature = "VK_KHR_video_encode_queue")]
    pub const fn enable_khr_video_encode_queue(mut self) -> Self {
        self = self.enable_khr_video_queue().enable_khr_synchronization_2();
        self.khr_video_encode_queue = true;
        self
    }
    ///Checks if `VK_KHR_video_encode_queue` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_KHR_video_encode_queue")]
    pub const fn khr_video_encode_queue(&self) -> bool {
        self.khr_video_encode_queue
    }
    ///Enables `VK_EXT_fragment_density_map2` and it dependencies
    #[cfg(feature = "VK_EXT_fragment_density_map2")]
    pub const fn enable_ext_fragment_density_map_2(mut self) -> Self {
        self = self.enable_ext_fragment_density_map();
        self.ext_fragment_density_map_2 = true;
        self
    }
    ///Checks if `VK_EXT_fragment_density_map2` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_EXT_fragment_density_map2")]
    pub const fn ext_fragment_density_map_2(&self) -> bool {
        self.ext_fragment_density_map_2
    }
    ///Enables `VK_EXT_debug_marker` and it dependencies
    #[cfg(feature = "VK_EXT_debug_marker")]
    pub const fn enable_ext_debug_marker(mut self) -> Self {
        self = self.enable_ext_debug_report();
        self.ext_debug_marker = true;
        self
    }
    ///Checks if `VK_EXT_debug_marker` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_EXT_debug_marker")]
    pub const fn ext_debug_marker(&self) -> bool {
        self.ext_debug_marker
    }
    ///Enables `VK_EXT_physical_device_drm` and it dependencies
    #[cfg(feature = "VK_EXT_physical_device_drm")]
    pub const fn enable_ext_physical_device_drm(mut self) -> Self {
        self = self.enable_khr_get_physical_device_properties_2();
        self.ext_physical_device_drm = true;
        self
    }
    ///Checks if `VK_EXT_physical_device_drm` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_EXT_physical_device_drm")]
    pub const fn ext_physical_device_drm(&self) -> bool {
        self.ext_physical_device_drm
    }
    ///Enables `VK_NV_coverage_reduction_mode` and it dependencies
    #[cfg(feature = "VK_NV_coverage_reduction_mode")]
    pub const fn enable_nv_coverage_reduction_mode(mut self) -> Self {
        self = self.enable_nv_framebuffer_mixed_samples();
        self.nv_coverage_reduction_mode = true;
        self
    }
    ///Checks if `VK_NV_coverage_reduction_mode` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_NV_coverage_reduction_mode")]
    pub const fn nv_coverage_reduction_mode(&self) -> bool {
        self.nv_coverage_reduction_mode
    }
    ///Enables `VK_KHR_shader_non_semantic_info` and it dependencies
    #[cfg(feature = "VK_KHR_shader_non_semantic_info")]
    pub const fn enable_khr_shader_non_semantic_info(mut self) -> Self {
        self = self;
        self.khr_shader_non_semantic_info = true;
        self
    }
    ///Checks if `VK_KHR_shader_non_semantic_info` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_KHR_shader_non_semantic_info")]
    pub const fn khr_shader_non_semantic_info(&self) -> bool {
        self.khr_shader_non_semantic_info
    }
    ///Enables `VK_ARM_rasterization_order_attachment_access` and it dependencies
    #[cfg(feature = "VK_ARM_rasterization_order_attachment_access")]
    pub const fn enable_arm_rasterization_order_attachment_access(mut self) -> Self {
        self = self.enable_khr_get_physical_device_properties_2();
        self.arm_rasterization_order_attachment_access = true;
        self
    }
    ///Checks if `VK_ARM_rasterization_order_attachment_access` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_ARM_rasterization_order_attachment_access")]
    pub const fn arm_rasterization_order_attachment_access(&self) -> bool {
        self.arm_rasterization_order_attachment_access
    }
    ///Enables `VK_KHR_shader_draw_parameters` and it dependencies
    #[cfg(feature = "VK_KHR_shader_draw_parameters")]
    pub const fn enable_khr_shader_draw_parameters(mut self) -> Self {
        self = self;
        self.khr_shader_draw_parameters = true;
        self
    }
    ///Checks if `VK_KHR_shader_draw_parameters` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_KHR_shader_draw_parameters")]
    pub const fn khr_shader_draw_parameters(&self) -> bool {
        self.khr_shader_draw_parameters
    }
    ///Enables `VK_EXT_texture_compression_astc_hdr` and it dependencies
    #[cfg(feature = "VK_EXT_texture_compression_astc_hdr")]
    pub const fn enable_ext_texture_compression_astc_hdr(mut self) -> Self {
        self = self.enable_khr_get_physical_device_properties_2();
        self.ext_texture_compression_astc_hdr = true;
        self
    }
    ///Checks if `VK_EXT_texture_compression_astc_hdr` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_EXT_texture_compression_astc_hdr")]
    pub const fn ext_texture_compression_astc_hdr(&self) -> bool {
        self.ext_texture_compression_astc_hdr
    }
    ///Enables `VK_AMD_gpu_shader_half_float` and it dependencies
    #[cfg(feature = "VK_AMD_gpu_shader_half_float")]
    pub const fn enable_amd_gpu_shader_half_float(mut self) -> Self {
        self = self;
        self.amd_gpu_shader_half_float = true;
        self
    }
    ///Checks if `VK_AMD_gpu_shader_half_float` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_AMD_gpu_shader_half_float")]
    pub const fn amd_gpu_shader_half_float(&self) -> bool {
        self.amd_gpu_shader_half_float
    }
    ///Enables `VK_NV_device_diagnostic_checkpoints` and it dependencies
    #[cfg(feature = "VK_NV_device_diagnostic_checkpoints")]
    pub const fn enable_nv_device_diagnostic_checkpoints(mut self) -> Self {
        self = self.enable_khr_get_physical_device_properties_2();
        self.nv_device_diagnostic_checkpoints = true;
        self
    }
    ///Checks if `VK_NV_device_diagnostic_checkpoints` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_NV_device_diagnostic_checkpoints")]
    pub const fn nv_device_diagnostic_checkpoints(&self) -> bool {
        self.nv_device_diagnostic_checkpoints
    }
    ///Enables `VK_KHR_video_decode_queue` and it dependencies
    #[cfg(feature = "VK_KHR_video_decode_queue")]
    pub const fn enable_khr_video_decode_queue(mut self) -> Self {
        self = self.enable_khr_video_queue().enable_khr_synchronization_2();
        self.khr_video_decode_queue = true;
        self
    }
    ///Checks if `VK_KHR_video_decode_queue` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_KHR_video_decode_queue")]
    pub const fn khr_video_decode_queue(&self) -> bool {
        self.khr_video_decode_queue
    }
    ///Enables `VK_KHR_deferred_host_operations` and it dependencies
    #[cfg(feature = "VK_KHR_deferred_host_operations")]
    pub const fn enable_khr_deferred_host_operations(mut self) -> Self {
        self = self;
        self.khr_deferred_host_operations = true;
        self
    }
    ///Checks if `VK_KHR_deferred_host_operations` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_KHR_deferred_host_operations")]
    pub const fn khr_deferred_host_operations(&self) -> bool {
        self.khr_deferred_host_operations
    }
    ///Enables `VK_KHR_sampler_mirror_clamp_to_edge` and it dependencies
    #[cfg(feature = "VK_KHR_sampler_mirror_clamp_to_edge")]
    pub const fn enable_khr_sampler_mirror_clamp_to_edge(mut self) -> Self {
        self = self;
        self.khr_sampler_mirror_clamp_to_edge = true;
        self
    }
    ///Checks if `VK_KHR_sampler_mirror_clamp_to_edge` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_KHR_sampler_mirror_clamp_to_edge")]
    pub const fn khr_sampler_mirror_clamp_to_edge(&self) -> bool {
        self.khr_sampler_mirror_clamp_to_edge
    }
    ///Enables `VK_KHR_external_semaphore` and it dependencies
    #[cfg(feature = "VK_KHR_external_semaphore")]
    pub const fn enable_khr_external_semaphore(mut self) -> Self {
        self = self.enable_khr_external_semaphore_capabilities();
        self.khr_external_semaphore = true;
        self
    }
    ///Checks if `VK_KHR_external_semaphore` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_KHR_external_semaphore")]
    pub const fn khr_external_semaphore(&self) -> bool {
        self.khr_external_semaphore
    }
    ///Enables `VK_NV_viewport_array2` and it dependencies
    #[cfg(feature = "VK_NV_viewport_array2")]
    pub const fn enable_nv_viewport_array_2(mut self) -> Self {
        self = self;
        self.nv_viewport_array_2 = true;
        self
    }
    ///Checks if `VK_NV_viewport_array2` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_NV_viewport_array2")]
    pub const fn nv_viewport_array_2(&self) -> bool {
        self.nv_viewport_array_2
    }
    ///Enables `VK_KHR_external_fence_win32` and it dependencies
    #[cfg(feature = "VK_KHR_external_fence_win32")]
    pub const fn enable_khr_external_fence_win_32(mut self) -> Self {
        self = self.enable_khr_external_fence();
        self.khr_external_fence_win_32 = true;
        self
    }
    ///Checks if `VK_KHR_external_fence_win32` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_KHR_external_fence_win32")]
    pub const fn khr_external_fence_win_32(&self) -> bool {
        self.khr_external_fence_win_32
    }
    ///Enables `VK_EXT_ycbcr_image_arrays` and it dependencies
    #[cfg(feature = "VK_EXT_ycbcr_image_arrays")]
    pub const fn enable_ext_ycbcr_image_arrays(mut self) -> Self {
        self = self.enable_khr_sampler_ycbcr_conversion();
        self.ext_ycbcr_image_arrays = true;
        self
    }
    ///Checks if `VK_EXT_ycbcr_image_arrays` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_EXT_ycbcr_image_arrays")]
    pub const fn ext_ycbcr_image_arrays(&self) -> bool {
        self.ext_ycbcr_image_arrays
    }
    ///Enables `VK_KHR_buffer_device_address` and it dependencies
    #[cfg(feature = "VK_KHR_buffer_device_address")]
    pub const fn enable_khr_buffer_device_address(mut self) -> Self {
        self = self.enable_khr_get_physical_device_properties_2();
        self.khr_buffer_device_address = true;
        self
    }
    ///Checks if `VK_KHR_buffer_device_address` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_KHR_buffer_device_address")]
    pub const fn khr_buffer_device_address(&self) -> bool {
        self.khr_buffer_device_address
    }
    ///Enables `VK_AMD_pipeline_compiler_control` and it dependencies
    #[cfg(feature = "VK_AMD_pipeline_compiler_control")]
    pub const fn enable_amd_pipeline_compiler_control(mut self) -> Self {
        self = self;
        self.amd_pipeline_compiler_control = true;
        self
    }
    ///Checks if `VK_AMD_pipeline_compiler_control` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_AMD_pipeline_compiler_control")]
    pub const fn amd_pipeline_compiler_control(&self) -> bool {
        self.amd_pipeline_compiler_control
    }
    ///Enables `VK_KHR_swapchain_mutable_format` and it dependencies
    #[cfg(feature = "VK_KHR_swapchain_mutable_format")]
    pub const fn enable_khr_swapchain_mutable_format(mut self) -> Self {
        self = self
            .enable_khr_swapchain()
            .enable_khr_maintenance_2()
            .enable_khr_image_format_list();
        self.khr_swapchain_mutable_format = true;
        self
    }
    ///Checks if `VK_KHR_swapchain_mutable_format` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_KHR_swapchain_mutable_format")]
    pub const fn khr_swapchain_mutable_format(&self) -> bool {
        self.khr_swapchain_mutable_format
    }
    ///Enables `VK_QCOM_rotated_copy_commands` and it dependencies
    #[cfg(feature = "VK_QCOM_rotated_copy_commands")]
    pub const fn enable_qcom_rotated_copy_commands(mut self) -> Self {
        self = self.enable_khr_swapchain().enable_khr_copy_commands_2();
        self.qcom_rotated_copy_commands = true;
        self
    }
    ///Checks if `VK_QCOM_rotated_copy_commands` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_QCOM_rotated_copy_commands")]
    pub const fn qcom_rotated_copy_commands(&self) -> bool {
        self.qcom_rotated_copy_commands
    }
    ///Enables `VK_EXT_color_write_enable` and it dependencies
    #[cfg(feature = "VK_EXT_color_write_enable")]
    pub const fn enable_ext_color_write_enable(mut self) -> Self {
        self = self.enable_khr_get_physical_device_properties_2();
        self.ext_color_write_enable = true;
        self
    }
    ///Checks if `VK_EXT_color_write_enable` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_EXT_color_write_enable")]
    pub const fn ext_color_write_enable(&self) -> bool {
        self.ext_color_write_enable
    }
    ///Enables `VK_EXT_image_drm_format_modifier` and it dependencies
    #[cfg(feature = "VK_EXT_image_drm_format_modifier")]
    pub const fn enable_ext_image_drm_format_modifier(mut self) -> Self {
        self = self
            .enable_khr_bind_memory_2()
            .enable_khr_get_physical_device_properties_2()
            .enable_khr_image_format_list()
            .enable_khr_sampler_ycbcr_conversion();
        self.ext_image_drm_format_modifier = true;
        self
    }
    ///Checks if `VK_EXT_image_drm_format_modifier` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_EXT_image_drm_format_modifier")]
    pub const fn ext_image_drm_format_modifier(&self) -> bool {
        self.ext_image_drm_format_modifier
    }
    ///Enables `VK_EXT_headless_surface` and it dependencies
    #[cfg(feature = "VK_EXT_headless_surface")]
    pub const fn enable_ext_headless_surface(mut self) -> Self {
        self = self.enable_khr_surface();
        self.ext_headless_surface = true;
        self
    }
    ///Checks if `VK_EXT_headless_surface` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_EXT_headless_surface")]
    pub const fn ext_headless_surface(&self) -> bool {
        self.ext_headless_surface
    }
    ///Enables `VK_GGP_frame_token` and it dependencies
    #[cfg(feature = "VK_GGP_frame_token")]
    pub const fn enable_ggp_frame_token(mut self) -> Self {
        self = self.enable_khr_swapchain().enable_ggp_stream_descriptor_surface();
        self.ggp_frame_token = true;
        self
    }
    ///Checks if `VK_GGP_frame_token` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_GGP_frame_token")]
    pub const fn ggp_frame_token(&self) -> bool {
        self.ggp_frame_token
    }
    ///Enables `VK_NV_external_memory` and it dependencies
    #[cfg(feature = "VK_NV_external_memory")]
    pub const fn enable_nv_external_memory(mut self) -> Self {
        self = self.enable_nv_external_memory_capabilities();
        self.nv_external_memory = true;
        self
    }
    ///Checks if `VK_NV_external_memory` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_NV_external_memory")]
    pub const fn nv_external_memory(&self) -> bool {
        self.nv_external_memory
    }
    ///Enables `VK_EXT_shader_stencil_export` and it dependencies
    #[cfg(feature = "VK_EXT_shader_stencil_export")]
    pub const fn enable_ext_shader_stencil_export(mut self) -> Self {
        self = self;
        self.ext_shader_stencil_export = true;
        self
    }
    ///Checks if `VK_EXT_shader_stencil_export` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_EXT_shader_stencil_export")]
    pub const fn ext_shader_stencil_export(&self) -> bool {
        self.ext_shader_stencil_export
    }
    ///Enables `VK_KHR_xlib_surface` and it dependencies
    #[cfg(feature = "VK_KHR_xlib_surface")]
    pub const fn enable_khr_xlib_surface(mut self) -> Self {
        self = self.enable_khr_surface();
        self.khr_xlib_surface = true;
        self
    }
    ///Checks if `VK_KHR_xlib_surface` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_KHR_xlib_surface")]
    pub const fn khr_xlib_surface(&self) -> bool {
        self.khr_xlib_surface
    }
    ///Enables `VK_AMD_gcn_shader` and it dependencies
    #[cfg(feature = "VK_AMD_gcn_shader")]
    pub const fn enable_amd_gcn_shader(mut self) -> Self {
        self = self;
        self.amd_gcn_shader = true;
        self
    }
    ///Checks if `VK_AMD_gcn_shader` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_AMD_gcn_shader")]
    pub const fn amd_gcn_shader(&self) -> bool {
        self.amd_gcn_shader
    }
    ///Enables `VK_EXT_4444_formats` and it dependencies
    #[cfg(feature = "VK_EXT_4444_formats")]
    pub const fn enable_ext_4444_formats(mut self) -> Self {
        self = self.enable_khr_get_physical_device_properties_2();
        self.ext_4444_formats = true;
        self
    }
    ///Checks if `VK_EXT_4444_formats` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_EXT_4444_formats")]
    pub const fn ext_4444_formats(&self) -> bool {
        self.ext_4444_formats
    }
    ///Enables `VK_EXT_texel_buffer_alignment` and it dependencies
    #[cfg(feature = "VK_EXT_texel_buffer_alignment")]
    pub const fn enable_ext_texel_buffer_alignment(mut self) -> Self {
        self = self.enable_khr_get_physical_device_properties_2();
        self.ext_texel_buffer_alignment = true;
        self
    }
    ///Checks if `VK_EXT_texel_buffer_alignment` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_EXT_texel_buffer_alignment")]
    pub const fn ext_texel_buffer_alignment(&self) -> bool {
        self.ext_texel_buffer_alignment
    }
    ///Enables `VK_EXT_validation_flags` and it dependencies
    #[cfg(feature = "VK_EXT_validation_flags")]
    pub const fn enable_ext_validation_flags(mut self) -> Self {
        self = self;
        self.ext_validation_flags = true;
        self
    }
    ///Checks if `VK_EXT_validation_flags` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_EXT_validation_flags")]
    pub const fn ext_validation_flags(&self) -> bool {
        self.ext_validation_flags
    }
    ///Enables `VK_KHR_shader_clock` and it dependencies
    #[cfg(feature = "VK_KHR_shader_clock")]
    pub const fn enable_khr_shader_clock(mut self) -> Self {
        self = self.enable_khr_get_physical_device_properties_2();
        self.khr_shader_clock = true;
        self
    }
    ///Checks if `VK_KHR_shader_clock` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_KHR_shader_clock")]
    pub const fn khr_shader_clock(&self) -> bool {
        self.khr_shader_clock
    }
    ///Enables `VK_QCOM_fragment_density_map_offset` and it dependencies
    #[cfg(feature = "VK_QCOM_fragment_density_map_offset")]
    pub const fn enable_qcom_fragment_density_map_offset(mut self) -> Self {
        self = self
            .enable_khr_get_physical_device_properties_2()
            .enable_ext_fragment_density_map();
        self.qcom_fragment_density_map_offset = true;
        self
    }
    ///Checks if `VK_QCOM_fragment_density_map_offset` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_QCOM_fragment_density_map_offset")]
    pub const fn qcom_fragment_density_map_offset(&self) -> bool {
        self.qcom_fragment_density_map_offset
    }
    ///Enables `VK_EXT_hdr_metadata` and it dependencies
    #[cfg(feature = "VK_EXT_hdr_metadata")]
    pub const fn enable_ext_hdr_metadata(mut self) -> Self {
        self = self.enable_khr_swapchain();
        self.ext_hdr_metadata = true;
        self
    }
    ///Checks if `VK_EXT_hdr_metadata` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_EXT_hdr_metadata")]
    pub const fn ext_hdr_metadata(&self) -> bool {
        self.ext_hdr_metadata
    }
    ///Enables `VK_EXT_pipeline_creation_feedback` and it dependencies
    #[cfg(feature = "VK_EXT_pipeline_creation_feedback")]
    pub const fn enable_ext_pipeline_creation_feedback(mut self) -> Self {
        self = self;
        self.ext_pipeline_creation_feedback = true;
        self
    }
    ///Checks if `VK_EXT_pipeline_creation_feedback` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_EXT_pipeline_creation_feedback")]
    pub const fn ext_pipeline_creation_feedback(&self) -> bool {
        self.ext_pipeline_creation_feedback
    }
    ///Enables `VK_NV_inherited_viewport_scissor` and it dependencies
    #[cfg(feature = "VK_NV_inherited_viewport_scissor")]
    pub const fn enable_nv_inherited_viewport_scissor(mut self) -> Self {
        self = self;
        self.nv_inherited_viewport_scissor = true;
        self
    }
    ///Checks if `VK_NV_inherited_viewport_scissor` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_NV_inherited_viewport_scissor")]
    pub const fn nv_inherited_viewport_scissor(&self) -> bool {
        self.nv_inherited_viewport_scissor
    }
    ///Enables `VK_AMD_draw_indirect_count` and it dependencies
    #[cfg(feature = "VK_AMD_draw_indirect_count")]
    pub const fn enable_amd_draw_indirect_count(mut self) -> Self {
        self = self;
        self.amd_draw_indirect_count = true;
        self
    }
    ///Checks if `VK_AMD_draw_indirect_count` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_AMD_draw_indirect_count")]
    pub const fn amd_draw_indirect_count(&self) -> bool {
        self.amd_draw_indirect_count
    }
    ///Enables `VK_NV_device_diagnostics_config` and it dependencies
    #[cfg(feature = "VK_NV_device_diagnostics_config")]
    pub const fn enable_nv_device_diagnostics_config(mut self) -> Self {
        self = self.enable_khr_get_physical_device_properties_2();
        self.nv_device_diagnostics_config = true;
        self
    }
    ///Checks if `VK_NV_device_diagnostics_config` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_NV_device_diagnostics_config")]
    pub const fn nv_device_diagnostics_config(&self) -> bool {
        self.nv_device_diagnostics_config
    }
    ///Enables `VK_KHR_dynamic_rendering` and it dependencies
    #[cfg(feature = "VK_KHR_dynamic_rendering")]
    pub const fn enable_khr_dynamic_rendering(mut self) -> Self {
        self = self.enable_khr_get_physical_device_properties_2();
        self.khr_dynamic_rendering = true;
        self
    }
    ///Checks if `VK_KHR_dynamic_rendering` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_KHR_dynamic_rendering")]
    pub const fn khr_dynamic_rendering(&self) -> bool {
        self.khr_dynamic_rendering
    }
    ///Enables `VK_AMD_device_coherent_memory` and it dependencies
    #[cfg(feature = "VK_AMD_device_coherent_memory")]
    pub const fn enable_amd_device_coherent_memory(mut self) -> Self {
        self = self;
        self.amd_device_coherent_memory = true;
        self
    }
    ///Checks if `VK_AMD_device_coherent_memory` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_AMD_device_coherent_memory")]
    pub const fn amd_device_coherent_memory(&self) -> bool {
        self.amd_device_coherent_memory
    }
    ///Enables `VK_AMD_buffer_marker` and it dependencies
    #[cfg(feature = "VK_AMD_buffer_marker")]
    pub const fn enable_amd_buffer_marker(mut self) -> Self {
        self = self;
        self.amd_buffer_marker = true;
        self
    }
    ///Checks if `VK_AMD_buffer_marker` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_AMD_buffer_marker")]
    pub const fn amd_buffer_marker(&self) -> bool {
        self.amd_buffer_marker
    }
    ///Enables `VK_NV_compute_shader_derivatives` and it dependencies
    #[cfg(feature = "VK_NV_compute_shader_derivatives")]
    pub const fn enable_nv_compute_shader_derivatives(mut self) -> Self {
        self = self.enable_khr_get_physical_device_properties_2();
        self.nv_compute_shader_derivatives = true;
        self
    }
    ///Checks if `VK_NV_compute_shader_derivatives` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_NV_compute_shader_derivatives")]
    pub const fn nv_compute_shader_derivatives(&self) -> bool {
        self.nv_compute_shader_derivatives
    }
    ///Enables `VK_NV_external_memory_rdma` and it dependencies
    #[cfg(feature = "VK_NV_external_memory_rdma")]
    pub const fn enable_nv_external_memory_rdma(mut self) -> Self {
        self = self.enable_khr_external_memory();
        self.nv_external_memory_rdma = true;
        self
    }
    ///Checks if `VK_NV_external_memory_rdma` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_NV_external_memory_rdma")]
    pub const fn nv_external_memory_rdma(&self) -> bool {
        self.nv_external_memory_rdma
    }
    ///Enables `VK_EXT_private_data` and it dependencies
    #[cfg(feature = "VK_EXT_private_data")]
    pub const fn enable_ext_private_data(mut self) -> Self {
        self = self;
        self.ext_private_data = true;
        self
    }
    ///Checks if `VK_EXT_private_data` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_EXT_private_data")]
    pub const fn ext_private_data(&self) -> bool {
        self.ext_private_data
    }
    ///Enables `VK_NV_ray_tracing_motion_blur` and it dependencies
    #[cfg(feature = "VK_NV_ray_tracing_motion_blur")]
    pub const fn enable_nv_ray_tracing_motion_blur(mut self) -> Self {
        self = self.enable_khr_ray_tracing_pipeline();
        self.nv_ray_tracing_motion_blur = true;
        self
    }
    ///Checks if `VK_NV_ray_tracing_motion_blur` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_NV_ray_tracing_motion_blur")]
    pub const fn nv_ray_tracing_motion_blur(&self) -> bool {
        self.nv_ray_tracing_motion_blur
    }
    ///Enables `VK_NV_glsl_shader` and it dependencies
    #[cfg(feature = "VK_NV_glsl_shader")]
    pub const fn enable_nv_glsl_shader(mut self) -> Self {
        self = self;
        self.nv_glsl_shader = true;
        self
    }
    ///Checks if `VK_NV_glsl_shader` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_NV_glsl_shader")]
    pub const fn nv_glsl_shader(&self) -> bool {
        self.nv_glsl_shader
    }
    ///Enables `VK_EXT_buffer_device_address` and it dependencies
    #[cfg(feature = "VK_EXT_buffer_device_address")]
    pub const fn enable_ext_buffer_device_address(mut self) -> Self {
        self = self.enable_khr_get_physical_device_properties_2();
        self.ext_buffer_device_address = true;
        self
    }
    ///Checks if `VK_EXT_buffer_device_address` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_EXT_buffer_device_address")]
    pub const fn ext_buffer_device_address(&self) -> bool {
        self.ext_buffer_device_address
    }
    ///Enables `VK_EXT_host_query_reset` and it dependencies
    #[cfg(feature = "VK_EXT_host_query_reset")]
    pub const fn enable_ext_host_query_reset(mut self) -> Self {
        self = self.enable_khr_get_physical_device_properties_2();
        self.ext_host_query_reset = true;
        self
    }
    ///Checks if `VK_EXT_host_query_reset` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_EXT_host_query_reset")]
    pub const fn ext_host_query_reset(&self) -> bool {
        self.ext_host_query_reset
    }
    ///Enables `VK_EXT_load_store_op_none` and it dependencies
    #[cfg(feature = "VK_EXT_load_store_op_none")]
    pub const fn enable_ext_load_store_op_none(mut self) -> Self {
        self = self;
        self.ext_load_store_op_none = true;
        self
    }
    ///Checks if `VK_EXT_load_store_op_none` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_EXT_load_store_op_none")]
    pub const fn ext_load_store_op_none(&self) -> bool {
        self.ext_load_store_op_none
    }
    ///Enables `VK_AMD_shader_core_properties2` and it dependencies
    #[cfg(feature = "VK_AMD_shader_core_properties2")]
    pub const fn enable_amd_shader_core_properties_2(mut self) -> Self {
        self = self.enable_amd_shader_core_properties();
        self.amd_shader_core_properties_2 = true;
        self
    }
    ///Checks if `VK_AMD_shader_core_properties2` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_AMD_shader_core_properties2")]
    pub const fn amd_shader_core_properties_2(&self) -> bool {
        self.amd_shader_core_properties_2
    }
    ///Enables `VK_QCOM_render_pass_store_ops` and it dependencies
    #[cfg(feature = "VK_QCOM_render_pass_store_ops")]
    pub const fn enable_qcom_render_pass_store_ops(mut self) -> Self {
        self = self;
        self.qcom_render_pass_store_ops = true;
        self
    }
    ///Checks if `VK_QCOM_render_pass_store_ops` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_QCOM_render_pass_store_ops")]
    pub const fn qcom_render_pass_store_ops(&self) -> bool {
        self.qcom_render_pass_store_ops
    }
    ///Enables `VK_NV_win32_keyed_mutex` and it dependencies
    #[cfg(feature = "VK_NV_win32_keyed_mutex")]
    pub const fn enable_nv_win_32_keyed_mutex(mut self) -> Self {
        self = self.enable_nv_external_memory_win_32();
        self.nv_win_32_keyed_mutex = true;
        self
    }
    ///Checks if `VK_NV_win32_keyed_mutex` is enabled
    #[inline(always)]
    #[cfg(feature = "VK_NV_win32_keyed_mutex")]
    pub const fn nv_win_32_keyed_mutex(&self) -> bool {
        self.nv_win_32_keyed_mutex
    }
    ///Gets the list of extension names to use when creating the [`crate::vulkan1_0::Device`
    pub fn device_extension_names(&self) -> Vec<*const c_char> {
        let mut out = Vec::with_capacity(238usize);
        #[cfg(feature = "VK_AMD_negative_viewport_height")]
        if self.amd_negative_viewport_height() {
            out.push(cstr_ptr!("VK_AMD_negative_viewport_height"));
        }
        #[cfg(feature = "VK_EXT_image_view_min_lod")]
        if self.ext_image_view_min_lod() {
            out.push(cstr_ptr!("VK_EXT_image_view_min_lod"));
        }
        #[cfg(feature = "VK_KHR_external_memory")]
        if self.khr_external_memory() {
            out.push(cstr_ptr!("VK_KHR_external_memory"));
        }
        #[cfg(feature = "VK_KHR_bind_memory2")]
        if self.khr_bind_memory_2() {
            out.push(cstr_ptr!("VK_KHR_bind_memory2"));
        }
        #[cfg(feature = "VK_KHR_present_id")]
        if self.khr_present_id() {
            out.push(cstr_ptr!("VK_KHR_present_id"));
        }
        #[cfg(feature = "VK_KHR_pipeline_library")]
        if self.khr_pipeline_library() {
            out.push(cstr_ptr!("VK_KHR_pipeline_library"));
        }
        #[cfg(feature = "VK_AMD_shader_image_load_store_lod")]
        if self.amd_shader_image_load_store_lod() {
            out.push(cstr_ptr!("VK_AMD_shader_image_load_store_lod"));
        }
        #[cfg(feature = "VK_KHR_video_queue")]
        if self.khr_video_queue() {
            out.push(cstr_ptr!("VK_KHR_video_queue"));
        }
        #[cfg(feature = "VK_EXT_tooling_info")]
        if self.ext_tooling_info() {
            out.push(cstr_ptr!("VK_EXT_tooling_info"));
        }
        #[cfg(feature = "VK_KHR_swapchain")]
        if self.khr_swapchain() {
            out.push(cstr_ptr!("VK_KHR_swapchain"));
        }
        #[cfg(feature = "VK_EXT_shader_subgroup_ballot")]
        if self.ext_shader_subgroup_ballot() {
            out.push(cstr_ptr!("VK_EXT_shader_subgroup_ballot"));
        }
        #[cfg(feature = "VK_KHR_push_descriptor")]
        if self.khr_push_descriptor() {
            out.push(cstr_ptr!("VK_KHR_push_descriptor"));
        }
        #[cfg(feature = "VK_KHR_incremental_present")]
        if self.khr_incremental_present() {
            out.push(cstr_ptr!("VK_KHR_incremental_present"));
        }
        #[cfg(feature = "VK_EXT_external_memory_host")]
        if self.ext_external_memory_host() {
            out.push(cstr_ptr!("VK_EXT_external_memory_host"));
        }
        #[cfg(feature = "VK_NV_device_generated_commands")]
        if self.nv_device_generated_commands() {
            out.push(cstr_ptr!("VK_NV_device_generated_commands"));
        }
        #[cfg(feature = "VK_EXT_robustness2")]
        if self.ext_robustness_2() {
            out.push(cstr_ptr!("VK_EXT_robustness2"));
        }
        #[cfg(feature = "VK_NV_fill_rectangle")]
        if self.nv_fill_rectangle() {
            out.push(cstr_ptr!("VK_NV_fill_rectangle"));
        }
        #[cfg(feature = "VK_KHR_maintenance3")]
        if self.khr_maintenance_3() {
            out.push(cstr_ptr!("VK_KHR_maintenance3"));
        }
        #[cfg(feature = "VK_EXT_pci_bus_info")]
        if self.ext_pci_bus_info() {
            out.push(cstr_ptr!("VK_EXT_pci_bus_info"));
        }
        #[cfg(feature = "VK_NV_external_memory_win32")]
        if self.nv_external_memory_win_32() {
            out.push(cstr_ptr!("VK_NV_external_memory_win32"));
        }
        #[cfg(feature = "VK_EXT_sampler_filter_minmax")]
        if self.ext_sampler_filter_minmax() {
            out.push(cstr_ptr!("VK_EXT_sampler_filter_minmax"));
        }
        #[cfg(feature = "VK_AMD_display_native_hdr")]
        if self.amd_display_native_hdr() {
            out.push(cstr_ptr!("VK_AMD_display_native_hdr"));
        }
        #[cfg(feature = "VK_EXT_fragment_density_map")]
        if self.ext_fragment_density_map() {
            out.push(cstr_ptr!("VK_EXT_fragment_density_map"));
        }
        #[cfg(feature = "VK_NV_cooperative_matrix")]
        if self.nv_cooperative_matrix() {
            out.push(cstr_ptr!("VK_NV_cooperative_matrix"));
        }
        #[cfg(feature = "VK_EXT_multi_draw")]
        if self.ext_multi_draw() {
            out.push(cstr_ptr!("VK_EXT_multi_draw"));
        }
        #[cfg(feature = "VK_KHR_vulkan_memory_model")]
        if self.khr_vulkan_memory_model() {
            out.push(cstr_ptr!("VK_KHR_vulkan_memory_model"));
        }
        #[cfg(feature = "VK_NV_linear_color_attachment")]
        if self.nv_linear_color_attachment() {
            out.push(cstr_ptr!("VK_NV_linear_color_attachment"));
        }
        #[cfg(feature = "VK_AMD_shader_info")]
        if self.amd_shader_info() {
            out.push(cstr_ptr!("VK_AMD_shader_info"));
        }
        #[cfg(feature = "VK_EXT_global_priority_query")]
        if self.ext_global_priority_query() {
            out.push(cstr_ptr!("VK_EXT_global_priority_query"));
        }
        #[cfg(feature = "VK_KHR_external_semaphore_win32")]
        if self.khr_external_semaphore_win_32() {
            out.push(cstr_ptr!("VK_KHR_external_semaphore_win32"));
        }
        #[cfg(feature = "VK_NV_dedicated_allocation_image_aliasing")]
        if self.nv_dedicated_allocation_image_aliasing() {
            out.push(cstr_ptr!("VK_NV_dedicated_allocation_image_aliasing"));
        }
        #[cfg(feature = "VK_KHR_shared_presentable_image")]
        if self.khr_shared_presentable_image() {
            out.push(cstr_ptr!("VK_KHR_shared_presentable_image"));
        }
        #[cfg(feature = "VK_NV_sample_mask_override_coverage")]
        if self.nv_sample_mask_override_coverage() {
            out.push(cstr_ptr!("VK_NV_sample_mask_override_coverage"));
        }
        #[cfg(feature = "VK_FUCHSIA_buffer_collection")]
        if self.fuchsia_buffer_collection() {
            out.push(cstr_ptr!("VK_FUCHSIA_buffer_collection"));
        }
        #[cfg(feature = "VK_NV_shader_image_footprint")]
        if self.nv_shader_image_footprint() {
            out.push(cstr_ptr!("VK_NV_shader_image_footprint"));
        }
        #[cfg(feature = "VK_EXT_discard_rectangles")]
        if self.ext_discard_rectangles() {
            out.push(cstr_ptr!("VK_EXT_discard_rectangles"));
        }
        #[cfg(feature = "VK_EXT_shader_atomic_float")]
        if self.ext_shader_atomic_float() {
            out.push(cstr_ptr!("VK_EXT_shader_atomic_float"));
        }
        #[cfg(feature = "VK_KHR_win32_keyed_mutex")]
        if self.khr_win_32_keyed_mutex() {
            out.push(cstr_ptr!("VK_KHR_win32_keyed_mutex"));
        }
        #[cfg(feature = "VK_NVX_multiview_per_view_attributes")]
        if self.nvx_multiview_per_view_attributes() {
            out.push(cstr_ptr!("VK_NVX_multiview_per_view_attributes"));
        }
        #[cfg(feature = "VK_KHR_external_memory_fd")]
        if self.khr_external_memory_fd() {
            out.push(cstr_ptr!("VK_KHR_external_memory_fd"));
        }
        #[cfg(feature = "VK_EXT_scalar_block_layout")]
        if self.ext_scalar_block_layout() {
            out.push(cstr_ptr!("VK_EXT_scalar_block_layout"));
        }
        #[cfg(feature = "VK_AMD_rasterization_order")]
        if self.amd_rasterization_order() {
            out.push(cstr_ptr!("VK_AMD_rasterization_order"));
        }
        #[cfg(feature = "VK_EXT_image_robustness")]
        if self.ext_image_robustness() {
            out.push(cstr_ptr!("VK_EXT_image_robustness"));
        }
        #[cfg(feature = "VK_NVX_image_view_handle")]
        if self.nvx_image_view_handle() {
            out.push(cstr_ptr!("VK_NVX_image_view_handle"));
        }
        #[cfg(feature = "VK_NV_fragment_shader_barycentric")]
        if self.nv_fragment_shader_barycentric() {
            out.push(cstr_ptr!("VK_NV_fragment_shader_barycentric"));
        }
        #[cfg(feature = "VK_EXT_depth_clip_enable")]
        if self.ext_depth_clip_enable() {
            out.push(cstr_ptr!("VK_EXT_depth_clip_enable"));
        }
        #[cfg(feature = "VK_VALVE_descriptor_set_host_mapping")]
        if self.valve_descriptor_set_host_mapping() {
            out.push(cstr_ptr!("VK_VALVE_descriptor_set_host_mapping"));
        }
        #[cfg(feature = "VK_EXT_custom_border_color")]
        if self.ext_custom_border_color() {
            out.push(cstr_ptr!("VK_EXT_custom_border_color"));
        }
        #[cfg(feature = "VK_KHR_global_priority")]
        if self.khr_global_priority() {
            out.push(cstr_ptr!("VK_KHR_global_priority"));
        }
        #[cfg(feature = "VK_NV_dedicated_allocation")]
        if self.nv_dedicated_allocation() {
            out.push(cstr_ptr!("VK_NV_dedicated_allocation"));
        }
        #[cfg(feature = "VK_EXT_conditional_rendering")]
        if self.ext_conditional_rendering() {
            out.push(cstr_ptr!("VK_EXT_conditional_rendering"));
        }
        #[cfg(feature = "VK_EXT_pageable_device_local_memory")]
        if self.ext_pageable_device_local_memory() {
            out.push(cstr_ptr!("VK_EXT_pageable_device_local_memory"));
        }
        #[cfg(feature = "VK_KHR_create_renderpass2")]
        if self.khr_create_renderpass_2() {
            out.push(cstr_ptr!("VK_KHR_create_renderpass2"));
        }
        #[cfg(feature = "VK_QCOM_render_pass_transform")]
        if self.qcom_render_pass_transform() {
            out.push(cstr_ptr!("VK_QCOM_render_pass_transform"));
        }
        #[cfg(feature = "VK_EXT_depth_clip_control")]
        if self.ext_depth_clip_control() {
            out.push(cstr_ptr!("VK_EXT_depth_clip_control"));
        }
        #[cfg(feature = "VK_KHR_present_wait")]
        if self.khr_present_wait() {
            out.push(cstr_ptr!("VK_KHR_present_wait"));
        }
        #[cfg(feature = "VK_KHR_maintenance2")]
        if self.khr_maintenance_2() {
            out.push(cstr_ptr!("VK_KHR_maintenance2"));
        }
        #[cfg(feature = "VK_KHR_relaxed_block_layout")]
        if self.khr_relaxed_block_layout() {
            out.push(cstr_ptr!("VK_KHR_relaxed_block_layout"));
        }
        #[cfg(feature = "VK_KHR_external_memory_win32")]
        if self.khr_external_memory_win_32() {
            out.push(cstr_ptr!("VK_KHR_external_memory_win32"));
        }
        #[cfg(feature = "VK_EXT_subgroup_size_control")]
        if self.ext_subgroup_size_control() {
            out.push(cstr_ptr!("VK_EXT_subgroup_size_control"));
        }
        #[cfg(feature = "VK_EXT_shader_demote_to_helper_invocation")]
        if self.ext_shader_demote_to_helper_invocation() {
            out.push(cstr_ptr!("VK_EXT_shader_demote_to_helper_invocation"));
        }
        #[cfg(feature = "VK_KHR_external_fence")]
        if self.khr_external_fence() {
            out.push(cstr_ptr!("VK_KHR_external_fence"));
        }
        #[cfg(feature = "VK_VALVE_mutable_descriptor_type")]
        if self.valve_mutable_descriptor_type() {
            out.push(cstr_ptr!("VK_VALVE_mutable_descriptor_type"));
        }
        #[cfg(feature = "VK_NV_corner_sampled_image")]
        if self.nv_corner_sampled_image() {
            out.push(cstr_ptr!("VK_NV_corner_sampled_image"));
        }
        #[cfg(feature = "VK_EXT_validation_cache")]
        if self.ext_validation_cache() {
            out.push(cstr_ptr!("VK_EXT_validation_cache"));
        }
        #[cfg(feature = "VK_KHR_depth_stencil_resolve")]
        if self.khr_depth_stencil_resolve() {
            out.push(cstr_ptr!("VK_KHR_depth_stencil_resolve"));
        }
        #[cfg(feature = "VK_NV_shader_sm_builtins")]
        if self.nv_shader_sm_builtins() {
            out.push(cstr_ptr!("VK_NV_shader_sm_builtins"));
        }
        #[cfg(feature = "VK_EXT_provoking_vertex")]
        if self.ext_provoking_vertex() {
            out.push(cstr_ptr!("VK_EXT_provoking_vertex"));
        }
        #[cfg(feature = "VK_KHR_performance_query")]
        if self.khr_performance_query() {
            out.push(cstr_ptr!("VK_KHR_performance_query"));
        }
        #[cfg(feature = "VK_KHR_imageless_framebuffer")]
        if self.khr_imageless_framebuffer() {
            out.push(cstr_ptr!("VK_KHR_imageless_framebuffer"));
        }
        #[cfg(feature = "VK_KHR_descriptor_update_template")]
        if self.khr_descriptor_update_template() {
            out.push(cstr_ptr!("VK_KHR_descriptor_update_template"));
        }
        #[cfg(feature = "VK_NV_clip_space_w_scaling")]
        if self.nv_clip_space_w_scaling() {
            out.push(cstr_ptr!("VK_NV_clip_space_w_scaling"));
        }
        #[cfg(feature = "VK_KHR_storage_buffer_storage_class")]
        if self.khr_storage_buffer_storage_class() {
            out.push(cstr_ptr!("VK_KHR_storage_buffer_storage_class"));
        }
        #[cfg(feature = "VK_KHR_get_memory_requirements2")]
        if self.khr_get_memory_requirements_2() {
            out.push(cstr_ptr!("VK_KHR_get_memory_requirements2"));
        }
        #[cfg(feature = "VK_KHR_external_fence_fd")]
        if self.khr_external_fence_fd() {
            out.push(cstr_ptr!("VK_KHR_external_fence_fd"));
        }
        #[cfg(feature = "VK_EXT_shader_viewport_index_layer")]
        if self.ext_shader_viewport_index_layer() {
            out.push(cstr_ptr!("VK_EXT_shader_viewport_index_layer"));
        }
        #[cfg(feature = "VK_NV_representative_fragment_test")]
        if self.nv_representative_fragment_test() {
            out.push(cstr_ptr!("VK_NV_representative_fragment_test"));
        }
        #[cfg(feature = "VK_KHR_shader_terminate_invocation")]
        if self.khr_shader_terminate_invocation() {
            out.push(cstr_ptr!("VK_KHR_shader_terminate_invocation"));
        }
        #[cfg(feature = "VK_NV_viewport_swizzle")]
        if self.nv_viewport_swizzle() {
            out.push(cstr_ptr!("VK_NV_viewport_swizzle"));
        }
        #[cfg(feature = "VK_KHR_fragment_shading_rate")]
        if self.khr_fragment_shading_rate() {
            out.push(cstr_ptr!("VK_KHR_fragment_shading_rate"));
        }
        #[cfg(feature = "VK_EXT_memory_budget")]
        if self.ext_memory_budget() {
            out.push(cstr_ptr!("VK_EXT_memory_budget"));
        }
        #[cfg(feature = "VK_EXT_device_memory_report")]
        if self.ext_device_memory_report() {
            out.push(cstr_ptr!("VK_EXT_device_memory_report"));
        }
        #[cfg(feature = "VK_EXT_external_memory_dma_buf")]
        if self.ext_external_memory_dma_buf() {
            out.push(cstr_ptr!("VK_EXT_external_memory_dma_buf"));
        }
        #[cfg(feature = "VK_KHR_copy_commands2")]
        if self.khr_copy_commands_2() {
            out.push(cstr_ptr!("VK_KHR_copy_commands2"));
        }
        #[cfg(feature = "VK_EXT_video_decode_h265")]
        if self.ext_video_decode_h_265() {
            out.push(cstr_ptr!("VK_EXT_video_decode_h265"));
        }
        #[cfg(feature = "VK_EXT_line_rasterization")]
        if self.ext_line_rasterization() {
            out.push(cstr_ptr!("VK_EXT_line_rasterization"));
        }
        #[cfg(feature = "VK_NV_scissor_exclusive")]
        if self.nv_scissor_exclusive() {
            out.push(cstr_ptr!("VK_NV_scissor_exclusive"));
        }
        #[cfg(feature = "VK_EXT_queue_family_foreign")]
        if self.ext_queue_family_foreign() {
            out.push(cstr_ptr!("VK_EXT_queue_family_foreign"));
        }
        #[cfg(feature = "VK_EXT_video_encode_h265")]
        if self.ext_video_encode_h_265() {
            out.push(cstr_ptr!("VK_EXT_video_encode_h265"));
        }
        #[cfg(feature = "VK_KHR_portability_subset")]
        if self.khr_portability_subset() {
            out.push(cstr_ptr!("VK_KHR_portability_subset"));
        }
        #[cfg(feature = "VK_AMD_gpu_shader_int16")]
        if self.amd_gpu_shader_int_16() {
            out.push(cstr_ptr!("VK_AMD_gpu_shader_int16"));
        }
        #[cfg(feature = "VK_KHR_zero_initialize_workgroup_memory")]
        if self.khr_zero_initialize_workgroup_memory() {
            out.push(cstr_ptr!("VK_KHR_zero_initialize_workgroup_memory"));
        }
        #[cfg(feature = "VK_INTEL_performance_query")]
        if self.intel_performance_query() {
            out.push(cstr_ptr!("VK_INTEL_performance_query"));
        }
        #[cfg(feature = "VK_EXT_video_encode_h264")]
        if self.ext_video_encode_h_264() {
            out.push(cstr_ptr!("VK_EXT_video_encode_h264"));
        }
        #[cfg(feature = "VK_AMD_shader_fragment_mask")]
        if self.amd_shader_fragment_mask() {
            out.push(cstr_ptr!("VK_AMD_shader_fragment_mask"));
        }
        #[cfg(feature = "VK_AMD_shader_core_properties")]
        if self.amd_shader_core_properties() {
            out.push(cstr_ptr!("VK_AMD_shader_core_properties"));
        }
        #[cfg(feature = "VK_KHR_ray_query")]
        if self.khr_ray_query() {
            out.push(cstr_ptr!("VK_KHR_ray_query"));
        }
        #[cfg(feature = "VK_NV_framebuffer_mixed_samples")]
        if self.nv_framebuffer_mixed_samples() {
            out.push(cstr_ptr!("VK_NV_framebuffer_mixed_samples"));
        }
        #[cfg(feature = "VK_KHR_format_feature_flags2")]
        if self.khr_format_feature_flags_2() {
            out.push(cstr_ptr!("VK_KHR_format_feature_flags2"));
        }
        #[cfg(feature = "VK_EXT_pipeline_creation_cache_control")]
        if self.ext_pipeline_creation_cache_control() {
            out.push(cstr_ptr!("VK_EXT_pipeline_creation_cache_control"));
        }
        #[cfg(feature = "VK_HUAWEI_invocation_mask")]
        if self.huawei_invocation_mask() {
            out.push(cstr_ptr!("VK_HUAWEI_invocation_mask"));
        }
        #[cfg(feature = "VK_KHR_timeline_semaphore")]
        if self.khr_timeline_semaphore() {
            out.push(cstr_ptr!("VK_KHR_timeline_semaphore"));
        }
        #[cfg(feature = "VK_EXT_global_priority")]
        if self.ext_global_priority() {
            out.push(cstr_ptr!("VK_EXT_global_priority"));
        }
        #[cfg(feature = "VK_KHR_pipeline_executable_properties")]
        if self.khr_pipeline_executable_properties() {
            out.push(cstr_ptr!("VK_KHR_pipeline_executable_properties"));
        }
        #[cfg(feature = "VK_INTEL_shader_integer_functions2")]
        if self.intel_shader_integer_functions_2() {
            out.push(cstr_ptr!("VK_INTEL_shader_integer_functions2"));
        }
        #[cfg(feature = "VK_HUAWEI_subpass_shading")]
        if self.huawei_subpass_shading() {
            out.push(cstr_ptr!("VK_HUAWEI_subpass_shading"));
        }
        #[cfg(feature = "VK_NV_acquire_winrt_display")]
        if self.nv_acquire_winrt_display() {
            out.push(cstr_ptr!("VK_NV_acquire_winrt_display"));
        }
        #[cfg(feature = "VK_FUCHSIA_external_semaphore")]
        if self.fuchsia_external_semaphore() {
            out.push(cstr_ptr!("VK_FUCHSIA_external_semaphore"));
        }
        #[cfg(feature = "VK_KHR_acceleration_structure")]
        if self.khr_acceleration_structure() {
            out.push(cstr_ptr!("VK_KHR_acceleration_structure"));
        }
        #[cfg(feature = "VK_AMD_shader_trinary_minmax")]
        if self.amd_shader_trinary_minmax() {
            out.push(cstr_ptr!("VK_AMD_shader_trinary_minmax"));
        }
        #[cfg(feature = "VK_EXT_separate_stencil_usage")]
        if self.ext_separate_stencil_usage() {
            out.push(cstr_ptr!("VK_EXT_separate_stencil_usage"));
        }
        #[cfg(feature = "VK_NV_ray_tracing")]
        if self.nv_ray_tracing() {
            out.push(cstr_ptr!("VK_NV_ray_tracing"));
        }
        #[cfg(feature = "VK_EXT_post_depth_coverage")]
        if self.ext_post_depth_coverage() {
            out.push(cstr_ptr!("VK_EXT_post_depth_coverage"));
        }
        #[cfg(feature = "VK_KHR_16bit_storage")]
        if self.khr_16_bit_storage() {
            out.push(cstr_ptr!("VK_KHR_16bit_storage"));
        }
        #[cfg(feature = "VK_EXT_vertex_attribute_divisor")]
        if self.ext_vertex_attribute_divisor() {
            out.push(cstr_ptr!("VK_EXT_vertex_attribute_divisor"));
        }
        #[cfg(feature = "VK_KHR_shader_integer_dot_product")]
        if self.khr_shader_integer_dot_product() {
            out.push(cstr_ptr!("VK_KHR_shader_integer_dot_product"));
        }
        #[cfg(feature = "VK_NV_fragment_shading_rate_enums")]
        if self.nv_fragment_shading_rate_enums() {
            out.push(cstr_ptr!("VK_NV_fragment_shading_rate_enums"));
        }
        #[cfg(feature = "VK_KHR_image_format_list")]
        if self.khr_image_format_list() {
            out.push(cstr_ptr!("VK_KHR_image_format_list"));
        }
        #[cfg(feature = "VK_KHR_maintenance4")]
        if self.khr_maintenance_4() {
            out.push(cstr_ptr!("VK_KHR_maintenance4"));
        }
        #[cfg(feature = "VK_EXT_display_control")]
        if self.ext_display_control() {
            out.push(cstr_ptr!("VK_EXT_display_control"));
        }
        #[cfg(feature = "VK_EXT_video_decode_h264")]
        if self.ext_video_decode_h_264() {
            out.push(cstr_ptr!("VK_EXT_video_decode_h264"));
        }
        #[cfg(feature = "VK_EXT_full_screen_exclusive")]
        if self.ext_full_screen_exclusive() {
            out.push(cstr_ptr!("VK_EXT_full_screen_exclusive"));
        }
        #[cfg(feature = "VK_AMD_shader_ballot")]
        if self.amd_shader_ballot() {
            out.push(cstr_ptr!("VK_AMD_shader_ballot"));
        }
        #[cfg(feature = "VK_KHR_shader_float_controls")]
        if self.khr_shader_float_controls() {
            out.push(cstr_ptr!("VK_KHR_shader_float_controls"));
        }
        #[cfg(feature = "VK_EXT_sample_locations")]
        if self.ext_sample_locations() {
            out.push(cstr_ptr!("VK_EXT_sample_locations"));
        }
        #[cfg(feature = "VK_KHR_variable_pointers")]
        if self.khr_variable_pointers() {
            out.push(cstr_ptr!("VK_KHR_variable_pointers"));
        }
        #[cfg(feature = "VK_EXT_extended_dynamic_state2")]
        if self.ext_extended_dynamic_state_2() {
            out.push(cstr_ptr!("VK_EXT_extended_dynamic_state2"));
        }
        #[cfg(feature = "VK_EXT_shader_atomic_float2")]
        if self.ext_shader_atomic_float_2() {
            out.push(cstr_ptr!("VK_EXT_shader_atomic_float2"));
        }
        #[cfg(feature = "VK_KHR_separate_depth_stencil_layouts")]
        if self.khr_separate_depth_stencil_layouts() {
            out.push(cstr_ptr!("VK_KHR_separate_depth_stencil_layouts"));
        }
        #[cfg(feature = "VK_NV_fragment_coverage_to_color")]
        if self.nv_fragment_coverage_to_color() {
            out.push(cstr_ptr!("VK_NV_fragment_coverage_to_color"));
        }
        #[cfg(feature = "VK_GOOGLE_decorate_string")]
        if self.google_decorate_string() {
            out.push(cstr_ptr!("VK_GOOGLE_decorate_string"));
        }
        #[cfg(feature = "VK_NV_geometry_shader_passthrough")]
        if self.nv_geometry_shader_passthrough() {
            out.push(cstr_ptr!("VK_NV_geometry_shader_passthrough"));
        }
        #[cfg(feature = "VK_NVX_binary_import")]
        if self.nvx_binary_import() {
            out.push(cstr_ptr!("VK_NVX_binary_import"));
        }
        #[cfg(feature = "VK_KHR_shader_subgroup_extended_types")]
        if self.khr_shader_subgroup_extended_types() {
            out.push(cstr_ptr!("VK_KHR_shader_subgroup_extended_types"));
        }
        #[cfg(feature = "VK_EXT_transform_feedback")]
        if self.ext_transform_feedback() {
            out.push(cstr_ptr!("VK_EXT_transform_feedback"));
        }
        #[cfg(feature = "VK_EXT_rgba10x6_formats")]
        if self.ext_rgba_10_x_6_formats() {
            out.push(cstr_ptr!("VK_EXT_rgba10x6_formats"));
        }
        #[cfg(feature = "VK_KHR_draw_indirect_count")]
        if self.khr_draw_indirect_count() {
            out.push(cstr_ptr!("VK_KHR_draw_indirect_count"));
        }
        #[cfg(feature = "VK_GOOGLE_user_type")]
        if self.google_user_type() {
            out.push(cstr_ptr!("VK_GOOGLE_user_type"));
        }
        #[cfg(feature = "VK_KHR_shader_subgroup_uniform_control_flow")]
        if self.khr_shader_subgroup_uniform_control_flow() {
            out.push(cstr_ptr!("VK_KHR_shader_subgroup_uniform_control_flow"));
        }
        #[cfg(feature = "VK_ANDROID_external_memory_android_hardware_buffer")]
        if self.android_external_memory_android_hardware_buffer() {
            out.push(cstr_ptr!("VK_ANDROID_external_memory_android_hardware_buffer"));
        }
        #[cfg(feature = "VK_NV_shading_rate_image")]
        if self.nv_shading_rate_image() {
            out.push(cstr_ptr!("VK_NV_shading_rate_image"));
        }
        #[cfg(feature = "VK_EXT_astc_decode_mode")]
        if self.ext_astc_decode_mode() {
            out.push(cstr_ptr!("VK_EXT_astc_decode_mode"));
        }
        #[cfg(feature = "VK_AMD_shader_explicit_vertex_parameter")]
        if self.amd_shader_explicit_vertex_parameter() {
            out.push(cstr_ptr!("VK_AMD_shader_explicit_vertex_parameter"));
        }
        #[cfg(feature = "VK_KHR_device_group")]
        if self.khr_device_group() {
            out.push(cstr_ptr!("VK_KHR_device_group"));
        }
        #[cfg(feature = "VK_KHR_dedicated_allocation")]
        if self.khr_dedicated_allocation() {
            out.push(cstr_ptr!("VK_KHR_dedicated_allocation"));
        }
        #[cfg(feature = "VK_IMG_format_pvrtc")]
        if self.img_format_pvrtc() {
            out.push(cstr_ptr!("VK_IMG_format_pvrtc"));
        }
        #[cfg(feature = "VK_KHR_ray_tracing_pipeline")]
        if self.khr_ray_tracing_pipeline() {
            out.push(cstr_ptr!("VK_KHR_ray_tracing_pipeline"));
        }
        #[cfg(feature = "VK_QCOM_render_pass_shader_resolve")]
        if self.qcom_render_pass_shader_resolve() {
            out.push(cstr_ptr!("VK_QCOM_render_pass_shader_resolve"));
        }
        #[cfg(feature = "VK_KHR_workgroup_memory_explicit_layout")]
        if self.khr_workgroup_memory_explicit_layout() {
            out.push(cstr_ptr!("VK_KHR_workgroup_memory_explicit_layout"));
        }
        #[cfg(feature = "VK_EXT_vertex_input_dynamic_state")]
        if self.ext_vertex_input_dynamic_state() {
            out.push(cstr_ptr!("VK_EXT_vertex_input_dynamic_state"));
        }
        #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
        if self.khr_sampler_ycbcr_conversion() {
            out.push(cstr_ptr!("VK_KHR_sampler_ycbcr_conversion"));
        }
        #[cfg(feature = "VK_KHR_shader_atomic_int64")]
        if self.khr_shader_atomic_int_64() {
            out.push(cstr_ptr!("VK_KHR_shader_atomic_int64"));
        }
        #[cfg(feature = "VK_KHR_display_swapchain")]
        if self.khr_display_swapchain() {
            out.push(cstr_ptr!("VK_KHR_display_swapchain"));
        }
        #[cfg(feature = "VK_IMG_filter_cubic")]
        if self.img_filter_cubic() {
            out.push(cstr_ptr!("VK_IMG_filter_cubic"));
        }
        #[cfg(feature = "VK_EXT_index_type_uint8")]
        if self.ext_index_type_uint_8() {
            out.push(cstr_ptr!("VK_EXT_index_type_uint8"));
        }
        #[cfg(feature = "VK_AMD_memory_overallocation_behavior")]
        if self.amd_memory_overallocation_behavior() {
            out.push(cstr_ptr!("VK_AMD_memory_overallocation_behavior"));
        }
        #[cfg(feature = "VK_EXT_shader_image_atomic_int64")]
        if self.ext_shader_image_atomic_int_64() {
            out.push(cstr_ptr!("VK_EXT_shader_image_atomic_int64"));
        }
        #[cfg(feature = "VK_KHR_multiview")]
        if self.khr_multiview() {
            out.push(cstr_ptr!("VK_KHR_multiview"));
        }
        #[cfg(feature = "VK_AMD_mixed_attachment_samples")]
        if self.amd_mixed_attachment_samples() {
            out.push(cstr_ptr!("VK_AMD_mixed_attachment_samples"));
        }
        #[cfg(feature = "VK_KHR_uniform_buffer_standard_layout")]
        if self.khr_uniform_buffer_standard_layout() {
            out.push(cstr_ptr!("VK_KHR_uniform_buffer_standard_layout"));
        }
        #[cfg(feature = "VK_EXT_ycbcr_2plane_444_formats")]
        if self.ext_ycbcr_2_plane_444_formats() {
            out.push(cstr_ptr!("VK_EXT_ycbcr_2plane_444_formats"));
        }
        #[cfg(feature = "VK_KHR_shader_float16_int8")]
        if self.khr_shader_float_16_int_8() {
            out.push(cstr_ptr!("VK_KHR_shader_float16_int8"));
        }
        #[cfg(feature = "VK_FUCHSIA_external_memory")]
        if self.fuchsia_external_memory() {
            out.push(cstr_ptr!("VK_FUCHSIA_external_memory"));
        }
        #[cfg(feature = "VK_KHR_driver_properties")]
        if self.khr_driver_properties() {
            out.push(cstr_ptr!("VK_KHR_driver_properties"));
        }
        #[cfg(feature = "VK_EXT_filter_cubic")]
        if self.ext_filter_cubic() {
            out.push(cstr_ptr!("VK_EXT_filter_cubic"));
        }
        #[cfg(feature = "VK_KHR_maintenance1")]
        if self.khr_maintenance_1() {
            out.push(cstr_ptr!("VK_KHR_maintenance1"));
        }
        #[cfg(feature = "VK_KHR_spirv_1_4")]
        if self.khr_spirv_1_4() {
            out.push(cstr_ptr!("VK_KHR_spirv_1_4"));
        }
        #[cfg(feature = "VK_AMD_texture_gather_bias_lod")]
        if self.amd_texture_gather_bias_lod() {
            out.push(cstr_ptr!("VK_AMD_texture_gather_bias_lod"));
        }
        #[cfg(feature = "VK_EXT_extended_dynamic_state")]
        if self.ext_extended_dynamic_state() {
            out.push(cstr_ptr!("VK_EXT_extended_dynamic_state"));
        }
        #[cfg(feature = "VK_EXT_conservative_rasterization")]
        if self.ext_conservative_rasterization() {
            out.push(cstr_ptr!("VK_EXT_conservative_rasterization"));
        }
        #[cfg(feature = "VK_EXT_primitive_topology_list_restart")]
        if self.ext_primitive_topology_list_restart() {
            out.push(cstr_ptr!("VK_EXT_primitive_topology_list_restart"));
        }
        #[cfg(feature = "VK_KHR_8bit_storage")]
        if self.khr_8_bit_storage() {
            out.push(cstr_ptr!("VK_KHR_8bit_storage"));
        }
        #[cfg(feature = "VK_EXT_border_color_swizzle")]
        if self.ext_border_color_swizzle() {
            out.push(cstr_ptr!("VK_EXT_border_color_swizzle"));
        }
        #[cfg(feature = "VK_EXT_calibrated_timestamps")]
        if self.ext_calibrated_timestamps() {
            out.push(cstr_ptr!("VK_EXT_calibrated_timestamps"));
        }
        #[cfg(feature = "VK_EXT_depth_range_unrestricted")]
        if self.ext_depth_range_unrestricted() {
            out.push(cstr_ptr!("VK_EXT_depth_range_unrestricted"));
        }
        #[cfg(feature = "VK_NV_shader_subgroup_partitioned")]
        if self.nv_shader_subgroup_partitioned() {
            out.push(cstr_ptr!("VK_NV_shader_subgroup_partitioned"));
        }
        #[cfg(feature = "VK_GOOGLE_hlsl_functionality1")]
        if self.google_hlsl_functionality_1() {
            out.push(cstr_ptr!("VK_GOOGLE_hlsl_functionality1"));
        }
        #[cfg(feature = "VK_EXT_memory_priority")]
        if self.ext_memory_priority() {
            out.push(cstr_ptr!("VK_EXT_memory_priority"));
        }
        #[cfg(feature = "VK_KHR_external_semaphore_fd")]
        if self.khr_external_semaphore_fd() {
            out.push(cstr_ptr!("VK_KHR_external_semaphore_fd"));
        }
        #[cfg(feature = "VK_GOOGLE_display_timing")]
        if self.google_display_timing() {
            out.push(cstr_ptr!("VK_GOOGLE_display_timing"));
        }
        #[cfg(feature = "VK_NV_mesh_shader")]
        if self.nv_mesh_shader() {
            out.push(cstr_ptr!("VK_NV_mesh_shader"));
        }
        #[cfg(feature = "VK_KHR_synchronization2")]
        if self.khr_synchronization_2() {
            out.push(cstr_ptr!("VK_KHR_synchronization2"));
        }
        #[cfg(feature = "VK_EXT_shader_subgroup_vote")]
        if self.ext_shader_subgroup_vote() {
            out.push(cstr_ptr!("VK_EXT_shader_subgroup_vote"));
        }
        #[cfg(feature = "VK_EXT_fragment_shader_interlock")]
        if self.ext_fragment_shader_interlock() {
            out.push(cstr_ptr!("VK_EXT_fragment_shader_interlock"));
        }
        #[cfg(feature = "VK_EXT_blend_operation_advanced")]
        if self.ext_blend_operation_advanced() {
            out.push(cstr_ptr!("VK_EXT_blend_operation_advanced"));
        }
        #[cfg(feature = "VK_EXT_descriptor_indexing")]
        if self.ext_descriptor_indexing() {
            out.push(cstr_ptr!("VK_EXT_descriptor_indexing"));
        }
        #[cfg(feature = "VK_EXT_inline_uniform_block")]
        if self.ext_inline_uniform_block() {
            out.push(cstr_ptr!("VK_EXT_inline_uniform_block"));
        }
        #[cfg(feature = "VK_KHR_video_encode_queue")]
        if self.khr_video_encode_queue() {
            out.push(cstr_ptr!("VK_KHR_video_encode_queue"));
        }
        #[cfg(feature = "VK_EXT_fragment_density_map2")]
        if self.ext_fragment_density_map_2() {
            out.push(cstr_ptr!("VK_EXT_fragment_density_map2"));
        }
        #[cfg(feature = "VK_EXT_debug_marker")]
        if self.ext_debug_marker() {
            out.push(cstr_ptr!("VK_EXT_debug_marker"));
        }
        #[cfg(feature = "VK_EXT_physical_device_drm")]
        if self.ext_physical_device_drm() {
            out.push(cstr_ptr!("VK_EXT_physical_device_drm"));
        }
        #[cfg(feature = "VK_NV_coverage_reduction_mode")]
        if self.nv_coverage_reduction_mode() {
            out.push(cstr_ptr!("VK_NV_coverage_reduction_mode"));
        }
        #[cfg(feature = "VK_KHR_shader_non_semantic_info")]
        if self.khr_shader_non_semantic_info() {
            out.push(cstr_ptr!("VK_KHR_shader_non_semantic_info"));
        }
        #[cfg(feature = "VK_ARM_rasterization_order_attachment_access")]
        if self.arm_rasterization_order_attachment_access() {
            out.push(cstr_ptr!("VK_ARM_rasterization_order_attachment_access"));
        }
        #[cfg(feature = "VK_KHR_shader_draw_parameters")]
        if self.khr_shader_draw_parameters() {
            out.push(cstr_ptr!("VK_KHR_shader_draw_parameters"));
        }
        #[cfg(feature = "VK_EXT_texture_compression_astc_hdr")]
        if self.ext_texture_compression_astc_hdr() {
            out.push(cstr_ptr!("VK_EXT_texture_compression_astc_hdr"));
        }
        #[cfg(feature = "VK_AMD_gpu_shader_half_float")]
        if self.amd_gpu_shader_half_float() {
            out.push(cstr_ptr!("VK_AMD_gpu_shader_half_float"));
        }
        #[cfg(feature = "VK_NV_device_diagnostic_checkpoints")]
        if self.nv_device_diagnostic_checkpoints() {
            out.push(cstr_ptr!("VK_NV_device_diagnostic_checkpoints"));
        }
        #[cfg(feature = "VK_KHR_video_decode_queue")]
        if self.khr_video_decode_queue() {
            out.push(cstr_ptr!("VK_KHR_video_decode_queue"));
        }
        #[cfg(feature = "VK_KHR_deferred_host_operations")]
        if self.khr_deferred_host_operations() {
            out.push(cstr_ptr!("VK_KHR_deferred_host_operations"));
        }
        #[cfg(feature = "VK_KHR_sampler_mirror_clamp_to_edge")]
        if self.khr_sampler_mirror_clamp_to_edge() {
            out.push(cstr_ptr!("VK_KHR_sampler_mirror_clamp_to_edge"));
        }
        #[cfg(feature = "VK_KHR_external_semaphore")]
        if self.khr_external_semaphore() {
            out.push(cstr_ptr!("VK_KHR_external_semaphore"));
        }
        #[cfg(feature = "VK_NV_viewport_array2")]
        if self.nv_viewport_array_2() {
            out.push(cstr_ptr!("VK_NV_viewport_array2"));
        }
        #[cfg(feature = "VK_KHR_external_fence_win32")]
        if self.khr_external_fence_win_32() {
            out.push(cstr_ptr!("VK_KHR_external_fence_win32"));
        }
        #[cfg(feature = "VK_EXT_ycbcr_image_arrays")]
        if self.ext_ycbcr_image_arrays() {
            out.push(cstr_ptr!("VK_EXT_ycbcr_image_arrays"));
        }
        #[cfg(feature = "VK_KHR_buffer_device_address")]
        if self.khr_buffer_device_address() {
            out.push(cstr_ptr!("VK_KHR_buffer_device_address"));
        }
        #[cfg(feature = "VK_AMD_pipeline_compiler_control")]
        if self.amd_pipeline_compiler_control() {
            out.push(cstr_ptr!("VK_AMD_pipeline_compiler_control"));
        }
        #[cfg(feature = "VK_KHR_swapchain_mutable_format")]
        if self.khr_swapchain_mutable_format() {
            out.push(cstr_ptr!("VK_KHR_swapchain_mutable_format"));
        }
        #[cfg(feature = "VK_QCOM_rotated_copy_commands")]
        if self.qcom_rotated_copy_commands() {
            out.push(cstr_ptr!("VK_QCOM_rotated_copy_commands"));
        }
        #[cfg(feature = "VK_EXT_color_write_enable")]
        if self.ext_color_write_enable() {
            out.push(cstr_ptr!("VK_EXT_color_write_enable"));
        }
        #[cfg(feature = "VK_EXT_image_drm_format_modifier")]
        if self.ext_image_drm_format_modifier() {
            out.push(cstr_ptr!("VK_EXT_image_drm_format_modifier"));
        }
        #[cfg(feature = "VK_GGP_frame_token")]
        if self.ggp_frame_token() {
            out.push(cstr_ptr!("VK_GGP_frame_token"));
        }
        #[cfg(feature = "VK_NV_external_memory")]
        if self.nv_external_memory() {
            out.push(cstr_ptr!("VK_NV_external_memory"));
        }
        #[cfg(feature = "VK_EXT_shader_stencil_export")]
        if self.ext_shader_stencil_export() {
            out.push(cstr_ptr!("VK_EXT_shader_stencil_export"));
        }
        #[cfg(feature = "VK_AMD_gcn_shader")]
        if self.amd_gcn_shader() {
            out.push(cstr_ptr!("VK_AMD_gcn_shader"));
        }
        #[cfg(feature = "VK_EXT_4444_formats")]
        if self.ext_4444_formats() {
            out.push(cstr_ptr!("VK_EXT_4444_formats"));
        }
        #[cfg(feature = "VK_EXT_texel_buffer_alignment")]
        if self.ext_texel_buffer_alignment() {
            out.push(cstr_ptr!("VK_EXT_texel_buffer_alignment"));
        }
        #[cfg(feature = "VK_KHR_shader_clock")]
        if self.khr_shader_clock() {
            out.push(cstr_ptr!("VK_KHR_shader_clock"));
        }
        #[cfg(feature = "VK_QCOM_fragment_density_map_offset")]
        if self.qcom_fragment_density_map_offset() {
            out.push(cstr_ptr!("VK_QCOM_fragment_density_map_offset"));
        }
        #[cfg(feature = "VK_EXT_hdr_metadata")]
        if self.ext_hdr_metadata() {
            out.push(cstr_ptr!("VK_EXT_hdr_metadata"));
        }
        #[cfg(feature = "VK_EXT_pipeline_creation_feedback")]
        if self.ext_pipeline_creation_feedback() {
            out.push(cstr_ptr!("VK_EXT_pipeline_creation_feedback"));
        }
        #[cfg(feature = "VK_NV_inherited_viewport_scissor")]
        if self.nv_inherited_viewport_scissor() {
            out.push(cstr_ptr!("VK_NV_inherited_viewport_scissor"));
        }
        #[cfg(feature = "VK_AMD_draw_indirect_count")]
        if self.amd_draw_indirect_count() {
            out.push(cstr_ptr!("VK_AMD_draw_indirect_count"));
        }
        #[cfg(feature = "VK_NV_device_diagnostics_config")]
        if self.nv_device_diagnostics_config() {
            out.push(cstr_ptr!("VK_NV_device_diagnostics_config"));
        }
        #[cfg(feature = "VK_KHR_dynamic_rendering")]
        if self.khr_dynamic_rendering() {
            out.push(cstr_ptr!("VK_KHR_dynamic_rendering"));
        }
        #[cfg(feature = "VK_AMD_device_coherent_memory")]
        if self.amd_device_coherent_memory() {
            out.push(cstr_ptr!("VK_AMD_device_coherent_memory"));
        }
        #[cfg(feature = "VK_AMD_buffer_marker")]
        if self.amd_buffer_marker() {
            out.push(cstr_ptr!("VK_AMD_buffer_marker"));
        }
        #[cfg(feature = "VK_NV_compute_shader_derivatives")]
        if self.nv_compute_shader_derivatives() {
            out.push(cstr_ptr!("VK_NV_compute_shader_derivatives"));
        }
        #[cfg(feature = "VK_NV_external_memory_rdma")]
        if self.nv_external_memory_rdma() {
            out.push(cstr_ptr!("VK_NV_external_memory_rdma"));
        }
        #[cfg(feature = "VK_EXT_private_data")]
        if self.ext_private_data() {
            out.push(cstr_ptr!("VK_EXT_private_data"));
        }
        #[cfg(feature = "VK_NV_ray_tracing_motion_blur")]
        if self.nv_ray_tracing_motion_blur() {
            out.push(cstr_ptr!("VK_NV_ray_tracing_motion_blur"));
        }
        #[cfg(feature = "VK_NV_glsl_shader")]
        if self.nv_glsl_shader() {
            out.push(cstr_ptr!("VK_NV_glsl_shader"));
        }
        #[cfg(feature = "VK_EXT_buffer_device_address")]
        if self.ext_buffer_device_address() {
            out.push(cstr_ptr!("VK_EXT_buffer_device_address"));
        }
        #[cfg(feature = "VK_EXT_host_query_reset")]
        if self.ext_host_query_reset() {
            out.push(cstr_ptr!("VK_EXT_host_query_reset"));
        }
        #[cfg(feature = "VK_EXT_load_store_op_none")]
        if self.ext_load_store_op_none() {
            out.push(cstr_ptr!("VK_EXT_load_store_op_none"));
        }
        #[cfg(feature = "VK_AMD_shader_core_properties2")]
        if self.amd_shader_core_properties_2() {
            out.push(cstr_ptr!("VK_AMD_shader_core_properties2"));
        }
        #[cfg(feature = "VK_QCOM_render_pass_store_ops")]
        if self.qcom_render_pass_store_ops() {
            out.push(cstr_ptr!("VK_QCOM_render_pass_store_ops"));
        }
        #[cfg(feature = "VK_NV_win32_keyed_mutex")]
        if self.nv_win_32_keyed_mutex() {
            out.push(cstr_ptr!("VK_NV_win32_keyed_mutex"));
        }
        out
    }
    ///Gets the list of extension names to use when creating the [`crate::vulkan1_0::Instance`
    pub fn instance_extension_names(&self) -> Vec<*const c_char> {
        let mut out = Vec::with_capacity(238usize);
        #[cfg(feature = "VK_MVK_ios_surface")]
        if self.mvk_ios_surface() {
            out.push(cstr_ptr!("VK_MVK_ios_surface"));
        }
        #[cfg(feature = "VK_KHR_xcb_surface")]
        if self.khr_xcb_surface() {
            out.push(cstr_ptr!("VK_KHR_xcb_surface"));
        }
        #[cfg(feature = "VK_KHR_get_display_properties2")]
        if self.khr_get_display_properties_2() {
            out.push(cstr_ptr!("VK_KHR_get_display_properties2"));
        }
        #[cfg(feature = "VK_NN_vi_surface")]
        if self.nn_vi_surface() {
            out.push(cstr_ptr!("VK_NN_vi_surface"));
        }
        #[cfg(feature = "VK_EXT_metal_surface")]
        if self.ext_metal_surface() {
            out.push(cstr_ptr!("VK_EXT_metal_surface"));
        }
        #[cfg(feature = "VK_MVK_macos_surface")]
        if self.mvk_macos_surface() {
            out.push(cstr_ptr!("VK_MVK_macos_surface"));
        }
        #[cfg(feature = "VK_KHR_android_surface")]
        if self.khr_android_surface() {
            out.push(cstr_ptr!("VK_KHR_android_surface"));
        }
        #[cfg(feature = "VK_NV_external_memory_capabilities")]
        if self.nv_external_memory_capabilities() {
            out.push(cstr_ptr!("VK_NV_external_memory_capabilities"));
        }
        #[cfg(feature = "VK_KHR_external_fence_capabilities")]
        if self.khr_external_fence_capabilities() {
            out.push(cstr_ptr!("VK_KHR_external_fence_capabilities"));
        }
        #[cfg(feature = "VK_EXT_debug_report")]
        if self.ext_debug_report() {
            out.push(cstr_ptr!("VK_EXT_debug_report"));
        }
        #[cfg(feature = "VK_EXT_validation_features")]
        if self.ext_validation_features() {
            out.push(cstr_ptr!("VK_EXT_validation_features"));
        }
        #[cfg(feature = "VK_GGP_stream_descriptor_surface")]
        if self.ggp_stream_descriptor_surface() {
            out.push(cstr_ptr!("VK_GGP_stream_descriptor_surface"));
        }
        #[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
        if self.khr_external_semaphore_capabilities() {
            out.push(cstr_ptr!("VK_KHR_external_semaphore_capabilities"));
        }
        #[cfg(feature = "VK_EXT_debug_utils")]
        if self.ext_debug_utils() {
            out.push(cstr_ptr!("VK_EXT_debug_utils"));
        }
        #[cfg(feature = "VK_KHR_wayland_surface")]
        if self.khr_wayland_surface() {
            out.push(cstr_ptr!("VK_KHR_wayland_surface"));
        }
        #[cfg(feature = "VK_EXT_direct_mode_display")]
        if self.ext_direct_mode_display() {
            out.push(cstr_ptr!("VK_EXT_direct_mode_display"));
        }
        #[cfg(feature = "VK_KHR_portability_enumeration")]
        if self.khr_portability_enumeration() {
            out.push(cstr_ptr!("VK_KHR_portability_enumeration"));
        }
        #[cfg(feature = "VK_KHR_external_memory_capabilities")]
        if self.khr_external_memory_capabilities() {
            out.push(cstr_ptr!("VK_KHR_external_memory_capabilities"));
        }
        #[cfg(feature = "VK_KHR_surface")]
        if self.khr_surface() {
            out.push(cstr_ptr!("VK_KHR_surface"));
        }
        #[cfg(feature = "VK_KHR_win32_surface")]
        if self.khr_win_32_surface() {
            out.push(cstr_ptr!("VK_KHR_win32_surface"));
        }
        #[cfg(feature = "VK_EXT_acquire_xlib_display")]
        if self.ext_acquire_xlib_display() {
            out.push(cstr_ptr!("VK_EXT_acquire_xlib_display"));
        }
        #[cfg(feature = "VK_GOOGLE_surfaceless_query")]
        if self.google_surfaceless_query() {
            out.push(cstr_ptr!("VK_GOOGLE_surfaceless_query"));
        }
        #[cfg(feature = "VK_EXT_acquire_drm_display")]
        if self.ext_acquire_drm_display() {
            out.push(cstr_ptr!("VK_EXT_acquire_drm_display"));
        }
        #[cfg(feature = "VK_QNX_screen_surface")]
        if self.qnx_screen_surface() {
            out.push(cstr_ptr!("VK_QNX_screen_surface"));
        }
        #[cfg(feature = "VK_FUCHSIA_imagepipe_surface")]
        if self.fuchsia_imagepipe_surface() {
            out.push(cstr_ptr!("VK_FUCHSIA_imagepipe_surface"));
        }
        #[cfg(feature = "VK_KHR_device_group_creation")]
        if self.khr_device_group_creation() {
            out.push(cstr_ptr!("VK_KHR_device_group_creation"));
        }
        #[cfg(feature = "VK_KHR_get_surface_capabilities2")]
        if self.khr_get_surface_capabilities_2() {
            out.push(cstr_ptr!("VK_KHR_get_surface_capabilities2"));
        }
        #[cfg(feature = "VK_EXT_display_surface_counter")]
        if self.ext_display_surface_counter() {
            out.push(cstr_ptr!("VK_EXT_display_surface_counter"));
        }
        #[cfg(feature = "VK_KHR_display")]
        if self.khr_display() {
            out.push(cstr_ptr!("VK_KHR_display"));
        }
        #[cfg(feature = "VK_KHR_surface_protected_capabilities")]
        if self.khr_surface_protected_capabilities() {
            out.push(cstr_ptr!("VK_KHR_surface_protected_capabilities"));
        }
        #[cfg(feature = "VK_KHR_get_physical_device_properties2")]
        if self.khr_get_physical_device_properties_2() {
            out.push(cstr_ptr!("VK_KHR_get_physical_device_properties2"));
        }
        #[cfg(feature = "VK_EXT_swapchain_colorspace")]
        if self.ext_swapchain_colorspace() {
            out.push(cstr_ptr!("VK_EXT_swapchain_colorspace"));
        }
        #[cfg(feature = "VK_EXT_directfb_surface")]
        if self.ext_directfb_surface() {
            out.push(cstr_ptr!("VK_EXT_directfb_surface"));
        }
        #[cfg(feature = "VK_EXT_headless_surface")]
        if self.ext_headless_surface() {
            out.push(cstr_ptr!("VK_EXT_headless_surface"));
        }
        #[cfg(feature = "VK_KHR_xlib_surface")]
        if self.khr_xlib_surface() {
            out.push(cstr_ptr!("VK_KHR_xlib_surface"));
        }
        #[cfg(feature = "VK_EXT_validation_flags")]
        if self.ext_validation_flags() {
            out.push(cstr_ptr!("VK_EXT_validation_flags"));
        }
        out
    }
}
