pub use crate::common::vulkan1_2::ConformanceVersion;
use crate::native::{
    vulkan1_0::{
        AccessFlags, AllocationCallbacks, AttachmentDescriptionFlags, AttachmentLoadOp, AttachmentStoreOp,
        BaseInStructure, BaseOutStructure, Bool32, Buffer, CommandBuffer, DependencyFlags, Device, DeviceAddress,
        DeviceMemory, DeviceSize, Format, ImageAspectFlags, ImageCreateFlags, ImageLayout, ImageUsageFlags, ImageView,
        PipelineBindPoint, PipelineStageFlags, QueryPool, RenderPass, RenderPassBeginInfo, RenderPassCreateFlags,
        SampleCountFlagBits, SampleCountFlags, Semaphore, ShaderStageFlags, StructureType, SubpassContents,
        SubpassDescriptionFlags, VulkanResultCodes, LUID_SIZE, MAX_DRIVER_INFO_SIZE, MAX_DRIVER_NAME_SIZE,
    },
    vulkan1_1::{PointClippingBehavior, SubgroupFeatureFlags},
};
use uuid::Uuid;
#[doc(alias = "VkPhysicalDeviceDriverProperties")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceDriverProperties {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "driverID")]
    pub driver_id: DriverId,
    #[doc(alias = "driverName")]
    pub driver_name: [std::ffi::c_char; MAX_DRIVER_NAME_SIZE as usize],
    #[doc(alias = "driverInfo")]
    pub driver_info: [std::ffi::c_char; MAX_DRIVER_INFO_SIZE as usize],
    #[doc(alias = "conformanceVersion")]
    pub conformance_version: ConformanceVersion,
}
impl Default for PhysicalDeviceDriverProperties {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceDriverProperties,
            p_next: unsafe { std::mem::zeroed() },
            driver_id: unsafe { std::mem::zeroed() },
            driver_name: unsafe { std::mem::zeroed() },
            driver_info: unsafe { std::mem::zeroed() },
            conformance_version: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceShaderSubgroupExtendedTypesFeatures")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShaderSubgroupExtendedTypesFeatures {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "shaderSubgroupExtendedTypes")]
    pub shader_subgroup_extended_types: Bool32,
}
impl Default for PhysicalDeviceShaderSubgroupExtendedTypesFeatures {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceShaderSubgroupExtendedTypesFeatures,
            p_next: unsafe { std::mem::zeroed() },
            shader_subgroup_extended_types: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceSamplerFilterMinmaxProperties")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceSamplerFilterMinmaxProperties {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "filterMinmaxSingleComponentFormats")]
    pub filter_minmax_single_component_formats: Bool32,
    #[doc(alias = "filterMinmaxImageComponentMapping")]
    pub filter_minmax_image_component_mapping: Bool32,
}
impl Default for PhysicalDeviceSamplerFilterMinmaxProperties {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceSamplerFilterMinmaxProperties,
            p_next: unsafe { std::mem::zeroed() },
            filter_minmax_single_component_formats: unsafe { std::mem::zeroed() },
            filter_minmax_image_component_mapping: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkSamplerReductionModeCreateInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SamplerReductionModeCreateInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "reductionMode")]
    pub reduction_mode: SamplerReductionMode,
}
impl Default for SamplerReductionModeCreateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::SamplerReductionModeCreateInfo,
            p_next: unsafe { std::mem::zeroed() },
            reduction_mode: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkImageFormatListCreateInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ImageFormatListCreateInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "viewFormatCount")]
    pub view_format_count: u32,
    #[doc(alias = "pViewFormats")]
    pub view_formats: *const Format,
}
impl Default for ImageFormatListCreateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::ImageFormatListCreateInfo,
            p_next: unsafe { std::mem::zeroed() },
            view_format_count: unsafe { std::mem::zeroed() },
            view_formats: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceShaderFloat16Int8Features")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShaderFloat16Int8Features {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "shaderFloat16")]
    pub shader_float16: Bool32,
    #[doc(alias = "shaderInt8")]
    pub shader_int8: Bool32,
}
impl Default for PhysicalDeviceShaderFloat16Int8Features {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceShaderFloat16Int8Features,
            p_next: unsafe { std::mem::zeroed() },
            shader_float16: unsafe { std::mem::zeroed() },
            shader_int8: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceFloatControlsProperties")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceFloatControlsProperties {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "denormBehaviorIndependence")]
    pub denorm_behavior_independence: ShaderFloatControlsIndependence,
    #[doc(alias = "roundingModeIndependence")]
    pub rounding_mode_independence: ShaderFloatControlsIndependence,
    #[doc(alias = "shaderSignedZeroInfNanPreserveFloat16")]
    pub shader_signed_zero_inf_nan_preserve_float16: Bool32,
    #[doc(alias = "shaderSignedZeroInfNanPreserveFloat32")]
    pub shader_signed_zero_inf_nan_preserve_float32: Bool32,
    #[doc(alias = "shaderSignedZeroInfNanPreserveFloat64")]
    pub shader_signed_zero_inf_nan_preserve_float64: Bool32,
    #[doc(alias = "shaderDenormPreserveFloat16")]
    pub shader_denorm_preserve_float16: Bool32,
    #[doc(alias = "shaderDenormPreserveFloat32")]
    pub shader_denorm_preserve_float32: Bool32,
    #[doc(alias = "shaderDenormPreserveFloat64")]
    pub shader_denorm_preserve_float64: Bool32,
    #[doc(alias = "shaderDenormFlushToZeroFloat16")]
    pub shader_denorm_flush_to_zero_float16: Bool32,
    #[doc(alias = "shaderDenormFlushToZeroFloat32")]
    pub shader_denorm_flush_to_zero_float32: Bool32,
    #[doc(alias = "shaderDenormFlushToZeroFloat64")]
    pub shader_denorm_flush_to_zero_float64: Bool32,
    #[doc(alias = "shaderRoundingModeRTEFloat16")]
    pub shader_rounding_mode_rte_float16: Bool32,
    #[doc(alias = "shaderRoundingModeRTEFloat32")]
    pub shader_rounding_mode_rte_float32: Bool32,
    #[doc(alias = "shaderRoundingModeRTEFloat64")]
    pub shader_rounding_mode_rte_float64: Bool32,
    #[doc(alias = "shaderRoundingModeRTZFloat16")]
    pub shader_rounding_mode_rtz_float16: Bool32,
    #[doc(alias = "shaderRoundingModeRTZFloat32")]
    pub shader_rounding_mode_rtz_float32: Bool32,
    #[doc(alias = "shaderRoundingModeRTZFloat64")]
    pub shader_rounding_mode_rtz_float64: Bool32,
}
impl Default for PhysicalDeviceFloatControlsProperties {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceFloatControlsProperties,
            p_next: unsafe { std::mem::zeroed() },
            denorm_behavior_independence: unsafe { std::mem::zeroed() },
            rounding_mode_independence: unsafe { std::mem::zeroed() },
            shader_signed_zero_inf_nan_preserve_float16: unsafe { std::mem::zeroed() },
            shader_signed_zero_inf_nan_preserve_float32: unsafe { std::mem::zeroed() },
            shader_signed_zero_inf_nan_preserve_float64: unsafe { std::mem::zeroed() },
            shader_denorm_preserve_float16: unsafe { std::mem::zeroed() },
            shader_denorm_preserve_float32: unsafe { std::mem::zeroed() },
            shader_denorm_preserve_float64: unsafe { std::mem::zeroed() },
            shader_denorm_flush_to_zero_float16: unsafe { std::mem::zeroed() },
            shader_denorm_flush_to_zero_float32: unsafe { std::mem::zeroed() },
            shader_denorm_flush_to_zero_float64: unsafe { std::mem::zeroed() },
            shader_rounding_mode_rte_float16: unsafe { std::mem::zeroed() },
            shader_rounding_mode_rte_float32: unsafe { std::mem::zeroed() },
            shader_rounding_mode_rte_float64: unsafe { std::mem::zeroed() },
            shader_rounding_mode_rtz_float16: unsafe { std::mem::zeroed() },
            shader_rounding_mode_rtz_float32: unsafe { std::mem::zeroed() },
            shader_rounding_mode_rtz_float64: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceHostQueryResetFeatures")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceHostQueryResetFeatures {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "hostQueryReset")]
    pub host_query_reset: Bool32,
}
impl Default for PhysicalDeviceHostQueryResetFeatures {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceHostQueryResetFeatures,
            p_next: unsafe { std::mem::zeroed() },
            host_query_reset: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceDescriptorIndexingFeatures")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceDescriptorIndexingFeatures {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "shaderInputAttachmentArrayDynamicIndexing")]
    pub shader_input_attachment_array_dynamic_indexing: Bool32,
    #[doc(alias = "shaderUniformTexelBufferArrayDynamicIndexing")]
    pub shader_uniform_texel_buffer_array_dynamic_indexing: Bool32,
    #[doc(alias = "shaderStorageTexelBufferArrayDynamicIndexing")]
    pub shader_storage_texel_buffer_array_dynamic_indexing: Bool32,
    #[doc(alias = "shaderUniformBufferArrayNonUniformIndexing")]
    pub shader_uniform_buffer_array_non_uniform_indexing: Bool32,
    #[doc(alias = "shaderSampledImageArrayNonUniformIndexing")]
    pub shader_sampled_image_array_non_uniform_indexing: Bool32,
    #[doc(alias = "shaderStorageBufferArrayNonUniformIndexing")]
    pub shader_storage_buffer_array_non_uniform_indexing: Bool32,
    #[doc(alias = "shaderStorageImageArrayNonUniformIndexing")]
    pub shader_storage_image_array_non_uniform_indexing: Bool32,
    #[doc(alias = "shaderInputAttachmentArrayNonUniformIndexing")]
    pub shader_input_attachment_array_non_uniform_indexing: Bool32,
    #[doc(alias = "shaderUniformTexelBufferArrayNonUniformIndexing")]
    pub shader_uniform_texel_buffer_array_non_uniform_indexing: Bool32,
    #[doc(alias = "shaderStorageTexelBufferArrayNonUniformIndexing")]
    pub shader_storage_texel_buffer_array_non_uniform_indexing: Bool32,
    #[doc(alias = "descriptorBindingUniformBufferUpdateAfterBind")]
    pub descriptor_binding_uniform_buffer_update_after_bind: Bool32,
    #[doc(alias = "descriptorBindingSampledImageUpdateAfterBind")]
    pub descriptor_binding_sampled_image_update_after_bind: Bool32,
    #[doc(alias = "descriptorBindingStorageImageUpdateAfterBind")]
    pub descriptor_binding_storage_image_update_after_bind: Bool32,
    #[doc(alias = "descriptorBindingStorageBufferUpdateAfterBind")]
    pub descriptor_binding_storage_buffer_update_after_bind: Bool32,
    #[doc(alias = "descriptorBindingUniformTexelBufferUpdateAfterBind")]
    pub descriptor_binding_uniform_texel_buffer_update_after_bind: Bool32,
    #[doc(alias = "descriptorBindingStorageTexelBufferUpdateAfterBind")]
    pub descriptor_binding_storage_texel_buffer_update_after_bind: Bool32,
    #[doc(alias = "descriptorBindingUpdateUnusedWhilePending")]
    pub descriptor_binding_update_unused_while_pending: Bool32,
    #[doc(alias = "descriptorBindingPartiallyBound")]
    pub descriptor_binding_partially_bound: Bool32,
    #[doc(alias = "descriptorBindingVariableDescriptorCount")]
    pub descriptor_binding_variable_descriptor_count: Bool32,
    #[doc(alias = "runtimeDescriptorArray")]
    pub runtime_descriptor_array: Bool32,
}
impl Default for PhysicalDeviceDescriptorIndexingFeatures {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceDescriptorIndexingFeatures,
            p_next: unsafe { std::mem::zeroed() },
            shader_input_attachment_array_dynamic_indexing: unsafe { std::mem::zeroed() },
            shader_uniform_texel_buffer_array_dynamic_indexing: unsafe { std::mem::zeroed() },
            shader_storage_texel_buffer_array_dynamic_indexing: unsafe { std::mem::zeroed() },
            shader_uniform_buffer_array_non_uniform_indexing: unsafe { std::mem::zeroed() },
            shader_sampled_image_array_non_uniform_indexing: unsafe { std::mem::zeroed() },
            shader_storage_buffer_array_non_uniform_indexing: unsafe { std::mem::zeroed() },
            shader_storage_image_array_non_uniform_indexing: unsafe { std::mem::zeroed() },
            shader_input_attachment_array_non_uniform_indexing: unsafe { std::mem::zeroed() },
            shader_uniform_texel_buffer_array_non_uniform_indexing: unsafe { std::mem::zeroed() },
            shader_storage_texel_buffer_array_non_uniform_indexing: unsafe { std::mem::zeroed() },
            descriptor_binding_uniform_buffer_update_after_bind: unsafe { std::mem::zeroed() },
            descriptor_binding_sampled_image_update_after_bind: unsafe { std::mem::zeroed() },
            descriptor_binding_storage_image_update_after_bind: unsafe { std::mem::zeroed() },
            descriptor_binding_storage_buffer_update_after_bind: unsafe { std::mem::zeroed() },
            descriptor_binding_uniform_texel_buffer_update_after_bind: unsafe { std::mem::zeroed() },
            descriptor_binding_storage_texel_buffer_update_after_bind: unsafe { std::mem::zeroed() },
            descriptor_binding_update_unused_while_pending: unsafe { std::mem::zeroed() },
            descriptor_binding_partially_bound: unsafe { std::mem::zeroed() },
            descriptor_binding_variable_descriptor_count: unsafe { std::mem::zeroed() },
            runtime_descriptor_array: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceDescriptorIndexingProperties")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceDescriptorIndexingProperties {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "maxUpdateAfterBindDescriptorsInAllPools")]
    pub max_update_after_bind_descriptors_in_all_pools: u32,
    #[doc(alias = "shaderUniformBufferArrayNonUniformIndexingNative")]
    pub shader_uniform_buffer_array_non_uniform_indexing_native: Bool32,
    #[doc(alias = "shaderSampledImageArrayNonUniformIndexingNative")]
    pub shader_sampled_image_array_non_uniform_indexing_native: Bool32,
    #[doc(alias = "shaderStorageBufferArrayNonUniformIndexingNative")]
    pub shader_storage_buffer_array_non_uniform_indexing_native: Bool32,
    #[doc(alias = "shaderStorageImageArrayNonUniformIndexingNative")]
    pub shader_storage_image_array_non_uniform_indexing_native: Bool32,
    #[doc(alias = "shaderInputAttachmentArrayNonUniformIndexingNative")]
    pub shader_input_attachment_array_non_uniform_indexing_native: Bool32,
    #[doc(alias = "robustBufferAccessUpdateAfterBind")]
    pub robust_buffer_access_update_after_bind: Bool32,
    #[doc(alias = "quadDivergentImplicitLod")]
    pub quad_divergent_implicit_lod: Bool32,
    #[doc(alias = "maxPerStageDescriptorUpdateAfterBindSamplers")]
    pub max_per_stage_descriptor_update_after_bind_samplers: u32,
    #[doc(alias = "maxPerStageDescriptorUpdateAfterBindUniformBuffers")]
    pub max_per_stage_descriptor_update_after_bind_uniform_buffers: u32,
    #[doc(alias = "maxPerStageDescriptorUpdateAfterBindStorageBuffers")]
    pub max_per_stage_descriptor_update_after_bind_storage_buffers: u32,
    #[doc(alias = "maxPerStageDescriptorUpdateAfterBindSampledImages")]
    pub max_per_stage_descriptor_update_after_bind_sampled_images: u32,
    #[doc(alias = "maxPerStageDescriptorUpdateAfterBindStorageImages")]
    pub max_per_stage_descriptor_update_after_bind_storage_images: u32,
    #[doc(alias = "maxPerStageDescriptorUpdateAfterBindInputAttachments")]
    pub max_per_stage_descriptor_update_after_bind_input_attachments: u32,
    #[doc(alias = "maxPerStageUpdateAfterBindResources")]
    pub max_per_stage_update_after_bind_resources: u32,
    #[doc(alias = "maxDescriptorSetUpdateAfterBindSamplers")]
    pub max_descriptor_set_update_after_bind_samplers: u32,
    #[doc(alias = "maxDescriptorSetUpdateAfterBindUniformBuffers")]
    pub max_descriptor_set_update_after_bind_uniform_buffers: u32,
    #[doc(alias = "maxDescriptorSetUpdateAfterBindUniformBuffersDynamic")]
    pub max_descriptor_set_update_after_bind_uniform_buffers_dynamic: u32,
    #[doc(alias = "maxDescriptorSetUpdateAfterBindStorageBuffers")]
    pub max_descriptor_set_update_after_bind_storage_buffers: u32,
    #[doc(alias = "maxDescriptorSetUpdateAfterBindStorageBuffersDynamic")]
    pub max_descriptor_set_update_after_bind_storage_buffers_dynamic: u32,
    #[doc(alias = "maxDescriptorSetUpdateAfterBindSampledImages")]
    pub max_descriptor_set_update_after_bind_sampled_images: u32,
    #[doc(alias = "maxDescriptorSetUpdateAfterBindStorageImages")]
    pub max_descriptor_set_update_after_bind_storage_images: u32,
    #[doc(alias = "maxDescriptorSetUpdateAfterBindInputAttachments")]
    pub max_descriptor_set_update_after_bind_input_attachments: u32,
}
impl Default for PhysicalDeviceDescriptorIndexingProperties {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceDescriptorIndexingProperties,
            p_next: unsafe { std::mem::zeroed() },
            max_update_after_bind_descriptors_in_all_pools: unsafe { std::mem::zeroed() },
            shader_uniform_buffer_array_non_uniform_indexing_native: unsafe { std::mem::zeroed() },
            shader_sampled_image_array_non_uniform_indexing_native: unsafe { std::mem::zeroed() },
            shader_storage_buffer_array_non_uniform_indexing_native: unsafe { std::mem::zeroed() },
            shader_storage_image_array_non_uniform_indexing_native: unsafe { std::mem::zeroed() },
            shader_input_attachment_array_non_uniform_indexing_native: unsafe { std::mem::zeroed() },
            robust_buffer_access_update_after_bind: unsafe { std::mem::zeroed() },
            quad_divergent_implicit_lod: unsafe { std::mem::zeroed() },
            max_per_stage_descriptor_update_after_bind_samplers: unsafe { std::mem::zeroed() },
            max_per_stage_descriptor_update_after_bind_uniform_buffers: unsafe { std::mem::zeroed() },
            max_per_stage_descriptor_update_after_bind_storage_buffers: unsafe { std::mem::zeroed() },
            max_per_stage_descriptor_update_after_bind_sampled_images: unsafe { std::mem::zeroed() },
            max_per_stage_descriptor_update_after_bind_storage_images: unsafe { std::mem::zeroed() },
            max_per_stage_descriptor_update_after_bind_input_attachments: unsafe { std::mem::zeroed() },
            max_per_stage_update_after_bind_resources: unsafe { std::mem::zeroed() },
            max_descriptor_set_update_after_bind_samplers: unsafe { std::mem::zeroed() },
            max_descriptor_set_update_after_bind_uniform_buffers: unsafe { std::mem::zeroed() },
            max_descriptor_set_update_after_bind_uniform_buffers_dynamic: unsafe { std::mem::zeroed() },
            max_descriptor_set_update_after_bind_storage_buffers: unsafe { std::mem::zeroed() },
            max_descriptor_set_update_after_bind_storage_buffers_dynamic: unsafe { std::mem::zeroed() },
            max_descriptor_set_update_after_bind_sampled_images: unsafe { std::mem::zeroed() },
            max_descriptor_set_update_after_bind_storage_images: unsafe { std::mem::zeroed() },
            max_descriptor_set_update_after_bind_input_attachments: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkDescriptorSetLayoutBindingFlagsCreateInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DescriptorSetLayoutBindingFlagsCreateInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "bindingCount")]
    pub binding_count: u32,
    #[doc(alias = "pBindingFlags")]
    pub binding_flags: *const DescriptorBindingFlags,
}
impl Default for DescriptorSetLayoutBindingFlagsCreateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::DescriptorSetLayoutBindingFlagsCreateInfo,
            p_next: unsafe { std::mem::zeroed() },
            binding_count: unsafe { std::mem::zeroed() },
            binding_flags: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkDescriptorSetVariableDescriptorCountAllocateInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DescriptorSetVariableDescriptorCountAllocateInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "descriptorSetCount")]
    pub descriptor_set_count: u32,
    #[doc(alias = "pDescriptorCounts")]
    pub descriptor_counts: *const u32,
}
impl Default for DescriptorSetVariableDescriptorCountAllocateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::DescriptorSetVariableDescriptorCountAllocateInfo,
            p_next: unsafe { std::mem::zeroed() },
            descriptor_set_count: unsafe { std::mem::zeroed() },
            descriptor_counts: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkDescriptorSetVariableDescriptorCountLayoutSupport")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DescriptorSetVariableDescriptorCountLayoutSupport {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "maxVariableDescriptorCount")]
    pub max_variable_descriptor_count: u32,
}
impl Default for DescriptorSetVariableDescriptorCountLayoutSupport {
    fn default() -> Self {
        Self {
            s_type: StructureType::DescriptorSetVariableDescriptorCountLayoutSupport,
            p_next: unsafe { std::mem::zeroed() },
            max_variable_descriptor_count: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkAttachmentDescription2")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct AttachmentDescription2 {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub flags: AttachmentDescriptionFlags,
    pub format: Format,
    pub samples: SampleCountFlagBits,
    #[doc(alias = "loadOp")]
    pub load_op: AttachmentLoadOp,
    #[doc(alias = "storeOp")]
    pub store_op: AttachmentStoreOp,
    #[doc(alias = "stencilLoadOp")]
    pub stencil_load_op: AttachmentLoadOp,
    #[doc(alias = "stencilStoreOp")]
    pub stencil_store_op: AttachmentStoreOp,
    #[doc(alias = "initialLayout")]
    pub initial_layout: ImageLayout,
    #[doc(alias = "finalLayout")]
    pub final_layout: ImageLayout,
}
impl Default for AttachmentDescription2 {
    fn default() -> Self {
        Self {
            s_type: StructureType::AttachmentDescription2,
            p_next: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
            format: unsafe { std::mem::zeroed() },
            samples: unsafe { std::mem::zeroed() },
            load_op: unsafe { std::mem::zeroed() },
            store_op: unsafe { std::mem::zeroed() },
            stencil_load_op: unsafe { std::mem::zeroed() },
            stencil_store_op: unsafe { std::mem::zeroed() },
            initial_layout: unsafe { std::mem::zeroed() },
            final_layout: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkAttachmentReference2")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct AttachmentReference2 {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub attachment: u32,
    pub layout: ImageLayout,
    #[doc(alias = "aspectMask")]
    pub aspect_mask: ImageAspectFlags,
}
impl Default for AttachmentReference2 {
    fn default() -> Self {
        Self {
            s_type: StructureType::AttachmentReference2,
            p_next: unsafe { std::mem::zeroed() },
            attachment: unsafe { std::mem::zeroed() },
            layout: unsafe { std::mem::zeroed() },
            aspect_mask: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkSubpassDescription2")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SubpassDescription2 {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub flags: SubpassDescriptionFlags,
    #[doc(alias = "pipelineBindPoint")]
    pub pipeline_bind_point: PipelineBindPoint,
    #[doc(alias = "viewMask")]
    pub view_mask: u32,
    #[doc(alias = "inputAttachmentCount")]
    pub input_attachment_count: u32,
    #[doc(alias = "pInputAttachments")]
    pub input_attachments: *const AttachmentReference2,
    #[doc(alias = "colorAttachmentCount")]
    pub color_attachment_count: u32,
    #[doc(alias = "pColorAttachments")]
    pub color_attachments: *const AttachmentReference2,
    #[doc(alias = "pResolveAttachments")]
    pub resolve_attachments: *const AttachmentReference2,
    #[doc(alias = "pDepthStencilAttachment")]
    pub depth_stencil_attachment: *const AttachmentReference2,
    #[doc(alias = "preserveAttachmentCount")]
    pub preserve_attachment_count: u32,
    #[doc(alias = "pPreserveAttachments")]
    pub preserve_attachments: *const u32,
}
impl Default for SubpassDescription2 {
    fn default() -> Self {
        Self {
            s_type: StructureType::SubpassDescription2,
            p_next: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
            pipeline_bind_point: unsafe { std::mem::zeroed() },
            view_mask: unsafe { std::mem::zeroed() },
            input_attachment_count: unsafe { std::mem::zeroed() },
            input_attachments: unsafe { std::mem::zeroed() },
            color_attachment_count: unsafe { std::mem::zeroed() },
            color_attachments: unsafe { std::mem::zeroed() },
            resolve_attachments: unsafe { std::mem::zeroed() },
            depth_stencil_attachment: unsafe { std::mem::zeroed() },
            preserve_attachment_count: unsafe { std::mem::zeroed() },
            preserve_attachments: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkSubpassDependency2")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SubpassDependency2 {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "srcSubpass")]
    pub src_subpass: u32,
    #[doc(alias = "dstSubpass")]
    pub dst_subpass: u32,
    #[doc(alias = "srcStageMask")]
    pub src_stage_mask: PipelineStageFlags,
    #[doc(alias = "dstStageMask")]
    pub dst_stage_mask: PipelineStageFlags,
    #[doc(alias = "srcAccessMask")]
    pub src_access_mask: AccessFlags,
    #[doc(alias = "dstAccessMask")]
    pub dst_access_mask: AccessFlags,
    #[doc(alias = "dependencyFlags")]
    pub dependency_flags: DependencyFlags,
    #[doc(alias = "viewOffset")]
    pub view_offset: i32,
}
impl Default for SubpassDependency2 {
    fn default() -> Self {
        Self {
            s_type: StructureType::SubpassDependency2,
            p_next: unsafe { std::mem::zeroed() },
            src_subpass: unsafe { std::mem::zeroed() },
            dst_subpass: unsafe { std::mem::zeroed() },
            src_stage_mask: unsafe { std::mem::zeroed() },
            dst_stage_mask: unsafe { std::mem::zeroed() },
            src_access_mask: unsafe { std::mem::zeroed() },
            dst_access_mask: unsafe { std::mem::zeroed() },
            dependency_flags: unsafe { std::mem::zeroed() },
            view_offset: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkRenderPassCreateInfo2")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct RenderPassCreateInfo2 {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub flags: RenderPassCreateFlags,
    #[doc(alias = "attachmentCount")]
    pub attachment_count: u32,
    #[doc(alias = "pAttachments")]
    pub attachments: *const AttachmentDescription2,
    #[doc(alias = "subpassCount")]
    pub subpass_count: u32,
    #[doc(alias = "pSubpasses")]
    pub subpasses: *const SubpassDescription2,
    #[doc(alias = "dependencyCount")]
    pub dependency_count: u32,
    #[doc(alias = "pDependencies")]
    pub dependencies: *const SubpassDependency2,
    #[doc(alias = "correlatedViewMaskCount")]
    pub correlated_view_mask_count: u32,
    #[doc(alias = "pCorrelatedViewMasks")]
    pub correlated_view_masks: *const u32,
}
impl Default for RenderPassCreateInfo2 {
    fn default() -> Self {
        Self {
            s_type: StructureType::RenderPassCreateInfo2,
            p_next: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
            attachment_count: unsafe { std::mem::zeroed() },
            attachments: unsafe { std::mem::zeroed() },
            subpass_count: unsafe { std::mem::zeroed() },
            subpasses: unsafe { std::mem::zeroed() },
            dependency_count: unsafe { std::mem::zeroed() },
            dependencies: unsafe { std::mem::zeroed() },
            correlated_view_mask_count: unsafe { std::mem::zeroed() },
            correlated_view_masks: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkSubpassBeginInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SubpassBeginInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub contents: SubpassContents,
}
impl Default for SubpassBeginInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::SubpassBeginInfo,
            p_next: unsafe { std::mem::zeroed() },
            contents: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkSubpassEndInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SubpassEndInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
}
impl Default for SubpassEndInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::SubpassEndInfo,
            p_next: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceTimelineSemaphoreFeatures")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceTimelineSemaphoreFeatures {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "timelineSemaphore")]
    pub timeline_semaphore: Bool32,
}
impl Default for PhysicalDeviceTimelineSemaphoreFeatures {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceTimelineSemaphoreFeatures,
            p_next: unsafe { std::mem::zeroed() },
            timeline_semaphore: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceTimelineSemaphoreProperties")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceTimelineSemaphoreProperties {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "maxTimelineSemaphoreValueDifference")]
    pub max_timeline_semaphore_value_difference: u64,
}
impl Default for PhysicalDeviceTimelineSemaphoreProperties {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceTimelineSemaphoreProperties,
            p_next: unsafe { std::mem::zeroed() },
            max_timeline_semaphore_value_difference: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkSemaphoreTypeCreateInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SemaphoreTypeCreateInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "semaphoreType")]
    pub semaphore_type: SemaphoreType,
    #[doc(alias = "initialValue")]
    pub initial_value: u64,
}
impl Default for SemaphoreTypeCreateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::SemaphoreTypeCreateInfo,
            p_next: unsafe { std::mem::zeroed() },
            semaphore_type: unsafe { std::mem::zeroed() },
            initial_value: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkTimelineSemaphoreSubmitInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct TimelineSemaphoreSubmitInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "waitSemaphoreValueCount")]
    pub wait_semaphore_value_count: u32,
    #[doc(alias = "pWaitSemaphoreValues")]
    pub wait_semaphore_values: *const u64,
    #[doc(alias = "signalSemaphoreValueCount")]
    pub signal_semaphore_value_count: u32,
    #[doc(alias = "pSignalSemaphoreValues")]
    pub signal_semaphore_values: *const u64,
}
impl Default for TimelineSemaphoreSubmitInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::TimelineSemaphoreSubmitInfo,
            p_next: unsafe { std::mem::zeroed() },
            wait_semaphore_value_count: unsafe { std::mem::zeroed() },
            wait_semaphore_values: unsafe { std::mem::zeroed() },
            signal_semaphore_value_count: unsafe { std::mem::zeroed() },
            signal_semaphore_values: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkSemaphoreWaitInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SemaphoreWaitInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub flags: SemaphoreWaitFlags,
    #[doc(alias = "semaphoreCount")]
    pub semaphore_count: u32,
    #[doc(alias = "pSemaphores")]
    pub semaphores: *const Semaphore,
    #[doc(alias = "pValues")]
    pub values: *const u64,
}
impl Default for SemaphoreWaitInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::SemaphoreWaitInfo,
            p_next: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
            semaphore_count: unsafe { std::mem::zeroed() },
            semaphores: unsafe { std::mem::zeroed() },
            values: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkSemaphoreSignalInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SemaphoreSignalInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub semaphore: Semaphore,
    pub value: u64,
}
impl Default for SemaphoreSignalInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::SemaphoreSignalInfo,
            p_next: unsafe { std::mem::zeroed() },
            semaphore: unsafe { std::mem::zeroed() },
            value: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDevice8BitStorageFeatures")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDevice8BitStorageFeatures {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "storageBuffer8BitAccess")]
    pub storage_buffer8_bit_access: Bool32,
    #[doc(alias = "uniformAndStorageBuffer8BitAccess")]
    pub uniform_and_storage_buffer8_bit_access: Bool32,
    #[doc(alias = "storagePushConstant8")]
    pub storage_push_constant8: Bool32,
}
impl Default for PhysicalDevice8BitStorageFeatures {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDevice8bitStorageFeatures,
            p_next: unsafe { std::mem::zeroed() },
            storage_buffer8_bit_access: unsafe { std::mem::zeroed() },
            uniform_and_storage_buffer8_bit_access: unsafe { std::mem::zeroed() },
            storage_push_constant8: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceVulkanMemoryModelFeatures")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceVulkanMemoryModelFeatures {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "vulkanMemoryModel")]
    pub vulkan_memory_model: Bool32,
    #[doc(alias = "vulkanMemoryModelDeviceScope")]
    pub vulkan_memory_model_device_scope: Bool32,
    #[doc(alias = "vulkanMemoryModelAvailabilityVisibilityChains")]
    pub vulkan_memory_model_availability_visibility_chains: Bool32,
}
impl Default for PhysicalDeviceVulkanMemoryModelFeatures {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceVulkanMemoryModelFeatures,
            p_next: unsafe { std::mem::zeroed() },
            vulkan_memory_model: unsafe { std::mem::zeroed() },
            vulkan_memory_model_device_scope: unsafe { std::mem::zeroed() },
            vulkan_memory_model_availability_visibility_chains: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceShaderAtomicInt64Features")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShaderAtomicInt64Features {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "shaderBufferInt64Atomics")]
    pub shader_buffer_int64_atomics: Bool32,
    #[doc(alias = "shaderSharedInt64Atomics")]
    pub shader_shared_int64_atomics: Bool32,
}
impl Default for PhysicalDeviceShaderAtomicInt64Features {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceShaderAtomicInt64Features,
            p_next: unsafe { std::mem::zeroed() },
            shader_buffer_int64_atomics: unsafe { std::mem::zeroed() },
            shader_shared_int64_atomics: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceDepthStencilResolveProperties")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceDepthStencilResolveProperties {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "supportedDepthResolveModes")]
    pub supported_depth_resolve_modes: ResolveModeFlags,
    #[doc(alias = "supportedStencilResolveModes")]
    pub supported_stencil_resolve_modes: ResolveModeFlags,
    #[doc(alias = "independentResolveNone")]
    pub independent_resolve_none: Bool32,
    #[doc(alias = "independentResolve")]
    pub independent_resolve: Bool32,
}
impl Default for PhysicalDeviceDepthStencilResolveProperties {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceDepthStencilResolveProperties,
            p_next: unsafe { std::mem::zeroed() },
            supported_depth_resolve_modes: unsafe { std::mem::zeroed() },
            supported_stencil_resolve_modes: unsafe { std::mem::zeroed() },
            independent_resolve_none: unsafe { std::mem::zeroed() },
            independent_resolve: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkSubpassDescriptionDepthStencilResolve")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SubpassDescriptionDepthStencilResolve {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "depthResolveMode")]
    pub depth_resolve_mode: ResolveModeFlagBits,
    #[doc(alias = "stencilResolveMode")]
    pub stencil_resolve_mode: ResolveModeFlagBits,
    #[doc(alias = "pDepthStencilResolveAttachment")]
    pub depth_stencil_resolve_attachment: *const AttachmentReference2,
}
impl Default for SubpassDescriptionDepthStencilResolve {
    fn default() -> Self {
        Self {
            s_type: StructureType::SubpassDescriptionDepthStencilResolve,
            p_next: unsafe { std::mem::zeroed() },
            depth_resolve_mode: unsafe { std::mem::zeroed() },
            stencil_resolve_mode: unsafe { std::mem::zeroed() },
            depth_stencil_resolve_attachment: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkImageStencilUsageCreateInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ImageStencilUsageCreateInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "stencilUsage")]
    pub stencil_usage: ImageUsageFlags,
}
impl Default for ImageStencilUsageCreateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::ImageStencilUsageCreateInfo,
            p_next: unsafe { std::mem::zeroed() },
            stencil_usage: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceScalarBlockLayoutFeatures")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceScalarBlockLayoutFeatures {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "scalarBlockLayout")]
    pub scalar_block_layout: Bool32,
}
impl Default for PhysicalDeviceScalarBlockLayoutFeatures {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceScalarBlockLayoutFeatures,
            p_next: unsafe { std::mem::zeroed() },
            scalar_block_layout: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceUniformBufferStandardLayoutFeatures")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceUniformBufferStandardLayoutFeatures {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "uniformBufferStandardLayout")]
    pub uniform_buffer_standard_layout: Bool32,
}
impl Default for PhysicalDeviceUniformBufferStandardLayoutFeatures {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceUniformBufferStandardLayoutFeatures,
            p_next: unsafe { std::mem::zeroed() },
            uniform_buffer_standard_layout: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceBufferDeviceAddressFeatures")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceBufferDeviceAddressFeatures {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "bufferDeviceAddress")]
    pub buffer_device_address: Bool32,
    #[doc(alias = "bufferDeviceAddressCaptureReplay")]
    pub buffer_device_address_capture_replay: Bool32,
    #[doc(alias = "bufferDeviceAddressMultiDevice")]
    pub buffer_device_address_multi_device: Bool32,
}
impl Default for PhysicalDeviceBufferDeviceAddressFeatures {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceBufferDeviceAddressFeatures,
            p_next: unsafe { std::mem::zeroed() },
            buffer_device_address: unsafe { std::mem::zeroed() },
            buffer_device_address_capture_replay: unsafe { std::mem::zeroed() },
            buffer_device_address_multi_device: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkBufferDeviceAddressInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct BufferDeviceAddressInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub buffer: Buffer,
}
impl Default for BufferDeviceAddressInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::BufferDeviceAddressInfo,
            p_next: unsafe { std::mem::zeroed() },
            buffer: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkBufferOpaqueCaptureAddressCreateInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct BufferOpaqueCaptureAddressCreateInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "opaqueCaptureAddress")]
    pub opaque_capture_address: u64,
}
impl Default for BufferOpaqueCaptureAddressCreateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::BufferOpaqueCaptureAddressCreateInfo,
            p_next: unsafe { std::mem::zeroed() },
            opaque_capture_address: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceImagelessFramebufferFeatures")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceImagelessFramebufferFeatures {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "imagelessFramebuffer")]
    pub imageless_framebuffer: Bool32,
}
impl Default for PhysicalDeviceImagelessFramebufferFeatures {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceImagelessFramebufferFeatures,
            p_next: unsafe { std::mem::zeroed() },
            imageless_framebuffer: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkFramebufferAttachmentsCreateInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct FramebufferAttachmentsCreateInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "attachmentImageInfoCount")]
    pub attachment_image_info_count: u32,
    #[doc(alias = "pAttachmentImageInfos")]
    pub attachment_image_infos: *const FramebufferAttachmentImageInfo,
}
impl Default for FramebufferAttachmentsCreateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::FramebufferAttachmentsCreateInfo,
            p_next: unsafe { std::mem::zeroed() },
            attachment_image_info_count: unsafe { std::mem::zeroed() },
            attachment_image_infos: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkFramebufferAttachmentImageInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct FramebufferAttachmentImageInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub flags: ImageCreateFlags,
    pub usage: ImageUsageFlags,
    pub width: u32,
    pub height: u32,
    #[doc(alias = "layerCount")]
    pub layer_count: u32,
    #[doc(alias = "viewFormatCount")]
    pub view_format_count: u32,
    #[doc(alias = "pViewFormats")]
    pub view_formats: *const Format,
}
impl Default for FramebufferAttachmentImageInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::FramebufferAttachmentImageInfo,
            p_next: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
            usage: unsafe { std::mem::zeroed() },
            width: unsafe { std::mem::zeroed() },
            height: unsafe { std::mem::zeroed() },
            layer_count: unsafe { std::mem::zeroed() },
            view_format_count: unsafe { std::mem::zeroed() },
            view_formats: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkRenderPassAttachmentBeginInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct RenderPassAttachmentBeginInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "attachmentCount")]
    pub attachment_count: u32,
    #[doc(alias = "pAttachments")]
    pub attachments: *const ImageView,
}
impl Default for RenderPassAttachmentBeginInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::RenderPassAttachmentBeginInfo,
            p_next: unsafe { std::mem::zeroed() },
            attachment_count: unsafe { std::mem::zeroed() },
            attachments: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceSeparateDepthStencilLayoutsFeatures")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceSeparateDepthStencilLayoutsFeatures {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "separateDepthStencilLayouts")]
    pub separate_depth_stencil_layouts: Bool32,
}
impl Default for PhysicalDeviceSeparateDepthStencilLayoutsFeatures {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceSeparateDepthStencilLayoutsFeatures,
            p_next: unsafe { std::mem::zeroed() },
            separate_depth_stencil_layouts: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkAttachmentReferenceStencilLayout")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct AttachmentReferenceStencilLayout {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "stencilLayout")]
    pub stencil_layout: ImageLayout,
}
impl Default for AttachmentReferenceStencilLayout {
    fn default() -> Self {
        Self {
            s_type: StructureType::AttachmentReferenceStencilLayout,
            p_next: unsafe { std::mem::zeroed() },
            stencil_layout: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkAttachmentDescriptionStencilLayout")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct AttachmentDescriptionStencilLayout {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "stencilInitialLayout")]
    pub stencil_initial_layout: ImageLayout,
    #[doc(alias = "stencilFinalLayout")]
    pub stencil_final_layout: ImageLayout,
}
impl Default for AttachmentDescriptionStencilLayout {
    fn default() -> Self {
        Self {
            s_type: StructureType::AttachmentDescriptionStencilLayout,
            p_next: unsafe { std::mem::zeroed() },
            stencil_initial_layout: unsafe { std::mem::zeroed() },
            stencil_final_layout: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkMemoryOpaqueCaptureAddressAllocateInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct MemoryOpaqueCaptureAddressAllocateInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "opaqueCaptureAddress")]
    pub opaque_capture_address: u64,
}
impl Default for MemoryOpaqueCaptureAddressAllocateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::MemoryOpaqueCaptureAddressAllocateInfo,
            p_next: unsafe { std::mem::zeroed() },
            opaque_capture_address: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkDeviceMemoryOpaqueCaptureAddressInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DeviceMemoryOpaqueCaptureAddressInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub memory: DeviceMemory,
}
impl Default for DeviceMemoryOpaqueCaptureAddressInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::DeviceMemoryOpaqueCaptureAddressInfo,
            p_next: unsafe { std::mem::zeroed() },
            memory: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceVulkan11Features")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceVulkan11Features {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "storageBuffer16BitAccess")]
    pub storage_buffer16_bit_access: Bool32,
    #[doc(alias = "uniformAndStorageBuffer16BitAccess")]
    pub uniform_and_storage_buffer16_bit_access: Bool32,
    #[doc(alias = "storagePushConstant16")]
    pub storage_push_constant16: Bool32,
    #[doc(alias = "storageInputOutput16")]
    pub storage_input_output16: Bool32,
    pub multiview: Bool32,
    #[doc(alias = "multiviewGeometryShader")]
    pub multiview_geometry_shader: Bool32,
    #[doc(alias = "multiviewTessellationShader")]
    pub multiview_tessellation_shader: Bool32,
    #[doc(alias = "variablePointersStorageBuffer")]
    pub variable_pointers_storage_buffer: Bool32,
    #[doc(alias = "variablePointers")]
    pub variable_pointers: Bool32,
    #[doc(alias = "protectedMemory")]
    pub protected_memory: Bool32,
    #[doc(alias = "samplerYcbcrConversion")]
    pub sampler_ycbcr_conversion: Bool32,
    #[doc(alias = "shaderDrawParameters")]
    pub shader_draw_parameters: Bool32,
}
impl Default for PhysicalDeviceVulkan11Features {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceVulkan11Features,
            p_next: unsafe { std::mem::zeroed() },
            storage_buffer16_bit_access: unsafe { std::mem::zeroed() },
            uniform_and_storage_buffer16_bit_access: unsafe { std::mem::zeroed() },
            storage_push_constant16: unsafe { std::mem::zeroed() },
            storage_input_output16: unsafe { std::mem::zeroed() },
            multiview: unsafe { std::mem::zeroed() },
            multiview_geometry_shader: unsafe { std::mem::zeroed() },
            multiview_tessellation_shader: unsafe { std::mem::zeroed() },
            variable_pointers_storage_buffer: unsafe { std::mem::zeroed() },
            variable_pointers: unsafe { std::mem::zeroed() },
            protected_memory: unsafe { std::mem::zeroed() },
            sampler_ycbcr_conversion: unsafe { std::mem::zeroed() },
            shader_draw_parameters: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceVulkan11Properties")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceVulkan11Properties {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "deviceUUID")]
    pub device_uuid: Uuid,
    #[doc(alias = "driverUUID")]
    pub driver_uuid: Uuid,
    #[doc(alias = "deviceLUID")]
    pub device_luid: [u8; LUID_SIZE as usize],
    #[doc(alias = "deviceNodeMask")]
    pub device_node_mask: u32,
    #[doc(alias = "deviceLUIDValid")]
    pub device_luid_valid: Bool32,
    #[doc(alias = "subgroupSize")]
    pub subgroup_size: u32,
    #[doc(alias = "subgroupSupportedStages")]
    pub subgroup_supported_stages: ShaderStageFlags,
    #[doc(alias = "subgroupSupportedOperations")]
    pub subgroup_supported_operations: SubgroupFeatureFlags,
    #[doc(alias = "subgroupQuadOperationsInAllStages")]
    pub subgroup_quad_operations_in_all_stages: Bool32,
    #[doc(alias = "pointClippingBehavior")]
    pub point_clipping_behavior: PointClippingBehavior,
    #[doc(alias = "maxMultiviewViewCount")]
    pub max_multiview_view_count: u32,
    #[doc(alias = "maxMultiviewInstanceIndex")]
    pub max_multiview_instance_index: u32,
    #[doc(alias = "protectedNoFault")]
    pub protected_no_fault: Bool32,
    #[doc(alias = "maxPerSetDescriptors")]
    pub max_per_set_descriptors: u32,
    #[doc(alias = "maxMemoryAllocationSize")]
    pub max_memory_allocation_size: DeviceSize,
}
impl Default for PhysicalDeviceVulkan11Properties {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceVulkan11Properties,
            p_next: unsafe { std::mem::zeroed() },
            device_uuid: unsafe { std::mem::zeroed() },
            driver_uuid: unsafe { std::mem::zeroed() },
            device_luid: unsafe { std::mem::zeroed() },
            device_node_mask: unsafe { std::mem::zeroed() },
            device_luid_valid: unsafe { std::mem::zeroed() },
            subgroup_size: unsafe { std::mem::zeroed() },
            subgroup_supported_stages: unsafe { std::mem::zeroed() },
            subgroup_supported_operations: unsafe { std::mem::zeroed() },
            subgroup_quad_operations_in_all_stages: unsafe { std::mem::zeroed() },
            point_clipping_behavior: unsafe { std::mem::zeroed() },
            max_multiview_view_count: unsafe { std::mem::zeroed() },
            max_multiview_instance_index: unsafe { std::mem::zeroed() },
            protected_no_fault: unsafe { std::mem::zeroed() },
            max_per_set_descriptors: unsafe { std::mem::zeroed() },
            max_memory_allocation_size: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceVulkan12Features")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceVulkan12Features {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "samplerMirrorClampToEdge")]
    pub sampler_mirror_clamp_to_edge: Bool32,
    #[doc(alias = "drawIndirectCount")]
    pub draw_indirect_count: Bool32,
    #[doc(alias = "storageBuffer8BitAccess")]
    pub storage_buffer8_bit_access: Bool32,
    #[doc(alias = "uniformAndStorageBuffer8BitAccess")]
    pub uniform_and_storage_buffer8_bit_access: Bool32,
    #[doc(alias = "storagePushConstant8")]
    pub storage_push_constant8: Bool32,
    #[doc(alias = "shaderBufferInt64Atomics")]
    pub shader_buffer_int64_atomics: Bool32,
    #[doc(alias = "shaderSharedInt64Atomics")]
    pub shader_shared_int64_atomics: Bool32,
    #[doc(alias = "shaderFloat16")]
    pub shader_float16: Bool32,
    #[doc(alias = "shaderInt8")]
    pub shader_int8: Bool32,
    #[doc(alias = "descriptorIndexing")]
    pub descriptor_indexing: Bool32,
    #[doc(alias = "shaderInputAttachmentArrayDynamicIndexing")]
    pub shader_input_attachment_array_dynamic_indexing: Bool32,
    #[doc(alias = "shaderUniformTexelBufferArrayDynamicIndexing")]
    pub shader_uniform_texel_buffer_array_dynamic_indexing: Bool32,
    #[doc(alias = "shaderStorageTexelBufferArrayDynamicIndexing")]
    pub shader_storage_texel_buffer_array_dynamic_indexing: Bool32,
    #[doc(alias = "shaderUniformBufferArrayNonUniformIndexing")]
    pub shader_uniform_buffer_array_non_uniform_indexing: Bool32,
    #[doc(alias = "shaderSampledImageArrayNonUniformIndexing")]
    pub shader_sampled_image_array_non_uniform_indexing: Bool32,
    #[doc(alias = "shaderStorageBufferArrayNonUniformIndexing")]
    pub shader_storage_buffer_array_non_uniform_indexing: Bool32,
    #[doc(alias = "shaderStorageImageArrayNonUniformIndexing")]
    pub shader_storage_image_array_non_uniform_indexing: Bool32,
    #[doc(alias = "shaderInputAttachmentArrayNonUniformIndexing")]
    pub shader_input_attachment_array_non_uniform_indexing: Bool32,
    #[doc(alias = "shaderUniformTexelBufferArrayNonUniformIndexing")]
    pub shader_uniform_texel_buffer_array_non_uniform_indexing: Bool32,
    #[doc(alias = "shaderStorageTexelBufferArrayNonUniformIndexing")]
    pub shader_storage_texel_buffer_array_non_uniform_indexing: Bool32,
    #[doc(alias = "descriptorBindingUniformBufferUpdateAfterBind")]
    pub descriptor_binding_uniform_buffer_update_after_bind: Bool32,
    #[doc(alias = "descriptorBindingSampledImageUpdateAfterBind")]
    pub descriptor_binding_sampled_image_update_after_bind: Bool32,
    #[doc(alias = "descriptorBindingStorageImageUpdateAfterBind")]
    pub descriptor_binding_storage_image_update_after_bind: Bool32,
    #[doc(alias = "descriptorBindingStorageBufferUpdateAfterBind")]
    pub descriptor_binding_storage_buffer_update_after_bind: Bool32,
    #[doc(alias = "descriptorBindingUniformTexelBufferUpdateAfterBind")]
    pub descriptor_binding_uniform_texel_buffer_update_after_bind: Bool32,
    #[doc(alias = "descriptorBindingStorageTexelBufferUpdateAfterBind")]
    pub descriptor_binding_storage_texel_buffer_update_after_bind: Bool32,
    #[doc(alias = "descriptorBindingUpdateUnusedWhilePending")]
    pub descriptor_binding_update_unused_while_pending: Bool32,
    #[doc(alias = "descriptorBindingPartiallyBound")]
    pub descriptor_binding_partially_bound: Bool32,
    #[doc(alias = "descriptorBindingVariableDescriptorCount")]
    pub descriptor_binding_variable_descriptor_count: Bool32,
    #[doc(alias = "runtimeDescriptorArray")]
    pub runtime_descriptor_array: Bool32,
    #[doc(alias = "samplerFilterMinmax")]
    pub sampler_filter_minmax: Bool32,
    #[doc(alias = "scalarBlockLayout")]
    pub scalar_block_layout: Bool32,
    #[doc(alias = "imagelessFramebuffer")]
    pub imageless_framebuffer: Bool32,
    #[doc(alias = "uniformBufferStandardLayout")]
    pub uniform_buffer_standard_layout: Bool32,
    #[doc(alias = "shaderSubgroupExtendedTypes")]
    pub shader_subgroup_extended_types: Bool32,
    #[doc(alias = "separateDepthStencilLayouts")]
    pub separate_depth_stencil_layouts: Bool32,
    #[doc(alias = "hostQueryReset")]
    pub host_query_reset: Bool32,
    #[doc(alias = "timelineSemaphore")]
    pub timeline_semaphore: Bool32,
    #[doc(alias = "bufferDeviceAddress")]
    pub buffer_device_address: Bool32,
    #[doc(alias = "bufferDeviceAddressCaptureReplay")]
    pub buffer_device_address_capture_replay: Bool32,
    #[doc(alias = "bufferDeviceAddressMultiDevice")]
    pub buffer_device_address_multi_device: Bool32,
    #[doc(alias = "vulkanMemoryModel")]
    pub vulkan_memory_model: Bool32,
    #[doc(alias = "vulkanMemoryModelDeviceScope")]
    pub vulkan_memory_model_device_scope: Bool32,
    #[doc(alias = "vulkanMemoryModelAvailabilityVisibilityChains")]
    pub vulkan_memory_model_availability_visibility_chains: Bool32,
    #[doc(alias = "shaderOutputViewportIndex")]
    pub shader_output_viewport_index: Bool32,
    #[doc(alias = "shaderOutputLayer")]
    pub shader_output_layer: Bool32,
    #[doc(alias = "subgroupBroadcastDynamicId")]
    pub subgroup_broadcast_dynamic_id: Bool32,
}
impl Default for PhysicalDeviceVulkan12Features {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceVulkan12Features,
            p_next: unsafe { std::mem::zeroed() },
            sampler_mirror_clamp_to_edge: unsafe { std::mem::zeroed() },
            draw_indirect_count: unsafe { std::mem::zeroed() },
            storage_buffer8_bit_access: unsafe { std::mem::zeroed() },
            uniform_and_storage_buffer8_bit_access: unsafe { std::mem::zeroed() },
            storage_push_constant8: unsafe { std::mem::zeroed() },
            shader_buffer_int64_atomics: unsafe { std::mem::zeroed() },
            shader_shared_int64_atomics: unsafe { std::mem::zeroed() },
            shader_float16: unsafe { std::mem::zeroed() },
            shader_int8: unsafe { std::mem::zeroed() },
            descriptor_indexing: unsafe { std::mem::zeroed() },
            shader_input_attachment_array_dynamic_indexing: unsafe { std::mem::zeroed() },
            shader_uniform_texel_buffer_array_dynamic_indexing: unsafe { std::mem::zeroed() },
            shader_storage_texel_buffer_array_dynamic_indexing: unsafe { std::mem::zeroed() },
            shader_uniform_buffer_array_non_uniform_indexing: unsafe { std::mem::zeroed() },
            shader_sampled_image_array_non_uniform_indexing: unsafe { std::mem::zeroed() },
            shader_storage_buffer_array_non_uniform_indexing: unsafe { std::mem::zeroed() },
            shader_storage_image_array_non_uniform_indexing: unsafe { std::mem::zeroed() },
            shader_input_attachment_array_non_uniform_indexing: unsafe { std::mem::zeroed() },
            shader_uniform_texel_buffer_array_non_uniform_indexing: unsafe { std::mem::zeroed() },
            shader_storage_texel_buffer_array_non_uniform_indexing: unsafe { std::mem::zeroed() },
            descriptor_binding_uniform_buffer_update_after_bind: unsafe { std::mem::zeroed() },
            descriptor_binding_sampled_image_update_after_bind: unsafe { std::mem::zeroed() },
            descriptor_binding_storage_image_update_after_bind: unsafe { std::mem::zeroed() },
            descriptor_binding_storage_buffer_update_after_bind: unsafe { std::mem::zeroed() },
            descriptor_binding_uniform_texel_buffer_update_after_bind: unsafe { std::mem::zeroed() },
            descriptor_binding_storage_texel_buffer_update_after_bind: unsafe { std::mem::zeroed() },
            descriptor_binding_update_unused_while_pending: unsafe { std::mem::zeroed() },
            descriptor_binding_partially_bound: unsafe { std::mem::zeroed() },
            descriptor_binding_variable_descriptor_count: unsafe { std::mem::zeroed() },
            runtime_descriptor_array: unsafe { std::mem::zeroed() },
            sampler_filter_minmax: unsafe { std::mem::zeroed() },
            scalar_block_layout: unsafe { std::mem::zeroed() },
            imageless_framebuffer: unsafe { std::mem::zeroed() },
            uniform_buffer_standard_layout: unsafe { std::mem::zeroed() },
            shader_subgroup_extended_types: unsafe { std::mem::zeroed() },
            separate_depth_stencil_layouts: unsafe { std::mem::zeroed() },
            host_query_reset: unsafe { std::mem::zeroed() },
            timeline_semaphore: unsafe { std::mem::zeroed() },
            buffer_device_address: unsafe { std::mem::zeroed() },
            buffer_device_address_capture_replay: unsafe { std::mem::zeroed() },
            buffer_device_address_multi_device: unsafe { std::mem::zeroed() },
            vulkan_memory_model: unsafe { std::mem::zeroed() },
            vulkan_memory_model_device_scope: unsafe { std::mem::zeroed() },
            vulkan_memory_model_availability_visibility_chains: unsafe { std::mem::zeroed() },
            shader_output_viewport_index: unsafe { std::mem::zeroed() },
            shader_output_layer: unsafe { std::mem::zeroed() },
            subgroup_broadcast_dynamic_id: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceVulkan12Properties")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceVulkan12Properties {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "driverID")]
    pub driver_id: DriverId,
    #[doc(alias = "driverName")]
    pub driver_name: [std::ffi::c_char; MAX_DRIVER_NAME_SIZE as usize],
    #[doc(alias = "driverInfo")]
    pub driver_info: [std::ffi::c_char; MAX_DRIVER_INFO_SIZE as usize],
    #[doc(alias = "conformanceVersion")]
    pub conformance_version: ConformanceVersion,
    #[doc(alias = "denormBehaviorIndependence")]
    pub denorm_behavior_independence: ShaderFloatControlsIndependence,
    #[doc(alias = "roundingModeIndependence")]
    pub rounding_mode_independence: ShaderFloatControlsIndependence,
    #[doc(alias = "shaderSignedZeroInfNanPreserveFloat16")]
    pub shader_signed_zero_inf_nan_preserve_float16: Bool32,
    #[doc(alias = "shaderSignedZeroInfNanPreserveFloat32")]
    pub shader_signed_zero_inf_nan_preserve_float32: Bool32,
    #[doc(alias = "shaderSignedZeroInfNanPreserveFloat64")]
    pub shader_signed_zero_inf_nan_preserve_float64: Bool32,
    #[doc(alias = "shaderDenormPreserveFloat16")]
    pub shader_denorm_preserve_float16: Bool32,
    #[doc(alias = "shaderDenormPreserveFloat32")]
    pub shader_denorm_preserve_float32: Bool32,
    #[doc(alias = "shaderDenormPreserveFloat64")]
    pub shader_denorm_preserve_float64: Bool32,
    #[doc(alias = "shaderDenormFlushToZeroFloat16")]
    pub shader_denorm_flush_to_zero_float16: Bool32,
    #[doc(alias = "shaderDenormFlushToZeroFloat32")]
    pub shader_denorm_flush_to_zero_float32: Bool32,
    #[doc(alias = "shaderDenormFlushToZeroFloat64")]
    pub shader_denorm_flush_to_zero_float64: Bool32,
    #[doc(alias = "shaderRoundingModeRTEFloat16")]
    pub shader_rounding_mode_rte_float16: Bool32,
    #[doc(alias = "shaderRoundingModeRTEFloat32")]
    pub shader_rounding_mode_rte_float32: Bool32,
    #[doc(alias = "shaderRoundingModeRTEFloat64")]
    pub shader_rounding_mode_rte_float64: Bool32,
    #[doc(alias = "shaderRoundingModeRTZFloat16")]
    pub shader_rounding_mode_rtz_float16: Bool32,
    #[doc(alias = "shaderRoundingModeRTZFloat32")]
    pub shader_rounding_mode_rtz_float32: Bool32,
    #[doc(alias = "shaderRoundingModeRTZFloat64")]
    pub shader_rounding_mode_rtz_float64: Bool32,
    #[doc(alias = "maxUpdateAfterBindDescriptorsInAllPools")]
    pub max_update_after_bind_descriptors_in_all_pools: u32,
    #[doc(alias = "shaderUniformBufferArrayNonUniformIndexingNative")]
    pub shader_uniform_buffer_array_non_uniform_indexing_native: Bool32,
    #[doc(alias = "shaderSampledImageArrayNonUniformIndexingNative")]
    pub shader_sampled_image_array_non_uniform_indexing_native: Bool32,
    #[doc(alias = "shaderStorageBufferArrayNonUniformIndexingNative")]
    pub shader_storage_buffer_array_non_uniform_indexing_native: Bool32,
    #[doc(alias = "shaderStorageImageArrayNonUniformIndexingNative")]
    pub shader_storage_image_array_non_uniform_indexing_native: Bool32,
    #[doc(alias = "shaderInputAttachmentArrayNonUniformIndexingNative")]
    pub shader_input_attachment_array_non_uniform_indexing_native: Bool32,
    #[doc(alias = "robustBufferAccessUpdateAfterBind")]
    pub robust_buffer_access_update_after_bind: Bool32,
    #[doc(alias = "quadDivergentImplicitLod")]
    pub quad_divergent_implicit_lod: Bool32,
    #[doc(alias = "maxPerStageDescriptorUpdateAfterBindSamplers")]
    pub max_per_stage_descriptor_update_after_bind_samplers: u32,
    #[doc(alias = "maxPerStageDescriptorUpdateAfterBindUniformBuffers")]
    pub max_per_stage_descriptor_update_after_bind_uniform_buffers: u32,
    #[doc(alias = "maxPerStageDescriptorUpdateAfterBindStorageBuffers")]
    pub max_per_stage_descriptor_update_after_bind_storage_buffers: u32,
    #[doc(alias = "maxPerStageDescriptorUpdateAfterBindSampledImages")]
    pub max_per_stage_descriptor_update_after_bind_sampled_images: u32,
    #[doc(alias = "maxPerStageDescriptorUpdateAfterBindStorageImages")]
    pub max_per_stage_descriptor_update_after_bind_storage_images: u32,
    #[doc(alias = "maxPerStageDescriptorUpdateAfterBindInputAttachments")]
    pub max_per_stage_descriptor_update_after_bind_input_attachments: u32,
    #[doc(alias = "maxPerStageUpdateAfterBindResources")]
    pub max_per_stage_update_after_bind_resources: u32,
    #[doc(alias = "maxDescriptorSetUpdateAfterBindSamplers")]
    pub max_descriptor_set_update_after_bind_samplers: u32,
    #[doc(alias = "maxDescriptorSetUpdateAfterBindUniformBuffers")]
    pub max_descriptor_set_update_after_bind_uniform_buffers: u32,
    #[doc(alias = "maxDescriptorSetUpdateAfterBindUniformBuffersDynamic")]
    pub max_descriptor_set_update_after_bind_uniform_buffers_dynamic: u32,
    #[doc(alias = "maxDescriptorSetUpdateAfterBindStorageBuffers")]
    pub max_descriptor_set_update_after_bind_storage_buffers: u32,
    #[doc(alias = "maxDescriptorSetUpdateAfterBindStorageBuffersDynamic")]
    pub max_descriptor_set_update_after_bind_storage_buffers_dynamic: u32,
    #[doc(alias = "maxDescriptorSetUpdateAfterBindSampledImages")]
    pub max_descriptor_set_update_after_bind_sampled_images: u32,
    #[doc(alias = "maxDescriptorSetUpdateAfterBindStorageImages")]
    pub max_descriptor_set_update_after_bind_storage_images: u32,
    #[doc(alias = "maxDescriptorSetUpdateAfterBindInputAttachments")]
    pub max_descriptor_set_update_after_bind_input_attachments: u32,
    #[doc(alias = "supportedDepthResolveModes")]
    pub supported_depth_resolve_modes: ResolveModeFlags,
    #[doc(alias = "supportedStencilResolveModes")]
    pub supported_stencil_resolve_modes: ResolveModeFlags,
    #[doc(alias = "independentResolveNone")]
    pub independent_resolve_none: Bool32,
    #[doc(alias = "independentResolve")]
    pub independent_resolve: Bool32,
    #[doc(alias = "filterMinmaxSingleComponentFormats")]
    pub filter_minmax_single_component_formats: Bool32,
    #[doc(alias = "filterMinmaxImageComponentMapping")]
    pub filter_minmax_image_component_mapping: Bool32,
    #[doc(alias = "maxTimelineSemaphoreValueDifference")]
    pub max_timeline_semaphore_value_difference: u64,
    #[doc(alias = "framebufferIntegerColorSampleCounts")]
    pub framebuffer_integer_color_sample_counts: SampleCountFlags,
}
impl Default for PhysicalDeviceVulkan12Properties {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceVulkan12Properties,
            p_next: unsafe { std::mem::zeroed() },
            driver_id: unsafe { std::mem::zeroed() },
            driver_name: unsafe { std::mem::zeroed() },
            driver_info: unsafe { std::mem::zeroed() },
            conformance_version: unsafe { std::mem::zeroed() },
            denorm_behavior_independence: unsafe { std::mem::zeroed() },
            rounding_mode_independence: unsafe { std::mem::zeroed() },
            shader_signed_zero_inf_nan_preserve_float16: unsafe { std::mem::zeroed() },
            shader_signed_zero_inf_nan_preserve_float32: unsafe { std::mem::zeroed() },
            shader_signed_zero_inf_nan_preserve_float64: unsafe { std::mem::zeroed() },
            shader_denorm_preserve_float16: unsafe { std::mem::zeroed() },
            shader_denorm_preserve_float32: unsafe { std::mem::zeroed() },
            shader_denorm_preserve_float64: unsafe { std::mem::zeroed() },
            shader_denorm_flush_to_zero_float16: unsafe { std::mem::zeroed() },
            shader_denorm_flush_to_zero_float32: unsafe { std::mem::zeroed() },
            shader_denorm_flush_to_zero_float64: unsafe { std::mem::zeroed() },
            shader_rounding_mode_rte_float16: unsafe { std::mem::zeroed() },
            shader_rounding_mode_rte_float32: unsafe { std::mem::zeroed() },
            shader_rounding_mode_rte_float64: unsafe { std::mem::zeroed() },
            shader_rounding_mode_rtz_float16: unsafe { std::mem::zeroed() },
            shader_rounding_mode_rtz_float32: unsafe { std::mem::zeroed() },
            shader_rounding_mode_rtz_float64: unsafe { std::mem::zeroed() },
            max_update_after_bind_descriptors_in_all_pools: unsafe { std::mem::zeroed() },
            shader_uniform_buffer_array_non_uniform_indexing_native: unsafe { std::mem::zeroed() },
            shader_sampled_image_array_non_uniform_indexing_native: unsafe { std::mem::zeroed() },
            shader_storage_buffer_array_non_uniform_indexing_native: unsafe { std::mem::zeroed() },
            shader_storage_image_array_non_uniform_indexing_native: unsafe { std::mem::zeroed() },
            shader_input_attachment_array_non_uniform_indexing_native: unsafe { std::mem::zeroed() },
            robust_buffer_access_update_after_bind: unsafe { std::mem::zeroed() },
            quad_divergent_implicit_lod: unsafe { std::mem::zeroed() },
            max_per_stage_descriptor_update_after_bind_samplers: unsafe { std::mem::zeroed() },
            max_per_stage_descriptor_update_after_bind_uniform_buffers: unsafe { std::mem::zeroed() },
            max_per_stage_descriptor_update_after_bind_storage_buffers: unsafe { std::mem::zeroed() },
            max_per_stage_descriptor_update_after_bind_sampled_images: unsafe { std::mem::zeroed() },
            max_per_stage_descriptor_update_after_bind_storage_images: unsafe { std::mem::zeroed() },
            max_per_stage_descriptor_update_after_bind_input_attachments: unsafe { std::mem::zeroed() },
            max_per_stage_update_after_bind_resources: unsafe { std::mem::zeroed() },
            max_descriptor_set_update_after_bind_samplers: unsafe { std::mem::zeroed() },
            max_descriptor_set_update_after_bind_uniform_buffers: unsafe { std::mem::zeroed() },
            max_descriptor_set_update_after_bind_uniform_buffers_dynamic: unsafe { std::mem::zeroed() },
            max_descriptor_set_update_after_bind_storage_buffers: unsafe { std::mem::zeroed() },
            max_descriptor_set_update_after_bind_storage_buffers_dynamic: unsafe { std::mem::zeroed() },
            max_descriptor_set_update_after_bind_sampled_images: unsafe { std::mem::zeroed() },
            max_descriptor_set_update_after_bind_storage_images: unsafe { std::mem::zeroed() },
            max_descriptor_set_update_after_bind_input_attachments: unsafe { std::mem::zeroed() },
            supported_depth_resolve_modes: unsafe { std::mem::zeroed() },
            supported_stencil_resolve_modes: unsafe { std::mem::zeroed() },
            independent_resolve_none: unsafe { std::mem::zeroed() },
            independent_resolve: unsafe { std::mem::zeroed() },
            filter_minmax_single_component_formats: unsafe { std::mem::zeroed() },
            filter_minmax_image_component_mapping: unsafe { std::mem::zeroed() },
            max_timeline_semaphore_value_difference: unsafe { std::mem::zeroed() },
            framebuffer_integer_color_sample_counts: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::vulkan1_2::{
    DescriptorBindingFlagBits, DescriptorBindingFlags, DriverId, ResolveModeFlagBits, ResolveModeFlags,
    SamplerReductionMode, SemaphoreType, SemaphoreWaitFlagBits, SemaphoreWaitFlags, ShaderFloatControlsIndependence,
};
#[doc(alias = "vkResetQueryPool")]
pub type FNResetQueryPool =
    unsafe extern "system" fn(device: Device, query_pool: QueryPool, first_query: u32, query_count: u32);
#[doc(alias = "vkCreateRenderPass2")]
pub type FNCreateRenderPass2 = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const RenderPassCreateInfo2,
    p_allocator: *const AllocationCallbacks,
    p_render_pass: *mut RenderPass,
) -> VulkanResultCodes;
#[doc(alias = "vkGetSemaphoreCounterValue")]
pub type FNGetSemaphoreCounterValue =
    unsafe extern "system" fn(device: Device, semaphore: Semaphore, p_value: *mut u64) -> VulkanResultCodes;
