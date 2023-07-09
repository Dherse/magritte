pub use crate::common::vulkan1_1::{
    ChromaLocation, CommandPoolTrimFlags, DescriptorUpdateTemplateCreateFlags, DescriptorUpdateTemplateEntry,
    DescriptorUpdateTemplateType, DeviceQueueCreateFlagBits, ExternalFenceFeatureFlagBits, ExternalFenceFeatureFlags,
    ExternalFenceHandleTypeFlagBits, ExternalFenceHandleTypeFlags, ExternalMemoryFeatureFlagBits,
    ExternalMemoryFeatureFlags, ExternalMemoryHandleTypeFlagBits, ExternalMemoryHandleTypeFlags,
    ExternalMemoryProperties, ExternalSemaphoreFeatureFlagBits, ExternalSemaphoreFeatureFlags,
    ExternalSemaphoreHandleTypeFlagBits, ExternalSemaphoreHandleTypeFlags, FenceImportFlagBits, FenceImportFlags,
    InputAttachmentAspectReference, MemoryAllocateFlagBits, MemoryAllocateFlags, PeerMemoryFeatureFlagBits,
    PeerMemoryFeatureFlags, PointClippingBehavior, SamplerYcbcrModelConversion, SamplerYcbcrRange,
    SemaphoreImportFlagBits, SemaphoreImportFlags, SubgroupFeatureFlagBits, SubgroupFeatureFlags,
    TessellationDomainOrigin,
};
#[cfg(feature = "VK_AMD_device_coherent_memory")]
use crate::extensions::amd_device_coherent_memory::PhysicalDeviceCoherentMemoryFeaturesAMD;
#[cfg(feature = "VK_AMD_shader_core_properties")]
use crate::extensions::amd_shader_core_properties::PhysicalDeviceShaderCorePropertiesAMD;
#[cfg(feature = "VK_AMD_shader_core_properties2")]
use crate::extensions::amd_shader_core_properties2::PhysicalDeviceShaderCoreProperties2AMD;
#[cfg(feature = "VK_AMD_texture_gather_bias_lod")]
use crate::extensions::amd_texture_gather_bias_lod::TextureLodGatherFormatPropertiesAMD;
#[cfg(feature = "VK_ANDROID_external_memory_android_hardware_buffer")]
use crate::extensions::android_external_memory_android_hardware_buffer::AndroidHardwareBufferUsageANDROID;
#[cfg(feature = "VK_ANDROID_external_memory_android_hardware_buffer")]
use crate::extensions::android_external_memory_android_hardware_buffer::ExternalFormatANDROID;
#[cfg(feature = "VK_ARM_rasterization_order_attachment_access")]
use crate::extensions::arm_rasterization_order_attachment_access::PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM;
#[cfg(feature = "VK_EXT_4444_formats")]
use crate::extensions::ext_4444_formats::PhysicalDevice4444FormatsFeaturesEXT;
#[cfg(feature = "VK_EXT_astc_decode_mode")]
use crate::extensions::ext_astc_decode_mode::PhysicalDeviceAstcDecodeFeaturesEXT;
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
use crate::extensions::ext_blend_operation_advanced::PhysicalDeviceBlendOperationAdvancedFeaturesEXT;
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
use crate::extensions::ext_blend_operation_advanced::PhysicalDeviceBlendOperationAdvancedPropertiesEXT;
#[cfg(feature = "VK_EXT_border_color_swizzle")]
use crate::extensions::ext_border_color_swizzle::PhysicalDeviceBorderColorSwizzleFeaturesEXT;
#[cfg(feature = "VK_EXT_buffer_device_address")]
use crate::extensions::ext_buffer_device_address::PhysicalDeviceBufferDeviceAddressFeaturesEXT;
#[cfg(feature = "VK_EXT_color_write_enable")]
use crate::extensions::ext_color_write_enable::PhysicalDeviceColorWriteEnableFeaturesEXT;
#[cfg(feature = "VK_EXT_conditional_rendering")]
use crate::extensions::ext_conditional_rendering::PhysicalDeviceConditionalRenderingFeaturesEXT;
#[cfg(feature = "VK_EXT_conservative_rasterization")]
use crate::extensions::ext_conservative_rasterization::PhysicalDeviceConservativeRasterizationPropertiesEXT;
#[cfg(feature = "VK_EXT_custom_border_color")]
use crate::extensions::ext_custom_border_color::PhysicalDeviceCustomBorderColorFeaturesEXT;
#[cfg(feature = "VK_EXT_custom_border_color")]
use crate::extensions::ext_custom_border_color::PhysicalDeviceCustomBorderColorPropertiesEXT;
#[cfg(feature = "VK_EXT_depth_clip_control")]
use crate::extensions::ext_depth_clip_control::PhysicalDeviceDepthClipControlFeaturesEXT;
#[cfg(feature = "VK_EXT_depth_clip_enable")]
use crate::extensions::ext_depth_clip_enable::PhysicalDeviceDepthClipEnableFeaturesEXT;
#[cfg(feature = "VK_EXT_device_memory_report")]
use crate::extensions::ext_device_memory_report::PhysicalDeviceDeviceMemoryReportFeaturesEXT;
#[cfg(feature = "VK_EXT_discard_rectangles")]
use crate::extensions::ext_discard_rectangles::PhysicalDeviceDiscardRectanglePropertiesEXT;
#[cfg(feature = "VK_EXT_extended_dynamic_state")]
use crate::extensions::ext_extended_dynamic_state::PhysicalDeviceExtendedDynamicStateFeaturesEXT;
#[cfg(feature = "VK_EXT_extended_dynamic_state2")]
use crate::extensions::ext_extended_dynamic_state2::PhysicalDeviceExtendedDynamicState2FeaturesEXT;
#[cfg(feature = "VK_EXT_external_memory_host")]
use crate::extensions::ext_external_memory_host::PhysicalDeviceExternalMemoryHostPropertiesEXT;
#[cfg(feature = "VK_EXT_filter_cubic")]
use crate::extensions::ext_filter_cubic::FilterCubicImageViewImageFormatPropertiesEXT;
#[cfg(feature = "VK_EXT_filter_cubic")]
use crate::extensions::ext_filter_cubic::PhysicalDeviceImageViewImageFormatInfoEXT;
#[cfg(feature = "VK_EXT_fragment_density_map")]
use crate::extensions::ext_fragment_density_map::PhysicalDeviceFragmentDensityMapFeaturesEXT;
#[cfg(feature = "VK_EXT_fragment_density_map")]
use crate::extensions::ext_fragment_density_map::PhysicalDeviceFragmentDensityMapPropertiesEXT;
#[cfg(feature = "VK_EXT_fragment_density_map2")]
use crate::extensions::ext_fragment_density_map2::PhysicalDeviceFragmentDensityMap2FeaturesEXT;
#[cfg(feature = "VK_EXT_fragment_density_map2")]
use crate::extensions::ext_fragment_density_map2::PhysicalDeviceFragmentDensityMap2PropertiesEXT;
#[cfg(feature = "VK_EXT_fragment_shader_interlock")]
use crate::extensions::ext_fragment_shader_interlock::PhysicalDeviceFragmentShaderInterlockFeaturesEXT;
#[cfg(feature = "VK_EXT_image_drm_format_modifier")]
use crate::extensions::ext_image_drm_format_modifier::DrmFormatModifierPropertiesList2EXT;
#[cfg(feature = "VK_EXT_image_drm_format_modifier")]
use crate::extensions::ext_image_drm_format_modifier::DrmFormatModifierPropertiesListEXT;
#[cfg(feature = "VK_EXT_image_drm_format_modifier")]
use crate::extensions::ext_image_drm_format_modifier::PhysicalDeviceImageDrmFormatModifierInfoEXT;
#[cfg(feature = "VK_EXT_image_view_min_lod")]
use crate::extensions::ext_image_view_min_lod::PhysicalDeviceImageViewMinLodFeaturesEXT;
#[cfg(feature = "VK_EXT_index_type_uint8")]
use crate::extensions::ext_index_type_uint8::PhysicalDeviceIndexTypeUint8FeaturesEXT;
#[cfg(feature = "VK_EXT_line_rasterization")]
use crate::extensions::ext_line_rasterization::PhysicalDeviceLineRasterizationFeaturesEXT;
#[cfg(feature = "VK_EXT_line_rasterization")]
use crate::extensions::ext_line_rasterization::PhysicalDeviceLineRasterizationPropertiesEXT;
#[cfg(feature = "VK_EXT_memory_budget")]
use crate::extensions::ext_memory_budget::PhysicalDeviceMemoryBudgetPropertiesEXT;
#[cfg(feature = "VK_EXT_memory_priority")]
use crate::extensions::ext_memory_priority::PhysicalDeviceMemoryPriorityFeaturesEXT;
#[cfg(feature = "VK_EXT_multi_draw")]
use crate::extensions::ext_multi_draw::PhysicalDeviceMultiDrawFeaturesEXT;
#[cfg(feature = "VK_EXT_multi_draw")]
use crate::extensions::ext_multi_draw::PhysicalDeviceMultiDrawPropertiesEXT;
#[cfg(feature = "VK_EXT_pageable_device_local_memory")]
use crate::extensions::ext_pageable_device_local_memory::PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT;
#[cfg(feature = "VK_EXT_pci_bus_info")]
use crate::extensions::ext_pci_bus_info::PhysicalDevicePciBusInfoPropertiesEXT;
#[cfg(feature = "VK_EXT_physical_device_drm")]
use crate::extensions::ext_physical_device_drm::PhysicalDeviceDrmPropertiesEXT;
#[cfg(feature = "VK_EXT_primitive_topology_list_restart")]
use crate::extensions::ext_primitive_topology_list_restart::PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT;
#[cfg(feature = "VK_EXT_provoking_vertex")]
use crate::extensions::ext_provoking_vertex::PhysicalDeviceProvokingVertexFeaturesEXT;
#[cfg(feature = "VK_EXT_provoking_vertex")]
use crate::extensions::ext_provoking_vertex::PhysicalDeviceProvokingVertexPropertiesEXT;
#[cfg(feature = "VK_EXT_rgba10x6_formats")]
use crate::extensions::ext_rgba10x6_formats::PhysicalDeviceRgba10x6FormatsFeaturesEXT;
#[cfg(feature = "VK_EXT_robustness2")]
use crate::extensions::ext_robustness2::PhysicalDeviceRobustness2FeaturesEXT;
#[cfg(feature = "VK_EXT_robustness2")]
use crate::extensions::ext_robustness2::PhysicalDeviceRobustness2PropertiesEXT;
#[cfg(feature = "VK_EXT_sample_locations")]
use crate::extensions::ext_sample_locations::PhysicalDeviceSampleLocationsPropertiesEXT;
#[cfg(feature = "VK_EXT_shader_atomic_float")]
use crate::extensions::ext_shader_atomic_float::PhysicalDeviceShaderAtomicFloatFeaturesEXT;
#[cfg(feature = "VK_EXT_shader_atomic_float2")]
use crate::extensions::ext_shader_atomic_float2::PhysicalDeviceShaderAtomicFloat2FeaturesEXT;
#[cfg(feature = "VK_EXT_shader_image_atomic_int64")]
use crate::extensions::ext_shader_image_atomic_int64::PhysicalDeviceShaderImageAtomicInt64FeaturesEXT;
#[cfg(feature = "VK_EXT_texel_buffer_alignment")]
use crate::extensions::ext_texel_buffer_alignment::PhysicalDeviceTexelBufferAlignmentFeaturesEXT;
#[cfg(feature = "VK_EXT_transform_feedback")]
use crate::extensions::ext_transform_feedback::PhysicalDeviceTransformFeedbackFeaturesEXT;
#[cfg(feature = "VK_EXT_transform_feedback")]
use crate::extensions::ext_transform_feedback::PhysicalDeviceTransformFeedbackPropertiesEXT;
#[cfg(feature = "VK_EXT_vertex_attribute_divisor")]
use crate::extensions::ext_vertex_attribute_divisor::PhysicalDeviceVertexAttributeDivisorFeaturesEXT;
#[cfg(feature = "VK_EXT_vertex_attribute_divisor")]
use crate::extensions::ext_vertex_attribute_divisor::PhysicalDeviceVertexAttributeDivisorPropertiesEXT;
#[cfg(feature = "VK_EXT_vertex_input_dynamic_state")]
use crate::extensions::ext_vertex_input_dynamic_state::PhysicalDeviceVertexInputDynamicStateFeaturesEXT;
#[cfg(feature = "VK_EXT_ycbcr_2plane_444_formats")]
use crate::extensions::ext_ycbcr_2plane_444_formats::PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT;
#[cfg(feature = "VK_EXT_ycbcr_image_arrays")]
use crate::extensions::ext_ycbcr_image_arrays::PhysicalDeviceYcbcrImageArraysFeaturesEXT;
#[cfg(feature = "VK_HUAWEI_invocation_mask")]
use crate::extensions::huawei_invocation_mask::PhysicalDeviceInvocationMaskFeaturesHUAWEI;
#[cfg(feature = "VK_HUAWEI_subpass_shading")]
use crate::extensions::huawei_subpass_shading::PhysicalDeviceSubpassShadingFeaturesHUAWEI;
#[cfg(feature = "VK_HUAWEI_subpass_shading")]
use crate::extensions::huawei_subpass_shading::PhysicalDeviceSubpassShadingPropertiesHUAWEI;
#[cfg(feature = "VK_INTEL_shader_integer_functions2")]
use crate::extensions::intel_shader_integer_functions2::PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL;
#[cfg(feature = "VK_KHR_acceleration_structure")]
use crate::extensions::khr_acceleration_structure::PhysicalDeviceAccelerationStructureFeaturesKHR;
#[cfg(feature = "VK_KHR_acceleration_structure")]
use crate::extensions::khr_acceleration_structure::PhysicalDeviceAccelerationStructurePropertiesKHR;
#[cfg(feature = "VK_KHR_device_group")]
use crate::extensions::khr_device_group::BindImageMemorySwapchainInfoKHR;
#[cfg(feature = "VK_KHR_fragment_shading_rate")]
use crate::extensions::khr_fragment_shading_rate::PhysicalDeviceFragmentShadingRateFeaturesKHR;
#[cfg(feature = "VK_KHR_fragment_shading_rate")]
use crate::extensions::khr_fragment_shading_rate::PhysicalDeviceFragmentShadingRatePropertiesKHR;
#[cfg(feature = "VK_KHR_global_priority")]
use crate::extensions::khr_global_priority::PhysicalDeviceGlobalPriorityQueryFeaturesKHR;
#[cfg(feature = "VK_KHR_global_priority")]
use crate::extensions::khr_global_priority::QueueFamilyGlobalPriorityPropertiesKHR;
#[cfg(feature = "VK_KHR_performance_query")]
use crate::extensions::khr_performance_query::PhysicalDevicePerformanceQueryFeaturesKHR;
#[cfg(feature = "VK_KHR_performance_query")]
use crate::extensions::khr_performance_query::PhysicalDevicePerformanceQueryPropertiesKHR;
#[cfg(feature = "VK_KHR_pipeline_executable_properties")]
use crate::extensions::khr_pipeline_executable_properties::PhysicalDevicePipelineExecutablePropertiesFeaturesKHR;
#[cfg(feature = "VK_KHR_portability_subset")]
use crate::extensions::khr_portability_subset::PhysicalDevicePortabilitySubsetFeaturesKHR;
#[cfg(feature = "VK_KHR_portability_subset")]
use crate::extensions::khr_portability_subset::PhysicalDevicePortabilitySubsetPropertiesKHR;
#[cfg(feature = "VK_KHR_present_id")]
use crate::extensions::khr_present_id::PhysicalDevicePresentIdFeaturesKHR;
#[cfg(feature = "VK_KHR_present_wait")]
use crate::extensions::khr_present_wait::PhysicalDevicePresentWaitFeaturesKHR;
#[cfg(feature = "VK_KHR_push_descriptor")]
use crate::extensions::khr_push_descriptor::PhysicalDevicePushDescriptorPropertiesKHR;
#[cfg(feature = "VK_KHR_ray_query")]
use crate::extensions::khr_ray_query::PhysicalDeviceRayQueryFeaturesKHR;
#[cfg(feature = "VK_KHR_ray_tracing_pipeline")]
use crate::extensions::khr_ray_tracing_pipeline::PhysicalDeviceRayTracingPipelineFeaturesKHR;
#[cfg(feature = "VK_KHR_ray_tracing_pipeline")]
use crate::extensions::khr_ray_tracing_pipeline::PhysicalDeviceRayTracingPipelinePropertiesKHR;
#[cfg(feature = "VK_KHR_shader_clock")]
use crate::extensions::khr_shader_clock::PhysicalDeviceShaderClockFeaturesKHR;
#[cfg(feature = "VK_KHR_shader_subgroup_uniform_control_flow")]
use crate::extensions::khr_shader_subgroup_uniform_control_flow::PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR;
#[cfg(feature = "VK_KHR_synchronization2")]
use crate::extensions::khr_synchronization2::QueueFamilyCheckpointProperties2NV;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::extensions::khr_video_queue::QueueFamilyQueryResultStatusProperties2KHR;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::extensions::khr_video_queue::VideoProfileKHR;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::extensions::khr_video_queue::VideoProfilesKHR;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::extensions::khr_video_queue::VideoQueueFamilyProperties2KHR;
#[cfg(feature = "VK_KHR_workgroup_memory_explicit_layout")]
use crate::extensions::khr_workgroup_memory_explicit_layout::PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR;
#[cfg(feature = "VK_NV_compute_shader_derivatives")]
use crate::extensions::nv_compute_shader_derivatives::PhysicalDeviceComputeShaderDerivativesFeaturesNV;
#[cfg(feature = "VK_NV_cooperative_matrix")]
use crate::extensions::nv_cooperative_matrix::PhysicalDeviceCooperativeMatrixFeaturesNV;
#[cfg(feature = "VK_NV_cooperative_matrix")]
use crate::extensions::nv_cooperative_matrix::PhysicalDeviceCooperativeMatrixPropertiesNV;
#[cfg(feature = "VK_NV_corner_sampled_image")]
use crate::extensions::nv_corner_sampled_image::PhysicalDeviceCornerSampledImageFeaturesNV;
#[cfg(feature = "VK_NV_coverage_reduction_mode")]
use crate::extensions::nv_coverage_reduction_mode::PhysicalDeviceCoverageReductionModeFeaturesNV;
#[cfg(feature = "VK_NV_dedicated_allocation_image_aliasing")]
use crate::extensions::nv_dedicated_allocation_image_aliasing::PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV;
#[cfg(feature = "VK_NV_device_diagnostic_checkpoints")]
use crate::extensions::nv_device_diagnostic_checkpoints::QueueFamilyCheckpointPropertiesNV;
#[cfg(feature = "VK_NV_device_diagnostics_config")]
use crate::extensions::nv_device_diagnostics_config::PhysicalDeviceDiagnosticsConfigFeaturesNV;
#[cfg(feature = "VK_NV_device_generated_commands")]
use crate::extensions::nv_device_generated_commands::PhysicalDeviceDeviceGeneratedCommandsFeaturesNV;
#[cfg(feature = "VK_NV_device_generated_commands")]
use crate::extensions::nv_device_generated_commands::PhysicalDeviceDeviceGeneratedCommandsPropertiesNV;
#[cfg(feature = "VK_NV_external_memory_rdma")]
use crate::extensions::nv_external_memory_rdma::PhysicalDeviceExternalMemoryRdmaFeaturesNV;
#[cfg(feature = "VK_NV_fragment_shader_barycentric")]
use crate::extensions::nv_fragment_shader_barycentric::PhysicalDeviceFragmentShaderBarycentricFeaturesNV;
#[cfg(feature = "VK_NV_fragment_shading_rate_enums")]
use crate::extensions::nv_fragment_shading_rate_enums::PhysicalDeviceFragmentShadingRateEnumsFeaturesNV;
#[cfg(feature = "VK_NV_fragment_shading_rate_enums")]
use crate::extensions::nv_fragment_shading_rate_enums::PhysicalDeviceFragmentShadingRateEnumsPropertiesNV;
#[cfg(feature = "VK_NV_inherited_viewport_scissor")]
use crate::extensions::nv_inherited_viewport_scissor::PhysicalDeviceInheritedViewportScissorFeaturesNV;
#[cfg(feature = "VK_NV_linear_color_attachment")]
use crate::extensions::nv_linear_color_attachment::PhysicalDeviceLinearColorAttachmentFeaturesNV;
#[cfg(feature = "VK_NV_mesh_shader")]
use crate::extensions::nv_mesh_shader::PhysicalDeviceMeshShaderFeaturesNV;
#[cfg(feature = "VK_NV_mesh_shader")]
use crate::extensions::nv_mesh_shader::PhysicalDeviceMeshShaderPropertiesNV;
#[cfg(feature = "VK_NV_ray_tracing")]
use crate::extensions::nv_ray_tracing::PhysicalDeviceRayTracingPropertiesNV;
#[cfg(feature = "VK_NV_ray_tracing_motion_blur")]
use crate::extensions::nv_ray_tracing_motion_blur::PhysicalDeviceRayTracingMotionBlurFeaturesNV;
#[cfg(feature = "VK_NV_representative_fragment_test")]
use crate::extensions::nv_representative_fragment_test::PhysicalDeviceRepresentativeFragmentTestFeaturesNV;
#[cfg(feature = "VK_NV_scissor_exclusive")]
use crate::extensions::nv_scissor_exclusive::PhysicalDeviceExclusiveScissorFeaturesNV;
#[cfg(feature = "VK_NV_shader_image_footprint")]
use crate::extensions::nv_shader_image_footprint::PhysicalDeviceShaderImageFootprintFeaturesNV;
#[cfg(feature = "VK_NV_shader_sm_builtins")]
use crate::extensions::nv_shader_sm_builtins::PhysicalDeviceShaderSmBuiltinsFeaturesNV;
#[cfg(feature = "VK_NV_shader_sm_builtins")]
use crate::extensions::nv_shader_sm_builtins::PhysicalDeviceShaderSmBuiltinsPropertiesNV;
#[cfg(feature = "VK_NV_shading_rate_image")]
use crate::extensions::nv_shading_rate_image::PhysicalDeviceShadingRateImageFeaturesNV;
#[cfg(feature = "VK_NV_shading_rate_image")]
use crate::extensions::nv_shading_rate_image::PhysicalDeviceShadingRateImagePropertiesNV;
#[cfg(feature = "VK_NVX_multiview_per_view_attributes")]
use crate::extensions::nvx_multiview_per_view_attributes::PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX;
#[cfg(feature = "VK_VALVE_mutable_descriptor_type")]
use crate::extensions::valve_mutable_descriptor_type::PhysicalDeviceMutableDescriptorTypeFeaturesVALVE;
#[cfg(feature = "VULKAN_1_2")]
use crate::vulkan1_2::DescriptorSetVariableDescriptorCountLayoutSupport;
#[cfg(feature = "VULKAN_1_2")]
use crate::vulkan1_2::ImageFormatListCreateInfo;
#[cfg(feature = "VULKAN_1_2")]
use crate::vulkan1_2::ImageStencilUsageCreateInfo;
#[cfg(feature = "VULKAN_1_2")]
use crate::vulkan1_2::PhysicalDevice8BitStorageFeatures;
#[cfg(feature = "VULKAN_1_2")]
use crate::vulkan1_2::PhysicalDeviceBufferDeviceAddressFeatures;
#[cfg(feature = "VULKAN_1_2")]
use crate::vulkan1_2::PhysicalDeviceDepthStencilResolveProperties;
#[cfg(feature = "VULKAN_1_2")]
use crate::vulkan1_2::PhysicalDeviceDescriptorIndexingFeatures;
#[cfg(feature = "VULKAN_1_2")]
use crate::vulkan1_2::PhysicalDeviceDescriptorIndexingProperties;
#[cfg(feature = "VULKAN_1_2")]
use crate::vulkan1_2::PhysicalDeviceDriverProperties;
#[cfg(feature = "VULKAN_1_2")]
use crate::vulkan1_2::PhysicalDeviceFloatControlsProperties;
#[cfg(feature = "VULKAN_1_2")]
use crate::vulkan1_2::PhysicalDeviceHostQueryResetFeatures;
#[cfg(feature = "VULKAN_1_2")]
use crate::vulkan1_2::PhysicalDeviceImagelessFramebufferFeatures;
#[cfg(feature = "VULKAN_1_2")]
use crate::vulkan1_2::PhysicalDeviceSamplerFilterMinmaxProperties;
#[cfg(feature = "VULKAN_1_2")]
use crate::vulkan1_2::PhysicalDeviceScalarBlockLayoutFeatures;
#[cfg(feature = "VULKAN_1_2")]
use crate::vulkan1_2::PhysicalDeviceSeparateDepthStencilLayoutsFeatures;
#[cfg(feature = "VULKAN_1_2")]
use crate::vulkan1_2::PhysicalDeviceShaderAtomicInt64Features;
#[cfg(feature = "VULKAN_1_2")]
use crate::vulkan1_2::PhysicalDeviceShaderFloat16Int8Features;
#[cfg(feature = "VULKAN_1_2")]
use crate::vulkan1_2::PhysicalDeviceShaderSubgroupExtendedTypesFeatures;
#[cfg(feature = "VULKAN_1_2")]
use crate::vulkan1_2::PhysicalDeviceTimelineSemaphoreFeatures;
#[cfg(feature = "VULKAN_1_2")]
use crate::vulkan1_2::PhysicalDeviceTimelineSemaphoreProperties;
#[cfg(feature = "VULKAN_1_2")]
use crate::vulkan1_2::PhysicalDeviceUniformBufferStandardLayoutFeatures;
#[cfg(feature = "VULKAN_1_2")]
use crate::vulkan1_2::PhysicalDeviceVulkan11Features;
#[cfg(feature = "VULKAN_1_2")]
use crate::vulkan1_2::PhysicalDeviceVulkan11Properties;
#[cfg(feature = "VULKAN_1_2")]
use crate::vulkan1_2::PhysicalDeviceVulkan12Features;
#[cfg(feature = "VULKAN_1_2")]
use crate::vulkan1_2::PhysicalDeviceVulkan12Properties;
#[cfg(feature = "VULKAN_1_2")]
use crate::vulkan1_2::PhysicalDeviceVulkanMemoryModelFeatures;
#[cfg(feature = "VULKAN_1_2")]
use crate::vulkan1_2::SemaphoreTypeCreateInfo;
#[cfg(feature = "VULKAN_1_3")]
use crate::vulkan1_3::FormatProperties3;
#[cfg(feature = "VULKAN_1_3")]
use crate::vulkan1_3::PhysicalDeviceDynamicRenderingFeatures;
#[cfg(feature = "VULKAN_1_3")]
use crate::vulkan1_3::PhysicalDeviceImageRobustnessFeatures;
#[cfg(feature = "VULKAN_1_3")]
use crate::vulkan1_3::PhysicalDeviceInlineUniformBlockFeatures;
#[cfg(feature = "VULKAN_1_3")]
use crate::vulkan1_3::PhysicalDeviceInlineUniformBlockProperties;
#[cfg(feature = "VULKAN_1_3")]
use crate::vulkan1_3::PhysicalDeviceMaintenance4Features;
#[cfg(feature = "VULKAN_1_3")]
use crate::vulkan1_3::PhysicalDeviceMaintenance4Properties;
#[cfg(feature = "VULKAN_1_3")]
use crate::vulkan1_3::PhysicalDevicePipelineCreationCacheControlFeatures;
#[cfg(feature = "VULKAN_1_3")]
use crate::vulkan1_3::PhysicalDevicePrivateDataFeatures;
#[cfg(feature = "VULKAN_1_3")]
use crate::vulkan1_3::PhysicalDeviceShaderDemoteToHelperInvocationFeatures;
#[cfg(feature = "VULKAN_1_3")]
use crate::vulkan1_3::PhysicalDeviceShaderIntegerDotProductFeatures;
#[cfg(feature = "VULKAN_1_3")]
use crate::vulkan1_3::PhysicalDeviceShaderIntegerDotProductProperties;
#[cfg(feature = "VULKAN_1_3")]
use crate::vulkan1_3::PhysicalDeviceShaderTerminateInvocationFeatures;
#[cfg(feature = "VULKAN_1_3")]
use crate::vulkan1_3::PhysicalDeviceSubgroupSizeControlFeatures;
#[cfg(feature = "VULKAN_1_3")]
use crate::vulkan1_3::PhysicalDeviceSubgroupSizeControlProperties;
#[cfg(feature = "VULKAN_1_3")]
use crate::vulkan1_3::PhysicalDeviceSynchronization2Features;
#[cfg(feature = "VULKAN_1_3")]
use crate::vulkan1_3::PhysicalDeviceTexelBufferAlignmentProperties;
#[cfg(feature = "VULKAN_1_3")]
use crate::vulkan1_3::PhysicalDeviceTextureCompressionAstchdrFeatures;
#[cfg(feature = "VULKAN_1_3")]
use crate::vulkan1_3::PhysicalDeviceVulkan13Features;
#[cfg(feature = "VULKAN_1_3")]
use crate::vulkan1_3::PhysicalDeviceVulkan13Properties;
#[cfg(feature = "VULKAN_1_3")]
use crate::vulkan1_3::PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures;
use crate::{
    context::{Container, Context, Error, ObjectId},
    vulkan1_0::{
        Buffer, BufferCreateFlags, BufferUsageFlags, ComponentMapping, DescriptorSetLayout, DescriptorType,
        DeviceMemory, DeviceQueueCreateFlags, DeviceSize, Filter, Format, FormatProperties, Image, ImageAspectFlagBits,
        ImageAspectFlags, ImageCreateFlags, ImageFormatProperties, ImageTiling, ImageType, ImageUsageFlags,
        MemoryRequirements, PhysicalDevice, PhysicalDeviceFeatures, PhysicalDeviceMemoryProperties,
        PhysicalDeviceProperties, PipelineBindPoint, PipelineLayout, QueueFamilyProperties, Rect2D,
        SampleCountFlagBits, ShaderStageFlags, SparseImageFormatProperties, SparseImageMemoryRequirements,
        StructureType, LUID_SIZE, MAX_DEVICE_GROUP_SIZE,
    },
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use std::sync::Arc;
#[doc(alias = "VkPhysicalDeviceVariablePointerFeatures")]
pub type PhysicalDeviceVariablePointerFeatures = PhysicalDeviceVariablePointersFeatures;
#[doc(alias = "VkPhysicalDeviceShaderDrawParameterFeatures")]
pub type PhysicalDeviceShaderDrawParameterFeatures = PhysicalDeviceShaderDrawParametersFeatures;
#[doc(alias = "VkPhysicalDeviceFeatures2")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceFeatures2 {
    #[doc(alias = "pNext")]
    pub extensions: SmallVec<[PhysicalDeviceFeatures2Extension; 1]>,
    pub features: PhysicalDeviceFeatures,
}
impl PhysicalDeviceFeatures2 {
    ///Adds an extension to the struct
    pub fn with_extension(mut self, ext: impl Into<PhysicalDeviceFeatures2Extension>) -> Self {
        self.extensions.push(ext.into());
        self
    }
    ///Get a reference to the `extensions` field.
    pub fn extensions(&self) -> &SmallVec<[PhysicalDeviceFeatures2Extension; 1]> {
        &self.extensions
    }
    ///Get a reference to the `features` field.
    pub fn features(&self) -> &PhysicalDeviceFeatures {
        &self.features
    }
    ///Get a mutable reference to the `extensions` field.
    pub fn extensions_mut(&mut self) -> &mut SmallVec<[PhysicalDeviceFeatures2Extension; 1]> {
        &mut self.extensions
    }
    ///Get a mutable reference to the `features` field.
    pub fn features_mut(&mut self) -> &mut PhysicalDeviceFeatures {
        &mut self.features
    }
    ///Sets the `extensions` field.
    pub fn set_extensions(&mut self, extensions: SmallVec<[PhysicalDeviceFeatures2Extension; 1]>) -> &mut Self {
        self.extensions = extensions;
        self
    }
    ///Sets the `features` field.
    pub fn set_features(&mut self, features: PhysicalDeviceFeatures) -> &mut Self {
        self.features = features;
        self
    }
    ///Sets the `extensions` field in a builder way.
    pub fn with_extensions(mut self, extensions: SmallVec<[PhysicalDeviceFeatures2Extension; 1]>) -> Self {
        self.extensions = extensions;
        self
    }
    ///Sets the `features` field in a builder way.
    pub fn with_features(mut self, features: PhysicalDeviceFeatures) -> Self {
        self.features = features;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceFeatures2 {
    type LowLevel = crate::native::vulkan1_1::PhysicalDeviceFeatures2;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let mut next = std::ptr::null_mut();
        let mut extensions = self.extensions.iter();
        while let Some(ext) = extensions.next() {
            let ext = ext.into_low_level(context, bump);
            (*ext).next = next;
            next = ext;
        }
        crate::native::vulkan1_1::PhysicalDeviceFeatures2 {
            s_type: StructureType::PhysicalDeviceFeatures2,
            p_next: next,
            features: self.features.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceFeatures2 {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let mut next = value.p_next;
        let mut extensions = SmallVec::new();
        while !next.is_null() {
            extensions.push(crate::conv::FromLowLevel::from_low_level(context, next));
            next = std::ptr::read(next).next;
        }
        Self {
            extensions: extensions,
            features: crate::conv::FromLowLevel::from_low_level(context, value.features),
        }
    }
}
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
///Extensions for [`PhysicalDeviceFeatures2`]
pub enum PhysicalDeviceFeatures2Extension {
    #[cfg(feature = "VK_NV_device_generated_commands")]
    ///Contains a type [`PhysicalDeviceDeviceGeneratedCommandsFeaturesNV`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDeviceDeviceGeneratedCommandsFeaturesNV(PhysicalDeviceDeviceGeneratedCommandsFeaturesNV),
    #[cfg(feature = "VULKAN_1_3")]
    ///Contains a type [`PhysicalDevicePrivateDataFeatures`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDevicePrivateDataFeatures(PhysicalDevicePrivateDataFeatures),
    ///Contains a type [`PhysicalDeviceVariablePointersFeatures`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDeviceVariablePointersFeatures(PhysicalDeviceVariablePointersFeatures),
    ///Contains a type [`PhysicalDeviceMultiviewFeatures`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDeviceMultiviewFeatures(PhysicalDeviceMultiviewFeatures),
    #[cfg(feature = "VK_KHR_present_id")]
    ///Contains a type [`PhysicalDevicePresentIdFeaturesKHR`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDevicePresentIdFeaturesKHR(PhysicalDevicePresentIdFeaturesKHR),
    #[cfg(feature = "VK_KHR_present_wait")]
    ///Contains a type [`PhysicalDevicePresentWaitFeaturesKHR`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDevicePresentWaitFeaturesKHR(PhysicalDevicePresentWaitFeaturesKHR),
    ///Contains a type [`PhysicalDevice16BitStorageFeatures`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDevice16BitStorageFeatures(PhysicalDevice16BitStorageFeatures),
    #[cfg(feature = "VULKAN_1_2")]
    ///Contains a type [`PhysicalDeviceShaderSubgroupExtendedTypesFeatures`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDeviceShaderSubgroupExtendedTypesFeatures(PhysicalDeviceShaderSubgroupExtendedTypesFeatures),
    ///Contains a type [`PhysicalDeviceSamplerYcbcrConversionFeatures`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDeviceSamplerYcbcrConversionFeatures(PhysicalDeviceSamplerYcbcrConversionFeatures),
    ///Contains a type [`PhysicalDeviceProtectedMemoryFeatures`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDeviceProtectedMemoryFeatures(PhysicalDeviceProtectedMemoryFeatures),
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    ///Contains a type [`PhysicalDeviceBlendOperationAdvancedFeaturesEXT`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDeviceBlendOperationAdvancedFeaturesEXT(PhysicalDeviceBlendOperationAdvancedFeaturesEXT),
    #[cfg(feature = "VK_EXT_multi_draw")]
    ///Contains a type [`PhysicalDeviceMultiDrawFeaturesEXT`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDeviceMultiDrawFeaturesEXT(PhysicalDeviceMultiDrawFeaturesEXT),
    #[cfg(feature = "VULKAN_1_3")]
    ///Contains a type [`PhysicalDeviceInlineUniformBlockFeatures`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDeviceInlineUniformBlockFeatures(PhysicalDeviceInlineUniformBlockFeatures),
    #[cfg(feature = "VULKAN_1_3")]
    ///Contains a type [`PhysicalDeviceMaintenance4Features`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDeviceMaintenance4Features(PhysicalDeviceMaintenance4Features),
    ///Contains a type [`PhysicalDeviceShaderDrawParametersFeatures`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDeviceShaderDrawParametersFeatures(PhysicalDeviceShaderDrawParametersFeatures),
    #[cfg(feature = "VULKAN_1_2")]
    ///Contains a type [`PhysicalDeviceShaderFloat16Int8Features`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDeviceShaderFloat16Int8Features(PhysicalDeviceShaderFloat16Int8Features),
    #[cfg(feature = "VULKAN_1_2")]
    ///Contains a type [`PhysicalDeviceHostQueryResetFeatures`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDeviceHostQueryResetFeatures(PhysicalDeviceHostQueryResetFeatures),
    #[cfg(feature = "VK_KHR_global_priority")]
    ///Contains a type [`PhysicalDeviceGlobalPriorityQueryFeaturesKHR`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDeviceGlobalPriorityQueryFeaturesKHR(PhysicalDeviceGlobalPriorityQueryFeaturesKHR),
    #[cfg(feature = "VK_EXT_device_memory_report")]
    ///Contains a type [`PhysicalDeviceDeviceMemoryReportFeaturesEXT`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDeviceDeviceMemoryReportFeaturesEXT(PhysicalDeviceDeviceMemoryReportFeaturesEXT),
    #[cfg(feature = "VULKAN_1_2")]
    ///Contains a type [`PhysicalDeviceDescriptorIndexingFeatures`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDeviceDescriptorIndexingFeatures(PhysicalDeviceDescriptorIndexingFeatures),
    #[cfg(feature = "VULKAN_1_2")]
    ///Contains a type [`PhysicalDeviceTimelineSemaphoreFeatures`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDeviceTimelineSemaphoreFeatures(PhysicalDeviceTimelineSemaphoreFeatures),
    #[cfg(feature = "VULKAN_1_2")]
    ///Contains a type [`PhysicalDevice8BitStorageFeatures`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDevice8BitStorageFeatures(PhysicalDevice8BitStorageFeatures),
    #[cfg(feature = "VK_EXT_conditional_rendering")]
    ///Contains a type [`PhysicalDeviceConditionalRenderingFeaturesEXT`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDeviceConditionalRenderingFeaturesEXT(PhysicalDeviceConditionalRenderingFeaturesEXT),
    #[cfg(feature = "VULKAN_1_2")]
    ///Contains a type [`PhysicalDeviceVulkanMemoryModelFeatures`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDeviceVulkanMemoryModelFeatures(PhysicalDeviceVulkanMemoryModelFeatures),
    #[cfg(feature = "VULKAN_1_2")]
    ///Contains a type [`PhysicalDeviceShaderAtomicInt64Features`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDeviceShaderAtomicInt64Features(PhysicalDeviceShaderAtomicInt64Features),
    #[cfg(feature = "VK_EXT_shader_atomic_float")]
    ///Contains a type [`PhysicalDeviceShaderAtomicFloatFeaturesEXT`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDeviceShaderAtomicFloatFeaturesEXT(PhysicalDeviceShaderAtomicFloatFeaturesEXT),
    #[cfg(feature = "VK_EXT_shader_atomic_float2")]
    ///Contains a type [`PhysicalDeviceShaderAtomicFloat2FeaturesEXT`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDeviceShaderAtomicFloat2FeaturesEXT(PhysicalDeviceShaderAtomicFloat2FeaturesEXT),
    #[cfg(feature = "VK_EXT_vertex_attribute_divisor")]
    ///Contains a type [`PhysicalDeviceVertexAttributeDivisorFeaturesEXT`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDeviceVertexAttributeDivisorFeaturesEXT(PhysicalDeviceVertexAttributeDivisorFeaturesEXT),
    #[cfg(feature = "VK_EXT_astc_decode_mode")]
    ///Contains a type [`PhysicalDeviceAstcDecodeFeaturesEXT`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDeviceAstcDecodeFeaturesEXT(PhysicalDeviceAstcDecodeFeaturesEXT),
    #[cfg(feature = "VK_EXT_transform_feedback")]
    ///Contains a type [`PhysicalDeviceTransformFeedbackFeaturesEXT`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDeviceTransformFeedbackFeaturesEXT(PhysicalDeviceTransformFeedbackFeaturesEXT),
    #[cfg(feature = "VK_NV_representative_fragment_test")]
    ///Contains a type [`PhysicalDeviceRepresentativeFragmentTestFeaturesNV`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDeviceRepresentativeFragmentTestFeaturesNV(PhysicalDeviceRepresentativeFragmentTestFeaturesNV),
    #[cfg(feature = "VK_NV_scissor_exclusive")]
    ///Contains a type [`PhysicalDeviceExclusiveScissorFeaturesNV`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDeviceExclusiveScissorFeaturesNV(PhysicalDeviceExclusiveScissorFeaturesNV),
    #[cfg(feature = "VK_NV_corner_sampled_image")]
    ///Contains a type [`PhysicalDeviceCornerSampledImageFeaturesNV`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDeviceCornerSampledImageFeaturesNV(PhysicalDeviceCornerSampledImageFeaturesNV),
    #[cfg(feature = "VK_NV_compute_shader_derivatives")]
    ///Contains a type [`PhysicalDeviceComputeShaderDerivativesFeaturesNV`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDeviceComputeShaderDerivativesFeaturesNV(PhysicalDeviceComputeShaderDerivativesFeaturesNV),
    #[cfg(feature = "VK_NV_fragment_shader_barycentric")]
    ///Contains a type [`PhysicalDeviceFragmentShaderBarycentricFeaturesNV`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDeviceFragmentShaderBarycentricFeaturesNV(PhysicalDeviceFragmentShaderBarycentricFeaturesNV),
    #[cfg(feature = "VK_NV_shader_image_footprint")]
    ///Contains a type [`PhysicalDeviceShaderImageFootprintFeaturesNV`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDeviceShaderImageFootprintFeaturesNV(PhysicalDeviceShaderImageFootprintFeaturesNV),
    #[cfg(feature = "VK_NV_dedicated_allocation_image_aliasing")]
    ///Contains a type [`PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV(PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV),
    #[cfg(feature = "VK_NV_shading_rate_image")]
    ///Contains a type [`PhysicalDeviceShadingRateImageFeaturesNV`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDeviceShadingRateImageFeaturesNV(PhysicalDeviceShadingRateImageFeaturesNV),
    #[cfg(feature = "VK_HUAWEI_invocation_mask")]
    ///Contains a type [`PhysicalDeviceInvocationMaskFeaturesHUAWEI`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDeviceInvocationMaskFeaturesHUAWEI(PhysicalDeviceInvocationMaskFeaturesHUAWEI),
    #[cfg(feature = "VK_NV_mesh_shader")]
    ///Contains a type [`PhysicalDeviceMeshShaderFeaturesNV`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDeviceMeshShaderFeaturesNV(PhysicalDeviceMeshShaderFeaturesNV),
    #[cfg(feature = "VK_KHR_acceleration_structure")]
    ///Contains a type [`PhysicalDeviceAccelerationStructureFeaturesKHR`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDeviceAccelerationStructureFeaturesKHR(PhysicalDeviceAccelerationStructureFeaturesKHR),
    #[cfg(feature = "VK_KHR_ray_tracing_pipeline")]
    ///Contains a type [`PhysicalDeviceRayTracingPipelineFeaturesKHR`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDeviceRayTracingPipelineFeaturesKHR(PhysicalDeviceRayTracingPipelineFeaturesKHR),
    #[cfg(feature = "VK_KHR_ray_query")]
    ///Contains a type [`PhysicalDeviceRayQueryFeaturesKHR`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDeviceRayQueryFeaturesKHR(PhysicalDeviceRayQueryFeaturesKHR),
    #[cfg(feature = "VK_EXT_fragment_density_map")]
    ///Contains a type [`PhysicalDeviceFragmentDensityMapFeaturesEXT`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDeviceFragmentDensityMapFeaturesEXT(PhysicalDeviceFragmentDensityMapFeaturesEXT),
    #[cfg(feature = "VK_EXT_fragment_density_map2")]
    ///Contains a type [`PhysicalDeviceFragmentDensityMap2FeaturesEXT`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDeviceFragmentDensityMap2FeaturesEXT(PhysicalDeviceFragmentDensityMap2FeaturesEXT),
    #[cfg(feature = "VULKAN_1_2")]
    ///Contains a type [`PhysicalDeviceScalarBlockLayoutFeatures`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDeviceScalarBlockLayoutFeatures(PhysicalDeviceScalarBlockLayoutFeatures),
    #[cfg(feature = "VULKAN_1_2")]
    ///Contains a type [`PhysicalDeviceUniformBufferStandardLayoutFeatures`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDeviceUniformBufferStandardLayoutFeatures(PhysicalDeviceUniformBufferStandardLayoutFeatures),
    #[cfg(feature = "VK_EXT_depth_clip_enable")]
    ///Contains a type [`PhysicalDeviceDepthClipEnableFeaturesEXT`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDeviceDepthClipEnableFeaturesEXT(PhysicalDeviceDepthClipEnableFeaturesEXT),
    #[cfg(feature = "VK_EXT_memory_priority")]
    ///Contains a type [`PhysicalDeviceMemoryPriorityFeaturesEXT`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDeviceMemoryPriorityFeaturesEXT(PhysicalDeviceMemoryPriorityFeaturesEXT),
    #[cfg(feature = "VK_EXT_pageable_device_local_memory")]
    ///Contains a type [`PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT(PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT),
    #[cfg(feature = "VULKAN_1_2")]
    ///Contains a type [`PhysicalDeviceBufferDeviceAddressFeatures`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDeviceBufferDeviceAddressFeatures(PhysicalDeviceBufferDeviceAddressFeatures),
    #[cfg(feature = "VK_EXT_buffer_device_address")]
    ///Contains a type [`PhysicalDeviceBufferDeviceAddressFeaturesEXT`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDeviceBufferDeviceAddressFeaturesEXT(PhysicalDeviceBufferDeviceAddressFeaturesEXT),
    #[cfg(feature = "VULKAN_1_2")]
    ///Contains a type [`PhysicalDeviceImagelessFramebufferFeatures`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDeviceImagelessFramebufferFeatures(PhysicalDeviceImagelessFramebufferFeatures),
    #[cfg(feature = "VULKAN_1_3")]
    ///Contains a type [`PhysicalDeviceTextureCompressionAstchdrFeatures`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDeviceTextureCompressionAstchdrFeatures(PhysicalDeviceTextureCompressionAstchdrFeatures),
    #[cfg(feature = "VK_NV_cooperative_matrix")]
    ///Contains a type [`PhysicalDeviceCooperativeMatrixFeaturesNV`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDeviceCooperativeMatrixFeaturesNV(PhysicalDeviceCooperativeMatrixFeaturesNV),
    #[cfg(feature = "VK_EXT_ycbcr_image_arrays")]
    ///Contains a type [`PhysicalDeviceYcbcrImageArraysFeaturesEXT`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDeviceYcbcrImageArraysFeaturesEXT(PhysicalDeviceYcbcrImageArraysFeaturesEXT),
    #[cfg(feature = "VK_KHR_performance_query")]
    ///Contains a type [`PhysicalDevicePerformanceQueryFeaturesKHR`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDevicePerformanceQueryFeaturesKHR(PhysicalDevicePerformanceQueryFeaturesKHR),
    #[cfg(feature = "VK_NV_coverage_reduction_mode")]
    ///Contains a type [`PhysicalDeviceCoverageReductionModeFeaturesNV`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDeviceCoverageReductionModeFeaturesNV(PhysicalDeviceCoverageReductionModeFeaturesNV),
    #[cfg(feature = "VK_INTEL_shader_integer_functions2")]
    ///Contains a type [`PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL(PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL),
    #[cfg(feature = "VK_KHR_shader_clock")]
    ///Contains a type [`PhysicalDeviceShaderClockFeaturesKHR`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDeviceShaderClockFeaturesKHR(PhysicalDeviceShaderClockFeaturesKHR),
    #[cfg(feature = "VK_EXT_index_type_uint8")]
    ///Contains a type [`PhysicalDeviceIndexTypeUint8FeaturesEXT`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDeviceIndexTypeUint8FeaturesEXT(PhysicalDeviceIndexTypeUint8FeaturesEXT),
    #[cfg(feature = "VK_NV_shader_sm_builtins")]
    ///Contains a type [`PhysicalDeviceShaderSmBuiltinsFeaturesNV`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDeviceShaderSmBuiltinsFeaturesNV(PhysicalDeviceShaderSmBuiltinsFeaturesNV),
    #[cfg(feature = "VK_EXT_fragment_shader_interlock")]
    ///Contains a type [`PhysicalDeviceFragmentShaderInterlockFeaturesEXT`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDeviceFragmentShaderInterlockFeaturesEXT(PhysicalDeviceFragmentShaderInterlockFeaturesEXT),
    #[cfg(feature = "VULKAN_1_2")]
    ///Contains a type [`PhysicalDeviceSeparateDepthStencilLayoutsFeatures`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDeviceSeparateDepthStencilLayoutsFeatures(PhysicalDeviceSeparateDepthStencilLayoutsFeatures),
    #[cfg(feature = "VK_EXT_primitive_topology_list_restart")]
    ///Contains a type [`PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT(PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT),
    #[cfg(feature = "VK_KHR_pipeline_executable_properties")]
    ///Contains a type [`PhysicalDevicePipelineExecutablePropertiesFeaturesKHR`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDevicePipelineExecutablePropertiesFeaturesKHR(PhysicalDevicePipelineExecutablePropertiesFeaturesKHR),
    #[cfg(feature = "VULKAN_1_3")]
    ///Contains a type [`PhysicalDeviceShaderDemoteToHelperInvocationFeatures`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDeviceShaderDemoteToHelperInvocationFeatures(PhysicalDeviceShaderDemoteToHelperInvocationFeatures),
    #[cfg(feature = "VK_EXT_texel_buffer_alignment")]
    ///Contains a type [`PhysicalDeviceTexelBufferAlignmentFeaturesEXT`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDeviceTexelBufferAlignmentFeaturesEXT(PhysicalDeviceTexelBufferAlignmentFeaturesEXT),
    #[cfg(feature = "VULKAN_1_3")]
    ///Contains a type [`PhysicalDeviceSubgroupSizeControlFeatures`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDeviceSubgroupSizeControlFeatures(PhysicalDeviceSubgroupSizeControlFeatures),
    #[cfg(feature = "VK_EXT_line_rasterization")]
    ///Contains a type [`PhysicalDeviceLineRasterizationFeaturesEXT`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDeviceLineRasterizationFeaturesEXT(PhysicalDeviceLineRasterizationFeaturesEXT),
    #[cfg(feature = "VULKAN_1_3")]
    ///Contains a type [`PhysicalDevicePipelineCreationCacheControlFeatures`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDevicePipelineCreationCacheControlFeatures(PhysicalDevicePipelineCreationCacheControlFeatures),
    #[cfg(feature = "VULKAN_1_2")]
    ///Contains a type [`PhysicalDeviceVulkan11Features`] for extending [`PhysicalDeviceFeatures2`]
    PhysicalDeviceVulkan11Features(PhysicalDeviceVulkan11Features),
    #[cfg(feature = "VULKAN_1_2")]
    ///Contains a type [`PhysicalDeviceVulkan12Features`] for extending [`PhysicalDeviceFeatures2`]
    PhysicalDeviceVulkan12Features(PhysicalDeviceVulkan12Features),
    #[cfg(feature = "VULKAN_1_3")]
    ///Contains a type [`PhysicalDeviceVulkan13Features`] for extending [`PhysicalDeviceFeatures2`]
    PhysicalDeviceVulkan13Features(PhysicalDeviceVulkan13Features),
    #[cfg(feature = "VK_AMD_device_coherent_memory")]
    ///Contains a type [`PhysicalDeviceCoherentMemoryFeaturesAMD`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDeviceCoherentMemoryFeaturesAMD(PhysicalDeviceCoherentMemoryFeaturesAMD),
    #[cfg(feature = "VK_EXT_custom_border_color")]
    ///Contains a type [`PhysicalDeviceCustomBorderColorFeaturesEXT`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDeviceCustomBorderColorFeaturesEXT(PhysicalDeviceCustomBorderColorFeaturesEXT),
    #[cfg(feature = "VK_EXT_border_color_swizzle")]
    ///Contains a type [`PhysicalDeviceBorderColorSwizzleFeaturesEXT`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDeviceBorderColorSwizzleFeaturesEXT(PhysicalDeviceBorderColorSwizzleFeaturesEXT),
    #[cfg(feature = "VK_EXT_extended_dynamic_state")]
    ///Contains a type [`PhysicalDeviceExtendedDynamicStateFeaturesEXT`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDeviceExtendedDynamicStateFeaturesEXT(PhysicalDeviceExtendedDynamicStateFeaturesEXT),
    #[cfg(feature = "VK_EXT_extended_dynamic_state2")]
    ///Contains a type [`PhysicalDeviceExtendedDynamicState2FeaturesEXT`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDeviceExtendedDynamicState2FeaturesEXT(PhysicalDeviceExtendedDynamicState2FeaturesEXT),
    #[cfg(feature = "VK_NV_device_diagnostics_config")]
    ///Contains a type [`PhysicalDeviceDiagnosticsConfigFeaturesNV`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDeviceDiagnosticsConfigFeaturesNV(PhysicalDeviceDiagnosticsConfigFeaturesNV),
    #[cfg(feature = "VULKAN_1_3")]
    ///Contains a type [`PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures(PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures),
    #[cfg(feature = "VK_KHR_shader_subgroup_uniform_control_flow")]
    ///Contains a type [`PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR(
        PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR,
    ),
    #[cfg(feature = "VK_EXT_robustness2")]
    ///Contains a type [`PhysicalDeviceRobustness2FeaturesEXT`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDeviceRobustness2FeaturesEXT(PhysicalDeviceRobustness2FeaturesEXT),
    #[cfg(feature = "VULKAN_1_3")]
    ///Contains a type [`PhysicalDeviceImageRobustnessFeatures`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDeviceImageRobustnessFeatures(PhysicalDeviceImageRobustnessFeatures),
    #[cfg(feature = "VK_KHR_workgroup_memory_explicit_layout")]
    ///Contains a type [`PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR(PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR),
    #[cfg(feature = "VK_KHR_portability_subset")]
    ///Contains a type [`PhysicalDevicePortabilitySubsetFeaturesKHR`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDevicePortabilitySubsetFeaturesKHR(PhysicalDevicePortabilitySubsetFeaturesKHR),
    #[cfg(feature = "VK_EXT_4444_formats")]
    ///Contains a type [`PhysicalDevice4444FormatsFeaturesEXT`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDevice4444FormatsFeaturesEXT(PhysicalDevice4444FormatsFeaturesEXT),
    #[cfg(feature = "VK_HUAWEI_subpass_shading")]
    ///Contains a type [`PhysicalDeviceSubpassShadingFeaturesHUAWEI`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDeviceSubpassShadingFeaturesHUAWEI(PhysicalDeviceSubpassShadingFeaturesHUAWEI),
    #[cfg(feature = "VK_EXT_shader_image_atomic_int64")]
    ///Contains a type [`PhysicalDeviceShaderImageAtomicInt64FeaturesEXT`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDeviceShaderImageAtomicInt64FeaturesEXT(PhysicalDeviceShaderImageAtomicInt64FeaturesEXT),
    #[cfg(feature = "VK_KHR_fragment_shading_rate")]
    ///Contains a type [`PhysicalDeviceFragmentShadingRateFeaturesKHR`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDeviceFragmentShadingRateFeaturesKHR(PhysicalDeviceFragmentShadingRateFeaturesKHR),
    #[cfg(feature = "VULKAN_1_3")]
    ///Contains a type [`PhysicalDeviceShaderTerminateInvocationFeatures`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDeviceShaderTerminateInvocationFeatures(PhysicalDeviceShaderTerminateInvocationFeatures),
    #[cfg(feature = "VK_NV_fragment_shading_rate_enums")]
    ///Contains a type [`PhysicalDeviceFragmentShadingRateEnumsFeaturesNV`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDeviceFragmentShadingRateEnumsFeaturesNV(PhysicalDeviceFragmentShadingRateEnumsFeaturesNV),
    #[cfg(feature = "VK_VALVE_mutable_descriptor_type")]
    ///Contains a type [`PhysicalDeviceMutableDescriptorTypeFeaturesVALVE`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDeviceMutableDescriptorTypeFeaturesVALVE(PhysicalDeviceMutableDescriptorTypeFeaturesVALVE),
    #[cfg(feature = "VK_EXT_depth_clip_control")]
    ///Contains a type [`PhysicalDeviceDepthClipControlFeaturesEXT`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDeviceDepthClipControlFeaturesEXT(PhysicalDeviceDepthClipControlFeaturesEXT),
    #[cfg(feature = "VK_EXT_vertex_input_dynamic_state")]
    ///Contains a type [`PhysicalDeviceVertexInputDynamicStateFeaturesEXT`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDeviceVertexInputDynamicStateFeaturesEXT(PhysicalDeviceVertexInputDynamicStateFeaturesEXT),
    #[cfg(feature = "VK_NV_external_memory_rdma")]
    ///Contains a type [`PhysicalDeviceExternalMemoryRdmaFeaturesNV`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDeviceExternalMemoryRdmaFeaturesNV(PhysicalDeviceExternalMemoryRdmaFeaturesNV),
    #[cfg(feature = "VK_EXT_color_write_enable")]
    ///Contains a type [`PhysicalDeviceColorWriteEnableFeaturesEXT`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDeviceColorWriteEnableFeaturesEXT(PhysicalDeviceColorWriteEnableFeaturesEXT),
    #[cfg(feature = "VULKAN_1_3")]
    ///Contains a type [`PhysicalDeviceSynchronization2Features`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDeviceSynchronization2Features(PhysicalDeviceSynchronization2Features),
    #[cfg(feature = "VK_NV_inherited_viewport_scissor")]
    ///Contains a type [`PhysicalDeviceInheritedViewportScissorFeaturesNV`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDeviceInheritedViewportScissorFeaturesNV(PhysicalDeviceInheritedViewportScissorFeaturesNV),
    #[cfg(feature = "VK_EXT_ycbcr_2plane_444_formats")]
    ///Contains a type [`PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT(PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT),
    #[cfg(feature = "VK_EXT_provoking_vertex")]
    ///Contains a type [`PhysicalDeviceProvokingVertexFeaturesEXT`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDeviceProvokingVertexFeaturesEXT(PhysicalDeviceProvokingVertexFeaturesEXT),
    #[cfg(feature = "VULKAN_1_3")]
    ///Contains a type [`PhysicalDeviceShaderIntegerDotProductFeatures`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDeviceShaderIntegerDotProductFeatures(PhysicalDeviceShaderIntegerDotProductFeatures),
    #[cfg(feature = "VK_NV_ray_tracing_motion_blur")]
    ///Contains a type [`PhysicalDeviceRayTracingMotionBlurFeaturesNV`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDeviceRayTracingMotionBlurFeaturesNV(PhysicalDeviceRayTracingMotionBlurFeaturesNV),
    #[cfg(feature = "VK_EXT_rgba10x6_formats")]
    ///Contains a type [`PhysicalDeviceRgba10x6FormatsFeaturesEXT`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDeviceRgba10x6FormatsFeaturesEXT(PhysicalDeviceRgba10x6FormatsFeaturesEXT),
    #[cfg(feature = "VULKAN_1_3")]
    ///Contains a type [`PhysicalDeviceDynamicRenderingFeatures`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDeviceDynamicRenderingFeatures(PhysicalDeviceDynamicRenderingFeatures),
    #[cfg(feature = "VK_EXT_image_view_min_lod")]
    ///Contains a type [`PhysicalDeviceImageViewMinLodFeaturesEXT`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDeviceImageViewMinLodFeaturesEXT(PhysicalDeviceImageViewMinLodFeaturesEXT),
    #[cfg(feature = "VK_ARM_rasterization_order_attachment_access")]
    ///Contains a type [`PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM`] for
    /// extending [`PhysicalDeviceFeatures2`]
    PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM(
        PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM,
    ),
    #[cfg(feature = "VK_NV_linear_color_attachment")]
    ///Contains a type [`PhysicalDeviceLinearColorAttachmentFeaturesNV`] for extending
    /// [`PhysicalDeviceFeatures2`]
    PhysicalDeviceLinearColorAttachmentFeaturesNV(PhysicalDeviceLinearColorAttachmentFeaturesNV),
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceFeatures2Extension {
    type LowLevel = *mut crate::native::vulkan1_0::BaseOutStructure;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        match self { # [cfg (feature = "VK_NV_device_generated_commands")] Self :: PhysicalDeviceDeviceGeneratedCommandsFeaturesNV (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: nv_device_generated_commands :: PhysicalDeviceDeviceGeneratedCommandsFeaturesNV) . cast () , # [cfg (feature = "VULKAN_1_3")] Self :: PhysicalDevicePrivateDataFeatures (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: vulkan1_3 :: PhysicalDevicePrivateDataFeatures) . cast () , Self :: PhysicalDeviceVariablePointersFeatures (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: vulkan1_1 :: PhysicalDeviceVariablePointersFeatures) . cast () , Self :: PhysicalDeviceMultiviewFeatures (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: vulkan1_1 :: PhysicalDeviceMultiviewFeatures) . cast () , # [cfg (feature = "VK_KHR_present_id")] Self :: PhysicalDevicePresentIdFeaturesKHR (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: khr_present_id :: PhysicalDevicePresentIdFeaturesKHR) . cast () , # [cfg (feature = "VK_KHR_present_wait")] Self :: PhysicalDevicePresentWaitFeaturesKHR (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: khr_present_wait :: PhysicalDevicePresentWaitFeaturesKHR) . cast () , Self :: PhysicalDevice16BitStorageFeatures (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: vulkan1_1 :: PhysicalDevice16BitStorageFeatures) . cast () , # [cfg (feature = "VULKAN_1_2")] Self :: PhysicalDeviceShaderSubgroupExtendedTypesFeatures (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: vulkan1_2 :: PhysicalDeviceShaderSubgroupExtendedTypesFeatures) . cast () , Self :: PhysicalDeviceSamplerYcbcrConversionFeatures (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: vulkan1_1 :: PhysicalDeviceSamplerYcbcrConversionFeatures) . cast () , Self :: PhysicalDeviceProtectedMemoryFeatures (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: vulkan1_1 :: PhysicalDeviceProtectedMemoryFeatures) . cast () , # [cfg (feature = "VK_EXT_blend_operation_advanced")] Self :: PhysicalDeviceBlendOperationAdvancedFeaturesEXT (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: ext_blend_operation_advanced :: PhysicalDeviceBlendOperationAdvancedFeaturesEXT) . cast () , # [cfg (feature = "VK_EXT_multi_draw")] Self :: PhysicalDeviceMultiDrawFeaturesEXT (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: ext_multi_draw :: PhysicalDeviceMultiDrawFeaturesEXT) . cast () , # [cfg (feature = "VULKAN_1_3")] Self :: PhysicalDeviceInlineUniformBlockFeatures (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: vulkan1_3 :: PhysicalDeviceInlineUniformBlockFeatures) . cast () , # [cfg (feature = "VULKAN_1_3")] Self :: PhysicalDeviceMaintenance4Features (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: vulkan1_3 :: PhysicalDeviceMaintenance4Features) . cast () , Self :: PhysicalDeviceShaderDrawParametersFeatures (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: vulkan1_1 :: PhysicalDeviceShaderDrawParametersFeatures) . cast () , # [cfg (feature = "VULKAN_1_2")] Self :: PhysicalDeviceShaderFloat16Int8Features (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: vulkan1_2 :: PhysicalDeviceShaderFloat16Int8Features) . cast () , # [cfg (feature = "VULKAN_1_2")] Self :: PhysicalDeviceHostQueryResetFeatures (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: vulkan1_2 :: PhysicalDeviceHostQueryResetFeatures) . cast () , # [cfg (feature = "VK_KHR_global_priority")] Self :: PhysicalDeviceGlobalPriorityQueryFeaturesKHR (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: khr_global_priority :: PhysicalDeviceGlobalPriorityQueryFeaturesKHR) . cast () , # [cfg (feature = "VK_EXT_device_memory_report")] Self :: PhysicalDeviceDeviceMemoryReportFeaturesEXT (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: ext_device_memory_report :: PhysicalDeviceDeviceMemoryReportFeaturesEXT) . cast () , # [cfg (feature = "VULKAN_1_2")] Self :: PhysicalDeviceDescriptorIndexingFeatures (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: vulkan1_2 :: PhysicalDeviceDescriptorIndexingFeatures) . cast () , # [cfg (feature = "VULKAN_1_2")] Self :: PhysicalDeviceTimelineSemaphoreFeatures (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: vulkan1_2 :: PhysicalDeviceTimelineSemaphoreFeatures) . cast () , # [cfg (feature = "VULKAN_1_2")] Self :: PhysicalDevice8BitStorageFeatures (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: vulkan1_2 :: PhysicalDevice8BitStorageFeatures) . cast () , # [cfg (feature = "VK_EXT_conditional_rendering")] Self :: PhysicalDeviceConditionalRenderingFeaturesEXT (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: ext_conditional_rendering :: PhysicalDeviceConditionalRenderingFeaturesEXT) . cast () , # [cfg (feature = "VULKAN_1_2")] Self :: PhysicalDeviceVulkanMemoryModelFeatures (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: vulkan1_2 :: PhysicalDeviceVulkanMemoryModelFeatures) . cast () , # [cfg (feature = "VULKAN_1_2")] Self :: PhysicalDeviceShaderAtomicInt64Features (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: vulkan1_2 :: PhysicalDeviceShaderAtomicInt64Features) . cast () , # [cfg (feature = "VK_EXT_shader_atomic_float")] Self :: PhysicalDeviceShaderAtomicFloatFeaturesEXT (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: ext_shader_atomic_float :: PhysicalDeviceShaderAtomicFloatFeaturesEXT) . cast () , # [cfg (feature = "VK_EXT_shader_atomic_float2")] Self :: PhysicalDeviceShaderAtomicFloat2FeaturesEXT (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: ext_shader_atomic_float2 :: PhysicalDeviceShaderAtomicFloat2FeaturesEXT) . cast () , # [cfg (feature = "VK_EXT_vertex_attribute_divisor")] Self :: PhysicalDeviceVertexAttributeDivisorFeaturesEXT (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: ext_vertex_attribute_divisor :: PhysicalDeviceVertexAttributeDivisorFeaturesEXT) . cast () , # [cfg (feature = "VK_EXT_astc_decode_mode")] Self :: PhysicalDeviceAstcDecodeFeaturesEXT (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: ext_astc_decode_mode :: PhysicalDeviceAstcDecodeFeaturesEXT) . cast () , # [cfg (feature = "VK_EXT_transform_feedback")] Self :: PhysicalDeviceTransformFeedbackFeaturesEXT (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: ext_transform_feedback :: PhysicalDeviceTransformFeedbackFeaturesEXT) . cast () , # [cfg (feature = "VK_NV_representative_fragment_test")] Self :: PhysicalDeviceRepresentativeFragmentTestFeaturesNV (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: nv_representative_fragment_test :: PhysicalDeviceRepresentativeFragmentTestFeaturesNV) . cast () , # [cfg (feature = "VK_NV_scissor_exclusive")] Self :: PhysicalDeviceExclusiveScissorFeaturesNV (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: nv_scissor_exclusive :: PhysicalDeviceExclusiveScissorFeaturesNV) . cast () , # [cfg (feature = "VK_NV_corner_sampled_image")] Self :: PhysicalDeviceCornerSampledImageFeaturesNV (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: nv_corner_sampled_image :: PhysicalDeviceCornerSampledImageFeaturesNV) . cast () , # [cfg (feature = "VK_NV_compute_shader_derivatives")] Self :: PhysicalDeviceComputeShaderDerivativesFeaturesNV (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: nv_compute_shader_derivatives :: PhysicalDeviceComputeShaderDerivativesFeaturesNV) . cast () , # [cfg (feature = "VK_NV_fragment_shader_barycentric")] Self :: PhysicalDeviceFragmentShaderBarycentricFeaturesNV (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: nv_fragment_shader_barycentric :: PhysicalDeviceFragmentShaderBarycentricFeaturesNV) . cast () , # [cfg (feature = "VK_NV_shader_image_footprint")] Self :: PhysicalDeviceShaderImageFootprintFeaturesNV (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: nv_shader_image_footprint :: PhysicalDeviceShaderImageFootprintFeaturesNV) . cast () , # [cfg (feature = "VK_NV_dedicated_allocation_image_aliasing")] Self :: PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: nv_dedicated_allocation_image_aliasing :: PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV) . cast () , # [cfg (feature = "VK_NV_shading_rate_image")] Self :: PhysicalDeviceShadingRateImageFeaturesNV (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: nv_shading_rate_image :: PhysicalDeviceShadingRateImageFeaturesNV) . cast () , # [cfg (feature = "VK_HUAWEI_invocation_mask")] Self :: PhysicalDeviceInvocationMaskFeaturesHUAWEI (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: huawei_invocation_mask :: PhysicalDeviceInvocationMaskFeaturesHUAWEI) . cast () , # [cfg (feature = "VK_NV_mesh_shader")] Self :: PhysicalDeviceMeshShaderFeaturesNV (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: nv_mesh_shader :: PhysicalDeviceMeshShaderFeaturesNV) . cast () , # [cfg (feature = "VK_KHR_acceleration_structure")] Self :: PhysicalDeviceAccelerationStructureFeaturesKHR (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: khr_acceleration_structure :: PhysicalDeviceAccelerationStructureFeaturesKHR) . cast () , # [cfg (feature = "VK_KHR_ray_tracing_pipeline")] Self :: PhysicalDeviceRayTracingPipelineFeaturesKHR (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: khr_ray_tracing_pipeline :: PhysicalDeviceRayTracingPipelineFeaturesKHR) . cast () , # [cfg (feature = "VK_KHR_ray_query")] Self :: PhysicalDeviceRayQueryFeaturesKHR (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: khr_ray_query :: PhysicalDeviceRayQueryFeaturesKHR) . cast () , # [cfg (feature = "VK_EXT_fragment_density_map")] Self :: PhysicalDeviceFragmentDensityMapFeaturesEXT (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: ext_fragment_density_map :: PhysicalDeviceFragmentDensityMapFeaturesEXT) . cast () , # [cfg (feature = "VK_EXT_fragment_density_map2")] Self :: PhysicalDeviceFragmentDensityMap2FeaturesEXT (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: ext_fragment_density_map2 :: PhysicalDeviceFragmentDensityMap2FeaturesEXT) . cast () , # [cfg (feature = "VULKAN_1_2")] Self :: PhysicalDeviceScalarBlockLayoutFeatures (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: vulkan1_2 :: PhysicalDeviceScalarBlockLayoutFeatures) . cast () , # [cfg (feature = "VULKAN_1_2")] Self :: PhysicalDeviceUniformBufferStandardLayoutFeatures (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: vulkan1_2 :: PhysicalDeviceUniformBufferStandardLayoutFeatures) . cast () , # [cfg (feature = "VK_EXT_depth_clip_enable")] Self :: PhysicalDeviceDepthClipEnableFeaturesEXT (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: ext_depth_clip_enable :: PhysicalDeviceDepthClipEnableFeaturesEXT) . cast () , # [cfg (feature = "VK_EXT_memory_priority")] Self :: PhysicalDeviceMemoryPriorityFeaturesEXT (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: ext_memory_priority :: PhysicalDeviceMemoryPriorityFeaturesEXT) . cast () , # [cfg (feature = "VK_EXT_pageable_device_local_memory")] Self :: PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: ext_pageable_device_local_memory :: PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT) . cast () , # [cfg (feature = "VULKAN_1_2")] Self :: PhysicalDeviceBufferDeviceAddressFeatures (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: vulkan1_2 :: PhysicalDeviceBufferDeviceAddressFeatures) . cast () , # [cfg (feature = "VK_EXT_buffer_device_address")] Self :: PhysicalDeviceBufferDeviceAddressFeaturesEXT (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: ext_buffer_device_address :: PhysicalDeviceBufferDeviceAddressFeaturesEXT) . cast () , # [cfg (feature = "VULKAN_1_2")] Self :: PhysicalDeviceImagelessFramebufferFeatures (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: vulkan1_2 :: PhysicalDeviceImagelessFramebufferFeatures) . cast () , # [cfg (feature = "VULKAN_1_3")] Self :: PhysicalDeviceTextureCompressionAstchdrFeatures (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: vulkan1_3 :: PhysicalDeviceTextureCompressionAstchdrFeatures) . cast () , # [cfg (feature = "VK_NV_cooperative_matrix")] Self :: PhysicalDeviceCooperativeMatrixFeaturesNV (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: nv_cooperative_matrix :: PhysicalDeviceCooperativeMatrixFeaturesNV) . cast () , # [cfg (feature = "VK_EXT_ycbcr_image_arrays")] Self :: PhysicalDeviceYcbcrImageArraysFeaturesEXT (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: ext_ycbcr_image_arrays :: PhysicalDeviceYcbcrImageArraysFeaturesEXT) . cast () , # [cfg (feature = "VK_KHR_performance_query")] Self :: PhysicalDevicePerformanceQueryFeaturesKHR (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: khr_performance_query :: PhysicalDevicePerformanceQueryFeaturesKHR) . cast () , # [cfg (feature = "VK_NV_coverage_reduction_mode")] Self :: PhysicalDeviceCoverageReductionModeFeaturesNV (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: nv_coverage_reduction_mode :: PhysicalDeviceCoverageReductionModeFeaturesNV) . cast () , # [cfg (feature = "VK_INTEL_shader_integer_functions2")] Self :: PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: intel_shader_integer_functions2 :: PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL) . cast () , # [cfg (feature = "VK_KHR_shader_clock")] Self :: PhysicalDeviceShaderClockFeaturesKHR (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: khr_shader_clock :: PhysicalDeviceShaderClockFeaturesKHR) . cast () , # [cfg (feature = "VK_EXT_index_type_uint8")] Self :: PhysicalDeviceIndexTypeUint8FeaturesEXT (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: ext_index_type_uint8 :: PhysicalDeviceIndexTypeUint8FeaturesEXT) . cast () , # [cfg (feature = "VK_NV_shader_sm_builtins")] Self :: PhysicalDeviceShaderSmBuiltinsFeaturesNV (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: nv_shader_sm_builtins :: PhysicalDeviceShaderSmBuiltinsFeaturesNV) . cast () , # [cfg (feature = "VK_EXT_fragment_shader_interlock")] Self :: PhysicalDeviceFragmentShaderInterlockFeaturesEXT (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: ext_fragment_shader_interlock :: PhysicalDeviceFragmentShaderInterlockFeaturesEXT) . cast () , # [cfg (feature = "VULKAN_1_2")] Self :: PhysicalDeviceSeparateDepthStencilLayoutsFeatures (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: vulkan1_2 :: PhysicalDeviceSeparateDepthStencilLayoutsFeatures) . cast () , # [cfg (feature = "VK_EXT_primitive_topology_list_restart")] Self :: PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: ext_primitive_topology_list_restart :: PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT) . cast () , # [cfg (feature = "VK_KHR_pipeline_executable_properties")] Self :: PhysicalDevicePipelineExecutablePropertiesFeaturesKHR (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: khr_pipeline_executable_properties :: PhysicalDevicePipelineExecutablePropertiesFeaturesKHR) . cast () , # [cfg (feature = "VULKAN_1_3")] Self :: PhysicalDeviceShaderDemoteToHelperInvocationFeatures (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: vulkan1_3 :: PhysicalDeviceShaderDemoteToHelperInvocationFeatures) . cast () , # [cfg (feature = "VK_EXT_texel_buffer_alignment")] Self :: PhysicalDeviceTexelBufferAlignmentFeaturesEXT (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: ext_texel_buffer_alignment :: PhysicalDeviceTexelBufferAlignmentFeaturesEXT) . cast () , # [cfg (feature = "VULKAN_1_3")] Self :: PhysicalDeviceSubgroupSizeControlFeatures (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: vulkan1_3 :: PhysicalDeviceSubgroupSizeControlFeatures) . cast () , # [cfg (feature = "VK_EXT_line_rasterization")] Self :: PhysicalDeviceLineRasterizationFeaturesEXT (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: ext_line_rasterization :: PhysicalDeviceLineRasterizationFeaturesEXT) . cast () , # [cfg (feature = "VULKAN_1_3")] Self :: PhysicalDevicePipelineCreationCacheControlFeatures (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: vulkan1_3 :: PhysicalDevicePipelineCreationCacheControlFeatures) . cast () , # [cfg (feature = "VULKAN_1_2")] Self :: PhysicalDeviceVulkan11Features (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: vulkan1_2 :: PhysicalDeviceVulkan11Features) . cast () , # [cfg (feature = "VULKAN_1_2")] Self :: PhysicalDeviceVulkan12Features (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: vulkan1_2 :: PhysicalDeviceVulkan12Features) . cast () , # [cfg (feature = "VULKAN_1_3")] Self :: PhysicalDeviceVulkan13Features (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: vulkan1_3 :: PhysicalDeviceVulkan13Features) . cast () , # [cfg (feature = "VK_AMD_device_coherent_memory")] Self :: PhysicalDeviceCoherentMemoryFeaturesAMD (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: amd_device_coherent_memory :: PhysicalDeviceCoherentMemoryFeaturesAMD) . cast () , # [cfg (feature = "VK_EXT_custom_border_color")] Self :: PhysicalDeviceCustomBorderColorFeaturesEXT (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: ext_custom_border_color :: PhysicalDeviceCustomBorderColorFeaturesEXT) . cast () , # [cfg (feature = "VK_EXT_border_color_swizzle")] Self :: PhysicalDeviceBorderColorSwizzleFeaturesEXT (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: ext_border_color_swizzle :: PhysicalDeviceBorderColorSwizzleFeaturesEXT) . cast () , # [cfg (feature = "VK_EXT_extended_dynamic_state")] Self :: PhysicalDeviceExtendedDynamicStateFeaturesEXT (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: ext_extended_dynamic_state :: PhysicalDeviceExtendedDynamicStateFeaturesEXT) . cast () , # [cfg (feature = "VK_EXT_extended_dynamic_state2")] Self :: PhysicalDeviceExtendedDynamicState2FeaturesEXT (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: ext_extended_dynamic_state2 :: PhysicalDeviceExtendedDynamicState2FeaturesEXT) . cast () , # [cfg (feature = "VK_NV_device_diagnostics_config")] Self :: PhysicalDeviceDiagnosticsConfigFeaturesNV (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: nv_device_diagnostics_config :: PhysicalDeviceDiagnosticsConfigFeaturesNV) . cast () , # [cfg (feature = "VULKAN_1_3")] Self :: PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: vulkan1_3 :: PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures) . cast () , # [cfg (feature = "VK_KHR_shader_subgroup_uniform_control_flow")] Self :: PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: khr_shader_subgroup_uniform_control_flow :: PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR) . cast () , # [cfg (feature = "VK_EXT_robustness2")] Self :: PhysicalDeviceRobustness2FeaturesEXT (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: ext_robustness2 :: PhysicalDeviceRobustness2FeaturesEXT) . cast () , # [cfg (feature = "VULKAN_1_3")] Self :: PhysicalDeviceImageRobustnessFeatures (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: vulkan1_3 :: PhysicalDeviceImageRobustnessFeatures) . cast () , # [cfg (feature = "VK_KHR_workgroup_memory_explicit_layout")] Self :: PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: khr_workgroup_memory_explicit_layout :: PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR) . cast () , # [cfg (feature = "VK_KHR_portability_subset")] Self :: PhysicalDevicePortabilitySubsetFeaturesKHR (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: khr_portability_subset :: PhysicalDevicePortabilitySubsetFeaturesKHR) . cast () , # [cfg (feature = "VK_EXT_4444_formats")] Self :: PhysicalDevice4444FormatsFeaturesEXT (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: ext_4444_formats :: PhysicalDevice4444FormatsFeaturesEXT) . cast () , # [cfg (feature = "VK_HUAWEI_subpass_shading")] Self :: PhysicalDeviceSubpassShadingFeaturesHUAWEI (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: huawei_subpass_shading :: PhysicalDeviceSubpassShadingFeaturesHUAWEI) . cast () , # [cfg (feature = "VK_EXT_shader_image_atomic_int64")] Self :: PhysicalDeviceShaderImageAtomicInt64FeaturesEXT (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: ext_shader_image_atomic_int64 :: PhysicalDeviceShaderImageAtomicInt64FeaturesEXT) . cast () , # [cfg (feature = "VK_KHR_fragment_shading_rate")] Self :: PhysicalDeviceFragmentShadingRateFeaturesKHR (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: khr_fragment_shading_rate :: PhysicalDeviceFragmentShadingRateFeaturesKHR) . cast () , # [cfg (feature = "VULKAN_1_3")] Self :: PhysicalDeviceShaderTerminateInvocationFeatures (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: vulkan1_3 :: PhysicalDeviceShaderTerminateInvocationFeatures) . cast () , # [cfg (feature = "VK_NV_fragment_shading_rate_enums")] Self :: PhysicalDeviceFragmentShadingRateEnumsFeaturesNV (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: nv_fragment_shading_rate_enums :: PhysicalDeviceFragmentShadingRateEnumsFeaturesNV) . cast () , # [cfg (feature = "VK_VALVE_mutable_descriptor_type")] Self :: PhysicalDeviceMutableDescriptorTypeFeaturesVALVE (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: valve_mutable_descriptor_type :: PhysicalDeviceMutableDescriptorTypeFeaturesVALVE) . cast () , # [cfg (feature = "VK_EXT_depth_clip_control")] Self :: PhysicalDeviceDepthClipControlFeaturesEXT (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: ext_depth_clip_control :: PhysicalDeviceDepthClipControlFeaturesEXT) . cast () , # [cfg (feature = "VK_EXT_vertex_input_dynamic_state")] Self :: PhysicalDeviceVertexInputDynamicStateFeaturesEXT (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: ext_vertex_input_dynamic_state :: PhysicalDeviceVertexInputDynamicStateFeaturesEXT) . cast () , # [cfg (feature = "VK_NV_external_memory_rdma")] Self :: PhysicalDeviceExternalMemoryRdmaFeaturesNV (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: nv_external_memory_rdma :: PhysicalDeviceExternalMemoryRdmaFeaturesNV) . cast () , # [cfg (feature = "VK_EXT_color_write_enable")] Self :: PhysicalDeviceColorWriteEnableFeaturesEXT (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: ext_color_write_enable :: PhysicalDeviceColorWriteEnableFeaturesEXT) . cast () , # [cfg (feature = "VULKAN_1_3")] Self :: PhysicalDeviceSynchronization2Features (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: vulkan1_3 :: PhysicalDeviceSynchronization2Features) . cast () , # [cfg (feature = "VK_NV_inherited_viewport_scissor")] Self :: PhysicalDeviceInheritedViewportScissorFeaturesNV (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: nv_inherited_viewport_scissor :: PhysicalDeviceInheritedViewportScissorFeaturesNV) . cast () , # [cfg (feature = "VK_EXT_ycbcr_2plane_444_formats")] Self :: PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: ext_ycbcr_2plane_444_formats :: PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT) . cast () , # [cfg (feature = "VK_EXT_provoking_vertex")] Self :: PhysicalDeviceProvokingVertexFeaturesEXT (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: ext_provoking_vertex :: PhysicalDeviceProvokingVertexFeaturesEXT) . cast () , # [cfg (feature = "VULKAN_1_3")] Self :: PhysicalDeviceShaderIntegerDotProductFeatures (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: vulkan1_3 :: PhysicalDeviceShaderIntegerDotProductFeatures) . cast () , # [cfg (feature = "VK_NV_ray_tracing_motion_blur")] Self :: PhysicalDeviceRayTracingMotionBlurFeaturesNV (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: nv_ray_tracing_motion_blur :: PhysicalDeviceRayTracingMotionBlurFeaturesNV) . cast () , # [cfg (feature = "VK_EXT_rgba10x6_formats")] Self :: PhysicalDeviceRgba10x6FormatsFeaturesEXT (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: ext_rgba10x6_formats :: PhysicalDeviceRgba10x6FormatsFeaturesEXT) . cast () , # [cfg (feature = "VULKAN_1_3")] Self :: PhysicalDeviceDynamicRenderingFeatures (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: vulkan1_3 :: PhysicalDeviceDynamicRenderingFeatures) . cast () , # [cfg (feature = "VK_EXT_image_view_min_lod")] Self :: PhysicalDeviceImageViewMinLodFeaturesEXT (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: ext_image_view_min_lod :: PhysicalDeviceImageViewMinLodFeaturesEXT) . cast () , # [cfg (feature = "VK_ARM_rasterization_order_attachment_access")] Self :: PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: arm_rasterization_order_attachment_access :: PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM) . cast () , # [cfg (feature = "VK_NV_linear_color_attachment")] Self :: PhysicalDeviceLinearColorAttachmentFeaturesNV (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: nv_linear_color_attachment :: PhysicalDeviceLinearColorAttachmentFeaturesNV) . cast () , other => unreachable ! ("unexpected variant {:?}" , other) }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceFeatures2Extension {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        assert!(!value.is_null());
        match (* value) . s_type { # [cfg (feature = "VK_NV_device_generated_commands")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceDeviceGeneratedCommandsFeaturesNv => Self :: PhysicalDeviceDeviceGeneratedCommandsFeaturesNV (PhysicalDeviceDeviceGeneratedCommandsFeaturesNV :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: nv_device_generated_commands :: PhysicalDeviceDeviceGeneratedCommandsFeaturesNV > ()))) , # [cfg (feature = "VULKAN_1_3")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDevicePrivateDataFeatures => Self :: PhysicalDevicePrivateDataFeatures (PhysicalDevicePrivateDataFeatures :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: vulkan1_3 :: PhysicalDevicePrivateDataFeatures > ()))) , crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceVariablePointersFeatures => Self :: PhysicalDeviceVariablePointersFeatures (PhysicalDeviceVariablePointersFeatures :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: vulkan1_1 :: PhysicalDeviceVariablePointersFeatures > ()))) , crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceMultiviewFeatures => Self :: PhysicalDeviceMultiviewFeatures (PhysicalDeviceMultiviewFeatures :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: vulkan1_1 :: PhysicalDeviceMultiviewFeatures > ()))) , # [cfg (feature = "VK_KHR_present_id")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDevicePresentIdFeaturesKhr => Self :: PhysicalDevicePresentIdFeaturesKHR (PhysicalDevicePresentIdFeaturesKHR :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: khr_present_id :: PhysicalDevicePresentIdFeaturesKHR > ()))) , # [cfg (feature = "VK_KHR_present_wait")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDevicePresentWaitFeaturesKhr => Self :: PhysicalDevicePresentWaitFeaturesKHR (PhysicalDevicePresentWaitFeaturesKHR :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: khr_present_wait :: PhysicalDevicePresentWaitFeaturesKHR > ()))) , crate :: native :: vulkan1_0 :: StructureType :: PhysicalDevice16bitStorageFeatures => Self :: PhysicalDevice16BitStorageFeatures (PhysicalDevice16BitStorageFeatures :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: vulkan1_1 :: PhysicalDevice16BitStorageFeatures > ()))) , # [cfg (feature = "VULKAN_1_2")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceShaderSubgroupExtendedTypesFeatures => Self :: PhysicalDeviceShaderSubgroupExtendedTypesFeatures (PhysicalDeviceShaderSubgroupExtendedTypesFeatures :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: vulkan1_2 :: PhysicalDeviceShaderSubgroupExtendedTypesFeatures > ()))) , crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceSamplerYcbcrConversionFeatures => Self :: PhysicalDeviceSamplerYcbcrConversionFeatures (PhysicalDeviceSamplerYcbcrConversionFeatures :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: vulkan1_1 :: PhysicalDeviceSamplerYcbcrConversionFeatures > ()))) , crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceProtectedMemoryFeatures => Self :: PhysicalDeviceProtectedMemoryFeatures (PhysicalDeviceProtectedMemoryFeatures :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: vulkan1_1 :: PhysicalDeviceProtectedMemoryFeatures > ()))) , # [cfg (feature = "VK_EXT_blend_operation_advanced")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceBlendOperationAdvancedFeaturesExt => Self :: PhysicalDeviceBlendOperationAdvancedFeaturesEXT (PhysicalDeviceBlendOperationAdvancedFeaturesEXT :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: ext_blend_operation_advanced :: PhysicalDeviceBlendOperationAdvancedFeaturesEXT > ()))) , # [cfg (feature = "VK_EXT_multi_draw")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceMultiDrawFeaturesExt => Self :: PhysicalDeviceMultiDrawFeaturesEXT (PhysicalDeviceMultiDrawFeaturesEXT :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: ext_multi_draw :: PhysicalDeviceMultiDrawFeaturesEXT > ()))) , # [cfg (feature = "VULKAN_1_3")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceInlineUniformBlockFeatures => Self :: PhysicalDeviceInlineUniformBlockFeatures (PhysicalDeviceInlineUniformBlockFeatures :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: vulkan1_3 :: PhysicalDeviceInlineUniformBlockFeatures > ()))) , # [cfg (feature = "VULKAN_1_3")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceMaintenance4Features => Self :: PhysicalDeviceMaintenance4Features (PhysicalDeviceMaintenance4Features :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: vulkan1_3 :: PhysicalDeviceMaintenance4Features > ()))) , crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceShaderDrawParametersFeatures => Self :: PhysicalDeviceShaderDrawParametersFeatures (PhysicalDeviceShaderDrawParametersFeatures :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: vulkan1_1 :: PhysicalDeviceShaderDrawParametersFeatures > ()))) , # [cfg (feature = "VULKAN_1_2")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceShaderFloat16Int8Features => Self :: PhysicalDeviceShaderFloat16Int8Features (PhysicalDeviceShaderFloat16Int8Features :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: vulkan1_2 :: PhysicalDeviceShaderFloat16Int8Features > ()))) , # [cfg (feature = "VULKAN_1_2")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceHostQueryResetFeatures => Self :: PhysicalDeviceHostQueryResetFeatures (PhysicalDeviceHostQueryResetFeatures :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: vulkan1_2 :: PhysicalDeviceHostQueryResetFeatures > ()))) , # [cfg (feature = "VK_KHR_global_priority")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceGlobalPriorityQueryFeaturesKhr => Self :: PhysicalDeviceGlobalPriorityQueryFeaturesKHR (PhysicalDeviceGlobalPriorityQueryFeaturesKHR :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: khr_global_priority :: PhysicalDeviceGlobalPriorityQueryFeaturesKHR > ()))) , # [cfg (feature = "VK_EXT_device_memory_report")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceDeviceMemoryReportFeaturesExt => Self :: PhysicalDeviceDeviceMemoryReportFeaturesEXT (PhysicalDeviceDeviceMemoryReportFeaturesEXT :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: ext_device_memory_report :: PhysicalDeviceDeviceMemoryReportFeaturesEXT > ()))) , # [cfg (feature = "VULKAN_1_2")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceDescriptorIndexingFeatures => Self :: PhysicalDeviceDescriptorIndexingFeatures (PhysicalDeviceDescriptorIndexingFeatures :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: vulkan1_2 :: PhysicalDeviceDescriptorIndexingFeatures > ()))) , # [cfg (feature = "VULKAN_1_2")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceTimelineSemaphoreFeatures => Self :: PhysicalDeviceTimelineSemaphoreFeatures (PhysicalDeviceTimelineSemaphoreFeatures :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: vulkan1_2 :: PhysicalDeviceTimelineSemaphoreFeatures > ()))) , # [cfg (feature = "VULKAN_1_2")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDevice8bitStorageFeatures => Self :: PhysicalDevice8BitStorageFeatures (PhysicalDevice8BitStorageFeatures :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: vulkan1_2 :: PhysicalDevice8BitStorageFeatures > ()))) , # [cfg (feature = "VK_EXT_conditional_rendering")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceConditionalRenderingFeaturesExt => Self :: PhysicalDeviceConditionalRenderingFeaturesEXT (PhysicalDeviceConditionalRenderingFeaturesEXT :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: ext_conditional_rendering :: PhysicalDeviceConditionalRenderingFeaturesEXT > ()))) , # [cfg (feature = "VULKAN_1_2")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceVulkanMemoryModelFeatures => Self :: PhysicalDeviceVulkanMemoryModelFeatures (PhysicalDeviceVulkanMemoryModelFeatures :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: vulkan1_2 :: PhysicalDeviceVulkanMemoryModelFeatures > ()))) , # [cfg (feature = "VULKAN_1_2")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceShaderAtomicInt64Features => Self :: PhysicalDeviceShaderAtomicInt64Features (PhysicalDeviceShaderAtomicInt64Features :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: vulkan1_2 :: PhysicalDeviceShaderAtomicInt64Features > ()))) , # [cfg (feature = "VK_EXT_shader_atomic_float")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceShaderAtomicFloatFeaturesExt => Self :: PhysicalDeviceShaderAtomicFloatFeaturesEXT (PhysicalDeviceShaderAtomicFloatFeaturesEXT :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: ext_shader_atomic_float :: PhysicalDeviceShaderAtomicFloatFeaturesEXT > ()))) , # [cfg (feature = "VK_EXT_shader_atomic_float2")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceShaderAtomicFloat2FeaturesExt => Self :: PhysicalDeviceShaderAtomicFloat2FeaturesEXT (PhysicalDeviceShaderAtomicFloat2FeaturesEXT :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: ext_shader_atomic_float2 :: PhysicalDeviceShaderAtomicFloat2FeaturesEXT > ()))) , # [cfg (feature = "VK_EXT_vertex_attribute_divisor")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceVertexAttributeDivisorFeaturesExt => Self :: PhysicalDeviceVertexAttributeDivisorFeaturesEXT (PhysicalDeviceVertexAttributeDivisorFeaturesEXT :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: ext_vertex_attribute_divisor :: PhysicalDeviceVertexAttributeDivisorFeaturesEXT > ()))) , # [cfg (feature = "VK_EXT_astc_decode_mode")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceAstcDecodeFeaturesExt => Self :: PhysicalDeviceAstcDecodeFeaturesEXT (PhysicalDeviceAstcDecodeFeaturesEXT :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: ext_astc_decode_mode :: PhysicalDeviceAstcDecodeFeaturesEXT > ()))) , # [cfg (feature = "VK_EXT_transform_feedback")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceTransformFeedbackFeaturesExt => Self :: PhysicalDeviceTransformFeedbackFeaturesEXT (PhysicalDeviceTransformFeedbackFeaturesEXT :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: ext_transform_feedback :: PhysicalDeviceTransformFeedbackFeaturesEXT > ()))) , # [cfg (feature = "VK_NV_representative_fragment_test")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceRepresentativeFragmentTestFeaturesNv => Self :: PhysicalDeviceRepresentativeFragmentTestFeaturesNV (PhysicalDeviceRepresentativeFragmentTestFeaturesNV :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: nv_representative_fragment_test :: PhysicalDeviceRepresentativeFragmentTestFeaturesNV > ()))) , # [cfg (feature = "VK_NV_scissor_exclusive")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceExclusiveScissorFeaturesNv => Self :: PhysicalDeviceExclusiveScissorFeaturesNV (PhysicalDeviceExclusiveScissorFeaturesNV :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: nv_scissor_exclusive :: PhysicalDeviceExclusiveScissorFeaturesNV > ()))) , # [cfg (feature = "VK_NV_corner_sampled_image")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceCornerSampledImageFeaturesNv => Self :: PhysicalDeviceCornerSampledImageFeaturesNV (PhysicalDeviceCornerSampledImageFeaturesNV :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: nv_corner_sampled_image :: PhysicalDeviceCornerSampledImageFeaturesNV > ()))) , # [cfg (feature = "VK_NV_compute_shader_derivatives")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceComputeShaderDerivativesFeaturesNv => Self :: PhysicalDeviceComputeShaderDerivativesFeaturesNV (PhysicalDeviceComputeShaderDerivativesFeaturesNV :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: nv_compute_shader_derivatives :: PhysicalDeviceComputeShaderDerivativesFeaturesNV > ()))) , # [cfg (feature = "VK_NV_fragment_shader_barycentric")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceFragmentShaderBarycentricFeaturesNv => Self :: PhysicalDeviceFragmentShaderBarycentricFeaturesNV (PhysicalDeviceFragmentShaderBarycentricFeaturesNV :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: nv_fragment_shader_barycentric :: PhysicalDeviceFragmentShaderBarycentricFeaturesNV > ()))) , # [cfg (feature = "VK_NV_shader_image_footprint")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceShaderImageFootprintFeaturesNv => Self :: PhysicalDeviceShaderImageFootprintFeaturesNV (PhysicalDeviceShaderImageFootprintFeaturesNV :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: nv_shader_image_footprint :: PhysicalDeviceShaderImageFootprintFeaturesNV > ()))) , # [cfg (feature = "VK_NV_dedicated_allocation_image_aliasing")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNv => Self :: PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV (PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: nv_dedicated_allocation_image_aliasing :: PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV > ()))) , # [cfg (feature = "VK_NV_shading_rate_image")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceShadingRateImageFeaturesNv => Self :: PhysicalDeviceShadingRateImageFeaturesNV (PhysicalDeviceShadingRateImageFeaturesNV :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: nv_shading_rate_image :: PhysicalDeviceShadingRateImageFeaturesNV > ()))) , # [cfg (feature = "VK_HUAWEI_invocation_mask")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceInvocationMaskFeaturesHuawei => Self :: PhysicalDeviceInvocationMaskFeaturesHUAWEI (PhysicalDeviceInvocationMaskFeaturesHUAWEI :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: huawei_invocation_mask :: PhysicalDeviceInvocationMaskFeaturesHUAWEI > ()))) , # [cfg (feature = "VK_NV_mesh_shader")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceMeshShaderFeaturesNv => Self :: PhysicalDeviceMeshShaderFeaturesNV (PhysicalDeviceMeshShaderFeaturesNV :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: nv_mesh_shader :: PhysicalDeviceMeshShaderFeaturesNV > ()))) , # [cfg (feature = "VK_KHR_acceleration_structure")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceAccelerationStructureFeaturesKhr => Self :: PhysicalDeviceAccelerationStructureFeaturesKHR (PhysicalDeviceAccelerationStructureFeaturesKHR :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: khr_acceleration_structure :: PhysicalDeviceAccelerationStructureFeaturesKHR > ()))) , # [cfg (feature = "VK_KHR_ray_tracing_pipeline")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceRayTracingPipelineFeaturesKhr => Self :: PhysicalDeviceRayTracingPipelineFeaturesKHR (PhysicalDeviceRayTracingPipelineFeaturesKHR :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: khr_ray_tracing_pipeline :: PhysicalDeviceRayTracingPipelineFeaturesKHR > ()))) , # [cfg (feature = "VK_KHR_ray_query")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceRayQueryFeaturesKhr => Self :: PhysicalDeviceRayQueryFeaturesKHR (PhysicalDeviceRayQueryFeaturesKHR :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: khr_ray_query :: PhysicalDeviceRayQueryFeaturesKHR > ()))) , # [cfg (feature = "VK_EXT_fragment_density_map")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceFragmentDensityMapFeaturesExt => Self :: PhysicalDeviceFragmentDensityMapFeaturesEXT (PhysicalDeviceFragmentDensityMapFeaturesEXT :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: ext_fragment_density_map :: PhysicalDeviceFragmentDensityMapFeaturesEXT > ()))) , # [cfg (feature = "VK_EXT_fragment_density_map2")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceFragmentDensityMap2FeaturesExt => Self :: PhysicalDeviceFragmentDensityMap2FeaturesEXT (PhysicalDeviceFragmentDensityMap2FeaturesEXT :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: ext_fragment_density_map2 :: PhysicalDeviceFragmentDensityMap2FeaturesEXT > ()))) , # [cfg (feature = "VULKAN_1_2")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceScalarBlockLayoutFeatures => Self :: PhysicalDeviceScalarBlockLayoutFeatures (PhysicalDeviceScalarBlockLayoutFeatures :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: vulkan1_2 :: PhysicalDeviceScalarBlockLayoutFeatures > ()))) , # [cfg (feature = "VULKAN_1_2")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceUniformBufferStandardLayoutFeatures => Self :: PhysicalDeviceUniformBufferStandardLayoutFeatures (PhysicalDeviceUniformBufferStandardLayoutFeatures :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: vulkan1_2 :: PhysicalDeviceUniformBufferStandardLayoutFeatures > ()))) , # [cfg (feature = "VK_EXT_depth_clip_enable")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceDepthClipEnableFeaturesExt => Self :: PhysicalDeviceDepthClipEnableFeaturesEXT (PhysicalDeviceDepthClipEnableFeaturesEXT :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: ext_depth_clip_enable :: PhysicalDeviceDepthClipEnableFeaturesEXT > ()))) , # [cfg (feature = "VK_EXT_memory_priority")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceMemoryPriorityFeaturesExt => Self :: PhysicalDeviceMemoryPriorityFeaturesEXT (PhysicalDeviceMemoryPriorityFeaturesEXT :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: ext_memory_priority :: PhysicalDeviceMemoryPriorityFeaturesEXT > ()))) , # [cfg (feature = "VK_EXT_pageable_device_local_memory")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDevicePageableDeviceLocalMemoryFeaturesExt => Self :: PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT (PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: ext_pageable_device_local_memory :: PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT > ()))) , # [cfg (feature = "VULKAN_1_2")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceBufferDeviceAddressFeatures => Self :: PhysicalDeviceBufferDeviceAddressFeatures (PhysicalDeviceBufferDeviceAddressFeatures :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: vulkan1_2 :: PhysicalDeviceBufferDeviceAddressFeatures > ()))) , # [cfg (feature = "VK_EXT_buffer_device_address")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceBufferDeviceAddressFeaturesExt => Self :: PhysicalDeviceBufferDeviceAddressFeaturesEXT (PhysicalDeviceBufferDeviceAddressFeaturesEXT :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: ext_buffer_device_address :: PhysicalDeviceBufferDeviceAddressFeaturesEXT > ()))) , # [cfg (feature = "VULKAN_1_2")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceImagelessFramebufferFeatures => Self :: PhysicalDeviceImagelessFramebufferFeatures (PhysicalDeviceImagelessFramebufferFeatures :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: vulkan1_2 :: PhysicalDeviceImagelessFramebufferFeatures > ()))) , # [cfg (feature = "VULKAN_1_3")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceTextureCompressionAstcHdrFeatures => Self :: PhysicalDeviceTextureCompressionAstchdrFeatures (PhysicalDeviceTextureCompressionAstchdrFeatures :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: vulkan1_3 :: PhysicalDeviceTextureCompressionAstchdrFeatures > ()))) , # [cfg (feature = "VK_NV_cooperative_matrix")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceCooperativeMatrixFeaturesNv => Self :: PhysicalDeviceCooperativeMatrixFeaturesNV (PhysicalDeviceCooperativeMatrixFeaturesNV :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: nv_cooperative_matrix :: PhysicalDeviceCooperativeMatrixFeaturesNV > ()))) , # [cfg (feature = "VK_EXT_ycbcr_image_arrays")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceYcbcrImageArraysFeaturesExt => Self :: PhysicalDeviceYcbcrImageArraysFeaturesEXT (PhysicalDeviceYcbcrImageArraysFeaturesEXT :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: ext_ycbcr_image_arrays :: PhysicalDeviceYcbcrImageArraysFeaturesEXT > ()))) , # [cfg (feature = "VK_KHR_performance_query")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDevicePerformanceQueryFeaturesKhr => Self :: PhysicalDevicePerformanceQueryFeaturesKHR (PhysicalDevicePerformanceQueryFeaturesKHR :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: khr_performance_query :: PhysicalDevicePerformanceQueryFeaturesKHR > ()))) , # [cfg (feature = "VK_NV_coverage_reduction_mode")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceCoverageReductionModeFeaturesNv => Self :: PhysicalDeviceCoverageReductionModeFeaturesNV (PhysicalDeviceCoverageReductionModeFeaturesNV :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: nv_coverage_reduction_mode :: PhysicalDeviceCoverageReductionModeFeaturesNV > ()))) , # [cfg (feature = "VK_INTEL_shader_integer_functions2")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceShaderIntegerFunctions2FeaturesIntel => Self :: PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL (PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: intel_shader_integer_functions2 :: PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL > ()))) , # [cfg (feature = "VK_KHR_shader_clock")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceShaderClockFeaturesKhr => Self :: PhysicalDeviceShaderClockFeaturesKHR (PhysicalDeviceShaderClockFeaturesKHR :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: khr_shader_clock :: PhysicalDeviceShaderClockFeaturesKHR > ()))) , # [cfg (feature = "VK_EXT_index_type_uint8")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceIndexTypeUint8FeaturesExt => Self :: PhysicalDeviceIndexTypeUint8FeaturesEXT (PhysicalDeviceIndexTypeUint8FeaturesEXT :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: ext_index_type_uint8 :: PhysicalDeviceIndexTypeUint8FeaturesEXT > ()))) , # [cfg (feature = "VK_NV_shader_sm_builtins")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceShaderSmBuiltinsFeaturesNv => Self :: PhysicalDeviceShaderSmBuiltinsFeaturesNV (PhysicalDeviceShaderSmBuiltinsFeaturesNV :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: nv_shader_sm_builtins :: PhysicalDeviceShaderSmBuiltinsFeaturesNV > ()))) , # [cfg (feature = "VK_EXT_fragment_shader_interlock")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceFragmentShaderInterlockFeaturesExt => Self :: PhysicalDeviceFragmentShaderInterlockFeaturesEXT (PhysicalDeviceFragmentShaderInterlockFeaturesEXT :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: ext_fragment_shader_interlock :: PhysicalDeviceFragmentShaderInterlockFeaturesEXT > ()))) , # [cfg (feature = "VULKAN_1_2")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceSeparateDepthStencilLayoutsFeatures => Self :: PhysicalDeviceSeparateDepthStencilLayoutsFeatures (PhysicalDeviceSeparateDepthStencilLayoutsFeatures :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: vulkan1_2 :: PhysicalDeviceSeparateDepthStencilLayoutsFeatures > ()))) , # [cfg (feature = "VK_EXT_primitive_topology_list_restart")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDevicePrimitiveTopologyListRestartFeaturesExt => Self :: PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT (PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: ext_primitive_topology_list_restart :: PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT > ()))) , # [cfg (feature = "VK_KHR_pipeline_executable_properties")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDevicePipelineExecutablePropertiesFeaturesKhr => Self :: PhysicalDevicePipelineExecutablePropertiesFeaturesKHR (PhysicalDevicePipelineExecutablePropertiesFeaturesKHR :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: khr_pipeline_executable_properties :: PhysicalDevicePipelineExecutablePropertiesFeaturesKHR > ()))) , # [cfg (feature = "VULKAN_1_3")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceShaderDemoteToHelperInvocationFeatures => Self :: PhysicalDeviceShaderDemoteToHelperInvocationFeatures (PhysicalDeviceShaderDemoteToHelperInvocationFeatures :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: vulkan1_3 :: PhysicalDeviceShaderDemoteToHelperInvocationFeatures > ()))) , # [cfg (feature = "VK_EXT_texel_buffer_alignment")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceTexelBufferAlignmentFeaturesExt => Self :: PhysicalDeviceTexelBufferAlignmentFeaturesEXT (PhysicalDeviceTexelBufferAlignmentFeaturesEXT :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: ext_texel_buffer_alignment :: PhysicalDeviceTexelBufferAlignmentFeaturesEXT > ()))) , # [cfg (feature = "VULKAN_1_3")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceSubgroupSizeControlFeatures => Self :: PhysicalDeviceSubgroupSizeControlFeatures (PhysicalDeviceSubgroupSizeControlFeatures :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: vulkan1_3 :: PhysicalDeviceSubgroupSizeControlFeatures > ()))) , # [cfg (feature = "VK_EXT_line_rasterization")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceLineRasterizationFeaturesExt => Self :: PhysicalDeviceLineRasterizationFeaturesEXT (PhysicalDeviceLineRasterizationFeaturesEXT :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: ext_line_rasterization :: PhysicalDeviceLineRasterizationFeaturesEXT > ()))) , # [cfg (feature = "VULKAN_1_3")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDevicePipelineCreationCacheControlFeatures => Self :: PhysicalDevicePipelineCreationCacheControlFeatures (PhysicalDevicePipelineCreationCacheControlFeatures :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: vulkan1_3 :: PhysicalDevicePipelineCreationCacheControlFeatures > ()))) , # [cfg (feature = "VULKAN_1_2")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceVulkan11Features => Self :: PhysicalDeviceVulkan11Features (PhysicalDeviceVulkan11Features :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: vulkan1_2 :: PhysicalDeviceVulkan11Features > ()))) , # [cfg (feature = "VULKAN_1_2")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceVulkan12Features => Self :: PhysicalDeviceVulkan12Features (PhysicalDeviceVulkan12Features :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: vulkan1_2 :: PhysicalDeviceVulkan12Features > ()))) , # [cfg (feature = "VULKAN_1_3")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceVulkan13Features => Self :: PhysicalDeviceVulkan13Features (PhysicalDeviceVulkan13Features :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: vulkan1_3 :: PhysicalDeviceVulkan13Features > ()))) , # [cfg (feature = "VK_AMD_device_coherent_memory")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceCoherentMemoryFeaturesAmd => Self :: PhysicalDeviceCoherentMemoryFeaturesAMD (PhysicalDeviceCoherentMemoryFeaturesAMD :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: amd_device_coherent_memory :: PhysicalDeviceCoherentMemoryFeaturesAMD > ()))) , # [cfg (feature = "VK_EXT_custom_border_color")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceCustomBorderColorFeaturesExt => Self :: PhysicalDeviceCustomBorderColorFeaturesEXT (PhysicalDeviceCustomBorderColorFeaturesEXT :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: ext_custom_border_color :: PhysicalDeviceCustomBorderColorFeaturesEXT > ()))) , # [cfg (feature = "VK_EXT_border_color_swizzle")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceBorderColorSwizzleFeaturesExt => Self :: PhysicalDeviceBorderColorSwizzleFeaturesEXT (PhysicalDeviceBorderColorSwizzleFeaturesEXT :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: ext_border_color_swizzle :: PhysicalDeviceBorderColorSwizzleFeaturesEXT > ()))) , # [cfg (feature = "VK_EXT_extended_dynamic_state")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceExtendedDynamicStateFeaturesExt => Self :: PhysicalDeviceExtendedDynamicStateFeaturesEXT (PhysicalDeviceExtendedDynamicStateFeaturesEXT :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: ext_extended_dynamic_state :: PhysicalDeviceExtendedDynamicStateFeaturesEXT > ()))) , # [cfg (feature = "VK_EXT_extended_dynamic_state2")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceExtendedDynamicState2FeaturesExt => Self :: PhysicalDeviceExtendedDynamicState2FeaturesEXT (PhysicalDeviceExtendedDynamicState2FeaturesEXT :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: ext_extended_dynamic_state2 :: PhysicalDeviceExtendedDynamicState2FeaturesEXT > ()))) , # [cfg (feature = "VK_NV_device_diagnostics_config")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceDiagnosticsConfigFeaturesNv => Self :: PhysicalDeviceDiagnosticsConfigFeaturesNV (PhysicalDeviceDiagnosticsConfigFeaturesNV :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: nv_device_diagnostics_config :: PhysicalDeviceDiagnosticsConfigFeaturesNV > ()))) , # [cfg (feature = "VULKAN_1_3")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures => Self :: PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures (PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: vulkan1_3 :: PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures > ()))) , # [cfg (feature = "VK_KHR_shader_subgroup_uniform_control_flow")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKhr => Self :: PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR (PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: khr_shader_subgroup_uniform_control_flow :: PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR > ()))) , # [cfg (feature = "VK_EXT_robustness2")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceRobustness2FeaturesExt => Self :: PhysicalDeviceRobustness2FeaturesEXT (PhysicalDeviceRobustness2FeaturesEXT :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: ext_robustness2 :: PhysicalDeviceRobustness2FeaturesEXT > ()))) , # [cfg (feature = "VULKAN_1_3")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceImageRobustnessFeatures => Self :: PhysicalDeviceImageRobustnessFeatures (PhysicalDeviceImageRobustnessFeatures :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: vulkan1_3 :: PhysicalDeviceImageRobustnessFeatures > ()))) , # [cfg (feature = "VK_KHR_workgroup_memory_explicit_layout")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKhr => Self :: PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR (PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: khr_workgroup_memory_explicit_layout :: PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR > ()))) , # [cfg (feature = "VK_KHR_portability_subset")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDevicePortabilitySubsetFeaturesKhr => Self :: PhysicalDevicePortabilitySubsetFeaturesKHR (PhysicalDevicePortabilitySubsetFeaturesKHR :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: khr_portability_subset :: PhysicalDevicePortabilitySubsetFeaturesKHR > ()))) , # [cfg (feature = "VK_EXT_4444_formats")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDevice4444FormatsFeaturesExt => Self :: PhysicalDevice4444FormatsFeaturesEXT (PhysicalDevice4444FormatsFeaturesEXT :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: ext_4444_formats :: PhysicalDevice4444FormatsFeaturesEXT > ()))) , # [cfg (feature = "VK_HUAWEI_subpass_shading")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceSubpassShadingFeaturesHuawei => Self :: PhysicalDeviceSubpassShadingFeaturesHUAWEI (PhysicalDeviceSubpassShadingFeaturesHUAWEI :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: huawei_subpass_shading :: PhysicalDeviceSubpassShadingFeaturesHUAWEI > ()))) , # [cfg (feature = "VK_EXT_shader_image_atomic_int64")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceShaderImageAtomicInt64FeaturesExt => Self :: PhysicalDeviceShaderImageAtomicInt64FeaturesEXT (PhysicalDeviceShaderImageAtomicInt64FeaturesEXT :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: ext_shader_image_atomic_int64 :: PhysicalDeviceShaderImageAtomicInt64FeaturesEXT > ()))) , # [cfg (feature = "VK_KHR_fragment_shading_rate")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceFragmentShadingRateFeaturesKhr => Self :: PhysicalDeviceFragmentShadingRateFeaturesKHR (PhysicalDeviceFragmentShadingRateFeaturesKHR :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: khr_fragment_shading_rate :: PhysicalDeviceFragmentShadingRateFeaturesKHR > ()))) , # [cfg (feature = "VULKAN_1_3")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceShaderTerminateInvocationFeatures => Self :: PhysicalDeviceShaderTerminateInvocationFeatures (PhysicalDeviceShaderTerminateInvocationFeatures :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: vulkan1_3 :: PhysicalDeviceShaderTerminateInvocationFeatures > ()))) , # [cfg (feature = "VK_NV_fragment_shading_rate_enums")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceFragmentShadingRateEnumsFeaturesNv => Self :: PhysicalDeviceFragmentShadingRateEnumsFeaturesNV (PhysicalDeviceFragmentShadingRateEnumsFeaturesNV :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: nv_fragment_shading_rate_enums :: PhysicalDeviceFragmentShadingRateEnumsFeaturesNV > ()))) , # [cfg (feature = "VK_VALVE_mutable_descriptor_type")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceMutableDescriptorTypeFeaturesValve => Self :: PhysicalDeviceMutableDescriptorTypeFeaturesVALVE (PhysicalDeviceMutableDescriptorTypeFeaturesVALVE :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: valve_mutable_descriptor_type :: PhysicalDeviceMutableDescriptorTypeFeaturesVALVE > ()))) , # [cfg (feature = "VK_EXT_depth_clip_control")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceDepthClipControlFeaturesExt => Self :: PhysicalDeviceDepthClipControlFeaturesEXT (PhysicalDeviceDepthClipControlFeaturesEXT :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: ext_depth_clip_control :: PhysicalDeviceDepthClipControlFeaturesEXT > ()))) , # [cfg (feature = "VK_EXT_vertex_input_dynamic_state")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceVertexInputDynamicStateFeaturesExt => Self :: PhysicalDeviceVertexInputDynamicStateFeaturesEXT (PhysicalDeviceVertexInputDynamicStateFeaturesEXT :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: ext_vertex_input_dynamic_state :: PhysicalDeviceVertexInputDynamicStateFeaturesEXT > ()))) , # [cfg (feature = "VK_NV_external_memory_rdma")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceExternalMemoryRdmaFeaturesNv => Self :: PhysicalDeviceExternalMemoryRdmaFeaturesNV (PhysicalDeviceExternalMemoryRdmaFeaturesNV :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: nv_external_memory_rdma :: PhysicalDeviceExternalMemoryRdmaFeaturesNV > ()))) , # [cfg (feature = "VK_EXT_color_write_enable")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceColorWriteEnableFeaturesExt => Self :: PhysicalDeviceColorWriteEnableFeaturesEXT (PhysicalDeviceColorWriteEnableFeaturesEXT :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: ext_color_write_enable :: PhysicalDeviceColorWriteEnableFeaturesEXT > ()))) , # [cfg (feature = "VULKAN_1_3")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceSynchronization2Features => Self :: PhysicalDeviceSynchronization2Features (PhysicalDeviceSynchronization2Features :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: vulkan1_3 :: PhysicalDeviceSynchronization2Features > ()))) , # [cfg (feature = "VK_NV_inherited_viewport_scissor")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceInheritedViewportScissorFeaturesNv => Self :: PhysicalDeviceInheritedViewportScissorFeaturesNV (PhysicalDeviceInheritedViewportScissorFeaturesNV :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: nv_inherited_viewport_scissor :: PhysicalDeviceInheritedViewportScissorFeaturesNV > ()))) , # [cfg (feature = "VK_EXT_ycbcr_2plane_444_formats")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceYcbcr2Plane444FormatsFeaturesExt => Self :: PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT (PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: ext_ycbcr_2plane_444_formats :: PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT > ()))) , # [cfg (feature = "VK_EXT_provoking_vertex")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceProvokingVertexFeaturesExt => Self :: PhysicalDeviceProvokingVertexFeaturesEXT (PhysicalDeviceProvokingVertexFeaturesEXT :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: ext_provoking_vertex :: PhysicalDeviceProvokingVertexFeaturesEXT > ()))) , # [cfg (feature = "VULKAN_1_3")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceShaderIntegerDotProductFeatures => Self :: PhysicalDeviceShaderIntegerDotProductFeatures (PhysicalDeviceShaderIntegerDotProductFeatures :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: vulkan1_3 :: PhysicalDeviceShaderIntegerDotProductFeatures > ()))) , # [cfg (feature = "VK_NV_ray_tracing_motion_blur")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceRayTracingMotionBlurFeaturesNv => Self :: PhysicalDeviceRayTracingMotionBlurFeaturesNV (PhysicalDeviceRayTracingMotionBlurFeaturesNV :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: nv_ray_tracing_motion_blur :: PhysicalDeviceRayTracingMotionBlurFeaturesNV > ()))) , # [cfg (feature = "VK_EXT_rgba10x6_formats")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceRgba10x6FormatsFeaturesExt => Self :: PhysicalDeviceRgba10x6FormatsFeaturesEXT (PhysicalDeviceRgba10x6FormatsFeaturesEXT :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: ext_rgba10x6_formats :: PhysicalDeviceRgba10x6FormatsFeaturesEXT > ()))) , # [cfg (feature = "VULKAN_1_3")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceDynamicRenderingFeatures => Self :: PhysicalDeviceDynamicRenderingFeatures (PhysicalDeviceDynamicRenderingFeatures :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: vulkan1_3 :: PhysicalDeviceDynamicRenderingFeatures > ()))) , # [cfg (feature = "VK_EXT_image_view_min_lod")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceImageViewMinLodFeaturesExt => Self :: PhysicalDeviceImageViewMinLodFeaturesEXT (PhysicalDeviceImageViewMinLodFeaturesEXT :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: ext_image_view_min_lod :: PhysicalDeviceImageViewMinLodFeaturesEXT > ()))) , # [cfg (feature = "VK_ARM_rasterization_order_attachment_access")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesArm => Self :: PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM (PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: arm_rasterization_order_attachment_access :: PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM > ()))) , # [cfg (feature = "VK_NV_linear_color_attachment")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceLinearColorAttachmentFeaturesNv => Self :: PhysicalDeviceLinearColorAttachmentFeaturesNV (PhysicalDeviceLinearColorAttachmentFeaturesNV :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: nv_linear_color_attachment :: PhysicalDeviceLinearColorAttachmentFeaturesNV > ()))) , other => panic ! ("Structure type {:?} is not a member of {}" , other , stringify ! (PhysicalDeviceFeatures2)) }
    }
}
#[cfg(feature = "VK_NV_device_generated_commands")]
impl From<PhysicalDeviceDeviceGeneratedCommandsFeaturesNV> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceDeviceGeneratedCommandsFeaturesNV) -> Self {
        Self::PhysicalDeviceDeviceGeneratedCommandsFeaturesNV(ext)
    }
}
#[cfg(feature = "VK_NV_device_generated_commands")]
impl TryInto<PhysicalDeviceDeviceGeneratedCommandsFeaturesNV> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceDeviceGeneratedCommandsFeaturesNV, Self::Error> {
        match self {
            Self::PhysicalDeviceDeviceGeneratedCommandsFeaturesNV(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VULKAN_1_3")]
impl From<PhysicalDevicePrivateDataFeatures> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDevicePrivateDataFeatures) -> Self {
        Self::PhysicalDevicePrivateDataFeatures(ext)
    }
}
#[cfg(feature = "VULKAN_1_3")]
impl TryInto<PhysicalDevicePrivateDataFeatures> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDevicePrivateDataFeatures, Self::Error> {
        match self {
            Self::PhysicalDevicePrivateDataFeatures(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
impl From<PhysicalDeviceVariablePointersFeatures> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceVariablePointersFeatures) -> Self {
        Self::PhysicalDeviceVariablePointersFeatures(ext)
    }
}
impl TryInto<PhysicalDeviceVariablePointersFeatures> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceVariablePointersFeatures, Self::Error> {
        match self {
            Self::PhysicalDeviceVariablePointersFeatures(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
impl From<PhysicalDeviceMultiviewFeatures> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceMultiviewFeatures) -> Self {
        Self::PhysicalDeviceMultiviewFeatures(ext)
    }
}
impl TryInto<PhysicalDeviceMultiviewFeatures> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceMultiviewFeatures, Self::Error> {
        match self {
            Self::PhysicalDeviceMultiviewFeatures(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_KHR_present_id")]
impl From<PhysicalDevicePresentIdFeaturesKHR> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDevicePresentIdFeaturesKHR) -> Self {
        Self::PhysicalDevicePresentIdFeaturesKHR(ext)
    }
}
#[cfg(feature = "VK_KHR_present_id")]
impl TryInto<PhysicalDevicePresentIdFeaturesKHR> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDevicePresentIdFeaturesKHR, Self::Error> {
        match self {
            Self::PhysicalDevicePresentIdFeaturesKHR(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_KHR_present_wait")]
impl From<PhysicalDevicePresentWaitFeaturesKHR> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDevicePresentWaitFeaturesKHR) -> Self {
        Self::PhysicalDevicePresentWaitFeaturesKHR(ext)
    }
}
#[cfg(feature = "VK_KHR_present_wait")]
impl TryInto<PhysicalDevicePresentWaitFeaturesKHR> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDevicePresentWaitFeaturesKHR, Self::Error> {
        match self {
            Self::PhysicalDevicePresentWaitFeaturesKHR(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
impl From<PhysicalDevice16BitStorageFeatures> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDevice16BitStorageFeatures) -> Self {
        Self::PhysicalDevice16BitStorageFeatures(ext)
    }
}
impl TryInto<PhysicalDevice16BitStorageFeatures> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDevice16BitStorageFeatures, Self::Error> {
        match self {
            Self::PhysicalDevice16BitStorageFeatures(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VULKAN_1_2")]
impl From<PhysicalDeviceShaderSubgroupExtendedTypesFeatures> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceShaderSubgroupExtendedTypesFeatures) -> Self {
        Self::PhysicalDeviceShaderSubgroupExtendedTypesFeatures(ext)
    }
}
#[cfg(feature = "VULKAN_1_2")]
impl TryInto<PhysicalDeviceShaderSubgroupExtendedTypesFeatures> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceShaderSubgroupExtendedTypesFeatures, Self::Error> {
        match self {
            Self::PhysicalDeviceShaderSubgroupExtendedTypesFeatures(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
impl From<PhysicalDeviceSamplerYcbcrConversionFeatures> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceSamplerYcbcrConversionFeatures) -> Self {
        Self::PhysicalDeviceSamplerYcbcrConversionFeatures(ext)
    }
}
impl TryInto<PhysicalDeviceSamplerYcbcrConversionFeatures> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceSamplerYcbcrConversionFeatures, Self::Error> {
        match self {
            Self::PhysicalDeviceSamplerYcbcrConversionFeatures(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
impl From<PhysicalDeviceProtectedMemoryFeatures> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceProtectedMemoryFeatures) -> Self {
        Self::PhysicalDeviceProtectedMemoryFeatures(ext)
    }
}
impl TryInto<PhysicalDeviceProtectedMemoryFeatures> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceProtectedMemoryFeatures, Self::Error> {
        match self {
            Self::PhysicalDeviceProtectedMemoryFeatures(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
impl From<PhysicalDeviceBlendOperationAdvancedFeaturesEXT> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceBlendOperationAdvancedFeaturesEXT) -> Self {
        Self::PhysicalDeviceBlendOperationAdvancedFeaturesEXT(ext)
    }
}
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
impl TryInto<PhysicalDeviceBlendOperationAdvancedFeaturesEXT> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceBlendOperationAdvancedFeaturesEXT, Self::Error> {
        match self {
            Self::PhysicalDeviceBlendOperationAdvancedFeaturesEXT(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_EXT_multi_draw")]
impl From<PhysicalDeviceMultiDrawFeaturesEXT> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceMultiDrawFeaturesEXT) -> Self {
        Self::PhysicalDeviceMultiDrawFeaturesEXT(ext)
    }
}
#[cfg(feature = "VK_EXT_multi_draw")]
impl TryInto<PhysicalDeviceMultiDrawFeaturesEXT> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceMultiDrawFeaturesEXT, Self::Error> {
        match self {
            Self::PhysicalDeviceMultiDrawFeaturesEXT(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VULKAN_1_3")]
impl From<PhysicalDeviceInlineUniformBlockFeatures> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceInlineUniformBlockFeatures) -> Self {
        Self::PhysicalDeviceInlineUniformBlockFeatures(ext)
    }
}
#[cfg(feature = "VULKAN_1_3")]
impl TryInto<PhysicalDeviceInlineUniformBlockFeatures> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceInlineUniformBlockFeatures, Self::Error> {
        match self {
            Self::PhysicalDeviceInlineUniformBlockFeatures(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VULKAN_1_3")]
impl From<PhysicalDeviceMaintenance4Features> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceMaintenance4Features) -> Self {
        Self::PhysicalDeviceMaintenance4Features(ext)
    }
}
#[cfg(feature = "VULKAN_1_3")]
impl TryInto<PhysicalDeviceMaintenance4Features> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceMaintenance4Features, Self::Error> {
        match self {
            Self::PhysicalDeviceMaintenance4Features(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
impl From<PhysicalDeviceShaderDrawParametersFeatures> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceShaderDrawParametersFeatures) -> Self {
        Self::PhysicalDeviceShaderDrawParametersFeatures(ext)
    }
}
impl TryInto<PhysicalDeviceShaderDrawParametersFeatures> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceShaderDrawParametersFeatures, Self::Error> {
        match self {
            Self::PhysicalDeviceShaderDrawParametersFeatures(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VULKAN_1_2")]
impl From<PhysicalDeviceShaderFloat16Int8Features> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceShaderFloat16Int8Features) -> Self {
        Self::PhysicalDeviceShaderFloat16Int8Features(ext)
    }
}
#[cfg(feature = "VULKAN_1_2")]
impl TryInto<PhysicalDeviceShaderFloat16Int8Features> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceShaderFloat16Int8Features, Self::Error> {
        match self {
            Self::PhysicalDeviceShaderFloat16Int8Features(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VULKAN_1_2")]
impl From<PhysicalDeviceHostQueryResetFeatures> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceHostQueryResetFeatures) -> Self {
        Self::PhysicalDeviceHostQueryResetFeatures(ext)
    }
}
#[cfg(feature = "VULKAN_1_2")]
impl TryInto<PhysicalDeviceHostQueryResetFeatures> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceHostQueryResetFeatures, Self::Error> {
        match self {
            Self::PhysicalDeviceHostQueryResetFeatures(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_KHR_global_priority")]
impl From<PhysicalDeviceGlobalPriorityQueryFeaturesKHR> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceGlobalPriorityQueryFeaturesKHR) -> Self {
        Self::PhysicalDeviceGlobalPriorityQueryFeaturesKHR(ext)
    }
}
#[cfg(feature = "VK_KHR_global_priority")]
impl TryInto<PhysicalDeviceGlobalPriorityQueryFeaturesKHR> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceGlobalPriorityQueryFeaturesKHR, Self::Error> {
        match self {
            Self::PhysicalDeviceGlobalPriorityQueryFeaturesKHR(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_EXT_device_memory_report")]
impl From<PhysicalDeviceDeviceMemoryReportFeaturesEXT> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceDeviceMemoryReportFeaturesEXT) -> Self {
        Self::PhysicalDeviceDeviceMemoryReportFeaturesEXT(ext)
    }
}
#[cfg(feature = "VK_EXT_device_memory_report")]
impl TryInto<PhysicalDeviceDeviceMemoryReportFeaturesEXT> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceDeviceMemoryReportFeaturesEXT, Self::Error> {
        match self {
            Self::PhysicalDeviceDeviceMemoryReportFeaturesEXT(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VULKAN_1_2")]
impl From<PhysicalDeviceDescriptorIndexingFeatures> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceDescriptorIndexingFeatures) -> Self {
        Self::PhysicalDeviceDescriptorIndexingFeatures(ext)
    }
}
#[cfg(feature = "VULKAN_1_2")]
impl TryInto<PhysicalDeviceDescriptorIndexingFeatures> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceDescriptorIndexingFeatures, Self::Error> {
        match self {
            Self::PhysicalDeviceDescriptorIndexingFeatures(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VULKAN_1_2")]
impl From<PhysicalDeviceTimelineSemaphoreFeatures> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceTimelineSemaphoreFeatures) -> Self {
        Self::PhysicalDeviceTimelineSemaphoreFeatures(ext)
    }
}
#[cfg(feature = "VULKAN_1_2")]
impl TryInto<PhysicalDeviceTimelineSemaphoreFeatures> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceTimelineSemaphoreFeatures, Self::Error> {
        match self {
            Self::PhysicalDeviceTimelineSemaphoreFeatures(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VULKAN_1_2")]
impl From<PhysicalDevice8BitStorageFeatures> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDevice8BitStorageFeatures) -> Self {
        Self::PhysicalDevice8BitStorageFeatures(ext)
    }
}
#[cfg(feature = "VULKAN_1_2")]
impl TryInto<PhysicalDevice8BitStorageFeatures> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDevice8BitStorageFeatures, Self::Error> {
        match self {
            Self::PhysicalDevice8BitStorageFeatures(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_EXT_conditional_rendering")]
impl From<PhysicalDeviceConditionalRenderingFeaturesEXT> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceConditionalRenderingFeaturesEXT) -> Self {
        Self::PhysicalDeviceConditionalRenderingFeaturesEXT(ext)
    }
}
#[cfg(feature = "VK_EXT_conditional_rendering")]
impl TryInto<PhysicalDeviceConditionalRenderingFeaturesEXT> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceConditionalRenderingFeaturesEXT, Self::Error> {
        match self {
            Self::PhysicalDeviceConditionalRenderingFeaturesEXT(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VULKAN_1_2")]
impl From<PhysicalDeviceVulkanMemoryModelFeatures> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceVulkanMemoryModelFeatures) -> Self {
        Self::PhysicalDeviceVulkanMemoryModelFeatures(ext)
    }
}
#[cfg(feature = "VULKAN_1_2")]
impl TryInto<PhysicalDeviceVulkanMemoryModelFeatures> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceVulkanMemoryModelFeatures, Self::Error> {
        match self {
            Self::PhysicalDeviceVulkanMemoryModelFeatures(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VULKAN_1_2")]
impl From<PhysicalDeviceShaderAtomicInt64Features> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceShaderAtomicInt64Features) -> Self {
        Self::PhysicalDeviceShaderAtomicInt64Features(ext)
    }
}
#[cfg(feature = "VULKAN_1_2")]
impl TryInto<PhysicalDeviceShaderAtomicInt64Features> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceShaderAtomicInt64Features, Self::Error> {
        match self {
            Self::PhysicalDeviceShaderAtomicInt64Features(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_EXT_shader_atomic_float")]
impl From<PhysicalDeviceShaderAtomicFloatFeaturesEXT> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceShaderAtomicFloatFeaturesEXT) -> Self {
        Self::PhysicalDeviceShaderAtomicFloatFeaturesEXT(ext)
    }
}
#[cfg(feature = "VK_EXT_shader_atomic_float")]
impl TryInto<PhysicalDeviceShaderAtomicFloatFeaturesEXT> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceShaderAtomicFloatFeaturesEXT, Self::Error> {
        match self {
            Self::PhysicalDeviceShaderAtomicFloatFeaturesEXT(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_EXT_shader_atomic_float2")]
impl From<PhysicalDeviceShaderAtomicFloat2FeaturesEXT> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceShaderAtomicFloat2FeaturesEXT) -> Self {
        Self::PhysicalDeviceShaderAtomicFloat2FeaturesEXT(ext)
    }
}
#[cfg(feature = "VK_EXT_shader_atomic_float2")]
impl TryInto<PhysicalDeviceShaderAtomicFloat2FeaturesEXT> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceShaderAtomicFloat2FeaturesEXT, Self::Error> {
        match self {
            Self::PhysicalDeviceShaderAtomicFloat2FeaturesEXT(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_EXT_vertex_attribute_divisor")]
impl From<PhysicalDeviceVertexAttributeDivisorFeaturesEXT> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceVertexAttributeDivisorFeaturesEXT) -> Self {
        Self::PhysicalDeviceVertexAttributeDivisorFeaturesEXT(ext)
    }
}
#[cfg(feature = "VK_EXT_vertex_attribute_divisor")]
impl TryInto<PhysicalDeviceVertexAttributeDivisorFeaturesEXT> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceVertexAttributeDivisorFeaturesEXT, Self::Error> {
        match self {
            Self::PhysicalDeviceVertexAttributeDivisorFeaturesEXT(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_EXT_astc_decode_mode")]
impl From<PhysicalDeviceAstcDecodeFeaturesEXT> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceAstcDecodeFeaturesEXT) -> Self {
        Self::PhysicalDeviceAstcDecodeFeaturesEXT(ext)
    }
}
#[cfg(feature = "VK_EXT_astc_decode_mode")]
impl TryInto<PhysicalDeviceAstcDecodeFeaturesEXT> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceAstcDecodeFeaturesEXT, Self::Error> {
        match self {
            Self::PhysicalDeviceAstcDecodeFeaturesEXT(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_EXT_transform_feedback")]
impl From<PhysicalDeviceTransformFeedbackFeaturesEXT> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceTransformFeedbackFeaturesEXT) -> Self {
        Self::PhysicalDeviceTransformFeedbackFeaturesEXT(ext)
    }
}
#[cfg(feature = "VK_EXT_transform_feedback")]
impl TryInto<PhysicalDeviceTransformFeedbackFeaturesEXT> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceTransformFeedbackFeaturesEXT, Self::Error> {
        match self {
            Self::PhysicalDeviceTransformFeedbackFeaturesEXT(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_NV_representative_fragment_test")]
impl From<PhysicalDeviceRepresentativeFragmentTestFeaturesNV> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceRepresentativeFragmentTestFeaturesNV) -> Self {
        Self::PhysicalDeviceRepresentativeFragmentTestFeaturesNV(ext)
    }
}
#[cfg(feature = "VK_NV_representative_fragment_test")]
impl TryInto<PhysicalDeviceRepresentativeFragmentTestFeaturesNV> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceRepresentativeFragmentTestFeaturesNV, Self::Error> {
        match self {
            Self::PhysicalDeviceRepresentativeFragmentTestFeaturesNV(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_NV_scissor_exclusive")]
impl From<PhysicalDeviceExclusiveScissorFeaturesNV> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceExclusiveScissorFeaturesNV) -> Self {
        Self::PhysicalDeviceExclusiveScissorFeaturesNV(ext)
    }
}
#[cfg(feature = "VK_NV_scissor_exclusive")]
impl TryInto<PhysicalDeviceExclusiveScissorFeaturesNV> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceExclusiveScissorFeaturesNV, Self::Error> {
        match self {
            Self::PhysicalDeviceExclusiveScissorFeaturesNV(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_NV_corner_sampled_image")]
impl From<PhysicalDeviceCornerSampledImageFeaturesNV> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceCornerSampledImageFeaturesNV) -> Self {
        Self::PhysicalDeviceCornerSampledImageFeaturesNV(ext)
    }
}
#[cfg(feature = "VK_NV_corner_sampled_image")]
impl TryInto<PhysicalDeviceCornerSampledImageFeaturesNV> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceCornerSampledImageFeaturesNV, Self::Error> {
        match self {
            Self::PhysicalDeviceCornerSampledImageFeaturesNV(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_NV_compute_shader_derivatives")]
impl From<PhysicalDeviceComputeShaderDerivativesFeaturesNV> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceComputeShaderDerivativesFeaturesNV) -> Self {
        Self::PhysicalDeviceComputeShaderDerivativesFeaturesNV(ext)
    }
}
#[cfg(feature = "VK_NV_compute_shader_derivatives")]
impl TryInto<PhysicalDeviceComputeShaderDerivativesFeaturesNV> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceComputeShaderDerivativesFeaturesNV, Self::Error> {
        match self {
            Self::PhysicalDeviceComputeShaderDerivativesFeaturesNV(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_NV_fragment_shader_barycentric")]
impl From<PhysicalDeviceFragmentShaderBarycentricFeaturesNV> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceFragmentShaderBarycentricFeaturesNV) -> Self {
        Self::PhysicalDeviceFragmentShaderBarycentricFeaturesNV(ext)
    }
}
#[cfg(feature = "VK_NV_fragment_shader_barycentric")]
impl TryInto<PhysicalDeviceFragmentShaderBarycentricFeaturesNV> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceFragmentShaderBarycentricFeaturesNV, Self::Error> {
        match self {
            Self::PhysicalDeviceFragmentShaderBarycentricFeaturesNV(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_NV_shader_image_footprint")]
impl From<PhysicalDeviceShaderImageFootprintFeaturesNV> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceShaderImageFootprintFeaturesNV) -> Self {
        Self::PhysicalDeviceShaderImageFootprintFeaturesNV(ext)
    }
}
#[cfg(feature = "VK_NV_shader_image_footprint")]
impl TryInto<PhysicalDeviceShaderImageFootprintFeaturesNV> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceShaderImageFootprintFeaturesNV, Self::Error> {
        match self {
            Self::PhysicalDeviceShaderImageFootprintFeaturesNV(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_NV_dedicated_allocation_image_aliasing")]
impl From<PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV) -> Self {
        Self::PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV(ext)
    }
}
#[cfg(feature = "VK_NV_dedicated_allocation_image_aliasing")]
impl TryInto<PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV, Self::Error> {
        match self {
            Self::PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_NV_shading_rate_image")]
impl From<PhysicalDeviceShadingRateImageFeaturesNV> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceShadingRateImageFeaturesNV) -> Self {
        Self::PhysicalDeviceShadingRateImageFeaturesNV(ext)
    }
}
#[cfg(feature = "VK_NV_shading_rate_image")]
impl TryInto<PhysicalDeviceShadingRateImageFeaturesNV> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceShadingRateImageFeaturesNV, Self::Error> {
        match self {
            Self::PhysicalDeviceShadingRateImageFeaturesNV(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_HUAWEI_invocation_mask")]
impl From<PhysicalDeviceInvocationMaskFeaturesHUAWEI> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceInvocationMaskFeaturesHUAWEI) -> Self {
        Self::PhysicalDeviceInvocationMaskFeaturesHUAWEI(ext)
    }
}
#[cfg(feature = "VK_HUAWEI_invocation_mask")]
impl TryInto<PhysicalDeviceInvocationMaskFeaturesHUAWEI> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceInvocationMaskFeaturesHUAWEI, Self::Error> {
        match self {
            Self::PhysicalDeviceInvocationMaskFeaturesHUAWEI(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_NV_mesh_shader")]
impl From<PhysicalDeviceMeshShaderFeaturesNV> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceMeshShaderFeaturesNV) -> Self {
        Self::PhysicalDeviceMeshShaderFeaturesNV(ext)
    }
}
#[cfg(feature = "VK_NV_mesh_shader")]
impl TryInto<PhysicalDeviceMeshShaderFeaturesNV> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceMeshShaderFeaturesNV, Self::Error> {
        match self {
            Self::PhysicalDeviceMeshShaderFeaturesNV(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_KHR_acceleration_structure")]
impl From<PhysicalDeviceAccelerationStructureFeaturesKHR> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceAccelerationStructureFeaturesKHR) -> Self {
        Self::PhysicalDeviceAccelerationStructureFeaturesKHR(ext)
    }
}
#[cfg(feature = "VK_KHR_acceleration_structure")]
impl TryInto<PhysicalDeviceAccelerationStructureFeaturesKHR> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceAccelerationStructureFeaturesKHR, Self::Error> {
        match self {
            Self::PhysicalDeviceAccelerationStructureFeaturesKHR(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_KHR_ray_tracing_pipeline")]
impl From<PhysicalDeviceRayTracingPipelineFeaturesKHR> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceRayTracingPipelineFeaturesKHR) -> Self {
        Self::PhysicalDeviceRayTracingPipelineFeaturesKHR(ext)
    }
}
#[cfg(feature = "VK_KHR_ray_tracing_pipeline")]
impl TryInto<PhysicalDeviceRayTracingPipelineFeaturesKHR> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceRayTracingPipelineFeaturesKHR, Self::Error> {
        match self {
            Self::PhysicalDeviceRayTracingPipelineFeaturesKHR(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_KHR_ray_query")]
impl From<PhysicalDeviceRayQueryFeaturesKHR> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceRayQueryFeaturesKHR) -> Self {
        Self::PhysicalDeviceRayQueryFeaturesKHR(ext)
    }
}
#[cfg(feature = "VK_KHR_ray_query")]
impl TryInto<PhysicalDeviceRayQueryFeaturesKHR> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceRayQueryFeaturesKHR, Self::Error> {
        match self {
            Self::PhysicalDeviceRayQueryFeaturesKHR(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_EXT_fragment_density_map")]
impl From<PhysicalDeviceFragmentDensityMapFeaturesEXT> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceFragmentDensityMapFeaturesEXT) -> Self {
        Self::PhysicalDeviceFragmentDensityMapFeaturesEXT(ext)
    }
}
#[cfg(feature = "VK_EXT_fragment_density_map")]
impl TryInto<PhysicalDeviceFragmentDensityMapFeaturesEXT> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceFragmentDensityMapFeaturesEXT, Self::Error> {
        match self {
            Self::PhysicalDeviceFragmentDensityMapFeaturesEXT(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_EXT_fragment_density_map2")]
impl From<PhysicalDeviceFragmentDensityMap2FeaturesEXT> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceFragmentDensityMap2FeaturesEXT) -> Self {
        Self::PhysicalDeviceFragmentDensityMap2FeaturesEXT(ext)
    }
}
#[cfg(feature = "VK_EXT_fragment_density_map2")]
impl TryInto<PhysicalDeviceFragmentDensityMap2FeaturesEXT> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceFragmentDensityMap2FeaturesEXT, Self::Error> {
        match self {
            Self::PhysicalDeviceFragmentDensityMap2FeaturesEXT(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VULKAN_1_2")]
impl From<PhysicalDeviceScalarBlockLayoutFeatures> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceScalarBlockLayoutFeatures) -> Self {
        Self::PhysicalDeviceScalarBlockLayoutFeatures(ext)
    }
}
#[cfg(feature = "VULKAN_1_2")]
impl TryInto<PhysicalDeviceScalarBlockLayoutFeatures> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceScalarBlockLayoutFeatures, Self::Error> {
        match self {
            Self::PhysicalDeviceScalarBlockLayoutFeatures(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VULKAN_1_2")]
impl From<PhysicalDeviceUniformBufferStandardLayoutFeatures> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceUniformBufferStandardLayoutFeatures) -> Self {
        Self::PhysicalDeviceUniformBufferStandardLayoutFeatures(ext)
    }
}
#[cfg(feature = "VULKAN_1_2")]
impl TryInto<PhysicalDeviceUniformBufferStandardLayoutFeatures> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceUniformBufferStandardLayoutFeatures, Self::Error> {
        match self {
            Self::PhysicalDeviceUniformBufferStandardLayoutFeatures(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_EXT_depth_clip_enable")]
impl From<PhysicalDeviceDepthClipEnableFeaturesEXT> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceDepthClipEnableFeaturesEXT) -> Self {
        Self::PhysicalDeviceDepthClipEnableFeaturesEXT(ext)
    }
}
#[cfg(feature = "VK_EXT_depth_clip_enable")]
impl TryInto<PhysicalDeviceDepthClipEnableFeaturesEXT> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceDepthClipEnableFeaturesEXT, Self::Error> {
        match self {
            Self::PhysicalDeviceDepthClipEnableFeaturesEXT(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_EXT_memory_priority")]
impl From<PhysicalDeviceMemoryPriorityFeaturesEXT> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceMemoryPriorityFeaturesEXT) -> Self {
        Self::PhysicalDeviceMemoryPriorityFeaturesEXT(ext)
    }
}
#[cfg(feature = "VK_EXT_memory_priority")]
impl TryInto<PhysicalDeviceMemoryPriorityFeaturesEXT> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceMemoryPriorityFeaturesEXT, Self::Error> {
        match self {
            Self::PhysicalDeviceMemoryPriorityFeaturesEXT(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_EXT_pageable_device_local_memory")]
impl From<PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT) -> Self {
        Self::PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT(ext)
    }
}
#[cfg(feature = "VK_EXT_pageable_device_local_memory")]
impl TryInto<PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT, Self::Error> {
        match self {
            Self::PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VULKAN_1_2")]
impl From<PhysicalDeviceBufferDeviceAddressFeatures> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceBufferDeviceAddressFeatures) -> Self {
        Self::PhysicalDeviceBufferDeviceAddressFeatures(ext)
    }
}
#[cfg(feature = "VULKAN_1_2")]
impl TryInto<PhysicalDeviceBufferDeviceAddressFeatures> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceBufferDeviceAddressFeatures, Self::Error> {
        match self {
            Self::PhysicalDeviceBufferDeviceAddressFeatures(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_EXT_buffer_device_address")]
impl From<PhysicalDeviceBufferDeviceAddressFeaturesEXT> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceBufferDeviceAddressFeaturesEXT) -> Self {
        Self::PhysicalDeviceBufferDeviceAddressFeaturesEXT(ext)
    }
}
#[cfg(feature = "VK_EXT_buffer_device_address")]
impl TryInto<PhysicalDeviceBufferDeviceAddressFeaturesEXT> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceBufferDeviceAddressFeaturesEXT, Self::Error> {
        match self {
            Self::PhysicalDeviceBufferDeviceAddressFeaturesEXT(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VULKAN_1_2")]
impl From<PhysicalDeviceImagelessFramebufferFeatures> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceImagelessFramebufferFeatures) -> Self {
        Self::PhysicalDeviceImagelessFramebufferFeatures(ext)
    }
}
#[cfg(feature = "VULKAN_1_2")]
impl TryInto<PhysicalDeviceImagelessFramebufferFeatures> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceImagelessFramebufferFeatures, Self::Error> {
        match self {
            Self::PhysicalDeviceImagelessFramebufferFeatures(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VULKAN_1_3")]
impl From<PhysicalDeviceTextureCompressionAstchdrFeatures> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceTextureCompressionAstchdrFeatures) -> Self {
        Self::PhysicalDeviceTextureCompressionAstchdrFeatures(ext)
    }
}
#[cfg(feature = "VULKAN_1_3")]
impl TryInto<PhysicalDeviceTextureCompressionAstchdrFeatures> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceTextureCompressionAstchdrFeatures, Self::Error> {
        match self {
            Self::PhysicalDeviceTextureCompressionAstchdrFeatures(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_NV_cooperative_matrix")]
impl From<PhysicalDeviceCooperativeMatrixFeaturesNV> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceCooperativeMatrixFeaturesNV) -> Self {
        Self::PhysicalDeviceCooperativeMatrixFeaturesNV(ext)
    }
}
#[cfg(feature = "VK_NV_cooperative_matrix")]
impl TryInto<PhysicalDeviceCooperativeMatrixFeaturesNV> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceCooperativeMatrixFeaturesNV, Self::Error> {
        match self {
            Self::PhysicalDeviceCooperativeMatrixFeaturesNV(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_EXT_ycbcr_image_arrays")]
impl From<PhysicalDeviceYcbcrImageArraysFeaturesEXT> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceYcbcrImageArraysFeaturesEXT) -> Self {
        Self::PhysicalDeviceYcbcrImageArraysFeaturesEXT(ext)
    }
}
#[cfg(feature = "VK_EXT_ycbcr_image_arrays")]
impl TryInto<PhysicalDeviceYcbcrImageArraysFeaturesEXT> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceYcbcrImageArraysFeaturesEXT, Self::Error> {
        match self {
            Self::PhysicalDeviceYcbcrImageArraysFeaturesEXT(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_KHR_performance_query")]
impl From<PhysicalDevicePerformanceQueryFeaturesKHR> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDevicePerformanceQueryFeaturesKHR) -> Self {
        Self::PhysicalDevicePerformanceQueryFeaturesKHR(ext)
    }
}
#[cfg(feature = "VK_KHR_performance_query")]
impl TryInto<PhysicalDevicePerformanceQueryFeaturesKHR> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDevicePerformanceQueryFeaturesKHR, Self::Error> {
        match self {
            Self::PhysicalDevicePerformanceQueryFeaturesKHR(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_NV_coverage_reduction_mode")]
impl From<PhysicalDeviceCoverageReductionModeFeaturesNV> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceCoverageReductionModeFeaturesNV) -> Self {
        Self::PhysicalDeviceCoverageReductionModeFeaturesNV(ext)
    }
}
#[cfg(feature = "VK_NV_coverage_reduction_mode")]
impl TryInto<PhysicalDeviceCoverageReductionModeFeaturesNV> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceCoverageReductionModeFeaturesNV, Self::Error> {
        match self {
            Self::PhysicalDeviceCoverageReductionModeFeaturesNV(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_INTEL_shader_integer_functions2")]
impl From<PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL) -> Self {
        Self::PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL(ext)
    }
}
#[cfg(feature = "VK_INTEL_shader_integer_functions2")]
impl TryInto<PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL, Self::Error> {
        match self {
            Self::PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_KHR_shader_clock")]
impl From<PhysicalDeviceShaderClockFeaturesKHR> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceShaderClockFeaturesKHR) -> Self {
        Self::PhysicalDeviceShaderClockFeaturesKHR(ext)
    }
}
#[cfg(feature = "VK_KHR_shader_clock")]
impl TryInto<PhysicalDeviceShaderClockFeaturesKHR> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceShaderClockFeaturesKHR, Self::Error> {
        match self {
            Self::PhysicalDeviceShaderClockFeaturesKHR(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_EXT_index_type_uint8")]
impl From<PhysicalDeviceIndexTypeUint8FeaturesEXT> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceIndexTypeUint8FeaturesEXT) -> Self {
        Self::PhysicalDeviceIndexTypeUint8FeaturesEXT(ext)
    }
}
#[cfg(feature = "VK_EXT_index_type_uint8")]
impl TryInto<PhysicalDeviceIndexTypeUint8FeaturesEXT> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceIndexTypeUint8FeaturesEXT, Self::Error> {
        match self {
            Self::PhysicalDeviceIndexTypeUint8FeaturesEXT(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_NV_shader_sm_builtins")]
impl From<PhysicalDeviceShaderSmBuiltinsFeaturesNV> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceShaderSmBuiltinsFeaturesNV) -> Self {
        Self::PhysicalDeviceShaderSmBuiltinsFeaturesNV(ext)
    }
}
#[cfg(feature = "VK_NV_shader_sm_builtins")]
impl TryInto<PhysicalDeviceShaderSmBuiltinsFeaturesNV> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceShaderSmBuiltinsFeaturesNV, Self::Error> {
        match self {
            Self::PhysicalDeviceShaderSmBuiltinsFeaturesNV(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_EXT_fragment_shader_interlock")]
impl From<PhysicalDeviceFragmentShaderInterlockFeaturesEXT> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceFragmentShaderInterlockFeaturesEXT) -> Self {
        Self::PhysicalDeviceFragmentShaderInterlockFeaturesEXT(ext)
    }
}
#[cfg(feature = "VK_EXT_fragment_shader_interlock")]
impl TryInto<PhysicalDeviceFragmentShaderInterlockFeaturesEXT> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceFragmentShaderInterlockFeaturesEXT, Self::Error> {
        match self {
            Self::PhysicalDeviceFragmentShaderInterlockFeaturesEXT(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VULKAN_1_2")]
impl From<PhysicalDeviceSeparateDepthStencilLayoutsFeatures> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceSeparateDepthStencilLayoutsFeatures) -> Self {
        Self::PhysicalDeviceSeparateDepthStencilLayoutsFeatures(ext)
    }
}
#[cfg(feature = "VULKAN_1_2")]
impl TryInto<PhysicalDeviceSeparateDepthStencilLayoutsFeatures> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceSeparateDepthStencilLayoutsFeatures, Self::Error> {
        match self {
            Self::PhysicalDeviceSeparateDepthStencilLayoutsFeatures(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_EXT_primitive_topology_list_restart")]
impl From<PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT) -> Self {
        Self::PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT(ext)
    }
}
#[cfg(feature = "VK_EXT_primitive_topology_list_restart")]
impl TryInto<PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT, Self::Error> {
        match self {
            Self::PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_KHR_pipeline_executable_properties")]
impl From<PhysicalDevicePipelineExecutablePropertiesFeaturesKHR> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDevicePipelineExecutablePropertiesFeaturesKHR) -> Self {
        Self::PhysicalDevicePipelineExecutablePropertiesFeaturesKHR(ext)
    }
}
#[cfg(feature = "VK_KHR_pipeline_executable_properties")]
impl TryInto<PhysicalDevicePipelineExecutablePropertiesFeaturesKHR> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDevicePipelineExecutablePropertiesFeaturesKHR, Self::Error> {
        match self {
            Self::PhysicalDevicePipelineExecutablePropertiesFeaturesKHR(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VULKAN_1_3")]
impl From<PhysicalDeviceShaderDemoteToHelperInvocationFeatures> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceShaderDemoteToHelperInvocationFeatures) -> Self {
        Self::PhysicalDeviceShaderDemoteToHelperInvocationFeatures(ext)
    }
}
#[cfg(feature = "VULKAN_1_3")]
impl TryInto<PhysicalDeviceShaderDemoteToHelperInvocationFeatures> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceShaderDemoteToHelperInvocationFeatures, Self::Error> {
        match self {
            Self::PhysicalDeviceShaderDemoteToHelperInvocationFeatures(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_EXT_texel_buffer_alignment")]
impl From<PhysicalDeviceTexelBufferAlignmentFeaturesEXT> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceTexelBufferAlignmentFeaturesEXT) -> Self {
        Self::PhysicalDeviceTexelBufferAlignmentFeaturesEXT(ext)
    }
}
#[cfg(feature = "VK_EXT_texel_buffer_alignment")]
impl TryInto<PhysicalDeviceTexelBufferAlignmentFeaturesEXT> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceTexelBufferAlignmentFeaturesEXT, Self::Error> {
        match self {
            Self::PhysicalDeviceTexelBufferAlignmentFeaturesEXT(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VULKAN_1_3")]
impl From<PhysicalDeviceSubgroupSizeControlFeatures> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceSubgroupSizeControlFeatures) -> Self {
        Self::PhysicalDeviceSubgroupSizeControlFeatures(ext)
    }
}
#[cfg(feature = "VULKAN_1_3")]
impl TryInto<PhysicalDeviceSubgroupSizeControlFeatures> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceSubgroupSizeControlFeatures, Self::Error> {
        match self {
            Self::PhysicalDeviceSubgroupSizeControlFeatures(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_EXT_line_rasterization")]
impl From<PhysicalDeviceLineRasterizationFeaturesEXT> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceLineRasterizationFeaturesEXT) -> Self {
        Self::PhysicalDeviceLineRasterizationFeaturesEXT(ext)
    }
}
#[cfg(feature = "VK_EXT_line_rasterization")]
impl TryInto<PhysicalDeviceLineRasterizationFeaturesEXT> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceLineRasterizationFeaturesEXT, Self::Error> {
        match self {
            Self::PhysicalDeviceLineRasterizationFeaturesEXT(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VULKAN_1_3")]
impl From<PhysicalDevicePipelineCreationCacheControlFeatures> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDevicePipelineCreationCacheControlFeatures) -> Self {
        Self::PhysicalDevicePipelineCreationCacheControlFeatures(ext)
    }
}
#[cfg(feature = "VULKAN_1_3")]
impl TryInto<PhysicalDevicePipelineCreationCacheControlFeatures> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDevicePipelineCreationCacheControlFeatures, Self::Error> {
        match self {
            Self::PhysicalDevicePipelineCreationCacheControlFeatures(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VULKAN_1_2")]
impl From<PhysicalDeviceVulkan11Features> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceVulkan11Features) -> Self {
        Self::PhysicalDeviceVulkan11Features(ext)
    }
}
#[cfg(feature = "VULKAN_1_2")]
impl TryInto<PhysicalDeviceVulkan11Features> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceVulkan11Features, Self::Error> {
        match self {
            Self::PhysicalDeviceVulkan11Features(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VULKAN_1_2")]
impl From<PhysicalDeviceVulkan12Features> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceVulkan12Features) -> Self {
        Self::PhysicalDeviceVulkan12Features(ext)
    }
}
#[cfg(feature = "VULKAN_1_2")]
impl TryInto<PhysicalDeviceVulkan12Features> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceVulkan12Features, Self::Error> {
        match self {
            Self::PhysicalDeviceVulkan12Features(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VULKAN_1_3")]
impl From<PhysicalDeviceVulkan13Features> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceVulkan13Features) -> Self {
        Self::PhysicalDeviceVulkan13Features(ext)
    }
}
#[cfg(feature = "VULKAN_1_3")]
impl TryInto<PhysicalDeviceVulkan13Features> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceVulkan13Features, Self::Error> {
        match self {
            Self::PhysicalDeviceVulkan13Features(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_AMD_device_coherent_memory")]
impl From<PhysicalDeviceCoherentMemoryFeaturesAMD> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceCoherentMemoryFeaturesAMD) -> Self {
        Self::PhysicalDeviceCoherentMemoryFeaturesAMD(ext)
    }
}
#[cfg(feature = "VK_AMD_device_coherent_memory")]
impl TryInto<PhysicalDeviceCoherentMemoryFeaturesAMD> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceCoherentMemoryFeaturesAMD, Self::Error> {
        match self {
            Self::PhysicalDeviceCoherentMemoryFeaturesAMD(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_EXT_custom_border_color")]
impl From<PhysicalDeviceCustomBorderColorFeaturesEXT> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceCustomBorderColorFeaturesEXT) -> Self {
        Self::PhysicalDeviceCustomBorderColorFeaturesEXT(ext)
    }
}
#[cfg(feature = "VK_EXT_custom_border_color")]
impl TryInto<PhysicalDeviceCustomBorderColorFeaturesEXT> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceCustomBorderColorFeaturesEXT, Self::Error> {
        match self {
            Self::PhysicalDeviceCustomBorderColorFeaturesEXT(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_EXT_border_color_swizzle")]
impl From<PhysicalDeviceBorderColorSwizzleFeaturesEXT> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceBorderColorSwizzleFeaturesEXT) -> Self {
        Self::PhysicalDeviceBorderColorSwizzleFeaturesEXT(ext)
    }
}
#[cfg(feature = "VK_EXT_border_color_swizzle")]
impl TryInto<PhysicalDeviceBorderColorSwizzleFeaturesEXT> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceBorderColorSwizzleFeaturesEXT, Self::Error> {
        match self {
            Self::PhysicalDeviceBorderColorSwizzleFeaturesEXT(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_EXT_extended_dynamic_state")]
impl From<PhysicalDeviceExtendedDynamicStateFeaturesEXT> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceExtendedDynamicStateFeaturesEXT) -> Self {
        Self::PhysicalDeviceExtendedDynamicStateFeaturesEXT(ext)
    }
}
#[cfg(feature = "VK_EXT_extended_dynamic_state")]
impl TryInto<PhysicalDeviceExtendedDynamicStateFeaturesEXT> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceExtendedDynamicStateFeaturesEXT, Self::Error> {
        match self {
            Self::PhysicalDeviceExtendedDynamicStateFeaturesEXT(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_EXT_extended_dynamic_state2")]
impl From<PhysicalDeviceExtendedDynamicState2FeaturesEXT> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceExtendedDynamicState2FeaturesEXT) -> Self {
        Self::PhysicalDeviceExtendedDynamicState2FeaturesEXT(ext)
    }
}
#[cfg(feature = "VK_EXT_extended_dynamic_state2")]
impl TryInto<PhysicalDeviceExtendedDynamicState2FeaturesEXT> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceExtendedDynamicState2FeaturesEXT, Self::Error> {
        match self {
            Self::PhysicalDeviceExtendedDynamicState2FeaturesEXT(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_NV_device_diagnostics_config")]
impl From<PhysicalDeviceDiagnosticsConfigFeaturesNV> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceDiagnosticsConfigFeaturesNV) -> Self {
        Self::PhysicalDeviceDiagnosticsConfigFeaturesNV(ext)
    }
}
#[cfg(feature = "VK_NV_device_diagnostics_config")]
impl TryInto<PhysicalDeviceDiagnosticsConfigFeaturesNV> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceDiagnosticsConfigFeaturesNV, Self::Error> {
        match self {
            Self::PhysicalDeviceDiagnosticsConfigFeaturesNV(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VULKAN_1_3")]
impl From<PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures) -> Self {
        Self::PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures(ext)
    }
}
#[cfg(feature = "VULKAN_1_3")]
impl TryInto<PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures, Self::Error> {
        match self {
            Self::PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_KHR_shader_subgroup_uniform_control_flow")]
impl From<PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR) -> Self {
        Self::PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR(ext)
    }
}
#[cfg(feature = "VK_KHR_shader_subgroup_uniform_control_flow")]
impl TryInto<PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR, Self::Error> {
        match self {
            Self::PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_EXT_robustness2")]
impl From<PhysicalDeviceRobustness2FeaturesEXT> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceRobustness2FeaturesEXT) -> Self {
        Self::PhysicalDeviceRobustness2FeaturesEXT(ext)
    }
}
#[cfg(feature = "VK_EXT_robustness2")]
impl TryInto<PhysicalDeviceRobustness2FeaturesEXT> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceRobustness2FeaturesEXT, Self::Error> {
        match self {
            Self::PhysicalDeviceRobustness2FeaturesEXT(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VULKAN_1_3")]
impl From<PhysicalDeviceImageRobustnessFeatures> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceImageRobustnessFeatures) -> Self {
        Self::PhysicalDeviceImageRobustnessFeatures(ext)
    }
}
#[cfg(feature = "VULKAN_1_3")]
impl TryInto<PhysicalDeviceImageRobustnessFeatures> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceImageRobustnessFeatures, Self::Error> {
        match self {
            Self::PhysicalDeviceImageRobustnessFeatures(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_KHR_workgroup_memory_explicit_layout")]
impl From<PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR) -> Self {
        Self::PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR(ext)
    }
}
#[cfg(feature = "VK_KHR_workgroup_memory_explicit_layout")]
impl TryInto<PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR, Self::Error> {
        match self {
            Self::PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_KHR_portability_subset")]
impl From<PhysicalDevicePortabilitySubsetFeaturesKHR> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDevicePortabilitySubsetFeaturesKHR) -> Self {
        Self::PhysicalDevicePortabilitySubsetFeaturesKHR(ext)
    }
}
#[cfg(feature = "VK_KHR_portability_subset")]
impl TryInto<PhysicalDevicePortabilitySubsetFeaturesKHR> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDevicePortabilitySubsetFeaturesKHR, Self::Error> {
        match self {
            Self::PhysicalDevicePortabilitySubsetFeaturesKHR(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_EXT_4444_formats")]
impl From<PhysicalDevice4444FormatsFeaturesEXT> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDevice4444FormatsFeaturesEXT) -> Self {
        Self::PhysicalDevice4444FormatsFeaturesEXT(ext)
    }
}
#[cfg(feature = "VK_EXT_4444_formats")]
impl TryInto<PhysicalDevice4444FormatsFeaturesEXT> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDevice4444FormatsFeaturesEXT, Self::Error> {
        match self {
            Self::PhysicalDevice4444FormatsFeaturesEXT(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_HUAWEI_subpass_shading")]
impl From<PhysicalDeviceSubpassShadingFeaturesHUAWEI> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceSubpassShadingFeaturesHUAWEI) -> Self {
        Self::PhysicalDeviceSubpassShadingFeaturesHUAWEI(ext)
    }
}
#[cfg(feature = "VK_HUAWEI_subpass_shading")]
impl TryInto<PhysicalDeviceSubpassShadingFeaturesHUAWEI> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceSubpassShadingFeaturesHUAWEI, Self::Error> {
        match self {
            Self::PhysicalDeviceSubpassShadingFeaturesHUAWEI(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_EXT_shader_image_atomic_int64")]
impl From<PhysicalDeviceShaderImageAtomicInt64FeaturesEXT> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceShaderImageAtomicInt64FeaturesEXT) -> Self {
        Self::PhysicalDeviceShaderImageAtomicInt64FeaturesEXT(ext)
    }
}
#[cfg(feature = "VK_EXT_shader_image_atomic_int64")]
impl TryInto<PhysicalDeviceShaderImageAtomicInt64FeaturesEXT> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceShaderImageAtomicInt64FeaturesEXT, Self::Error> {
        match self {
            Self::PhysicalDeviceShaderImageAtomicInt64FeaturesEXT(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_KHR_fragment_shading_rate")]
impl From<PhysicalDeviceFragmentShadingRateFeaturesKHR> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceFragmentShadingRateFeaturesKHR) -> Self {
        Self::PhysicalDeviceFragmentShadingRateFeaturesKHR(ext)
    }
}
#[cfg(feature = "VK_KHR_fragment_shading_rate")]
impl TryInto<PhysicalDeviceFragmentShadingRateFeaturesKHR> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceFragmentShadingRateFeaturesKHR, Self::Error> {
        match self {
            Self::PhysicalDeviceFragmentShadingRateFeaturesKHR(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VULKAN_1_3")]
impl From<PhysicalDeviceShaderTerminateInvocationFeatures> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceShaderTerminateInvocationFeatures) -> Self {
        Self::PhysicalDeviceShaderTerminateInvocationFeatures(ext)
    }
}
#[cfg(feature = "VULKAN_1_3")]
impl TryInto<PhysicalDeviceShaderTerminateInvocationFeatures> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceShaderTerminateInvocationFeatures, Self::Error> {
        match self {
            Self::PhysicalDeviceShaderTerminateInvocationFeatures(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_NV_fragment_shading_rate_enums")]
impl From<PhysicalDeviceFragmentShadingRateEnumsFeaturesNV> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceFragmentShadingRateEnumsFeaturesNV) -> Self {
        Self::PhysicalDeviceFragmentShadingRateEnumsFeaturesNV(ext)
    }
}
#[cfg(feature = "VK_NV_fragment_shading_rate_enums")]
impl TryInto<PhysicalDeviceFragmentShadingRateEnumsFeaturesNV> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceFragmentShadingRateEnumsFeaturesNV, Self::Error> {
        match self {
            Self::PhysicalDeviceFragmentShadingRateEnumsFeaturesNV(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_VALVE_mutable_descriptor_type")]
impl From<PhysicalDeviceMutableDescriptorTypeFeaturesVALVE> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceMutableDescriptorTypeFeaturesVALVE) -> Self {
        Self::PhysicalDeviceMutableDescriptorTypeFeaturesVALVE(ext)
    }
}
#[cfg(feature = "VK_VALVE_mutable_descriptor_type")]
impl TryInto<PhysicalDeviceMutableDescriptorTypeFeaturesVALVE> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceMutableDescriptorTypeFeaturesVALVE, Self::Error> {
        match self {
            Self::PhysicalDeviceMutableDescriptorTypeFeaturesVALVE(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_EXT_depth_clip_control")]
impl From<PhysicalDeviceDepthClipControlFeaturesEXT> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceDepthClipControlFeaturesEXT) -> Self {
        Self::PhysicalDeviceDepthClipControlFeaturesEXT(ext)
    }
}
#[cfg(feature = "VK_EXT_depth_clip_control")]
impl TryInto<PhysicalDeviceDepthClipControlFeaturesEXT> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceDepthClipControlFeaturesEXT, Self::Error> {
        match self {
            Self::PhysicalDeviceDepthClipControlFeaturesEXT(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_EXT_vertex_input_dynamic_state")]
impl From<PhysicalDeviceVertexInputDynamicStateFeaturesEXT> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceVertexInputDynamicStateFeaturesEXT) -> Self {
        Self::PhysicalDeviceVertexInputDynamicStateFeaturesEXT(ext)
    }
}
#[cfg(feature = "VK_EXT_vertex_input_dynamic_state")]
impl TryInto<PhysicalDeviceVertexInputDynamicStateFeaturesEXT> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceVertexInputDynamicStateFeaturesEXT, Self::Error> {
        match self {
            Self::PhysicalDeviceVertexInputDynamicStateFeaturesEXT(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_NV_external_memory_rdma")]
impl From<PhysicalDeviceExternalMemoryRdmaFeaturesNV> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceExternalMemoryRdmaFeaturesNV) -> Self {
        Self::PhysicalDeviceExternalMemoryRdmaFeaturesNV(ext)
    }
}
#[cfg(feature = "VK_NV_external_memory_rdma")]
impl TryInto<PhysicalDeviceExternalMemoryRdmaFeaturesNV> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceExternalMemoryRdmaFeaturesNV, Self::Error> {
        match self {
            Self::PhysicalDeviceExternalMemoryRdmaFeaturesNV(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_EXT_color_write_enable")]
impl From<PhysicalDeviceColorWriteEnableFeaturesEXT> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceColorWriteEnableFeaturesEXT) -> Self {
        Self::PhysicalDeviceColorWriteEnableFeaturesEXT(ext)
    }
}
#[cfg(feature = "VK_EXT_color_write_enable")]
impl TryInto<PhysicalDeviceColorWriteEnableFeaturesEXT> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceColorWriteEnableFeaturesEXT, Self::Error> {
        match self {
            Self::PhysicalDeviceColorWriteEnableFeaturesEXT(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VULKAN_1_3")]
impl From<PhysicalDeviceSynchronization2Features> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceSynchronization2Features) -> Self {
        Self::PhysicalDeviceSynchronization2Features(ext)
    }
}
#[cfg(feature = "VULKAN_1_3")]
impl TryInto<PhysicalDeviceSynchronization2Features> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceSynchronization2Features, Self::Error> {
        match self {
            Self::PhysicalDeviceSynchronization2Features(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_NV_inherited_viewport_scissor")]
impl From<PhysicalDeviceInheritedViewportScissorFeaturesNV> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceInheritedViewportScissorFeaturesNV) -> Self {
        Self::PhysicalDeviceInheritedViewportScissorFeaturesNV(ext)
    }
}
#[cfg(feature = "VK_NV_inherited_viewport_scissor")]
impl TryInto<PhysicalDeviceInheritedViewportScissorFeaturesNV> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceInheritedViewportScissorFeaturesNV, Self::Error> {
        match self {
            Self::PhysicalDeviceInheritedViewportScissorFeaturesNV(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_EXT_ycbcr_2plane_444_formats")]
impl From<PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT) -> Self {
        Self::PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT(ext)
    }
}
#[cfg(feature = "VK_EXT_ycbcr_2plane_444_formats")]
impl TryInto<PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT, Self::Error> {
        match self {
            Self::PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_EXT_provoking_vertex")]
impl From<PhysicalDeviceProvokingVertexFeaturesEXT> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceProvokingVertexFeaturesEXT) -> Self {
        Self::PhysicalDeviceProvokingVertexFeaturesEXT(ext)
    }
}
#[cfg(feature = "VK_EXT_provoking_vertex")]
impl TryInto<PhysicalDeviceProvokingVertexFeaturesEXT> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceProvokingVertexFeaturesEXT, Self::Error> {
        match self {
            Self::PhysicalDeviceProvokingVertexFeaturesEXT(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VULKAN_1_3")]
impl From<PhysicalDeviceShaderIntegerDotProductFeatures> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceShaderIntegerDotProductFeatures) -> Self {
        Self::PhysicalDeviceShaderIntegerDotProductFeatures(ext)
    }
}
#[cfg(feature = "VULKAN_1_3")]
impl TryInto<PhysicalDeviceShaderIntegerDotProductFeatures> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceShaderIntegerDotProductFeatures, Self::Error> {
        match self {
            Self::PhysicalDeviceShaderIntegerDotProductFeatures(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_NV_ray_tracing_motion_blur")]
impl From<PhysicalDeviceRayTracingMotionBlurFeaturesNV> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceRayTracingMotionBlurFeaturesNV) -> Self {
        Self::PhysicalDeviceRayTracingMotionBlurFeaturesNV(ext)
    }
}
#[cfg(feature = "VK_NV_ray_tracing_motion_blur")]
impl TryInto<PhysicalDeviceRayTracingMotionBlurFeaturesNV> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceRayTracingMotionBlurFeaturesNV, Self::Error> {
        match self {
            Self::PhysicalDeviceRayTracingMotionBlurFeaturesNV(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_EXT_rgba10x6_formats")]
impl From<PhysicalDeviceRgba10x6FormatsFeaturesEXT> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceRgba10x6FormatsFeaturesEXT) -> Self {
        Self::PhysicalDeviceRgba10x6FormatsFeaturesEXT(ext)
    }
}
#[cfg(feature = "VK_EXT_rgba10x6_formats")]
impl TryInto<PhysicalDeviceRgba10x6FormatsFeaturesEXT> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceRgba10x6FormatsFeaturesEXT, Self::Error> {
        match self {
            Self::PhysicalDeviceRgba10x6FormatsFeaturesEXT(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VULKAN_1_3")]
impl From<PhysicalDeviceDynamicRenderingFeatures> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceDynamicRenderingFeatures) -> Self {
        Self::PhysicalDeviceDynamicRenderingFeatures(ext)
    }
}
#[cfg(feature = "VULKAN_1_3")]
impl TryInto<PhysicalDeviceDynamicRenderingFeatures> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceDynamicRenderingFeatures, Self::Error> {
        match self {
            Self::PhysicalDeviceDynamicRenderingFeatures(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_EXT_image_view_min_lod")]
impl From<PhysicalDeviceImageViewMinLodFeaturesEXT> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceImageViewMinLodFeaturesEXT) -> Self {
        Self::PhysicalDeviceImageViewMinLodFeaturesEXT(ext)
    }
}
#[cfg(feature = "VK_EXT_image_view_min_lod")]
impl TryInto<PhysicalDeviceImageViewMinLodFeaturesEXT> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceImageViewMinLodFeaturesEXT, Self::Error> {
        match self {
            Self::PhysicalDeviceImageViewMinLodFeaturesEXT(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_ARM_rasterization_order_attachment_access")]
impl From<PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM) -> Self {
        Self::PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM(ext)
    }
}
#[cfg(feature = "VK_ARM_rasterization_order_attachment_access")]
impl TryInto<PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM, Self::Error> {
        match self {
            Self::PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_NV_linear_color_attachment")]
impl From<PhysicalDeviceLinearColorAttachmentFeaturesNV> for PhysicalDeviceFeatures2Extension {
    fn from(ext: PhysicalDeviceLinearColorAttachmentFeaturesNV) -> Self {
        Self::PhysicalDeviceLinearColorAttachmentFeaturesNV(ext)
    }
}
#[cfg(feature = "VK_NV_linear_color_attachment")]
impl TryInto<PhysicalDeviceLinearColorAttachmentFeaturesNV> for PhysicalDeviceFeatures2Extension {
    type Error = PhysicalDeviceFeatures2Extension;
    fn try_into(self) -> Result<PhysicalDeviceLinearColorAttachmentFeaturesNV, Self::Error> {
        match self {
            Self::PhysicalDeviceLinearColorAttachmentFeaturesNV(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[doc(alias = "VkPhysicalDeviceProperties2")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceProperties2 {
    #[doc(alias = "pNext")]
    pub extensions: SmallVec<[PhysicalDeviceProperties2Extension; 1]>,
    pub properties: PhysicalDeviceProperties,
}
impl PhysicalDeviceProperties2 {
    ///Adds an extension to the struct
    pub fn with_extension(mut self, ext: impl Into<PhysicalDeviceProperties2Extension>) -> Self {
        self.extensions.push(ext.into());
        self
    }
    ///Get a reference to the `extensions` field.
    pub fn extensions(&self) -> &SmallVec<[PhysicalDeviceProperties2Extension; 1]> {
        &self.extensions
    }
    ///Get a reference to the `properties` field.
    pub fn properties(&self) -> &PhysicalDeviceProperties {
        &self.properties
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceProperties2 {
    type LowLevel = crate::native::vulkan1_1::PhysicalDeviceProperties2;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let mut next = std::ptr::null_mut();
        let mut extensions = self.extensions.iter();
        while let Some(ext) = extensions.next() {
            let ext = ext.into_low_level(context, bump);
            (*ext).next = next;
            next = ext;
        }
        crate::native::vulkan1_1::PhysicalDeviceProperties2 {
            s_type: StructureType::PhysicalDeviceProperties2,
            p_next: next,
            properties: self.properties.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceProperties2 {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let mut next = value.p_next;
        let mut extensions = SmallVec::new();
        while !next.is_null() {
            extensions.push(crate::conv::FromLowLevel::from_low_level(context, next));
            next = std::ptr::read(next).next;
        }
        Self {
            extensions: extensions,
            properties: crate::conv::FromLowLevel::from_low_level(context, value.properties),
        }
    }
}
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
///Extensions for [`PhysicalDeviceProperties2`]
pub enum PhysicalDeviceProperties2Extension {
    #[cfg(feature = "VK_NV_device_generated_commands")]
    ///Contains a type [`PhysicalDeviceDeviceGeneratedCommandsPropertiesNV`] for extending
    /// [`PhysicalDeviceProperties2`]
    PhysicalDeviceDeviceGeneratedCommandsPropertiesNV(PhysicalDeviceDeviceGeneratedCommandsPropertiesNV),
    #[cfg(feature = "VK_EXT_multi_draw")]
    ///Contains a type [`PhysicalDeviceMultiDrawPropertiesEXT`] for extending
    /// [`PhysicalDeviceProperties2`]
    PhysicalDeviceMultiDrawPropertiesEXT(PhysicalDeviceMultiDrawPropertiesEXT),
    #[cfg(feature = "VK_KHR_push_descriptor")]
    ///Contains a type [`PhysicalDevicePushDescriptorPropertiesKHR`] for extending
    /// [`PhysicalDeviceProperties2`]
    PhysicalDevicePushDescriptorPropertiesKHR(PhysicalDevicePushDescriptorPropertiesKHR),
    #[cfg(feature = "VULKAN_1_2")]
    ///Contains a type [`PhysicalDeviceDriverProperties`] for extending
    /// [`PhysicalDeviceProperties2`]
    PhysicalDeviceDriverProperties(PhysicalDeviceDriverProperties),
    ///Contains a type [`PhysicalDeviceIdProperties`] for extending [`PhysicalDeviceProperties2`]
    PhysicalDeviceIdProperties(PhysicalDeviceIdProperties),
    ///Contains a type [`PhysicalDeviceMultiviewProperties`] for extending
    /// [`PhysicalDeviceProperties2`]
    PhysicalDeviceMultiviewProperties(PhysicalDeviceMultiviewProperties),
    #[cfg(feature = "VK_EXT_discard_rectangles")]
    ///Contains a type [`PhysicalDeviceDiscardRectanglePropertiesEXT`] for extending
    /// [`PhysicalDeviceProperties2`]
    PhysicalDeviceDiscardRectanglePropertiesEXT(PhysicalDeviceDiscardRectanglePropertiesEXT),
    #[cfg(feature = "VK_NVX_multiview_per_view_attributes")]
    ///Contains a type [`PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX`] for extending
    /// [`PhysicalDeviceProperties2`]
    PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX(PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX),
    ///Contains a type [`PhysicalDeviceSubgroupProperties`] for extending
    /// [`PhysicalDeviceProperties2`]
    PhysicalDeviceSubgroupProperties(PhysicalDeviceSubgroupProperties),
    ///Contains a type [`PhysicalDevicePointClippingProperties`] for extending
    /// [`PhysicalDeviceProperties2`]
    PhysicalDevicePointClippingProperties(PhysicalDevicePointClippingProperties),
    ///Contains a type [`PhysicalDeviceProtectedMemoryProperties`] for extending
    /// [`PhysicalDeviceProperties2`]
    PhysicalDeviceProtectedMemoryProperties(PhysicalDeviceProtectedMemoryProperties),
    #[cfg(feature = "VULKAN_1_2")]
    ///Contains a type [`PhysicalDeviceSamplerFilterMinmaxProperties`] for extending
    /// [`PhysicalDeviceProperties2`]
    PhysicalDeviceSamplerFilterMinmaxProperties(PhysicalDeviceSamplerFilterMinmaxProperties),
    #[cfg(feature = "VK_EXT_sample_locations")]
    ///Contains a type [`PhysicalDeviceSampleLocationsPropertiesEXT`] for extending
    /// [`PhysicalDeviceProperties2`]
    PhysicalDeviceSampleLocationsPropertiesEXT(PhysicalDeviceSampleLocationsPropertiesEXT),
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    ///Contains a type [`PhysicalDeviceBlendOperationAdvancedPropertiesEXT`] for extending
    /// [`PhysicalDeviceProperties2`]
    PhysicalDeviceBlendOperationAdvancedPropertiesEXT(PhysicalDeviceBlendOperationAdvancedPropertiesEXT),
    #[cfg(feature = "VULKAN_1_3")]
    ///Contains a type [`PhysicalDeviceInlineUniformBlockProperties`] for extending
    /// [`PhysicalDeviceProperties2`]
    PhysicalDeviceInlineUniformBlockProperties(PhysicalDeviceInlineUniformBlockProperties),
    ///Contains a type [`PhysicalDeviceMaintenance3Properties`] for extending
    /// [`PhysicalDeviceProperties2`]
    PhysicalDeviceMaintenance3Properties(PhysicalDeviceMaintenance3Properties),
    #[cfg(feature = "VULKAN_1_3")]
    ///Contains a type [`PhysicalDeviceMaintenance4Properties`] for extending
    /// [`PhysicalDeviceProperties2`]
    PhysicalDeviceMaintenance4Properties(PhysicalDeviceMaintenance4Properties),
    #[cfg(feature = "VULKAN_1_2")]
    ///Contains a type [`PhysicalDeviceFloatControlsProperties`] for extending
    /// [`PhysicalDeviceProperties2`]
    PhysicalDeviceFloatControlsProperties(PhysicalDeviceFloatControlsProperties),
    #[cfg(feature = "VK_EXT_external_memory_host")]
    ///Contains a type [`PhysicalDeviceExternalMemoryHostPropertiesEXT`] for extending
    /// [`PhysicalDeviceProperties2`]
    PhysicalDeviceExternalMemoryHostPropertiesEXT(PhysicalDeviceExternalMemoryHostPropertiesEXT),
    #[cfg(feature = "VK_EXT_conservative_rasterization")]
    ///Contains a type [`PhysicalDeviceConservativeRasterizationPropertiesEXT`] for extending
    /// [`PhysicalDeviceProperties2`]
    PhysicalDeviceConservativeRasterizationPropertiesEXT(PhysicalDeviceConservativeRasterizationPropertiesEXT),
    #[cfg(feature = "VK_AMD_shader_core_properties")]
    ///Contains a type [`PhysicalDeviceShaderCorePropertiesAMD`] for extending
    /// [`PhysicalDeviceProperties2`]
    PhysicalDeviceShaderCorePropertiesAMD(PhysicalDeviceShaderCorePropertiesAMD),
    #[cfg(feature = "VK_AMD_shader_core_properties2")]
    ///Contains a type [`PhysicalDeviceShaderCoreProperties2AMD`] for extending
    /// [`PhysicalDeviceProperties2`]
    PhysicalDeviceShaderCoreProperties2AMD(PhysicalDeviceShaderCoreProperties2AMD),
    #[cfg(feature = "VULKAN_1_2")]
    ///Contains a type [`PhysicalDeviceDescriptorIndexingProperties`] for extending
    /// [`PhysicalDeviceProperties2`]
    PhysicalDeviceDescriptorIndexingProperties(PhysicalDeviceDescriptorIndexingProperties),
    #[cfg(feature = "VULKAN_1_2")]
    ///Contains a type [`PhysicalDeviceTimelineSemaphoreProperties`] for extending
    /// [`PhysicalDeviceProperties2`]
    PhysicalDeviceTimelineSemaphoreProperties(PhysicalDeviceTimelineSemaphoreProperties),
    #[cfg(feature = "VK_EXT_vertex_attribute_divisor")]
    ///Contains a type [`PhysicalDeviceVertexAttributeDivisorPropertiesEXT`] for extending
    /// [`PhysicalDeviceProperties2`]
    PhysicalDeviceVertexAttributeDivisorPropertiesEXT(PhysicalDeviceVertexAttributeDivisorPropertiesEXT),
    #[cfg(feature = "VK_EXT_pci_bus_info")]
    ///Contains a type [`PhysicalDevicePciBusInfoPropertiesEXT`] for extending
    /// [`PhysicalDeviceProperties2`]
    PhysicalDevicePciBusInfoPropertiesEXT(PhysicalDevicePciBusInfoPropertiesEXT),
    #[cfg(feature = "VULKAN_1_2")]
    ///Contains a type [`PhysicalDeviceDepthStencilResolveProperties`] for extending
    /// [`PhysicalDeviceProperties2`]
    PhysicalDeviceDepthStencilResolveProperties(PhysicalDeviceDepthStencilResolveProperties),
    #[cfg(feature = "VK_EXT_transform_feedback")]
    ///Contains a type [`PhysicalDeviceTransformFeedbackPropertiesEXT`] for extending
    /// [`PhysicalDeviceProperties2`]
    PhysicalDeviceTransformFeedbackPropertiesEXT(PhysicalDeviceTransformFeedbackPropertiesEXT),
    #[cfg(feature = "VK_NV_shading_rate_image")]
    ///Contains a type [`PhysicalDeviceShadingRateImagePropertiesNV`] for extending
    /// [`PhysicalDeviceProperties2`]
    PhysicalDeviceShadingRateImagePropertiesNV(PhysicalDeviceShadingRateImagePropertiesNV),
    #[cfg(feature = "VK_NV_mesh_shader")]
    ///Contains a type [`PhysicalDeviceMeshShaderPropertiesNV`] for extending
    /// [`PhysicalDeviceProperties2`]
    PhysicalDeviceMeshShaderPropertiesNV(PhysicalDeviceMeshShaderPropertiesNV),
    #[cfg(feature = "VK_KHR_acceleration_structure")]
    ///Contains a type [`PhysicalDeviceAccelerationStructurePropertiesKHR`] for extending
    /// [`PhysicalDeviceProperties2`]
    PhysicalDeviceAccelerationStructurePropertiesKHR(PhysicalDeviceAccelerationStructurePropertiesKHR),
    #[cfg(feature = "VK_KHR_ray_tracing_pipeline")]
    ///Contains a type [`PhysicalDeviceRayTracingPipelinePropertiesKHR`] for extending
    /// [`PhysicalDeviceProperties2`]
    PhysicalDeviceRayTracingPipelinePropertiesKHR(PhysicalDeviceRayTracingPipelinePropertiesKHR),
    #[cfg(feature = "VK_NV_ray_tracing")]
    ///Contains a type [`PhysicalDeviceRayTracingPropertiesNV`] for extending
    /// [`PhysicalDeviceProperties2`]
    PhysicalDeviceRayTracingPropertiesNV(PhysicalDeviceRayTracingPropertiesNV),
    #[cfg(feature = "VK_EXT_fragment_density_map")]
    ///Contains a type [`PhysicalDeviceFragmentDensityMapPropertiesEXT`] for extending
    /// [`PhysicalDeviceProperties2`]
    PhysicalDeviceFragmentDensityMapPropertiesEXT(PhysicalDeviceFragmentDensityMapPropertiesEXT),
    #[cfg(feature = "VK_EXT_fragment_density_map2")]
    ///Contains a type [`PhysicalDeviceFragmentDensityMap2PropertiesEXT`] for extending
    /// [`PhysicalDeviceProperties2`]
    PhysicalDeviceFragmentDensityMap2PropertiesEXT(PhysicalDeviceFragmentDensityMap2PropertiesEXT),
    #[cfg(feature = "VK_NV_cooperative_matrix")]
    ///Contains a type [`PhysicalDeviceCooperativeMatrixPropertiesNV`] for extending
    /// [`PhysicalDeviceProperties2`]
    PhysicalDeviceCooperativeMatrixPropertiesNV(PhysicalDeviceCooperativeMatrixPropertiesNV),
    #[cfg(feature = "VK_KHR_performance_query")]
    ///Contains a type [`PhysicalDevicePerformanceQueryPropertiesKHR`] for extending
    /// [`PhysicalDeviceProperties2`]
    PhysicalDevicePerformanceQueryPropertiesKHR(PhysicalDevicePerformanceQueryPropertiesKHR),
    #[cfg(feature = "VK_NV_shader_sm_builtins")]
    ///Contains a type [`PhysicalDeviceShaderSmBuiltinsPropertiesNV`] for extending
    /// [`PhysicalDeviceProperties2`]
    PhysicalDeviceShaderSmBuiltinsPropertiesNV(PhysicalDeviceShaderSmBuiltinsPropertiesNV),
    #[cfg(feature = "VULKAN_1_3")]
    ///Contains a type [`PhysicalDeviceTexelBufferAlignmentProperties`] for extending
    /// [`PhysicalDeviceProperties2`]
    PhysicalDeviceTexelBufferAlignmentProperties(PhysicalDeviceTexelBufferAlignmentProperties),
    #[cfg(feature = "VULKAN_1_3")]
    ///Contains a type [`PhysicalDeviceSubgroupSizeControlProperties`] for extending
    /// [`PhysicalDeviceProperties2`]
    PhysicalDeviceSubgroupSizeControlProperties(PhysicalDeviceSubgroupSizeControlProperties),
    #[cfg(feature = "VK_HUAWEI_subpass_shading")]
    ///Contains a type [`PhysicalDeviceSubpassShadingPropertiesHUAWEI`] for extending
    /// [`PhysicalDeviceProperties2`]
    PhysicalDeviceSubpassShadingPropertiesHUAWEI(PhysicalDeviceSubpassShadingPropertiesHUAWEI),
    #[cfg(feature = "VK_EXT_line_rasterization")]
    ///Contains a type [`PhysicalDeviceLineRasterizationPropertiesEXT`] for extending
    /// [`PhysicalDeviceProperties2`]
    PhysicalDeviceLineRasterizationPropertiesEXT(PhysicalDeviceLineRasterizationPropertiesEXT),
    #[cfg(feature = "VULKAN_1_2")]
    ///Contains a type [`PhysicalDeviceVulkan11Properties`] for extending
    /// [`PhysicalDeviceProperties2`]
    PhysicalDeviceVulkan11Properties(PhysicalDeviceVulkan11Properties),
    #[cfg(feature = "VULKAN_1_2")]
    ///Contains a type [`PhysicalDeviceVulkan12Properties`] for extending
    /// [`PhysicalDeviceProperties2`]
    PhysicalDeviceVulkan12Properties(PhysicalDeviceVulkan12Properties),
    #[cfg(feature = "VULKAN_1_3")]
    ///Contains a type [`PhysicalDeviceVulkan13Properties`] for extending
    /// [`PhysicalDeviceProperties2`]
    PhysicalDeviceVulkan13Properties(PhysicalDeviceVulkan13Properties),
    #[cfg(feature = "VK_EXT_custom_border_color")]
    ///Contains a type [`PhysicalDeviceCustomBorderColorPropertiesEXT`] for extending
    /// [`PhysicalDeviceProperties2`]
    PhysicalDeviceCustomBorderColorPropertiesEXT(PhysicalDeviceCustomBorderColorPropertiesEXT),
    #[cfg(feature = "VK_EXT_robustness2")]
    ///Contains a type [`PhysicalDeviceRobustness2PropertiesEXT`] for extending
    /// [`PhysicalDeviceProperties2`]
    PhysicalDeviceRobustness2PropertiesEXT(PhysicalDeviceRobustness2PropertiesEXT),
    #[cfg(feature = "VK_KHR_portability_subset")]
    ///Contains a type [`PhysicalDevicePortabilitySubsetPropertiesKHR`] for extending
    /// [`PhysicalDeviceProperties2`]
    PhysicalDevicePortabilitySubsetPropertiesKHR(PhysicalDevicePortabilitySubsetPropertiesKHR),
    #[cfg(feature = "VK_KHR_fragment_shading_rate")]
    ///Contains a type [`PhysicalDeviceFragmentShadingRatePropertiesKHR`] for extending
    /// [`PhysicalDeviceProperties2`]
    PhysicalDeviceFragmentShadingRatePropertiesKHR(PhysicalDeviceFragmentShadingRatePropertiesKHR),
    #[cfg(feature = "VK_NV_fragment_shading_rate_enums")]
    ///Contains a type [`PhysicalDeviceFragmentShadingRateEnumsPropertiesNV`] for extending
    /// [`PhysicalDeviceProperties2`]
    PhysicalDeviceFragmentShadingRateEnumsPropertiesNV(PhysicalDeviceFragmentShadingRateEnumsPropertiesNV),
    #[cfg(feature = "VK_EXT_provoking_vertex")]
    ///Contains a type [`PhysicalDeviceProvokingVertexPropertiesEXT`] for extending
    /// [`PhysicalDeviceProperties2`]
    PhysicalDeviceProvokingVertexPropertiesEXT(PhysicalDeviceProvokingVertexPropertiesEXT),
    #[cfg(feature = "VULKAN_1_3")]
    ///Contains a type [`PhysicalDeviceShaderIntegerDotProductProperties`] for extending
    /// [`PhysicalDeviceProperties2`]
    PhysicalDeviceShaderIntegerDotProductProperties(PhysicalDeviceShaderIntegerDotProductProperties),
    #[cfg(feature = "VK_EXT_physical_device_drm")]
    ///Contains a type [`PhysicalDeviceDrmPropertiesEXT`] for extending
    /// [`PhysicalDeviceProperties2`]
    PhysicalDeviceDrmPropertiesEXT(PhysicalDeviceDrmPropertiesEXT),
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceProperties2Extension {
    type LowLevel = *mut crate::native::vulkan1_0::BaseOutStructure;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        match self { # [cfg (feature = "VK_NV_device_generated_commands")] Self :: PhysicalDeviceDeviceGeneratedCommandsPropertiesNV (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: nv_device_generated_commands :: PhysicalDeviceDeviceGeneratedCommandsPropertiesNV) . cast () , # [cfg (feature = "VK_EXT_multi_draw")] Self :: PhysicalDeviceMultiDrawPropertiesEXT (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: ext_multi_draw :: PhysicalDeviceMultiDrawPropertiesEXT) . cast () , # [cfg (feature = "VK_KHR_push_descriptor")] Self :: PhysicalDevicePushDescriptorPropertiesKHR (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: khr_push_descriptor :: PhysicalDevicePushDescriptorPropertiesKHR) . cast () , # [cfg (feature = "VULKAN_1_2")] Self :: PhysicalDeviceDriverProperties (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: vulkan1_2 :: PhysicalDeviceDriverProperties) . cast () , Self :: PhysicalDeviceIdProperties (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: vulkan1_1 :: PhysicalDeviceIdProperties) . cast () , Self :: PhysicalDeviceMultiviewProperties (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: vulkan1_1 :: PhysicalDeviceMultiviewProperties) . cast () , # [cfg (feature = "VK_EXT_discard_rectangles")] Self :: PhysicalDeviceDiscardRectanglePropertiesEXT (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: ext_discard_rectangles :: PhysicalDeviceDiscardRectanglePropertiesEXT) . cast () , # [cfg (feature = "VK_NVX_multiview_per_view_attributes")] Self :: PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: nvx_multiview_per_view_attributes :: PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX) . cast () , Self :: PhysicalDeviceSubgroupProperties (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: vulkan1_1 :: PhysicalDeviceSubgroupProperties) . cast () , Self :: PhysicalDevicePointClippingProperties (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: vulkan1_1 :: PhysicalDevicePointClippingProperties) . cast () , Self :: PhysicalDeviceProtectedMemoryProperties (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: vulkan1_1 :: PhysicalDeviceProtectedMemoryProperties) . cast () , # [cfg (feature = "VULKAN_1_2")] Self :: PhysicalDeviceSamplerFilterMinmaxProperties (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: vulkan1_2 :: PhysicalDeviceSamplerFilterMinmaxProperties) . cast () , # [cfg (feature = "VK_EXT_sample_locations")] Self :: PhysicalDeviceSampleLocationsPropertiesEXT (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: ext_sample_locations :: PhysicalDeviceSampleLocationsPropertiesEXT) . cast () , # [cfg (feature = "VK_EXT_blend_operation_advanced")] Self :: PhysicalDeviceBlendOperationAdvancedPropertiesEXT (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: ext_blend_operation_advanced :: PhysicalDeviceBlendOperationAdvancedPropertiesEXT) . cast () , # [cfg (feature = "VULKAN_1_3")] Self :: PhysicalDeviceInlineUniformBlockProperties (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: vulkan1_3 :: PhysicalDeviceInlineUniformBlockProperties) . cast () , Self :: PhysicalDeviceMaintenance3Properties (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: vulkan1_1 :: PhysicalDeviceMaintenance3Properties) . cast () , # [cfg (feature = "VULKAN_1_3")] Self :: PhysicalDeviceMaintenance4Properties (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: vulkan1_3 :: PhysicalDeviceMaintenance4Properties) . cast () , # [cfg (feature = "VULKAN_1_2")] Self :: PhysicalDeviceFloatControlsProperties (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: vulkan1_2 :: PhysicalDeviceFloatControlsProperties) . cast () , # [cfg (feature = "VK_EXT_external_memory_host")] Self :: PhysicalDeviceExternalMemoryHostPropertiesEXT (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: ext_external_memory_host :: PhysicalDeviceExternalMemoryHostPropertiesEXT) . cast () , # [cfg (feature = "VK_EXT_conservative_rasterization")] Self :: PhysicalDeviceConservativeRasterizationPropertiesEXT (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: ext_conservative_rasterization :: PhysicalDeviceConservativeRasterizationPropertiesEXT) . cast () , # [cfg (feature = "VK_AMD_shader_core_properties")] Self :: PhysicalDeviceShaderCorePropertiesAMD (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: amd_shader_core_properties :: PhysicalDeviceShaderCorePropertiesAMD) . cast () , # [cfg (feature = "VK_AMD_shader_core_properties2")] Self :: PhysicalDeviceShaderCoreProperties2AMD (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: amd_shader_core_properties2 :: PhysicalDeviceShaderCoreProperties2AMD) . cast () , # [cfg (feature = "VULKAN_1_2")] Self :: PhysicalDeviceDescriptorIndexingProperties (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: vulkan1_2 :: PhysicalDeviceDescriptorIndexingProperties) . cast () , # [cfg (feature = "VULKAN_1_2")] Self :: PhysicalDeviceTimelineSemaphoreProperties (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: vulkan1_2 :: PhysicalDeviceTimelineSemaphoreProperties) . cast () , # [cfg (feature = "VK_EXT_vertex_attribute_divisor")] Self :: PhysicalDeviceVertexAttributeDivisorPropertiesEXT (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: ext_vertex_attribute_divisor :: PhysicalDeviceVertexAttributeDivisorPropertiesEXT) . cast () , # [cfg (feature = "VK_EXT_pci_bus_info")] Self :: PhysicalDevicePciBusInfoPropertiesEXT (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: ext_pci_bus_info :: PhysicalDevicePciBusInfoPropertiesEXT) . cast () , # [cfg (feature = "VULKAN_1_2")] Self :: PhysicalDeviceDepthStencilResolveProperties (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: vulkan1_2 :: PhysicalDeviceDepthStencilResolveProperties) . cast () , # [cfg (feature = "VK_EXT_transform_feedback")] Self :: PhysicalDeviceTransformFeedbackPropertiesEXT (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: ext_transform_feedback :: PhysicalDeviceTransformFeedbackPropertiesEXT) . cast () , # [cfg (feature = "VK_NV_shading_rate_image")] Self :: PhysicalDeviceShadingRateImagePropertiesNV (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: nv_shading_rate_image :: PhysicalDeviceShadingRateImagePropertiesNV) . cast () , # [cfg (feature = "VK_NV_mesh_shader")] Self :: PhysicalDeviceMeshShaderPropertiesNV (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: nv_mesh_shader :: PhysicalDeviceMeshShaderPropertiesNV) . cast () , # [cfg (feature = "VK_KHR_acceleration_structure")] Self :: PhysicalDeviceAccelerationStructurePropertiesKHR (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: khr_acceleration_structure :: PhysicalDeviceAccelerationStructurePropertiesKHR) . cast () , # [cfg (feature = "VK_KHR_ray_tracing_pipeline")] Self :: PhysicalDeviceRayTracingPipelinePropertiesKHR (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: khr_ray_tracing_pipeline :: PhysicalDeviceRayTracingPipelinePropertiesKHR) . cast () , # [cfg (feature = "VK_NV_ray_tracing")] Self :: PhysicalDeviceRayTracingPropertiesNV (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: nv_ray_tracing :: PhysicalDeviceRayTracingPropertiesNV) . cast () , # [cfg (feature = "VK_EXT_fragment_density_map")] Self :: PhysicalDeviceFragmentDensityMapPropertiesEXT (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: ext_fragment_density_map :: PhysicalDeviceFragmentDensityMapPropertiesEXT) . cast () , # [cfg (feature = "VK_EXT_fragment_density_map2")] Self :: PhysicalDeviceFragmentDensityMap2PropertiesEXT (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: ext_fragment_density_map2 :: PhysicalDeviceFragmentDensityMap2PropertiesEXT) . cast () , # [cfg (feature = "VK_NV_cooperative_matrix")] Self :: PhysicalDeviceCooperativeMatrixPropertiesNV (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: nv_cooperative_matrix :: PhysicalDeviceCooperativeMatrixPropertiesNV) . cast () , # [cfg (feature = "VK_KHR_performance_query")] Self :: PhysicalDevicePerformanceQueryPropertiesKHR (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: khr_performance_query :: PhysicalDevicePerformanceQueryPropertiesKHR) . cast () , # [cfg (feature = "VK_NV_shader_sm_builtins")] Self :: PhysicalDeviceShaderSmBuiltinsPropertiesNV (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: nv_shader_sm_builtins :: PhysicalDeviceShaderSmBuiltinsPropertiesNV) . cast () , # [cfg (feature = "VULKAN_1_3")] Self :: PhysicalDeviceTexelBufferAlignmentProperties (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: vulkan1_3 :: PhysicalDeviceTexelBufferAlignmentProperties) . cast () , # [cfg (feature = "VULKAN_1_3")] Self :: PhysicalDeviceSubgroupSizeControlProperties (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: vulkan1_3 :: PhysicalDeviceSubgroupSizeControlProperties) . cast () , # [cfg (feature = "VK_HUAWEI_subpass_shading")] Self :: PhysicalDeviceSubpassShadingPropertiesHUAWEI (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: huawei_subpass_shading :: PhysicalDeviceSubpassShadingPropertiesHUAWEI) . cast () , # [cfg (feature = "VK_EXT_line_rasterization")] Self :: PhysicalDeviceLineRasterizationPropertiesEXT (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: ext_line_rasterization :: PhysicalDeviceLineRasterizationPropertiesEXT) . cast () , # [cfg (feature = "VULKAN_1_2")] Self :: PhysicalDeviceVulkan11Properties (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: vulkan1_2 :: PhysicalDeviceVulkan11Properties) . cast () , # [cfg (feature = "VULKAN_1_2")] Self :: PhysicalDeviceVulkan12Properties (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: vulkan1_2 :: PhysicalDeviceVulkan12Properties) . cast () , # [cfg (feature = "VULKAN_1_3")] Self :: PhysicalDeviceVulkan13Properties (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: vulkan1_3 :: PhysicalDeviceVulkan13Properties) . cast () , # [cfg (feature = "VK_EXT_custom_border_color")] Self :: PhysicalDeviceCustomBorderColorPropertiesEXT (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: ext_custom_border_color :: PhysicalDeviceCustomBorderColorPropertiesEXT) . cast () , # [cfg (feature = "VK_EXT_robustness2")] Self :: PhysicalDeviceRobustness2PropertiesEXT (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: ext_robustness2 :: PhysicalDeviceRobustness2PropertiesEXT) . cast () , # [cfg (feature = "VK_KHR_portability_subset")] Self :: PhysicalDevicePortabilitySubsetPropertiesKHR (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: khr_portability_subset :: PhysicalDevicePortabilitySubsetPropertiesKHR) . cast () , # [cfg (feature = "VK_KHR_fragment_shading_rate")] Self :: PhysicalDeviceFragmentShadingRatePropertiesKHR (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: khr_fragment_shading_rate :: PhysicalDeviceFragmentShadingRatePropertiesKHR) . cast () , # [cfg (feature = "VK_NV_fragment_shading_rate_enums")] Self :: PhysicalDeviceFragmentShadingRateEnumsPropertiesNV (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: nv_fragment_shading_rate_enums :: PhysicalDeviceFragmentShadingRateEnumsPropertiesNV) . cast () , # [cfg (feature = "VK_EXT_provoking_vertex")] Self :: PhysicalDeviceProvokingVertexPropertiesEXT (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: ext_provoking_vertex :: PhysicalDeviceProvokingVertexPropertiesEXT) . cast () , # [cfg (feature = "VULKAN_1_3")] Self :: PhysicalDeviceShaderIntegerDotProductProperties (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: vulkan1_3 :: PhysicalDeviceShaderIntegerDotProductProperties) . cast () , # [cfg (feature = "VK_EXT_physical_device_drm")] Self :: PhysicalDeviceDrmPropertiesEXT (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: ext_physical_device_drm :: PhysicalDeviceDrmPropertiesEXT) . cast () , other => unreachable ! ("unexpected variant {:?}" , other) }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceProperties2Extension {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        assert!(!value.is_null());
        match (* value) . s_type { # [cfg (feature = "VK_NV_device_generated_commands")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceDeviceGeneratedCommandsPropertiesNv => Self :: PhysicalDeviceDeviceGeneratedCommandsPropertiesNV (PhysicalDeviceDeviceGeneratedCommandsPropertiesNV :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: nv_device_generated_commands :: PhysicalDeviceDeviceGeneratedCommandsPropertiesNV > ()))) , # [cfg (feature = "VK_EXT_multi_draw")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceMultiDrawPropertiesExt => Self :: PhysicalDeviceMultiDrawPropertiesEXT (PhysicalDeviceMultiDrawPropertiesEXT :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: ext_multi_draw :: PhysicalDeviceMultiDrawPropertiesEXT > ()))) , # [cfg (feature = "VK_KHR_push_descriptor")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDevicePushDescriptorPropertiesKhr => Self :: PhysicalDevicePushDescriptorPropertiesKHR (PhysicalDevicePushDescriptorPropertiesKHR :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: khr_push_descriptor :: PhysicalDevicePushDescriptorPropertiesKHR > ()))) , # [cfg (feature = "VULKAN_1_2")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceDriverProperties => Self :: PhysicalDeviceDriverProperties (PhysicalDeviceDriverProperties :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: vulkan1_2 :: PhysicalDeviceDriverProperties > ()))) , crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceIdProperties => Self :: PhysicalDeviceIdProperties (PhysicalDeviceIdProperties :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: vulkan1_1 :: PhysicalDeviceIdProperties > ()))) , crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceMultiviewProperties => Self :: PhysicalDeviceMultiviewProperties (PhysicalDeviceMultiviewProperties :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: vulkan1_1 :: PhysicalDeviceMultiviewProperties > ()))) , # [cfg (feature = "VK_EXT_discard_rectangles")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceDiscardRectanglePropertiesExt => Self :: PhysicalDeviceDiscardRectanglePropertiesEXT (PhysicalDeviceDiscardRectanglePropertiesEXT :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: ext_discard_rectangles :: PhysicalDeviceDiscardRectanglePropertiesEXT > ()))) , # [cfg (feature = "VK_NVX_multiview_per_view_attributes")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceMultiviewPerViewAttributesPropertiesNvx => Self :: PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX (PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: nvx_multiview_per_view_attributes :: PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX > ()))) , crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceSubgroupProperties => Self :: PhysicalDeviceSubgroupProperties (PhysicalDeviceSubgroupProperties :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: vulkan1_1 :: PhysicalDeviceSubgroupProperties > ()))) , crate :: native :: vulkan1_0 :: StructureType :: PhysicalDevicePointClippingProperties => Self :: PhysicalDevicePointClippingProperties (PhysicalDevicePointClippingProperties :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: vulkan1_1 :: PhysicalDevicePointClippingProperties > ()))) , crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceProtectedMemoryProperties => Self :: PhysicalDeviceProtectedMemoryProperties (PhysicalDeviceProtectedMemoryProperties :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: vulkan1_1 :: PhysicalDeviceProtectedMemoryProperties > ()))) , # [cfg (feature = "VULKAN_1_2")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceSamplerFilterMinmaxProperties => Self :: PhysicalDeviceSamplerFilterMinmaxProperties (PhysicalDeviceSamplerFilterMinmaxProperties :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: vulkan1_2 :: PhysicalDeviceSamplerFilterMinmaxProperties > ()))) , # [cfg (feature = "VK_EXT_sample_locations")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceSampleLocationsPropertiesExt => Self :: PhysicalDeviceSampleLocationsPropertiesEXT (PhysicalDeviceSampleLocationsPropertiesEXT :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: ext_sample_locations :: PhysicalDeviceSampleLocationsPropertiesEXT > ()))) , # [cfg (feature = "VK_EXT_blend_operation_advanced")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceBlendOperationAdvancedPropertiesExt => Self :: PhysicalDeviceBlendOperationAdvancedPropertiesEXT (PhysicalDeviceBlendOperationAdvancedPropertiesEXT :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: ext_blend_operation_advanced :: PhysicalDeviceBlendOperationAdvancedPropertiesEXT > ()))) , # [cfg (feature = "VULKAN_1_3")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceInlineUniformBlockProperties => Self :: PhysicalDeviceInlineUniformBlockProperties (PhysicalDeviceInlineUniformBlockProperties :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: vulkan1_3 :: PhysicalDeviceInlineUniformBlockProperties > ()))) , crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceMaintenance3Properties => Self :: PhysicalDeviceMaintenance3Properties (PhysicalDeviceMaintenance3Properties :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: vulkan1_1 :: PhysicalDeviceMaintenance3Properties > ()))) , # [cfg (feature = "VULKAN_1_3")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceMaintenance4Properties => Self :: PhysicalDeviceMaintenance4Properties (PhysicalDeviceMaintenance4Properties :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: vulkan1_3 :: PhysicalDeviceMaintenance4Properties > ()))) , # [cfg (feature = "VULKAN_1_2")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceFloatControlsProperties => Self :: PhysicalDeviceFloatControlsProperties (PhysicalDeviceFloatControlsProperties :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: vulkan1_2 :: PhysicalDeviceFloatControlsProperties > ()))) , # [cfg (feature = "VK_EXT_external_memory_host")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceExternalMemoryHostPropertiesExt => Self :: PhysicalDeviceExternalMemoryHostPropertiesEXT (PhysicalDeviceExternalMemoryHostPropertiesEXT :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: ext_external_memory_host :: PhysicalDeviceExternalMemoryHostPropertiesEXT > ()))) , # [cfg (feature = "VK_EXT_conservative_rasterization")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceConservativeRasterizationPropertiesExt => Self :: PhysicalDeviceConservativeRasterizationPropertiesEXT (PhysicalDeviceConservativeRasterizationPropertiesEXT :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: ext_conservative_rasterization :: PhysicalDeviceConservativeRasterizationPropertiesEXT > ()))) , # [cfg (feature = "VK_AMD_shader_core_properties")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceShaderCorePropertiesAmd => Self :: PhysicalDeviceShaderCorePropertiesAMD (PhysicalDeviceShaderCorePropertiesAMD :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: amd_shader_core_properties :: PhysicalDeviceShaderCorePropertiesAMD > ()))) , # [cfg (feature = "VK_AMD_shader_core_properties2")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceShaderCoreProperties2Amd => Self :: PhysicalDeviceShaderCoreProperties2AMD (PhysicalDeviceShaderCoreProperties2AMD :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: amd_shader_core_properties2 :: PhysicalDeviceShaderCoreProperties2AMD > ()))) , # [cfg (feature = "VULKAN_1_2")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceDescriptorIndexingProperties => Self :: PhysicalDeviceDescriptorIndexingProperties (PhysicalDeviceDescriptorIndexingProperties :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: vulkan1_2 :: PhysicalDeviceDescriptorIndexingProperties > ()))) , # [cfg (feature = "VULKAN_1_2")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceTimelineSemaphoreProperties => Self :: PhysicalDeviceTimelineSemaphoreProperties (PhysicalDeviceTimelineSemaphoreProperties :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: vulkan1_2 :: PhysicalDeviceTimelineSemaphoreProperties > ()))) , # [cfg (feature = "VK_EXT_vertex_attribute_divisor")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceVertexAttributeDivisorPropertiesExt => Self :: PhysicalDeviceVertexAttributeDivisorPropertiesEXT (PhysicalDeviceVertexAttributeDivisorPropertiesEXT :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: ext_vertex_attribute_divisor :: PhysicalDeviceVertexAttributeDivisorPropertiesEXT > ()))) , # [cfg (feature = "VK_EXT_pci_bus_info")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDevicePciBusInfoPropertiesExt => Self :: PhysicalDevicePciBusInfoPropertiesEXT (PhysicalDevicePciBusInfoPropertiesEXT :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: ext_pci_bus_info :: PhysicalDevicePciBusInfoPropertiesEXT > ()))) , # [cfg (feature = "VULKAN_1_2")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceDepthStencilResolveProperties => Self :: PhysicalDeviceDepthStencilResolveProperties (PhysicalDeviceDepthStencilResolveProperties :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: vulkan1_2 :: PhysicalDeviceDepthStencilResolveProperties > ()))) , # [cfg (feature = "VK_EXT_transform_feedback")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceTransformFeedbackPropertiesExt => Self :: PhysicalDeviceTransformFeedbackPropertiesEXT (PhysicalDeviceTransformFeedbackPropertiesEXT :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: ext_transform_feedback :: PhysicalDeviceTransformFeedbackPropertiesEXT > ()))) , # [cfg (feature = "VK_NV_shading_rate_image")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceShadingRateImagePropertiesNv => Self :: PhysicalDeviceShadingRateImagePropertiesNV (PhysicalDeviceShadingRateImagePropertiesNV :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: nv_shading_rate_image :: PhysicalDeviceShadingRateImagePropertiesNV > ()))) , # [cfg (feature = "VK_NV_mesh_shader")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceMeshShaderPropertiesNv => Self :: PhysicalDeviceMeshShaderPropertiesNV (PhysicalDeviceMeshShaderPropertiesNV :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: nv_mesh_shader :: PhysicalDeviceMeshShaderPropertiesNV > ()))) , # [cfg (feature = "VK_KHR_acceleration_structure")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceAccelerationStructurePropertiesKhr => Self :: PhysicalDeviceAccelerationStructurePropertiesKHR (PhysicalDeviceAccelerationStructurePropertiesKHR :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: khr_acceleration_structure :: PhysicalDeviceAccelerationStructurePropertiesKHR > ()))) , # [cfg (feature = "VK_KHR_ray_tracing_pipeline")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceRayTracingPipelinePropertiesKhr => Self :: PhysicalDeviceRayTracingPipelinePropertiesKHR (PhysicalDeviceRayTracingPipelinePropertiesKHR :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: khr_ray_tracing_pipeline :: PhysicalDeviceRayTracingPipelinePropertiesKHR > ()))) , # [cfg (feature = "VK_NV_ray_tracing")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceRayTracingPropertiesNv => Self :: PhysicalDeviceRayTracingPropertiesNV (PhysicalDeviceRayTracingPropertiesNV :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: nv_ray_tracing :: PhysicalDeviceRayTracingPropertiesNV > ()))) , # [cfg (feature = "VK_EXT_fragment_density_map")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceFragmentDensityMapPropertiesExt => Self :: PhysicalDeviceFragmentDensityMapPropertiesEXT (PhysicalDeviceFragmentDensityMapPropertiesEXT :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: ext_fragment_density_map :: PhysicalDeviceFragmentDensityMapPropertiesEXT > ()))) , # [cfg (feature = "VK_EXT_fragment_density_map2")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceFragmentDensityMap2PropertiesExt => Self :: PhysicalDeviceFragmentDensityMap2PropertiesEXT (PhysicalDeviceFragmentDensityMap2PropertiesEXT :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: ext_fragment_density_map2 :: PhysicalDeviceFragmentDensityMap2PropertiesEXT > ()))) , # [cfg (feature = "VK_NV_cooperative_matrix")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceCooperativeMatrixPropertiesNv => Self :: PhysicalDeviceCooperativeMatrixPropertiesNV (PhysicalDeviceCooperativeMatrixPropertiesNV :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: nv_cooperative_matrix :: PhysicalDeviceCooperativeMatrixPropertiesNV > ()))) , # [cfg (feature = "VK_KHR_performance_query")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDevicePerformanceQueryPropertiesKhr => Self :: PhysicalDevicePerformanceQueryPropertiesKHR (PhysicalDevicePerformanceQueryPropertiesKHR :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: khr_performance_query :: PhysicalDevicePerformanceQueryPropertiesKHR > ()))) , # [cfg (feature = "VK_NV_shader_sm_builtins")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceShaderSmBuiltinsPropertiesNv => Self :: PhysicalDeviceShaderSmBuiltinsPropertiesNV (PhysicalDeviceShaderSmBuiltinsPropertiesNV :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: nv_shader_sm_builtins :: PhysicalDeviceShaderSmBuiltinsPropertiesNV > ()))) , # [cfg (feature = "VULKAN_1_3")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceTexelBufferAlignmentProperties => Self :: PhysicalDeviceTexelBufferAlignmentProperties (PhysicalDeviceTexelBufferAlignmentProperties :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: vulkan1_3 :: PhysicalDeviceTexelBufferAlignmentProperties > ()))) , # [cfg (feature = "VULKAN_1_3")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceSubgroupSizeControlProperties => Self :: PhysicalDeviceSubgroupSizeControlProperties (PhysicalDeviceSubgroupSizeControlProperties :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: vulkan1_3 :: PhysicalDeviceSubgroupSizeControlProperties > ()))) , # [cfg (feature = "VK_HUAWEI_subpass_shading")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceSubpassShadingPropertiesHuawei => Self :: PhysicalDeviceSubpassShadingPropertiesHUAWEI (PhysicalDeviceSubpassShadingPropertiesHUAWEI :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: huawei_subpass_shading :: PhysicalDeviceSubpassShadingPropertiesHUAWEI > ()))) , # [cfg (feature = "VK_EXT_line_rasterization")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceLineRasterizationPropertiesExt => Self :: PhysicalDeviceLineRasterizationPropertiesEXT (PhysicalDeviceLineRasterizationPropertiesEXT :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: ext_line_rasterization :: PhysicalDeviceLineRasterizationPropertiesEXT > ()))) , # [cfg (feature = "VULKAN_1_2")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceVulkan11Properties => Self :: PhysicalDeviceVulkan11Properties (PhysicalDeviceVulkan11Properties :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: vulkan1_2 :: PhysicalDeviceVulkan11Properties > ()))) , # [cfg (feature = "VULKAN_1_2")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceVulkan12Properties => Self :: PhysicalDeviceVulkan12Properties (PhysicalDeviceVulkan12Properties :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: vulkan1_2 :: PhysicalDeviceVulkan12Properties > ()))) , # [cfg (feature = "VULKAN_1_3")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceVulkan13Properties => Self :: PhysicalDeviceVulkan13Properties (PhysicalDeviceVulkan13Properties :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: vulkan1_3 :: PhysicalDeviceVulkan13Properties > ()))) , # [cfg (feature = "VK_EXT_custom_border_color")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceCustomBorderColorPropertiesExt => Self :: PhysicalDeviceCustomBorderColorPropertiesEXT (PhysicalDeviceCustomBorderColorPropertiesEXT :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: ext_custom_border_color :: PhysicalDeviceCustomBorderColorPropertiesEXT > ()))) , # [cfg (feature = "VK_EXT_robustness2")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceRobustness2PropertiesExt => Self :: PhysicalDeviceRobustness2PropertiesEXT (PhysicalDeviceRobustness2PropertiesEXT :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: ext_robustness2 :: PhysicalDeviceRobustness2PropertiesEXT > ()))) , # [cfg (feature = "VK_KHR_portability_subset")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDevicePortabilitySubsetPropertiesKhr => Self :: PhysicalDevicePortabilitySubsetPropertiesKHR (PhysicalDevicePortabilitySubsetPropertiesKHR :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: khr_portability_subset :: PhysicalDevicePortabilitySubsetPropertiesKHR > ()))) , # [cfg (feature = "VK_KHR_fragment_shading_rate")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceFragmentShadingRatePropertiesKhr => Self :: PhysicalDeviceFragmentShadingRatePropertiesKHR (PhysicalDeviceFragmentShadingRatePropertiesKHR :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: khr_fragment_shading_rate :: PhysicalDeviceFragmentShadingRatePropertiesKHR > ()))) , # [cfg (feature = "VK_NV_fragment_shading_rate_enums")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceFragmentShadingRateEnumsPropertiesNv => Self :: PhysicalDeviceFragmentShadingRateEnumsPropertiesNV (PhysicalDeviceFragmentShadingRateEnumsPropertiesNV :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: nv_fragment_shading_rate_enums :: PhysicalDeviceFragmentShadingRateEnumsPropertiesNV > ()))) , # [cfg (feature = "VK_EXT_provoking_vertex")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceProvokingVertexPropertiesExt => Self :: PhysicalDeviceProvokingVertexPropertiesEXT (PhysicalDeviceProvokingVertexPropertiesEXT :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: ext_provoking_vertex :: PhysicalDeviceProvokingVertexPropertiesEXT > ()))) , # [cfg (feature = "VULKAN_1_3")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceShaderIntegerDotProductProperties => Self :: PhysicalDeviceShaderIntegerDotProductProperties (PhysicalDeviceShaderIntegerDotProductProperties :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: vulkan1_3 :: PhysicalDeviceShaderIntegerDotProductProperties > ()))) , # [cfg (feature = "VK_EXT_physical_device_drm")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceDrmPropertiesExt => Self :: PhysicalDeviceDrmPropertiesEXT (PhysicalDeviceDrmPropertiesEXT :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: ext_physical_device_drm :: PhysicalDeviceDrmPropertiesEXT > ()))) , other => panic ! ("Structure type {:?} is not a member of {}" , other , stringify ! (PhysicalDeviceProperties2)) }
    }
}
#[cfg(feature = "VK_NV_device_generated_commands")]
impl From<PhysicalDeviceDeviceGeneratedCommandsPropertiesNV> for PhysicalDeviceProperties2Extension {
    fn from(ext: PhysicalDeviceDeviceGeneratedCommandsPropertiesNV) -> Self {
        Self::PhysicalDeviceDeviceGeneratedCommandsPropertiesNV(ext)
    }
}
#[cfg(feature = "VK_NV_device_generated_commands")]
impl TryInto<PhysicalDeviceDeviceGeneratedCommandsPropertiesNV> for PhysicalDeviceProperties2Extension {
    type Error = PhysicalDeviceProperties2Extension;
    fn try_into(self) -> Result<PhysicalDeviceDeviceGeneratedCommandsPropertiesNV, Self::Error> {
        match self {
            Self::PhysicalDeviceDeviceGeneratedCommandsPropertiesNV(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_EXT_multi_draw")]
impl From<PhysicalDeviceMultiDrawPropertiesEXT> for PhysicalDeviceProperties2Extension {
    fn from(ext: PhysicalDeviceMultiDrawPropertiesEXT) -> Self {
        Self::PhysicalDeviceMultiDrawPropertiesEXT(ext)
    }
}
#[cfg(feature = "VK_EXT_multi_draw")]
impl TryInto<PhysicalDeviceMultiDrawPropertiesEXT> for PhysicalDeviceProperties2Extension {
    type Error = PhysicalDeviceProperties2Extension;
    fn try_into(self) -> Result<PhysicalDeviceMultiDrawPropertiesEXT, Self::Error> {
        match self {
            Self::PhysicalDeviceMultiDrawPropertiesEXT(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_KHR_push_descriptor")]
impl From<PhysicalDevicePushDescriptorPropertiesKHR> for PhysicalDeviceProperties2Extension {
    fn from(ext: PhysicalDevicePushDescriptorPropertiesKHR) -> Self {
        Self::PhysicalDevicePushDescriptorPropertiesKHR(ext)
    }
}
#[cfg(feature = "VK_KHR_push_descriptor")]
impl TryInto<PhysicalDevicePushDescriptorPropertiesKHR> for PhysicalDeviceProperties2Extension {
    type Error = PhysicalDeviceProperties2Extension;
    fn try_into(self) -> Result<PhysicalDevicePushDescriptorPropertiesKHR, Self::Error> {
        match self {
            Self::PhysicalDevicePushDescriptorPropertiesKHR(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VULKAN_1_2")]
impl From<PhysicalDeviceDriverProperties> for PhysicalDeviceProperties2Extension {
    fn from(ext: PhysicalDeviceDriverProperties) -> Self {
        Self::PhysicalDeviceDriverProperties(ext)
    }
}
#[cfg(feature = "VULKAN_1_2")]
impl TryInto<PhysicalDeviceDriverProperties> for PhysicalDeviceProperties2Extension {
    type Error = PhysicalDeviceProperties2Extension;
    fn try_into(self) -> Result<PhysicalDeviceDriverProperties, Self::Error> {
        match self {
            Self::PhysicalDeviceDriverProperties(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
impl From<PhysicalDeviceIdProperties> for PhysicalDeviceProperties2Extension {
    fn from(ext: PhysicalDeviceIdProperties) -> Self {
        Self::PhysicalDeviceIdProperties(ext)
    }
}
impl TryInto<PhysicalDeviceIdProperties> for PhysicalDeviceProperties2Extension {
    type Error = PhysicalDeviceProperties2Extension;
    fn try_into(self) -> Result<PhysicalDeviceIdProperties, Self::Error> {
        match self {
            Self::PhysicalDeviceIdProperties(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
impl From<PhysicalDeviceMultiviewProperties> for PhysicalDeviceProperties2Extension {
    fn from(ext: PhysicalDeviceMultiviewProperties) -> Self {
        Self::PhysicalDeviceMultiviewProperties(ext)
    }
}
impl TryInto<PhysicalDeviceMultiviewProperties> for PhysicalDeviceProperties2Extension {
    type Error = PhysicalDeviceProperties2Extension;
    fn try_into(self) -> Result<PhysicalDeviceMultiviewProperties, Self::Error> {
        match self {
            Self::PhysicalDeviceMultiviewProperties(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_EXT_discard_rectangles")]
impl From<PhysicalDeviceDiscardRectanglePropertiesEXT> for PhysicalDeviceProperties2Extension {
    fn from(ext: PhysicalDeviceDiscardRectanglePropertiesEXT) -> Self {
        Self::PhysicalDeviceDiscardRectanglePropertiesEXT(ext)
    }
}
#[cfg(feature = "VK_EXT_discard_rectangles")]
impl TryInto<PhysicalDeviceDiscardRectanglePropertiesEXT> for PhysicalDeviceProperties2Extension {
    type Error = PhysicalDeviceProperties2Extension;
    fn try_into(self) -> Result<PhysicalDeviceDiscardRectanglePropertiesEXT, Self::Error> {
        match self {
            Self::PhysicalDeviceDiscardRectanglePropertiesEXT(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_NVX_multiview_per_view_attributes")]
impl From<PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX> for PhysicalDeviceProperties2Extension {
    fn from(ext: PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX) -> Self {
        Self::PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX(ext)
    }
}
#[cfg(feature = "VK_NVX_multiview_per_view_attributes")]
impl TryInto<PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX> for PhysicalDeviceProperties2Extension {
    type Error = PhysicalDeviceProperties2Extension;
    fn try_into(self) -> Result<PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX, Self::Error> {
        match self {
            Self::PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
impl From<PhysicalDeviceSubgroupProperties> for PhysicalDeviceProperties2Extension {
    fn from(ext: PhysicalDeviceSubgroupProperties) -> Self {
        Self::PhysicalDeviceSubgroupProperties(ext)
    }
}
impl TryInto<PhysicalDeviceSubgroupProperties> for PhysicalDeviceProperties2Extension {
    type Error = PhysicalDeviceProperties2Extension;
    fn try_into(self) -> Result<PhysicalDeviceSubgroupProperties, Self::Error> {
        match self {
            Self::PhysicalDeviceSubgroupProperties(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
impl From<PhysicalDevicePointClippingProperties> for PhysicalDeviceProperties2Extension {
    fn from(ext: PhysicalDevicePointClippingProperties) -> Self {
        Self::PhysicalDevicePointClippingProperties(ext)
    }
}
impl TryInto<PhysicalDevicePointClippingProperties> for PhysicalDeviceProperties2Extension {
    type Error = PhysicalDeviceProperties2Extension;
    fn try_into(self) -> Result<PhysicalDevicePointClippingProperties, Self::Error> {
        match self {
            Self::PhysicalDevicePointClippingProperties(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
impl From<PhysicalDeviceProtectedMemoryProperties> for PhysicalDeviceProperties2Extension {
    fn from(ext: PhysicalDeviceProtectedMemoryProperties) -> Self {
        Self::PhysicalDeviceProtectedMemoryProperties(ext)
    }
}
impl TryInto<PhysicalDeviceProtectedMemoryProperties> for PhysicalDeviceProperties2Extension {
    type Error = PhysicalDeviceProperties2Extension;
    fn try_into(self) -> Result<PhysicalDeviceProtectedMemoryProperties, Self::Error> {
        match self {
            Self::PhysicalDeviceProtectedMemoryProperties(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VULKAN_1_2")]
impl From<PhysicalDeviceSamplerFilterMinmaxProperties> for PhysicalDeviceProperties2Extension {
    fn from(ext: PhysicalDeviceSamplerFilterMinmaxProperties) -> Self {
        Self::PhysicalDeviceSamplerFilterMinmaxProperties(ext)
    }
}
#[cfg(feature = "VULKAN_1_2")]
impl TryInto<PhysicalDeviceSamplerFilterMinmaxProperties> for PhysicalDeviceProperties2Extension {
    type Error = PhysicalDeviceProperties2Extension;
    fn try_into(self) -> Result<PhysicalDeviceSamplerFilterMinmaxProperties, Self::Error> {
        match self {
            Self::PhysicalDeviceSamplerFilterMinmaxProperties(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_EXT_sample_locations")]
impl From<PhysicalDeviceSampleLocationsPropertiesEXT> for PhysicalDeviceProperties2Extension {
    fn from(ext: PhysicalDeviceSampleLocationsPropertiesEXT) -> Self {
        Self::PhysicalDeviceSampleLocationsPropertiesEXT(ext)
    }
}
#[cfg(feature = "VK_EXT_sample_locations")]
impl TryInto<PhysicalDeviceSampleLocationsPropertiesEXT> for PhysicalDeviceProperties2Extension {
    type Error = PhysicalDeviceProperties2Extension;
    fn try_into(self) -> Result<PhysicalDeviceSampleLocationsPropertiesEXT, Self::Error> {
        match self {
            Self::PhysicalDeviceSampleLocationsPropertiesEXT(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
impl From<PhysicalDeviceBlendOperationAdvancedPropertiesEXT> for PhysicalDeviceProperties2Extension {
    fn from(ext: PhysicalDeviceBlendOperationAdvancedPropertiesEXT) -> Self {
        Self::PhysicalDeviceBlendOperationAdvancedPropertiesEXT(ext)
    }
}
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
impl TryInto<PhysicalDeviceBlendOperationAdvancedPropertiesEXT> for PhysicalDeviceProperties2Extension {
    type Error = PhysicalDeviceProperties2Extension;
    fn try_into(self) -> Result<PhysicalDeviceBlendOperationAdvancedPropertiesEXT, Self::Error> {
        match self {
            Self::PhysicalDeviceBlendOperationAdvancedPropertiesEXT(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VULKAN_1_3")]
impl From<PhysicalDeviceInlineUniformBlockProperties> for PhysicalDeviceProperties2Extension {
    fn from(ext: PhysicalDeviceInlineUniformBlockProperties) -> Self {
        Self::PhysicalDeviceInlineUniformBlockProperties(ext)
    }
}
#[cfg(feature = "VULKAN_1_3")]
impl TryInto<PhysicalDeviceInlineUniformBlockProperties> for PhysicalDeviceProperties2Extension {
    type Error = PhysicalDeviceProperties2Extension;
    fn try_into(self) -> Result<PhysicalDeviceInlineUniformBlockProperties, Self::Error> {
        match self {
            Self::PhysicalDeviceInlineUniformBlockProperties(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
impl From<PhysicalDeviceMaintenance3Properties> for PhysicalDeviceProperties2Extension {
    fn from(ext: PhysicalDeviceMaintenance3Properties) -> Self {
        Self::PhysicalDeviceMaintenance3Properties(ext)
    }
}
impl TryInto<PhysicalDeviceMaintenance3Properties> for PhysicalDeviceProperties2Extension {
    type Error = PhysicalDeviceProperties2Extension;
    fn try_into(self) -> Result<PhysicalDeviceMaintenance3Properties, Self::Error> {
        match self {
            Self::PhysicalDeviceMaintenance3Properties(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VULKAN_1_3")]
impl From<PhysicalDeviceMaintenance4Properties> for PhysicalDeviceProperties2Extension {
    fn from(ext: PhysicalDeviceMaintenance4Properties) -> Self {
        Self::PhysicalDeviceMaintenance4Properties(ext)
    }
}
#[cfg(feature = "VULKAN_1_3")]
impl TryInto<PhysicalDeviceMaintenance4Properties> for PhysicalDeviceProperties2Extension {
    type Error = PhysicalDeviceProperties2Extension;
    fn try_into(self) -> Result<PhysicalDeviceMaintenance4Properties, Self::Error> {
        match self {
            Self::PhysicalDeviceMaintenance4Properties(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VULKAN_1_2")]
impl From<PhysicalDeviceFloatControlsProperties> for PhysicalDeviceProperties2Extension {
    fn from(ext: PhysicalDeviceFloatControlsProperties) -> Self {
        Self::PhysicalDeviceFloatControlsProperties(ext)
    }
}
#[cfg(feature = "VULKAN_1_2")]
impl TryInto<PhysicalDeviceFloatControlsProperties> for PhysicalDeviceProperties2Extension {
    type Error = PhysicalDeviceProperties2Extension;
    fn try_into(self) -> Result<PhysicalDeviceFloatControlsProperties, Self::Error> {
        match self {
            Self::PhysicalDeviceFloatControlsProperties(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_EXT_external_memory_host")]
impl From<PhysicalDeviceExternalMemoryHostPropertiesEXT> for PhysicalDeviceProperties2Extension {
    fn from(ext: PhysicalDeviceExternalMemoryHostPropertiesEXT) -> Self {
        Self::PhysicalDeviceExternalMemoryHostPropertiesEXT(ext)
    }
}
#[cfg(feature = "VK_EXT_external_memory_host")]
impl TryInto<PhysicalDeviceExternalMemoryHostPropertiesEXT> for PhysicalDeviceProperties2Extension {
    type Error = PhysicalDeviceProperties2Extension;
    fn try_into(self) -> Result<PhysicalDeviceExternalMemoryHostPropertiesEXT, Self::Error> {
        match self {
            Self::PhysicalDeviceExternalMemoryHostPropertiesEXT(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_EXT_conservative_rasterization")]
impl From<PhysicalDeviceConservativeRasterizationPropertiesEXT> for PhysicalDeviceProperties2Extension {
    fn from(ext: PhysicalDeviceConservativeRasterizationPropertiesEXT) -> Self {
        Self::PhysicalDeviceConservativeRasterizationPropertiesEXT(ext)
    }
}
#[cfg(feature = "VK_EXT_conservative_rasterization")]
impl TryInto<PhysicalDeviceConservativeRasterizationPropertiesEXT> for PhysicalDeviceProperties2Extension {
    type Error = PhysicalDeviceProperties2Extension;
    fn try_into(self) -> Result<PhysicalDeviceConservativeRasterizationPropertiesEXT, Self::Error> {
        match self {
            Self::PhysicalDeviceConservativeRasterizationPropertiesEXT(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_AMD_shader_core_properties")]
impl From<PhysicalDeviceShaderCorePropertiesAMD> for PhysicalDeviceProperties2Extension {
    fn from(ext: PhysicalDeviceShaderCorePropertiesAMD) -> Self {
        Self::PhysicalDeviceShaderCorePropertiesAMD(ext)
    }
}
#[cfg(feature = "VK_AMD_shader_core_properties")]
impl TryInto<PhysicalDeviceShaderCorePropertiesAMD> for PhysicalDeviceProperties2Extension {
    type Error = PhysicalDeviceProperties2Extension;
    fn try_into(self) -> Result<PhysicalDeviceShaderCorePropertiesAMD, Self::Error> {
        match self {
            Self::PhysicalDeviceShaderCorePropertiesAMD(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_AMD_shader_core_properties2")]
impl From<PhysicalDeviceShaderCoreProperties2AMD> for PhysicalDeviceProperties2Extension {
    fn from(ext: PhysicalDeviceShaderCoreProperties2AMD) -> Self {
        Self::PhysicalDeviceShaderCoreProperties2AMD(ext)
    }
}
#[cfg(feature = "VK_AMD_shader_core_properties2")]
impl TryInto<PhysicalDeviceShaderCoreProperties2AMD> for PhysicalDeviceProperties2Extension {
    type Error = PhysicalDeviceProperties2Extension;
    fn try_into(self) -> Result<PhysicalDeviceShaderCoreProperties2AMD, Self::Error> {
        match self {
            Self::PhysicalDeviceShaderCoreProperties2AMD(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VULKAN_1_2")]
impl From<PhysicalDeviceDescriptorIndexingProperties> for PhysicalDeviceProperties2Extension {
    fn from(ext: PhysicalDeviceDescriptorIndexingProperties) -> Self {
        Self::PhysicalDeviceDescriptorIndexingProperties(ext)
    }
}
#[cfg(feature = "VULKAN_1_2")]
impl TryInto<PhysicalDeviceDescriptorIndexingProperties> for PhysicalDeviceProperties2Extension {
    type Error = PhysicalDeviceProperties2Extension;
    fn try_into(self) -> Result<PhysicalDeviceDescriptorIndexingProperties, Self::Error> {
        match self {
            Self::PhysicalDeviceDescriptorIndexingProperties(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VULKAN_1_2")]
impl From<PhysicalDeviceTimelineSemaphoreProperties> for PhysicalDeviceProperties2Extension {
    fn from(ext: PhysicalDeviceTimelineSemaphoreProperties) -> Self {
        Self::PhysicalDeviceTimelineSemaphoreProperties(ext)
    }
}
#[cfg(feature = "VULKAN_1_2")]
impl TryInto<PhysicalDeviceTimelineSemaphoreProperties> for PhysicalDeviceProperties2Extension {
    type Error = PhysicalDeviceProperties2Extension;
    fn try_into(self) -> Result<PhysicalDeviceTimelineSemaphoreProperties, Self::Error> {
        match self {
            Self::PhysicalDeviceTimelineSemaphoreProperties(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_EXT_vertex_attribute_divisor")]
impl From<PhysicalDeviceVertexAttributeDivisorPropertiesEXT> for PhysicalDeviceProperties2Extension {
    fn from(ext: PhysicalDeviceVertexAttributeDivisorPropertiesEXT) -> Self {
        Self::PhysicalDeviceVertexAttributeDivisorPropertiesEXT(ext)
    }
}
#[cfg(feature = "VK_EXT_vertex_attribute_divisor")]
impl TryInto<PhysicalDeviceVertexAttributeDivisorPropertiesEXT> for PhysicalDeviceProperties2Extension {
    type Error = PhysicalDeviceProperties2Extension;
    fn try_into(self) -> Result<PhysicalDeviceVertexAttributeDivisorPropertiesEXT, Self::Error> {
        match self {
            Self::PhysicalDeviceVertexAttributeDivisorPropertiesEXT(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_EXT_pci_bus_info")]
impl From<PhysicalDevicePciBusInfoPropertiesEXT> for PhysicalDeviceProperties2Extension {
    fn from(ext: PhysicalDevicePciBusInfoPropertiesEXT) -> Self {
        Self::PhysicalDevicePciBusInfoPropertiesEXT(ext)
    }
}
#[cfg(feature = "VK_EXT_pci_bus_info")]
impl TryInto<PhysicalDevicePciBusInfoPropertiesEXT> for PhysicalDeviceProperties2Extension {
    type Error = PhysicalDeviceProperties2Extension;
    fn try_into(self) -> Result<PhysicalDevicePciBusInfoPropertiesEXT, Self::Error> {
        match self {
            Self::PhysicalDevicePciBusInfoPropertiesEXT(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VULKAN_1_2")]
impl From<PhysicalDeviceDepthStencilResolveProperties> for PhysicalDeviceProperties2Extension {
    fn from(ext: PhysicalDeviceDepthStencilResolveProperties) -> Self {
        Self::PhysicalDeviceDepthStencilResolveProperties(ext)
    }
}
#[cfg(feature = "VULKAN_1_2")]
impl TryInto<PhysicalDeviceDepthStencilResolveProperties> for PhysicalDeviceProperties2Extension {
    type Error = PhysicalDeviceProperties2Extension;
    fn try_into(self) -> Result<PhysicalDeviceDepthStencilResolveProperties, Self::Error> {
        match self {
            Self::PhysicalDeviceDepthStencilResolveProperties(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_EXT_transform_feedback")]
impl From<PhysicalDeviceTransformFeedbackPropertiesEXT> for PhysicalDeviceProperties2Extension {
    fn from(ext: PhysicalDeviceTransformFeedbackPropertiesEXT) -> Self {
        Self::PhysicalDeviceTransformFeedbackPropertiesEXT(ext)
    }
}
#[cfg(feature = "VK_EXT_transform_feedback")]
impl TryInto<PhysicalDeviceTransformFeedbackPropertiesEXT> for PhysicalDeviceProperties2Extension {
    type Error = PhysicalDeviceProperties2Extension;
    fn try_into(self) -> Result<PhysicalDeviceTransformFeedbackPropertiesEXT, Self::Error> {
        match self {
            Self::PhysicalDeviceTransformFeedbackPropertiesEXT(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_NV_shading_rate_image")]
impl From<PhysicalDeviceShadingRateImagePropertiesNV> for PhysicalDeviceProperties2Extension {
    fn from(ext: PhysicalDeviceShadingRateImagePropertiesNV) -> Self {
        Self::PhysicalDeviceShadingRateImagePropertiesNV(ext)
    }
}
#[cfg(feature = "VK_NV_shading_rate_image")]
impl TryInto<PhysicalDeviceShadingRateImagePropertiesNV> for PhysicalDeviceProperties2Extension {
    type Error = PhysicalDeviceProperties2Extension;
    fn try_into(self) -> Result<PhysicalDeviceShadingRateImagePropertiesNV, Self::Error> {
        match self {
            Self::PhysicalDeviceShadingRateImagePropertiesNV(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_NV_mesh_shader")]
impl From<PhysicalDeviceMeshShaderPropertiesNV> for PhysicalDeviceProperties2Extension {
    fn from(ext: PhysicalDeviceMeshShaderPropertiesNV) -> Self {
        Self::PhysicalDeviceMeshShaderPropertiesNV(ext)
    }
}
#[cfg(feature = "VK_NV_mesh_shader")]
impl TryInto<PhysicalDeviceMeshShaderPropertiesNV> for PhysicalDeviceProperties2Extension {
    type Error = PhysicalDeviceProperties2Extension;
    fn try_into(self) -> Result<PhysicalDeviceMeshShaderPropertiesNV, Self::Error> {
        match self {
            Self::PhysicalDeviceMeshShaderPropertiesNV(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_KHR_acceleration_structure")]
impl From<PhysicalDeviceAccelerationStructurePropertiesKHR> for PhysicalDeviceProperties2Extension {
    fn from(ext: PhysicalDeviceAccelerationStructurePropertiesKHR) -> Self {
        Self::PhysicalDeviceAccelerationStructurePropertiesKHR(ext)
    }
}
#[cfg(feature = "VK_KHR_acceleration_structure")]
impl TryInto<PhysicalDeviceAccelerationStructurePropertiesKHR> for PhysicalDeviceProperties2Extension {
    type Error = PhysicalDeviceProperties2Extension;
    fn try_into(self) -> Result<PhysicalDeviceAccelerationStructurePropertiesKHR, Self::Error> {
        match self {
            Self::PhysicalDeviceAccelerationStructurePropertiesKHR(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_KHR_ray_tracing_pipeline")]
impl From<PhysicalDeviceRayTracingPipelinePropertiesKHR> for PhysicalDeviceProperties2Extension {
    fn from(ext: PhysicalDeviceRayTracingPipelinePropertiesKHR) -> Self {
        Self::PhysicalDeviceRayTracingPipelinePropertiesKHR(ext)
    }
}
#[cfg(feature = "VK_KHR_ray_tracing_pipeline")]
impl TryInto<PhysicalDeviceRayTracingPipelinePropertiesKHR> for PhysicalDeviceProperties2Extension {
    type Error = PhysicalDeviceProperties2Extension;
    fn try_into(self) -> Result<PhysicalDeviceRayTracingPipelinePropertiesKHR, Self::Error> {
        match self {
            Self::PhysicalDeviceRayTracingPipelinePropertiesKHR(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_NV_ray_tracing")]
impl From<PhysicalDeviceRayTracingPropertiesNV> for PhysicalDeviceProperties2Extension {
    fn from(ext: PhysicalDeviceRayTracingPropertiesNV) -> Self {
        Self::PhysicalDeviceRayTracingPropertiesNV(ext)
    }
}
#[cfg(feature = "VK_NV_ray_tracing")]
impl TryInto<PhysicalDeviceRayTracingPropertiesNV> for PhysicalDeviceProperties2Extension {
    type Error = PhysicalDeviceProperties2Extension;
    fn try_into(self) -> Result<PhysicalDeviceRayTracingPropertiesNV, Self::Error> {
        match self {
            Self::PhysicalDeviceRayTracingPropertiesNV(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_EXT_fragment_density_map")]
impl From<PhysicalDeviceFragmentDensityMapPropertiesEXT> for PhysicalDeviceProperties2Extension {
    fn from(ext: PhysicalDeviceFragmentDensityMapPropertiesEXT) -> Self {
        Self::PhysicalDeviceFragmentDensityMapPropertiesEXT(ext)
    }
}
#[cfg(feature = "VK_EXT_fragment_density_map")]
impl TryInto<PhysicalDeviceFragmentDensityMapPropertiesEXT> for PhysicalDeviceProperties2Extension {
    type Error = PhysicalDeviceProperties2Extension;
    fn try_into(self) -> Result<PhysicalDeviceFragmentDensityMapPropertiesEXT, Self::Error> {
        match self {
            Self::PhysicalDeviceFragmentDensityMapPropertiesEXT(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_EXT_fragment_density_map2")]
impl From<PhysicalDeviceFragmentDensityMap2PropertiesEXT> for PhysicalDeviceProperties2Extension {
    fn from(ext: PhysicalDeviceFragmentDensityMap2PropertiesEXT) -> Self {
        Self::PhysicalDeviceFragmentDensityMap2PropertiesEXT(ext)
    }
}
#[cfg(feature = "VK_EXT_fragment_density_map2")]
impl TryInto<PhysicalDeviceFragmentDensityMap2PropertiesEXT> for PhysicalDeviceProperties2Extension {
    type Error = PhysicalDeviceProperties2Extension;
    fn try_into(self) -> Result<PhysicalDeviceFragmentDensityMap2PropertiesEXT, Self::Error> {
        match self {
            Self::PhysicalDeviceFragmentDensityMap2PropertiesEXT(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_NV_cooperative_matrix")]
impl From<PhysicalDeviceCooperativeMatrixPropertiesNV> for PhysicalDeviceProperties2Extension {
    fn from(ext: PhysicalDeviceCooperativeMatrixPropertiesNV) -> Self {
        Self::PhysicalDeviceCooperativeMatrixPropertiesNV(ext)
    }
}
#[cfg(feature = "VK_NV_cooperative_matrix")]
impl TryInto<PhysicalDeviceCooperativeMatrixPropertiesNV> for PhysicalDeviceProperties2Extension {
    type Error = PhysicalDeviceProperties2Extension;
    fn try_into(self) -> Result<PhysicalDeviceCooperativeMatrixPropertiesNV, Self::Error> {
        match self {
            Self::PhysicalDeviceCooperativeMatrixPropertiesNV(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_KHR_performance_query")]
impl From<PhysicalDevicePerformanceQueryPropertiesKHR> for PhysicalDeviceProperties2Extension {
    fn from(ext: PhysicalDevicePerformanceQueryPropertiesKHR) -> Self {
        Self::PhysicalDevicePerformanceQueryPropertiesKHR(ext)
    }
}
#[cfg(feature = "VK_KHR_performance_query")]
impl TryInto<PhysicalDevicePerformanceQueryPropertiesKHR> for PhysicalDeviceProperties2Extension {
    type Error = PhysicalDeviceProperties2Extension;
    fn try_into(self) -> Result<PhysicalDevicePerformanceQueryPropertiesKHR, Self::Error> {
        match self {
            Self::PhysicalDevicePerformanceQueryPropertiesKHR(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_NV_shader_sm_builtins")]
impl From<PhysicalDeviceShaderSmBuiltinsPropertiesNV> for PhysicalDeviceProperties2Extension {
    fn from(ext: PhysicalDeviceShaderSmBuiltinsPropertiesNV) -> Self {
        Self::PhysicalDeviceShaderSmBuiltinsPropertiesNV(ext)
    }
}
#[cfg(feature = "VK_NV_shader_sm_builtins")]
impl TryInto<PhysicalDeviceShaderSmBuiltinsPropertiesNV> for PhysicalDeviceProperties2Extension {
    type Error = PhysicalDeviceProperties2Extension;
    fn try_into(self) -> Result<PhysicalDeviceShaderSmBuiltinsPropertiesNV, Self::Error> {
        match self {
            Self::PhysicalDeviceShaderSmBuiltinsPropertiesNV(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VULKAN_1_3")]
impl From<PhysicalDeviceTexelBufferAlignmentProperties> for PhysicalDeviceProperties2Extension {
    fn from(ext: PhysicalDeviceTexelBufferAlignmentProperties) -> Self {
        Self::PhysicalDeviceTexelBufferAlignmentProperties(ext)
    }
}
#[cfg(feature = "VULKAN_1_3")]
impl TryInto<PhysicalDeviceTexelBufferAlignmentProperties> for PhysicalDeviceProperties2Extension {
    type Error = PhysicalDeviceProperties2Extension;
    fn try_into(self) -> Result<PhysicalDeviceTexelBufferAlignmentProperties, Self::Error> {
        match self {
            Self::PhysicalDeviceTexelBufferAlignmentProperties(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VULKAN_1_3")]
impl From<PhysicalDeviceSubgroupSizeControlProperties> for PhysicalDeviceProperties2Extension {
    fn from(ext: PhysicalDeviceSubgroupSizeControlProperties) -> Self {
        Self::PhysicalDeviceSubgroupSizeControlProperties(ext)
    }
}
#[cfg(feature = "VULKAN_1_3")]
impl TryInto<PhysicalDeviceSubgroupSizeControlProperties> for PhysicalDeviceProperties2Extension {
    type Error = PhysicalDeviceProperties2Extension;
    fn try_into(self) -> Result<PhysicalDeviceSubgroupSizeControlProperties, Self::Error> {
        match self {
            Self::PhysicalDeviceSubgroupSizeControlProperties(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_HUAWEI_subpass_shading")]
impl From<PhysicalDeviceSubpassShadingPropertiesHUAWEI> for PhysicalDeviceProperties2Extension {
    fn from(ext: PhysicalDeviceSubpassShadingPropertiesHUAWEI) -> Self {
        Self::PhysicalDeviceSubpassShadingPropertiesHUAWEI(ext)
    }
}
#[cfg(feature = "VK_HUAWEI_subpass_shading")]
impl TryInto<PhysicalDeviceSubpassShadingPropertiesHUAWEI> for PhysicalDeviceProperties2Extension {
    type Error = PhysicalDeviceProperties2Extension;
    fn try_into(self) -> Result<PhysicalDeviceSubpassShadingPropertiesHUAWEI, Self::Error> {
        match self {
            Self::PhysicalDeviceSubpassShadingPropertiesHUAWEI(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_EXT_line_rasterization")]
impl From<PhysicalDeviceLineRasterizationPropertiesEXT> for PhysicalDeviceProperties2Extension {
    fn from(ext: PhysicalDeviceLineRasterizationPropertiesEXT) -> Self {
        Self::PhysicalDeviceLineRasterizationPropertiesEXT(ext)
    }
}
#[cfg(feature = "VK_EXT_line_rasterization")]
impl TryInto<PhysicalDeviceLineRasterizationPropertiesEXT> for PhysicalDeviceProperties2Extension {
    type Error = PhysicalDeviceProperties2Extension;
    fn try_into(self) -> Result<PhysicalDeviceLineRasterizationPropertiesEXT, Self::Error> {
        match self {
            Self::PhysicalDeviceLineRasterizationPropertiesEXT(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VULKAN_1_2")]
impl From<PhysicalDeviceVulkan11Properties> for PhysicalDeviceProperties2Extension {
    fn from(ext: PhysicalDeviceVulkan11Properties) -> Self {
        Self::PhysicalDeviceVulkan11Properties(ext)
    }
}
#[cfg(feature = "VULKAN_1_2")]
impl TryInto<PhysicalDeviceVulkan11Properties> for PhysicalDeviceProperties2Extension {
    type Error = PhysicalDeviceProperties2Extension;
    fn try_into(self) -> Result<PhysicalDeviceVulkan11Properties, Self::Error> {
        match self {
            Self::PhysicalDeviceVulkan11Properties(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VULKAN_1_2")]
impl From<PhysicalDeviceVulkan12Properties> for PhysicalDeviceProperties2Extension {
    fn from(ext: PhysicalDeviceVulkan12Properties) -> Self {
        Self::PhysicalDeviceVulkan12Properties(ext)
    }
}
#[cfg(feature = "VULKAN_1_2")]
impl TryInto<PhysicalDeviceVulkan12Properties> for PhysicalDeviceProperties2Extension {
    type Error = PhysicalDeviceProperties2Extension;
    fn try_into(self) -> Result<PhysicalDeviceVulkan12Properties, Self::Error> {
        match self {
            Self::PhysicalDeviceVulkan12Properties(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VULKAN_1_3")]
impl From<PhysicalDeviceVulkan13Properties> for PhysicalDeviceProperties2Extension {
    fn from(ext: PhysicalDeviceVulkan13Properties) -> Self {
        Self::PhysicalDeviceVulkan13Properties(ext)
    }
}
#[cfg(feature = "VULKAN_1_3")]
impl TryInto<PhysicalDeviceVulkan13Properties> for PhysicalDeviceProperties2Extension {
    type Error = PhysicalDeviceProperties2Extension;
    fn try_into(self) -> Result<PhysicalDeviceVulkan13Properties, Self::Error> {
        match self {
            Self::PhysicalDeviceVulkan13Properties(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_EXT_custom_border_color")]
impl From<PhysicalDeviceCustomBorderColorPropertiesEXT> for PhysicalDeviceProperties2Extension {
    fn from(ext: PhysicalDeviceCustomBorderColorPropertiesEXT) -> Self {
        Self::PhysicalDeviceCustomBorderColorPropertiesEXT(ext)
    }
}
#[cfg(feature = "VK_EXT_custom_border_color")]
impl TryInto<PhysicalDeviceCustomBorderColorPropertiesEXT> for PhysicalDeviceProperties2Extension {
    type Error = PhysicalDeviceProperties2Extension;
    fn try_into(self) -> Result<PhysicalDeviceCustomBorderColorPropertiesEXT, Self::Error> {
        match self {
            Self::PhysicalDeviceCustomBorderColorPropertiesEXT(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_EXT_robustness2")]
impl From<PhysicalDeviceRobustness2PropertiesEXT> for PhysicalDeviceProperties2Extension {
    fn from(ext: PhysicalDeviceRobustness2PropertiesEXT) -> Self {
        Self::PhysicalDeviceRobustness2PropertiesEXT(ext)
    }
}
#[cfg(feature = "VK_EXT_robustness2")]
impl TryInto<PhysicalDeviceRobustness2PropertiesEXT> for PhysicalDeviceProperties2Extension {
    type Error = PhysicalDeviceProperties2Extension;
    fn try_into(self) -> Result<PhysicalDeviceRobustness2PropertiesEXT, Self::Error> {
        match self {
            Self::PhysicalDeviceRobustness2PropertiesEXT(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_KHR_portability_subset")]
impl From<PhysicalDevicePortabilitySubsetPropertiesKHR> for PhysicalDeviceProperties2Extension {
    fn from(ext: PhysicalDevicePortabilitySubsetPropertiesKHR) -> Self {
        Self::PhysicalDevicePortabilitySubsetPropertiesKHR(ext)
    }
}
#[cfg(feature = "VK_KHR_portability_subset")]
impl TryInto<PhysicalDevicePortabilitySubsetPropertiesKHR> for PhysicalDeviceProperties2Extension {
    type Error = PhysicalDeviceProperties2Extension;
    fn try_into(self) -> Result<PhysicalDevicePortabilitySubsetPropertiesKHR, Self::Error> {
        match self {
            Self::PhysicalDevicePortabilitySubsetPropertiesKHR(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_KHR_fragment_shading_rate")]
impl From<PhysicalDeviceFragmentShadingRatePropertiesKHR> for PhysicalDeviceProperties2Extension {
    fn from(ext: PhysicalDeviceFragmentShadingRatePropertiesKHR) -> Self {
        Self::PhysicalDeviceFragmentShadingRatePropertiesKHR(ext)
    }
}
#[cfg(feature = "VK_KHR_fragment_shading_rate")]
impl TryInto<PhysicalDeviceFragmentShadingRatePropertiesKHR> for PhysicalDeviceProperties2Extension {
    type Error = PhysicalDeviceProperties2Extension;
    fn try_into(self) -> Result<PhysicalDeviceFragmentShadingRatePropertiesKHR, Self::Error> {
        match self {
            Self::PhysicalDeviceFragmentShadingRatePropertiesKHR(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_NV_fragment_shading_rate_enums")]
impl From<PhysicalDeviceFragmentShadingRateEnumsPropertiesNV> for PhysicalDeviceProperties2Extension {
    fn from(ext: PhysicalDeviceFragmentShadingRateEnumsPropertiesNV) -> Self {
        Self::PhysicalDeviceFragmentShadingRateEnumsPropertiesNV(ext)
    }
}
#[cfg(feature = "VK_NV_fragment_shading_rate_enums")]
impl TryInto<PhysicalDeviceFragmentShadingRateEnumsPropertiesNV> for PhysicalDeviceProperties2Extension {
    type Error = PhysicalDeviceProperties2Extension;
    fn try_into(self) -> Result<PhysicalDeviceFragmentShadingRateEnumsPropertiesNV, Self::Error> {
        match self {
            Self::PhysicalDeviceFragmentShadingRateEnumsPropertiesNV(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_EXT_provoking_vertex")]
impl From<PhysicalDeviceProvokingVertexPropertiesEXT> for PhysicalDeviceProperties2Extension {
    fn from(ext: PhysicalDeviceProvokingVertexPropertiesEXT) -> Self {
        Self::PhysicalDeviceProvokingVertexPropertiesEXT(ext)
    }
}
#[cfg(feature = "VK_EXT_provoking_vertex")]
impl TryInto<PhysicalDeviceProvokingVertexPropertiesEXT> for PhysicalDeviceProperties2Extension {
    type Error = PhysicalDeviceProperties2Extension;
    fn try_into(self) -> Result<PhysicalDeviceProvokingVertexPropertiesEXT, Self::Error> {
        match self {
            Self::PhysicalDeviceProvokingVertexPropertiesEXT(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VULKAN_1_3")]
impl From<PhysicalDeviceShaderIntegerDotProductProperties> for PhysicalDeviceProperties2Extension {
    fn from(ext: PhysicalDeviceShaderIntegerDotProductProperties) -> Self {
        Self::PhysicalDeviceShaderIntegerDotProductProperties(ext)
    }
}
#[cfg(feature = "VULKAN_1_3")]
impl TryInto<PhysicalDeviceShaderIntegerDotProductProperties> for PhysicalDeviceProperties2Extension {
    type Error = PhysicalDeviceProperties2Extension;
    fn try_into(self) -> Result<PhysicalDeviceShaderIntegerDotProductProperties, Self::Error> {
        match self {
            Self::PhysicalDeviceShaderIntegerDotProductProperties(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_EXT_physical_device_drm")]
impl From<PhysicalDeviceDrmPropertiesEXT> for PhysicalDeviceProperties2Extension {
    fn from(ext: PhysicalDeviceDrmPropertiesEXT) -> Self {
        Self::PhysicalDeviceDrmPropertiesEXT(ext)
    }
}
#[cfg(feature = "VK_EXT_physical_device_drm")]
impl TryInto<PhysicalDeviceDrmPropertiesEXT> for PhysicalDeviceProperties2Extension {
    type Error = PhysicalDeviceProperties2Extension;
    fn try_into(self) -> Result<PhysicalDeviceDrmPropertiesEXT, Self::Error> {
        match self {
            Self::PhysicalDeviceDrmPropertiesEXT(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[doc(alias = "VkFormatProperties2")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FormatProperties2 {
    #[doc(alias = "pNext")]
    pub extensions: SmallVec<[FormatProperties2Extension; 1]>,
    #[doc(alias = "formatProperties")]
    pub format_properties: FormatProperties,
}
impl FormatProperties2 {
    ///Adds an extension to the struct
    pub fn with_extension(mut self, ext: impl Into<FormatProperties2Extension>) -> Self {
        self.extensions.push(ext.into());
        self
    }
    ///Get a reference to the `extensions` field.
    pub fn extensions(&self) -> &SmallVec<[FormatProperties2Extension; 1]> {
        &self.extensions
    }
    ///Get a reference to the `format_properties` field.
    pub fn format_properties(&self) -> FormatProperties {
        self.format_properties
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for FormatProperties2 {
    type LowLevel = crate::native::vulkan1_1::FormatProperties2;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let mut next = std::ptr::null_mut();
        let mut extensions = self.extensions.iter();
        while let Some(ext) = extensions.next() {
            let ext = ext.into_low_level(context, bump);
            (*ext).next = next;
            next = ext;
        }
        crate::native::vulkan1_1::FormatProperties2 {
            s_type: StructureType::FormatProperties2,
            p_next: next,
            format_properties: self.format_properties.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for FormatProperties2 {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let mut next = value.p_next;
        let mut extensions = SmallVec::new();
        while !next.is_null() {
            extensions.push(crate::conv::FromLowLevel::from_low_level(context, next));
            next = std::ptr::read(next).next;
        }
        Self {
            extensions: extensions,
            format_properties: crate::conv::FromLowLevel::from_low_level(context, value.format_properties),
        }
    }
}
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
///Extensions for [`FormatProperties2`]
pub enum FormatProperties2Extension {
    #[cfg(feature = "VK_EXT_image_drm_format_modifier")]
    ///Contains a type [`DrmFormatModifierPropertiesListEXT`] for extending [`FormatProperties2`]
    DrmFormatModifierPropertiesListEXT(DrmFormatModifierPropertiesListEXT),
    #[cfg(feature = "VK_KHR_video_queue")]
    ///Contains a type [`VideoProfilesKHR`] for extending [`FormatProperties2`]
    VideoProfilesKHR(VideoProfilesKHR),
    #[cfg(feature = "VK_KHR_video_queue")]
    ///Contains a type [`VideoProfileKHR`] for extending [`FormatProperties2`]
    VideoProfileKHR(VideoProfileKHR),
    #[cfg(feature = "VULKAN_1_3")]
    ///Contains a type [`FormatProperties3`] for extending [`FormatProperties2`]
    FormatProperties3(FormatProperties3),
    #[cfg(feature = "VK_EXT_image_drm_format_modifier")]
    ///Contains a type [`DrmFormatModifierPropertiesList2EXT`] for extending [`FormatProperties2`]
    DrmFormatModifierPropertiesList2EXT(DrmFormatModifierPropertiesList2EXT),
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for FormatProperties2Extension {
    type LowLevel = *mut crate::native::vulkan1_0::BaseOutStructure;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        match self {
            #[cfg(feature = "VK_EXT_image_drm_format_modifier")]
            Self::DrmFormatModifierPropertiesListEXT(ext) => (bump.alloc(ext.into_low_level(context, bump))
                as *mut crate::native::extensions::ext_image_drm_format_modifier::DrmFormatModifierPropertiesListEXT)
                .cast(),
            #[cfg(feature = "VK_KHR_video_queue")]
            Self::VideoProfilesKHR(ext) => (bump.alloc(ext.into_low_level(context, bump))
                as *mut crate::native::extensions::khr_video_queue::VideoProfilesKHR)
                .cast(),
            #[cfg(feature = "VK_KHR_video_queue")]
            Self::VideoProfileKHR(ext) => (bump.alloc(ext.into_low_level(context, bump))
                as *mut crate::native::extensions::khr_video_queue::VideoProfileKHR)
                .cast(),
            #[cfg(feature = "VULKAN_1_3")]
            Self::FormatProperties3(ext) => (bump.alloc(ext.into_low_level(context, bump))
                as *mut crate::native::vulkan1_3::FormatProperties3)
                .cast(),
            #[cfg(feature = "VK_EXT_image_drm_format_modifier")]
            Self::DrmFormatModifierPropertiesList2EXT(ext) => (bump.alloc(ext.into_low_level(context, bump))
                as *mut crate::native::extensions::ext_image_drm_format_modifier::DrmFormatModifierPropertiesList2EXT)
                .cast(),
            other => unreachable!("unexpected variant {:?}", other),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for FormatProperties2Extension {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        assert!(!value.is_null());
        match (* value) . s_type { # [cfg (feature = "VK_EXT_image_drm_format_modifier")] crate :: native :: vulkan1_0 :: StructureType :: DrmFormatModifierPropertiesListExt => Self :: DrmFormatModifierPropertiesListEXT (DrmFormatModifierPropertiesListEXT :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: ext_image_drm_format_modifier :: DrmFormatModifierPropertiesListEXT > ()))) , # [cfg (feature = "VK_KHR_video_queue")] crate :: native :: vulkan1_0 :: StructureType :: VideoProfilesKhr => Self :: VideoProfilesKHR (VideoProfilesKHR :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: khr_video_queue :: VideoProfilesKHR > ()))) , # [cfg (feature = "VK_KHR_video_queue")] crate :: native :: vulkan1_0 :: StructureType :: VideoProfileKhr => Self :: VideoProfileKHR (VideoProfileKHR :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: khr_video_queue :: VideoProfileKHR > ()))) , # [cfg (feature = "VULKAN_1_3")] crate :: native :: vulkan1_0 :: StructureType :: FormatProperties3 => Self :: FormatProperties3 (FormatProperties3 :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: vulkan1_3 :: FormatProperties3 > ()))) , # [cfg (feature = "VK_EXT_image_drm_format_modifier")] crate :: native :: vulkan1_0 :: StructureType :: DrmFormatModifierPropertiesList2Ext => Self :: DrmFormatModifierPropertiesList2EXT (DrmFormatModifierPropertiesList2EXT :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: ext_image_drm_format_modifier :: DrmFormatModifierPropertiesList2EXT > ()))) , other => panic ! ("Structure type {:?} is not a member of {}" , other , stringify ! (FormatProperties2)) }
    }
}
#[cfg(feature = "VK_EXT_image_drm_format_modifier")]
impl From<DrmFormatModifierPropertiesListEXT> for FormatProperties2Extension {
    fn from(ext: DrmFormatModifierPropertiesListEXT) -> Self {
        Self::DrmFormatModifierPropertiesListEXT(ext)
    }
}
#[cfg(feature = "VK_EXT_image_drm_format_modifier")]
impl TryInto<DrmFormatModifierPropertiesListEXT> for FormatProperties2Extension {
    type Error = FormatProperties2Extension;
    fn try_into(self) -> Result<DrmFormatModifierPropertiesListEXT, Self::Error> {
        match self {
            Self::DrmFormatModifierPropertiesListEXT(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_KHR_video_queue")]
impl From<VideoProfilesKHR> for FormatProperties2Extension {
    fn from(ext: VideoProfilesKHR) -> Self {
        Self::VideoProfilesKHR(ext)
    }
}
#[cfg(feature = "VK_KHR_video_queue")]
impl TryInto<VideoProfilesKHR> for FormatProperties2Extension {
    type Error = FormatProperties2Extension;
    fn try_into(self) -> Result<VideoProfilesKHR, Self::Error> {
        match self {
            Self::VideoProfilesKHR(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_KHR_video_queue")]
impl From<VideoProfileKHR> for FormatProperties2Extension {
    fn from(ext: VideoProfileKHR) -> Self {
        Self::VideoProfileKHR(ext)
    }
}
#[cfg(feature = "VK_KHR_video_queue")]
impl TryInto<VideoProfileKHR> for FormatProperties2Extension {
    type Error = FormatProperties2Extension;
    fn try_into(self) -> Result<VideoProfileKHR, Self::Error> {
        match self {
            Self::VideoProfileKHR(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VULKAN_1_3")]
impl From<FormatProperties3> for FormatProperties2Extension {
    fn from(ext: FormatProperties3) -> Self {
        Self::FormatProperties3(ext)
    }
}
#[cfg(feature = "VULKAN_1_3")]
impl TryInto<FormatProperties3> for FormatProperties2Extension {
    type Error = FormatProperties2Extension;
    fn try_into(self) -> Result<FormatProperties3, Self::Error> {
        match self {
            Self::FormatProperties3(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_EXT_image_drm_format_modifier")]
impl From<DrmFormatModifierPropertiesList2EXT> for FormatProperties2Extension {
    fn from(ext: DrmFormatModifierPropertiesList2EXT) -> Self {
        Self::DrmFormatModifierPropertiesList2EXT(ext)
    }
}
#[cfg(feature = "VK_EXT_image_drm_format_modifier")]
impl TryInto<DrmFormatModifierPropertiesList2EXT> for FormatProperties2Extension {
    type Error = FormatProperties2Extension;
    fn try_into(self) -> Result<DrmFormatModifierPropertiesList2EXT, Self::Error> {
        match self {
            Self::DrmFormatModifierPropertiesList2EXT(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[doc(alias = "VkImageFormatProperties2")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ImageFormatProperties2 {
    #[doc(alias = "pNext")]
    pub extensions: SmallVec<[ImageFormatProperties2Extension; 1]>,
    #[doc(alias = "imageFormatProperties")]
    pub image_format_properties: ImageFormatProperties,
}
impl ImageFormatProperties2 {
    ///Adds an extension to the struct
    pub fn with_extension(mut self, ext: impl Into<ImageFormatProperties2Extension>) -> Self {
        self.extensions.push(ext.into());
        self
    }
    ///Get a reference to the `extensions` field.
    pub fn extensions(&self) -> &SmallVec<[ImageFormatProperties2Extension; 1]> {
        &self.extensions
    }
    ///Get a reference to the `image_format_properties` field.
    pub fn image_format_properties(&self) -> &ImageFormatProperties {
        &self.image_format_properties
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for ImageFormatProperties2 {
    type LowLevel = crate::native::vulkan1_1::ImageFormatProperties2;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let mut next = std::ptr::null_mut();
        let mut extensions = self.extensions.iter();
        while let Some(ext) = extensions.next() {
            let ext = ext.into_low_level(context, bump);
            (*ext).next = next;
            next = ext;
        }
        crate::native::vulkan1_1::ImageFormatProperties2 {
            s_type: StructureType::ImageFormatProperties2,
            p_next: next,
            image_format_properties: self.image_format_properties.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for ImageFormatProperties2 {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let mut next = value.p_next;
        let mut extensions = SmallVec::new();
        while !next.is_null() {
            extensions.push(crate::conv::FromLowLevel::from_low_level(context, next));
            next = std::ptr::read(next).next;
        }
        Self {
            extensions: extensions,
            image_format_properties: crate::conv::FromLowLevel::from_low_level(context, value.image_format_properties),
        }
    }
}
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
///Extensions for [`ImageFormatProperties2`]
pub enum ImageFormatProperties2Extension {
    ///Contains a type [`ExternalImageFormatProperties`] for extending [`ImageFormatProperties2`]
    ExternalImageFormatProperties(ExternalImageFormatProperties),
    ///Contains a type [`SamplerYcbcrConversionImageFormatProperties`] for extending
    /// [`ImageFormatProperties2`]
    SamplerYcbcrConversionImageFormatProperties(SamplerYcbcrConversionImageFormatProperties),
    #[cfg(feature = "VK_AMD_texture_gather_bias_lod")]
    ///Contains a type [`TextureLodGatherFormatPropertiesAMD`] for extending
    /// [`ImageFormatProperties2`]
    TextureLodGatherFormatPropertiesAMD(TextureLodGatherFormatPropertiesAMD),
    #[cfg(feature = "VK_ANDROID_external_memory_android_hardware_buffer")]
    ///Contains a type [`AndroidHardwareBufferUsageANDROID`] for extending
    /// [`ImageFormatProperties2`]
    AndroidHardwareBufferUsageANDROID(AndroidHardwareBufferUsageANDROID),
    #[cfg(feature = "VK_EXT_filter_cubic")]
    ///Contains a type [`FilterCubicImageViewImageFormatPropertiesEXT`] for extending
    /// [`ImageFormatProperties2`]
    FilterCubicImageViewImageFormatPropertiesEXT(FilterCubicImageViewImageFormatPropertiesEXT),
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for ImageFormatProperties2Extension {
    type LowLevel = *mut crate::native::vulkan1_0::BaseOutStructure;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        match self { Self :: ExternalImageFormatProperties (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: vulkan1_1 :: ExternalImageFormatProperties) . cast () , Self :: SamplerYcbcrConversionImageFormatProperties (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: vulkan1_1 :: SamplerYcbcrConversionImageFormatProperties) . cast () , # [cfg (feature = "VK_AMD_texture_gather_bias_lod")] Self :: TextureLodGatherFormatPropertiesAMD (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: amd_texture_gather_bias_lod :: TextureLodGatherFormatPropertiesAMD) . cast () , # [cfg (feature = "VK_ANDROID_external_memory_android_hardware_buffer")] Self :: AndroidHardwareBufferUsageANDROID (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: android_external_memory_android_hardware_buffer :: AndroidHardwareBufferUsageANDROID) . cast () , # [cfg (feature = "VK_EXT_filter_cubic")] Self :: FilterCubicImageViewImageFormatPropertiesEXT (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: ext_filter_cubic :: FilterCubicImageViewImageFormatPropertiesEXT) . cast () , other => unreachable ! ("unexpected variant {:?}" , other) }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for ImageFormatProperties2Extension {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        assert!(!value.is_null());
        match (* value) . s_type { crate :: native :: vulkan1_0 :: StructureType :: ExternalImageFormatProperties => Self :: ExternalImageFormatProperties (ExternalImageFormatProperties :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: vulkan1_1 :: ExternalImageFormatProperties > ()))) , crate :: native :: vulkan1_0 :: StructureType :: SamplerYcbcrConversionImageFormatProperties => Self :: SamplerYcbcrConversionImageFormatProperties (SamplerYcbcrConversionImageFormatProperties :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: vulkan1_1 :: SamplerYcbcrConversionImageFormatProperties > ()))) , # [cfg (feature = "VK_AMD_texture_gather_bias_lod")] crate :: native :: vulkan1_0 :: StructureType :: TextureLodGatherFormatPropertiesAmd => Self :: TextureLodGatherFormatPropertiesAMD (TextureLodGatherFormatPropertiesAMD :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: amd_texture_gather_bias_lod :: TextureLodGatherFormatPropertiesAMD > ()))) , # [cfg (feature = "VK_ANDROID_external_memory_android_hardware_buffer")] crate :: native :: vulkan1_0 :: StructureType :: AndroidHardwareBufferUsageAndroid => Self :: AndroidHardwareBufferUsageANDROID (AndroidHardwareBufferUsageANDROID :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: android_external_memory_android_hardware_buffer :: AndroidHardwareBufferUsageANDROID > ()))) , # [cfg (feature = "VK_EXT_filter_cubic")] crate :: native :: vulkan1_0 :: StructureType :: FilterCubicImageViewImageFormatPropertiesExt => Self :: FilterCubicImageViewImageFormatPropertiesEXT (FilterCubicImageViewImageFormatPropertiesEXT :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: ext_filter_cubic :: FilterCubicImageViewImageFormatPropertiesEXT > ()))) , other => panic ! ("Structure type {:?} is not a member of {}" , other , stringify ! (ImageFormatProperties2)) }
    }
}
impl From<ExternalImageFormatProperties> for ImageFormatProperties2Extension {
    fn from(ext: ExternalImageFormatProperties) -> Self {
        Self::ExternalImageFormatProperties(ext)
    }
}
impl TryInto<ExternalImageFormatProperties> for ImageFormatProperties2Extension {
    type Error = ImageFormatProperties2Extension;
    fn try_into(self) -> Result<ExternalImageFormatProperties, Self::Error> {
        match self {
            Self::ExternalImageFormatProperties(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
impl From<SamplerYcbcrConversionImageFormatProperties> for ImageFormatProperties2Extension {
    fn from(ext: SamplerYcbcrConversionImageFormatProperties) -> Self {
        Self::SamplerYcbcrConversionImageFormatProperties(ext)
    }
}
impl TryInto<SamplerYcbcrConversionImageFormatProperties> for ImageFormatProperties2Extension {
    type Error = ImageFormatProperties2Extension;
    fn try_into(self) -> Result<SamplerYcbcrConversionImageFormatProperties, Self::Error> {
        match self {
            Self::SamplerYcbcrConversionImageFormatProperties(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_AMD_texture_gather_bias_lod")]
impl From<TextureLodGatherFormatPropertiesAMD> for ImageFormatProperties2Extension {
    fn from(ext: TextureLodGatherFormatPropertiesAMD) -> Self {
        Self::TextureLodGatherFormatPropertiesAMD(ext)
    }
}
#[cfg(feature = "VK_AMD_texture_gather_bias_lod")]
impl TryInto<TextureLodGatherFormatPropertiesAMD> for ImageFormatProperties2Extension {
    type Error = ImageFormatProperties2Extension;
    fn try_into(self) -> Result<TextureLodGatherFormatPropertiesAMD, Self::Error> {
        match self {
            Self::TextureLodGatherFormatPropertiesAMD(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_ANDROID_external_memory_android_hardware_buffer")]
impl From<AndroidHardwareBufferUsageANDROID> for ImageFormatProperties2Extension {
    fn from(ext: AndroidHardwareBufferUsageANDROID) -> Self {
        Self::AndroidHardwareBufferUsageANDROID(ext)
    }
}
#[cfg(feature = "VK_ANDROID_external_memory_android_hardware_buffer")]
impl TryInto<AndroidHardwareBufferUsageANDROID> for ImageFormatProperties2Extension {
    type Error = ImageFormatProperties2Extension;
    fn try_into(self) -> Result<AndroidHardwareBufferUsageANDROID, Self::Error> {
        match self {
            Self::AndroidHardwareBufferUsageANDROID(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_EXT_filter_cubic")]
impl From<FilterCubicImageViewImageFormatPropertiesEXT> for ImageFormatProperties2Extension {
    fn from(ext: FilterCubicImageViewImageFormatPropertiesEXT) -> Self {
        Self::FilterCubicImageViewImageFormatPropertiesEXT(ext)
    }
}
#[cfg(feature = "VK_EXT_filter_cubic")]
impl TryInto<FilterCubicImageViewImageFormatPropertiesEXT> for ImageFormatProperties2Extension {
    type Error = ImageFormatProperties2Extension;
    fn try_into(self) -> Result<FilterCubicImageViewImageFormatPropertiesEXT, Self::Error> {
        match self {
            Self::FilterCubicImageViewImageFormatPropertiesEXT(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[doc(alias = "VkPhysicalDeviceImageFormatInfo2")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceImageFormatInfo2 {
    #[doc(alias = "pNext")]
    pub extensions: SmallVec<[PhysicalDeviceImageFormatInfo2Extension; 1]>,
    pub format: Format,
    #[doc(alias = "type")]
    pub type_: ImageType,
    pub tiling: ImageTiling,
    pub usage: ImageUsageFlags,
    pub flags: ImageCreateFlags,
}
impl PhysicalDeviceImageFormatInfo2 {
    ///Adds an extension to the struct
    pub fn with_extension(mut self, ext: impl Into<PhysicalDeviceImageFormatInfo2Extension>) -> Self {
        self.extensions.push(ext.into());
        self
    }
    ///Get a reference to the `extensions` field.
    pub fn extensions(&self) -> &SmallVec<[PhysicalDeviceImageFormatInfo2Extension; 1]> {
        &self.extensions
    }
    ///Get a reference to the `format` field.
    pub fn format(&self) -> Format {
        self.format
    }
    ///Get a reference to the `type_` field.
    pub fn type_(&self) -> ImageType {
        self.type_
    }
    ///Get a reference to the `tiling` field.
    pub fn tiling(&self) -> ImageTiling {
        self.tiling
    }
    ///Get a reference to the `usage` field.
    pub fn usage(&self) -> ImageUsageFlags {
        self.usage
    }
    ///Get a reference to the `flags` field.
    pub fn flags(&self) -> ImageCreateFlags {
        self.flags
    }
    ///Get a mutable reference to the `extensions` field.
    pub fn extensions_mut(&mut self) -> &mut SmallVec<[PhysicalDeviceImageFormatInfo2Extension; 1]> {
        &mut self.extensions
    }
    ///Get a mutable reference to the `format` field.
    pub fn format_mut(&mut self) -> &mut Format {
        &mut self.format
    }
    ///Get a mutable reference to the `type_` field.
    pub fn type__mut(&mut self) -> &mut ImageType {
        &mut self.type_
    }
    ///Get a mutable reference to the `tiling` field.
    pub fn tiling_mut(&mut self) -> &mut ImageTiling {
        &mut self.tiling
    }
    ///Get a mutable reference to the `usage` field.
    pub fn usage_mut(&mut self) -> &mut ImageUsageFlags {
        &mut self.usage
    }
    ///Get a mutable reference to the `flags` field.
    pub fn flags_mut(&mut self) -> &mut ImageCreateFlags {
        &mut self.flags
    }
    ///Sets the `extensions` field.
    pub fn set_extensions(&mut self, extensions: SmallVec<[PhysicalDeviceImageFormatInfo2Extension; 1]>) -> &mut Self {
        self.extensions = extensions;
        self
    }
    ///Sets the `format` field.
    pub fn set_format(&mut self, format: Format) -> &mut Self {
        self.format = format;
        self
    }
    ///Sets the `type_` field.
    pub fn set_type_(&mut self, type_: ImageType) -> &mut Self {
        self.type_ = type_;
        self
    }
    ///Sets the `tiling` field.
    pub fn set_tiling(&mut self, tiling: ImageTiling) -> &mut Self {
        self.tiling = tiling;
        self
    }
    ///Sets the `usage` field.
    pub fn set_usage(&mut self, usage: ImageUsageFlags) -> &mut Self {
        self.usage = usage;
        self
    }
    ///Sets the `flags` field.
    pub fn set_flags(&mut self, flags: ImageCreateFlags) -> &mut Self {
        self.flags = flags;
        self
    }
    ///Sets the `extensions` field in a builder way.
    pub fn with_extensions(mut self, extensions: SmallVec<[PhysicalDeviceImageFormatInfo2Extension; 1]>) -> Self {
        self.extensions = extensions;
        self
    }
    ///Sets the `format` field in a builder way.
    pub fn with_format(mut self, format: Format) -> Self {
        self.format = format;
        self
    }
    ///Sets the `type_` field in a builder way.
    pub fn with_type_(mut self, type_: ImageType) -> Self {
        self.type_ = type_;
        self
    }
    ///Sets the `tiling` field in a builder way.
    pub fn with_tiling(mut self, tiling: ImageTiling) -> Self {
        self.tiling = tiling;
        self
    }
    ///Sets the `usage` field in a builder way.
    pub fn with_usage(mut self, usage: ImageUsageFlags) -> Self {
        self.usage = usage;
        self
    }
    ///Sets the `flags` field in a builder way.
    pub fn with_flags(mut self, flags: ImageCreateFlags) -> Self {
        self.flags = flags;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceImageFormatInfo2 {
    type LowLevel = crate::native::vulkan1_1::PhysicalDeviceImageFormatInfo2;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let mut next = std::ptr::null();
        let mut extensions = self.extensions.iter();
        while let Some(ext) = extensions.next() {
            let ext = ext.into_low_level(context, bump);
            (*ext).next = next;
            next = ext;
        }
        crate::native::vulkan1_1::PhysicalDeviceImageFormatInfo2 {
            s_type: StructureType::PhysicalDeviceImageFormatInfo2,
            p_next: next,
            format: self.format.into_low_level(context, bump),
            type_: self.type_.into_low_level(context, bump),
            tiling: self.tiling.into_low_level(context, bump),
            usage: self.usage.into_low_level(context, bump),
            flags: self.flags.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceImageFormatInfo2 {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let mut next = value.p_next;
        let mut extensions = SmallVec::new();
        while !next.is_null() {
            extensions.push(crate::conv::FromLowLevel::from_low_level(context, next));
            next = std::ptr::read(next).next;
        }
        Self {
            extensions: extensions,
            format: crate::conv::FromLowLevel::from_low_level(context, value.format),
            type_: crate::conv::FromLowLevel::from_low_level(context, value.type_),
            tiling: crate::conv::FromLowLevel::from_low_level(context, value.tiling),
            usage: crate::conv::FromLowLevel::from_low_level(context, value.usage),
            flags: crate::conv::FromLowLevel::from_low_level(context, value.flags),
        }
    }
}
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
///Extensions for [`PhysicalDeviceImageFormatInfo2`]
pub enum PhysicalDeviceImageFormatInfo2Extension {
    ///Contains a type [`PhysicalDeviceExternalImageFormatInfo`] for extending
    /// [`PhysicalDeviceImageFormatInfo2`]
    PhysicalDeviceExternalImageFormatInfo(PhysicalDeviceExternalImageFormatInfo),
    #[cfg(feature = "VULKAN_1_2")]
    ///Contains a type [`ImageFormatListCreateInfo`] for extending
    /// [`PhysicalDeviceImageFormatInfo2`]
    ImageFormatListCreateInfo(ImageFormatListCreateInfo),
    #[cfg(feature = "VK_EXT_image_drm_format_modifier")]
    ///Contains a type [`PhysicalDeviceImageDrmFormatModifierInfoEXT`] for extending
    /// [`PhysicalDeviceImageFormatInfo2`]
    PhysicalDeviceImageDrmFormatModifierInfoEXT(PhysicalDeviceImageDrmFormatModifierInfoEXT),
    #[cfg(feature = "VULKAN_1_2")]
    ///Contains a type [`ImageStencilUsageCreateInfo`] for extending
    /// [`PhysicalDeviceImageFormatInfo2`]
    ImageStencilUsageCreateInfo(ImageStencilUsageCreateInfo),
    #[cfg(feature = "VK_EXT_filter_cubic")]
    ///Contains a type [`PhysicalDeviceImageViewImageFormatInfoEXT`] for extending
    /// [`PhysicalDeviceImageFormatInfo2`]
    PhysicalDeviceImageViewImageFormatInfoEXT(PhysicalDeviceImageViewImageFormatInfoEXT),
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceImageFormatInfo2Extension {
    type LowLevel = *const crate::native::vulkan1_0::BaseInStructure;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        match self { Self :: PhysicalDeviceExternalImageFormatInfo (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: vulkan1_1 :: PhysicalDeviceExternalImageFormatInfo) . cast () , # [cfg (feature = "VULKAN_1_2")] Self :: ImageFormatListCreateInfo (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: vulkan1_2 :: ImageFormatListCreateInfo) . cast () , # [cfg (feature = "VK_EXT_image_drm_format_modifier")] Self :: PhysicalDeviceImageDrmFormatModifierInfoEXT (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: ext_image_drm_format_modifier :: PhysicalDeviceImageDrmFormatModifierInfoEXT) . cast () , # [cfg (feature = "VULKAN_1_2")] Self :: ImageStencilUsageCreateInfo (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: vulkan1_2 :: ImageStencilUsageCreateInfo) . cast () , # [cfg (feature = "VK_EXT_filter_cubic")] Self :: PhysicalDeviceImageViewImageFormatInfoEXT (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: ext_filter_cubic :: PhysicalDeviceImageViewImageFormatInfoEXT) . cast () , other => unreachable ! ("unexpected variant {:?}" , other) }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceImageFormatInfo2Extension {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        assert!(!value.is_null());
        match (* value) . s_type { crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceExternalImageFormatInfo => Self :: PhysicalDeviceExternalImageFormatInfo (PhysicalDeviceExternalImageFormatInfo :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: vulkan1_1 :: PhysicalDeviceExternalImageFormatInfo > ()))) , # [cfg (feature = "VULKAN_1_2")] crate :: native :: vulkan1_0 :: StructureType :: ImageFormatListCreateInfo => Self :: ImageFormatListCreateInfo (ImageFormatListCreateInfo :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: vulkan1_2 :: ImageFormatListCreateInfo > ()))) , # [cfg (feature = "VK_EXT_image_drm_format_modifier")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceImageDrmFormatModifierInfoExt => Self :: PhysicalDeviceImageDrmFormatModifierInfoEXT (PhysicalDeviceImageDrmFormatModifierInfoEXT :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: ext_image_drm_format_modifier :: PhysicalDeviceImageDrmFormatModifierInfoEXT > ()))) , # [cfg (feature = "VULKAN_1_2")] crate :: native :: vulkan1_0 :: StructureType :: ImageStencilUsageCreateInfo => Self :: ImageStencilUsageCreateInfo (ImageStencilUsageCreateInfo :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: vulkan1_2 :: ImageStencilUsageCreateInfo > ()))) , # [cfg (feature = "VK_EXT_filter_cubic")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceImageViewImageFormatInfoExt => Self :: PhysicalDeviceImageViewImageFormatInfoEXT (PhysicalDeviceImageViewImageFormatInfoEXT :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: ext_filter_cubic :: PhysicalDeviceImageViewImageFormatInfoEXT > ()))) , other => panic ! ("Structure type {:?} is not a member of {}" , other , stringify ! (PhysicalDeviceImageFormatInfo2)) }
    }
}
impl From<PhysicalDeviceExternalImageFormatInfo> for PhysicalDeviceImageFormatInfo2Extension {
    fn from(ext: PhysicalDeviceExternalImageFormatInfo) -> Self {
        Self::PhysicalDeviceExternalImageFormatInfo(ext)
    }
}
impl TryInto<PhysicalDeviceExternalImageFormatInfo> for PhysicalDeviceImageFormatInfo2Extension {
    type Error = PhysicalDeviceImageFormatInfo2Extension;
    fn try_into(self) -> Result<PhysicalDeviceExternalImageFormatInfo, Self::Error> {
        match self {
            Self::PhysicalDeviceExternalImageFormatInfo(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VULKAN_1_2")]
impl From<ImageFormatListCreateInfo> for PhysicalDeviceImageFormatInfo2Extension {
    fn from(ext: ImageFormatListCreateInfo) -> Self {
        Self::ImageFormatListCreateInfo(ext)
    }
}
#[cfg(feature = "VULKAN_1_2")]
impl TryInto<ImageFormatListCreateInfo> for PhysicalDeviceImageFormatInfo2Extension {
    type Error = PhysicalDeviceImageFormatInfo2Extension;
    fn try_into(self) -> Result<ImageFormatListCreateInfo, Self::Error> {
        match self {
            Self::ImageFormatListCreateInfo(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_EXT_image_drm_format_modifier")]
impl From<PhysicalDeviceImageDrmFormatModifierInfoEXT> for PhysicalDeviceImageFormatInfo2Extension {
    fn from(ext: PhysicalDeviceImageDrmFormatModifierInfoEXT) -> Self {
        Self::PhysicalDeviceImageDrmFormatModifierInfoEXT(ext)
    }
}
#[cfg(feature = "VK_EXT_image_drm_format_modifier")]
impl TryInto<PhysicalDeviceImageDrmFormatModifierInfoEXT> for PhysicalDeviceImageFormatInfo2Extension {
    type Error = PhysicalDeviceImageFormatInfo2Extension;
    fn try_into(self) -> Result<PhysicalDeviceImageDrmFormatModifierInfoEXT, Self::Error> {
        match self {
            Self::PhysicalDeviceImageDrmFormatModifierInfoEXT(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VULKAN_1_2")]
impl From<ImageStencilUsageCreateInfo> for PhysicalDeviceImageFormatInfo2Extension {
    fn from(ext: ImageStencilUsageCreateInfo) -> Self {
        Self::ImageStencilUsageCreateInfo(ext)
    }
}
#[cfg(feature = "VULKAN_1_2")]
impl TryInto<ImageStencilUsageCreateInfo> for PhysicalDeviceImageFormatInfo2Extension {
    type Error = PhysicalDeviceImageFormatInfo2Extension;
    fn try_into(self) -> Result<ImageStencilUsageCreateInfo, Self::Error> {
        match self {
            Self::ImageStencilUsageCreateInfo(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_EXT_filter_cubic")]
impl From<PhysicalDeviceImageViewImageFormatInfoEXT> for PhysicalDeviceImageFormatInfo2Extension {
    fn from(ext: PhysicalDeviceImageViewImageFormatInfoEXT) -> Self {
        Self::PhysicalDeviceImageViewImageFormatInfoEXT(ext)
    }
}
#[cfg(feature = "VK_EXT_filter_cubic")]
impl TryInto<PhysicalDeviceImageViewImageFormatInfoEXT> for PhysicalDeviceImageFormatInfo2Extension {
    type Error = PhysicalDeviceImageFormatInfo2Extension;
    fn try_into(self) -> Result<PhysicalDeviceImageViewImageFormatInfoEXT, Self::Error> {
        match self {
            Self::PhysicalDeviceImageViewImageFormatInfoEXT(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[doc(alias = "VkQueueFamilyProperties2")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueueFamilyProperties2 {
    #[doc(alias = "pNext")]
    pub extensions: SmallVec<[QueueFamilyProperties2Extension; 1]>,
    #[doc(alias = "queueFamilyProperties")]
    pub queue_family_properties: QueueFamilyProperties,
}
impl QueueFamilyProperties2 {
    ///Adds an extension to the struct
    pub fn with_extension(mut self, ext: impl Into<QueueFamilyProperties2Extension>) -> Self {
        self.extensions.push(ext.into());
        self
    }
    ///Get a reference to the `extensions` field.
    pub fn extensions(&self) -> &SmallVec<[QueueFamilyProperties2Extension; 1]> {
        &self.extensions
    }
    ///Get a reference to the `queue_family_properties` field.
    pub fn queue_family_properties(&self) -> QueueFamilyProperties {
        self.queue_family_properties
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for QueueFamilyProperties2 {
    type LowLevel = crate::native::vulkan1_1::QueueFamilyProperties2;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let mut next = std::ptr::null_mut();
        let mut extensions = self.extensions.iter();
        while let Some(ext) = extensions.next() {
            let ext = ext.into_low_level(context, bump);
            (*ext).next = next;
            next = ext;
        }
        crate::native::vulkan1_1::QueueFamilyProperties2 {
            s_type: StructureType::QueueFamilyProperties2,
            p_next: next,
            queue_family_properties: self.queue_family_properties.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for QueueFamilyProperties2 {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let mut next = value.p_next;
        let mut extensions = SmallVec::new();
        while !next.is_null() {
            extensions.push(crate::conv::FromLowLevel::from_low_level(context, next));
            next = std::ptr::read(next).next;
        }
        Self {
            extensions: extensions,
            queue_family_properties: crate::conv::FromLowLevel::from_low_level(context, value.queue_family_properties),
        }
    }
}
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
///Extensions for [`QueueFamilyProperties2`]
pub enum QueueFamilyProperties2Extension {
    #[cfg(feature = "VK_KHR_global_priority")]
    ///Contains a type [`QueueFamilyGlobalPriorityPropertiesKHR`] for extending
    /// [`QueueFamilyProperties2`]
    QueueFamilyGlobalPriorityPropertiesKHR(QueueFamilyGlobalPriorityPropertiesKHR),
    #[cfg(feature = "VK_NV_device_diagnostic_checkpoints")]
    ///Contains a type [`QueueFamilyCheckpointPropertiesNV`] for extending
    /// [`QueueFamilyProperties2`]
    QueueFamilyCheckpointPropertiesNV(QueueFamilyCheckpointPropertiesNV),
    #[cfg(feature = "VK_KHR_synchronization2")]
    ///Contains a type [`QueueFamilyCheckpointProperties2NV`] for extending
    /// [`QueueFamilyProperties2`]
    QueueFamilyCheckpointProperties2NV(QueueFamilyCheckpointProperties2NV),
    #[cfg(feature = "VK_KHR_video_queue")]
    ///Contains a type [`VideoQueueFamilyProperties2KHR`] for extending [`QueueFamilyProperties2`]
    VideoQueueFamilyProperties2KHR(VideoQueueFamilyProperties2KHR),
    #[cfg(feature = "VK_KHR_video_queue")]
    ///Contains a type [`QueueFamilyQueryResultStatusProperties2KHR`] for extending
    /// [`QueueFamilyProperties2`]
    QueueFamilyQueryResultStatusProperties2KHR(QueueFamilyQueryResultStatusProperties2KHR),
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for QueueFamilyProperties2Extension {
    type LowLevel = *mut crate::native::vulkan1_0::BaseOutStructure;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        match self {
            #[cfg(feature = "VK_KHR_global_priority")]
            Self::QueueFamilyGlobalPriorityPropertiesKHR(ext) => (bump.alloc(ext.into_low_level(context, bump))
                as *mut crate::native::extensions::khr_global_priority::QueueFamilyGlobalPriorityPropertiesKHR)
                .cast(),
            #[cfg(feature = "VK_NV_device_diagnostic_checkpoints")]
            Self::QueueFamilyCheckpointPropertiesNV(ext) => (bump.alloc(ext.into_low_level(context, bump))
                as *mut crate::native::extensions::nv_device_diagnostic_checkpoints::QueueFamilyCheckpointPropertiesNV)
                .cast(),
            #[cfg(feature = "VK_KHR_synchronization2")]
            Self::QueueFamilyCheckpointProperties2NV(ext) => (bump.alloc(ext.into_low_level(context, bump))
                as *mut crate::native::extensions::khr_synchronization2::QueueFamilyCheckpointProperties2NV)
                .cast(),
            #[cfg(feature = "VK_KHR_video_queue")]
            Self::VideoQueueFamilyProperties2KHR(ext) => (bump.alloc(ext.into_low_level(context, bump))
                as *mut crate::native::extensions::khr_video_queue::VideoQueueFamilyProperties2KHR)
                .cast(),
            #[cfg(feature = "VK_KHR_video_queue")]
            Self::QueueFamilyQueryResultStatusProperties2KHR(ext) => (bump.alloc(ext.into_low_level(context, bump))
                as *mut crate::native::extensions::khr_video_queue::QueueFamilyQueryResultStatusProperties2KHR)
                .cast(),
            other => unreachable!("unexpected variant {:?}", other),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for QueueFamilyProperties2Extension {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        assert!(!value.is_null());
        match (* value) . s_type { # [cfg (feature = "VK_KHR_global_priority")] crate :: native :: vulkan1_0 :: StructureType :: QueueFamilyGlobalPriorityPropertiesKhr => Self :: QueueFamilyGlobalPriorityPropertiesKHR (QueueFamilyGlobalPriorityPropertiesKHR :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: khr_global_priority :: QueueFamilyGlobalPriorityPropertiesKHR > ()))) , # [cfg (feature = "VK_NV_device_diagnostic_checkpoints")] crate :: native :: vulkan1_0 :: StructureType :: QueueFamilyCheckpointPropertiesNv => Self :: QueueFamilyCheckpointPropertiesNV (QueueFamilyCheckpointPropertiesNV :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: nv_device_diagnostic_checkpoints :: QueueFamilyCheckpointPropertiesNV > ()))) , # [cfg (feature = "VK_KHR_synchronization2")] crate :: native :: vulkan1_0 :: StructureType :: QueueFamilyCheckpointProperties2Nv => Self :: QueueFamilyCheckpointProperties2NV (QueueFamilyCheckpointProperties2NV :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: khr_synchronization2 :: QueueFamilyCheckpointProperties2NV > ()))) , # [cfg (feature = "VK_KHR_video_queue")] crate :: native :: vulkan1_0 :: StructureType :: VideoQueueFamilyProperties2Khr => Self :: VideoQueueFamilyProperties2KHR (VideoQueueFamilyProperties2KHR :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: khr_video_queue :: VideoQueueFamilyProperties2KHR > ()))) , # [cfg (feature = "VK_KHR_video_queue")] crate :: native :: vulkan1_0 :: StructureType :: QueueFamilyQueryResultStatusProperties2Khr => Self :: QueueFamilyQueryResultStatusProperties2KHR (QueueFamilyQueryResultStatusProperties2KHR :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: khr_video_queue :: QueueFamilyQueryResultStatusProperties2KHR > ()))) , other => panic ! ("Structure type {:?} is not a member of {}" , other , stringify ! (QueueFamilyProperties2)) }
    }
}
#[cfg(feature = "VK_KHR_global_priority")]
impl From<QueueFamilyGlobalPriorityPropertiesKHR> for QueueFamilyProperties2Extension {
    fn from(ext: QueueFamilyGlobalPriorityPropertiesKHR) -> Self {
        Self::QueueFamilyGlobalPriorityPropertiesKHR(ext)
    }
}
#[cfg(feature = "VK_KHR_global_priority")]
impl TryInto<QueueFamilyGlobalPriorityPropertiesKHR> for QueueFamilyProperties2Extension {
    type Error = QueueFamilyProperties2Extension;
    fn try_into(self) -> Result<QueueFamilyGlobalPriorityPropertiesKHR, Self::Error> {
        match self {
            Self::QueueFamilyGlobalPriorityPropertiesKHR(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_NV_device_diagnostic_checkpoints")]
impl From<QueueFamilyCheckpointPropertiesNV> for QueueFamilyProperties2Extension {
    fn from(ext: QueueFamilyCheckpointPropertiesNV) -> Self {
        Self::QueueFamilyCheckpointPropertiesNV(ext)
    }
}
#[cfg(feature = "VK_NV_device_diagnostic_checkpoints")]
impl TryInto<QueueFamilyCheckpointPropertiesNV> for QueueFamilyProperties2Extension {
    type Error = QueueFamilyProperties2Extension;
    fn try_into(self) -> Result<QueueFamilyCheckpointPropertiesNV, Self::Error> {
        match self {
            Self::QueueFamilyCheckpointPropertiesNV(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_KHR_synchronization2")]
impl From<QueueFamilyCheckpointProperties2NV> for QueueFamilyProperties2Extension {
    fn from(ext: QueueFamilyCheckpointProperties2NV) -> Self {
        Self::QueueFamilyCheckpointProperties2NV(ext)
    }
}
#[cfg(feature = "VK_KHR_synchronization2")]
impl TryInto<QueueFamilyCheckpointProperties2NV> for QueueFamilyProperties2Extension {
    type Error = QueueFamilyProperties2Extension;
    fn try_into(self) -> Result<QueueFamilyCheckpointProperties2NV, Self::Error> {
        match self {
            Self::QueueFamilyCheckpointProperties2NV(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_KHR_video_queue")]
impl From<VideoQueueFamilyProperties2KHR> for QueueFamilyProperties2Extension {
    fn from(ext: VideoQueueFamilyProperties2KHR) -> Self {
        Self::VideoQueueFamilyProperties2KHR(ext)
    }
}
#[cfg(feature = "VK_KHR_video_queue")]
impl TryInto<VideoQueueFamilyProperties2KHR> for QueueFamilyProperties2Extension {
    type Error = QueueFamilyProperties2Extension;
    fn try_into(self) -> Result<VideoQueueFamilyProperties2KHR, Self::Error> {
        match self {
            Self::VideoQueueFamilyProperties2KHR(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_KHR_video_queue")]
impl From<QueueFamilyQueryResultStatusProperties2KHR> for QueueFamilyProperties2Extension {
    fn from(ext: QueueFamilyQueryResultStatusProperties2KHR) -> Self {
        Self::QueueFamilyQueryResultStatusProperties2KHR(ext)
    }
}
#[cfg(feature = "VK_KHR_video_queue")]
impl TryInto<QueueFamilyQueryResultStatusProperties2KHR> for QueueFamilyProperties2Extension {
    type Error = QueueFamilyProperties2Extension;
    fn try_into(self) -> Result<QueueFamilyQueryResultStatusProperties2KHR, Self::Error> {
        match self {
            Self::QueueFamilyQueryResultStatusProperties2KHR(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[doc(alias = "VkPhysicalDeviceMemoryProperties2")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceMemoryProperties2 {
    #[doc(alias = "pNext")]
    pub extensions: SmallVec<[PhysicalDeviceMemoryProperties2Extension; 1]>,
    #[doc(alias = "memoryProperties")]
    pub memory_properties: PhysicalDeviceMemoryProperties,
}
impl PhysicalDeviceMemoryProperties2 {
    ///Adds an extension to the struct
    pub fn with_extension(mut self, ext: impl Into<PhysicalDeviceMemoryProperties2Extension>) -> Self {
        self.extensions.push(ext.into());
        self
    }
    ///Get a reference to the `extensions` field.
    pub fn extensions(&self) -> &SmallVec<[PhysicalDeviceMemoryProperties2Extension; 1]> {
        &self.extensions
    }
    ///Get a reference to the `memory_properties` field.
    pub fn memory_properties(&self) -> &PhysicalDeviceMemoryProperties {
        &self.memory_properties
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceMemoryProperties2 {
    type LowLevel = crate::native::vulkan1_1::PhysicalDeviceMemoryProperties2;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let mut next = std::ptr::null_mut();
        let mut extensions = self.extensions.iter();
        while let Some(ext) = extensions.next() {
            let ext = ext.into_low_level(context, bump);
            (*ext).next = next;
            next = ext;
        }
        crate::native::vulkan1_1::PhysicalDeviceMemoryProperties2 {
            s_type: StructureType::PhysicalDeviceMemoryProperties2,
            p_next: next,
            memory_properties: self.memory_properties.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceMemoryProperties2 {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let mut next = value.p_next;
        let mut extensions = SmallVec::new();
        while !next.is_null() {
            extensions.push(crate::conv::FromLowLevel::from_low_level(context, next));
            next = std::ptr::read(next).next;
        }
        Self {
            extensions: extensions,
            memory_properties: crate::conv::FromLowLevel::from_low_level(context, value.memory_properties),
        }
    }
}
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
///Extensions for [`PhysicalDeviceMemoryProperties2`]
pub enum PhysicalDeviceMemoryProperties2Extension {
    #[cfg(feature = "VK_EXT_memory_budget")]
    ///Contains a type [`PhysicalDeviceMemoryBudgetPropertiesEXT`] for extending
    /// [`PhysicalDeviceMemoryProperties2`]
    PhysicalDeviceMemoryBudgetPropertiesEXT(PhysicalDeviceMemoryBudgetPropertiesEXT),
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceMemoryProperties2Extension {
    type LowLevel = *mut crate::native::vulkan1_0::BaseOutStructure;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        match self {
            #[cfg(feature = "VK_EXT_memory_budget")]
            Self::PhysicalDeviceMemoryBudgetPropertiesEXT(ext) => (bump.alloc(ext.into_low_level(context, bump))
                as *mut crate::native::extensions::ext_memory_budget::PhysicalDeviceMemoryBudgetPropertiesEXT)
                .cast(),
            other => unreachable!("unexpected variant {:?}", other),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceMemoryProperties2Extension {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        assert!(!value.is_null());
        match (* value) . s_type { # [cfg (feature = "VK_EXT_memory_budget")] crate :: native :: vulkan1_0 :: StructureType :: PhysicalDeviceMemoryBudgetPropertiesExt => Self :: PhysicalDeviceMemoryBudgetPropertiesEXT (PhysicalDeviceMemoryBudgetPropertiesEXT :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: ext_memory_budget :: PhysicalDeviceMemoryBudgetPropertiesEXT > ()))) , other => panic ! ("Structure type {:?} is not a member of {}" , other , stringify ! (PhysicalDeviceMemoryProperties2)) }
    }
}
#[cfg(feature = "VK_EXT_memory_budget")]
impl From<PhysicalDeviceMemoryBudgetPropertiesEXT> for PhysicalDeviceMemoryProperties2Extension {
    fn from(ext: PhysicalDeviceMemoryBudgetPropertiesEXT) -> Self {
        Self::PhysicalDeviceMemoryBudgetPropertiesEXT(ext)
    }
}
#[cfg(feature = "VK_EXT_memory_budget")]
impl TryInto<PhysicalDeviceMemoryBudgetPropertiesEXT> for PhysicalDeviceMemoryProperties2Extension {
    type Error = PhysicalDeviceMemoryProperties2Extension;
    fn try_into(self) -> Result<PhysicalDeviceMemoryBudgetPropertiesEXT, Self::Error> {
        match self {
            Self::PhysicalDeviceMemoryBudgetPropertiesEXT(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[doc(alias = "VkSparseImageFormatProperties2")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SparseImageFormatProperties2 {
    pub properties: SparseImageFormatProperties,
}
impl SparseImageFormatProperties2 {
    ///Get a reference to the `properties` field.
    pub fn properties(&self) -> SparseImageFormatProperties {
        self.properties
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for SparseImageFormatProperties2 {
    type LowLevel = crate::native::vulkan1_1::SparseImageFormatProperties2;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_1::SparseImageFormatProperties2 {
            s_type: StructureType::SparseImageFormatProperties2,
            p_next: std::ptr::null_mut(),
            properties: self.properties.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for SparseImageFormatProperties2 {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            properties: crate::conv::FromLowLevel::from_low_level(context, value.properties),
        }
    }
}
#[doc(alias = "VkPhysicalDeviceSparseImageFormatInfo2")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceSparseImageFormatInfo2 {
    pub format: Format,
    #[doc(alias = "type")]
    pub type_: ImageType,
    pub samples: SampleCountFlagBits,
    pub usage: ImageUsageFlags,
    pub tiling: ImageTiling,
}
impl PhysicalDeviceSparseImageFormatInfo2 {
    ///Get a reference to the `format` field.
    pub fn format(&self) -> Format {
        self.format
    }
    ///Get a reference to the `type_` field.
    pub fn type_(&self) -> ImageType {
        self.type_
    }
    ///Get a reference to the `samples` field.
    pub fn samples(&self) -> SampleCountFlagBits {
        self.samples
    }
    ///Get a reference to the `usage` field.
    pub fn usage(&self) -> ImageUsageFlags {
        self.usage
    }
    ///Get a reference to the `tiling` field.
    pub fn tiling(&self) -> ImageTiling {
        self.tiling
    }
    ///Get a mutable reference to the `format` field.
    pub fn format_mut(&mut self) -> &mut Format {
        &mut self.format
    }
    ///Get a mutable reference to the `type_` field.
    pub fn type__mut(&mut self) -> &mut ImageType {
        &mut self.type_
    }
    ///Get a mutable reference to the `samples` field.
    pub fn samples_mut(&mut self) -> &mut SampleCountFlagBits {
        &mut self.samples
    }
    ///Get a mutable reference to the `usage` field.
    pub fn usage_mut(&mut self) -> &mut ImageUsageFlags {
        &mut self.usage
    }
    ///Get a mutable reference to the `tiling` field.
    pub fn tiling_mut(&mut self) -> &mut ImageTiling {
        &mut self.tiling
    }
    ///Sets the `format` field.
    pub fn set_format(&mut self, format: Format) -> &mut Self {
        self.format = format;
        self
    }
    ///Sets the `type_` field.
    pub fn set_type_(&mut self, type_: ImageType) -> &mut Self {
        self.type_ = type_;
        self
    }
    ///Sets the `samples` field.
    pub fn set_samples(&mut self, samples: SampleCountFlagBits) -> &mut Self {
        self.samples = samples;
        self
    }
    ///Sets the `usage` field.
    pub fn set_usage(&mut self, usage: ImageUsageFlags) -> &mut Self {
        self.usage = usage;
        self
    }
    ///Sets the `tiling` field.
    pub fn set_tiling(&mut self, tiling: ImageTiling) -> &mut Self {
        self.tiling = tiling;
        self
    }
    ///Sets the `format` field in a builder way.
    pub fn with_format(mut self, format: Format) -> Self {
        self.format = format;
        self
    }
    ///Sets the `type_` field in a builder way.
    pub fn with_type_(mut self, type_: ImageType) -> Self {
        self.type_ = type_;
        self
    }
    ///Sets the `samples` field in a builder way.
    pub fn with_samples(mut self, samples: SampleCountFlagBits) -> Self {
        self.samples = samples;
        self
    }
    ///Sets the `usage` field in a builder way.
    pub fn with_usage(mut self, usage: ImageUsageFlags) -> Self {
        self.usage = usage;
        self
    }
    ///Sets the `tiling` field in a builder way.
    pub fn with_tiling(mut self, tiling: ImageTiling) -> Self {
        self.tiling = tiling;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceSparseImageFormatInfo2 {
    type LowLevel = crate::native::vulkan1_1::PhysicalDeviceSparseImageFormatInfo2;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_1::PhysicalDeviceSparseImageFormatInfo2 {
            s_type: StructureType::PhysicalDeviceSparseImageFormatInfo2,
            p_next: std::ptr::null(),
            format: self.format.into_low_level(context, bump),
            type_: self.type_.into_low_level(context, bump),
            samples: self.samples.into_low_level(context, bump),
            usage: self.usage.into_low_level(context, bump),
            tiling: self.tiling.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceSparseImageFormatInfo2 {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            format: crate::conv::FromLowLevel::from_low_level(context, value.format),
            type_: crate::conv::FromLowLevel::from_low_level(context, value.type_),
            samples: crate::conv::FromLowLevel::from_low_level(context, value.samples),
            usage: crate::conv::FromLowLevel::from_low_level(context, value.usage),
            tiling: crate::conv::FromLowLevel::from_low_level(context, value.tiling),
        }
    }
}
#[doc(alias = "VkPhysicalDeviceVariablePointersFeatures")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceVariablePointersFeatures {
    #[doc(alias = "variablePointersStorageBuffer")]
    pub variable_pointers_storage_buffer: bool,
    #[doc(alias = "variablePointers")]
    pub variable_pointers: bool,
}
impl PhysicalDeviceVariablePointersFeatures {
    ///Get a reference to the `variable_pointers_storage_buffer` field.
    pub fn variable_pointers_storage_buffer(&self) -> &bool {
        &self.variable_pointers_storage_buffer
    }
    ///Get a reference to the `variable_pointers` field.
    pub fn variable_pointers(&self) -> &bool {
        &self.variable_pointers
    }
    ///Get a mutable reference to the `variable_pointers_storage_buffer` field.
    pub fn variable_pointers_storage_buffer_mut(&mut self) -> &mut bool {
        &mut self.variable_pointers_storage_buffer
    }
    ///Get a mutable reference to the `variable_pointers` field.
    pub fn variable_pointers_mut(&mut self) -> &mut bool {
        &mut self.variable_pointers
    }
    ///Sets the `variable_pointers_storage_buffer` field.
    pub fn set_variable_pointers_storage_buffer(&mut self, variable_pointers_storage_buffer: bool) -> &mut Self {
        self.variable_pointers_storage_buffer = variable_pointers_storage_buffer;
        self
    }
    ///Sets the `variable_pointers` field.
    pub fn set_variable_pointers(&mut self, variable_pointers: bool) -> &mut Self {
        self.variable_pointers = variable_pointers;
        self
    }
    ///Sets the `variable_pointers_storage_buffer` field in a builder way.
    pub fn with_variable_pointers_storage_buffer(mut self, variable_pointers_storage_buffer: bool) -> Self {
        self.variable_pointers_storage_buffer = variable_pointers_storage_buffer;
        self
    }
    ///Sets the `variable_pointers` field in a builder way.
    pub fn with_variable_pointers(mut self, variable_pointers: bool) -> Self {
        self.variable_pointers = variable_pointers;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceVariablePointersFeatures {
    type LowLevel = crate::native::vulkan1_1::PhysicalDeviceVariablePointersFeatures;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_1::PhysicalDeviceVariablePointersFeatures {
            s_type: StructureType::PhysicalDeviceVariablePointersFeatures,
            p_next: std::ptr::null_mut(),
            variable_pointers_storage_buffer: self.variable_pointers_storage_buffer.into_low_level(context, bump),
            variable_pointers: self.variable_pointers.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceVariablePointersFeatures {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            variable_pointers_storage_buffer: crate::conv::FromLowLevel::from_low_level(
                context,
                value.variable_pointers_storage_buffer,
            ),
            variable_pointers: crate::conv::FromLowLevel::from_low_level(context, value.variable_pointers),
        }
    }
}
impl ExternalMemoryProperties {
    ///Get a reference to the `external_memory_features` field.
    pub fn external_memory_features(&self) -> ExternalMemoryFeatureFlags {
        self.external_memory_features
    }
    ///Get a reference to the `export_from_imported_handle_types` field.
    pub fn export_from_imported_handle_types(&self) -> ExternalMemoryHandleTypeFlags {
        self.export_from_imported_handle_types
    }
    ///Get a reference to the `compatible_handle_types` field.
    pub fn compatible_handle_types(&self) -> ExternalMemoryHandleTypeFlags {
        self.compatible_handle_types
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for ExternalMemoryProperties {
    type LowLevel = crate::native::vulkan1_1::ExternalMemoryProperties;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_1::ExternalMemoryProperties {
            external_memory_features: self.external_memory_features.into_low_level(context, bump),
            export_from_imported_handle_types: self.export_from_imported_handle_types.into_low_level(context, bump),
            compatible_handle_types: self.compatible_handle_types.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for ExternalMemoryProperties {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            external_memory_features: crate::conv::FromLowLevel::from_low_level(
                context,
                value.external_memory_features,
            ),
            export_from_imported_handle_types: crate::conv::FromLowLevel::from_low_level(
                context,
                value.export_from_imported_handle_types,
            ),
            compatible_handle_types: crate::conv::FromLowLevel::from_low_level(context, value.compatible_handle_types),
        }
    }
}
#[doc(alias = "VkPhysicalDeviceExternalImageFormatInfo")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceExternalImageFormatInfo {
    #[doc(alias = "handleType")]
    pub handle_type: ExternalMemoryHandleTypeFlagBits,
}
impl PhysicalDeviceExternalImageFormatInfo {
    ///Get a reference to the `handle_type` field.
    pub fn handle_type(&self) -> ExternalMemoryHandleTypeFlagBits {
        self.handle_type
    }
    ///Get a mutable reference to the `handle_type` field.
    pub fn handle_type_mut(&mut self) -> &mut ExternalMemoryHandleTypeFlagBits {
        &mut self.handle_type
    }
    ///Sets the `handle_type` field.
    pub fn set_handle_type(&mut self, handle_type: ExternalMemoryHandleTypeFlagBits) -> &mut Self {
        self.handle_type = handle_type;
        self
    }
    ///Sets the `handle_type` field in a builder way.
    pub fn with_handle_type(mut self, handle_type: ExternalMemoryHandleTypeFlagBits) -> Self {
        self.handle_type = handle_type;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceExternalImageFormatInfo {
    type LowLevel = crate::native::vulkan1_1::PhysicalDeviceExternalImageFormatInfo;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_1::PhysicalDeviceExternalImageFormatInfo {
            s_type: StructureType::PhysicalDeviceExternalImageFormatInfo,
            p_next: std::ptr::null(),
            handle_type: self.handle_type.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceExternalImageFormatInfo {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            handle_type: crate::conv::FromLowLevel::from_low_level(context, value.handle_type),
        }
    }
}
#[doc(alias = "VkExternalImageFormatProperties")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ExternalImageFormatProperties {
    #[doc(alias = "externalMemoryProperties")]
    pub external_memory_properties: ExternalMemoryProperties,
}
impl ExternalImageFormatProperties {
    ///Get a reference to the `external_memory_properties` field.
    pub fn external_memory_properties(&self) -> ExternalMemoryProperties {
        self.external_memory_properties
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for ExternalImageFormatProperties {
    type LowLevel = crate::native::vulkan1_1::ExternalImageFormatProperties;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_1::ExternalImageFormatProperties {
            s_type: StructureType::ExternalImageFormatProperties,
            p_next: std::ptr::null_mut(),
            external_memory_properties: self.external_memory_properties.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for ExternalImageFormatProperties {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            external_memory_properties: crate::conv::FromLowLevel::from_low_level(
                context,
                value.external_memory_properties,
            ),
        }
    }
}
#[doc(alias = "VkPhysicalDeviceExternalBufferInfo")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceExternalBufferInfo {
    pub flags: BufferCreateFlags,
    pub usage: BufferUsageFlags,
    #[doc(alias = "handleType")]
    pub handle_type: ExternalMemoryHandleTypeFlagBits,
}
impl PhysicalDeviceExternalBufferInfo {
    ///Get a reference to the `flags` field.
    pub fn flags(&self) -> BufferCreateFlags {
        self.flags
    }
    ///Get a reference to the `usage` field.
    pub fn usage(&self) -> BufferUsageFlags {
        self.usage
    }
    ///Get a reference to the `handle_type` field.
    pub fn handle_type(&self) -> ExternalMemoryHandleTypeFlagBits {
        self.handle_type
    }
    ///Get a mutable reference to the `flags` field.
    pub fn flags_mut(&mut self) -> &mut BufferCreateFlags {
        &mut self.flags
    }
    ///Get a mutable reference to the `usage` field.
    pub fn usage_mut(&mut self) -> &mut BufferUsageFlags {
        &mut self.usage
    }
    ///Get a mutable reference to the `handle_type` field.
    pub fn handle_type_mut(&mut self) -> &mut ExternalMemoryHandleTypeFlagBits {
        &mut self.handle_type
    }
    ///Sets the `flags` field.
    pub fn set_flags(&mut self, flags: BufferCreateFlags) -> &mut Self {
        self.flags = flags;
        self
    }
    ///Sets the `usage` field.
    pub fn set_usage(&mut self, usage: BufferUsageFlags) -> &mut Self {
        self.usage = usage;
        self
    }
    ///Sets the `handle_type` field.
    pub fn set_handle_type(&mut self, handle_type: ExternalMemoryHandleTypeFlagBits) -> &mut Self {
        self.handle_type = handle_type;
        self
    }
    ///Sets the `flags` field in a builder way.
    pub fn with_flags(mut self, flags: BufferCreateFlags) -> Self {
        self.flags = flags;
        self
    }
    ///Sets the `usage` field in a builder way.
    pub fn with_usage(mut self, usage: BufferUsageFlags) -> Self {
        self.usage = usage;
        self
    }
    ///Sets the `handle_type` field in a builder way.
    pub fn with_handle_type(mut self, handle_type: ExternalMemoryHandleTypeFlagBits) -> Self {
        self.handle_type = handle_type;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceExternalBufferInfo {
    type LowLevel = crate::native::vulkan1_1::PhysicalDeviceExternalBufferInfo;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_1::PhysicalDeviceExternalBufferInfo {
            s_type: StructureType::PhysicalDeviceExternalBufferInfo,
            p_next: std::ptr::null(),
            flags: self.flags.into_low_level(context, bump),
            usage: self.usage.into_low_level(context, bump),
            handle_type: self.handle_type.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceExternalBufferInfo {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            flags: crate::conv::FromLowLevel::from_low_level(context, value.flags),
            usage: crate::conv::FromLowLevel::from_low_level(context, value.usage),
            handle_type: crate::conv::FromLowLevel::from_low_level(context, value.handle_type),
        }
    }
}
#[doc(alias = "VkExternalBufferProperties")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ExternalBufferProperties {
    #[doc(alias = "externalMemoryProperties")]
    pub external_memory_properties: ExternalMemoryProperties,
}
impl ExternalBufferProperties {
    ///Get a reference to the `external_memory_properties` field.
    pub fn external_memory_properties(&self) -> ExternalMemoryProperties {
        self.external_memory_properties
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for ExternalBufferProperties {
    type LowLevel = crate::native::vulkan1_1::ExternalBufferProperties;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_1::ExternalBufferProperties {
            s_type: StructureType::ExternalBufferProperties,
            p_next: std::ptr::null_mut(),
            external_memory_properties: self.external_memory_properties.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for ExternalBufferProperties {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            external_memory_properties: crate::conv::FromLowLevel::from_low_level(
                context,
                value.external_memory_properties,
            ),
        }
    }
}
#[doc(alias = "VkPhysicalDeviceIDProperties")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceIdProperties {
    #[doc(alias = "deviceUUID")]
    pub device_uuid: uuid::Uuid,
    #[doc(alias = "driverUUID")]
    pub driver_uuid: uuid::Uuid,
    #[doc(alias = "deviceLUID")]
    pub device_luid: [u8; LUID_SIZE as usize],
    #[doc(alias = "deviceNodeMask")]
    pub device_node_mask: u32,
    #[doc(alias = "deviceLUIDValid")]
    pub device_luid_valid: bool,
}
impl PhysicalDeviceIdProperties {
    ///Get a reference to the `device_uuid` field.
    pub fn device_uuid(&self) -> uuid::Uuid {
        self.device_uuid
    }
    ///Get a reference to the `driver_uuid` field.
    pub fn driver_uuid(&self) -> uuid::Uuid {
        self.driver_uuid
    }
    ///Get a reference to the `device_luid` field.
    pub fn device_luid(&self) -> [u8; LUID_SIZE as usize] {
        self.device_luid
    }
    ///Get a reference to the `device_node_mask` field.
    pub fn device_node_mask(&self) -> u32 {
        self.device_node_mask
    }
    ///Get a reference to the `device_luid_valid` field.
    pub fn device_luid_valid(&self) -> &bool {
        &self.device_luid_valid
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceIdProperties {
    type LowLevel = crate::native::vulkan1_1::PhysicalDeviceIdProperties;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_1::PhysicalDeviceIdProperties {
            s_type: StructureType::PhysicalDeviceIdProperties,
            p_next: std::ptr::null_mut(),
            device_uuid: self.device_uuid.into_low_level(context, bump),
            driver_uuid: self.driver_uuid.into_low_level(context, bump),
            device_luid: self.device_luid.into_low_level(context, bump),
            device_node_mask: self.device_node_mask.into_low_level(context, bump),
            device_luid_valid: self.device_luid_valid.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceIdProperties {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            device_uuid: crate::conv::FromLowLevel::from_low_level(context, value.device_uuid),
            driver_uuid: crate::conv::FromLowLevel::from_low_level(context, value.driver_uuid),
            device_luid: crate::conv::FromLowLevel::from_low_level(context, value.device_luid),
            device_node_mask: crate::conv::FromLowLevel::from_low_level(context, value.device_node_mask),
            device_luid_valid: crate::conv::FromLowLevel::from_low_level(context, value.device_luid_valid),
        }
    }
}
#[doc(alias = "VkExternalMemoryImageCreateInfo")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ExternalMemoryImageCreateInfo {
    #[doc(alias = "handleTypes")]
    pub handle_types: ExternalMemoryHandleTypeFlags,
}
impl ExternalMemoryImageCreateInfo {
    ///Get a reference to the `handle_types` field.
    pub fn handle_types(&self) -> ExternalMemoryHandleTypeFlags {
        self.handle_types
    }
    ///Get a mutable reference to the `handle_types` field.
    pub fn handle_types_mut(&mut self) -> &mut ExternalMemoryHandleTypeFlags {
        &mut self.handle_types
    }
    ///Sets the `handle_types` field.
    pub fn set_handle_types(&mut self, handle_types: ExternalMemoryHandleTypeFlags) -> &mut Self {
        self.handle_types = handle_types;
        self
    }
    ///Sets the `handle_types` field in a builder way.
    pub fn with_handle_types(mut self, handle_types: ExternalMemoryHandleTypeFlags) -> Self {
        self.handle_types = handle_types;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for ExternalMemoryImageCreateInfo {
    type LowLevel = crate::native::vulkan1_1::ExternalMemoryImageCreateInfo;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_1::ExternalMemoryImageCreateInfo {
            s_type: StructureType::ExternalMemoryImageCreateInfo,
            p_next: std::ptr::null(),
            handle_types: self.handle_types.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for ExternalMemoryImageCreateInfo {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            handle_types: crate::conv::FromLowLevel::from_low_level(context, value.handle_types),
        }
    }
}
#[doc(alias = "VkExternalMemoryBufferCreateInfo")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ExternalMemoryBufferCreateInfo {
    #[doc(alias = "handleTypes")]
    pub handle_types: ExternalMemoryHandleTypeFlags,
}
impl ExternalMemoryBufferCreateInfo {
    ///Get a reference to the `handle_types` field.
    pub fn handle_types(&self) -> ExternalMemoryHandleTypeFlags {
        self.handle_types
    }
    ///Get a mutable reference to the `handle_types` field.
    pub fn handle_types_mut(&mut self) -> &mut ExternalMemoryHandleTypeFlags {
        &mut self.handle_types
    }
    ///Sets the `handle_types` field.
    pub fn set_handle_types(&mut self, handle_types: ExternalMemoryHandleTypeFlags) -> &mut Self {
        self.handle_types = handle_types;
        self
    }
    ///Sets the `handle_types` field in a builder way.
    pub fn with_handle_types(mut self, handle_types: ExternalMemoryHandleTypeFlags) -> Self {
        self.handle_types = handle_types;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for ExternalMemoryBufferCreateInfo {
    type LowLevel = crate::native::vulkan1_1::ExternalMemoryBufferCreateInfo;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_1::ExternalMemoryBufferCreateInfo {
            s_type: StructureType::ExternalMemoryBufferCreateInfo,
            p_next: std::ptr::null(),
            handle_types: self.handle_types.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for ExternalMemoryBufferCreateInfo {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            handle_types: crate::conv::FromLowLevel::from_low_level(context, value.handle_types),
        }
    }
}
#[doc(alias = "VkExportMemoryAllocateInfo")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ExportMemoryAllocateInfo {
    #[doc(alias = "handleTypes")]
    pub handle_types: ExternalMemoryHandleTypeFlags,
}
impl ExportMemoryAllocateInfo {
    ///Get a reference to the `handle_types` field.
    pub fn handle_types(&self) -> ExternalMemoryHandleTypeFlags {
        self.handle_types
    }
    ///Get a mutable reference to the `handle_types` field.
    pub fn handle_types_mut(&mut self) -> &mut ExternalMemoryHandleTypeFlags {
        &mut self.handle_types
    }
    ///Sets the `handle_types` field.
    pub fn set_handle_types(&mut self, handle_types: ExternalMemoryHandleTypeFlags) -> &mut Self {
        self.handle_types = handle_types;
        self
    }
    ///Sets the `handle_types` field in a builder way.
    pub fn with_handle_types(mut self, handle_types: ExternalMemoryHandleTypeFlags) -> Self {
        self.handle_types = handle_types;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for ExportMemoryAllocateInfo {
    type LowLevel = crate::native::vulkan1_1::ExportMemoryAllocateInfo;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_1::ExportMemoryAllocateInfo {
            s_type: StructureType::ExportMemoryAllocateInfo,
            p_next: std::ptr::null(),
            handle_types: self.handle_types.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for ExportMemoryAllocateInfo {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            handle_types: crate::conv::FromLowLevel::from_low_level(context, value.handle_types),
        }
    }
}
#[doc(alias = "VkPhysicalDeviceExternalSemaphoreInfo")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceExternalSemaphoreInfo {
    #[doc(alias = "pNext")]
    pub extensions: SmallVec<[PhysicalDeviceExternalSemaphoreInfoExtension; 1]>,
    #[doc(alias = "handleType")]
    pub handle_type: ExternalSemaphoreHandleTypeFlagBits,
}
impl PhysicalDeviceExternalSemaphoreInfo {
    ///Adds an extension to the struct
    pub fn with_extension(mut self, ext: impl Into<PhysicalDeviceExternalSemaphoreInfoExtension>) -> Self {
        self.extensions.push(ext.into());
        self
    }
    ///Get a reference to the `extensions` field.
    pub fn extensions(&self) -> &SmallVec<[PhysicalDeviceExternalSemaphoreInfoExtension; 1]> {
        &self.extensions
    }
    ///Get a reference to the `handle_type` field.
    pub fn handle_type(&self) -> ExternalSemaphoreHandleTypeFlagBits {
        self.handle_type
    }
    ///Get a mutable reference to the `extensions` field.
    pub fn extensions_mut(&mut self) -> &mut SmallVec<[PhysicalDeviceExternalSemaphoreInfoExtension; 1]> {
        &mut self.extensions
    }
    ///Get a mutable reference to the `handle_type` field.
    pub fn handle_type_mut(&mut self) -> &mut ExternalSemaphoreHandleTypeFlagBits {
        &mut self.handle_type
    }
    ///Sets the `extensions` field.
    pub fn set_extensions(
        &mut self,
        extensions: SmallVec<[PhysicalDeviceExternalSemaphoreInfoExtension; 1]>,
    ) -> &mut Self {
        self.extensions = extensions;
        self
    }
    ///Sets the `handle_type` field.
    pub fn set_handle_type(&mut self, handle_type: ExternalSemaphoreHandleTypeFlagBits) -> &mut Self {
        self.handle_type = handle_type;
        self
    }
    ///Sets the `extensions` field in a builder way.
    pub fn with_extensions(mut self, extensions: SmallVec<[PhysicalDeviceExternalSemaphoreInfoExtension; 1]>) -> Self {
        self.extensions = extensions;
        self
    }
    ///Sets the `handle_type` field in a builder way.
    pub fn with_handle_type(mut self, handle_type: ExternalSemaphoreHandleTypeFlagBits) -> Self {
        self.handle_type = handle_type;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceExternalSemaphoreInfo {
    type LowLevel = crate::native::vulkan1_1::PhysicalDeviceExternalSemaphoreInfo;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let mut next = std::ptr::null();
        let mut extensions = self.extensions.iter();
        while let Some(ext) = extensions.next() {
            let ext = ext.into_low_level(context, bump);
            (*ext).next = next;
            next = ext;
        }
        crate::native::vulkan1_1::PhysicalDeviceExternalSemaphoreInfo {
            s_type: StructureType::PhysicalDeviceExternalSemaphoreInfo,
            p_next: next,
            handle_type: self.handle_type.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceExternalSemaphoreInfo {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let mut next = value.p_next;
        let mut extensions = SmallVec::new();
        while !next.is_null() {
            extensions.push(crate::conv::FromLowLevel::from_low_level(context, next));
            next = std::ptr::read(next).next;
        }
        Self {
            extensions: extensions,
            handle_type: crate::conv::FromLowLevel::from_low_level(context, value.handle_type),
        }
    }
}
#[derive(Clone, PartialEq, Debug, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
///Extensions for [`PhysicalDeviceExternalSemaphoreInfo`]
pub enum PhysicalDeviceExternalSemaphoreInfoExtension {
    #[cfg(feature = "VULKAN_1_2")]
    ///Contains a type [`SemaphoreTypeCreateInfo`] for extending
    /// [`PhysicalDeviceExternalSemaphoreInfo`]
    SemaphoreTypeCreateInfo(SemaphoreTypeCreateInfo),
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceExternalSemaphoreInfoExtension {
    type LowLevel = *const crate::native::vulkan1_0::BaseInStructure;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        match self {
            #[cfg(feature = "VULKAN_1_2")]
            Self::SemaphoreTypeCreateInfo(ext) => (bump.alloc(ext.into_low_level(context, bump))
                as *mut crate::native::vulkan1_2::SemaphoreTypeCreateInfo)
                .cast(),
            other => unreachable!("unexpected variant {:?}", other),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceExternalSemaphoreInfoExtension {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        assert!(!value.is_null());
        match (*value).s_type {
            #[cfg(feature = "VULKAN_1_2")]
            crate::native::vulkan1_0::StructureType::SemaphoreTypeCreateInfo => {
                Self::SemaphoreTypeCreateInfo(SemaphoreTypeCreateInfo::from_low_level(
                    context,
                    std::ptr::read(value.cast::<crate::native::vulkan1_2::SemaphoreTypeCreateInfo>()),
                ))
            },
            other => panic!(
                "Structure type {:?} is not a member of {}",
                other,
                stringify!(PhysicalDeviceExternalSemaphoreInfo)
            ),
        }
    }
}
#[cfg(feature = "VULKAN_1_2")]
impl From<SemaphoreTypeCreateInfo> for PhysicalDeviceExternalSemaphoreInfoExtension {
    fn from(ext: SemaphoreTypeCreateInfo) -> Self {
        Self::SemaphoreTypeCreateInfo(ext)
    }
}
#[cfg(feature = "VULKAN_1_2")]
impl TryInto<SemaphoreTypeCreateInfo> for PhysicalDeviceExternalSemaphoreInfoExtension {
    type Error = PhysicalDeviceExternalSemaphoreInfoExtension;
    fn try_into(self) -> Result<SemaphoreTypeCreateInfo, Self::Error> {
        match self {
            Self::SemaphoreTypeCreateInfo(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[doc(alias = "VkExternalSemaphoreProperties")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ExternalSemaphoreProperties {
    #[doc(alias = "exportFromImportedHandleTypes")]
    pub export_from_imported_handle_types: ExternalSemaphoreHandleTypeFlags,
    #[doc(alias = "compatibleHandleTypes")]
    pub compatible_handle_types: ExternalSemaphoreHandleTypeFlags,
    #[doc(alias = "externalSemaphoreFeatures")]
    pub external_semaphore_features: ExternalSemaphoreFeatureFlags,
}
impl ExternalSemaphoreProperties {
    ///Get a reference to the `export_from_imported_handle_types` field.
    pub fn export_from_imported_handle_types(&self) -> ExternalSemaphoreHandleTypeFlags {
        self.export_from_imported_handle_types
    }
    ///Get a reference to the `compatible_handle_types` field.
    pub fn compatible_handle_types(&self) -> ExternalSemaphoreHandleTypeFlags {
        self.compatible_handle_types
    }
    ///Get a reference to the `external_semaphore_features` field.
    pub fn external_semaphore_features(&self) -> ExternalSemaphoreFeatureFlags {
        self.external_semaphore_features
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for ExternalSemaphoreProperties {
    type LowLevel = crate::native::vulkan1_1::ExternalSemaphoreProperties;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_1::ExternalSemaphoreProperties {
            s_type: StructureType::ExternalSemaphoreProperties,
            p_next: std::ptr::null_mut(),
            export_from_imported_handle_types: self.export_from_imported_handle_types.into_low_level(context, bump),
            compatible_handle_types: self.compatible_handle_types.into_low_level(context, bump),
            external_semaphore_features: self.external_semaphore_features.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for ExternalSemaphoreProperties {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            export_from_imported_handle_types: crate::conv::FromLowLevel::from_low_level(
                context,
                value.export_from_imported_handle_types,
            ),
            compatible_handle_types: crate::conv::FromLowLevel::from_low_level(context, value.compatible_handle_types),
            external_semaphore_features: crate::conv::FromLowLevel::from_low_level(
                context,
                value.external_semaphore_features,
            ),
        }
    }
}
#[doc(alias = "VkExportSemaphoreCreateInfo")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ExportSemaphoreCreateInfo {
    #[doc(alias = "handleTypes")]
    pub handle_types: ExternalSemaphoreHandleTypeFlags,
}
impl ExportSemaphoreCreateInfo {
    ///Get a reference to the `handle_types` field.
    pub fn handle_types(&self) -> ExternalSemaphoreHandleTypeFlags {
        self.handle_types
    }
    ///Get a mutable reference to the `handle_types` field.
    pub fn handle_types_mut(&mut self) -> &mut ExternalSemaphoreHandleTypeFlags {
        &mut self.handle_types
    }
    ///Sets the `handle_types` field.
    pub fn set_handle_types(&mut self, handle_types: ExternalSemaphoreHandleTypeFlags) -> &mut Self {
        self.handle_types = handle_types;
        self
    }
    ///Sets the `handle_types` field in a builder way.
    pub fn with_handle_types(mut self, handle_types: ExternalSemaphoreHandleTypeFlags) -> Self {
        self.handle_types = handle_types;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for ExportSemaphoreCreateInfo {
    type LowLevel = crate::native::vulkan1_1::ExportSemaphoreCreateInfo;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_1::ExportSemaphoreCreateInfo {
            s_type: StructureType::ExportSemaphoreCreateInfo,
            p_next: std::ptr::null(),
            handle_types: self.handle_types.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for ExportSemaphoreCreateInfo {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            handle_types: crate::conv::FromLowLevel::from_low_level(context, value.handle_types),
        }
    }
}
#[doc(alias = "VkPhysicalDeviceExternalFenceInfo")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceExternalFenceInfo {
    #[doc(alias = "handleType")]
    pub handle_type: ExternalFenceHandleTypeFlagBits,
}
impl PhysicalDeviceExternalFenceInfo {
    ///Get a reference to the `handle_type` field.
    pub fn handle_type(&self) -> ExternalFenceHandleTypeFlagBits {
        self.handle_type
    }
    ///Get a mutable reference to the `handle_type` field.
    pub fn handle_type_mut(&mut self) -> &mut ExternalFenceHandleTypeFlagBits {
        &mut self.handle_type
    }
    ///Sets the `handle_type` field.
    pub fn set_handle_type(&mut self, handle_type: ExternalFenceHandleTypeFlagBits) -> &mut Self {
        self.handle_type = handle_type;
        self
    }
    ///Sets the `handle_type` field in a builder way.
    pub fn with_handle_type(mut self, handle_type: ExternalFenceHandleTypeFlagBits) -> Self {
        self.handle_type = handle_type;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceExternalFenceInfo {
    type LowLevel = crate::native::vulkan1_1::PhysicalDeviceExternalFenceInfo;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_1::PhysicalDeviceExternalFenceInfo {
            s_type: StructureType::PhysicalDeviceExternalFenceInfo,
            p_next: std::ptr::null(),
            handle_type: self.handle_type.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceExternalFenceInfo {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            handle_type: crate::conv::FromLowLevel::from_low_level(context, value.handle_type),
        }
    }
}
#[doc(alias = "VkExternalFenceProperties")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ExternalFenceProperties {
    #[doc(alias = "exportFromImportedHandleTypes")]
    pub export_from_imported_handle_types: ExternalFenceHandleTypeFlags,
    #[doc(alias = "compatibleHandleTypes")]
    pub compatible_handle_types: ExternalFenceHandleTypeFlags,
    #[doc(alias = "externalFenceFeatures")]
    pub external_fence_features: ExternalFenceFeatureFlags,
}
impl ExternalFenceProperties {
    ///Get a reference to the `export_from_imported_handle_types` field.
    pub fn export_from_imported_handle_types(&self) -> ExternalFenceHandleTypeFlags {
        self.export_from_imported_handle_types
    }
    ///Get a reference to the `compatible_handle_types` field.
    pub fn compatible_handle_types(&self) -> ExternalFenceHandleTypeFlags {
        self.compatible_handle_types
    }
    ///Get a reference to the `external_fence_features` field.
    pub fn external_fence_features(&self) -> ExternalFenceFeatureFlags {
        self.external_fence_features
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for ExternalFenceProperties {
    type LowLevel = crate::native::vulkan1_1::ExternalFenceProperties;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_1::ExternalFenceProperties {
            s_type: StructureType::ExternalFenceProperties,
            p_next: std::ptr::null_mut(),
            export_from_imported_handle_types: self.export_from_imported_handle_types.into_low_level(context, bump),
            compatible_handle_types: self.compatible_handle_types.into_low_level(context, bump),
            external_fence_features: self.external_fence_features.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for ExternalFenceProperties {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            export_from_imported_handle_types: crate::conv::FromLowLevel::from_low_level(
                context,
                value.export_from_imported_handle_types,
            ),
            compatible_handle_types: crate::conv::FromLowLevel::from_low_level(context, value.compatible_handle_types),
            external_fence_features: crate::conv::FromLowLevel::from_low_level(context, value.external_fence_features),
        }
    }
}
#[doc(alias = "VkExportFenceCreateInfo")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ExportFenceCreateInfo {
    #[doc(alias = "handleTypes")]
    pub handle_types: ExternalFenceHandleTypeFlags,
}
impl ExportFenceCreateInfo {
    ///Get a reference to the `handle_types` field.
    pub fn handle_types(&self) -> ExternalFenceHandleTypeFlags {
        self.handle_types
    }
    ///Get a mutable reference to the `handle_types` field.
    pub fn handle_types_mut(&mut self) -> &mut ExternalFenceHandleTypeFlags {
        &mut self.handle_types
    }
    ///Sets the `handle_types` field.
    pub fn set_handle_types(&mut self, handle_types: ExternalFenceHandleTypeFlags) -> &mut Self {
        self.handle_types = handle_types;
        self
    }
    ///Sets the `handle_types` field in a builder way.
    pub fn with_handle_types(mut self, handle_types: ExternalFenceHandleTypeFlags) -> Self {
        self.handle_types = handle_types;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for ExportFenceCreateInfo {
    type LowLevel = crate::native::vulkan1_1::ExportFenceCreateInfo;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_1::ExportFenceCreateInfo {
            s_type: StructureType::ExportFenceCreateInfo,
            p_next: std::ptr::null(),
            handle_types: self.handle_types.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for ExportFenceCreateInfo {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            handle_types: crate::conv::FromLowLevel::from_low_level(context, value.handle_types),
        }
    }
}
#[doc(alias = "VkPhysicalDeviceMultiviewFeatures")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceMultiviewFeatures {
    pub multiview: bool,
    #[doc(alias = "multiviewGeometryShader")]
    pub multiview_geometry_shader: bool,
    #[doc(alias = "multiviewTessellationShader")]
    pub multiview_tessellation_shader: bool,
}
impl PhysicalDeviceMultiviewFeatures {
    ///Get a reference to the `multiview` field.
    pub fn multiview(&self) -> &bool {
        &self.multiview
    }
    ///Get a reference to the `multiview_geometry_shader` field.
    pub fn multiview_geometry_shader(&self) -> &bool {
        &self.multiview_geometry_shader
    }
    ///Get a reference to the `multiview_tessellation_shader` field.
    pub fn multiview_tessellation_shader(&self) -> &bool {
        &self.multiview_tessellation_shader
    }
    ///Get a mutable reference to the `multiview` field.
    pub fn multiview_mut(&mut self) -> &mut bool {
        &mut self.multiview
    }
    ///Get a mutable reference to the `multiview_geometry_shader` field.
    pub fn multiview_geometry_shader_mut(&mut self) -> &mut bool {
        &mut self.multiview_geometry_shader
    }
    ///Get a mutable reference to the `multiview_tessellation_shader` field.
    pub fn multiview_tessellation_shader_mut(&mut self) -> &mut bool {
        &mut self.multiview_tessellation_shader
    }
    ///Sets the `multiview` field.
    pub fn set_multiview(&mut self, multiview: bool) -> &mut Self {
        self.multiview = multiview;
        self
    }
    ///Sets the `multiview_geometry_shader` field.
    pub fn set_multiview_geometry_shader(&mut self, multiview_geometry_shader: bool) -> &mut Self {
        self.multiview_geometry_shader = multiview_geometry_shader;
        self
    }
    ///Sets the `multiview_tessellation_shader` field.
    pub fn set_multiview_tessellation_shader(&mut self, multiview_tessellation_shader: bool) -> &mut Self {
        self.multiview_tessellation_shader = multiview_tessellation_shader;
        self
    }
    ///Sets the `multiview` field in a builder way.
    pub fn with_multiview(mut self, multiview: bool) -> Self {
        self.multiview = multiview;
        self
    }
    ///Sets the `multiview_geometry_shader` field in a builder way.
    pub fn with_multiview_geometry_shader(mut self, multiview_geometry_shader: bool) -> Self {
        self.multiview_geometry_shader = multiview_geometry_shader;
        self
    }
    ///Sets the `multiview_tessellation_shader` field in a builder way.
    pub fn with_multiview_tessellation_shader(mut self, multiview_tessellation_shader: bool) -> Self {
        self.multiview_tessellation_shader = multiview_tessellation_shader;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceMultiviewFeatures {
    type LowLevel = crate::native::vulkan1_1::PhysicalDeviceMultiviewFeatures;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_1::PhysicalDeviceMultiviewFeatures {
            s_type: StructureType::PhysicalDeviceMultiviewFeatures,
            p_next: std::ptr::null_mut(),
            multiview: self.multiview.into_low_level(context, bump),
            multiview_geometry_shader: self.multiview_geometry_shader.into_low_level(context, bump),
            multiview_tessellation_shader: self.multiview_tessellation_shader.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceMultiviewFeatures {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            multiview: crate::conv::FromLowLevel::from_low_level(context, value.multiview),
            multiview_geometry_shader: crate::conv::FromLowLevel::from_low_level(
                context,
                value.multiview_geometry_shader,
            ),
            multiview_tessellation_shader: crate::conv::FromLowLevel::from_low_level(
                context,
                value.multiview_tessellation_shader,
            ),
        }
    }
}
#[doc(alias = "VkPhysicalDeviceMultiviewProperties")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceMultiviewProperties {
    #[doc(alias = "maxMultiviewViewCount")]
    pub max_multiview_view_count: u32,
    #[doc(alias = "maxMultiviewInstanceIndex")]
    pub max_multiview_instance_index: u32,
}
impl PhysicalDeviceMultiviewProperties {
    ///Get a reference to the `max_multiview_view_count` field.
    pub fn max_multiview_view_count(&self) -> u32 {
        self.max_multiview_view_count
    }
    ///Get a reference to the `max_multiview_instance_index` field.
    pub fn max_multiview_instance_index(&self) -> u32 {
        self.max_multiview_instance_index
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceMultiviewProperties {
    type LowLevel = crate::native::vulkan1_1::PhysicalDeviceMultiviewProperties;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_1::PhysicalDeviceMultiviewProperties {
            s_type: StructureType::PhysicalDeviceMultiviewProperties,
            p_next: std::ptr::null_mut(),
            max_multiview_view_count: self.max_multiview_view_count.into_low_level(context, bump),
            max_multiview_instance_index: self.max_multiview_instance_index.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceMultiviewProperties {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            max_multiview_view_count: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_multiview_view_count,
            ),
            max_multiview_instance_index: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_multiview_instance_index,
            ),
        }
    }
}
#[doc(alias = "VkRenderPassMultiviewCreateInfo")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RenderPassMultiviewCreateInfo {
    #[doc(alias = "pViewMasks")]
    pub view_masks: SmallVec<[u32; 8]>,
    #[doc(alias = "pViewOffsets")]
    pub view_offsets: SmallVec<[i32; 8]>,
    #[doc(alias = "pCorrelationMasks")]
    pub correlation_masks: SmallVec<[u32; 8]>,
}
impl RenderPassMultiviewCreateInfo {
    ///Get a reference to the `view_masks` field.
    pub fn view_masks(&self) -> &SmallVec<[u32; 8]> {
        &self.view_masks
    }
    ///Get a reference to the `view_offsets` field.
    pub fn view_offsets(&self) -> &SmallVec<[i32; 8]> {
        &self.view_offsets
    }
    ///Get a reference to the `correlation_masks` field.
    pub fn correlation_masks(&self) -> &SmallVec<[u32; 8]> {
        &self.correlation_masks
    }
    ///Get a mutable reference to the `view_masks` field.
    pub fn view_masks_mut(&mut self) -> &mut SmallVec<[u32; 8]> {
        &mut self.view_masks
    }
    ///Get a mutable reference to the `view_offsets` field.
    pub fn view_offsets_mut(&mut self) -> &mut SmallVec<[i32; 8]> {
        &mut self.view_offsets
    }
    ///Get a mutable reference to the `correlation_masks` field.
    pub fn correlation_masks_mut(&mut self) -> &mut SmallVec<[u32; 8]> {
        &mut self.correlation_masks
    }
    ///Sets the `view_masks` field.
    pub fn set_view_masks(&mut self, view_masks: SmallVec<[u32; 8]>) -> &mut Self {
        self.view_masks = view_masks;
        self
    }
    ///Sets the `view_offsets` field.
    pub fn set_view_offsets(&mut self, view_offsets: SmallVec<[i32; 8]>) -> &mut Self {
        self.view_offsets = view_offsets;
        self
    }
    ///Sets the `correlation_masks` field.
    pub fn set_correlation_masks(&mut self, correlation_masks: SmallVec<[u32; 8]>) -> &mut Self {
        self.correlation_masks = correlation_masks;
        self
    }
    ///Sets the `view_masks` field in a builder way.
    pub fn with_view_masks(mut self, view_masks: SmallVec<[u32; 8]>) -> Self {
        self.view_masks = view_masks;
        self
    }
    ///Sets the `view_offsets` field in a builder way.
    pub fn with_view_offsets(mut self, view_offsets: SmallVec<[i32; 8]>) -> Self {
        self.view_offsets = view_offsets;
        self
    }
    ///Sets the `correlation_masks` field in a builder way.
    pub fn with_correlation_masks(mut self, correlation_masks: SmallVec<[u32; 8]>) -> Self {
        self.correlation_masks = correlation_masks;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for RenderPassMultiviewCreateInfo {
    type LowLevel = crate::native::vulkan1_1::RenderPassMultiviewCreateInfo;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let len_view_masks = self.view_masks.len() as u32;
        let view_masks = bump
            .alloc_slice_fill_iter(self.view_masks.iter().map(|x| x.into_low_level(context, bump)))
            .as_ptr()
            .cast();
        let len_view_offsets = self.view_offsets.len() as u32;
        let view_offsets = bump
            .alloc_slice_fill_iter(self.view_offsets.iter().map(|x| x.into_low_level(context, bump)))
            .as_ptr()
            .cast();
        let len_correlation_masks = self.correlation_masks.len() as u32;
        let correlation_masks = bump
            .alloc_slice_fill_iter(self.correlation_masks.iter().map(|x| x.into_low_level(context, bump)))
            .as_ptr()
            .cast();
        crate::native::vulkan1_1::RenderPassMultiviewCreateInfo {
            s_type: StructureType::RenderPassMultiviewCreateInfo,
            p_next: std::ptr::null(),
            subpass_count: len_view_masks,
            view_masks: view_masks,
            dependency_count: len_view_offsets,
            view_offsets: view_offsets,
            correlation_mask_count: len_correlation_masks,
            correlation_masks: correlation_masks,
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for RenderPassMultiviewCreateInfo {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let view_masks_len = value.subpass_count;
        let mut view_masks = SmallVec::with_capacity(view_masks_len as usize);
        for i in 0..view_masks_len {
            view_masks.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.view_masks.add(i as usize).read(),
            ));
        }
        let view_offsets_len = value.dependency_count;
        let mut view_offsets = SmallVec::with_capacity(view_offsets_len as usize);
        for i in 0..view_offsets_len {
            view_offsets.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.view_offsets.add(i as usize).read(),
            ));
        }
        let correlation_masks_len = value.correlation_mask_count;
        let mut correlation_masks = SmallVec::with_capacity(correlation_masks_len as usize);
        for i in 0..correlation_masks_len {
            correlation_masks.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.correlation_masks.add(i as usize).read(),
            ));
        }
        Self {
            view_masks: view_masks,
            view_offsets: view_offsets,
            correlation_masks: correlation_masks,
        }
    }
}
#[doc(alias = "VkPhysicalDeviceGroupProperties")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceGroupProperties {
    #[doc(alias = "physicalDeviceCount")]
    pub physical_device_count: u32,
    #[doc(alias = "physicalDevices")]
    pub physical_devices: [PhysicalDevice; MAX_DEVICE_GROUP_SIZE as usize],
    #[doc(alias = "subsetAllocation")]
    pub subset_allocation: bool,
}
impl PhysicalDeviceGroupProperties {
    ///Get a reference to the `physical_device_count` field.
    pub fn physical_device_count(&self) -> u32 {
        self.physical_device_count
    }
    ///Get a reference to the `physical_devices` field.
    pub fn physical_devices(&self) -> &[PhysicalDevice; MAX_DEVICE_GROUP_SIZE as usize] {
        &self.physical_devices
    }
    ///Get a reference to the `subset_allocation` field.
    pub fn subset_allocation(&self) -> &bool {
        &self.subset_allocation
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceGroupProperties {
    type LowLevel = crate::native::vulkan1_1::PhysicalDeviceGroupProperties;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_1::PhysicalDeviceGroupProperties {
            s_type: StructureType::PhysicalDeviceGroupProperties,
            p_next: std::ptr::null_mut(),
            physical_device_count: self.physical_device_count.into_low_level(context, bump),
            physical_devices: self.physical_devices.into_low_level(context, bump),
            subset_allocation: self.subset_allocation.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceGroupProperties {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            physical_device_count: crate::conv::FromLowLevel::from_low_level(context, value.physical_device_count),
            physical_devices: crate::conv::FromLowLevel::from_low_level(context, value.physical_devices),
            subset_allocation: crate::conv::FromLowLevel::from_low_level(context, value.subset_allocation),
        }
    }
}
#[doc(alias = "VkMemoryAllocateFlagsInfo")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MemoryAllocateFlagsInfo {
    pub flags: MemoryAllocateFlags,
    #[doc(alias = "deviceMask")]
    pub device_mask: u32,
}
impl MemoryAllocateFlagsInfo {
    ///Get a reference to the `flags` field.
    pub fn flags(&self) -> MemoryAllocateFlags {
        self.flags
    }
    ///Get a reference to the `device_mask` field.
    pub fn device_mask(&self) -> u32 {
        self.device_mask
    }
    ///Get a mutable reference to the `flags` field.
    pub fn flags_mut(&mut self) -> &mut MemoryAllocateFlags {
        &mut self.flags
    }
    ///Get a mutable reference to the `device_mask` field.
    pub fn device_mask_mut(&mut self) -> &mut u32 {
        &mut self.device_mask
    }
    ///Sets the `flags` field.
    pub fn set_flags(&mut self, flags: MemoryAllocateFlags) -> &mut Self {
        self.flags = flags;
        self
    }
    ///Sets the `device_mask` field.
    pub fn set_device_mask(&mut self, device_mask: u32) -> &mut Self {
        self.device_mask = device_mask;
        self
    }
    ///Sets the `flags` field in a builder way.
    pub fn with_flags(mut self, flags: MemoryAllocateFlags) -> Self {
        self.flags = flags;
        self
    }
    ///Sets the `device_mask` field in a builder way.
    pub fn with_device_mask(mut self, device_mask: u32) -> Self {
        self.device_mask = device_mask;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for MemoryAllocateFlagsInfo {
    type LowLevel = crate::native::vulkan1_1::MemoryAllocateFlagsInfo;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_1::MemoryAllocateFlagsInfo {
            s_type: StructureType::MemoryAllocateFlagsInfo,
            p_next: std::ptr::null(),
            flags: self.flags.into_low_level(context, bump),
            device_mask: self.device_mask.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for MemoryAllocateFlagsInfo {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            flags: crate::conv::FromLowLevel::from_low_level(context, value.flags),
            device_mask: crate::conv::FromLowLevel::from_low_level(context, value.device_mask),
        }
    }
}
#[doc(alias = "VkBindBufferMemoryInfo")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BindBufferMemoryInfo {
    #[doc(alias = "pNext")]
    pub extensions: SmallVec<[BindBufferMemoryInfoExtension; 1]>,
    pub buffer: Buffer,
    pub memory: DeviceMemory,
    #[doc(alias = "memoryOffset")]
    pub memory_offset: DeviceSize,
}
impl BindBufferMemoryInfo {
    ///Adds an extension to the struct
    pub fn with_extension(mut self, ext: impl Into<BindBufferMemoryInfoExtension>) -> Self {
        self.extensions.push(ext.into());
        self
    }
    ///Get a reference to the `extensions` field.
    pub fn extensions(&self) -> &SmallVec<[BindBufferMemoryInfoExtension; 1]> {
        &self.extensions
    }
    ///Get a reference to the `buffer` field.
    pub fn buffer(&self) -> &Buffer {
        &self.buffer
    }
    ///Get a reference to the `memory` field.
    pub fn memory(&self) -> &DeviceMemory {
        &self.memory
    }
    ///Get a reference to the `memory_offset` field.
    pub fn memory_offset(&self) -> &DeviceSize {
        &self.memory_offset
    }
    ///Get a mutable reference to the `extensions` field.
    pub fn extensions_mut(&mut self) -> &mut SmallVec<[BindBufferMemoryInfoExtension; 1]> {
        &mut self.extensions
    }
    ///Get a mutable reference to the `buffer` field.
    pub fn buffer_mut(&mut self) -> &mut Buffer {
        &mut self.buffer
    }
    ///Get a mutable reference to the `memory` field.
    pub fn memory_mut(&mut self) -> &mut DeviceMemory {
        &mut self.memory
    }
    ///Get a mutable reference to the `memory_offset` field.
    pub fn memory_offset_mut(&mut self) -> &mut DeviceSize {
        &mut self.memory_offset
    }
    ///Sets the `extensions` field.
    pub fn set_extensions(&mut self, extensions: SmallVec<[BindBufferMemoryInfoExtension; 1]>) -> &mut Self {
        self.extensions = extensions;
        self
    }
    ///Sets the `buffer` field.
    pub fn set_buffer(&mut self, buffer: Buffer) -> &mut Self {
        self.buffer = buffer;
        self
    }
    ///Sets the `memory` field.
    pub fn set_memory(&mut self, memory: DeviceMemory) -> &mut Self {
        self.memory = memory;
        self
    }
    ///Sets the `memory_offset` field.
    pub fn set_memory_offset(&mut self, memory_offset: DeviceSize) -> &mut Self {
        self.memory_offset = memory_offset;
        self
    }
    ///Sets the `extensions` field in a builder way.
    pub fn with_extensions(mut self, extensions: SmallVec<[BindBufferMemoryInfoExtension; 1]>) -> Self {
        self.extensions = extensions;
        self
    }
    ///Sets the `buffer` field in a builder way.
    pub fn with_buffer(mut self, buffer: Buffer) -> Self {
        self.buffer = buffer;
        self
    }
    ///Sets the `memory` field in a builder way.
    pub fn with_memory(mut self, memory: DeviceMemory) -> Self {
        self.memory = memory;
        self
    }
    ///Sets the `memory_offset` field in a builder way.
    pub fn with_memory_offset(mut self, memory_offset: DeviceSize) -> Self {
        self.memory_offset = memory_offset;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for BindBufferMemoryInfo {
    type LowLevel = crate::native::vulkan1_1::BindBufferMemoryInfo;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let mut next = std::ptr::null();
        let mut extensions = self.extensions.iter();
        while let Some(ext) = extensions.next() {
            let ext = ext.into_low_level(context, bump);
            (*ext).next = next;
            next = ext;
        }
        crate::native::vulkan1_1::BindBufferMemoryInfo {
            s_type: StructureType::BindBufferMemoryInfo,
            p_next: next,
            buffer: self.buffer.into_low_level(context, bump),
            memory: self.memory.into_low_level(context, bump),
            memory_offset: self.memory_offset.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for BindBufferMemoryInfo {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let mut next = value.p_next;
        let mut extensions = SmallVec::new();
        while !next.is_null() {
            extensions.push(crate::conv::FromLowLevel::from_low_level(context, next));
            next = std::ptr::read(next).next;
        }
        Self {
            extensions: extensions,
            buffer: crate::conv::FromLowLevel::from_low_level(context, value.buffer),
            memory: crate::conv::FromLowLevel::from_low_level(context, value.memory),
            memory_offset: crate::conv::FromLowLevel::from_low_level(context, value.memory_offset),
        }
    }
}
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
///Extensions for [`BindBufferMemoryInfo`]
pub enum BindBufferMemoryInfoExtension {
    ///Contains a type [`BindBufferMemoryDeviceGroupInfo`] for extending [`BindBufferMemoryInfo`]
    BindBufferMemoryDeviceGroupInfo(BindBufferMemoryDeviceGroupInfo),
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for BindBufferMemoryInfoExtension {
    type LowLevel = *const crate::native::vulkan1_0::BaseInStructure;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        match self {
            Self::BindBufferMemoryDeviceGroupInfo(ext) => (bump.alloc(ext.into_low_level(context, bump))
                as *mut crate::native::vulkan1_1::BindBufferMemoryDeviceGroupInfo)
                .cast(),
            other => unreachable!("unexpected variant {:?}", other),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for BindBufferMemoryInfoExtension {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        assert!(!value.is_null());
        match (*value).s_type {
            crate::native::vulkan1_0::StructureType::BindBufferMemoryDeviceGroupInfo => {
                Self::BindBufferMemoryDeviceGroupInfo(BindBufferMemoryDeviceGroupInfo::from_low_level(
                    context,
                    std::ptr::read(value.cast::<crate::native::vulkan1_1::BindBufferMemoryDeviceGroupInfo>()),
                ))
            },
            other => panic!(
                "Structure type {:?} is not a member of {}",
                other,
                stringify!(BindBufferMemoryInfo)
            ),
        }
    }
}
impl From<BindBufferMemoryDeviceGroupInfo> for BindBufferMemoryInfoExtension {
    fn from(ext: BindBufferMemoryDeviceGroupInfo) -> Self {
        Self::BindBufferMemoryDeviceGroupInfo(ext)
    }
}
impl TryInto<BindBufferMemoryDeviceGroupInfo> for BindBufferMemoryInfoExtension {
    type Error = BindBufferMemoryInfoExtension;
    fn try_into(self) -> Result<BindBufferMemoryDeviceGroupInfo, Self::Error> {
        match self {
            Self::BindBufferMemoryDeviceGroupInfo(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[doc(alias = "VkBindBufferMemoryDeviceGroupInfo")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BindBufferMemoryDeviceGroupInfo {
    #[doc(alias = "pDeviceIndices")]
    pub device_indices: SmallVec<[u32; 8]>,
}
impl BindBufferMemoryDeviceGroupInfo {
    ///Get a reference to the `device_indices` field.
    pub fn device_indices(&self) -> &SmallVec<[u32; 8]> {
        &self.device_indices
    }
    ///Get a mutable reference to the `device_indices` field.
    pub fn device_indices_mut(&mut self) -> &mut SmallVec<[u32; 8]> {
        &mut self.device_indices
    }
    ///Sets the `device_indices` field.
    pub fn set_device_indices(&mut self, device_indices: SmallVec<[u32; 8]>) -> &mut Self {
        self.device_indices = device_indices;
        self
    }
    ///Sets the `device_indices` field in a builder way.
    pub fn with_device_indices(mut self, device_indices: SmallVec<[u32; 8]>) -> Self {
        self.device_indices = device_indices;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for BindBufferMemoryDeviceGroupInfo {
    type LowLevel = crate::native::vulkan1_1::BindBufferMemoryDeviceGroupInfo;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let len_device_indices = self.device_indices.len() as u32;
        let device_indices = bump
            .alloc_slice_fill_iter(self.device_indices.iter().map(|x| x.into_low_level(context, bump)))
            .as_ptr()
            .cast();
        crate::native::vulkan1_1::BindBufferMemoryDeviceGroupInfo {
            s_type: StructureType::BindBufferMemoryDeviceGroupInfo,
            p_next: std::ptr::null(),
            device_index_count: len_device_indices,
            device_indices: device_indices,
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for BindBufferMemoryDeviceGroupInfo {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let device_indices_len = value.device_index_count;
        let mut device_indices = SmallVec::with_capacity(device_indices_len as usize);
        for i in 0..device_indices_len {
            device_indices.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.device_indices.add(i as usize).read(),
            ));
        }
        Self {
            device_indices: device_indices,
        }
    }
}
#[doc(alias = "VkBindImageMemoryInfo")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BindImageMemoryInfo {
    #[doc(alias = "pNext")]
    pub extensions: SmallVec<[BindImageMemoryInfoExtension; 1]>,
    pub image: Image,
    pub memory: DeviceMemory,
    #[doc(alias = "memoryOffset")]
    pub memory_offset: DeviceSize,
}
impl BindImageMemoryInfo {
    ///Adds an extension to the struct
    pub fn with_extension(mut self, ext: impl Into<BindImageMemoryInfoExtension>) -> Self {
        self.extensions.push(ext.into());
        self
    }
    ///Get a reference to the `extensions` field.
    pub fn extensions(&self) -> &SmallVec<[BindImageMemoryInfoExtension; 1]> {
        &self.extensions
    }
    ///Get a reference to the `image` field.
    pub fn image(&self) -> &Image {
        &self.image
    }
    ///Get a reference to the `memory` field.
    pub fn memory(&self) -> &DeviceMemory {
        &self.memory
    }
    ///Get a reference to the `memory_offset` field.
    pub fn memory_offset(&self) -> &DeviceSize {
        &self.memory_offset
    }
    ///Get a mutable reference to the `extensions` field.
    pub fn extensions_mut(&mut self) -> &mut SmallVec<[BindImageMemoryInfoExtension; 1]> {
        &mut self.extensions
    }
    ///Get a mutable reference to the `image` field.
    pub fn image_mut(&mut self) -> &mut Image {
        &mut self.image
    }
    ///Get a mutable reference to the `memory` field.
    pub fn memory_mut(&mut self) -> &mut DeviceMemory {
        &mut self.memory
    }
    ///Get a mutable reference to the `memory_offset` field.
    pub fn memory_offset_mut(&mut self) -> &mut DeviceSize {
        &mut self.memory_offset
    }
    ///Sets the `extensions` field.
    pub fn set_extensions(&mut self, extensions: SmallVec<[BindImageMemoryInfoExtension; 1]>) -> &mut Self {
        self.extensions = extensions;
        self
    }
    ///Sets the `image` field.
    pub fn set_image(&mut self, image: Image) -> &mut Self {
        self.image = image;
        self
    }
    ///Sets the `memory` field.
    pub fn set_memory(&mut self, memory: DeviceMemory) -> &mut Self {
        self.memory = memory;
        self
    }
    ///Sets the `memory_offset` field.
    pub fn set_memory_offset(&mut self, memory_offset: DeviceSize) -> &mut Self {
        self.memory_offset = memory_offset;
        self
    }
    ///Sets the `extensions` field in a builder way.
    pub fn with_extensions(mut self, extensions: SmallVec<[BindImageMemoryInfoExtension; 1]>) -> Self {
        self.extensions = extensions;
        self
    }
    ///Sets the `image` field in a builder way.
    pub fn with_image(mut self, image: Image) -> Self {
        self.image = image;
        self
    }
    ///Sets the `memory` field in a builder way.
    pub fn with_memory(mut self, memory: DeviceMemory) -> Self {
        self.memory = memory;
        self
    }
    ///Sets the `memory_offset` field in a builder way.
    pub fn with_memory_offset(mut self, memory_offset: DeviceSize) -> Self {
        self.memory_offset = memory_offset;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for BindImageMemoryInfo {
    type LowLevel = crate::native::vulkan1_1::BindImageMemoryInfo;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let mut next = std::ptr::null();
        let mut extensions = self.extensions.iter();
        while let Some(ext) = extensions.next() {
            let ext = ext.into_low_level(context, bump);
            (*ext).next = next;
            next = ext;
        }
        crate::native::vulkan1_1::BindImageMemoryInfo {
            s_type: StructureType::BindImageMemoryInfo,
            p_next: next,
            image: self.image.into_low_level(context, bump),
            memory: self.memory.into_low_level(context, bump),
            memory_offset: self.memory_offset.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for BindImageMemoryInfo {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let mut next = value.p_next;
        let mut extensions = SmallVec::new();
        while !next.is_null() {
            extensions.push(crate::conv::FromLowLevel::from_low_level(context, next));
            next = std::ptr::read(next).next;
        }
        Self {
            extensions: extensions,
            image: crate::conv::FromLowLevel::from_low_level(context, value.image),
            memory: crate::conv::FromLowLevel::from_low_level(context, value.memory),
            memory_offset: crate::conv::FromLowLevel::from_low_level(context, value.memory_offset),
        }
    }
}
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
///Extensions for [`BindImageMemoryInfo`]
pub enum BindImageMemoryInfoExtension {
    ///Contains a type [`BindImageMemoryDeviceGroupInfo`] for extending [`BindImageMemoryInfo`]
    BindImageMemoryDeviceGroupInfo(BindImageMemoryDeviceGroupInfo),
    #[cfg(feature = "VK_KHR_device_group")]
    ///Contains a type [`BindImageMemorySwapchainInfoKHR`] for extending [`BindImageMemoryInfo`]
    BindImageMemorySwapchainInfoKHR(BindImageMemorySwapchainInfoKHR),
    ///Contains a type [`BindImagePlaneMemoryInfo`] for extending [`BindImageMemoryInfo`]
    BindImagePlaneMemoryInfo(BindImagePlaneMemoryInfo),
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for BindImageMemoryInfoExtension {
    type LowLevel = *const crate::native::vulkan1_0::BaseInStructure;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        match self {
            Self::BindImageMemoryDeviceGroupInfo(ext) => (bump.alloc(ext.into_low_level(context, bump))
                as *mut crate::native::vulkan1_1::BindImageMemoryDeviceGroupInfo)
                .cast(),
            #[cfg(feature = "VK_KHR_device_group")]
            Self::BindImageMemorySwapchainInfoKHR(ext) => (bump.alloc(ext.into_low_level(context, bump))
                as *mut crate::native::extensions::khr_device_group::BindImageMemorySwapchainInfoKHR)
                .cast(),
            Self::BindImagePlaneMemoryInfo(ext) => (bump.alloc(ext.into_low_level(context, bump))
                as *mut crate::native::vulkan1_1::BindImagePlaneMemoryInfo)
                .cast(),
            other => unreachable!("unexpected variant {:?}", other),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for BindImageMemoryInfoExtension {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        assert!(!value.is_null());
        match (*value).s_type {
            crate::native::vulkan1_0::StructureType::BindImageMemoryDeviceGroupInfo => {
                Self::BindImageMemoryDeviceGroupInfo(BindImageMemoryDeviceGroupInfo::from_low_level(
                    context,
                    std::ptr::read(value.cast::<crate::native::vulkan1_1::BindImageMemoryDeviceGroupInfo>()),
                ))
            },
            #[cfg(feature = "VK_KHR_device_group")]
            crate::native::vulkan1_0::StructureType::BindImageMemorySwapchainInfoKhr => {
                Self::BindImageMemorySwapchainInfoKHR(BindImageMemorySwapchainInfoKHR::from_low_level(
                    context,
                    std::ptr::read(
                        value.cast::<crate::native::extensions::khr_device_group::BindImageMemorySwapchainInfoKHR>(),
                    ),
                ))
            },
            crate::native::vulkan1_0::StructureType::BindImagePlaneMemoryInfo => {
                Self::BindImagePlaneMemoryInfo(BindImagePlaneMemoryInfo::from_low_level(
                    context,
                    std::ptr::read(value.cast::<crate::native::vulkan1_1::BindImagePlaneMemoryInfo>()),
                ))
            },
            other => panic!(
                "Structure type {:?} is not a member of {}",
                other,
                stringify!(BindImageMemoryInfo)
            ),
        }
    }
}
impl From<BindImageMemoryDeviceGroupInfo> for BindImageMemoryInfoExtension {
    fn from(ext: BindImageMemoryDeviceGroupInfo) -> Self {
        Self::BindImageMemoryDeviceGroupInfo(ext)
    }
}
impl TryInto<BindImageMemoryDeviceGroupInfo> for BindImageMemoryInfoExtension {
    type Error = BindImageMemoryInfoExtension;
    fn try_into(self) -> Result<BindImageMemoryDeviceGroupInfo, Self::Error> {
        match self {
            Self::BindImageMemoryDeviceGroupInfo(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_KHR_device_group")]
impl From<BindImageMemorySwapchainInfoKHR> for BindImageMemoryInfoExtension {
    fn from(ext: BindImageMemorySwapchainInfoKHR) -> Self {
        Self::BindImageMemorySwapchainInfoKHR(ext)
    }
}
#[cfg(feature = "VK_KHR_device_group")]
impl TryInto<BindImageMemorySwapchainInfoKHR> for BindImageMemoryInfoExtension {
    type Error = BindImageMemoryInfoExtension;
    fn try_into(self) -> Result<BindImageMemorySwapchainInfoKHR, Self::Error> {
        match self {
            Self::BindImageMemorySwapchainInfoKHR(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
impl From<BindImagePlaneMemoryInfo> for BindImageMemoryInfoExtension {
    fn from(ext: BindImagePlaneMemoryInfo) -> Self {
        Self::BindImagePlaneMemoryInfo(ext)
    }
}
impl TryInto<BindImagePlaneMemoryInfo> for BindImageMemoryInfoExtension {
    type Error = BindImageMemoryInfoExtension;
    fn try_into(self) -> Result<BindImagePlaneMemoryInfo, Self::Error> {
        match self {
            Self::BindImagePlaneMemoryInfo(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[doc(alias = "VkBindImageMemoryDeviceGroupInfo")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BindImageMemoryDeviceGroupInfo {
    #[doc(alias = "pDeviceIndices")]
    pub device_indices: SmallVec<[u32; 8]>,
    #[doc(alias = "pSplitInstanceBindRegions")]
    pub split_instance_bind_regions: SmallVec<[Rect2D; 8]>,
}
impl BindImageMemoryDeviceGroupInfo {
    ///Get a reference to the `device_indices` field.
    pub fn device_indices(&self) -> &SmallVec<[u32; 8]> {
        &self.device_indices
    }
    ///Get a reference to the `split_instance_bind_regions` field.
    pub fn split_instance_bind_regions(&self) -> &SmallVec<[Rect2D; 8]> {
        &self.split_instance_bind_regions
    }
    ///Get a mutable reference to the `device_indices` field.
    pub fn device_indices_mut(&mut self) -> &mut SmallVec<[u32; 8]> {
        &mut self.device_indices
    }
    ///Get a mutable reference to the `split_instance_bind_regions` field.
    pub fn split_instance_bind_regions_mut(&mut self) -> &mut SmallVec<[Rect2D; 8]> {
        &mut self.split_instance_bind_regions
    }
    ///Sets the `device_indices` field.
    pub fn set_device_indices(&mut self, device_indices: SmallVec<[u32; 8]>) -> &mut Self {
        self.device_indices = device_indices;
        self
    }
    ///Sets the `split_instance_bind_regions` field.
    pub fn set_split_instance_bind_regions(&mut self, split_instance_bind_regions: SmallVec<[Rect2D; 8]>) -> &mut Self {
        self.split_instance_bind_regions = split_instance_bind_regions;
        self
    }
    ///Sets the `device_indices` field in a builder way.
    pub fn with_device_indices(mut self, device_indices: SmallVec<[u32; 8]>) -> Self {
        self.device_indices = device_indices;
        self
    }
    ///Sets the `split_instance_bind_regions` field in a builder way.
    pub fn with_split_instance_bind_regions(mut self, split_instance_bind_regions: SmallVec<[Rect2D; 8]>) -> Self {
        self.split_instance_bind_regions = split_instance_bind_regions;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for BindImageMemoryDeviceGroupInfo {
    type LowLevel = crate::native::vulkan1_1::BindImageMemoryDeviceGroupInfo;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let len_device_indices = self.device_indices.len() as u32;
        let device_indices = bump
            .alloc_slice_fill_iter(self.device_indices.iter().map(|x| x.into_low_level(context, bump)))
            .as_ptr()
            .cast();
        let len_split_instance_bind_regions = self.split_instance_bind_regions.len() as u32;
        let split_instance_bind_regions = bump
            .alloc_slice_fill_iter(
                self.split_instance_bind_regions
                    .iter()
                    .map(|x| x.into_low_level(context, bump)),
            )
            .as_ptr()
            .cast();
        crate::native::vulkan1_1::BindImageMemoryDeviceGroupInfo {
            s_type: StructureType::BindImageMemoryDeviceGroupInfo,
            p_next: std::ptr::null(),
            device_index_count: len_device_indices,
            device_indices: device_indices,
            split_instance_bind_region_count: len_split_instance_bind_regions,
            split_instance_bind_regions: split_instance_bind_regions,
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for BindImageMemoryDeviceGroupInfo {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let device_indices_len = value.device_index_count;
        let mut device_indices = SmallVec::with_capacity(device_indices_len as usize);
        for i in 0..device_indices_len {
            device_indices.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.device_indices.add(i as usize).read(),
            ));
        }
        let split_instance_bind_regions_len = value.split_instance_bind_region_count;
        let mut split_instance_bind_regions = SmallVec::with_capacity(split_instance_bind_regions_len as usize);
        for i in 0..split_instance_bind_regions_len {
            split_instance_bind_regions.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.split_instance_bind_regions.add(i as usize).read(),
            ));
        }
        Self {
            device_indices: device_indices,
            split_instance_bind_regions: split_instance_bind_regions,
        }
    }
}
#[doc(alias = "VkDeviceGroupRenderPassBeginInfo")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DeviceGroupRenderPassBeginInfo {
    #[doc(alias = "deviceMask")]
    pub device_mask: u32,
    #[doc(alias = "pDeviceRenderAreas")]
    pub device_render_areas: SmallVec<[Rect2D; 8]>,
}
impl DeviceGroupRenderPassBeginInfo {
    ///Get a reference to the `device_mask` field.
    pub fn device_mask(&self) -> u32 {
        self.device_mask
    }
    ///Get a reference to the `device_render_areas` field.
    pub fn device_render_areas(&self) -> &SmallVec<[Rect2D; 8]> {
        &self.device_render_areas
    }
    ///Get a mutable reference to the `device_mask` field.
    pub fn device_mask_mut(&mut self) -> &mut u32 {
        &mut self.device_mask
    }
    ///Get a mutable reference to the `device_render_areas` field.
    pub fn device_render_areas_mut(&mut self) -> &mut SmallVec<[Rect2D; 8]> {
        &mut self.device_render_areas
    }
    ///Sets the `device_mask` field.
    pub fn set_device_mask(&mut self, device_mask: u32) -> &mut Self {
        self.device_mask = device_mask;
        self
    }
    ///Sets the `device_render_areas` field.
    pub fn set_device_render_areas(&mut self, device_render_areas: SmallVec<[Rect2D; 8]>) -> &mut Self {
        self.device_render_areas = device_render_areas;
        self
    }
    ///Sets the `device_mask` field in a builder way.
    pub fn with_device_mask(mut self, device_mask: u32) -> Self {
        self.device_mask = device_mask;
        self
    }
    ///Sets the `device_render_areas` field in a builder way.
    pub fn with_device_render_areas(mut self, device_render_areas: SmallVec<[Rect2D; 8]>) -> Self {
        self.device_render_areas = device_render_areas;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for DeviceGroupRenderPassBeginInfo {
    type LowLevel = crate::native::vulkan1_1::DeviceGroupRenderPassBeginInfo;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let len_device_render_areas = self.device_render_areas.len() as u32;
        let device_render_areas = bump
            .alloc_slice_fill_iter(self.device_render_areas.iter().map(|x| x.into_low_level(context, bump)))
            .as_ptr()
            .cast();
        crate::native::vulkan1_1::DeviceGroupRenderPassBeginInfo {
            s_type: StructureType::DeviceGroupRenderPassBeginInfo,
            p_next: std::ptr::null(),
            device_mask: self.device_mask.into_low_level(context, bump),
            device_render_area_count: len_device_render_areas,
            device_render_areas: device_render_areas,
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for DeviceGroupRenderPassBeginInfo {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let device_render_areas_len = value.device_render_area_count;
        let mut device_render_areas = SmallVec::with_capacity(device_render_areas_len as usize);
        for i in 0..device_render_areas_len {
            device_render_areas.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.device_render_areas.add(i as usize).read(),
            ));
        }
        Self {
            device_mask: crate::conv::FromLowLevel::from_low_level(context, value.device_mask),
            device_render_areas: device_render_areas,
        }
    }
}
#[doc(alias = "VkDeviceGroupCommandBufferBeginInfo")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DeviceGroupCommandBufferBeginInfo {
    #[doc(alias = "deviceMask")]
    pub device_mask: u32,
}
impl DeviceGroupCommandBufferBeginInfo {
    ///Get a reference to the `device_mask` field.
    pub fn device_mask(&self) -> u32 {
        self.device_mask
    }
    ///Get a mutable reference to the `device_mask` field.
    pub fn device_mask_mut(&mut self) -> &mut u32 {
        &mut self.device_mask
    }
    ///Sets the `device_mask` field.
    pub fn set_device_mask(&mut self, device_mask: u32) -> &mut Self {
        self.device_mask = device_mask;
        self
    }
    ///Sets the `device_mask` field in a builder way.
    pub fn with_device_mask(mut self, device_mask: u32) -> Self {
        self.device_mask = device_mask;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for DeviceGroupCommandBufferBeginInfo {
    type LowLevel = crate::native::vulkan1_1::DeviceGroupCommandBufferBeginInfo;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_1::DeviceGroupCommandBufferBeginInfo {
            s_type: StructureType::DeviceGroupCommandBufferBeginInfo,
            p_next: std::ptr::null(),
            device_mask: self.device_mask.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for DeviceGroupCommandBufferBeginInfo {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            device_mask: crate::conv::FromLowLevel::from_low_level(context, value.device_mask),
        }
    }
}
#[doc(alias = "VkDeviceGroupSubmitInfo")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DeviceGroupSubmitInfo {
    #[doc(alias = "pWaitSemaphoreDeviceIndices")]
    pub wait_semaphore_device_indices: SmallVec<[u32; 8]>,
    #[doc(alias = "pCommandBufferDeviceMasks")]
    pub command_buffer_device_masks: SmallVec<[u32; 8]>,
    #[doc(alias = "pSignalSemaphoreDeviceIndices")]
    pub signal_semaphore_device_indices: SmallVec<[u32; 8]>,
}
impl DeviceGroupSubmitInfo {
    ///Get a reference to the `wait_semaphore_device_indices` field.
    pub fn wait_semaphore_device_indices(&self) -> &SmallVec<[u32; 8]> {
        &self.wait_semaphore_device_indices
    }
    ///Get a reference to the `command_buffer_device_masks` field.
    pub fn command_buffer_device_masks(&self) -> &SmallVec<[u32; 8]> {
        &self.command_buffer_device_masks
    }
    ///Get a reference to the `signal_semaphore_device_indices` field.
    pub fn signal_semaphore_device_indices(&self) -> &SmallVec<[u32; 8]> {
        &self.signal_semaphore_device_indices
    }
    ///Get a mutable reference to the `wait_semaphore_device_indices` field.
    pub fn wait_semaphore_device_indices_mut(&mut self) -> &mut SmallVec<[u32; 8]> {
        &mut self.wait_semaphore_device_indices
    }
    ///Get a mutable reference to the `command_buffer_device_masks` field.
    pub fn command_buffer_device_masks_mut(&mut self) -> &mut SmallVec<[u32; 8]> {
        &mut self.command_buffer_device_masks
    }
    ///Get a mutable reference to the `signal_semaphore_device_indices` field.
    pub fn signal_semaphore_device_indices_mut(&mut self) -> &mut SmallVec<[u32; 8]> {
        &mut self.signal_semaphore_device_indices
    }
    ///Sets the `wait_semaphore_device_indices` field.
    pub fn set_wait_semaphore_device_indices(
        &mut self,
        wait_semaphore_device_indices: SmallVec<[u32; 8]>,
    ) -> &mut Self {
        self.wait_semaphore_device_indices = wait_semaphore_device_indices;
        self
    }
    ///Sets the `command_buffer_device_masks` field.
    pub fn set_command_buffer_device_masks(&mut self, command_buffer_device_masks: SmallVec<[u32; 8]>) -> &mut Self {
        self.command_buffer_device_masks = command_buffer_device_masks;
        self
    }
    ///Sets the `signal_semaphore_device_indices` field.
    pub fn set_signal_semaphore_device_indices(
        &mut self,
        signal_semaphore_device_indices: SmallVec<[u32; 8]>,
    ) -> &mut Self {
        self.signal_semaphore_device_indices = signal_semaphore_device_indices;
        self
    }
    ///Sets the `wait_semaphore_device_indices` field in a builder way.
    pub fn with_wait_semaphore_device_indices(mut self, wait_semaphore_device_indices: SmallVec<[u32; 8]>) -> Self {
        self.wait_semaphore_device_indices = wait_semaphore_device_indices;
        self
    }
    ///Sets the `command_buffer_device_masks` field in a builder way.
    pub fn with_command_buffer_device_masks(mut self, command_buffer_device_masks: SmallVec<[u32; 8]>) -> Self {
        self.command_buffer_device_masks = command_buffer_device_masks;
        self
    }
    ///Sets the `signal_semaphore_device_indices` field in a builder way.
    pub fn with_signal_semaphore_device_indices(mut self, signal_semaphore_device_indices: SmallVec<[u32; 8]>) -> Self {
        self.signal_semaphore_device_indices = signal_semaphore_device_indices;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for DeviceGroupSubmitInfo {
    type LowLevel = crate::native::vulkan1_1::DeviceGroupSubmitInfo;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let len_wait_semaphore_device_indices = self.wait_semaphore_device_indices.len() as u32;
        let wait_semaphore_device_indices = bump
            .alloc_slice_fill_iter(
                self.wait_semaphore_device_indices
                    .iter()
                    .map(|x| x.into_low_level(context, bump)),
            )
            .as_ptr()
            .cast();
        let len_command_buffer_device_masks = self.command_buffer_device_masks.len() as u32;
        let command_buffer_device_masks = bump
            .alloc_slice_fill_iter(
                self.command_buffer_device_masks
                    .iter()
                    .map(|x| x.into_low_level(context, bump)),
            )
            .as_ptr()
            .cast();
        let len_signal_semaphore_device_indices = self.signal_semaphore_device_indices.len() as u32;
        let signal_semaphore_device_indices = bump
            .alloc_slice_fill_iter(
                self.signal_semaphore_device_indices
                    .iter()
                    .map(|x| x.into_low_level(context, bump)),
            )
            .as_ptr()
            .cast();
        crate::native::vulkan1_1::DeviceGroupSubmitInfo {
            s_type: StructureType::DeviceGroupSubmitInfo,
            p_next: std::ptr::null(),
            wait_semaphore_count: len_wait_semaphore_device_indices,
            wait_semaphore_device_indices: wait_semaphore_device_indices,
            command_buffer_count: len_command_buffer_device_masks,
            command_buffer_device_masks: command_buffer_device_masks,
            signal_semaphore_count: len_signal_semaphore_device_indices,
            signal_semaphore_device_indices: signal_semaphore_device_indices,
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for DeviceGroupSubmitInfo {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let wait_semaphore_device_indices_len = value.wait_semaphore_count;
        let mut wait_semaphore_device_indices = SmallVec::with_capacity(wait_semaphore_device_indices_len as usize);
        for i in 0..wait_semaphore_device_indices_len {
            wait_semaphore_device_indices.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.wait_semaphore_device_indices.add(i as usize).read(),
            ));
        }
        let command_buffer_device_masks_len = value.command_buffer_count;
        let mut command_buffer_device_masks = SmallVec::with_capacity(command_buffer_device_masks_len as usize);
        for i in 0..command_buffer_device_masks_len {
            command_buffer_device_masks.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.command_buffer_device_masks.add(i as usize).read(),
            ));
        }
        let signal_semaphore_device_indices_len = value.signal_semaphore_count;
        let mut signal_semaphore_device_indices = SmallVec::with_capacity(signal_semaphore_device_indices_len as usize);
        for i in 0..signal_semaphore_device_indices_len {
            signal_semaphore_device_indices.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.signal_semaphore_device_indices.add(i as usize).read(),
            ));
        }
        Self {
            wait_semaphore_device_indices: wait_semaphore_device_indices,
            command_buffer_device_masks: command_buffer_device_masks,
            signal_semaphore_device_indices: signal_semaphore_device_indices,
        }
    }
}
#[doc(alias = "VkDeviceGroupBindSparseInfo")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DeviceGroupBindSparseInfo {
    #[doc(alias = "resourceDeviceIndex")]
    pub resource_device_index: u32,
    #[doc(alias = "memoryDeviceIndex")]
    pub memory_device_index: u32,
}
impl DeviceGroupBindSparseInfo {
    ///Get a reference to the `resource_device_index` field.
    pub fn resource_device_index(&self) -> u32 {
        self.resource_device_index
    }
    ///Get a reference to the `memory_device_index` field.
    pub fn memory_device_index(&self) -> u32 {
        self.memory_device_index
    }
    ///Get a mutable reference to the `resource_device_index` field.
    pub fn resource_device_index_mut(&mut self) -> &mut u32 {
        &mut self.resource_device_index
    }
    ///Get a mutable reference to the `memory_device_index` field.
    pub fn memory_device_index_mut(&mut self) -> &mut u32 {
        &mut self.memory_device_index
    }
    ///Sets the `resource_device_index` field.
    pub fn set_resource_device_index(&mut self, resource_device_index: u32) -> &mut Self {
        self.resource_device_index = resource_device_index;
        self
    }
    ///Sets the `memory_device_index` field.
    pub fn set_memory_device_index(&mut self, memory_device_index: u32) -> &mut Self {
        self.memory_device_index = memory_device_index;
        self
    }
    ///Sets the `resource_device_index` field in a builder way.
    pub fn with_resource_device_index(mut self, resource_device_index: u32) -> Self {
        self.resource_device_index = resource_device_index;
        self
    }
    ///Sets the `memory_device_index` field in a builder way.
    pub fn with_memory_device_index(mut self, memory_device_index: u32) -> Self {
        self.memory_device_index = memory_device_index;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for DeviceGroupBindSparseInfo {
    type LowLevel = crate::native::vulkan1_1::DeviceGroupBindSparseInfo;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_1::DeviceGroupBindSparseInfo {
            s_type: StructureType::DeviceGroupBindSparseInfo,
            p_next: std::ptr::null(),
            resource_device_index: self.resource_device_index.into_low_level(context, bump),
            memory_device_index: self.memory_device_index.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for DeviceGroupBindSparseInfo {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            resource_device_index: crate::conv::FromLowLevel::from_low_level(context, value.resource_device_index),
            memory_device_index: crate::conv::FromLowLevel::from_low_level(context, value.memory_device_index),
        }
    }
}
#[doc(alias = "VkDeviceGroupDeviceCreateInfo")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DeviceGroupDeviceCreateInfo {
    #[doc(alias = "pPhysicalDevices")]
    pub physical_devices: SmallVec<[PhysicalDevice; 8]>,
}
impl DeviceGroupDeviceCreateInfo {
    ///Get a reference to the `physical_devices` field.
    pub fn physical_devices(&self) -> &SmallVec<[PhysicalDevice; 8]> {
        &self.physical_devices
    }
    ///Get a mutable reference to the `physical_devices` field.
    pub fn physical_devices_mut(&mut self) -> &mut SmallVec<[PhysicalDevice; 8]> {
        &mut self.physical_devices
    }
    ///Sets the `physical_devices` field.
    pub fn set_physical_devices(&mut self, physical_devices: SmallVec<[PhysicalDevice; 8]>) -> &mut Self {
        self.physical_devices = physical_devices;
        self
    }
    ///Sets the `physical_devices` field in a builder way.
    pub fn with_physical_devices(mut self, physical_devices: SmallVec<[PhysicalDevice; 8]>) -> Self {
        self.physical_devices = physical_devices;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for DeviceGroupDeviceCreateInfo {
    type LowLevel = crate::native::vulkan1_1::DeviceGroupDeviceCreateInfo;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let len_physical_devices = self.physical_devices.len() as u32;
        let physical_devices = bump
            .alloc_slice_fill_iter(self.physical_devices.iter().map(|x| x.into_low_level(context, bump)))
            .as_ptr()
            .cast();
        crate::native::vulkan1_1::DeviceGroupDeviceCreateInfo {
            s_type: StructureType::DeviceGroupDeviceCreateInfo,
            p_next: std::ptr::null(),
            physical_device_count: len_physical_devices,
            physical_devices: physical_devices,
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for DeviceGroupDeviceCreateInfo {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let physical_devices_len = value.physical_device_count;
        let mut physical_devices = SmallVec::with_capacity(physical_devices_len as usize);
        for i in 0..physical_devices_len {
            physical_devices.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.physical_devices.add(i as usize).read(),
            ));
        }
        Self {
            physical_devices: physical_devices,
        }
    }
}
impl DescriptorUpdateTemplateEntry {
    ///Get a reference to the `dst_binding` field.
    pub fn dst_binding(&self) -> u32 {
        self.dst_binding
    }
    ///Get a reference to the `dst_array_element` field.
    pub fn dst_array_element(&self) -> u32 {
        self.dst_array_element
    }
    ///Get a reference to the `descriptor_count` field.
    pub fn descriptor_count(&self) -> u32 {
        self.descriptor_count
    }
    ///Get a reference to the `descriptor_type` field.
    pub fn descriptor_type(&self) -> DescriptorType {
        self.descriptor_type
    }
    ///Get a reference to the `offset` field.
    pub fn offset(&self) -> usize {
        self.offset
    }
    ///Get a reference to the `stride` field.
    pub fn stride(&self) -> usize {
        self.stride
    }
    ///Get a mutable reference to the `dst_binding` field.
    pub fn dst_binding_mut(&mut self) -> &mut u32 {
        &mut self.dst_binding
    }
    ///Get a mutable reference to the `dst_array_element` field.
    pub fn dst_array_element_mut(&mut self) -> &mut u32 {
        &mut self.dst_array_element
    }
    ///Get a mutable reference to the `descriptor_count` field.
    pub fn descriptor_count_mut(&mut self) -> &mut u32 {
        &mut self.descriptor_count
    }
    ///Get a mutable reference to the `descriptor_type` field.
    pub fn descriptor_type_mut(&mut self) -> &mut DescriptorType {
        &mut self.descriptor_type
    }
    ///Get a mutable reference to the `offset` field.
    pub fn offset_mut(&mut self) -> &mut usize {
        &mut self.offset
    }
    ///Get a mutable reference to the `stride` field.
    pub fn stride_mut(&mut self) -> &mut usize {
        &mut self.stride
    }
    ///Sets the `dst_binding` field.
    pub fn set_dst_binding(&mut self, dst_binding: u32) -> &mut Self {
        self.dst_binding = dst_binding;
        self
    }
    ///Sets the `dst_array_element` field.
    pub fn set_dst_array_element(&mut self, dst_array_element: u32) -> &mut Self {
        self.dst_array_element = dst_array_element;
        self
    }
    ///Sets the `descriptor_count` field.
    pub fn set_descriptor_count(&mut self, descriptor_count: u32) -> &mut Self {
        self.descriptor_count = descriptor_count;
        self
    }
    ///Sets the `descriptor_type` field.
    pub fn set_descriptor_type(&mut self, descriptor_type: DescriptorType) -> &mut Self {
        self.descriptor_type = descriptor_type;
        self
    }
    ///Sets the `offset` field.
    pub fn set_offset(&mut self, offset: usize) -> &mut Self {
        self.offset = offset;
        self
    }
    ///Sets the `stride` field.
    pub fn set_stride(&mut self, stride: usize) -> &mut Self {
        self.stride = stride;
        self
    }
    ///Sets the `dst_binding` field in a builder way.
    pub fn with_dst_binding(mut self, dst_binding: u32) -> Self {
        self.dst_binding = dst_binding;
        self
    }
    ///Sets the `dst_array_element` field in a builder way.
    pub fn with_dst_array_element(mut self, dst_array_element: u32) -> Self {
        self.dst_array_element = dst_array_element;
        self
    }
    ///Sets the `descriptor_count` field in a builder way.
    pub fn with_descriptor_count(mut self, descriptor_count: u32) -> Self {
        self.descriptor_count = descriptor_count;
        self
    }
    ///Sets the `descriptor_type` field in a builder way.
    pub fn with_descriptor_type(mut self, descriptor_type: DescriptorType) -> Self {
        self.descriptor_type = descriptor_type;
        self
    }
    ///Sets the `offset` field in a builder way.
    pub fn with_offset(mut self, offset: usize) -> Self {
        self.offset = offset;
        self
    }
    ///Sets the `stride` field in a builder way.
    pub fn with_stride(mut self, stride: usize) -> Self {
        self.stride = stride;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for DescriptorUpdateTemplateEntry {
    type LowLevel = crate::native::vulkan1_1::DescriptorUpdateTemplateEntry;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_1::DescriptorUpdateTemplateEntry {
            dst_binding: self.dst_binding.into_low_level(context, bump),
            dst_array_element: self.dst_array_element.into_low_level(context, bump),
            descriptor_count: self.descriptor_count.into_low_level(context, bump),
            descriptor_type: self.descriptor_type.into_low_level(context, bump),
            offset: self.offset.into_low_level(context, bump),
            stride: self.stride.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for DescriptorUpdateTemplateEntry {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            dst_binding: crate::conv::FromLowLevel::from_low_level(context, value.dst_binding),
            dst_array_element: crate::conv::FromLowLevel::from_low_level(context, value.dst_array_element),
            descriptor_count: crate::conv::FromLowLevel::from_low_level(context, value.descriptor_count),
            descriptor_type: crate::conv::FromLowLevel::from_low_level(context, value.descriptor_type),
            offset: crate::conv::FromLowLevel::from_low_level(context, value.offset),
            stride: crate::conv::FromLowLevel::from_low_level(context, value.stride),
        }
    }
}
#[doc(alias = "VkDescriptorUpdateTemplateCreateInfo")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DescriptorUpdateTemplateCreateInfo {
    pub flags: DescriptorUpdateTemplateCreateFlags,
    #[doc(alias = "pDescriptorUpdateEntries")]
    pub descriptor_update_entries: SmallVec<[DescriptorUpdateTemplateEntry; 8]>,
    #[doc(alias = "templateType")]
    pub template_type: DescriptorUpdateTemplateType,
    #[doc(alias = "descriptorSetLayout")]
    pub descriptor_set_layout: DescriptorSetLayout,
    #[doc(alias = "pipelineBindPoint")]
    pub pipeline_bind_point: PipelineBindPoint,
    #[doc(alias = "pipelineLayout")]
    pub pipeline_layout: PipelineLayout,
    pub set: u32,
}
impl DescriptorUpdateTemplateCreateInfo {
    ///Get a reference to the `flags` field.
    pub fn flags(&self) -> DescriptorUpdateTemplateCreateFlags {
        self.flags
    }
    ///Get a reference to the `descriptor_update_entries` field.
    pub fn descriptor_update_entries(&self) -> &SmallVec<[DescriptorUpdateTemplateEntry; 8]> {
        &self.descriptor_update_entries
    }
    ///Get a reference to the `template_type` field.
    pub fn template_type(&self) -> DescriptorUpdateTemplateType {
        self.template_type
    }
    ///Get a reference to the `descriptor_set_layout` field.
    pub fn descriptor_set_layout(&self) -> &DescriptorSetLayout {
        &self.descriptor_set_layout
    }
    ///Get a reference to the `pipeline_bind_point` field.
    pub fn pipeline_bind_point(&self) -> PipelineBindPoint {
        self.pipeline_bind_point
    }
    ///Get a reference to the `pipeline_layout` field.
    pub fn pipeline_layout(&self) -> &PipelineLayout {
        &self.pipeline_layout
    }
    ///Get a reference to the `set` field.
    pub fn set(&self) -> u32 {
        self.set
    }
    ///Get a mutable reference to the `flags` field.
    pub fn flags_mut(&mut self) -> &mut DescriptorUpdateTemplateCreateFlags {
        &mut self.flags
    }
    ///Get a mutable reference to the `descriptor_update_entries` field.
    pub fn descriptor_update_entries_mut(&mut self) -> &mut SmallVec<[DescriptorUpdateTemplateEntry; 8]> {
        &mut self.descriptor_update_entries
    }
    ///Get a mutable reference to the `template_type` field.
    pub fn template_type_mut(&mut self) -> &mut DescriptorUpdateTemplateType {
        &mut self.template_type
    }
    ///Get a mutable reference to the `descriptor_set_layout` field.
    pub fn descriptor_set_layout_mut(&mut self) -> &mut DescriptorSetLayout {
        &mut self.descriptor_set_layout
    }
    ///Get a mutable reference to the `pipeline_bind_point` field.
    pub fn pipeline_bind_point_mut(&mut self) -> &mut PipelineBindPoint {
        &mut self.pipeline_bind_point
    }
    ///Get a mutable reference to the `pipeline_layout` field.
    pub fn pipeline_layout_mut(&mut self) -> &mut PipelineLayout {
        &mut self.pipeline_layout
    }
    ///Get a mutable reference to the `set` field.
    pub fn set_mut(&mut self) -> &mut u32 {
        &mut self.set
    }
    ///Sets the `flags` field.
    pub fn set_flags(&mut self, flags: DescriptorUpdateTemplateCreateFlags) -> &mut Self {
        self.flags = flags;
        self
    }
    ///Sets the `descriptor_update_entries` field.
    pub fn set_descriptor_update_entries(
        &mut self,
        descriptor_update_entries: SmallVec<[DescriptorUpdateTemplateEntry; 8]>,
    ) -> &mut Self {
        self.descriptor_update_entries = descriptor_update_entries;
        self
    }
    ///Sets the `template_type` field.
    pub fn set_template_type(&mut self, template_type: DescriptorUpdateTemplateType) -> &mut Self {
        self.template_type = template_type;
        self
    }
    ///Sets the `descriptor_set_layout` field.
    pub fn set_descriptor_set_layout(&mut self, descriptor_set_layout: DescriptorSetLayout) -> &mut Self {
        self.descriptor_set_layout = descriptor_set_layout;
        self
    }
    ///Sets the `pipeline_bind_point` field.
    pub fn set_pipeline_bind_point(&mut self, pipeline_bind_point: PipelineBindPoint) -> &mut Self {
        self.pipeline_bind_point = pipeline_bind_point;
        self
    }
    ///Sets the `pipeline_layout` field.
    pub fn set_pipeline_layout(&mut self, pipeline_layout: PipelineLayout) -> &mut Self {
        self.pipeline_layout = pipeline_layout;
        self
    }
    ///Sets the `set` field.
    pub fn set_set(&mut self, set: u32) -> &mut Self {
        self.set = set;
        self
    }
    ///Sets the `flags` field in a builder way.
    pub fn with_flags(mut self, flags: DescriptorUpdateTemplateCreateFlags) -> Self {
        self.flags = flags;
        self
    }
    ///Sets the `descriptor_update_entries` field in a builder way.
    pub fn with_descriptor_update_entries(
        mut self,
        descriptor_update_entries: SmallVec<[DescriptorUpdateTemplateEntry; 8]>,
    ) -> Self {
        self.descriptor_update_entries = descriptor_update_entries;
        self
    }
    ///Sets the `template_type` field in a builder way.
    pub fn with_template_type(mut self, template_type: DescriptorUpdateTemplateType) -> Self {
        self.template_type = template_type;
        self
    }
    ///Sets the `descriptor_set_layout` field in a builder way.
    pub fn with_descriptor_set_layout(mut self, descriptor_set_layout: DescriptorSetLayout) -> Self {
        self.descriptor_set_layout = descriptor_set_layout;
        self
    }
    ///Sets the `pipeline_bind_point` field in a builder way.
    pub fn with_pipeline_bind_point(mut self, pipeline_bind_point: PipelineBindPoint) -> Self {
        self.pipeline_bind_point = pipeline_bind_point;
        self
    }
    ///Sets the `pipeline_layout` field in a builder way.
    pub fn with_pipeline_layout(mut self, pipeline_layout: PipelineLayout) -> Self {
        self.pipeline_layout = pipeline_layout;
        self
    }
    ///Sets the `set` field in a builder way.
    pub fn with_set(mut self, set: u32) -> Self {
        self.set = set;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for DescriptorUpdateTemplateCreateInfo {
    type LowLevel = crate::native::vulkan1_1::DescriptorUpdateTemplateCreateInfo;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let len_descriptor_update_entries = self.descriptor_update_entries.len() as u32;
        let descriptor_update_entries = bump
            .alloc_slice_fill_iter(
                self.descriptor_update_entries
                    .iter()
                    .map(|x| x.into_low_level(context, bump)),
            )
            .as_ptr()
            .cast();
        crate::native::vulkan1_1::DescriptorUpdateTemplateCreateInfo {
            s_type: StructureType::DescriptorUpdateTemplateCreateInfo,
            p_next: std::ptr::null(),
            flags: self.flags.into_low_level(context, bump),
            descriptor_update_entry_count: len_descriptor_update_entries,
            descriptor_update_entries: descriptor_update_entries,
            template_type: self.template_type.into_low_level(context, bump),
            descriptor_set_layout: self.descriptor_set_layout.into_low_level(context, bump),
            pipeline_bind_point: self.pipeline_bind_point.into_low_level(context, bump),
            pipeline_layout: self.pipeline_layout.into_low_level(context, bump),
            set: self.set.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for DescriptorUpdateTemplateCreateInfo {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let descriptor_update_entries_len = value.descriptor_update_entry_count;
        let mut descriptor_update_entries = SmallVec::with_capacity(descriptor_update_entries_len as usize);
        for i in 0..descriptor_update_entries_len {
            descriptor_update_entries.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.descriptor_update_entries.add(i as usize).read(),
            ));
        }
        Self {
            flags: crate::conv::FromLowLevel::from_low_level(context, value.flags),
            descriptor_update_entries: descriptor_update_entries,
            template_type: crate::conv::FromLowLevel::from_low_level(context, value.template_type),
            descriptor_set_layout: crate::conv::FromLowLevel::from_low_level(context, value.descriptor_set_layout),
            pipeline_bind_point: crate::conv::FromLowLevel::from_low_level(context, value.pipeline_bind_point),
            pipeline_layout: crate::conv::FromLowLevel::from_low_level(context, value.pipeline_layout),
            set: crate::conv::FromLowLevel::from_low_level(context, value.set),
        }
    }
}
impl InputAttachmentAspectReference {
    ///Get a reference to the `subpass` field.
    pub fn subpass(&self) -> u32 {
        self.subpass
    }
    ///Get a reference to the `input_attachment_index` field.
    pub fn input_attachment_index(&self) -> u32 {
        self.input_attachment_index
    }
    ///Get a reference to the `aspect_mask` field.
    pub fn aspect_mask(&self) -> ImageAspectFlags {
        self.aspect_mask
    }
    ///Get a mutable reference to the `subpass` field.
    pub fn subpass_mut(&mut self) -> &mut u32 {
        &mut self.subpass
    }
    ///Get a mutable reference to the `input_attachment_index` field.
    pub fn input_attachment_index_mut(&mut self) -> &mut u32 {
        &mut self.input_attachment_index
    }
    ///Get a mutable reference to the `aspect_mask` field.
    pub fn aspect_mask_mut(&mut self) -> &mut ImageAspectFlags {
        &mut self.aspect_mask
    }
    ///Sets the `subpass` field.
    pub fn set_subpass(&mut self, subpass: u32) -> &mut Self {
        self.subpass = subpass;
        self
    }
    ///Sets the `input_attachment_index` field.
    pub fn set_input_attachment_index(&mut self, input_attachment_index: u32) -> &mut Self {
        self.input_attachment_index = input_attachment_index;
        self
    }
    ///Sets the `aspect_mask` field.
    pub fn set_aspect_mask(&mut self, aspect_mask: ImageAspectFlags) -> &mut Self {
        self.aspect_mask = aspect_mask;
        self
    }
    ///Sets the `subpass` field in a builder way.
    pub fn with_subpass(mut self, subpass: u32) -> Self {
        self.subpass = subpass;
        self
    }
    ///Sets the `input_attachment_index` field in a builder way.
    pub fn with_input_attachment_index(mut self, input_attachment_index: u32) -> Self {
        self.input_attachment_index = input_attachment_index;
        self
    }
    ///Sets the `aspect_mask` field in a builder way.
    pub fn with_aspect_mask(mut self, aspect_mask: ImageAspectFlags) -> Self {
        self.aspect_mask = aspect_mask;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for InputAttachmentAspectReference {
    type LowLevel = crate::native::vulkan1_1::InputAttachmentAspectReference;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_1::InputAttachmentAspectReference {
            subpass: self.subpass.into_low_level(context, bump),
            input_attachment_index: self.input_attachment_index.into_low_level(context, bump),
            aspect_mask: self.aspect_mask.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for InputAttachmentAspectReference {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            subpass: crate::conv::FromLowLevel::from_low_level(context, value.subpass),
            input_attachment_index: crate::conv::FromLowLevel::from_low_level(context, value.input_attachment_index),
            aspect_mask: crate::conv::FromLowLevel::from_low_level(context, value.aspect_mask),
        }
    }
}
#[doc(alias = "VkRenderPassInputAttachmentAspectCreateInfo")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RenderPassInputAttachmentAspectCreateInfo {
    #[doc(alias = "pAspectReferences")]
    pub aspect_references: SmallVec<[InputAttachmentAspectReference; 8]>,
}
impl RenderPassInputAttachmentAspectCreateInfo {
    ///Get a reference to the `aspect_references` field.
    pub fn aspect_references(&self) -> &SmallVec<[InputAttachmentAspectReference; 8]> {
        &self.aspect_references
    }
    ///Get a mutable reference to the `aspect_references` field.
    pub fn aspect_references_mut(&mut self) -> &mut SmallVec<[InputAttachmentAspectReference; 8]> {
        &mut self.aspect_references
    }
    ///Sets the `aspect_references` field.
    pub fn set_aspect_references(
        &mut self,
        aspect_references: SmallVec<[InputAttachmentAspectReference; 8]>,
    ) -> &mut Self {
        self.aspect_references = aspect_references;
        self
    }
    ///Sets the `aspect_references` field in a builder way.
    pub fn with_aspect_references(mut self, aspect_references: SmallVec<[InputAttachmentAspectReference; 8]>) -> Self {
        self.aspect_references = aspect_references;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for RenderPassInputAttachmentAspectCreateInfo {
    type LowLevel = crate::native::vulkan1_1::RenderPassInputAttachmentAspectCreateInfo;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let len_aspect_references = self.aspect_references.len() as u32;
        let aspect_references = bump
            .alloc_slice_fill_iter(self.aspect_references.iter().map(|x| x.into_low_level(context, bump)))
            .as_ptr()
            .cast();
        crate::native::vulkan1_1::RenderPassInputAttachmentAspectCreateInfo {
            s_type: StructureType::RenderPassInputAttachmentAspectCreateInfo,
            p_next: std::ptr::null(),
            aspect_reference_count: len_aspect_references,
            aspect_references: aspect_references,
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for RenderPassInputAttachmentAspectCreateInfo {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let aspect_references_len = value.aspect_reference_count;
        let mut aspect_references = SmallVec::with_capacity(aspect_references_len as usize);
        for i in 0..aspect_references_len {
            aspect_references.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.aspect_references.add(i as usize).read(),
            ));
        }
        Self {
            aspect_references: aspect_references,
        }
    }
}
#[doc(alias = "VkPhysicalDevice16BitStorageFeatures")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDevice16BitStorageFeatures {
    #[doc(alias = "storageBuffer16BitAccess")]
    pub storage_buffer16_bit_access: bool,
    #[doc(alias = "uniformAndStorageBuffer16BitAccess")]
    pub uniform_and_storage_buffer16_bit_access: bool,
    #[doc(alias = "storagePushConstant16")]
    pub storage_push_constant16: bool,
    #[doc(alias = "storageInputOutput16")]
    pub storage_input_output16: bool,
}
impl PhysicalDevice16BitStorageFeatures {
    ///Get a reference to the `storage_buffer16_bit_access` field.
    pub fn storage_buffer16_bit_access(&self) -> &bool {
        &self.storage_buffer16_bit_access
    }
    ///Get a reference to the `uniform_and_storage_buffer16_bit_access` field.
    pub fn uniform_and_storage_buffer16_bit_access(&self) -> &bool {
        &self.uniform_and_storage_buffer16_bit_access
    }
    ///Get a reference to the `storage_push_constant16` field.
    pub fn storage_push_constant16(&self) -> &bool {
        &self.storage_push_constant16
    }
    ///Get a reference to the `storage_input_output16` field.
    pub fn storage_input_output16(&self) -> &bool {
        &self.storage_input_output16
    }
    ///Get a mutable reference to the `storage_buffer16_bit_access` field.
    pub fn storage_buffer16_bit_access_mut(&mut self) -> &mut bool {
        &mut self.storage_buffer16_bit_access
    }
    ///Get a mutable reference to the `uniform_and_storage_buffer16_bit_access` field.
    pub fn uniform_and_storage_buffer16_bit_access_mut(&mut self) -> &mut bool {
        &mut self.uniform_and_storage_buffer16_bit_access
    }
    ///Get a mutable reference to the `storage_push_constant16` field.
    pub fn storage_push_constant16_mut(&mut self) -> &mut bool {
        &mut self.storage_push_constant16
    }
    ///Get a mutable reference to the `storage_input_output16` field.
    pub fn storage_input_output16_mut(&mut self) -> &mut bool {
        &mut self.storage_input_output16
    }
    ///Sets the `storage_buffer16_bit_access` field.
    pub fn set_storage_buffer16_bit_access(&mut self, storage_buffer16_bit_access: bool) -> &mut Self {
        self.storage_buffer16_bit_access = storage_buffer16_bit_access;
        self
    }
    ///Sets the `uniform_and_storage_buffer16_bit_access` field.
    pub fn set_uniform_and_storage_buffer16_bit_access(
        &mut self,
        uniform_and_storage_buffer16_bit_access: bool,
    ) -> &mut Self {
        self.uniform_and_storage_buffer16_bit_access = uniform_and_storage_buffer16_bit_access;
        self
    }
    ///Sets the `storage_push_constant16` field.
    pub fn set_storage_push_constant16(&mut self, storage_push_constant16: bool) -> &mut Self {
        self.storage_push_constant16 = storage_push_constant16;
        self
    }
    ///Sets the `storage_input_output16` field.
    pub fn set_storage_input_output16(&mut self, storage_input_output16: bool) -> &mut Self {
        self.storage_input_output16 = storage_input_output16;
        self
    }
    ///Sets the `storage_buffer16_bit_access` field in a builder way.
    pub fn with_storage_buffer16_bit_access(mut self, storage_buffer16_bit_access: bool) -> Self {
        self.storage_buffer16_bit_access = storage_buffer16_bit_access;
        self
    }
    ///Sets the `uniform_and_storage_buffer16_bit_access` field in a builder way.
    pub fn with_uniform_and_storage_buffer16_bit_access(
        mut self,
        uniform_and_storage_buffer16_bit_access: bool,
    ) -> Self {
        self.uniform_and_storage_buffer16_bit_access = uniform_and_storage_buffer16_bit_access;
        self
    }
    ///Sets the `storage_push_constant16` field in a builder way.
    pub fn with_storage_push_constant16(mut self, storage_push_constant16: bool) -> Self {
        self.storage_push_constant16 = storage_push_constant16;
        self
    }
    ///Sets the `storage_input_output16` field in a builder way.
    pub fn with_storage_input_output16(mut self, storage_input_output16: bool) -> Self {
        self.storage_input_output16 = storage_input_output16;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDevice16BitStorageFeatures {
    type LowLevel = crate::native::vulkan1_1::PhysicalDevice16BitStorageFeatures;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_1::PhysicalDevice16BitStorageFeatures {
            s_type: StructureType::PhysicalDevice16bitStorageFeatures,
            p_next: std::ptr::null_mut(),
            storage_buffer16_bit_access: self.storage_buffer16_bit_access.into_low_level(context, bump),
            uniform_and_storage_buffer16_bit_access: self
                .uniform_and_storage_buffer16_bit_access
                .into_low_level(context, bump),
            storage_push_constant16: self.storage_push_constant16.into_low_level(context, bump),
            storage_input_output16: self.storage_input_output16.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDevice16BitStorageFeatures {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            storage_buffer16_bit_access: crate::conv::FromLowLevel::from_low_level(
                context,
                value.storage_buffer16_bit_access,
            ),
            uniform_and_storage_buffer16_bit_access: crate::conv::FromLowLevel::from_low_level(
                context,
                value.uniform_and_storage_buffer16_bit_access,
            ),
            storage_push_constant16: crate::conv::FromLowLevel::from_low_level(context, value.storage_push_constant16),
            storage_input_output16: crate::conv::FromLowLevel::from_low_level(context, value.storage_input_output16),
        }
    }
}
#[doc(alias = "VkPhysicalDeviceSubgroupProperties")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceSubgroupProperties {
    #[doc(alias = "subgroupSize")]
    pub subgroup_size: u32,
    #[doc(alias = "supportedStages")]
    pub supported_stages: ShaderStageFlags,
    #[doc(alias = "supportedOperations")]
    pub supported_operations: SubgroupFeatureFlags,
    #[doc(alias = "quadOperationsInAllStages")]
    pub quad_operations_in_all_stages: bool,
}
impl PhysicalDeviceSubgroupProperties {
    ///Get a reference to the `subgroup_size` field.
    pub fn subgroup_size(&self) -> u32 {
        self.subgroup_size
    }
    ///Get a reference to the `supported_stages` field.
    pub fn supported_stages(&self) -> ShaderStageFlags {
        self.supported_stages
    }
    ///Get a reference to the `supported_operations` field.
    pub fn supported_operations(&self) -> SubgroupFeatureFlags {
        self.supported_operations
    }
    ///Get a reference to the `quad_operations_in_all_stages` field.
    pub fn quad_operations_in_all_stages(&self) -> &bool {
        &self.quad_operations_in_all_stages
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceSubgroupProperties {
    type LowLevel = crate::native::vulkan1_1::PhysicalDeviceSubgroupProperties;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_1::PhysicalDeviceSubgroupProperties {
            s_type: StructureType::PhysicalDeviceSubgroupProperties,
            p_next: std::ptr::null_mut(),
            subgroup_size: self.subgroup_size.into_low_level(context, bump),
            supported_stages: self.supported_stages.into_low_level(context, bump),
            supported_operations: self.supported_operations.into_low_level(context, bump),
            quad_operations_in_all_stages: self.quad_operations_in_all_stages.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceSubgroupProperties {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            subgroup_size: crate::conv::FromLowLevel::from_low_level(context, value.subgroup_size),
            supported_stages: crate::conv::FromLowLevel::from_low_level(context, value.supported_stages),
            supported_operations: crate::conv::FromLowLevel::from_low_level(context, value.supported_operations),
            quad_operations_in_all_stages: crate::conv::FromLowLevel::from_low_level(
                context,
                value.quad_operations_in_all_stages,
            ),
        }
    }
}
#[doc(alias = "VkBufferMemoryRequirementsInfo2")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BufferMemoryRequirementsInfo2 {
    pub buffer: Buffer,
}
impl BufferMemoryRequirementsInfo2 {
    ///Get a reference to the `buffer` field.
    pub fn buffer(&self) -> &Buffer {
        &self.buffer
    }
    ///Get a mutable reference to the `buffer` field.
    pub fn buffer_mut(&mut self) -> &mut Buffer {
        &mut self.buffer
    }
    ///Sets the `buffer` field.
    pub fn set_buffer(&mut self, buffer: Buffer) -> &mut Self {
        self.buffer = buffer;
        self
    }
    ///Sets the `buffer` field in a builder way.
    pub fn with_buffer(mut self, buffer: Buffer) -> Self {
        self.buffer = buffer;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for BufferMemoryRequirementsInfo2 {
    type LowLevel = crate::native::vulkan1_1::BufferMemoryRequirementsInfo2;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_1::BufferMemoryRequirementsInfo2 {
            s_type: StructureType::BufferMemoryRequirementsInfo2,
            p_next: std::ptr::null(),
            buffer: self.buffer.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for BufferMemoryRequirementsInfo2 {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            buffer: crate::conv::FromLowLevel::from_low_level(context, value.buffer),
        }
    }
}
#[doc(alias = "VkImageMemoryRequirementsInfo2")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ImageMemoryRequirementsInfo2 {
    #[doc(alias = "pNext")]
    pub extensions: SmallVec<[ImageMemoryRequirementsInfo2Extension; 1]>,
    pub image: Image,
}
impl ImageMemoryRequirementsInfo2 {
    ///Adds an extension to the struct
    pub fn with_extension(mut self, ext: impl Into<ImageMemoryRequirementsInfo2Extension>) -> Self {
        self.extensions.push(ext.into());
        self
    }
    ///Get a reference to the `extensions` field.
    pub fn extensions(&self) -> &SmallVec<[ImageMemoryRequirementsInfo2Extension; 1]> {
        &self.extensions
    }
    ///Get a reference to the `image` field.
    pub fn image(&self) -> &Image {
        &self.image
    }
    ///Get a mutable reference to the `extensions` field.
    pub fn extensions_mut(&mut self) -> &mut SmallVec<[ImageMemoryRequirementsInfo2Extension; 1]> {
        &mut self.extensions
    }
    ///Get a mutable reference to the `image` field.
    pub fn image_mut(&mut self) -> &mut Image {
        &mut self.image
    }
    ///Sets the `extensions` field.
    pub fn set_extensions(&mut self, extensions: SmallVec<[ImageMemoryRequirementsInfo2Extension; 1]>) -> &mut Self {
        self.extensions = extensions;
        self
    }
    ///Sets the `image` field.
    pub fn set_image(&mut self, image: Image) -> &mut Self {
        self.image = image;
        self
    }
    ///Sets the `extensions` field in a builder way.
    pub fn with_extensions(mut self, extensions: SmallVec<[ImageMemoryRequirementsInfo2Extension; 1]>) -> Self {
        self.extensions = extensions;
        self
    }
    ///Sets the `image` field in a builder way.
    pub fn with_image(mut self, image: Image) -> Self {
        self.image = image;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for ImageMemoryRequirementsInfo2 {
    type LowLevel = crate::native::vulkan1_1::ImageMemoryRequirementsInfo2;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let mut next = std::ptr::null();
        let mut extensions = self.extensions.iter();
        while let Some(ext) = extensions.next() {
            let ext = ext.into_low_level(context, bump);
            (*ext).next = next;
            next = ext;
        }
        crate::native::vulkan1_1::ImageMemoryRequirementsInfo2 {
            s_type: StructureType::ImageMemoryRequirementsInfo2,
            p_next: next,
            image: self.image.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for ImageMemoryRequirementsInfo2 {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let mut next = value.p_next;
        let mut extensions = SmallVec::new();
        while !next.is_null() {
            extensions.push(crate::conv::FromLowLevel::from_low_level(context, next));
            next = std::ptr::read(next).next;
        }
        Self {
            extensions: extensions,
            image: crate::conv::FromLowLevel::from_low_level(context, value.image),
        }
    }
}
#[derive(Clone, PartialEq, Debug, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
///Extensions for [`ImageMemoryRequirementsInfo2`]
pub enum ImageMemoryRequirementsInfo2Extension {
    ///Contains a type [`ImagePlaneMemoryRequirementsInfo`] for extending
    /// [`ImageMemoryRequirementsInfo2`]
    ImagePlaneMemoryRequirementsInfo(ImagePlaneMemoryRequirementsInfo),
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for ImageMemoryRequirementsInfo2Extension {
    type LowLevel = *const crate::native::vulkan1_0::BaseInStructure;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        match self {
            Self::ImagePlaneMemoryRequirementsInfo(ext) => (bump.alloc(ext.into_low_level(context, bump))
                as *mut crate::native::vulkan1_1::ImagePlaneMemoryRequirementsInfo)
                .cast(),
            other => unreachable!("unexpected variant {:?}", other),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for ImageMemoryRequirementsInfo2Extension {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        assert!(!value.is_null());
        match (*value).s_type {
            crate::native::vulkan1_0::StructureType::ImagePlaneMemoryRequirementsInfo => {
                Self::ImagePlaneMemoryRequirementsInfo(ImagePlaneMemoryRequirementsInfo::from_low_level(
                    context,
                    std::ptr::read(value.cast::<crate::native::vulkan1_1::ImagePlaneMemoryRequirementsInfo>()),
                ))
            },
            other => panic!(
                "Structure type {:?} is not a member of {}",
                other,
                stringify!(ImageMemoryRequirementsInfo2)
            ),
        }
    }
}
impl From<ImagePlaneMemoryRequirementsInfo> for ImageMemoryRequirementsInfo2Extension {
    fn from(ext: ImagePlaneMemoryRequirementsInfo) -> Self {
        Self::ImagePlaneMemoryRequirementsInfo(ext)
    }
}
impl TryInto<ImagePlaneMemoryRequirementsInfo> for ImageMemoryRequirementsInfo2Extension {
    type Error = ImageMemoryRequirementsInfo2Extension;
    fn try_into(self) -> Result<ImagePlaneMemoryRequirementsInfo, Self::Error> {
        match self {
            Self::ImagePlaneMemoryRequirementsInfo(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[doc(alias = "VkImageSparseMemoryRequirementsInfo2")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ImageSparseMemoryRequirementsInfo2 {
    pub image: Image,
}
impl ImageSparseMemoryRequirementsInfo2 {
    ///Get a reference to the `image` field.
    pub fn image(&self) -> &Image {
        &self.image
    }
    ///Get a mutable reference to the `image` field.
    pub fn image_mut(&mut self) -> &mut Image {
        &mut self.image
    }
    ///Sets the `image` field.
    pub fn set_image(&mut self, image: Image) -> &mut Self {
        self.image = image;
        self
    }
    ///Sets the `image` field in a builder way.
    pub fn with_image(mut self, image: Image) -> Self {
        self.image = image;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for ImageSparseMemoryRequirementsInfo2 {
    type LowLevel = crate::native::vulkan1_1::ImageSparseMemoryRequirementsInfo2;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_1::ImageSparseMemoryRequirementsInfo2 {
            s_type: StructureType::ImageSparseMemoryRequirementsInfo2,
            p_next: std::ptr::null(),
            image: self.image.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for ImageSparseMemoryRequirementsInfo2 {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            image: crate::conv::FromLowLevel::from_low_level(context, value.image),
        }
    }
}
#[doc(alias = "VkMemoryRequirements2")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MemoryRequirements2 {
    #[doc(alias = "pNext")]
    pub extensions: SmallVec<[MemoryRequirements2Extension; 1]>,
    #[doc(alias = "memoryRequirements")]
    pub memory_requirements: MemoryRequirements,
}
impl MemoryRequirements2 {
    ///Adds an extension to the struct
    pub fn with_extension(mut self, ext: impl Into<MemoryRequirements2Extension>) -> Self {
        self.extensions.push(ext.into());
        self
    }
    ///Get a reference to the `extensions` field.
    pub fn extensions(&self) -> &SmallVec<[MemoryRequirements2Extension; 1]> {
        &self.extensions
    }
    ///Get a reference to the `memory_requirements` field.
    pub fn memory_requirements(&self) -> &MemoryRequirements {
        &self.memory_requirements
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for MemoryRequirements2 {
    type LowLevel = crate::native::vulkan1_1::MemoryRequirements2;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let mut next = std::ptr::null_mut();
        let mut extensions = self.extensions.iter();
        while let Some(ext) = extensions.next() {
            let ext = ext.into_low_level(context, bump);
            (*ext).next = next;
            next = ext;
        }
        crate::native::vulkan1_1::MemoryRequirements2 {
            s_type: StructureType::MemoryRequirements2,
            p_next: next,
            memory_requirements: self.memory_requirements.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for MemoryRequirements2 {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let mut next = value.p_next;
        let mut extensions = SmallVec::new();
        while !next.is_null() {
            extensions.push(crate::conv::FromLowLevel::from_low_level(context, next));
            next = std::ptr::read(next).next;
        }
        Self {
            extensions: extensions,
            memory_requirements: crate::conv::FromLowLevel::from_low_level(context, value.memory_requirements),
        }
    }
}
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
///Extensions for [`MemoryRequirements2`]
pub enum MemoryRequirements2Extension {
    ///Contains a type [`MemoryDedicatedRequirements`] for extending [`MemoryRequirements2`]
    MemoryDedicatedRequirements(MemoryDedicatedRequirements),
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for MemoryRequirements2Extension {
    type LowLevel = *mut crate::native::vulkan1_0::BaseOutStructure;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        match self {
            Self::MemoryDedicatedRequirements(ext) => (bump.alloc(ext.into_low_level(context, bump))
                as *mut crate::native::vulkan1_1::MemoryDedicatedRequirements)
                .cast(),
            other => unreachable!("unexpected variant {:?}", other),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for MemoryRequirements2Extension {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        assert!(!value.is_null());
        match (*value).s_type {
            crate::native::vulkan1_0::StructureType::MemoryDedicatedRequirements => {
                Self::MemoryDedicatedRequirements(MemoryDedicatedRequirements::from_low_level(
                    context,
                    std::ptr::read(value.cast::<crate::native::vulkan1_1::MemoryDedicatedRequirements>()),
                ))
            },
            other => panic!(
                "Structure type {:?} is not a member of {}",
                other,
                stringify!(MemoryRequirements2)
            ),
        }
    }
}
impl From<MemoryDedicatedRequirements> for MemoryRequirements2Extension {
    fn from(ext: MemoryDedicatedRequirements) -> Self {
        Self::MemoryDedicatedRequirements(ext)
    }
}
impl TryInto<MemoryDedicatedRequirements> for MemoryRequirements2Extension {
    type Error = MemoryRequirements2Extension;
    fn try_into(self) -> Result<MemoryDedicatedRequirements, Self::Error> {
        match self {
            Self::MemoryDedicatedRequirements(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[doc(alias = "VkSparseImageMemoryRequirements2")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SparseImageMemoryRequirements2 {
    #[doc(alias = "memoryRequirements")]
    pub memory_requirements: SparseImageMemoryRequirements,
}
impl SparseImageMemoryRequirements2 {
    ///Get a reference to the `memory_requirements` field.
    pub fn memory_requirements(&self) -> &SparseImageMemoryRequirements {
        &self.memory_requirements
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for SparseImageMemoryRequirements2 {
    type LowLevel = crate::native::vulkan1_1::SparseImageMemoryRequirements2;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_1::SparseImageMemoryRequirements2 {
            s_type: StructureType::SparseImageMemoryRequirements2,
            p_next: std::ptr::null_mut(),
            memory_requirements: self.memory_requirements.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for SparseImageMemoryRequirements2 {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            memory_requirements: crate::conv::FromLowLevel::from_low_level(context, value.memory_requirements),
        }
    }
}
#[doc(alias = "VkPhysicalDevicePointClippingProperties")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDevicePointClippingProperties {
    #[doc(alias = "pointClippingBehavior")]
    pub point_clipping_behavior: PointClippingBehavior,
}
impl PhysicalDevicePointClippingProperties {
    ///Get a reference to the `point_clipping_behavior` field.
    pub fn point_clipping_behavior(&self) -> PointClippingBehavior {
        self.point_clipping_behavior
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDevicePointClippingProperties {
    type LowLevel = crate::native::vulkan1_1::PhysicalDevicePointClippingProperties;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_1::PhysicalDevicePointClippingProperties {
            s_type: StructureType::PhysicalDevicePointClippingProperties,
            p_next: std::ptr::null_mut(),
            point_clipping_behavior: self.point_clipping_behavior.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDevicePointClippingProperties {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            point_clipping_behavior: crate::conv::FromLowLevel::from_low_level(context, value.point_clipping_behavior),
        }
    }
}
#[doc(alias = "VkMemoryDedicatedRequirements")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MemoryDedicatedRequirements {
    #[doc(alias = "prefersDedicatedAllocation")]
    pub prefers_dedicated_allocation: bool,
    #[doc(alias = "requiresDedicatedAllocation")]
    pub requires_dedicated_allocation: bool,
}
impl MemoryDedicatedRequirements {
    ///Get a reference to the `prefers_dedicated_allocation` field.
    pub fn prefers_dedicated_allocation(&self) -> &bool {
        &self.prefers_dedicated_allocation
    }
    ///Get a reference to the `requires_dedicated_allocation` field.
    pub fn requires_dedicated_allocation(&self) -> &bool {
        &self.requires_dedicated_allocation
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for MemoryDedicatedRequirements {
    type LowLevel = crate::native::vulkan1_1::MemoryDedicatedRequirements;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_1::MemoryDedicatedRequirements {
            s_type: StructureType::MemoryDedicatedRequirements,
            p_next: std::ptr::null_mut(),
            prefers_dedicated_allocation: self.prefers_dedicated_allocation.into_low_level(context, bump),
            requires_dedicated_allocation: self.requires_dedicated_allocation.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for MemoryDedicatedRequirements {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            prefers_dedicated_allocation: crate::conv::FromLowLevel::from_low_level(
                context,
                value.prefers_dedicated_allocation,
            ),
            requires_dedicated_allocation: crate::conv::FromLowLevel::from_low_level(
                context,
                value.requires_dedicated_allocation,
            ),
        }
    }
}
#[doc(alias = "VkMemoryDedicatedAllocateInfo")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MemoryDedicatedAllocateInfo {
    pub image: Option<Image>,
    pub buffer: Option<Buffer>,
}
impl MemoryDedicatedAllocateInfo {
    ///Get a reference to the `image` field.
    pub fn image(&self) -> &Option<Image> {
        &self.image
    }
    ///Get a reference to the `buffer` field.
    pub fn buffer(&self) -> &Option<Buffer> {
        &self.buffer
    }
    ///Get a mutable reference to the `image` field.
    pub fn image_mut(&mut self) -> &mut Option<Image> {
        &mut self.image
    }
    ///Get a mutable reference to the `buffer` field.
    pub fn buffer_mut(&mut self) -> &mut Option<Buffer> {
        &mut self.buffer
    }
    ///Sets the `image` field.
    pub fn set_image(&mut self, image: Option<Image>) -> &mut Self {
        self.image = image;
        self
    }
    ///Sets the `buffer` field.
    pub fn set_buffer(&mut self, buffer: Option<Buffer>) -> &mut Self {
        self.buffer = buffer;
        self
    }
    ///Sets the `image` field in a builder way.
    pub fn with_image(mut self, image: Option<Image>) -> Self {
        self.image = image;
        self
    }
    ///Sets the `buffer` field in a builder way.
    pub fn with_buffer(mut self, buffer: Option<Buffer>) -> Self {
        self.buffer = buffer;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for MemoryDedicatedAllocateInfo {
    type LowLevel = crate::native::vulkan1_1::MemoryDedicatedAllocateInfo;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_1::MemoryDedicatedAllocateInfo {
            s_type: StructureType::MemoryDedicatedAllocateInfo,
            p_next: std::ptr::null(),
            image: self
                .image
                .as_ref()
                .map(|v| v.into_low_level(context, bump))
                .unwrap_or_else(Default::default),
            buffer: self
                .buffer
                .as_ref()
                .map(|v| v.into_low_level(context, bump))
                .unwrap_or_else(Default::default),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for MemoryDedicatedAllocateInfo {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            image: if value.image == crate::native::vulkan1_0::Image::null() {
                None
            } else {
                Some(crate::conv::FromLowLevel::from_low_level(context, value.image))
            },
            buffer: if value.buffer == crate::native::vulkan1_0::Buffer::null() {
                None
            } else {
                Some(crate::conv::FromLowLevel::from_low_level(context, value.buffer))
            },
        }
    }
}
#[doc(alias = "VkImageViewUsageCreateInfo")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ImageViewUsageCreateInfo {
    pub usage: ImageUsageFlags,
}
impl ImageViewUsageCreateInfo {
    ///Get a reference to the `usage` field.
    pub fn usage(&self) -> ImageUsageFlags {
        self.usage
    }
    ///Get a mutable reference to the `usage` field.
    pub fn usage_mut(&mut self) -> &mut ImageUsageFlags {
        &mut self.usage
    }
    ///Sets the `usage` field.
    pub fn set_usage(&mut self, usage: ImageUsageFlags) -> &mut Self {
        self.usage = usage;
        self
    }
    ///Sets the `usage` field in a builder way.
    pub fn with_usage(mut self, usage: ImageUsageFlags) -> Self {
        self.usage = usage;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for ImageViewUsageCreateInfo {
    type LowLevel = crate::native::vulkan1_1::ImageViewUsageCreateInfo;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_1::ImageViewUsageCreateInfo {
            s_type: StructureType::ImageViewUsageCreateInfo,
            p_next: std::ptr::null(),
            usage: self.usage.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for ImageViewUsageCreateInfo {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            usage: crate::conv::FromLowLevel::from_low_level(context, value.usage),
        }
    }
}
#[doc(alias = "VkPipelineTessellationDomainOriginStateCreateInfo")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PipelineTessellationDomainOriginStateCreateInfo {
    #[doc(alias = "domainOrigin")]
    pub domain_origin: TessellationDomainOrigin,
}
impl PipelineTessellationDomainOriginStateCreateInfo {
    ///Get a reference to the `domain_origin` field.
    pub fn domain_origin(&self) -> TessellationDomainOrigin {
        self.domain_origin
    }
    ///Get a mutable reference to the `domain_origin` field.
    pub fn domain_origin_mut(&mut self) -> &mut TessellationDomainOrigin {
        &mut self.domain_origin
    }
    ///Sets the `domain_origin` field.
    pub fn set_domain_origin(&mut self, domain_origin: TessellationDomainOrigin) -> &mut Self {
        self.domain_origin = domain_origin;
        self
    }
    ///Sets the `domain_origin` field in a builder way.
    pub fn with_domain_origin(mut self, domain_origin: TessellationDomainOrigin) -> Self {
        self.domain_origin = domain_origin;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PipelineTessellationDomainOriginStateCreateInfo {
    type LowLevel = crate::native::vulkan1_1::PipelineTessellationDomainOriginStateCreateInfo;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_1::PipelineTessellationDomainOriginStateCreateInfo {
            s_type: StructureType::PipelineTessellationDomainOriginStateCreateInfo,
            p_next: std::ptr::null(),
            domain_origin: self.domain_origin.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PipelineTessellationDomainOriginStateCreateInfo {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            domain_origin: crate::conv::FromLowLevel::from_low_level(context, value.domain_origin),
        }
    }
}
#[doc(alias = "VkSamplerYcbcrConversionInfo")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SamplerYcbcrConversionInfo {
    pub conversion: SamplerYcbcrConversion,
}
impl SamplerYcbcrConversionInfo {
    ///Get a reference to the `conversion` field.
    pub fn conversion(&self) -> &SamplerYcbcrConversion {
        &self.conversion
    }
    ///Get a mutable reference to the `conversion` field.
    pub fn conversion_mut(&mut self) -> &mut SamplerYcbcrConversion {
        &mut self.conversion
    }
    ///Sets the `conversion` field.
    pub fn set_conversion(&mut self, conversion: SamplerYcbcrConversion) -> &mut Self {
        self.conversion = conversion;
        self
    }
    ///Sets the `conversion` field in a builder way.
    pub fn with_conversion(mut self, conversion: SamplerYcbcrConversion) -> Self {
        self.conversion = conversion;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for SamplerYcbcrConversionInfo {
    type LowLevel = crate::native::vulkan1_1::SamplerYcbcrConversionInfo;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_1::SamplerYcbcrConversionInfo {
            s_type: StructureType::SamplerYcbcrConversionInfo,
            p_next: std::ptr::null(),
            conversion: self.conversion.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for SamplerYcbcrConversionInfo {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            conversion: crate::conv::FromLowLevel::from_low_level(context, value.conversion),
        }
    }
}
#[doc(alias = "VkSamplerYcbcrConversionCreateInfo")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SamplerYcbcrConversionCreateInfo {
    #[doc(alias = "pNext")]
    pub extensions: SmallVec<[SamplerYcbcrConversionCreateInfoExtension; 1]>,
    pub format: Format,
    #[doc(alias = "ycbcrModel")]
    pub ycbcr_model: SamplerYcbcrModelConversion,
    #[doc(alias = "ycbcrRange")]
    pub ycbcr_range: SamplerYcbcrRange,
    pub components: ComponentMapping,
    #[doc(alias = "xChromaOffset")]
    pub x_chroma_offset: ChromaLocation,
    #[doc(alias = "yChromaOffset")]
    pub y_chroma_offset: ChromaLocation,
    #[doc(alias = "chromaFilter")]
    pub chroma_filter: Filter,
    #[doc(alias = "forceExplicitReconstruction")]
    pub force_explicit_reconstruction: bool,
}
impl SamplerYcbcrConversionCreateInfo {
    ///Adds an extension to the struct
    pub fn with_extension(mut self, ext: impl Into<SamplerYcbcrConversionCreateInfoExtension>) -> Self {
        self.extensions.push(ext.into());
        self
    }
    ///Get a reference to the `extensions` field.
    pub fn extensions(&self) -> &SmallVec<[SamplerYcbcrConversionCreateInfoExtension; 1]> {
        &self.extensions
    }
    ///Get a reference to the `format` field.
    pub fn format(&self) -> Format {
        self.format
    }
    ///Get a reference to the `ycbcr_model` field.
    pub fn ycbcr_model(&self) -> SamplerYcbcrModelConversion {
        self.ycbcr_model
    }
    ///Get a reference to the `ycbcr_range` field.
    pub fn ycbcr_range(&self) -> SamplerYcbcrRange {
        self.ycbcr_range
    }
    ///Get a reference to the `components` field.
    pub fn components(&self) -> ComponentMapping {
        self.components
    }
    ///Get a reference to the `x_chroma_offset` field.
    pub fn x_chroma_offset(&self) -> ChromaLocation {
        self.x_chroma_offset
    }
    ///Get a reference to the `y_chroma_offset` field.
    pub fn y_chroma_offset(&self) -> ChromaLocation {
        self.y_chroma_offset
    }
    ///Get a reference to the `chroma_filter` field.
    pub fn chroma_filter(&self) -> Filter {
        self.chroma_filter
    }
    ///Get a reference to the `force_explicit_reconstruction` field.
    pub fn force_explicit_reconstruction(&self) -> &bool {
        &self.force_explicit_reconstruction
    }
    ///Get a mutable reference to the `extensions` field.
    pub fn extensions_mut(&mut self) -> &mut SmallVec<[SamplerYcbcrConversionCreateInfoExtension; 1]> {
        &mut self.extensions
    }
    ///Get a mutable reference to the `format` field.
    pub fn format_mut(&mut self) -> &mut Format {
        &mut self.format
    }
    ///Get a mutable reference to the `ycbcr_model` field.
    pub fn ycbcr_model_mut(&mut self) -> &mut SamplerYcbcrModelConversion {
        &mut self.ycbcr_model
    }
    ///Get a mutable reference to the `ycbcr_range` field.
    pub fn ycbcr_range_mut(&mut self) -> &mut SamplerYcbcrRange {
        &mut self.ycbcr_range
    }
    ///Get a mutable reference to the `components` field.
    pub fn components_mut(&mut self) -> &mut ComponentMapping {
        &mut self.components
    }
    ///Get a mutable reference to the `x_chroma_offset` field.
    pub fn x_chroma_offset_mut(&mut self) -> &mut ChromaLocation {
        &mut self.x_chroma_offset
    }
    ///Get a mutable reference to the `y_chroma_offset` field.
    pub fn y_chroma_offset_mut(&mut self) -> &mut ChromaLocation {
        &mut self.y_chroma_offset
    }
    ///Get a mutable reference to the `chroma_filter` field.
    pub fn chroma_filter_mut(&mut self) -> &mut Filter {
        &mut self.chroma_filter
    }
    ///Get a mutable reference to the `force_explicit_reconstruction` field.
    pub fn force_explicit_reconstruction_mut(&mut self) -> &mut bool {
        &mut self.force_explicit_reconstruction
    }
    ///Sets the `extensions` field.
    pub fn set_extensions(
        &mut self,
        extensions: SmallVec<[SamplerYcbcrConversionCreateInfoExtension; 1]>,
    ) -> &mut Self {
        self.extensions = extensions;
        self
    }
    ///Sets the `format` field.
    pub fn set_format(&mut self, format: Format) -> &mut Self {
        self.format = format;
        self
    }
    ///Sets the `ycbcr_model` field.
    pub fn set_ycbcr_model(&mut self, ycbcr_model: SamplerYcbcrModelConversion) -> &mut Self {
        self.ycbcr_model = ycbcr_model;
        self
    }
    ///Sets the `ycbcr_range` field.
    pub fn set_ycbcr_range(&mut self, ycbcr_range: SamplerYcbcrRange) -> &mut Self {
        self.ycbcr_range = ycbcr_range;
        self
    }
    ///Sets the `components` field.
    pub fn set_components(&mut self, components: ComponentMapping) -> &mut Self {
        self.components = components;
        self
    }
    ///Sets the `x_chroma_offset` field.
    pub fn set_x_chroma_offset(&mut self, x_chroma_offset: ChromaLocation) -> &mut Self {
        self.x_chroma_offset = x_chroma_offset;
        self
    }
    ///Sets the `y_chroma_offset` field.
    pub fn set_y_chroma_offset(&mut self, y_chroma_offset: ChromaLocation) -> &mut Self {
        self.y_chroma_offset = y_chroma_offset;
        self
    }
    ///Sets the `chroma_filter` field.
    pub fn set_chroma_filter(&mut self, chroma_filter: Filter) -> &mut Self {
        self.chroma_filter = chroma_filter;
        self
    }
    ///Sets the `force_explicit_reconstruction` field.
    pub fn set_force_explicit_reconstruction(&mut self, force_explicit_reconstruction: bool) -> &mut Self {
        self.force_explicit_reconstruction = force_explicit_reconstruction;
        self
    }
    ///Sets the `extensions` field in a builder way.
    pub fn with_extensions(mut self, extensions: SmallVec<[SamplerYcbcrConversionCreateInfoExtension; 1]>) -> Self {
        self.extensions = extensions;
        self
    }
    ///Sets the `format` field in a builder way.
    pub fn with_format(mut self, format: Format) -> Self {
        self.format = format;
        self
    }
    ///Sets the `ycbcr_model` field in a builder way.
    pub fn with_ycbcr_model(mut self, ycbcr_model: SamplerYcbcrModelConversion) -> Self {
        self.ycbcr_model = ycbcr_model;
        self
    }
    ///Sets the `ycbcr_range` field in a builder way.
    pub fn with_ycbcr_range(mut self, ycbcr_range: SamplerYcbcrRange) -> Self {
        self.ycbcr_range = ycbcr_range;
        self
    }
    ///Sets the `components` field in a builder way.
    pub fn with_components(mut self, components: ComponentMapping) -> Self {
        self.components = components;
        self
    }
    ///Sets the `x_chroma_offset` field in a builder way.
    pub fn with_x_chroma_offset(mut self, x_chroma_offset: ChromaLocation) -> Self {
        self.x_chroma_offset = x_chroma_offset;
        self
    }
    ///Sets the `y_chroma_offset` field in a builder way.
    pub fn with_y_chroma_offset(mut self, y_chroma_offset: ChromaLocation) -> Self {
        self.y_chroma_offset = y_chroma_offset;
        self
    }
    ///Sets the `chroma_filter` field in a builder way.
    pub fn with_chroma_filter(mut self, chroma_filter: Filter) -> Self {
        self.chroma_filter = chroma_filter;
        self
    }
    ///Sets the `force_explicit_reconstruction` field in a builder way.
    pub fn with_force_explicit_reconstruction(mut self, force_explicit_reconstruction: bool) -> Self {
        self.force_explicit_reconstruction = force_explicit_reconstruction;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for SamplerYcbcrConversionCreateInfo {
    type LowLevel = crate::native::vulkan1_1::SamplerYcbcrConversionCreateInfo;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let mut next = std::ptr::null();
        let mut extensions = self.extensions.iter();
        while let Some(ext) = extensions.next() {
            let ext = ext.into_low_level(context, bump);
            (*ext).next = next;
            next = ext;
        }
        crate::native::vulkan1_1::SamplerYcbcrConversionCreateInfo {
            s_type: StructureType::SamplerYcbcrConversionCreateInfo,
            p_next: next,
            format: self.format.into_low_level(context, bump),
            ycbcr_model: self.ycbcr_model.into_low_level(context, bump),
            ycbcr_range: self.ycbcr_range.into_low_level(context, bump),
            components: self.components.into_low_level(context, bump),
            x_chroma_offset: self.x_chroma_offset.into_low_level(context, bump),
            y_chroma_offset: self.y_chroma_offset.into_low_level(context, bump),
            chroma_filter: self.chroma_filter.into_low_level(context, bump),
            force_explicit_reconstruction: self.force_explicit_reconstruction.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for SamplerYcbcrConversionCreateInfo {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let mut next = value.p_next;
        let mut extensions = SmallVec::new();
        while !next.is_null() {
            extensions.push(crate::conv::FromLowLevel::from_low_level(context, next));
            next = std::ptr::read(next).next;
        }
        Self {
            extensions: extensions,
            format: crate::conv::FromLowLevel::from_low_level(context, value.format),
            ycbcr_model: crate::conv::FromLowLevel::from_low_level(context, value.ycbcr_model),
            ycbcr_range: crate::conv::FromLowLevel::from_low_level(context, value.ycbcr_range),
            components: crate::conv::FromLowLevel::from_low_level(context, value.components),
            x_chroma_offset: crate::conv::FromLowLevel::from_low_level(context, value.x_chroma_offset),
            y_chroma_offset: crate::conv::FromLowLevel::from_low_level(context, value.y_chroma_offset),
            chroma_filter: crate::conv::FromLowLevel::from_low_level(context, value.chroma_filter),
            force_explicit_reconstruction: crate::conv::FromLowLevel::from_low_level(
                context,
                value.force_explicit_reconstruction,
            ),
        }
    }
}
#[derive(Clone, PartialEq, Debug, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
///Extensions for [`SamplerYcbcrConversionCreateInfo`]
pub enum SamplerYcbcrConversionCreateInfoExtension {
    #[cfg(feature = "VK_ANDROID_external_memory_android_hardware_buffer")]
    ///Contains a type [`ExternalFormatANDROID`] for extending [`SamplerYcbcrConversionCreateInfo`]
    ExternalFormatANDROID(ExternalFormatANDROID),
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for SamplerYcbcrConversionCreateInfoExtension {
    type LowLevel = *const crate::native::vulkan1_0::BaseInStructure;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        match self { # [cfg (feature = "VK_ANDROID_external_memory_android_hardware_buffer")] Self :: ExternalFormatANDROID (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: android_external_memory_android_hardware_buffer :: ExternalFormatANDROID) . cast () , other => unreachable ! ("unexpected variant {:?}" , other) }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for SamplerYcbcrConversionCreateInfoExtension {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        assert!(!value.is_null());
        match (* value) . s_type { # [cfg (feature = "VK_ANDROID_external_memory_android_hardware_buffer")] crate :: native :: vulkan1_0 :: StructureType :: ExternalFormatAndroid => Self :: ExternalFormatANDROID (ExternalFormatANDROID :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: android_external_memory_android_hardware_buffer :: ExternalFormatANDROID > ()))) , other => panic ! ("Structure type {:?} is not a member of {}" , other , stringify ! (SamplerYcbcrConversionCreateInfo)) }
    }
}
#[cfg(feature = "VK_ANDROID_external_memory_android_hardware_buffer")]
impl From<ExternalFormatANDROID> for SamplerYcbcrConversionCreateInfoExtension {
    fn from(ext: ExternalFormatANDROID) -> Self {
        Self::ExternalFormatANDROID(ext)
    }
}
#[cfg(feature = "VK_ANDROID_external_memory_android_hardware_buffer")]
impl TryInto<ExternalFormatANDROID> for SamplerYcbcrConversionCreateInfoExtension {
    type Error = SamplerYcbcrConversionCreateInfoExtension;
    fn try_into(self) -> Result<ExternalFormatANDROID, Self::Error> {
        match self {
            Self::ExternalFormatANDROID(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[doc(alias = "VkBindImagePlaneMemoryInfo")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BindImagePlaneMemoryInfo {
    #[doc(alias = "planeAspect")]
    pub plane_aspect: ImageAspectFlagBits,
}
impl BindImagePlaneMemoryInfo {
    ///Get a reference to the `plane_aspect` field.
    pub fn plane_aspect(&self) -> ImageAspectFlagBits {
        self.plane_aspect
    }
    ///Get a mutable reference to the `plane_aspect` field.
    pub fn plane_aspect_mut(&mut self) -> &mut ImageAspectFlagBits {
        &mut self.plane_aspect
    }
    ///Sets the `plane_aspect` field.
    pub fn set_plane_aspect(&mut self, plane_aspect: ImageAspectFlagBits) -> &mut Self {
        self.plane_aspect = plane_aspect;
        self
    }
    ///Sets the `plane_aspect` field in a builder way.
    pub fn with_plane_aspect(mut self, plane_aspect: ImageAspectFlagBits) -> Self {
        self.plane_aspect = plane_aspect;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for BindImagePlaneMemoryInfo {
    type LowLevel = crate::native::vulkan1_1::BindImagePlaneMemoryInfo;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_1::BindImagePlaneMemoryInfo {
            s_type: StructureType::BindImagePlaneMemoryInfo,
            p_next: std::ptr::null(),
            plane_aspect: self.plane_aspect.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for BindImagePlaneMemoryInfo {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            plane_aspect: crate::conv::FromLowLevel::from_low_level(context, value.plane_aspect),
        }
    }
}
#[doc(alias = "VkImagePlaneMemoryRequirementsInfo")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ImagePlaneMemoryRequirementsInfo {
    #[doc(alias = "planeAspect")]
    pub plane_aspect: ImageAspectFlagBits,
}
impl ImagePlaneMemoryRequirementsInfo {
    ///Get a reference to the `plane_aspect` field.
    pub fn plane_aspect(&self) -> ImageAspectFlagBits {
        self.plane_aspect
    }
    ///Get a mutable reference to the `plane_aspect` field.
    pub fn plane_aspect_mut(&mut self) -> &mut ImageAspectFlagBits {
        &mut self.plane_aspect
    }
    ///Sets the `plane_aspect` field.
    pub fn set_plane_aspect(&mut self, plane_aspect: ImageAspectFlagBits) -> &mut Self {
        self.plane_aspect = plane_aspect;
        self
    }
    ///Sets the `plane_aspect` field in a builder way.
    pub fn with_plane_aspect(mut self, plane_aspect: ImageAspectFlagBits) -> Self {
        self.plane_aspect = plane_aspect;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for ImagePlaneMemoryRequirementsInfo {
    type LowLevel = crate::native::vulkan1_1::ImagePlaneMemoryRequirementsInfo;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_1::ImagePlaneMemoryRequirementsInfo {
            s_type: StructureType::ImagePlaneMemoryRequirementsInfo,
            p_next: std::ptr::null(),
            plane_aspect: self.plane_aspect.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for ImagePlaneMemoryRequirementsInfo {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            plane_aspect: crate::conv::FromLowLevel::from_low_level(context, value.plane_aspect),
        }
    }
}
#[doc(alias = "VkPhysicalDeviceSamplerYcbcrConversionFeatures")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceSamplerYcbcrConversionFeatures {
    #[doc(alias = "samplerYcbcrConversion")]
    pub sampler_ycbcr_conversion: bool,
}
impl PhysicalDeviceSamplerYcbcrConversionFeatures {
    ///Get a reference to the `sampler_ycbcr_conversion` field.
    pub fn sampler_ycbcr_conversion(&self) -> &bool {
        &self.sampler_ycbcr_conversion
    }
    ///Get a mutable reference to the `sampler_ycbcr_conversion` field.
    pub fn sampler_ycbcr_conversion_mut(&mut self) -> &mut bool {
        &mut self.sampler_ycbcr_conversion
    }
    ///Sets the `sampler_ycbcr_conversion` field.
    pub fn set_sampler_ycbcr_conversion(&mut self, sampler_ycbcr_conversion: bool) -> &mut Self {
        self.sampler_ycbcr_conversion = sampler_ycbcr_conversion;
        self
    }
    ///Sets the `sampler_ycbcr_conversion` field in a builder way.
    pub fn with_sampler_ycbcr_conversion(mut self, sampler_ycbcr_conversion: bool) -> Self {
        self.sampler_ycbcr_conversion = sampler_ycbcr_conversion;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceSamplerYcbcrConversionFeatures {
    type LowLevel = crate::native::vulkan1_1::PhysicalDeviceSamplerYcbcrConversionFeatures;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_1::PhysicalDeviceSamplerYcbcrConversionFeatures {
            s_type: StructureType::PhysicalDeviceSamplerYcbcrConversionFeatures,
            p_next: std::ptr::null_mut(),
            sampler_ycbcr_conversion: self.sampler_ycbcr_conversion.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceSamplerYcbcrConversionFeatures {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            sampler_ycbcr_conversion: crate::conv::FromLowLevel::from_low_level(
                context,
                value.sampler_ycbcr_conversion,
            ),
        }
    }
}
#[doc(alias = "VkSamplerYcbcrConversionImageFormatProperties")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SamplerYcbcrConversionImageFormatProperties {
    #[doc(alias = "combinedImageSamplerDescriptorCount")]
    pub combined_image_sampler_descriptor_count: u32,
}
impl SamplerYcbcrConversionImageFormatProperties {
    ///Get a reference to the `combined_image_sampler_descriptor_count` field.
    pub fn combined_image_sampler_descriptor_count(&self) -> u32 {
        self.combined_image_sampler_descriptor_count
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for SamplerYcbcrConversionImageFormatProperties {
    type LowLevel = crate::native::vulkan1_1::SamplerYcbcrConversionImageFormatProperties;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_1::SamplerYcbcrConversionImageFormatProperties {
            s_type: StructureType::SamplerYcbcrConversionImageFormatProperties,
            p_next: std::ptr::null_mut(),
            combined_image_sampler_descriptor_count: self
                .combined_image_sampler_descriptor_count
                .into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for SamplerYcbcrConversionImageFormatProperties {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            combined_image_sampler_descriptor_count: crate::conv::FromLowLevel::from_low_level(
                context,
                value.combined_image_sampler_descriptor_count,
            ),
        }
    }
}
#[doc(alias = "VkProtectedSubmitInfo")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ProtectedSubmitInfo {
    #[doc(alias = "protectedSubmit")]
    pub protected_submit: bool,
}
impl ProtectedSubmitInfo {
    ///Get a reference to the `protected_submit` field.
    pub fn protected_submit(&self) -> &bool {
        &self.protected_submit
    }
    ///Get a mutable reference to the `protected_submit` field.
    pub fn protected_submit_mut(&mut self) -> &mut bool {
        &mut self.protected_submit
    }
    ///Sets the `protected_submit` field.
    pub fn set_protected_submit(&mut self, protected_submit: bool) -> &mut Self {
        self.protected_submit = protected_submit;
        self
    }
    ///Sets the `protected_submit` field in a builder way.
    pub fn with_protected_submit(mut self, protected_submit: bool) -> Self {
        self.protected_submit = protected_submit;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for ProtectedSubmitInfo {
    type LowLevel = crate::native::vulkan1_1::ProtectedSubmitInfo;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_1::ProtectedSubmitInfo {
            s_type: StructureType::ProtectedSubmitInfo,
            p_next: std::ptr::null(),
            protected_submit: self.protected_submit.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for ProtectedSubmitInfo {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            protected_submit: crate::conv::FromLowLevel::from_low_level(context, value.protected_submit),
        }
    }
}
#[doc(alias = "VkPhysicalDeviceProtectedMemoryFeatures")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceProtectedMemoryFeatures {
    #[doc(alias = "protectedMemory")]
    pub protected_memory: bool,
}
impl PhysicalDeviceProtectedMemoryFeatures {
    ///Get a reference to the `protected_memory` field.
    pub fn protected_memory(&self) -> &bool {
        &self.protected_memory
    }
    ///Get a mutable reference to the `protected_memory` field.
    pub fn protected_memory_mut(&mut self) -> &mut bool {
        &mut self.protected_memory
    }
    ///Sets the `protected_memory` field.
    pub fn set_protected_memory(&mut self, protected_memory: bool) -> &mut Self {
        self.protected_memory = protected_memory;
        self
    }
    ///Sets the `protected_memory` field in a builder way.
    pub fn with_protected_memory(mut self, protected_memory: bool) -> Self {
        self.protected_memory = protected_memory;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceProtectedMemoryFeatures {
    type LowLevel = crate::native::vulkan1_1::PhysicalDeviceProtectedMemoryFeatures;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_1::PhysicalDeviceProtectedMemoryFeatures {
            s_type: StructureType::PhysicalDeviceProtectedMemoryFeatures,
            p_next: std::ptr::null_mut(),
            protected_memory: self.protected_memory.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceProtectedMemoryFeatures {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            protected_memory: crate::conv::FromLowLevel::from_low_level(context, value.protected_memory),
        }
    }
}
#[doc(alias = "VkPhysicalDeviceProtectedMemoryProperties")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceProtectedMemoryProperties {
    #[doc(alias = "protectedNoFault")]
    pub protected_no_fault: bool,
}
impl PhysicalDeviceProtectedMemoryProperties {
    ///Get a reference to the `protected_no_fault` field.
    pub fn protected_no_fault(&self) -> &bool {
        &self.protected_no_fault
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceProtectedMemoryProperties {
    type LowLevel = crate::native::vulkan1_1::PhysicalDeviceProtectedMemoryProperties;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_1::PhysicalDeviceProtectedMemoryProperties {
            s_type: StructureType::PhysicalDeviceProtectedMemoryProperties,
            p_next: std::ptr::null_mut(),
            protected_no_fault: self.protected_no_fault.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceProtectedMemoryProperties {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            protected_no_fault: crate::conv::FromLowLevel::from_low_level(context, value.protected_no_fault),
        }
    }
}
#[doc(alias = "VkDeviceQueueInfo2")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DeviceQueueInfo2 {
    pub flags: DeviceQueueCreateFlags,
    #[doc(alias = "queueFamilyIndex")]
    pub queue_family_index: u32,
    #[doc(alias = "queueIndex")]
    pub queue_index: u32,
}
impl DeviceQueueInfo2 {
    ///Get a reference to the `flags` field.
    pub fn flags(&self) -> DeviceQueueCreateFlags {
        self.flags
    }
    ///Get a reference to the `queue_family_index` field.
    pub fn queue_family_index(&self) -> u32 {
        self.queue_family_index
    }
    ///Get a reference to the `queue_index` field.
    pub fn queue_index(&self) -> u32 {
        self.queue_index
    }
    ///Get a mutable reference to the `flags` field.
    pub fn flags_mut(&mut self) -> &mut DeviceQueueCreateFlags {
        &mut self.flags
    }
    ///Get a mutable reference to the `queue_family_index` field.
    pub fn queue_family_index_mut(&mut self) -> &mut u32 {
        &mut self.queue_family_index
    }
    ///Get a mutable reference to the `queue_index` field.
    pub fn queue_index_mut(&mut self) -> &mut u32 {
        &mut self.queue_index
    }
    ///Sets the `flags` field.
    pub fn set_flags(&mut self, flags: DeviceQueueCreateFlags) -> &mut Self {
        self.flags = flags;
        self
    }
    ///Sets the `queue_family_index` field.
    pub fn set_queue_family_index(&mut self, queue_family_index: u32) -> &mut Self {
        self.queue_family_index = queue_family_index;
        self
    }
    ///Sets the `queue_index` field.
    pub fn set_queue_index(&mut self, queue_index: u32) -> &mut Self {
        self.queue_index = queue_index;
        self
    }
    ///Sets the `flags` field in a builder way.
    pub fn with_flags(mut self, flags: DeviceQueueCreateFlags) -> Self {
        self.flags = flags;
        self
    }
    ///Sets the `queue_family_index` field in a builder way.
    pub fn with_queue_family_index(mut self, queue_family_index: u32) -> Self {
        self.queue_family_index = queue_family_index;
        self
    }
    ///Sets the `queue_index` field in a builder way.
    pub fn with_queue_index(mut self, queue_index: u32) -> Self {
        self.queue_index = queue_index;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for DeviceQueueInfo2 {
    type LowLevel = crate::native::vulkan1_1::DeviceQueueInfo2;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_1::DeviceQueueInfo2 {
            s_type: StructureType::DeviceQueueInfo2,
            p_next: std::ptr::null(),
            flags: self.flags.into_low_level(context, bump),
            queue_family_index: self.queue_family_index.into_low_level(context, bump),
            queue_index: self.queue_index.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for DeviceQueueInfo2 {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            flags: crate::conv::FromLowLevel::from_low_level(context, value.flags),
            queue_family_index: crate::conv::FromLowLevel::from_low_level(context, value.queue_family_index),
            queue_index: crate::conv::FromLowLevel::from_low_level(context, value.queue_index),
        }
    }
}
#[doc(alias = "VkPhysicalDeviceMaintenance3Properties")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceMaintenance3Properties {
    #[doc(alias = "maxPerSetDescriptors")]
    pub max_per_set_descriptors: u32,
    #[doc(alias = "maxMemoryAllocationSize")]
    pub max_memory_allocation_size: DeviceSize,
}
impl PhysicalDeviceMaintenance3Properties {
    ///Get a reference to the `max_per_set_descriptors` field.
    pub fn max_per_set_descriptors(&self) -> u32 {
        self.max_per_set_descriptors
    }
    ///Get a reference to the `max_memory_allocation_size` field.
    pub fn max_memory_allocation_size(&self) -> &DeviceSize {
        &self.max_memory_allocation_size
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceMaintenance3Properties {
    type LowLevel = crate::native::vulkan1_1::PhysicalDeviceMaintenance3Properties;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_1::PhysicalDeviceMaintenance3Properties {
            s_type: StructureType::PhysicalDeviceMaintenance3Properties,
            p_next: std::ptr::null_mut(),
            max_per_set_descriptors: self.max_per_set_descriptors.into_low_level(context, bump),
            max_memory_allocation_size: self.max_memory_allocation_size.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceMaintenance3Properties {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            max_per_set_descriptors: crate::conv::FromLowLevel::from_low_level(context, value.max_per_set_descriptors),
            max_memory_allocation_size: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_memory_allocation_size,
            ),
        }
    }
}
#[doc(alias = "VkDescriptorSetLayoutSupport")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DescriptorSetLayoutSupport {
    #[doc(alias = "pNext")]
    pub extensions: SmallVec<[DescriptorSetLayoutSupportExtension; 1]>,
    pub supported: bool,
}
impl DescriptorSetLayoutSupport {
    ///Adds an extension to the struct
    pub fn with_extension(mut self, ext: impl Into<DescriptorSetLayoutSupportExtension>) -> Self {
        self.extensions.push(ext.into());
        self
    }
    ///Get a reference to the `extensions` field.
    pub fn extensions(&self) -> &SmallVec<[DescriptorSetLayoutSupportExtension; 1]> {
        &self.extensions
    }
    ///Get a reference to the `supported` field.
    pub fn supported(&self) -> &bool {
        &self.supported
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for DescriptorSetLayoutSupport {
    type LowLevel = crate::native::vulkan1_1::DescriptorSetLayoutSupport;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let mut next = std::ptr::null_mut();
        let mut extensions = self.extensions.iter();
        while let Some(ext) = extensions.next() {
            let ext = ext.into_low_level(context, bump);
            (*ext).next = next;
            next = ext;
        }
        crate::native::vulkan1_1::DescriptorSetLayoutSupport {
            s_type: StructureType::DescriptorSetLayoutSupport,
            p_next: next,
            supported: self.supported.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for DescriptorSetLayoutSupport {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let mut next = value.p_next;
        let mut extensions = SmallVec::new();
        while !next.is_null() {
            extensions.push(crate::conv::FromLowLevel::from_low_level(context, next));
            next = std::ptr::read(next).next;
        }
        Self {
            extensions: extensions,
            supported: crate::conv::FromLowLevel::from_low_level(context, value.supported),
        }
    }
}
#[derive(Clone, PartialEq, Debug, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
///Extensions for [`DescriptorSetLayoutSupport`]
pub enum DescriptorSetLayoutSupportExtension {
    #[cfg(feature = "VULKAN_1_2")]
    ///Contains a type [`DescriptorSetVariableDescriptorCountLayoutSupport`] for extending
    /// [`DescriptorSetLayoutSupport`]
    DescriptorSetVariableDescriptorCountLayoutSupport(DescriptorSetVariableDescriptorCountLayoutSupport),
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for DescriptorSetLayoutSupportExtension {
    type LowLevel = *mut crate::native::vulkan1_0::BaseOutStructure;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        match self {
            #[cfg(feature = "VULKAN_1_2")]
            Self::DescriptorSetVariableDescriptorCountLayoutSupport(ext) => (bump
                .alloc(ext.into_low_level(context, bump))
                as *mut crate::native::vulkan1_2::DescriptorSetVariableDescriptorCountLayoutSupport)
                .cast(),
            other => unreachable!("unexpected variant {:?}", other),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for DescriptorSetLayoutSupportExtension {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        assert!(!value.is_null());
        match (*value).s_type {
            #[cfg(feature = "VULKAN_1_2")]
            crate::native::vulkan1_0::StructureType::DescriptorSetVariableDescriptorCountLayoutSupport => {
                Self::DescriptorSetVariableDescriptorCountLayoutSupport(
                    DescriptorSetVariableDescriptorCountLayoutSupport::from_low_level(
                        context,
                        std::ptr::read(
                            value.cast::<crate::native::vulkan1_2::DescriptorSetVariableDescriptorCountLayoutSupport>(),
                        ),
                    ),
                )
            },
            other => panic!(
                "Structure type {:?} is not a member of {}",
                other,
                stringify!(DescriptorSetLayoutSupport)
            ),
        }
    }
}
#[cfg(feature = "VULKAN_1_2")]
impl From<DescriptorSetVariableDescriptorCountLayoutSupport> for DescriptorSetLayoutSupportExtension {
    fn from(ext: DescriptorSetVariableDescriptorCountLayoutSupport) -> Self {
        Self::DescriptorSetVariableDescriptorCountLayoutSupport(ext)
    }
}
#[cfg(feature = "VULKAN_1_2")]
impl TryInto<DescriptorSetVariableDescriptorCountLayoutSupport> for DescriptorSetLayoutSupportExtension {
    type Error = DescriptorSetLayoutSupportExtension;
    fn try_into(self) -> Result<DescriptorSetVariableDescriptorCountLayoutSupport, Self::Error> {
        match self {
            Self::DescriptorSetVariableDescriptorCountLayoutSupport(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[doc(alias = "VkPhysicalDeviceShaderDrawParametersFeatures")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceShaderDrawParametersFeatures {
    #[doc(alias = "shaderDrawParameters")]
    pub shader_draw_parameters: bool,
}
impl PhysicalDeviceShaderDrawParametersFeatures {
    ///Get a reference to the `shader_draw_parameters` field.
    pub fn shader_draw_parameters(&self) -> &bool {
        &self.shader_draw_parameters
    }
    ///Get a mutable reference to the `shader_draw_parameters` field.
    pub fn shader_draw_parameters_mut(&mut self) -> &mut bool {
        &mut self.shader_draw_parameters
    }
    ///Sets the `shader_draw_parameters` field.
    pub fn set_shader_draw_parameters(&mut self, shader_draw_parameters: bool) -> &mut Self {
        self.shader_draw_parameters = shader_draw_parameters;
        self
    }
    ///Sets the `shader_draw_parameters` field in a builder way.
    pub fn with_shader_draw_parameters(mut self, shader_draw_parameters: bool) -> Self {
        self.shader_draw_parameters = shader_draw_parameters;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceShaderDrawParametersFeatures {
    type LowLevel = crate::native::vulkan1_1::PhysicalDeviceShaderDrawParametersFeatures;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::vulkan1_1::PhysicalDeviceShaderDrawParametersFeatures {
            s_type: StructureType::PhysicalDeviceShaderDrawParametersFeatures,
            p_next: std::ptr::null_mut(),
            shader_draw_parameters: self.shader_draw_parameters.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceShaderDrawParametersFeatures {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            shader_draw_parameters: crate::conv::FromLowLevel::from_low_level(context, value.shader_draw_parameters),
        }
    }
}
#[doc(alias = "VkDescriptorUpdateTemplate")]
#[derive(Debug)]
pub struct DescriptorUpdateTemplate {
    context: Arc<Context>,
    id: ObjectId,
}
impl Clone for DescriptorUpdateTemplate {
    fn clone(&self) -> Self {
        self.context.clone_descriptor_update_template(self.id);
        Self {
            context: Arc::clone(&self.context),
            id: self.id,
        }
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for DescriptorUpdateTemplate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.id.serialize(serializer)
    }
}
impl<'de> Deserialize<'de> for DescriptorUpdateTemplate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let id = ObjectId::deserialize(deserializer)?;
        crate::context::CONTEXT.with(|context| {
            let borrow = context.borrow();
            let context = borrow.as_ref().expect("Context not set.");
            Ok(Self {
                context: Arc::clone(context),
                id,
            })
        })
    }
}
impl Drop for DescriptorUpdateTemplate {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            self.context.drop_descriptor_update_template(&self.id);
        }
    }
}
impl PartialEq for DescriptorUpdateTemplate {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl DescriptorUpdateTemplate {
    ///Creates a new instance of this handle from its core components.
    pub(crate) const unsafe fn new(context: Arc<Context>, id: ObjectId) -> Self {
        Self { context, id }
    }
    ///Gets the object id
    pub fn id(&self) -> &ObjectId {
        &self.id
    }
    ///Gets a reference to the context
    pub fn context(&self) -> &Context {
        &self.context
    }
    ///Gets a reference to the context wrapped in an [`Arc`]
    pub fn arc_context(&self) -> &Arc<Context> {
        &self.context
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for DescriptorUpdateTemplate {
    type LowLevel = crate::native::vulkan1_1::DescriptorUpdateTemplate;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        *context
            .descriptor_update_template()
            .get(&self.id)
            .expect("unknwon handle")
            .handle()
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for DescriptorUpdateTemplate {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let object_id = ObjectId::random();
        context
            .descriptor_update_template()
            .insert(object_id, Container::new(value));
        Self {
            context: context.clone(),
            id: object_id,
        }
    }
}
#[doc(alias = "VkSamplerYcbcrConversion")]
#[derive(Debug)]
pub struct SamplerYcbcrConversion {
    context: Arc<Context>,
    id: ObjectId,
}
impl Clone for SamplerYcbcrConversion {
    fn clone(&self) -> Self {
        self.context.clone_sampler_ycbcr_conversion(self.id);
        Self {
            context: Arc::clone(&self.context),
            id: self.id,
        }
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for SamplerYcbcrConversion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.id.serialize(serializer)
    }
}
impl<'de> Deserialize<'de> for SamplerYcbcrConversion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let id = ObjectId::deserialize(deserializer)?;
        crate::context::CONTEXT.with(|context| {
            let borrow = context.borrow();
            let context = borrow.as_ref().expect("Context not set.");
            Ok(Self {
                context: Arc::clone(context),
                id,
            })
        })
    }
}
impl Drop for SamplerYcbcrConversion {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            self.context.drop_sampler_ycbcr_conversion(&self.id);
        }
    }
}
impl PartialEq for SamplerYcbcrConversion {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl SamplerYcbcrConversion {
    ///Creates a new instance of this handle from its core components.
    pub(crate) const unsafe fn new(context: Arc<Context>, id: ObjectId) -> Self {
        Self { context, id }
    }
    ///Gets the object id
    pub fn id(&self) -> &ObjectId {
        &self.id
    }
    ///Gets a reference to the context
    pub fn context(&self) -> &Context {
        &self.context
    }
    ///Gets a reference to the context wrapped in an [`Arc`]
    pub fn arc_context(&self) -> &Arc<Context> {
        &self.context
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for SamplerYcbcrConversion {
    type LowLevel = crate::native::vulkan1_1::SamplerYcbcrConversion;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        *context
            .sampler_ycbcr_conversion()
            .get(&self.id)
            .expect("unknwon handle")
            .handle()
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for SamplerYcbcrConversion {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let object_id = ObjectId::random();
        context
            .sampler_ycbcr_conversion()
            .insert(object_id, Container::new(value));
        Self {
            context: context.clone(),
            id: object_id,
        }
    }
}
