# ! [doc = include_str ! ("../../../doc/VK_VERSION_1_2.md")]
use crate::{
    vulkan1_0::{
        AccessFlags, AllocationCallbacks, AttachmentDescriptionFlags, AttachmentLoadOp, AttachmentStoreOp,
        BaseInStructure, BaseOutStructure, Bool32, Buffer, CommandBuffer, DependencyFlags, Device, DeviceAddress,
        DeviceMemory, DeviceSize, Format, ImageAspectFlags, ImageCreateFlags, ImageLayout, ImageUsageFlags, ImageView,
        PipelineBindPoint, PipelineStageFlags, QueryPool, RenderPass, RenderPassBeginInfo, RenderPassCreateFlags,
        SampleCountFlagBits, SampleCountFlags, Semaphore, ShaderStageFlags, StructureType, SubpassContents,
        SubpassDescriptionFlags, VulkanResultCodes, LUID_SIZE, MAX_DRIVER_INFO_SIZE, MAX_DRIVER_NAME_SIZE, UUID_SIZE,
    },
    vulkan1_1::{PointClippingBehavior, SubgroupFeatureFlags},
};
# [doc = include_str ! ("../../../doc/VkConformanceVersion.md")]
#[doc(alias = "VkConformanceVersion")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ConformanceVersion {
    # [doc = include_str ! ("../../../doc/VkConformanceVersion$major.md")]
    major: u8,
    # [doc = include_str ! ("../../../doc/VkConformanceVersion$minor.md")]
    minor: u8,
    # [doc = include_str ! ("../../../doc/VkConformanceVersion$subminor.md")]
    subminor: u8,
    # [doc = include_str ! ("../../../doc/VkConformanceVersion$patch.md")]
    patch: u8,
}
# [doc = include_str ! ("../../../doc/VkPhysicalDeviceDriverProperties.md")]
#[doc(alias = "VkPhysicalDeviceDriverProperties")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceDriverProperties {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "driverID")]
    driver_id: DriverId,
    #[doc(alias = "driverName")]
    driver_name: [std::ffi::c_char; MAX_DRIVER_NAME_SIZE as usize],
    #[doc(alias = "driverInfo")]
    driver_info: [std::ffi::c_char; MAX_DRIVER_INFO_SIZE as usize],
    #[doc(alias = "conformanceVersion")]
    conformance_version: ConformanceVersion,
}
# [doc = include_str ! ("../../../doc/VkPhysicalDeviceShaderSubgroupExtendedTypesFeatures.md")]
#[doc(alias = "VkPhysicalDeviceShaderSubgroupExtendedTypesFeatures")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShaderSubgroupExtendedTypesFeatures {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "shaderSubgroupExtendedTypes")]
    shader_subgroup_extended_types: Bool32,
}
# [doc = include_str ! ("../../../doc/VkPhysicalDeviceSamplerFilterMinmaxProperties.md")]
#[doc(alias = "VkPhysicalDeviceSamplerFilterMinmaxProperties")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceSamplerFilterMinmaxProperties {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "filterMinmaxSingleComponentFormats")]
    filter_minmax_single_component_formats: Bool32,
    #[doc(alias = "filterMinmaxImageComponentMapping")]
    filter_minmax_image_component_mapping: Bool32,
}
# [doc = include_str ! ("../../../doc/VkSamplerReductionModeCreateInfo.md")]
#[doc(alias = "VkSamplerReductionModeCreateInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SamplerReductionModeCreateInfo {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "reductionMode")]
    reduction_mode: SamplerReductionMode,
}
# [doc = include_str ! ("../../../doc/VkImageFormatListCreateInfo.md")]
#[doc(alias = "VkImageFormatListCreateInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ImageFormatListCreateInfo {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "viewFormatCount")]
    view_format_count: u32,
    #[doc(alias = "pViewFormats")]
    view_formats: *const Format,
}
# [doc = include_str ! ("../../../doc/VkPhysicalDeviceShaderFloat16Int8Features.md")]
#[doc(alias = "VkPhysicalDeviceShaderFloat16Int8Features")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShaderFloat16Int8Features {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "shaderFloat16")]
    shader_float16: Bool32,
    #[doc(alias = "shaderInt8")]
    shader_int8: Bool32,
}
# [doc = include_str ! ("../../../doc/VkPhysicalDeviceFloatControlsProperties.md")]
#[doc(alias = "VkPhysicalDeviceFloatControlsProperties")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceFloatControlsProperties {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "denormBehaviorIndependence")]
    denorm_behavior_independence: ShaderFloatControlsIndependence,
    #[doc(alias = "roundingModeIndependence")]
    rounding_mode_independence: ShaderFloatControlsIndependence,
    #[doc(alias = "shaderSignedZeroInfNanPreserveFloat16")]
    shader_signed_zero_inf_nan_preserve_float16: Bool32,
    #[doc(alias = "shaderSignedZeroInfNanPreserveFloat32")]
    shader_signed_zero_inf_nan_preserve_float32: Bool32,
    #[doc(alias = "shaderSignedZeroInfNanPreserveFloat64")]
    shader_signed_zero_inf_nan_preserve_float64: Bool32,
    #[doc(alias = "shaderDenormPreserveFloat16")]
    shader_denorm_preserve_float16: Bool32,
    #[doc(alias = "shaderDenormPreserveFloat32")]
    shader_denorm_preserve_float32: Bool32,
    #[doc(alias = "shaderDenormPreserveFloat64")]
    shader_denorm_preserve_float64: Bool32,
    #[doc(alias = "shaderDenormFlushToZeroFloat16")]
    shader_denorm_flush_to_zero_float16: Bool32,
    #[doc(alias = "shaderDenormFlushToZeroFloat32")]
    shader_denorm_flush_to_zero_float32: Bool32,
    #[doc(alias = "shaderDenormFlushToZeroFloat64")]
    shader_denorm_flush_to_zero_float64: Bool32,
    #[doc(alias = "shaderRoundingModeRTEFloat16")]
    shader_rounding_mode_rte_float16: Bool32,
    #[doc(alias = "shaderRoundingModeRTEFloat32")]
    shader_rounding_mode_rte_float32: Bool32,
    #[doc(alias = "shaderRoundingModeRTEFloat64")]
    shader_rounding_mode_rte_float64: Bool32,
    #[doc(alias = "shaderRoundingModeRTZFloat16")]
    shader_rounding_mode_rtz_float16: Bool32,
    #[doc(alias = "shaderRoundingModeRTZFloat32")]
    shader_rounding_mode_rtz_float32: Bool32,
    #[doc(alias = "shaderRoundingModeRTZFloat64")]
    shader_rounding_mode_rtz_float64: Bool32,
}
# [doc = include_str ! ("../../../doc/VkPhysicalDeviceHostQueryResetFeatures.md")]
#[doc(alias = "VkPhysicalDeviceHostQueryResetFeatures")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceHostQueryResetFeatures {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "hostQueryReset")]
    host_query_reset: Bool32,
}
# [doc = include_str ! ("../../../doc/VkPhysicalDeviceDescriptorIndexingFeatures.md")]
#[doc(alias = "VkPhysicalDeviceDescriptorIndexingFeatures")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceDescriptorIndexingFeatures {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "shaderInputAttachmentArrayDynamicIndexing")]
    shader_input_attachment_array_dynamic_indexing: Bool32,
    #[doc(alias = "shaderUniformTexelBufferArrayDynamicIndexing")]
    shader_uniform_texel_buffer_array_dynamic_indexing: Bool32,
    #[doc(alias = "shaderStorageTexelBufferArrayDynamicIndexing")]
    shader_storage_texel_buffer_array_dynamic_indexing: Bool32,
    #[doc(alias = "shaderUniformBufferArrayNonUniformIndexing")]
    shader_uniform_buffer_array_non_uniform_indexing: Bool32,
    #[doc(alias = "shaderSampledImageArrayNonUniformIndexing")]
    shader_sampled_image_array_non_uniform_indexing: Bool32,
    #[doc(alias = "shaderStorageBufferArrayNonUniformIndexing")]
    shader_storage_buffer_array_non_uniform_indexing: Bool32,
    #[doc(alias = "shaderStorageImageArrayNonUniformIndexing")]
    shader_storage_image_array_non_uniform_indexing: Bool32,
    #[doc(alias = "shaderInputAttachmentArrayNonUniformIndexing")]
    shader_input_attachment_array_non_uniform_indexing: Bool32,
    #[doc(alias = "shaderUniformTexelBufferArrayNonUniformIndexing")]
    shader_uniform_texel_buffer_array_non_uniform_indexing: Bool32,
    #[doc(alias = "shaderStorageTexelBufferArrayNonUniformIndexing")]
    shader_storage_texel_buffer_array_non_uniform_indexing: Bool32,
    #[doc(alias = "descriptorBindingUniformBufferUpdateAfterBind")]
    descriptor_binding_uniform_buffer_update_after_bind: Bool32,
    #[doc(alias = "descriptorBindingSampledImageUpdateAfterBind")]
    descriptor_binding_sampled_image_update_after_bind: Bool32,
    #[doc(alias = "descriptorBindingStorageImageUpdateAfterBind")]
    descriptor_binding_storage_image_update_after_bind: Bool32,
    #[doc(alias = "descriptorBindingStorageBufferUpdateAfterBind")]
    descriptor_binding_storage_buffer_update_after_bind: Bool32,
    #[doc(alias = "descriptorBindingUniformTexelBufferUpdateAfterBind")]
    descriptor_binding_uniform_texel_buffer_update_after_bind: Bool32,
    #[doc(alias = "descriptorBindingStorageTexelBufferUpdateAfterBind")]
    descriptor_binding_storage_texel_buffer_update_after_bind: Bool32,
    #[doc(alias = "descriptorBindingUpdateUnusedWhilePending")]
    descriptor_binding_update_unused_while_pending: Bool32,
    #[doc(alias = "descriptorBindingPartiallyBound")]
    descriptor_binding_partially_bound: Bool32,
    #[doc(alias = "descriptorBindingVariableDescriptorCount")]
    descriptor_binding_variable_descriptor_count: Bool32,
    #[doc(alias = "runtimeDescriptorArray")]
    runtime_descriptor_array: Bool32,
}
# [doc = include_str ! ("../../../doc/VkPhysicalDeviceDescriptorIndexingProperties.md")]
#[doc(alias = "VkPhysicalDeviceDescriptorIndexingProperties")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceDescriptorIndexingProperties {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "maxUpdateAfterBindDescriptorsInAllPools")]
    max_update_after_bind_descriptors_in_all_pools: u32,
    #[doc(alias = "shaderUniformBufferArrayNonUniformIndexingNative")]
    shader_uniform_buffer_array_non_uniform_indexing_native: Bool32,
    #[doc(alias = "shaderSampledImageArrayNonUniformIndexingNative")]
    shader_sampled_image_array_non_uniform_indexing_native: Bool32,
    #[doc(alias = "shaderStorageBufferArrayNonUniformIndexingNative")]
    shader_storage_buffer_array_non_uniform_indexing_native: Bool32,
    #[doc(alias = "shaderStorageImageArrayNonUniformIndexingNative")]
    shader_storage_image_array_non_uniform_indexing_native: Bool32,
    #[doc(alias = "shaderInputAttachmentArrayNonUniformIndexingNative")]
    shader_input_attachment_array_non_uniform_indexing_native: Bool32,
    #[doc(alias = "robustBufferAccessUpdateAfterBind")]
    robust_buffer_access_update_after_bind: Bool32,
    #[doc(alias = "quadDivergentImplicitLod")]
    quad_divergent_implicit_lod: Bool32,
    #[doc(alias = "maxPerStageDescriptorUpdateAfterBindSamplers")]
    max_per_stage_descriptor_update_after_bind_samplers: u32,
    #[doc(alias = "maxPerStageDescriptorUpdateAfterBindUniformBuffers")]
    max_per_stage_descriptor_update_after_bind_uniform_buffers: u32,
    #[doc(alias = "maxPerStageDescriptorUpdateAfterBindStorageBuffers")]
    max_per_stage_descriptor_update_after_bind_storage_buffers: u32,
    #[doc(alias = "maxPerStageDescriptorUpdateAfterBindSampledImages")]
    max_per_stage_descriptor_update_after_bind_sampled_images: u32,
    #[doc(alias = "maxPerStageDescriptorUpdateAfterBindStorageImages")]
    max_per_stage_descriptor_update_after_bind_storage_images: u32,
    #[doc(alias = "maxPerStageDescriptorUpdateAfterBindInputAttachments")]
    max_per_stage_descriptor_update_after_bind_input_attachments: u32,
    #[doc(alias = "maxPerStageUpdateAfterBindResources")]
    max_per_stage_update_after_bind_resources: u32,
    #[doc(alias = "maxDescriptorSetUpdateAfterBindSamplers")]
    max_descriptor_set_update_after_bind_samplers: u32,
    #[doc(alias = "maxDescriptorSetUpdateAfterBindUniformBuffers")]
    max_descriptor_set_update_after_bind_uniform_buffers: u32,
    #[doc(alias = "maxDescriptorSetUpdateAfterBindUniformBuffersDynamic")]
    max_descriptor_set_update_after_bind_uniform_buffers_dynamic: u32,
    #[doc(alias = "maxDescriptorSetUpdateAfterBindStorageBuffers")]
    max_descriptor_set_update_after_bind_storage_buffers: u32,
    #[doc(alias = "maxDescriptorSetUpdateAfterBindStorageBuffersDynamic")]
    max_descriptor_set_update_after_bind_storage_buffers_dynamic: u32,
    #[doc(alias = "maxDescriptorSetUpdateAfterBindSampledImages")]
    max_descriptor_set_update_after_bind_sampled_images: u32,
    #[doc(alias = "maxDescriptorSetUpdateAfterBindStorageImages")]
    max_descriptor_set_update_after_bind_storage_images: u32,
    #[doc(alias = "maxDescriptorSetUpdateAfterBindInputAttachments")]
    max_descriptor_set_update_after_bind_input_attachments: u32,
}
# [doc = include_str ! ("../../../doc/VkDescriptorSetLayoutBindingFlagsCreateInfo.md")]
#[doc(alias = "VkDescriptorSetLayoutBindingFlagsCreateInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DescriptorSetLayoutBindingFlagsCreateInfo {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "bindingCount")]
    binding_count: u32,
    #[doc(alias = "pBindingFlags")]
    binding_flags: *const DescriptorBindingFlags,
}
# [doc = include_str ! ("../../../doc/VkDescriptorSetVariableDescriptorCountAllocateInfo.md")]
#[doc(alias = "VkDescriptorSetVariableDescriptorCountAllocateInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DescriptorSetVariableDescriptorCountAllocateInfo {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "descriptorSetCount")]
    descriptor_set_count: u32,
    #[doc(alias = "pDescriptorCounts")]
    descriptor_counts: *const u32,
}
# [doc = include_str ! ("../../../doc/VkDescriptorSetVariableDescriptorCountLayoutSupport.md")]
#[doc(alias = "VkDescriptorSetVariableDescriptorCountLayoutSupport")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DescriptorSetVariableDescriptorCountLayoutSupport {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "maxVariableDescriptorCount")]
    max_variable_descriptor_count: u32,
}
# [doc = include_str ! ("../../../doc/VkAttachmentDescription2.md")]
#[doc(alias = "VkAttachmentDescription2")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct AttachmentDescription2 {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    # [doc = include_str ! ("../../../doc/VkAttachmentDescription2$flags.md")]
    flags: AttachmentDescriptionFlags,
    # [doc = include_str ! ("../../../doc/VkAttachmentDescription2$format.md")]
    format: Format,
    # [doc = include_str ! ("../../../doc/VkAttachmentDescription2$samples.md")]
    samples: SampleCountFlagBits,
    #[doc(alias = "loadOp")]
    load_op: AttachmentLoadOp,
    #[doc(alias = "storeOp")]
    store_op: AttachmentStoreOp,
    #[doc(alias = "stencilLoadOp")]
    stencil_load_op: AttachmentLoadOp,
    #[doc(alias = "stencilStoreOp")]
    stencil_store_op: AttachmentStoreOp,
    #[doc(alias = "initialLayout")]
    initial_layout: ImageLayout,
    #[doc(alias = "finalLayout")]
    final_layout: ImageLayout,
}
# [doc = include_str ! ("../../../doc/VkAttachmentReference2.md")]
#[doc(alias = "VkAttachmentReference2")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct AttachmentReference2 {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    # [doc = include_str ! ("../../../doc/VkAttachmentReference2$attachment.md")]
    attachment: u32,
    # [doc = include_str ! ("../../../doc/VkAttachmentReference2$layout.md")]
    layout: ImageLayout,
    #[doc(alias = "aspectMask")]
    aspect_mask: ImageAspectFlags,
}
# [doc = include_str ! ("../../../doc/VkSubpassDescription2.md")]
#[doc(alias = "VkSubpassDescription2")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SubpassDescription2 {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    # [doc = include_str ! ("../../../doc/VkSubpassDescription2$flags.md")]
    flags: SubpassDescriptionFlags,
    #[doc(alias = "pipelineBindPoint")]
    pipeline_bind_point: PipelineBindPoint,
    #[doc(alias = "viewMask")]
    view_mask: u32,
    #[doc(alias = "inputAttachmentCount")]
    input_attachment_count: u32,
    #[doc(alias = "pInputAttachments")]
    input_attachments: *const AttachmentReference2,
    #[doc(alias = "colorAttachmentCount")]
    color_attachment_count: u32,
    #[doc(alias = "pColorAttachments")]
    color_attachments: *const AttachmentReference2,
    #[doc(alias = "pResolveAttachments")]
    resolve_attachments: *const AttachmentReference2,
    #[doc(alias = "pDepthStencilAttachment")]
    depth_stencil_attachment: *const AttachmentReference2,
    #[doc(alias = "preserveAttachmentCount")]
    preserve_attachment_count: u32,
    #[doc(alias = "pPreserveAttachments")]
    preserve_attachments: *const u32,
}
# [doc = include_str ! ("../../../doc/VkSubpassDependency2.md")]
#[doc(alias = "VkSubpassDependency2")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SubpassDependency2 {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "srcSubpass")]
    src_subpass: u32,
    #[doc(alias = "dstSubpass")]
    dst_subpass: u32,
    #[doc(alias = "srcStageMask")]
    src_stage_mask: PipelineStageFlags,
    #[doc(alias = "dstStageMask")]
    dst_stage_mask: PipelineStageFlags,
    #[doc(alias = "srcAccessMask")]
    src_access_mask: AccessFlags,
    #[doc(alias = "dstAccessMask")]
    dst_access_mask: AccessFlags,
    #[doc(alias = "dependencyFlags")]
    dependency_flags: DependencyFlags,
    #[doc(alias = "viewOffset")]
    view_offset: i32,
}
# [doc = include_str ! ("../../../doc/VkRenderPassCreateInfo2.md")]
#[doc(alias = "VkRenderPassCreateInfo2")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct RenderPassCreateInfo2 {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    # [doc = include_str ! ("../../../doc/VkRenderPassCreateInfo2$flags.md")]
    flags: RenderPassCreateFlags,
    #[doc(alias = "attachmentCount")]
    attachment_count: u32,
    #[doc(alias = "pAttachments")]
    attachments: *const AttachmentDescription2,
    #[doc(alias = "subpassCount")]
    subpass_count: u32,
    #[doc(alias = "pSubpasses")]
    subpasses: *const SubpassDescription2,
    #[doc(alias = "dependencyCount")]
    dependency_count: u32,
    #[doc(alias = "pDependencies")]
    dependencies: *const SubpassDependency2,
    #[doc(alias = "correlatedViewMaskCount")]
    correlated_view_mask_count: u32,
    #[doc(alias = "pCorrelatedViewMasks")]
    correlated_view_masks: *const u32,
}
# [doc = include_str ! ("../../../doc/VkSubpassBeginInfo.md")]
#[doc(alias = "VkSubpassBeginInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SubpassBeginInfo {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    # [doc = include_str ! ("../../../doc/VkSubpassBeginInfo$contents.md")]
    contents: SubpassContents,
}
# [doc = include_str ! ("../../../doc/VkSubpassEndInfo.md")]
#[doc(alias = "VkSubpassEndInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SubpassEndInfo {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
}
# [doc = include_str ! ("../../../doc/VkPhysicalDeviceTimelineSemaphoreFeatures.md")]
#[doc(alias = "VkPhysicalDeviceTimelineSemaphoreFeatures")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceTimelineSemaphoreFeatures {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "timelineSemaphore")]
    timeline_semaphore: Bool32,
}
# [doc = include_str ! ("../../../doc/VkPhysicalDeviceTimelineSemaphoreProperties.md")]
#[doc(alias = "VkPhysicalDeviceTimelineSemaphoreProperties")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceTimelineSemaphoreProperties {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "maxTimelineSemaphoreValueDifference")]
    max_timeline_semaphore_value_difference: u64,
}
# [doc = include_str ! ("../../../doc/VkSemaphoreTypeCreateInfo.md")]
#[doc(alias = "VkSemaphoreTypeCreateInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SemaphoreTypeCreateInfo {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "semaphoreType")]
    semaphore_type: SemaphoreType,
    #[doc(alias = "initialValue")]
    initial_value: u64,
}
# [doc = include_str ! ("../../../doc/VkTimelineSemaphoreSubmitInfo.md")]
#[doc(alias = "VkTimelineSemaphoreSubmitInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct TimelineSemaphoreSubmitInfo {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "waitSemaphoreValueCount")]
    wait_semaphore_value_count: u32,
    #[doc(alias = "pWaitSemaphoreValues")]
    wait_semaphore_values: *const u64,
    #[doc(alias = "signalSemaphoreValueCount")]
    signal_semaphore_value_count: u32,
    #[doc(alias = "pSignalSemaphoreValues")]
    signal_semaphore_values: *const u64,
}
# [doc = include_str ! ("../../../doc/VkSemaphoreWaitInfo.md")]
#[doc(alias = "VkSemaphoreWaitInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SemaphoreWaitInfo {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    # [doc = include_str ! ("../../../doc/VkSemaphoreWaitInfo$flags.md")]
    flags: SemaphoreWaitFlags,
    #[doc(alias = "semaphoreCount")]
    semaphore_count: u32,
    #[doc(alias = "pSemaphores")]
    semaphores: *const Semaphore,
    #[doc(alias = "pValues")]
    values: *const u64,
}
# [doc = include_str ! ("../../../doc/VkSemaphoreSignalInfo.md")]
#[doc(alias = "VkSemaphoreSignalInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SemaphoreSignalInfo {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    # [doc = include_str ! ("../../../doc/VkSemaphoreSignalInfo$semaphore.md")]
    semaphore: Semaphore,
    # [doc = include_str ! ("../../../doc/VkSemaphoreSignalInfo$value.md")]
    value: u64,
}
# [doc = include_str ! ("../../../doc/VkPhysicalDevice8BitStorageFeatures.md")]
#[doc(alias = "VkPhysicalDevice8BitStorageFeatures")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDevice8BitStorageFeatures {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "storageBuffer8BitAccess")]
    storage_buffer8_bit_access: Bool32,
    #[doc(alias = "uniformAndStorageBuffer8BitAccess")]
    uniform_and_storage_buffer8_bit_access: Bool32,
    #[doc(alias = "storagePushConstant8")]
    storage_push_constant8: Bool32,
}
# [doc = include_str ! ("../../../doc/VkPhysicalDeviceVulkanMemoryModelFeatures.md")]
#[doc(alias = "VkPhysicalDeviceVulkanMemoryModelFeatures")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceVulkanMemoryModelFeatures {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "vulkanMemoryModel")]
    vulkan_memory_model: Bool32,
    #[doc(alias = "vulkanMemoryModelDeviceScope")]
    vulkan_memory_model_device_scope: Bool32,
    #[doc(alias = "vulkanMemoryModelAvailabilityVisibilityChains")]
    vulkan_memory_model_availability_visibility_chains: Bool32,
}
# [doc = include_str ! ("../../../doc/VkPhysicalDeviceShaderAtomicInt64Features.md")]
#[doc(alias = "VkPhysicalDeviceShaderAtomicInt64Features")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShaderAtomicInt64Features {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "shaderBufferInt64Atomics")]
    shader_buffer_int64_atomics: Bool32,
    #[doc(alias = "shaderSharedInt64Atomics")]
    shader_shared_int64_atomics: Bool32,
}
# [doc = include_str ! ("../../../doc/VkPhysicalDeviceDepthStencilResolveProperties.md")]
#[doc(alias = "VkPhysicalDeviceDepthStencilResolveProperties")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceDepthStencilResolveProperties {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "supportedDepthResolveModes")]
    supported_depth_resolve_modes: ResolveModeFlags,
    #[doc(alias = "supportedStencilResolveModes")]
    supported_stencil_resolve_modes: ResolveModeFlags,
    #[doc(alias = "independentResolveNone")]
    independent_resolve_none: Bool32,
    #[doc(alias = "independentResolve")]
    independent_resolve: Bool32,
}
# [doc = include_str ! ("../../../doc/VkSubpassDescriptionDepthStencilResolve.md")]
#[doc(alias = "VkSubpassDescriptionDepthStencilResolve")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SubpassDescriptionDepthStencilResolve {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "depthResolveMode")]
    depth_resolve_mode: ResolveModeFlagBits,
    #[doc(alias = "stencilResolveMode")]
    stencil_resolve_mode: ResolveModeFlagBits,
    #[doc(alias = "pDepthStencilResolveAttachment")]
    depth_stencil_resolve_attachment: *const AttachmentReference2,
}
# [doc = include_str ! ("../../../doc/VkImageStencilUsageCreateInfo.md")]
#[doc(alias = "VkImageStencilUsageCreateInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ImageStencilUsageCreateInfo {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "stencilUsage")]
    stencil_usage: ImageUsageFlags,
}
# [doc = include_str ! ("../../../doc/VkPhysicalDeviceScalarBlockLayoutFeatures.md")]
#[doc(alias = "VkPhysicalDeviceScalarBlockLayoutFeatures")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceScalarBlockLayoutFeatures {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "scalarBlockLayout")]
    scalar_block_layout: Bool32,
}
# [doc = include_str ! ("../../../doc/VkPhysicalDeviceUniformBufferStandardLayoutFeatures.md")]
#[doc(alias = "VkPhysicalDeviceUniformBufferStandardLayoutFeatures")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceUniformBufferStandardLayoutFeatures {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "uniformBufferStandardLayout")]
    uniform_buffer_standard_layout: Bool32,
}
# [doc = include_str ! ("../../../doc/VkPhysicalDeviceBufferDeviceAddressFeatures.md")]
#[doc(alias = "VkPhysicalDeviceBufferDeviceAddressFeatures")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceBufferDeviceAddressFeatures {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "bufferDeviceAddress")]
    buffer_device_address: Bool32,
    #[doc(alias = "bufferDeviceAddressCaptureReplay")]
    buffer_device_address_capture_replay: Bool32,
    #[doc(alias = "bufferDeviceAddressMultiDevice")]
    buffer_device_address_multi_device: Bool32,
}
# [doc = include_str ! ("../../../doc/VkBufferDeviceAddressInfo.md")]
#[doc(alias = "VkBufferDeviceAddressInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct BufferDeviceAddressInfo {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    # [doc = include_str ! ("../../../doc/VkBufferDeviceAddressInfo$buffer.md")]
    buffer: Buffer,
}
# [doc = include_str ! ("../../../doc/VkBufferOpaqueCaptureAddressCreateInfo.md")]
#[doc(alias = "VkBufferOpaqueCaptureAddressCreateInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct BufferOpaqueCaptureAddressCreateInfo {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "opaqueCaptureAddress")]
    opaque_capture_address: u64,
}
# [doc = include_str ! ("../../../doc/VkPhysicalDeviceImagelessFramebufferFeatures.md")]
#[doc(alias = "VkPhysicalDeviceImagelessFramebufferFeatures")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceImagelessFramebufferFeatures {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "imagelessFramebuffer")]
    imageless_framebuffer: Bool32,
}
# [doc = include_str ! ("../../../doc/VkFramebufferAttachmentsCreateInfo.md")]
#[doc(alias = "VkFramebufferAttachmentsCreateInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct FramebufferAttachmentsCreateInfo {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "attachmentImageInfoCount")]
    attachment_image_info_count: u32,
    #[doc(alias = "pAttachmentImageInfos")]
    attachment_image_infos: *const FramebufferAttachmentImageInfo,
}
# [doc = include_str ! ("../../../doc/VkFramebufferAttachmentImageInfo.md")]
#[doc(alias = "VkFramebufferAttachmentImageInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct FramebufferAttachmentImageInfo {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    # [doc = include_str ! ("../../../doc/VkFramebufferAttachmentImageInfo$flags.md")]
    flags: ImageCreateFlags,
    # [doc = include_str ! ("../../../doc/VkFramebufferAttachmentImageInfo$usage.md")]
    usage: ImageUsageFlags,
    # [doc = include_str ! ("../../../doc/VkFramebufferAttachmentImageInfo$width.md")]
    width: u32,
    # [doc = include_str ! ("../../../doc/VkFramebufferAttachmentImageInfo$height.md")]
    height: u32,
    #[doc(alias = "layerCount")]
    layer_count: u32,
    #[doc(alias = "viewFormatCount")]
    view_format_count: u32,
    #[doc(alias = "pViewFormats")]
    view_formats: *const Format,
}
# [doc = include_str ! ("../../../doc/VkRenderPassAttachmentBeginInfo.md")]
#[doc(alias = "VkRenderPassAttachmentBeginInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct RenderPassAttachmentBeginInfo {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "attachmentCount")]
    attachment_count: u32,
    #[doc(alias = "pAttachments")]
    attachments: *const ImageView,
}
# [doc = include_str ! ("../../../doc/VkPhysicalDeviceSeparateDepthStencilLayoutsFeatures.md")]
#[doc(alias = "VkPhysicalDeviceSeparateDepthStencilLayoutsFeatures")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceSeparateDepthStencilLayoutsFeatures {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "separateDepthStencilLayouts")]
    separate_depth_stencil_layouts: Bool32,
}
# [doc = include_str ! ("../../../doc/VkAttachmentReferenceStencilLayout.md")]
#[doc(alias = "VkAttachmentReferenceStencilLayout")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct AttachmentReferenceStencilLayout {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "stencilLayout")]
    stencil_layout: ImageLayout,
}
# [doc = include_str ! ("../../../doc/VkAttachmentDescriptionStencilLayout.md")]
#[doc(alias = "VkAttachmentDescriptionStencilLayout")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct AttachmentDescriptionStencilLayout {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "stencilInitialLayout")]
    stencil_initial_layout: ImageLayout,
    #[doc(alias = "stencilFinalLayout")]
    stencil_final_layout: ImageLayout,
}
# [doc = include_str ! ("../../../doc/VkMemoryOpaqueCaptureAddressAllocateInfo.md")]
#[doc(alias = "VkMemoryOpaqueCaptureAddressAllocateInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct MemoryOpaqueCaptureAddressAllocateInfo {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "opaqueCaptureAddress")]
    opaque_capture_address: u64,
}
# [doc = include_str ! ("../../../doc/VkDeviceMemoryOpaqueCaptureAddressInfo.md")]
#[doc(alias = "VkDeviceMemoryOpaqueCaptureAddressInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DeviceMemoryOpaqueCaptureAddressInfo {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    # [doc = include_str ! ("../../../doc/VkDeviceMemoryOpaqueCaptureAddressInfo$memory.md")]
    memory: DeviceMemory,
}
# [doc = include_str ! ("../../../doc/VkPhysicalDeviceVulkan11Features.md")]
#[doc(alias = "VkPhysicalDeviceVulkan11Features")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceVulkan11Features {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "storageBuffer16BitAccess")]
    storage_buffer16_bit_access: Bool32,
    #[doc(alias = "uniformAndStorageBuffer16BitAccess")]
    uniform_and_storage_buffer16_bit_access: Bool32,
    #[doc(alias = "storagePushConstant16")]
    storage_push_constant16: Bool32,
    #[doc(alias = "storageInputOutput16")]
    storage_input_output16: Bool32,
    # [doc = include_str ! ("../../../doc/VkPhysicalDeviceVulkan11Features$multiview.md")]
    multiview: Bool32,
    #[doc(alias = "multiviewGeometryShader")]
    multiview_geometry_shader: Bool32,
    #[doc(alias = "multiviewTessellationShader")]
    multiview_tessellation_shader: Bool32,
    #[doc(alias = "variablePointersStorageBuffer")]
    variable_pointers_storage_buffer: Bool32,
    #[doc(alias = "variablePointers")]
    variable_pointers: Bool32,
    #[doc(alias = "protectedMemory")]
    protected_memory: Bool32,
    #[doc(alias = "samplerYcbcrConversion")]
    sampler_ycbcr_conversion: Bool32,
    #[doc(alias = "shaderDrawParameters")]
    shader_draw_parameters: Bool32,
}
# [doc = include_str ! ("../../../doc/VkPhysicalDeviceVulkan11Properties.md")]
#[doc(alias = "VkPhysicalDeviceVulkan11Properties")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceVulkan11Properties {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "deviceUUID")]
    device_uuid: [u8; UUID_SIZE as usize],
    #[doc(alias = "driverUUID")]
    driver_uuid: [u8; UUID_SIZE as usize],
    #[doc(alias = "deviceLUID")]
    device_luid: [u8; LUID_SIZE as usize],
    #[doc(alias = "deviceNodeMask")]
    device_node_mask: u32,
    #[doc(alias = "deviceLUIDValid")]
    device_luid_valid: Bool32,
    #[doc(alias = "subgroupSize")]
    subgroup_size: u32,
    #[doc(alias = "subgroupSupportedStages")]
    subgroup_supported_stages: ShaderStageFlags,
    #[doc(alias = "subgroupSupportedOperations")]
    subgroup_supported_operations: SubgroupFeatureFlags,
    #[doc(alias = "subgroupQuadOperationsInAllStages")]
    subgroup_quad_operations_in_all_stages: Bool32,
    #[doc(alias = "pointClippingBehavior")]
    point_clipping_behavior: PointClippingBehavior,
    #[doc(alias = "maxMultiviewViewCount")]
    max_multiview_view_count: u32,
    #[doc(alias = "maxMultiviewInstanceIndex")]
    max_multiview_instance_index: u32,
    #[doc(alias = "protectedNoFault")]
    protected_no_fault: Bool32,
    #[doc(alias = "maxPerSetDescriptors")]
    max_per_set_descriptors: u32,
    #[doc(alias = "maxMemoryAllocationSize")]
    max_memory_allocation_size: DeviceSize,
}
# [doc = include_str ! ("../../../doc/VkPhysicalDeviceVulkan12Features.md")]
#[doc(alias = "VkPhysicalDeviceVulkan12Features")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceVulkan12Features {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "samplerMirrorClampToEdge")]
    sampler_mirror_clamp_to_edge: Bool32,
    #[doc(alias = "drawIndirectCount")]
    draw_indirect_count: Bool32,
    #[doc(alias = "storageBuffer8BitAccess")]
    storage_buffer8_bit_access: Bool32,
    #[doc(alias = "uniformAndStorageBuffer8BitAccess")]
    uniform_and_storage_buffer8_bit_access: Bool32,
    #[doc(alias = "storagePushConstant8")]
    storage_push_constant8: Bool32,
    #[doc(alias = "shaderBufferInt64Atomics")]
    shader_buffer_int64_atomics: Bool32,
    #[doc(alias = "shaderSharedInt64Atomics")]
    shader_shared_int64_atomics: Bool32,
    #[doc(alias = "shaderFloat16")]
    shader_float16: Bool32,
    #[doc(alias = "shaderInt8")]
    shader_int8: Bool32,
    #[doc(alias = "descriptorIndexing")]
    descriptor_indexing: Bool32,
    #[doc(alias = "shaderInputAttachmentArrayDynamicIndexing")]
    shader_input_attachment_array_dynamic_indexing: Bool32,
    #[doc(alias = "shaderUniformTexelBufferArrayDynamicIndexing")]
    shader_uniform_texel_buffer_array_dynamic_indexing: Bool32,
    #[doc(alias = "shaderStorageTexelBufferArrayDynamicIndexing")]
    shader_storage_texel_buffer_array_dynamic_indexing: Bool32,
    #[doc(alias = "shaderUniformBufferArrayNonUniformIndexing")]
    shader_uniform_buffer_array_non_uniform_indexing: Bool32,
    #[doc(alias = "shaderSampledImageArrayNonUniformIndexing")]
    shader_sampled_image_array_non_uniform_indexing: Bool32,
    #[doc(alias = "shaderStorageBufferArrayNonUniformIndexing")]
    shader_storage_buffer_array_non_uniform_indexing: Bool32,
    #[doc(alias = "shaderStorageImageArrayNonUniformIndexing")]
    shader_storage_image_array_non_uniform_indexing: Bool32,
    #[doc(alias = "shaderInputAttachmentArrayNonUniformIndexing")]
    shader_input_attachment_array_non_uniform_indexing: Bool32,
    #[doc(alias = "shaderUniformTexelBufferArrayNonUniformIndexing")]
    shader_uniform_texel_buffer_array_non_uniform_indexing: Bool32,
    #[doc(alias = "shaderStorageTexelBufferArrayNonUniformIndexing")]
    shader_storage_texel_buffer_array_non_uniform_indexing: Bool32,
    #[doc(alias = "descriptorBindingUniformBufferUpdateAfterBind")]
    descriptor_binding_uniform_buffer_update_after_bind: Bool32,
    #[doc(alias = "descriptorBindingSampledImageUpdateAfterBind")]
    descriptor_binding_sampled_image_update_after_bind: Bool32,
    #[doc(alias = "descriptorBindingStorageImageUpdateAfterBind")]
    descriptor_binding_storage_image_update_after_bind: Bool32,
    #[doc(alias = "descriptorBindingStorageBufferUpdateAfterBind")]
    descriptor_binding_storage_buffer_update_after_bind: Bool32,
    #[doc(alias = "descriptorBindingUniformTexelBufferUpdateAfterBind")]
    descriptor_binding_uniform_texel_buffer_update_after_bind: Bool32,
    #[doc(alias = "descriptorBindingStorageTexelBufferUpdateAfterBind")]
    descriptor_binding_storage_texel_buffer_update_after_bind: Bool32,
    #[doc(alias = "descriptorBindingUpdateUnusedWhilePending")]
    descriptor_binding_update_unused_while_pending: Bool32,
    #[doc(alias = "descriptorBindingPartiallyBound")]
    descriptor_binding_partially_bound: Bool32,
    #[doc(alias = "descriptorBindingVariableDescriptorCount")]
    descriptor_binding_variable_descriptor_count: Bool32,
    #[doc(alias = "runtimeDescriptorArray")]
    runtime_descriptor_array: Bool32,
    #[doc(alias = "samplerFilterMinmax")]
    sampler_filter_minmax: Bool32,
    #[doc(alias = "scalarBlockLayout")]
    scalar_block_layout: Bool32,
    #[doc(alias = "imagelessFramebuffer")]
    imageless_framebuffer: Bool32,
    #[doc(alias = "uniformBufferStandardLayout")]
    uniform_buffer_standard_layout: Bool32,
    #[doc(alias = "shaderSubgroupExtendedTypes")]
    shader_subgroup_extended_types: Bool32,
    #[doc(alias = "separateDepthStencilLayouts")]
    separate_depth_stencil_layouts: Bool32,
    #[doc(alias = "hostQueryReset")]
    host_query_reset: Bool32,
    #[doc(alias = "timelineSemaphore")]
    timeline_semaphore: Bool32,
    #[doc(alias = "bufferDeviceAddress")]
    buffer_device_address: Bool32,
    #[doc(alias = "bufferDeviceAddressCaptureReplay")]
    buffer_device_address_capture_replay: Bool32,
    #[doc(alias = "bufferDeviceAddressMultiDevice")]
    buffer_device_address_multi_device: Bool32,
    #[doc(alias = "vulkanMemoryModel")]
    vulkan_memory_model: Bool32,
    #[doc(alias = "vulkanMemoryModelDeviceScope")]
    vulkan_memory_model_device_scope: Bool32,
    #[doc(alias = "vulkanMemoryModelAvailabilityVisibilityChains")]
    vulkan_memory_model_availability_visibility_chains: Bool32,
    #[doc(alias = "shaderOutputViewportIndex")]
    shader_output_viewport_index: Bool32,
    #[doc(alias = "shaderOutputLayer")]
    shader_output_layer: Bool32,
    #[doc(alias = "subgroupBroadcastDynamicId")]
    subgroup_broadcast_dynamic_id: Bool32,
}
# [doc = include_str ! ("../../../doc/VkPhysicalDeviceVulkan12Properties.md")]
#[doc(alias = "VkPhysicalDeviceVulkan12Properties")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceVulkan12Properties {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "driverID")]
    driver_id: DriverId,
    #[doc(alias = "driverName")]
    driver_name: [std::ffi::c_char; MAX_DRIVER_NAME_SIZE as usize],
    #[doc(alias = "driverInfo")]
    driver_info: [std::ffi::c_char; MAX_DRIVER_INFO_SIZE as usize],
    #[doc(alias = "conformanceVersion")]
    conformance_version: ConformanceVersion,
    #[doc(alias = "denormBehaviorIndependence")]
    denorm_behavior_independence: ShaderFloatControlsIndependence,
    #[doc(alias = "roundingModeIndependence")]
    rounding_mode_independence: ShaderFloatControlsIndependence,
    #[doc(alias = "shaderSignedZeroInfNanPreserveFloat16")]
    shader_signed_zero_inf_nan_preserve_float16: Bool32,
    #[doc(alias = "shaderSignedZeroInfNanPreserveFloat32")]
    shader_signed_zero_inf_nan_preserve_float32: Bool32,
    #[doc(alias = "shaderSignedZeroInfNanPreserveFloat64")]
    shader_signed_zero_inf_nan_preserve_float64: Bool32,
    #[doc(alias = "shaderDenormPreserveFloat16")]
    shader_denorm_preserve_float16: Bool32,
    #[doc(alias = "shaderDenormPreserveFloat32")]
    shader_denorm_preserve_float32: Bool32,
    #[doc(alias = "shaderDenormPreserveFloat64")]
    shader_denorm_preserve_float64: Bool32,
    #[doc(alias = "shaderDenormFlushToZeroFloat16")]
    shader_denorm_flush_to_zero_float16: Bool32,
    #[doc(alias = "shaderDenormFlushToZeroFloat32")]
    shader_denorm_flush_to_zero_float32: Bool32,
    #[doc(alias = "shaderDenormFlushToZeroFloat64")]
    shader_denorm_flush_to_zero_float64: Bool32,
    #[doc(alias = "shaderRoundingModeRTEFloat16")]
    shader_rounding_mode_rte_float16: Bool32,
    #[doc(alias = "shaderRoundingModeRTEFloat32")]
    shader_rounding_mode_rte_float32: Bool32,
    #[doc(alias = "shaderRoundingModeRTEFloat64")]
    shader_rounding_mode_rte_float64: Bool32,
    #[doc(alias = "shaderRoundingModeRTZFloat16")]
    shader_rounding_mode_rtz_float16: Bool32,
    #[doc(alias = "shaderRoundingModeRTZFloat32")]
    shader_rounding_mode_rtz_float32: Bool32,
    #[doc(alias = "shaderRoundingModeRTZFloat64")]
    shader_rounding_mode_rtz_float64: Bool32,
    #[doc(alias = "maxUpdateAfterBindDescriptorsInAllPools")]
    max_update_after_bind_descriptors_in_all_pools: u32,
    #[doc(alias = "shaderUniformBufferArrayNonUniformIndexingNative")]
    shader_uniform_buffer_array_non_uniform_indexing_native: Bool32,
    #[doc(alias = "shaderSampledImageArrayNonUniformIndexingNative")]
    shader_sampled_image_array_non_uniform_indexing_native: Bool32,
    #[doc(alias = "shaderStorageBufferArrayNonUniformIndexingNative")]
    shader_storage_buffer_array_non_uniform_indexing_native: Bool32,
    #[doc(alias = "shaderStorageImageArrayNonUniformIndexingNative")]
    shader_storage_image_array_non_uniform_indexing_native: Bool32,
    #[doc(alias = "shaderInputAttachmentArrayNonUniformIndexingNative")]
    shader_input_attachment_array_non_uniform_indexing_native: Bool32,
    #[doc(alias = "robustBufferAccessUpdateAfterBind")]
    robust_buffer_access_update_after_bind: Bool32,
    #[doc(alias = "quadDivergentImplicitLod")]
    quad_divergent_implicit_lod: Bool32,
    #[doc(alias = "maxPerStageDescriptorUpdateAfterBindSamplers")]
    max_per_stage_descriptor_update_after_bind_samplers: u32,
    #[doc(alias = "maxPerStageDescriptorUpdateAfterBindUniformBuffers")]
    max_per_stage_descriptor_update_after_bind_uniform_buffers: u32,
    #[doc(alias = "maxPerStageDescriptorUpdateAfterBindStorageBuffers")]
    max_per_stage_descriptor_update_after_bind_storage_buffers: u32,
    #[doc(alias = "maxPerStageDescriptorUpdateAfterBindSampledImages")]
    max_per_stage_descriptor_update_after_bind_sampled_images: u32,
    #[doc(alias = "maxPerStageDescriptorUpdateAfterBindStorageImages")]
    max_per_stage_descriptor_update_after_bind_storage_images: u32,
    #[doc(alias = "maxPerStageDescriptorUpdateAfterBindInputAttachments")]
    max_per_stage_descriptor_update_after_bind_input_attachments: u32,
    #[doc(alias = "maxPerStageUpdateAfterBindResources")]
    max_per_stage_update_after_bind_resources: u32,
    #[doc(alias = "maxDescriptorSetUpdateAfterBindSamplers")]
    max_descriptor_set_update_after_bind_samplers: u32,
    #[doc(alias = "maxDescriptorSetUpdateAfterBindUniformBuffers")]
    max_descriptor_set_update_after_bind_uniform_buffers: u32,
    #[doc(alias = "maxDescriptorSetUpdateAfterBindUniformBuffersDynamic")]
    max_descriptor_set_update_after_bind_uniform_buffers_dynamic: u32,
    #[doc(alias = "maxDescriptorSetUpdateAfterBindStorageBuffers")]
    max_descriptor_set_update_after_bind_storage_buffers: u32,
    #[doc(alias = "maxDescriptorSetUpdateAfterBindStorageBuffersDynamic")]
    max_descriptor_set_update_after_bind_storage_buffers_dynamic: u32,
    #[doc(alias = "maxDescriptorSetUpdateAfterBindSampledImages")]
    max_descriptor_set_update_after_bind_sampled_images: u32,
    #[doc(alias = "maxDescriptorSetUpdateAfterBindStorageImages")]
    max_descriptor_set_update_after_bind_storage_images: u32,
    #[doc(alias = "maxDescriptorSetUpdateAfterBindInputAttachments")]
    max_descriptor_set_update_after_bind_input_attachments: u32,
    #[doc(alias = "supportedDepthResolveModes")]
    supported_depth_resolve_modes: ResolveModeFlags,
    #[doc(alias = "supportedStencilResolveModes")]
    supported_stencil_resolve_modes: ResolveModeFlags,
    #[doc(alias = "independentResolveNone")]
    independent_resolve_none: Bool32,
    #[doc(alias = "independentResolve")]
    independent_resolve: Bool32,
    #[doc(alias = "filterMinmaxSingleComponentFormats")]
    filter_minmax_single_component_formats: Bool32,
    #[doc(alias = "filterMinmaxImageComponentMapping")]
    filter_minmax_image_component_mapping: Bool32,
    #[doc(alias = "maxTimelineSemaphoreValueDifference")]
    max_timeline_semaphore_value_difference: u64,
    #[doc(alias = "framebufferIntegerColorSampleCounts")]
    framebuffer_integer_color_sample_counts: SampleCountFlags,
}
# [doc = include_str ! ("../../../doc/VkSemaphoreWaitFlagBits.md")]
#[doc(alias = "VkSemaphoreWaitFlags")]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SemaphoreWaitFlags(u32);
impl SemaphoreWaitFlags {
    #[doc(alias = "VK_SEMAPHORE_WAIT_ANY_BIT")]
    pub const ANY: Self = Self(1);
    #[doc(alias = "VK_SEMAPHORE_WAIT_ANY_BIT_KHR")]
    #[cfg(feature = "VK_KHR_timeline_semaphore")]
    pub const ANY_KHR: Self = Self::ANY;
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
    ///Returns a value with all of the flags enabled
    #[inline]
    #[allow(unused_mut)]
    pub const fn all() -> Self {
        let mut all = Self::empty();
        {
            all |= Self::ANY;
        }
        #[cfg(feature = "VK_KHR_timeline_semaphore")]
        {
            all |= Self::ANY_KHR;
        }
        all
    }
    ///Returns the raw bits
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0
    }
    ///Convert raw bits into a bit flags checking that only valid
    ///bits are contained.
    #[inline]
    pub const fn from_bits(bits: u32) -> Option<Self> {
        if (bits & !Self::all().bits()) == 0 {
            Some(Self(bits))
        } else {
            None
        }
    }
    ///Convert raw bits into a bit flags truncating all invalid
    ///bits that may be contained.
    #[inline]
    pub const fn from_bits_truncate(bits: u32) -> Self {
        Self(Self::all().0 & bits)
    }
    ///Convert raw bits into a bit preserving all bits
    ///
    ///# Safety
    ///The caller of this function must ensure that all of the bits are valid.
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
    ///Returns `true` if no flags are currently set
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.bits() == Self::empty().bits()
    }
    ///Returns `true` if all flags are currently set
    #[inline]
    pub const fn is_all(&self) -> bool {
        self.bits() == Self::all().bits()
    }
    ///Returns `true` if there are flags in common to `self` and `other`
    #[inline]
    pub const fn intersects(&self, other: Self) -> bool {
        !Self(self.bits() & other.bits()).is_empty()
    }
    ///Returns `true` if all of the flags in `other` are contained `self`
    #[inline]
    pub const fn contains(&self, other: Self) -> bool {
        (self.bits() & other.bits()) == other.bits()
    }
    ///Inserts a set of flags in place
    #[inline]
    pub fn insert(&mut self, other: Self) {
        self.0 |= other.bits()
    }
    ///Removes a set of flags in place
    #[inline]
    pub fn remove(&mut self, other: Self) {
        self.0 &= !other.bits();
    }
    ///Toggles a set of flags in place
    #[inline]
    pub fn toggle(&mut self, other: Self) {
        self.0 ^= other.bits();
    }
    ///Inserts or removes the specified flags depending on the value of `is_insert`
    #[inline]
    pub fn set(&mut self, other: Self, is_insert: bool) {
        if is_insert {
            self.insert(other);
        } else {
            self.remove(other);
        }
    }
    ///Returns the intersection between `self` and `other`
    #[inline]
    pub const fn intersection(self, other: Self) -> Self {
        Self(self.bits() & other.bits())
    }
    ///Returns the union between `self` and `other`
    #[inline]
    pub const fn union(self, other: Self) -> Self {
        Self(self.bits() | other.bits())
    }
    ///Returns the difference between `self` and `other`
    #[inline]
    pub const fn difference(self, other: Self) -> Self {
        Self(self.bits() & !other.bits())
    }
    ///Returns the [symmetric difference][sym-diff] between `self` and `other`
    ///
    ///[sym-diff]: https://en.wikipedia.org/wiki/Symmetric_difference
    #[inline]
    pub const fn symmetric_difference(self, other: Self) -> Self {
        Self(self.bits() ^ other.bits())
    }
    ///Returns the complement of `self`.
    #[inline]
    pub const fn complement(self) -> Self {
        Self::from_bits_truncate(!self.bits())
    }
}
impl const std::ops::BitOr for SemaphoreWaitFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl const std::ops::BitOrAssign for SemaphoreWaitFlags {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for SemaphoreWaitFlags {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl const std::ops::BitXorAssign for SemaphoreWaitFlags {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for SemaphoreWaitFlags {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl const std::ops::BitAndAssign for SemaphoreWaitFlags {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for SemaphoreWaitFlags {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl const std::ops::SubAssign for SemaphoreWaitFlags {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for SemaphoreWaitFlags {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<SemaphoreWaitFlags> for SemaphoreWaitFlags {
    fn extend<T: IntoIterator<Item = SemaphoreWaitFlags>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl FromIterator<SemaphoreWaitFlags> for SemaphoreWaitFlags {
    fn from_iter<T: IntoIterator<Item = SemaphoreWaitFlags>>(iterator: T) -> SemaphoreWaitFlags {
        let mut out = Self::empty();
        <Self as Extend<SemaphoreWaitFlags>>::extend(&mut out, iterator);
        out
    }
}
impl const Default for SemaphoreWaitFlags {
    fn default() -> Self {
        Self::empty()
    }
}
impl const From<SemaphoreWaitFlagBits> for SemaphoreWaitFlags {
    fn from(bit: SemaphoreWaitFlagBits) -> Self {
        unsafe { Self::from_bits_unchecked(bit.bits()) }
    }
}
impl Extend<SemaphoreWaitFlagBits> for SemaphoreWaitFlags {
    fn extend<T: IntoIterator<Item = SemaphoreWaitFlagBits>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, Self::from(i));
        }
    }
}
impl FromIterator<SemaphoreWaitFlagBits> for SemaphoreWaitFlags {
    fn from_iter<T: IntoIterator<Item = SemaphoreWaitFlagBits>>(iterator: T) -> SemaphoreWaitFlags {
        let mut out = Self::empty();
        <Self as Extend<SemaphoreWaitFlagBits>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for SemaphoreWaitFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(SemaphoreWaitFlags);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == SemaphoreWaitFlags::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(SemaphoreWaitFlags::ANY) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(ANY))?;
                    }
                    #[cfg(feature = "VK_KHR_timeline_semaphore")]
                    if self.0.contains(SemaphoreWaitFlags::ANY_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(ANY_KHR))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(SemaphoreWaitFlags))
            .field(&Flags(*self))
            .finish()
    }
}
# [doc = include_str ! ("../../../doc/VkDescriptorBindingFlagBits.md")]
#[doc(alias = "VkDescriptorBindingFlags")]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DescriptorBindingFlags(u32);
impl DescriptorBindingFlags {
    #[doc(alias = "VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT")]
    pub const UPDATE_AFTER_BIND: Self = Self(1);
    #[doc(alias = "VK_DESCRIPTOR_BINDING_UPDATE_UNUSED_WHILE_PENDING_BIT")]
    pub const UPDATE_UNUSED_WHILE_PENDING: Self = Self(2);
    #[doc(alias = "VK_DESCRIPTOR_BINDING_PARTIALLY_BOUND_BIT")]
    pub const PARTIALLY_BOUND: Self = Self(4);
    #[doc(alias = "VK_DESCRIPTOR_BINDING_VARIABLE_DESCRIPTOR_COUNT_BIT")]
    pub const VARIABLE_DESCRIPTOR_COUNT: Self = Self(8);
    #[doc(alias = "VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT_EXT")]
    #[cfg(feature = "VK_EXT_descriptor_indexing")]
    pub const UPDATE_AFTER_BIND_EXT: Self = Self::UPDATE_AFTER_BIND;
    #[doc(alias = "VK_DESCRIPTOR_BINDING_UPDATE_UNUSED_WHILE_PENDING_BIT_EXT")]
    #[cfg(feature = "VK_EXT_descriptor_indexing")]
    pub const UPDATE_UNUSED_WHILE_PENDING_EXT: Self = Self::UPDATE_UNUSED_WHILE_PENDING;
    #[doc(alias = "VK_DESCRIPTOR_BINDING_PARTIALLY_BOUND_BIT_EXT")]
    #[cfg(feature = "VK_EXT_descriptor_indexing")]
    pub const PARTIALLY_BOUND_EXT: Self = Self::PARTIALLY_BOUND;
    #[doc(alias = "VK_DESCRIPTOR_BINDING_VARIABLE_DESCRIPTOR_COUNT_BIT_EXT")]
    #[cfg(feature = "VK_EXT_descriptor_indexing")]
    pub const VARIABLE_DESCRIPTOR_COUNT_EXT: Self = Self::VARIABLE_DESCRIPTOR_COUNT;
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
    ///Returns a value with all of the flags enabled
    #[inline]
    #[allow(unused_mut)]
    pub const fn all() -> Self {
        let mut all = Self::empty();
        {
            all |= Self::UPDATE_AFTER_BIND;
        }
        {
            all |= Self::UPDATE_UNUSED_WHILE_PENDING;
        }
        {
            all |= Self::PARTIALLY_BOUND;
        }
        {
            all |= Self::VARIABLE_DESCRIPTOR_COUNT;
        }
        #[cfg(feature = "VK_EXT_descriptor_indexing")]
        {
            all |= Self::UPDATE_AFTER_BIND_EXT;
        }
        #[cfg(feature = "VK_EXT_descriptor_indexing")]
        {
            all |= Self::UPDATE_UNUSED_WHILE_PENDING_EXT;
        }
        #[cfg(feature = "VK_EXT_descriptor_indexing")]
        {
            all |= Self::PARTIALLY_BOUND_EXT;
        }
        #[cfg(feature = "VK_EXT_descriptor_indexing")]
        {
            all |= Self::VARIABLE_DESCRIPTOR_COUNT_EXT;
        }
        all
    }
    ///Returns the raw bits
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0
    }
    ///Convert raw bits into a bit flags checking that only valid
    ///bits are contained.
    #[inline]
    pub const fn from_bits(bits: u32) -> Option<Self> {
        if (bits & !Self::all().bits()) == 0 {
            Some(Self(bits))
        } else {
            None
        }
    }
    ///Convert raw bits into a bit flags truncating all invalid
    ///bits that may be contained.
    #[inline]
    pub const fn from_bits_truncate(bits: u32) -> Self {
        Self(Self::all().0 & bits)
    }
    ///Convert raw bits into a bit preserving all bits
    ///
    ///# Safety
    ///The caller of this function must ensure that all of the bits are valid.
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
    ///Returns `true` if no flags are currently set
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.bits() == Self::empty().bits()
    }
    ///Returns `true` if all flags are currently set
    #[inline]
    pub const fn is_all(&self) -> bool {
        self.bits() == Self::all().bits()
    }
    ///Returns `true` if there are flags in common to `self` and `other`
    #[inline]
    pub const fn intersects(&self, other: Self) -> bool {
        !Self(self.bits() & other.bits()).is_empty()
    }
    ///Returns `true` if all of the flags in `other` are contained `self`
    #[inline]
    pub const fn contains(&self, other: Self) -> bool {
        (self.bits() & other.bits()) == other.bits()
    }
    ///Inserts a set of flags in place
    #[inline]
    pub fn insert(&mut self, other: Self) {
        self.0 |= other.bits()
    }
    ///Removes a set of flags in place
    #[inline]
    pub fn remove(&mut self, other: Self) {
        self.0 &= !other.bits();
    }
    ///Toggles a set of flags in place
    #[inline]
    pub fn toggle(&mut self, other: Self) {
        self.0 ^= other.bits();
    }
    ///Inserts or removes the specified flags depending on the value of `is_insert`
    #[inline]
    pub fn set(&mut self, other: Self, is_insert: bool) {
        if is_insert {
            self.insert(other);
        } else {
            self.remove(other);
        }
    }
    ///Returns the intersection between `self` and `other`
    #[inline]
    pub const fn intersection(self, other: Self) -> Self {
        Self(self.bits() & other.bits())
    }
    ///Returns the union between `self` and `other`
    #[inline]
    pub const fn union(self, other: Self) -> Self {
        Self(self.bits() | other.bits())
    }
    ///Returns the difference between `self` and `other`
    #[inline]
    pub const fn difference(self, other: Self) -> Self {
        Self(self.bits() & !other.bits())
    }
    ///Returns the [symmetric difference][sym-diff] between `self` and `other`
    ///
    ///[sym-diff]: https://en.wikipedia.org/wiki/Symmetric_difference
    #[inline]
    pub const fn symmetric_difference(self, other: Self) -> Self {
        Self(self.bits() ^ other.bits())
    }
    ///Returns the complement of `self`.
    #[inline]
    pub const fn complement(self) -> Self {
        Self::from_bits_truncate(!self.bits())
    }
}
impl const std::ops::BitOr for DescriptorBindingFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl const std::ops::BitOrAssign for DescriptorBindingFlags {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for DescriptorBindingFlags {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl const std::ops::BitXorAssign for DescriptorBindingFlags {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for DescriptorBindingFlags {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl const std::ops::BitAndAssign for DescriptorBindingFlags {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for DescriptorBindingFlags {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl const std::ops::SubAssign for DescriptorBindingFlags {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for DescriptorBindingFlags {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<DescriptorBindingFlags> for DescriptorBindingFlags {
    fn extend<T: IntoIterator<Item = DescriptorBindingFlags>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl FromIterator<DescriptorBindingFlags> for DescriptorBindingFlags {
    fn from_iter<T: IntoIterator<Item = DescriptorBindingFlags>>(iterator: T) -> DescriptorBindingFlags {
        let mut out = Self::empty();
        <Self as Extend<DescriptorBindingFlags>>::extend(&mut out, iterator);
        out
    }
}
impl const Default for DescriptorBindingFlags {
    fn default() -> Self {
        Self::empty()
    }
}
impl const From<DescriptorBindingFlagBits> for DescriptorBindingFlags {
    fn from(bit: DescriptorBindingFlagBits) -> Self {
        unsafe { Self::from_bits_unchecked(bit.bits()) }
    }
}
impl Extend<DescriptorBindingFlagBits> for DescriptorBindingFlags {
    fn extend<T: IntoIterator<Item = DescriptorBindingFlagBits>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, Self::from(i));
        }
    }
}
impl FromIterator<DescriptorBindingFlagBits> for DescriptorBindingFlags {
    fn from_iter<T: IntoIterator<Item = DescriptorBindingFlagBits>>(iterator: T) -> DescriptorBindingFlags {
        let mut out = Self::empty();
        <Self as Extend<DescriptorBindingFlagBits>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for DescriptorBindingFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(DescriptorBindingFlags);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == DescriptorBindingFlags::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(DescriptorBindingFlags::UPDATE_AFTER_BIND) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(UPDATE_AFTER_BIND))?;
                    }
                    if self.0.contains(DescriptorBindingFlags::UPDATE_UNUSED_WHILE_PENDING) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(UPDATE_UNUSED_WHILE_PENDING))?;
                    }
                    if self.0.contains(DescriptorBindingFlags::PARTIALLY_BOUND) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PARTIALLY_BOUND))?;
                    }
                    if self.0.contains(DescriptorBindingFlags::VARIABLE_DESCRIPTOR_COUNT) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(VARIABLE_DESCRIPTOR_COUNT))?;
                    }
                    #[cfg(feature = "VK_EXT_descriptor_indexing")]
                    if self.0.contains(DescriptorBindingFlags::UPDATE_AFTER_BIND_EXT) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(UPDATE_AFTER_BIND_EXT))?;
                    }
                    #[cfg(feature = "VK_EXT_descriptor_indexing")]
                    if self.0.contains(DescriptorBindingFlags::UPDATE_UNUSED_WHILE_PENDING_EXT) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(UPDATE_UNUSED_WHILE_PENDING_EXT))?;
                    }
                    #[cfg(feature = "VK_EXT_descriptor_indexing")]
                    if self.0.contains(DescriptorBindingFlags::PARTIALLY_BOUND_EXT) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PARTIALLY_BOUND_EXT))?;
                    }
                    #[cfg(feature = "VK_EXT_descriptor_indexing")]
                    if self.0.contains(DescriptorBindingFlags::VARIABLE_DESCRIPTOR_COUNT_EXT) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(VARIABLE_DESCRIPTOR_COUNT_EXT))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(DescriptorBindingFlags))
            .field(&Flags(*self))
            .finish()
    }
}
# [doc = include_str ! ("../../../doc/VkResolveModeFlagBits.md")]
#[doc(alias = "VkResolveModeFlags")]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ResolveModeFlags(u32);
impl ResolveModeFlags {
    #[doc(alias = "VK_RESOLVE_MODE_NONE")]
    pub const NONE: Self = Self(0);
    #[doc(alias = "VK_RESOLVE_MODE_SAMPLE_ZERO_BIT")]
    pub const SAMPLE_ZERO: Self = Self(1);
    #[doc(alias = "VK_RESOLVE_MODE_AVERAGE_BIT")]
    pub const AVERAGE: Self = Self(2);
    #[doc(alias = "VK_RESOLVE_MODE_MIN_BIT")]
    pub const MIN: Self = Self(4);
    #[doc(alias = "VK_RESOLVE_MODE_MAX_BIT")]
    pub const MAX: Self = Self(8);
    #[doc(alias = "VK_RESOLVE_MODE_NONE_KHR")]
    #[cfg(feature = "VK_KHR_depth_stencil_resolve")]
    pub const NONE_KHR: Self = Self::NONE;
    #[doc(alias = "VK_RESOLVE_MODE_SAMPLE_ZERO_BIT_KHR")]
    #[cfg(feature = "VK_KHR_depth_stencil_resolve")]
    pub const SAMPLE_ZERO_KHR: Self = Self::SAMPLE_ZERO;
    #[doc(alias = "VK_RESOLVE_MODE_AVERAGE_BIT_KHR")]
    #[cfg(feature = "VK_KHR_depth_stencil_resolve")]
    pub const AVERAGE_KHR: Self = Self::AVERAGE;
    #[doc(alias = "VK_RESOLVE_MODE_MIN_BIT_KHR")]
    #[cfg(feature = "VK_KHR_depth_stencil_resolve")]
    pub const MIN_KHR: Self = Self::MIN;
    #[doc(alias = "VK_RESOLVE_MODE_MAX_BIT_KHR")]
    #[cfg(feature = "VK_KHR_depth_stencil_resolve")]
    pub const MAX_KHR: Self = Self::MAX;
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
    ///Returns a value with all of the flags enabled
    #[inline]
    #[allow(unused_mut)]
    pub const fn all() -> Self {
        let mut all = Self::empty();
        {
            all |= Self::NONE;
        }
        {
            all |= Self::SAMPLE_ZERO;
        }
        {
            all |= Self::AVERAGE;
        }
        {
            all |= Self::MIN;
        }
        {
            all |= Self::MAX;
        }
        #[cfg(feature = "VK_KHR_depth_stencil_resolve")]
        {
            all |= Self::NONE_KHR;
        }
        #[cfg(feature = "VK_KHR_depth_stencil_resolve")]
        {
            all |= Self::SAMPLE_ZERO_KHR;
        }
        #[cfg(feature = "VK_KHR_depth_stencil_resolve")]
        {
            all |= Self::AVERAGE_KHR;
        }
        #[cfg(feature = "VK_KHR_depth_stencil_resolve")]
        {
            all |= Self::MIN_KHR;
        }
        #[cfg(feature = "VK_KHR_depth_stencil_resolve")]
        {
            all |= Self::MAX_KHR;
        }
        all
    }
    ///Returns the raw bits
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0
    }
    ///Convert raw bits into a bit flags checking that only valid
    ///bits are contained.
    #[inline]
    pub const fn from_bits(bits: u32) -> Option<Self> {
        if (bits & !Self::all().bits()) == 0 {
            Some(Self(bits))
        } else {
            None
        }
    }
    ///Convert raw bits into a bit flags truncating all invalid
    ///bits that may be contained.
    #[inline]
    pub const fn from_bits_truncate(bits: u32) -> Self {
        Self(Self::all().0 & bits)
    }
    ///Convert raw bits into a bit preserving all bits
    ///
    ///# Safety
    ///The caller of this function must ensure that all of the bits are valid.
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
    ///Returns `true` if no flags are currently set
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.bits() == Self::empty().bits()
    }
    ///Returns `true` if all flags are currently set
    #[inline]
    pub const fn is_all(&self) -> bool {
        self.bits() == Self::all().bits()
    }
    ///Returns `true` if there are flags in common to `self` and `other`
    #[inline]
    pub const fn intersects(&self, other: Self) -> bool {
        !Self(self.bits() & other.bits()).is_empty()
    }
    ///Returns `true` if all of the flags in `other` are contained `self`
    #[inline]
    pub const fn contains(&self, other: Self) -> bool {
        (self.bits() & other.bits()) == other.bits()
    }
    ///Inserts a set of flags in place
    #[inline]
    pub fn insert(&mut self, other: Self) {
        self.0 |= other.bits()
    }
    ///Removes a set of flags in place
    #[inline]
    pub fn remove(&mut self, other: Self) {
        self.0 &= !other.bits();
    }
    ///Toggles a set of flags in place
    #[inline]
    pub fn toggle(&mut self, other: Self) {
        self.0 ^= other.bits();
    }
    ///Inserts or removes the specified flags depending on the value of `is_insert`
    #[inline]
    pub fn set(&mut self, other: Self, is_insert: bool) {
        if is_insert {
            self.insert(other);
        } else {
            self.remove(other);
        }
    }
    ///Returns the intersection between `self` and `other`
    #[inline]
    pub const fn intersection(self, other: Self) -> Self {
        Self(self.bits() & other.bits())
    }
    ///Returns the union between `self` and `other`
    #[inline]
    pub const fn union(self, other: Self) -> Self {
        Self(self.bits() | other.bits())
    }
    ///Returns the difference between `self` and `other`
    #[inline]
    pub const fn difference(self, other: Self) -> Self {
        Self(self.bits() & !other.bits())
    }
    ///Returns the [symmetric difference][sym-diff] between `self` and `other`
    ///
    ///[sym-diff]: https://en.wikipedia.org/wiki/Symmetric_difference
    #[inline]
    pub const fn symmetric_difference(self, other: Self) -> Self {
        Self(self.bits() ^ other.bits())
    }
    ///Returns the complement of `self`.
    #[inline]
    pub const fn complement(self) -> Self {
        Self::from_bits_truncate(!self.bits())
    }
}
impl const std::ops::BitOr for ResolveModeFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl const std::ops::BitOrAssign for ResolveModeFlags {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for ResolveModeFlags {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl const std::ops::BitXorAssign for ResolveModeFlags {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for ResolveModeFlags {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl const std::ops::BitAndAssign for ResolveModeFlags {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for ResolveModeFlags {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl const std::ops::SubAssign for ResolveModeFlags {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for ResolveModeFlags {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<ResolveModeFlags> for ResolveModeFlags {
    fn extend<T: IntoIterator<Item = ResolveModeFlags>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl FromIterator<ResolveModeFlags> for ResolveModeFlags {
    fn from_iter<T: IntoIterator<Item = ResolveModeFlags>>(iterator: T) -> ResolveModeFlags {
        let mut out = Self::empty();
        <Self as Extend<ResolveModeFlags>>::extend(&mut out, iterator);
        out
    }
}
impl const Default for ResolveModeFlags {
    fn default() -> Self {
        Self::empty()
    }
}
impl const From<ResolveModeFlagBits> for ResolveModeFlags {
    fn from(bit: ResolveModeFlagBits) -> Self {
        unsafe { Self::from_bits_unchecked(bit.bits()) }
    }
}
impl Extend<ResolveModeFlagBits> for ResolveModeFlags {
    fn extend<T: IntoIterator<Item = ResolveModeFlagBits>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, Self::from(i));
        }
    }
}
impl FromIterator<ResolveModeFlagBits> for ResolveModeFlags {
    fn from_iter<T: IntoIterator<Item = ResolveModeFlagBits>>(iterator: T) -> ResolveModeFlags {
        let mut out = Self::empty();
        <Self as Extend<ResolveModeFlagBits>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for ResolveModeFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(ResolveModeFlags);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == ResolveModeFlags::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(ResolveModeFlags::NONE) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(NONE))?;
                    }
                    if self.0.contains(ResolveModeFlags::SAMPLE_ZERO) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(SAMPLE_ZERO))?;
                    }
                    if self.0.contains(ResolveModeFlags::AVERAGE) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(AVERAGE))?;
                    }
                    if self.0.contains(ResolveModeFlags::MIN) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(MIN))?;
                    }
                    if self.0.contains(ResolveModeFlags::MAX) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(MAX))?;
                    }
                    #[cfg(feature = "VK_KHR_depth_stencil_resolve")]
                    if self.0.contains(ResolveModeFlags::NONE_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(NONE_KHR))?;
                    }
                    #[cfg(feature = "VK_KHR_depth_stencil_resolve")]
                    if self.0.contains(ResolveModeFlags::SAMPLE_ZERO_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(SAMPLE_ZERO_KHR))?;
                    }
                    #[cfg(feature = "VK_KHR_depth_stencil_resolve")]
                    if self.0.contains(ResolveModeFlags::AVERAGE_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(AVERAGE_KHR))?;
                    }
                    #[cfg(feature = "VK_KHR_depth_stencil_resolve")]
                    if self.0.contains(ResolveModeFlags::MIN_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(MIN_KHR))?;
                    }
                    #[cfg(feature = "VK_KHR_depth_stencil_resolve")]
                    if self.0.contains(ResolveModeFlags::MAX_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(MAX_KHR))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(ResolveModeFlags))
            .field(&Flags(*self))
            .finish()
    }
}
# [doc = include_str ! ("../../../doc/VkSemaphoreWaitFlagBits.md")]
#[doc(alias = "VkSemaphoreWaitFlagBits")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct SemaphoreWaitFlagBits(u32);
impl SemaphoreWaitFlagBits {
    #[doc(alias = "VK_SEMAPHORE_WAIT_ANY_BIT")]
    pub const ANY: Self = Self(1);
    #[doc(alias = "VK_SEMAPHORE_WAIT_ANY_BIT_KHR")]
    #[cfg(feature = "VK_KHR_timeline_semaphore")]
    pub const ANY_KHR: Self = Self::ANY;
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
    ///The bits of this variant
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0
    }
    ///Builds a bitmask from the bits of this variant
    #[inline]
    pub const fn from_bits(bits: u32) -> Option<Self> {
        match bits {
            x if x == Self::ANY.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
}
# [doc = include_str ! ("../../../doc/VkDescriptorBindingFlagBits.md")]
#[doc(alias = "VkDescriptorBindingFlagBits")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct DescriptorBindingFlagBits(u32);
impl DescriptorBindingFlagBits {
    #[doc(alias = "VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT")]
    pub const UPDATE_AFTER_BIND: Self = Self(1);
    #[doc(alias = "VK_DESCRIPTOR_BINDING_UPDATE_UNUSED_WHILE_PENDING_BIT")]
    pub const UPDATE_UNUSED_WHILE_PENDING: Self = Self(2);
    #[doc(alias = "VK_DESCRIPTOR_BINDING_PARTIALLY_BOUND_BIT")]
    pub const PARTIALLY_BOUND: Self = Self(4);
    #[doc(alias = "VK_DESCRIPTOR_BINDING_VARIABLE_DESCRIPTOR_COUNT_BIT")]
    pub const VARIABLE_DESCRIPTOR_COUNT: Self = Self(8);
    #[doc(alias = "VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT_EXT")]
    #[cfg(feature = "VK_EXT_descriptor_indexing")]
    pub const UPDATE_AFTER_BIND_EXT: Self = Self::UPDATE_AFTER_BIND;
    #[doc(alias = "VK_DESCRIPTOR_BINDING_UPDATE_UNUSED_WHILE_PENDING_BIT_EXT")]
    #[cfg(feature = "VK_EXT_descriptor_indexing")]
    pub const UPDATE_UNUSED_WHILE_PENDING_EXT: Self = Self::UPDATE_UNUSED_WHILE_PENDING;
    #[doc(alias = "VK_DESCRIPTOR_BINDING_PARTIALLY_BOUND_BIT_EXT")]
    #[cfg(feature = "VK_EXT_descriptor_indexing")]
    pub const PARTIALLY_BOUND_EXT: Self = Self::PARTIALLY_BOUND;
    #[doc(alias = "VK_DESCRIPTOR_BINDING_VARIABLE_DESCRIPTOR_COUNT_BIT_EXT")]
    #[cfg(feature = "VK_EXT_descriptor_indexing")]
    pub const VARIABLE_DESCRIPTOR_COUNT_EXT: Self = Self::VARIABLE_DESCRIPTOR_COUNT;
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
    ///The bits of this variant
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0
    }
    ///Builds a bitmask from the bits of this variant
    #[inline]
    pub const fn from_bits(bits: u32) -> Option<Self> {
        match bits {
            x if x == Self::UPDATE_AFTER_BIND.bits() => Some(Self(x)),
            x if x == Self::UPDATE_UNUSED_WHILE_PENDING.bits() => Some(Self(x)),
            x if x == Self::PARTIALLY_BOUND.bits() => Some(Self(x)),
            x if x == Self::VARIABLE_DESCRIPTOR_COUNT.bits() => Some(Self(x)),
            #[cfg(feature = "VK_QCOM_extension_369")]
            x if x == Self::RESERVED4_QCOM.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
}
# [doc = include_str ! ("../../../doc/VkResolveModeFlagBits.md")]
#[doc(alias = "VkResolveModeFlagBits")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct ResolveModeFlagBits(u32);
impl ResolveModeFlagBits {
    #[doc(alias = "VK_RESOLVE_MODE_NONE")]
    pub const NONE: Self = Self(0);
    #[doc(alias = "VK_RESOLVE_MODE_SAMPLE_ZERO_BIT")]
    pub const SAMPLE_ZERO: Self = Self(1);
    #[doc(alias = "VK_RESOLVE_MODE_AVERAGE_BIT")]
    pub const AVERAGE: Self = Self(2);
    #[doc(alias = "VK_RESOLVE_MODE_MIN_BIT")]
    pub const MIN: Self = Self(4);
    #[doc(alias = "VK_RESOLVE_MODE_MAX_BIT")]
    pub const MAX: Self = Self(8);
    #[doc(alias = "VK_RESOLVE_MODE_NONE_KHR")]
    #[cfg(feature = "VK_KHR_depth_stencil_resolve")]
    pub const NONE_KHR: Self = Self::NONE;
    #[doc(alias = "VK_RESOLVE_MODE_SAMPLE_ZERO_BIT_KHR")]
    #[cfg(feature = "VK_KHR_depth_stencil_resolve")]
    pub const SAMPLE_ZERO_KHR: Self = Self::SAMPLE_ZERO;
    #[doc(alias = "VK_RESOLVE_MODE_AVERAGE_BIT_KHR")]
    #[cfg(feature = "VK_KHR_depth_stencil_resolve")]
    pub const AVERAGE_KHR: Self = Self::AVERAGE;
    #[doc(alias = "VK_RESOLVE_MODE_MIN_BIT_KHR")]
    #[cfg(feature = "VK_KHR_depth_stencil_resolve")]
    pub const MIN_KHR: Self = Self::MIN;
    #[doc(alias = "VK_RESOLVE_MODE_MAX_BIT_KHR")]
    #[cfg(feature = "VK_KHR_depth_stencil_resolve")]
    pub const MAX_KHR: Self = Self::MAX;
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
    ///The bits of this variant
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0
    }
    ///Builds a bitmask from the bits of this variant
    #[inline]
    pub const fn from_bits(bits: u32) -> Option<Self> {
        match bits {
            x if x == Self::NONE.bits() => Some(Self(x)),
            x if x == Self::SAMPLE_ZERO.bits() => Some(Self(x)),
            x if x == Self::AVERAGE.bits() => Some(Self(x)),
            x if x == Self::MIN.bits() => Some(Self(x)),
            x if x == Self::MAX.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
}
# [doc = include_str ! ("../../../doc/VkSemaphoreType.md")]
#[doc(alias = "VkSemaphoreType")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct SemaphoreType(i32);
impl SemaphoreType {
    #[doc(alias = "VK_SEMAPHORE_TYPE_BINARY")]
    pub const BINARY: Self = Self(0);
    #[doc(alias = "VK_SEMAPHORE_TYPE_TIMELINE")]
    pub const TIMELINE: Self = Self(1);
    #[doc(alias = "VK_SEMAPHORE_TYPE_BINARY_KHR")]
    #[cfg(feature = "VK_KHR_timeline_semaphore")]
    pub const BINARY_KHR: Self = Self::BINARY;
    #[doc(alias = "VK_SEMAPHORE_TYPE_TIMELINE_KHR")]
    #[cfg(feature = "VK_KHR_timeline_semaphore")]
    pub const TIMELINE_KHR: Self = Self::TIMELINE;
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
    ///The bits of this variant
    #[inline]
    pub const fn bits(&self) -> i32 {
        self.0
    }
    ///Builds a bitmask from the bits of this variant
    #[inline]
    pub const fn from_bits(bits: i32) -> Option<Self> {
        match bits {
            x if x == Self::BINARY.bits() => Some(Self(x)),
            x if x == Self::TIMELINE.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: i32) -> Self {
        Self(bits)
    }
}
# [doc = include_str ! ("../../../doc/VkSamplerReductionMode.md")]
#[doc(alias = "VkSamplerReductionMode")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct SamplerReductionMode(i32);
impl SamplerReductionMode {
    #[doc(alias = "VK_SAMPLER_REDUCTION_MODE_WEIGHTED_AVERAGE")]
    pub const WEIGHTED_AVERAGE: Self = Self(0);
    #[doc(alias = "VK_SAMPLER_REDUCTION_MODE_MIN")]
    pub const MIN: Self = Self(1);
    #[doc(alias = "VK_SAMPLER_REDUCTION_MODE_MAX")]
    pub const MAX: Self = Self(2);
    #[doc(alias = "VK_SAMPLER_REDUCTION_MODE_WEIGHTED_AVERAGE_EXT")]
    #[cfg(feature = "VK_EXT_sampler_filter_minmax")]
    pub const WEIGHTED_AVERAGE_EXT: Self = Self::WEIGHTED_AVERAGE;
    #[doc(alias = "VK_SAMPLER_REDUCTION_MODE_MIN_EXT")]
    #[cfg(feature = "VK_EXT_sampler_filter_minmax")]
    pub const MIN_EXT: Self = Self::MIN;
    #[doc(alias = "VK_SAMPLER_REDUCTION_MODE_MAX_EXT")]
    #[cfg(feature = "VK_EXT_sampler_filter_minmax")]
    pub const MAX_EXT: Self = Self::MAX;
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
    ///The bits of this variant
    #[inline]
    pub const fn bits(&self) -> i32 {
        self.0
    }
    ///Builds a bitmask from the bits of this variant
    #[inline]
    pub const fn from_bits(bits: i32) -> Option<Self> {
        match bits {
            x if x == Self::WEIGHTED_AVERAGE.bits() => Some(Self(x)),
            x if x == Self::MIN.bits() => Some(Self(x)),
            x if x == Self::MAX.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: i32) -> Self {
        Self(bits)
    }
}
# [doc = include_str ! ("../../../doc/VkDriverId.md")]
#[doc(alias = "VkDriverId")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct DriverId(i32);
impl DriverId {
    #[doc(alias = "VK_DRIVER_ID_AMD_PROPRIETARY")]
    pub const AMD_PROPRIETARY: Self = Self(1);
    #[doc(alias = "VK_DRIVER_ID_AMD_OPEN_SOURCE")]
    pub const AMD_OPEN_SOURCE: Self = Self(2);
    #[doc(alias = "VK_DRIVER_ID_MESA_RADV")]
    pub const MESA_RADV: Self = Self(3);
    #[doc(alias = "VK_DRIVER_ID_NVIDIA_PROPRIETARY")]
    pub const NVIDIA_PROPRIETARY: Self = Self(4);
    #[doc(alias = "VK_DRIVER_ID_INTEL_PROPRIETARY_WINDOWS")]
    pub const INTEL_PROPRIETARY_WINDOWS: Self = Self(5);
    #[doc(alias = "VK_DRIVER_ID_INTEL_OPEN_SOURCE_MESA")]
    pub const INTEL_OPEN_SOURCE_MESA: Self = Self(6);
    #[doc(alias = "VK_DRIVER_ID_IMAGINATION_PROPRIETARY")]
    pub const IMAGINATION_PROPRIETARY: Self = Self(7);
    #[doc(alias = "VK_DRIVER_ID_QUALCOMM_PROPRIETARY")]
    pub const QUALCOMM_PROPRIETARY: Self = Self(8);
    #[doc(alias = "VK_DRIVER_ID_ARM_PROPRIETARY")]
    pub const ARM_PROPRIETARY: Self = Self(9);
    #[doc(alias = "VK_DRIVER_ID_GOOGLE_SWIFTSHADER")]
    pub const GOOGLE_SWIFTSHADER: Self = Self(10);
    #[doc(alias = "VK_DRIVER_ID_GGP_PROPRIETARY")]
    pub const GGP_PROPRIETARY: Self = Self(11);
    #[doc(alias = "VK_DRIVER_ID_BROADCOM_PROPRIETARY")]
    pub const BROADCOM_PROPRIETARY: Self = Self(12);
    #[doc(alias = "VK_DRIVER_ID_MESA_LLVMPIPE")]
    pub const MESA_LLVMPIPE: Self = Self(13);
    #[doc(alias = "VK_DRIVER_ID_MOLTENVK")]
    pub const MOLTENVK: Self = Self(14);
    #[doc(alias = "VK_DRIVER_ID_COREAVI_PROPRIETARY")]
    pub const COREAVI_PROPRIETARY: Self = Self(15);
    #[doc(alias = "VK_DRIVER_ID_JUICE_PROPRIETARY")]
    pub const JUICE_PROPRIETARY: Self = Self(16);
    #[doc(alias = "VK_DRIVER_ID_VERISILICON_PROPRIETARY")]
    pub const VERISILICON_PROPRIETARY: Self = Self(17);
    #[doc(alias = "VK_DRIVER_ID_MESA_TURNIP")]
    pub const MESA_TURNIP: Self = Self(18);
    #[doc(alias = "VK_DRIVER_ID_MESA_V3DV")]
    pub const MESA_V3DV: Self = Self(19);
    #[doc(alias = "VK_DRIVER_ID_MESA_PANVK")]
    pub const MESA_PANVK: Self = Self(20);
    #[doc(alias = "VK_DRIVER_ID_SAMSUNG_PROPRIETARY")]
    pub const SAMSUNG_PROPRIETARY: Self = Self(21);
    #[doc(alias = "VK_DRIVER_ID_MESA_VENUS")]
    pub const MESA_VENUS: Self = Self(22);
    #[doc(alias = "VK_DRIVER_ID_AMD_PROPRIETARY_KHR")]
    #[cfg(feature = "VK_KHR_driver_properties")]
    pub const AMD_PROPRIETARY_KHR: Self = Self::AMD_PROPRIETARY;
    #[doc(alias = "VK_DRIVER_ID_AMD_OPEN_SOURCE_KHR")]
    #[cfg(feature = "VK_KHR_driver_properties")]
    pub const AMD_OPEN_SOURCE_KHR: Self = Self::AMD_OPEN_SOURCE;
    #[doc(alias = "VK_DRIVER_ID_MESA_RADV_KHR")]
    #[cfg(feature = "VK_KHR_driver_properties")]
    pub const MESA_RADV_KHR: Self = Self::MESA_RADV;
    #[doc(alias = "VK_DRIVER_ID_NVIDIA_PROPRIETARY_KHR")]
    #[cfg(feature = "VK_KHR_driver_properties")]
    pub const NVIDIA_PROPRIETARY_KHR: Self = Self::NVIDIA_PROPRIETARY;
    #[doc(alias = "VK_DRIVER_ID_INTEL_PROPRIETARY_WINDOWS_KHR")]
    #[cfg(feature = "VK_KHR_driver_properties")]
    pub const INTEL_PROPRIETARY_WINDOWS_KHR: Self = Self::INTEL_PROPRIETARY_WINDOWS;
    #[doc(alias = "VK_DRIVER_ID_INTEL_OPEN_SOURCE_MESA_KHR")]
    #[cfg(feature = "VK_KHR_driver_properties")]
    pub const INTEL_OPEN_SOURCE_MESA_KHR: Self = Self::INTEL_OPEN_SOURCE_MESA;
    #[doc(alias = "VK_DRIVER_ID_IMAGINATION_PROPRIETARY_KHR")]
    #[cfg(feature = "VK_KHR_driver_properties")]
    pub const IMAGINATION_PROPRIETARY_KHR: Self = Self::IMAGINATION_PROPRIETARY;
    #[doc(alias = "VK_DRIVER_ID_QUALCOMM_PROPRIETARY_KHR")]
    #[cfg(feature = "VK_KHR_driver_properties")]
    pub const QUALCOMM_PROPRIETARY_KHR: Self = Self::QUALCOMM_PROPRIETARY;
    #[doc(alias = "VK_DRIVER_ID_ARM_PROPRIETARY_KHR")]
    #[cfg(feature = "VK_KHR_driver_properties")]
    pub const ARM_PROPRIETARY_KHR: Self = Self::ARM_PROPRIETARY;
    #[doc(alias = "VK_DRIVER_ID_GOOGLE_SWIFTSHADER_KHR")]
    #[cfg(feature = "VK_KHR_driver_properties")]
    pub const GOOGLE_SWIFTSHADER_KHR: Self = Self::GOOGLE_SWIFTSHADER;
    #[doc(alias = "VK_DRIVER_ID_GGP_PROPRIETARY_KHR")]
    #[cfg(feature = "VK_KHR_driver_properties")]
    pub const GGP_PROPRIETARY_KHR: Self = Self::GGP_PROPRIETARY;
    #[doc(alias = "VK_DRIVER_ID_BROADCOM_PROPRIETARY_KHR")]
    #[cfg(feature = "VK_KHR_driver_properties")]
    pub const BROADCOM_PROPRIETARY_KHR: Self = Self::BROADCOM_PROPRIETARY;
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
    ///The bits of this variant
    #[inline]
    pub const fn bits(&self) -> i32 {
        self.0
    }
    ///Builds a bitmask from the bits of this variant
    #[inline]
    pub const fn from_bits(bits: i32) -> Option<Self> {
        match bits {
            x if x == Self::AMD_PROPRIETARY.bits() => Some(Self(x)),
            x if x == Self::AMD_OPEN_SOURCE.bits() => Some(Self(x)),
            x if x == Self::MESA_RADV.bits() => Some(Self(x)),
            x if x == Self::NVIDIA_PROPRIETARY.bits() => Some(Self(x)),
            x if x == Self::INTEL_PROPRIETARY_WINDOWS.bits() => Some(Self(x)),
            x if x == Self::INTEL_OPEN_SOURCE_MESA.bits() => Some(Self(x)),
            x if x == Self::IMAGINATION_PROPRIETARY.bits() => Some(Self(x)),
            x if x == Self::QUALCOMM_PROPRIETARY.bits() => Some(Self(x)),
            x if x == Self::ARM_PROPRIETARY.bits() => Some(Self(x)),
            x if x == Self::GOOGLE_SWIFTSHADER.bits() => Some(Self(x)),
            x if x == Self::GGP_PROPRIETARY.bits() => Some(Self(x)),
            x if x == Self::BROADCOM_PROPRIETARY.bits() => Some(Self(x)),
            x if x == Self::MESA_LLVMPIPE.bits() => Some(Self(x)),
            x if x == Self::MOLTENVK.bits() => Some(Self(x)),
            x if x == Self::COREAVI_PROPRIETARY.bits() => Some(Self(x)),
            x if x == Self::JUICE_PROPRIETARY.bits() => Some(Self(x)),
            x if x == Self::VERISILICON_PROPRIETARY.bits() => Some(Self(x)),
            x if x == Self::MESA_TURNIP.bits() => Some(Self(x)),
            x if x == Self::MESA_V3DV.bits() => Some(Self(x)),
            x if x == Self::MESA_PANVK.bits() => Some(Self(x)),
            x if x == Self::SAMSUNG_PROPRIETARY.bits() => Some(Self(x)),
            x if x == Self::MESA_VENUS.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: i32) -> Self {
        Self(bits)
    }
}
# [doc = include_str ! ("../../../doc/VkShaderFloatControlsIndependence.md")]
#[doc(alias = "VkShaderFloatControlsIndependence")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct ShaderFloatControlsIndependence(i32);
impl ShaderFloatControlsIndependence {
    #[doc(alias = "VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_32_BIT_ONLY")]
    pub const N32_BIT_ONLY: Self = Self(0);
    #[doc(alias = "VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_ALL")]
    pub const ALL: Self = Self(1);
    #[doc(alias = "VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_NONE")]
    pub const NONE: Self = Self(2);
    #[doc(alias = "VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_32_BIT_ONLY_KHR")]
    #[cfg(feature = "VK_KHR_shader_float_controls")]
    pub const N32_BIT_ONLY_KHR: Self = Self::N32_BIT_ONLY;
    #[doc(alias = "VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_ALL_KHR")]
    #[cfg(feature = "VK_KHR_shader_float_controls")]
    pub const ALL_KHR: Self = Self::ALL;
    #[doc(alias = "VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_NONE_KHR")]
    #[cfg(feature = "VK_KHR_shader_float_controls")]
    pub const NONE_KHR: Self = Self::NONE;
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
    ///The bits of this variant
    #[inline]
    pub const fn bits(&self) -> i32 {
        self.0
    }
    ///Builds a bitmask from the bits of this variant
    #[inline]
    pub const fn from_bits(bits: i32) -> Option<Self> {
        match bits {
            x if x == Self::N32_BIT_ONLY.bits() => Some(Self(x)),
            x if x == Self::ALL.bits() => Some(Self(x)),
            x if x == Self::NONE.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: i32) -> Self {
        Self(bits)
    }
}
# [doc = include_str ! ("../../../doc/vkResetQueryPool.md")]
#[doc(alias = "vkResetQueryPool")]
pub type FNResetQueryPool =
    unsafe extern "system" fn(device: Device, query_pool: QueryPool, first_query: u32, query_count: u32);
# [doc = include_str ! ("../../../doc/vkCreateRenderPass2.md")]
#[doc(alias = "vkCreateRenderPass2")]
pub type FNCreateRenderPass2 = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const RenderPassCreateInfo2,
    p_allocator: *const AllocationCallbacks,
    p_render_pass: *mut RenderPass,
) -> VulkanResultCodes;
# [doc = include_str ! ("../../../doc/vkGetSemaphoreCounterValue.md")]
#[doc(alias = "vkGetSemaphoreCounterValue")]
pub type FNGetSemaphoreCounterValue =
    unsafe extern "system" fn(device: Device, semaphore: Semaphore, p_value: *mut u64) -> VulkanResultCodes;
# [doc = include_str ! ("../../../doc/vkWaitSemaphores.md")]
#[doc(alias = "vkWaitSemaphores")]
pub type FNWaitSemaphores =
    unsafe extern "system" fn(device: Device, p_wait_info: *const SemaphoreWaitInfo, timeout: u64) -> VulkanResultCodes;
# [doc = include_str ! ("../../../doc/vkSignalSemaphore.md")]
#[doc(alias = "vkSignalSemaphore")]
pub type FNSignalSemaphore =
    unsafe extern "system" fn(device: Device, p_signal_info: *const SemaphoreSignalInfo) -> VulkanResultCodes;
# [doc = include_str ! ("../../../doc/vkGetBufferOpaqueCaptureAddress.md")]
#[doc(alias = "vkGetBufferOpaqueCaptureAddress")]
pub type FNGetBufferOpaqueCaptureAddress =
    unsafe extern "system" fn(device: Device, p_info: *const BufferDeviceAddressInfo) -> u64;
# [doc = include_str ! ("../../../doc/vkGetBufferDeviceAddress.md")]
#[doc(alias = "vkGetBufferDeviceAddress")]
pub type FNGetBufferDeviceAddress =
    unsafe extern "system" fn(device: Device, p_info: *const BufferDeviceAddressInfo) -> DeviceAddress;
# [doc = include_str ! ("../../../doc/vkGetDeviceMemoryOpaqueCaptureAddress.md")]
#[doc(alias = "vkGetDeviceMemoryOpaqueCaptureAddress")]
pub type FNGetDeviceMemoryOpaqueCaptureAddress =
    unsafe extern "system" fn(device: Device, p_info: *const DeviceMemoryOpaqueCaptureAddressInfo) -> u64;
# [doc = include_str ! ("../../../doc/vkCmdBeginRenderPass2.md")]
#[doc(alias = "vkCmdBeginRenderPass2")]
pub type FNCmdBeginRenderPass2 = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_render_pass_begin: *const RenderPassBeginInfo,
    p_subpass_begin_info: *const SubpassBeginInfo,
);
# [doc = include_str ! ("../../../doc/vkCmdNextSubpass2.md")]
#[doc(alias = "vkCmdNextSubpass2")]
pub type FNCmdNextSubpass2 = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_subpass_begin_info: *const SubpassBeginInfo,
    p_subpass_end_info: *const SubpassEndInfo,
);
# [doc = include_str ! ("../../../doc/vkCmdEndRenderPass2.md")]
#[doc(alias = "vkCmdEndRenderPass2")]
pub type FNCmdEndRenderPass2 =
    unsafe extern "system" fn(command_buffer: CommandBuffer, p_subpass_end_info: *const SubpassEndInfo);
# [doc = include_str ! ("../../../doc/vkCmdDrawIndirectCount.md")]
#[doc(alias = "vkCmdDrawIndirectCount")]
pub type FNCmdDrawIndirectCount = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    buffer: Buffer,
    offset: DeviceSize,
    count_buffer: Buffer,
    count_buffer_offset: DeviceSize,
    max_draw_count: u32,
    stride: u32,
);
# [doc = include_str ! ("../../../doc/vkCmdDrawIndexedIndirectCount.md")]
#[doc(alias = "vkCmdDrawIndexedIndirectCount")]
pub type FNCmdDrawIndexedIndirectCount = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    buffer: Buffer,
    offset: DeviceSize,
    count_buffer: Buffer,
    count_buffer_offset: DeviceSize,
    max_draw_count: u32,
    stride: u32,
);