#[doc(alias = "vkWaitSemaphores")]
pub type FNWaitSemaphores =
    unsafe extern "system" fn(device: Device, p_wait_info: *const SemaphoreWaitInfo, timeout: u64) -> VulkanResultCodes;
#[doc(alias = "vkSignalSemaphore")]
pub type FNSignalSemaphore =
    unsafe extern "system" fn(device: Device, p_signal_info: *const SemaphoreSignalInfo) -> VulkanResultCodes;
#[doc(alias = "vkGetBufferOpaqueCaptureAddress")]
pub type FNGetBufferOpaqueCaptureAddress =
    unsafe extern "system" fn(device: Device, p_info: *const BufferDeviceAddressInfo) -> u64;
#[doc(alias = "vkGetBufferDeviceAddress")]
pub type FNGetBufferDeviceAddress =
    unsafe extern "system" fn(device: Device, p_info: *const BufferDeviceAddressInfo) -> DeviceAddress;
#[doc(alias = "vkGetDeviceMemoryOpaqueCaptureAddress")]
pub type FNGetDeviceMemoryOpaqueCaptureAddress =
    unsafe extern "system" fn(device: Device, p_info: *const DeviceMemoryOpaqueCaptureAddressInfo) -> u64;
#[doc(alias = "vkCmdBeginRenderPass2")]
pub type FNCmdBeginRenderPass2 = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_render_pass_begin: *const RenderPassBeginInfo,
    p_subpass_begin_info: *const SubpassBeginInfo,
);
#[doc(alias = "vkCmdNextSubpass2")]
pub type FNCmdNextSubpass2 = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_subpass_begin_info: *const SubpassBeginInfo,
    p_subpass_end_info: *const SubpassEndInfo,
);
#[doc(alias = "vkCmdEndRenderPass2")]
pub type FNCmdEndRenderPass2 =
    unsafe extern "system" fn(command_buffer: CommandBuffer, p_subpass_end_info: *const SubpassEndInfo);
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
