pub use crate::common::vulkan1_3::PipelineCreationFeedback;
#[cfg(feature = "VULKAN_1_2")]
use crate::native::vulkan1_2::ResolveModeFlagBits;
use crate::native::{
    vulkan1_0::{
        AllocationCallbacks, AttachmentLoadOp, AttachmentStoreOp, BaseInStructure, BaseOutStructure, Bool32, Buffer,
        BufferCreateInfo, ClearValue, CommandBuffer, CompareOp, CullModeFlags, DependencyFlags, Device, DeviceSize,
        Event, Extent3D, Fence, Filter, Format, FrontFace, Image, ImageAspectFlagBits, ImageCreateInfo, ImageLayout,
        ImageSubresourceLayers, ImageSubresourceRange, ImageView, ObjectType, Offset3D, PhysicalDevice,
        PrimitiveTopology, QueryPool, Queue, Rect2D, SampleCountFlagBits, Semaphore, ShaderStageFlags,
        StencilFaceFlags, StencilOp, StructureType, Viewport, VulkanResultCodes, MAX_DESCRIPTION_SIZE,
        MAX_EXTENSION_NAME_SIZE,
    },
    vulkan1_1::{MemoryRequirements2, SparseImageMemoryRequirements2},
};
#[doc(alias = "VkDevicePrivateDataCreateInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DevicePrivateDataCreateInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "privateDataSlotRequestCount")]
    pub private_data_slot_request_count: u32,
}
impl Default for DevicePrivateDataCreateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::DevicePrivateDataCreateInfo,
            p_next: unsafe { std::mem::zeroed() },
            private_data_slot_request_count: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPrivateDataSlotCreateInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PrivateDataSlotCreateInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub flags: PrivateDataSlotCreateFlags,
}
impl Default for PrivateDataSlotCreateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::PrivateDataSlotCreateInfo,
            p_next: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDevicePrivateDataFeatures")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDevicePrivateDataFeatures {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "privateData")]
    pub private_data: Bool32,
}
impl Default for PhysicalDevicePrivateDataFeatures {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDevicePrivateDataFeatures,
            p_next: unsafe { std::mem::zeroed() },
            private_data: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkDeviceBufferMemoryRequirements")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DeviceBufferMemoryRequirements {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "pCreateInfo")]
    pub create_info: *const BufferCreateInfo,
}
impl Default for DeviceBufferMemoryRequirements {
    fn default() -> Self {
        Self {
            s_type: StructureType::DeviceBufferMemoryRequirements,
            p_next: unsafe { std::mem::zeroed() },
            create_info: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkDeviceImageMemoryRequirements")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DeviceImageMemoryRequirements {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "pCreateInfo")]
    pub create_info: *const ImageCreateInfo,
    #[doc(alias = "planeAspect")]
    pub plane_aspect: ImageAspectFlagBits,
}
impl Default for DeviceImageMemoryRequirements {
    fn default() -> Self {
        Self {
            s_type: StructureType::DeviceImageMemoryRequirements,
            p_next: unsafe { std::mem::zeroed() },
            create_info: unsafe { std::mem::zeroed() },
            plane_aspect: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceInlineUniformBlockFeatures")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceInlineUniformBlockFeatures {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "inlineUniformBlock")]
    pub inline_uniform_block: Bool32,
    #[doc(alias = "descriptorBindingInlineUniformBlockUpdateAfterBind")]
    pub descriptor_binding_inline_uniform_block_update_after_bind: Bool32,
}
impl Default for PhysicalDeviceInlineUniformBlockFeatures {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceInlineUniformBlockFeatures,
            p_next: unsafe { std::mem::zeroed() },
            inline_uniform_block: unsafe { std::mem::zeroed() },
            descriptor_binding_inline_uniform_block_update_after_bind: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceInlineUniformBlockProperties")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceInlineUniformBlockProperties {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "maxInlineUniformBlockSize")]
    pub max_inline_uniform_block_size: u32,
    #[doc(alias = "maxPerStageDescriptorInlineUniformBlocks")]
    pub max_per_stage_descriptor_inline_uniform_blocks: u32,
    #[doc(alias = "maxPerStageDescriptorUpdateAfterBindInlineUniformBlocks")]
    pub max_per_stage_descriptor_update_after_bind_inline_uniform_blocks: u32,
    #[doc(alias = "maxDescriptorSetInlineUniformBlocks")]
    pub max_descriptor_set_inline_uniform_blocks: u32,
    #[doc(alias = "maxDescriptorSetUpdateAfterBindInlineUniformBlocks")]
    pub max_descriptor_set_update_after_bind_inline_uniform_blocks: u32,
}
impl Default for PhysicalDeviceInlineUniformBlockProperties {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceInlineUniformBlockProperties,
            p_next: unsafe { std::mem::zeroed() },
            max_inline_uniform_block_size: unsafe { std::mem::zeroed() },
            max_per_stage_descriptor_inline_uniform_blocks: unsafe { std::mem::zeroed() },
            max_per_stage_descriptor_update_after_bind_inline_uniform_blocks: unsafe { std::mem::zeroed() },
            max_descriptor_set_inline_uniform_blocks: unsafe { std::mem::zeroed() },
            max_descriptor_set_update_after_bind_inline_uniform_blocks: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkWriteDescriptorSetInlineUniformBlock")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct WriteDescriptorSetInlineUniformBlock {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "dataSize")]
    pub data_size: u32,
    #[doc(alias = "pData")]
    pub data: *const std::ffi::c_void,
}
impl Default for WriteDescriptorSetInlineUniformBlock {
    fn default() -> Self {
        Self {
            s_type: StructureType::WriteDescriptorSetInlineUniformBlock,
            p_next: unsafe { std::mem::zeroed() },
            data_size: unsafe { std::mem::zeroed() },
            data: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkDescriptorPoolInlineUniformBlockCreateInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DescriptorPoolInlineUniformBlockCreateInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "maxInlineUniformBlockBindings")]
    pub max_inline_uniform_block_bindings: u32,
}
impl Default for DescriptorPoolInlineUniformBlockCreateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::DescriptorPoolInlineUniformBlockCreateInfo,
            p_next: unsafe { std::mem::zeroed() },
            max_inline_uniform_block_bindings: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceMaintenance4Features")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceMaintenance4Features {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    pub maintenance4: Bool32,
}
impl Default for PhysicalDeviceMaintenance4Features {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceMaintenance4Features,
            p_next: unsafe { std::mem::zeroed() },
            maintenance4: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceMaintenance4Properties")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceMaintenance4Properties {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "maxBufferSize")]
    pub max_buffer_size: DeviceSize,
}
impl Default for PhysicalDeviceMaintenance4Properties {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceMaintenance4Properties,
            p_next: unsafe { std::mem::zeroed() },
            max_buffer_size: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceTextureCompressionASTCHDRFeatures")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceTextureCompressionAstchdrFeatures {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "textureCompressionASTC_HDR")]
    pub texture_compression_astc_hdr: Bool32,
}
impl Default for PhysicalDeviceTextureCompressionAstchdrFeatures {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceTextureCompressionAstcHdrFeatures,
            p_next: unsafe { std::mem::zeroed() },
            texture_compression_astc_hdr: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPipelineCreationFeedbackCreateInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PipelineCreationFeedbackCreateInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "pPipelineCreationFeedback")]
    pub pipeline_creation_feedback: *mut PipelineCreationFeedback,
    #[doc(alias = "pipelineStageCreationFeedbackCount")]
    pub pipeline_stage_creation_feedback_count: u32,
    #[doc(alias = "pPipelineStageCreationFeedbacks")]
    pub pipeline_stage_creation_feedbacks: *mut PipelineCreationFeedback,
}
impl Default for PipelineCreationFeedbackCreateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::PipelineCreationFeedbackCreateInfo,
            p_next: unsafe { std::mem::zeroed() },
            pipeline_creation_feedback: unsafe { std::mem::zeroed() },
            pipeline_stage_creation_feedback_count: unsafe { std::mem::zeroed() },
            pipeline_stage_creation_feedbacks: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceShaderDemoteToHelperInvocationFeatures")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShaderDemoteToHelperInvocationFeatures {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "shaderDemoteToHelperInvocation")]
    pub shader_demote_to_helper_invocation: Bool32,
}
impl Default for PhysicalDeviceShaderDemoteToHelperInvocationFeatures {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceShaderDemoteToHelperInvocationFeatures,
            p_next: unsafe { std::mem::zeroed() },
            shader_demote_to_helper_invocation: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceTexelBufferAlignmentProperties")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceTexelBufferAlignmentProperties {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "storageTexelBufferOffsetAlignmentBytes")]
    pub storage_texel_buffer_offset_alignment_bytes: DeviceSize,
    #[doc(alias = "storageTexelBufferOffsetSingleTexelAlignment")]
    pub storage_texel_buffer_offset_single_texel_alignment: Bool32,
    #[doc(alias = "uniformTexelBufferOffsetAlignmentBytes")]
    pub uniform_texel_buffer_offset_alignment_bytes: DeviceSize,
    #[doc(alias = "uniformTexelBufferOffsetSingleTexelAlignment")]
    pub uniform_texel_buffer_offset_single_texel_alignment: Bool32,
}
impl Default for PhysicalDeviceTexelBufferAlignmentProperties {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceTexelBufferAlignmentProperties,
            p_next: unsafe { std::mem::zeroed() },
            storage_texel_buffer_offset_alignment_bytes: unsafe { std::mem::zeroed() },
            storage_texel_buffer_offset_single_texel_alignment: unsafe { std::mem::zeroed() },
            uniform_texel_buffer_offset_alignment_bytes: unsafe { std::mem::zeroed() },
            uniform_texel_buffer_offset_single_texel_alignment: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceSubgroupSizeControlFeatures")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceSubgroupSizeControlFeatures {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "subgroupSizeControl")]
    pub subgroup_size_control: Bool32,
    #[doc(alias = "computeFullSubgroups")]
    pub compute_full_subgroups: Bool32,
}
impl Default for PhysicalDeviceSubgroupSizeControlFeatures {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceSubgroupSizeControlFeatures,
            p_next: unsafe { std::mem::zeroed() },
            subgroup_size_control: unsafe { std::mem::zeroed() },
            compute_full_subgroups: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceSubgroupSizeControlProperties")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceSubgroupSizeControlProperties {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "minSubgroupSize")]
    pub min_subgroup_size: u32,
    #[doc(alias = "maxSubgroupSize")]
    pub max_subgroup_size: u32,
    #[doc(alias = "maxComputeWorkgroupSubgroups")]
    pub max_compute_workgroup_subgroups: u32,
    #[doc(alias = "requiredSubgroupSizeStages")]
    pub required_subgroup_size_stages: ShaderStageFlags,
}
impl Default for PhysicalDeviceSubgroupSizeControlProperties {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceSubgroupSizeControlProperties,
            p_next: unsafe { std::mem::zeroed() },
            min_subgroup_size: unsafe { std::mem::zeroed() },
            max_subgroup_size: unsafe { std::mem::zeroed() },
            max_compute_workgroup_subgroups: unsafe { std::mem::zeroed() },
            required_subgroup_size_stages: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPipelineShaderStageRequiredSubgroupSizeCreateInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PipelineShaderStageRequiredSubgroupSizeCreateInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "requiredSubgroupSize")]
    pub required_subgroup_size: u32,
}
impl Default for PipelineShaderStageRequiredSubgroupSizeCreateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::PipelineShaderStageRequiredSubgroupSizeCreateInfo,
            p_next: unsafe { std::mem::zeroed() },
            required_subgroup_size: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDevicePipelineCreationCacheControlFeatures")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDevicePipelineCreationCacheControlFeatures {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "pipelineCreationCacheControl")]
    pub pipeline_creation_cache_control: Bool32,
}
impl Default for PhysicalDevicePipelineCreationCacheControlFeatures {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDevicePipelineCreationCacheControlFeatures,
            p_next: unsafe { std::mem::zeroed() },
            pipeline_creation_cache_control: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceVulkan13Features")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceVulkan13Features {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "robustImageAccess")]
    pub robust_image_access: Bool32,
    #[doc(alias = "inlineUniformBlock")]
    pub inline_uniform_block: Bool32,
    #[doc(alias = "descriptorBindingInlineUniformBlockUpdateAfterBind")]
    pub descriptor_binding_inline_uniform_block_update_after_bind: Bool32,
    #[doc(alias = "pipelineCreationCacheControl")]
    pub pipeline_creation_cache_control: Bool32,
    #[doc(alias = "privateData")]
    pub private_data: Bool32,
    #[doc(alias = "shaderDemoteToHelperInvocation")]
    pub shader_demote_to_helper_invocation: Bool32,
    #[doc(alias = "shaderTerminateInvocation")]
    pub shader_terminate_invocation: Bool32,
    #[doc(alias = "subgroupSizeControl")]
    pub subgroup_size_control: Bool32,
    #[doc(alias = "computeFullSubgroups")]
    pub compute_full_subgroups: Bool32,
    pub synchronization2: Bool32,
    #[doc(alias = "textureCompressionASTC_HDR")]
    pub texture_compression_astc_hdr: Bool32,
    #[doc(alias = "shaderZeroInitializeWorkgroupMemory")]
    pub shader_zero_initialize_workgroup_memory: Bool32,
    #[doc(alias = "dynamicRendering")]
    pub dynamic_rendering: Bool32,
    #[doc(alias = "shaderIntegerDotProduct")]
    pub shader_integer_dot_product: Bool32,
    pub maintenance4: Bool32,
}
impl Default for PhysicalDeviceVulkan13Features {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceVulkan13Features,
            p_next: unsafe { std::mem::zeroed() },
            robust_image_access: unsafe { std::mem::zeroed() },
            inline_uniform_block: unsafe { std::mem::zeroed() },
            descriptor_binding_inline_uniform_block_update_after_bind: unsafe { std::mem::zeroed() },
            pipeline_creation_cache_control: unsafe { std::mem::zeroed() },
            private_data: unsafe { std::mem::zeroed() },
            shader_demote_to_helper_invocation: unsafe { std::mem::zeroed() },
            shader_terminate_invocation: unsafe { std::mem::zeroed() },
            subgroup_size_control: unsafe { std::mem::zeroed() },
            compute_full_subgroups: unsafe { std::mem::zeroed() },
            synchronization2: unsafe { std::mem::zeroed() },
            texture_compression_astc_hdr: unsafe { std::mem::zeroed() },
            shader_zero_initialize_workgroup_memory: unsafe { std::mem::zeroed() },
            dynamic_rendering: unsafe { std::mem::zeroed() },
            shader_integer_dot_product: unsafe { std::mem::zeroed() },
            maintenance4: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceVulkan13Properties")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceVulkan13Properties {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "minSubgroupSize")]
    pub min_subgroup_size: u32,
    #[doc(alias = "maxSubgroupSize")]
    pub max_subgroup_size: u32,
    #[doc(alias = "maxComputeWorkgroupSubgroups")]
    pub max_compute_workgroup_subgroups: u32,
    #[doc(alias = "requiredSubgroupSizeStages")]
    pub required_subgroup_size_stages: ShaderStageFlags,
    #[doc(alias = "maxInlineUniformBlockSize")]
    pub max_inline_uniform_block_size: u32,
    #[doc(alias = "maxPerStageDescriptorInlineUniformBlocks")]
    pub max_per_stage_descriptor_inline_uniform_blocks: u32,
    #[doc(alias = "maxPerStageDescriptorUpdateAfterBindInlineUniformBlocks")]
    pub max_per_stage_descriptor_update_after_bind_inline_uniform_blocks: u32,
    #[doc(alias = "maxDescriptorSetInlineUniformBlocks")]
    pub max_descriptor_set_inline_uniform_blocks: u32,
    #[doc(alias = "maxDescriptorSetUpdateAfterBindInlineUniformBlocks")]
    pub max_descriptor_set_update_after_bind_inline_uniform_blocks: u32,
    #[doc(alias = "maxInlineUniformTotalSize")]
    pub max_inline_uniform_total_size: u32,
    #[doc(alias = "integerDotProduct8BitUnsignedAccelerated")]
    pub integer_dot_product8_bit_unsigned_accelerated: Bool32,
    #[doc(alias = "integerDotProduct8BitSignedAccelerated")]
    pub integer_dot_product8_bit_signed_accelerated: Bool32,
    #[doc(alias = "integerDotProduct8BitMixedSignednessAccelerated")]
    pub integer_dot_product8_bit_mixed_signedness_accelerated: Bool32,
    #[doc(alias = "integerDotProduct4x8BitPackedUnsignedAccelerated")]
    pub integer_dot_product4x8_bit_packed_unsigned_accelerated: Bool32,
    #[doc(alias = "integerDotProduct4x8BitPackedSignedAccelerated")]
    pub integer_dot_product4x8_bit_packed_signed_accelerated: Bool32,
    #[doc(alias = "integerDotProduct4x8BitPackedMixedSignednessAccelerated")]
    pub integer_dot_product4x8_bit_packed_mixed_signedness_accelerated: Bool32,
    #[doc(alias = "integerDotProduct16BitUnsignedAccelerated")]
    pub integer_dot_product16_bit_unsigned_accelerated: Bool32,
    #[doc(alias = "integerDotProduct16BitSignedAccelerated")]
    pub integer_dot_product16_bit_signed_accelerated: Bool32,
    #[doc(alias = "integerDotProduct16BitMixedSignednessAccelerated")]
    pub integer_dot_product16_bit_mixed_signedness_accelerated: Bool32,
    #[doc(alias = "integerDotProduct32BitUnsignedAccelerated")]
    pub integer_dot_product32_bit_unsigned_accelerated: Bool32,
    #[doc(alias = "integerDotProduct32BitSignedAccelerated")]
    pub integer_dot_product32_bit_signed_accelerated: Bool32,
    #[doc(alias = "integerDotProduct32BitMixedSignednessAccelerated")]
    pub integer_dot_product32_bit_mixed_signedness_accelerated: Bool32,
    #[doc(alias = "integerDotProduct64BitUnsignedAccelerated")]
    pub integer_dot_product64_bit_unsigned_accelerated: Bool32,
    #[doc(alias = "integerDotProduct64BitSignedAccelerated")]
    pub integer_dot_product64_bit_signed_accelerated: Bool32,
    #[doc(alias = "integerDotProduct64BitMixedSignednessAccelerated")]
    pub integer_dot_product64_bit_mixed_signedness_accelerated: Bool32,
    #[doc(alias = "integerDotProductAccumulatingSaturating8BitUnsignedAccelerated")]
    pub integer_dot_product_accumulating_saturating8_bit_unsigned_accelerated: Bool32,
    #[doc(alias = "integerDotProductAccumulatingSaturating8BitSignedAccelerated")]
    pub integer_dot_product_accumulating_saturating8_bit_signed_accelerated: Bool32,
    #[doc(alias = "integerDotProductAccumulatingSaturating8BitMixedSignednessAccelerated")]
    pub integer_dot_product_accumulating_saturating8_bit_mixed_signedness_accelerated: Bool32,
    #[doc(alias = "integerDotProductAccumulatingSaturating4x8BitPackedUnsignedAccelerated")]
    pub integer_dot_product_accumulating_saturating4x8_bit_packed_unsigned_accelerated: Bool32,
    #[doc(alias = "integerDotProductAccumulatingSaturating4x8BitPackedSignedAccelerated")]
    pub integer_dot_product_accumulating_saturating4x8_bit_packed_signed_accelerated: Bool32,
    #[doc(alias = "integerDotProductAccumulatingSaturating4x8BitPackedMixedSignednessAccelerated")]
    pub integer_dot_product_accumulating_saturating4x8_bit_packed_mixed_signedness_accelerated: Bool32,
    #[doc(alias = "integerDotProductAccumulatingSaturating16BitUnsignedAccelerated")]
    pub integer_dot_product_accumulating_saturating16_bit_unsigned_accelerated: Bool32,
    #[doc(alias = "integerDotProductAccumulatingSaturating16BitSignedAccelerated")]
    pub integer_dot_product_accumulating_saturating16_bit_signed_accelerated: Bool32,
    #[doc(alias = "integerDotProductAccumulatingSaturating16BitMixedSignednessAccelerated")]
    pub integer_dot_product_accumulating_saturating16_bit_mixed_signedness_accelerated: Bool32,
    #[doc(alias = "integerDotProductAccumulatingSaturating32BitUnsignedAccelerated")]
    pub integer_dot_product_accumulating_saturating32_bit_unsigned_accelerated: Bool32,
    #[doc(alias = "integerDotProductAccumulatingSaturating32BitSignedAccelerated")]
    pub integer_dot_product_accumulating_saturating32_bit_signed_accelerated: Bool32,
    #[doc(alias = "integerDotProductAccumulatingSaturating32BitMixedSignednessAccelerated")]
    pub integer_dot_product_accumulating_saturating32_bit_mixed_signedness_accelerated: Bool32,
    #[doc(alias = "integerDotProductAccumulatingSaturating64BitUnsignedAccelerated")]
    pub integer_dot_product_accumulating_saturating64_bit_unsigned_accelerated: Bool32,
    #[doc(alias = "integerDotProductAccumulatingSaturating64BitSignedAccelerated")]
    pub integer_dot_product_accumulating_saturating64_bit_signed_accelerated: Bool32,
    #[doc(alias = "integerDotProductAccumulatingSaturating64BitMixedSignednessAccelerated")]
    pub integer_dot_product_accumulating_saturating64_bit_mixed_signedness_accelerated: Bool32,
    #[doc(alias = "storageTexelBufferOffsetAlignmentBytes")]
    pub storage_texel_buffer_offset_alignment_bytes: DeviceSize,
    #[doc(alias = "storageTexelBufferOffsetSingleTexelAlignment")]
    pub storage_texel_buffer_offset_single_texel_alignment: Bool32,
    #[doc(alias = "uniformTexelBufferOffsetAlignmentBytes")]
    pub uniform_texel_buffer_offset_alignment_bytes: DeviceSize,
    #[doc(alias = "uniformTexelBufferOffsetSingleTexelAlignment")]
    pub uniform_texel_buffer_offset_single_texel_alignment: Bool32,
    #[doc(alias = "maxBufferSize")]
    pub max_buffer_size: DeviceSize,
}
impl Default for PhysicalDeviceVulkan13Properties {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceVulkan13Properties,
            p_next: unsafe { std::mem::zeroed() },
            min_subgroup_size: unsafe { std::mem::zeroed() },
            max_subgroup_size: unsafe { std::mem::zeroed() },
            max_compute_workgroup_subgroups: unsafe { std::mem::zeroed() },
            required_subgroup_size_stages: unsafe { std::mem::zeroed() },
            max_inline_uniform_block_size: unsafe { std::mem::zeroed() },
            max_per_stage_descriptor_inline_uniform_blocks: unsafe { std::mem::zeroed() },
            max_per_stage_descriptor_update_after_bind_inline_uniform_blocks: unsafe { std::mem::zeroed() },
            max_descriptor_set_inline_uniform_blocks: unsafe { std::mem::zeroed() },
            max_descriptor_set_update_after_bind_inline_uniform_blocks: unsafe { std::mem::zeroed() },
            max_inline_uniform_total_size: unsafe { std::mem::zeroed() },
            integer_dot_product8_bit_unsigned_accelerated: unsafe { std::mem::zeroed() },
            integer_dot_product8_bit_signed_accelerated: unsafe { std::mem::zeroed() },
            integer_dot_product8_bit_mixed_signedness_accelerated: unsafe { std::mem::zeroed() },
            integer_dot_product4x8_bit_packed_unsigned_accelerated: unsafe { std::mem::zeroed() },
            integer_dot_product4x8_bit_packed_signed_accelerated: unsafe { std::mem::zeroed() },
            integer_dot_product4x8_bit_packed_mixed_signedness_accelerated: unsafe { std::mem::zeroed() },
            integer_dot_product16_bit_unsigned_accelerated: unsafe { std::mem::zeroed() },
            integer_dot_product16_bit_signed_accelerated: unsafe { std::mem::zeroed() },
            integer_dot_product16_bit_mixed_signedness_accelerated: unsafe { std::mem::zeroed() },
            integer_dot_product32_bit_unsigned_accelerated: unsafe { std::mem::zeroed() },
            integer_dot_product32_bit_signed_accelerated: unsafe { std::mem::zeroed() },
            integer_dot_product32_bit_mixed_signedness_accelerated: unsafe { std::mem::zeroed() },
            integer_dot_product64_bit_unsigned_accelerated: unsafe { std::mem::zeroed() },
            integer_dot_product64_bit_signed_accelerated: unsafe { std::mem::zeroed() },
            integer_dot_product64_bit_mixed_signedness_accelerated: unsafe { std::mem::zeroed() },
            integer_dot_product_accumulating_saturating8_bit_unsigned_accelerated: unsafe { std::mem::zeroed() },
            integer_dot_product_accumulating_saturating8_bit_signed_accelerated: unsafe { std::mem::zeroed() },
            integer_dot_product_accumulating_saturating8_bit_mixed_signedness_accelerated: unsafe {
                std::mem::zeroed()
            },
            integer_dot_product_accumulating_saturating4x8_bit_packed_unsigned_accelerated: unsafe {
                std::mem::zeroed()
            },
            integer_dot_product_accumulating_saturating4x8_bit_packed_signed_accelerated: unsafe { std::mem::zeroed() },
            integer_dot_product_accumulating_saturating4x8_bit_packed_mixed_signedness_accelerated: unsafe {
                std::mem::zeroed()
            },
            integer_dot_product_accumulating_saturating16_bit_unsigned_accelerated: unsafe { std::mem::zeroed() },
            integer_dot_product_accumulating_saturating16_bit_signed_accelerated: unsafe { std::mem::zeroed() },
            integer_dot_product_accumulating_saturating16_bit_mixed_signedness_accelerated: unsafe {
                std::mem::zeroed()
            },
            integer_dot_product_accumulating_saturating32_bit_unsigned_accelerated: unsafe { std::mem::zeroed() },
            integer_dot_product_accumulating_saturating32_bit_signed_accelerated: unsafe { std::mem::zeroed() },
            integer_dot_product_accumulating_saturating32_bit_mixed_signedness_accelerated: unsafe {
                std::mem::zeroed()
            },
            integer_dot_product_accumulating_saturating64_bit_unsigned_accelerated: unsafe { std::mem::zeroed() },
            integer_dot_product_accumulating_saturating64_bit_signed_accelerated: unsafe { std::mem::zeroed() },
            integer_dot_product_accumulating_saturating64_bit_mixed_signedness_accelerated: unsafe {
                std::mem::zeroed()
            },
            storage_texel_buffer_offset_alignment_bytes: unsafe { std::mem::zeroed() },
            storage_texel_buffer_offset_single_texel_alignment: unsafe { std::mem::zeroed() },
            uniform_texel_buffer_offset_alignment_bytes: unsafe { std::mem::zeroed() },
            uniform_texel_buffer_offset_single_texel_alignment: unsafe { std::mem::zeroed() },
            max_buffer_size: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceToolProperties")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceToolProperties {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    pub name: [std::ffi::c_char; MAX_EXTENSION_NAME_SIZE as usize],
    pub version: [std::ffi::c_char; MAX_EXTENSION_NAME_SIZE as usize],
    pub purposes: ToolPurposeFlags,
    pub description: [std::ffi::c_char; MAX_DESCRIPTION_SIZE as usize],
    pub layer: [std::ffi::c_char; MAX_EXTENSION_NAME_SIZE as usize],
}
impl Default for PhysicalDeviceToolProperties {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceToolProperties,
            p_next: unsafe { std::mem::zeroed() },
            name: unsafe { std::mem::zeroed() },
            version: unsafe { std::mem::zeroed() },
            purposes: unsafe { std::mem::zeroed() },
            description: unsafe { std::mem::zeroed() },
            layer: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceZeroInitializeWorkgroupMemoryFeatures")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "shaderZeroInitializeWorkgroupMemory")]
    pub shader_zero_initialize_workgroup_memory: Bool32,
}
impl Default for PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures,
            p_next: unsafe { std::mem::zeroed() },
            shader_zero_initialize_workgroup_memory: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceImageRobustnessFeatures")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceImageRobustnessFeatures {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "robustImageAccess")]
    pub robust_image_access: Bool32,
}
impl Default for PhysicalDeviceImageRobustnessFeatures {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceImageRobustnessFeatures,
            p_next: unsafe { std::mem::zeroed() },
            robust_image_access: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkBufferCopy2")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct BufferCopy2 {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "srcOffset")]
    pub src_offset: DeviceSize,
    #[doc(alias = "dstOffset")]
    pub dst_offset: DeviceSize,
    pub size: DeviceSize,
}
impl Default for BufferCopy2 {
    fn default() -> Self {
        Self {
            s_type: StructureType::BufferCopy2,
            p_next: unsafe { std::mem::zeroed() },
            src_offset: unsafe { std::mem::zeroed() },
            dst_offset: unsafe { std::mem::zeroed() },
            size: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkImageCopy2")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ImageCopy2 {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "srcSubresource")]
    pub src_subresource: ImageSubresourceLayers,
    #[doc(alias = "srcOffset")]
    pub src_offset: Offset3D,
    #[doc(alias = "dstSubresource")]
    pub dst_subresource: ImageSubresourceLayers,
    #[doc(alias = "dstOffset")]
    pub dst_offset: Offset3D,
    pub extent: Extent3D,
}
impl Default for ImageCopy2 {
    fn default() -> Self {
        Self {
            s_type: StructureType::ImageCopy2,
            p_next: unsafe { std::mem::zeroed() },
            src_subresource: unsafe { std::mem::zeroed() },
            src_offset: unsafe { std::mem::zeroed() },
            dst_subresource: unsafe { std::mem::zeroed() },
            dst_offset: unsafe { std::mem::zeroed() },
            extent: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkImageBlit2")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ImageBlit2 {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "srcSubresource")]
    pub src_subresource: ImageSubresourceLayers,
    #[doc(alias = "srcOffsets")]
    pub src_offsets: [Offset3D; 2 as usize],
    #[doc(alias = "dstSubresource")]
    pub dst_subresource: ImageSubresourceLayers,
    #[doc(alias = "dstOffsets")]
    pub dst_offsets: [Offset3D; 2 as usize],
}
impl Default for ImageBlit2 {
    fn default() -> Self {
        Self {
            s_type: StructureType::ImageBlit2,
            p_next: unsafe { std::mem::zeroed() },
            src_subresource: unsafe { std::mem::zeroed() },
            src_offsets: unsafe { std::mem::zeroed() },
            dst_subresource: unsafe { std::mem::zeroed() },
            dst_offsets: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkBufferImageCopy2")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct BufferImageCopy2 {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "bufferOffset")]
    pub buffer_offset: DeviceSize,
    #[doc(alias = "bufferRowLength")]
    pub buffer_row_length: u32,
    #[doc(alias = "bufferImageHeight")]
    pub buffer_image_height: u32,
    #[doc(alias = "imageSubresource")]
    pub image_subresource: ImageSubresourceLayers,
    #[doc(alias = "imageOffset")]
    pub image_offset: Offset3D,
    #[doc(alias = "imageExtent")]
    pub image_extent: Extent3D,
}
impl Default for BufferImageCopy2 {
    fn default() -> Self {
        Self {
            s_type: StructureType::BufferImageCopy2,
            p_next: unsafe { std::mem::zeroed() },
            buffer_offset: unsafe { std::mem::zeroed() },
            buffer_row_length: unsafe { std::mem::zeroed() },
            buffer_image_height: unsafe { std::mem::zeroed() },
            image_subresource: unsafe { std::mem::zeroed() },
            image_offset: unsafe { std::mem::zeroed() },
            image_extent: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkImageResolve2")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ImageResolve2 {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "srcSubresource")]
    pub src_subresource: ImageSubresourceLayers,
    #[doc(alias = "srcOffset")]
    pub src_offset: Offset3D,
    #[doc(alias = "dstSubresource")]
    pub dst_subresource: ImageSubresourceLayers,
    #[doc(alias = "dstOffset")]
    pub dst_offset: Offset3D,
    pub extent: Extent3D,
}
impl Default for ImageResolve2 {
    fn default() -> Self {
        Self {
            s_type: StructureType::ImageResolve2,
            p_next: unsafe { std::mem::zeroed() },
            src_subresource: unsafe { std::mem::zeroed() },
            src_offset: unsafe { std::mem::zeroed() },
            dst_subresource: unsafe { std::mem::zeroed() },
            dst_offset: unsafe { std::mem::zeroed() },
            extent: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkCopyBufferInfo2")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct CopyBufferInfo2 {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "srcBuffer")]
    pub src_buffer: Buffer,
    #[doc(alias = "dstBuffer")]
    pub dst_buffer: Buffer,
    #[doc(alias = "regionCount")]
    pub region_count: u32,
    #[doc(alias = "pRegions")]
    pub regions: *const BufferCopy2,
}
impl Default for CopyBufferInfo2 {
    fn default() -> Self {
        Self {
            s_type: StructureType::CopyBufferInfo2,
            p_next: unsafe { std::mem::zeroed() },
            src_buffer: unsafe { std::mem::zeroed() },
            dst_buffer: unsafe { std::mem::zeroed() },
            region_count: unsafe { std::mem::zeroed() },
            regions: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkCopyImageInfo2")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct CopyImageInfo2 {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "srcImage")]
    pub src_image: Image,
    #[doc(alias = "srcImageLayout")]
    pub src_image_layout: ImageLayout,
    #[doc(alias = "dstImage")]
    pub dst_image: Image,
    #[doc(alias = "dstImageLayout")]
    pub dst_image_layout: ImageLayout,
    #[doc(alias = "regionCount")]
    pub region_count: u32,
    #[doc(alias = "pRegions")]
    pub regions: *const ImageCopy2,
}
impl Default for CopyImageInfo2 {
    fn default() -> Self {
        Self {
            s_type: StructureType::CopyImageInfo2,
            p_next: unsafe { std::mem::zeroed() },
            src_image: unsafe { std::mem::zeroed() },
            src_image_layout: unsafe { std::mem::zeroed() },
            dst_image: unsafe { std::mem::zeroed() },
            dst_image_layout: unsafe { std::mem::zeroed() },
            region_count: unsafe { std::mem::zeroed() },
            regions: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkBlitImageInfo2")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct BlitImageInfo2 {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "srcImage")]
    pub src_image: Image,
    #[doc(alias = "srcImageLayout")]
    pub src_image_layout: ImageLayout,
    #[doc(alias = "dstImage")]
    pub dst_image: Image,
    #[doc(alias = "dstImageLayout")]
    pub dst_image_layout: ImageLayout,
    #[doc(alias = "regionCount")]
    pub region_count: u32,
    #[doc(alias = "pRegions")]
    pub regions: *const ImageBlit2,
    pub filter: Filter,
}
impl Default for BlitImageInfo2 {
    fn default() -> Self {
        Self {
            s_type: StructureType::BlitImageInfo2,
            p_next: unsafe { std::mem::zeroed() },
            src_image: unsafe { std::mem::zeroed() },
            src_image_layout: unsafe { std::mem::zeroed() },
            dst_image: unsafe { std::mem::zeroed() },
            dst_image_layout: unsafe { std::mem::zeroed() },
            region_count: unsafe { std::mem::zeroed() },
            regions: unsafe { std::mem::zeroed() },
            filter: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkCopyBufferToImageInfo2")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct CopyBufferToImageInfo2 {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "srcBuffer")]
    pub src_buffer: Buffer,
    #[doc(alias = "dstImage")]
    pub dst_image: Image,
    #[doc(alias = "dstImageLayout")]
    pub dst_image_layout: ImageLayout,
    #[doc(alias = "regionCount")]
    pub region_count: u32,
    #[doc(alias = "pRegions")]
    pub regions: *const BufferImageCopy2,
}
impl Default for CopyBufferToImageInfo2 {
    fn default() -> Self {
        Self {
            s_type: StructureType::CopyBufferToImageInfo2,
            p_next: unsafe { std::mem::zeroed() },
            src_buffer: unsafe { std::mem::zeroed() },
            dst_image: unsafe { std::mem::zeroed() },
            dst_image_layout: unsafe { std::mem::zeroed() },
            region_count: unsafe { std::mem::zeroed() },
            regions: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkCopyImageToBufferInfo2")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct CopyImageToBufferInfo2 {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "srcImage")]
    pub src_image: Image,
    #[doc(alias = "srcImageLayout")]
    pub src_image_layout: ImageLayout,
    #[doc(alias = "dstBuffer")]
    pub dst_buffer: Buffer,
    #[doc(alias = "regionCount")]
    pub region_count: u32,
    #[doc(alias = "pRegions")]
    pub regions: *const BufferImageCopy2,
}
impl Default for CopyImageToBufferInfo2 {
    fn default() -> Self {
        Self {
            s_type: StructureType::CopyImageToBufferInfo2,
            p_next: unsafe { std::mem::zeroed() },
            src_image: unsafe { std::mem::zeroed() },
            src_image_layout: unsafe { std::mem::zeroed() },
            dst_buffer: unsafe { std::mem::zeroed() },
            region_count: unsafe { std::mem::zeroed() },
            regions: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkResolveImageInfo2")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ResolveImageInfo2 {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "srcImage")]
    pub src_image: Image,
    #[doc(alias = "srcImageLayout")]
    pub src_image_layout: ImageLayout,
    #[doc(alias = "dstImage")]
    pub dst_image: Image,
    #[doc(alias = "dstImageLayout")]
    pub dst_image_layout: ImageLayout,
    #[doc(alias = "regionCount")]
    pub region_count: u32,
    #[doc(alias = "pRegions")]
    pub regions: *const ImageResolve2,
}
impl Default for ResolveImageInfo2 {
    fn default() -> Self {
        Self {
            s_type: StructureType::ResolveImageInfo2,
            p_next: unsafe { std::mem::zeroed() },
            src_image: unsafe { std::mem::zeroed() },
            src_image_layout: unsafe { std::mem::zeroed() },
            dst_image: unsafe { std::mem::zeroed() },
            dst_image_layout: unsafe { std::mem::zeroed() },
            region_count: unsafe { std::mem::zeroed() },
            regions: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceShaderTerminateInvocationFeatures")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShaderTerminateInvocationFeatures {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "shaderTerminateInvocation")]
    pub shader_terminate_invocation: Bool32,
}
impl Default for PhysicalDeviceShaderTerminateInvocationFeatures {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceShaderTerminateInvocationFeatures,
            p_next: unsafe { std::mem::zeroed() },
            shader_terminate_invocation: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkMemoryBarrier2")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct MemoryBarrier2 {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "srcStageMask")]
    pub src_stage_mask: PipelineStageFlags2,
    #[doc(alias = "srcAccessMask")]
    pub src_access_mask: AccessFlags2,
    #[doc(alias = "dstStageMask")]
    pub dst_stage_mask: PipelineStageFlags2,
    #[doc(alias = "dstAccessMask")]
    pub dst_access_mask: AccessFlags2,
}
impl Default for MemoryBarrier2 {
    fn default() -> Self {
        Self {
            s_type: StructureType::MemoryBarrier2,
            p_next: unsafe { std::mem::zeroed() },
            src_stage_mask: unsafe { std::mem::zeroed() },
            src_access_mask: unsafe { std::mem::zeroed() },
            dst_stage_mask: unsafe { std::mem::zeroed() },
            dst_access_mask: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkImageMemoryBarrier2")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ImageMemoryBarrier2 {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "srcStageMask")]
    pub src_stage_mask: PipelineStageFlags2,
    #[doc(alias = "srcAccessMask")]
    pub src_access_mask: AccessFlags2,
    #[doc(alias = "dstStageMask")]
    pub dst_stage_mask: PipelineStageFlags2,
    #[doc(alias = "dstAccessMask")]
    pub dst_access_mask: AccessFlags2,
    #[doc(alias = "oldLayout")]
    pub old_layout: ImageLayout,
    #[doc(alias = "newLayout")]
    pub new_layout: ImageLayout,
    #[doc(alias = "srcQueueFamilyIndex")]
    pub src_queue_family_index: u32,
    #[doc(alias = "dstQueueFamilyIndex")]
    pub dst_queue_family_index: u32,
    pub image: Image,
    #[doc(alias = "subresourceRange")]
    pub subresource_range: ImageSubresourceRange,
}
impl Default for ImageMemoryBarrier2 {
    fn default() -> Self {
        Self {
            s_type: StructureType::ImageMemoryBarrier2,
            p_next: unsafe { std::mem::zeroed() },
            src_stage_mask: unsafe { std::mem::zeroed() },
            src_access_mask: unsafe { std::mem::zeroed() },
            dst_stage_mask: unsafe { std::mem::zeroed() },
            dst_access_mask: unsafe { std::mem::zeroed() },
            old_layout: unsafe { std::mem::zeroed() },
            new_layout: unsafe { std::mem::zeroed() },
            src_queue_family_index: unsafe { std::mem::zeroed() },
            dst_queue_family_index: unsafe { std::mem::zeroed() },
            image: unsafe { std::mem::zeroed() },
            subresource_range: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkBufferMemoryBarrier2")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct BufferMemoryBarrier2 {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "srcStageMask")]
    pub src_stage_mask: PipelineStageFlags2,
    #[doc(alias = "srcAccessMask")]
    pub src_access_mask: AccessFlags2,
    #[doc(alias = "dstStageMask")]
    pub dst_stage_mask: PipelineStageFlags2,
    #[doc(alias = "dstAccessMask")]
    pub dst_access_mask: AccessFlags2,
    #[doc(alias = "srcQueueFamilyIndex")]
    pub src_queue_family_index: u32,
    #[doc(alias = "dstQueueFamilyIndex")]
    pub dst_queue_family_index: u32,
    pub buffer: Buffer,
    pub offset: DeviceSize,
    pub size: DeviceSize,
}
impl Default for BufferMemoryBarrier2 {
    fn default() -> Self {
        Self {
            s_type: StructureType::BufferMemoryBarrier2,
            p_next: unsafe { std::mem::zeroed() },
            src_stage_mask: unsafe { std::mem::zeroed() },
            src_access_mask: unsafe { std::mem::zeroed() },
            dst_stage_mask: unsafe { std::mem::zeroed() },
            dst_access_mask: unsafe { std::mem::zeroed() },
            src_queue_family_index: unsafe { std::mem::zeroed() },
            dst_queue_family_index: unsafe { std::mem::zeroed() },
            buffer: unsafe { std::mem::zeroed() },
            offset: unsafe { std::mem::zeroed() },
            size: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkDependencyInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DependencyInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "dependencyFlags")]
    pub dependency_flags: DependencyFlags,
    #[doc(alias = "memoryBarrierCount")]
    pub memory_barrier_count: u32,
    #[doc(alias = "pMemoryBarriers")]
    pub memory_barriers: *const MemoryBarrier2,
    #[doc(alias = "bufferMemoryBarrierCount")]
    pub buffer_memory_barrier_count: u32,
    #[doc(alias = "pBufferMemoryBarriers")]
    pub buffer_memory_barriers: *const BufferMemoryBarrier2,
    #[doc(alias = "imageMemoryBarrierCount")]
    pub image_memory_barrier_count: u32,
    #[doc(alias = "pImageMemoryBarriers")]
    pub image_memory_barriers: *const ImageMemoryBarrier2,
}
impl Default for DependencyInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::DependencyInfo,
            p_next: unsafe { std::mem::zeroed() },
            dependency_flags: unsafe { std::mem::zeroed() },
            memory_barrier_count: unsafe { std::mem::zeroed() },
            memory_barriers: unsafe { std::mem::zeroed() },
            buffer_memory_barrier_count: unsafe { std::mem::zeroed() },
            buffer_memory_barriers: unsafe { std::mem::zeroed() },
            image_memory_barrier_count: unsafe { std::mem::zeroed() },
            image_memory_barriers: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkSemaphoreSubmitInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SemaphoreSubmitInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub semaphore: Semaphore,
    pub value: u64,
    #[doc(alias = "stageMask")]
    pub stage_mask: PipelineStageFlags2,
    #[doc(alias = "deviceIndex")]
    pub device_index: u32,
}
impl Default for SemaphoreSubmitInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::SemaphoreSubmitInfo,
            p_next: unsafe { std::mem::zeroed() },
            semaphore: unsafe { std::mem::zeroed() },
            value: unsafe { std::mem::zeroed() },
            stage_mask: unsafe { std::mem::zeroed() },
            device_index: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkCommandBufferSubmitInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct CommandBufferSubmitInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "commandBuffer")]
    pub command_buffer: CommandBuffer,
    #[doc(alias = "deviceMask")]
    pub device_mask: u32,
}
impl Default for CommandBufferSubmitInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::CommandBufferSubmitInfo,
            p_next: unsafe { std::mem::zeroed() },
            command_buffer: unsafe { std::mem::zeroed() },
            device_mask: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkSubmitInfo2")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SubmitInfo2 {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub flags: SubmitFlags,
    #[doc(alias = "waitSemaphoreInfoCount")]
    pub wait_semaphore_info_count: u32,
    #[doc(alias = "pWaitSemaphoreInfos")]
    pub wait_semaphore_infos: *const SemaphoreSubmitInfo,
    #[doc(alias = "commandBufferInfoCount")]
    pub command_buffer_info_count: u32,
    #[doc(alias = "pCommandBufferInfos")]
    pub command_buffer_infos: *const CommandBufferSubmitInfo,
    #[doc(alias = "signalSemaphoreInfoCount")]
    pub signal_semaphore_info_count: u32,
    #[doc(alias = "pSignalSemaphoreInfos")]
    pub signal_semaphore_infos: *const SemaphoreSubmitInfo,
}
impl Default for SubmitInfo2 {
    fn default() -> Self {
        Self {
            s_type: StructureType::SubmitInfo2,
            p_next: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
            wait_semaphore_info_count: unsafe { std::mem::zeroed() },
            wait_semaphore_infos: unsafe { std::mem::zeroed() },
            command_buffer_info_count: unsafe { std::mem::zeroed() },
            command_buffer_infos: unsafe { std::mem::zeroed() },
            signal_semaphore_info_count: unsafe { std::mem::zeroed() },
            signal_semaphore_infos: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceSynchronization2Features")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceSynchronization2Features {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    pub synchronization2: Bool32,
}
impl Default for PhysicalDeviceSynchronization2Features {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceSynchronization2Features,
            p_next: unsafe { std::mem::zeroed() },
            synchronization2: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceShaderIntegerDotProductFeatures")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShaderIntegerDotProductFeatures {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "shaderIntegerDotProduct")]
    pub shader_integer_dot_product: Bool32,
}
impl Default for PhysicalDeviceShaderIntegerDotProductFeatures {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceShaderIntegerDotProductFeatures,
            p_next: unsafe { std::mem::zeroed() },
            shader_integer_dot_product: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceShaderIntegerDotProductProperties")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShaderIntegerDotProductProperties {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "integerDotProduct8BitUnsignedAccelerated")]
    pub integer_dot_product8_bit_unsigned_accelerated: Bool32,
    #[doc(alias = "integerDotProduct8BitSignedAccelerated")]
    pub integer_dot_product8_bit_signed_accelerated: Bool32,
    #[doc(alias = "integerDotProduct8BitMixedSignednessAccelerated")]
    pub integer_dot_product8_bit_mixed_signedness_accelerated: Bool32,
    #[doc(alias = "integerDotProduct4x8BitPackedUnsignedAccelerated")]
    pub integer_dot_product4x8_bit_packed_unsigned_accelerated: Bool32,
    #[doc(alias = "integerDotProduct4x8BitPackedSignedAccelerated")]
    pub integer_dot_product4x8_bit_packed_signed_accelerated: Bool32,
    #[doc(alias = "integerDotProduct4x8BitPackedMixedSignednessAccelerated")]
    pub integer_dot_product4x8_bit_packed_mixed_signedness_accelerated: Bool32,
    #[doc(alias = "integerDotProduct16BitUnsignedAccelerated")]
    pub integer_dot_product16_bit_unsigned_accelerated: Bool32,
    #[doc(alias = "integerDotProduct16BitSignedAccelerated")]
    pub integer_dot_product16_bit_signed_accelerated: Bool32,
    #[doc(alias = "integerDotProduct16BitMixedSignednessAccelerated")]
    pub integer_dot_product16_bit_mixed_signedness_accelerated: Bool32,
    #[doc(alias = "integerDotProduct32BitUnsignedAccelerated")]
    pub integer_dot_product32_bit_unsigned_accelerated: Bool32,
    #[doc(alias = "integerDotProduct32BitSignedAccelerated")]
    pub integer_dot_product32_bit_signed_accelerated: Bool32,
    #[doc(alias = "integerDotProduct32BitMixedSignednessAccelerated")]
    pub integer_dot_product32_bit_mixed_signedness_accelerated: Bool32,
    #[doc(alias = "integerDotProduct64BitUnsignedAccelerated")]
    pub integer_dot_product64_bit_unsigned_accelerated: Bool32,
    #[doc(alias = "integerDotProduct64BitSignedAccelerated")]
    pub integer_dot_product64_bit_signed_accelerated: Bool32,
    #[doc(alias = "integerDotProduct64BitMixedSignednessAccelerated")]
    pub integer_dot_product64_bit_mixed_signedness_accelerated: Bool32,
    #[doc(alias = "integerDotProductAccumulatingSaturating8BitUnsignedAccelerated")]
    pub integer_dot_product_accumulating_saturating8_bit_unsigned_accelerated: Bool32,
    #[doc(alias = "integerDotProductAccumulatingSaturating8BitSignedAccelerated")]
    pub integer_dot_product_accumulating_saturating8_bit_signed_accelerated: Bool32,
    #[doc(alias = "integerDotProductAccumulatingSaturating8BitMixedSignednessAccelerated")]
    pub integer_dot_product_accumulating_saturating8_bit_mixed_signedness_accelerated: Bool32,
    #[doc(alias = "integerDotProductAccumulatingSaturating4x8BitPackedUnsignedAccelerated")]
    pub integer_dot_product_accumulating_saturating4x8_bit_packed_unsigned_accelerated: Bool32,
    #[doc(alias = "integerDotProductAccumulatingSaturating4x8BitPackedSignedAccelerated")]
    pub integer_dot_product_accumulating_saturating4x8_bit_packed_signed_accelerated: Bool32,
    #[doc(alias = "integerDotProductAccumulatingSaturating4x8BitPackedMixedSignednessAccelerated")]
    pub integer_dot_product_accumulating_saturating4x8_bit_packed_mixed_signedness_accelerated: Bool32,
    #[doc(alias = "integerDotProductAccumulatingSaturating16BitUnsignedAccelerated")]
    pub integer_dot_product_accumulating_saturating16_bit_unsigned_accelerated: Bool32,
    #[doc(alias = "integerDotProductAccumulatingSaturating16BitSignedAccelerated")]
    pub integer_dot_product_accumulating_saturating16_bit_signed_accelerated: Bool32,
    #[doc(alias = "integerDotProductAccumulatingSaturating16BitMixedSignednessAccelerated")]
    pub integer_dot_product_accumulating_saturating16_bit_mixed_signedness_accelerated: Bool32,
    #[doc(alias = "integerDotProductAccumulatingSaturating32BitUnsignedAccelerated")]
    pub integer_dot_product_accumulating_saturating32_bit_unsigned_accelerated: Bool32,
    #[doc(alias = "integerDotProductAccumulatingSaturating32BitSignedAccelerated")]
    pub integer_dot_product_accumulating_saturating32_bit_signed_accelerated: Bool32,
    #[doc(alias = "integerDotProductAccumulatingSaturating32BitMixedSignednessAccelerated")]
    pub integer_dot_product_accumulating_saturating32_bit_mixed_signedness_accelerated: Bool32,
    #[doc(alias = "integerDotProductAccumulatingSaturating64BitUnsignedAccelerated")]
    pub integer_dot_product_accumulating_saturating64_bit_unsigned_accelerated: Bool32,
    #[doc(alias = "integerDotProductAccumulatingSaturating64BitSignedAccelerated")]
    pub integer_dot_product_accumulating_saturating64_bit_signed_accelerated: Bool32,
    #[doc(alias = "integerDotProductAccumulatingSaturating64BitMixedSignednessAccelerated")]
    pub integer_dot_product_accumulating_saturating64_bit_mixed_signedness_accelerated: Bool32,
}
impl Default for PhysicalDeviceShaderIntegerDotProductProperties {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceShaderIntegerDotProductProperties,
            p_next: unsafe { std::mem::zeroed() },
            integer_dot_product8_bit_unsigned_accelerated: unsafe { std::mem::zeroed() },
            integer_dot_product8_bit_signed_accelerated: unsafe { std::mem::zeroed() },
            integer_dot_product8_bit_mixed_signedness_accelerated: unsafe { std::mem::zeroed() },
            integer_dot_product4x8_bit_packed_unsigned_accelerated: unsafe { std::mem::zeroed() },
            integer_dot_product4x8_bit_packed_signed_accelerated: unsafe { std::mem::zeroed() },
            integer_dot_product4x8_bit_packed_mixed_signedness_accelerated: unsafe { std::mem::zeroed() },
            integer_dot_product16_bit_unsigned_accelerated: unsafe { std::mem::zeroed() },
            integer_dot_product16_bit_signed_accelerated: unsafe { std::mem::zeroed() },
            integer_dot_product16_bit_mixed_signedness_accelerated: unsafe { std::mem::zeroed() },
            integer_dot_product32_bit_unsigned_accelerated: unsafe { std::mem::zeroed() },
            integer_dot_product32_bit_signed_accelerated: unsafe { std::mem::zeroed() },
            integer_dot_product32_bit_mixed_signedness_accelerated: unsafe { std::mem::zeroed() },
            integer_dot_product64_bit_unsigned_accelerated: unsafe { std::mem::zeroed() },
            integer_dot_product64_bit_signed_accelerated: unsafe { std::mem::zeroed() },
            integer_dot_product64_bit_mixed_signedness_accelerated: unsafe { std::mem::zeroed() },
            integer_dot_product_accumulating_saturating8_bit_unsigned_accelerated: unsafe { std::mem::zeroed() },
            integer_dot_product_accumulating_saturating8_bit_signed_accelerated: unsafe { std::mem::zeroed() },
            integer_dot_product_accumulating_saturating8_bit_mixed_signedness_accelerated: unsafe {
                std::mem::zeroed()
            },
            integer_dot_product_accumulating_saturating4x8_bit_packed_unsigned_accelerated: unsafe {
                std::mem::zeroed()
            },
            integer_dot_product_accumulating_saturating4x8_bit_packed_signed_accelerated: unsafe { std::mem::zeroed() },
            integer_dot_product_accumulating_saturating4x8_bit_packed_mixed_signedness_accelerated: unsafe {
                std::mem::zeroed()
            },
            integer_dot_product_accumulating_saturating16_bit_unsigned_accelerated: unsafe { std::mem::zeroed() },
            integer_dot_product_accumulating_saturating16_bit_signed_accelerated: unsafe { std::mem::zeroed() },
            integer_dot_product_accumulating_saturating16_bit_mixed_signedness_accelerated: unsafe {
                std::mem::zeroed()
            },
            integer_dot_product_accumulating_saturating32_bit_unsigned_accelerated: unsafe { std::mem::zeroed() },
            integer_dot_product_accumulating_saturating32_bit_signed_accelerated: unsafe { std::mem::zeroed() },
            integer_dot_product_accumulating_saturating32_bit_mixed_signedness_accelerated: unsafe {
                std::mem::zeroed()
            },
            integer_dot_product_accumulating_saturating64_bit_unsigned_accelerated: unsafe { std::mem::zeroed() },
            integer_dot_product_accumulating_saturating64_bit_signed_accelerated: unsafe { std::mem::zeroed() },
            integer_dot_product_accumulating_saturating64_bit_mixed_signedness_accelerated: unsafe {
                std::mem::zeroed()
            },
        }
    }
}
#[doc(alias = "VkFormatProperties3")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct FormatProperties3 {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "linearTilingFeatures")]
    pub linear_tiling_features: FormatFeatureFlags2,
    #[doc(alias = "optimalTilingFeatures")]
    pub optimal_tiling_features: FormatFeatureFlags2,
    #[doc(alias = "bufferFeatures")]
    pub buffer_features: FormatFeatureFlags2,
}
impl Default for FormatProperties3 {
    fn default() -> Self {
        Self {
            s_type: StructureType::FormatProperties3,
            p_next: unsafe { std::mem::zeroed() },
            linear_tiling_features: unsafe { std::mem::zeroed() },
            optimal_tiling_features: unsafe { std::mem::zeroed() },
            buffer_features: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPipelineRenderingCreateInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PipelineRenderingCreateInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "viewMask")]
    pub view_mask: u32,
    #[doc(alias = "colorAttachmentCount")]
    pub color_attachment_count: u32,
    #[doc(alias = "pColorAttachmentFormats")]
    pub color_attachment_formats: *const Format,
    #[doc(alias = "depthAttachmentFormat")]
    pub depth_attachment_format: Format,
    #[doc(alias = "stencilAttachmentFormat")]
    pub stencil_attachment_format: Format,
}
impl Default for PipelineRenderingCreateInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::PipelineRenderingCreateInfo,
            p_next: unsafe { std::mem::zeroed() },
            view_mask: unsafe { std::mem::zeroed() },
            color_attachment_count: unsafe { std::mem::zeroed() },
            color_attachment_formats: unsafe { std::mem::zeroed() },
            depth_attachment_format: unsafe { std::mem::zeroed() },
            stencil_attachment_format: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkRenderingInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct RenderingInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub flags: RenderingFlags,
    #[doc(alias = "renderArea")]
    pub render_area: Rect2D,
    #[doc(alias = "layerCount")]
    pub layer_count: u32,
    #[doc(alias = "viewMask")]
    pub view_mask: u32,
    #[doc(alias = "colorAttachmentCount")]
    pub color_attachment_count: u32,
    #[doc(alias = "pColorAttachments")]
    pub color_attachments: *const RenderingAttachmentInfo,
    #[doc(alias = "pDepthAttachment")]
    pub depth_attachment: *const RenderingAttachmentInfo,
    #[doc(alias = "pStencilAttachment")]
    pub stencil_attachment: *const RenderingAttachmentInfo,
}
impl Default for RenderingInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::RenderingInfo,
            p_next: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
            render_area: unsafe { std::mem::zeroed() },
            layer_count: unsafe { std::mem::zeroed() },
            view_mask: unsafe { std::mem::zeroed() },
            color_attachment_count: unsafe { std::mem::zeroed() },
            color_attachments: unsafe { std::mem::zeroed() },
            depth_attachment: unsafe { std::mem::zeroed() },
            stencil_attachment: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkRenderingAttachmentInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct RenderingAttachmentInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "imageView")]
    pub image_view: ImageView,
    #[doc(alias = "imageLayout")]
    pub image_layout: ImageLayout,
    #[doc(alias = "resolveMode")]
    pub resolve_mode: ResolveModeFlagBits,
    #[doc(alias = "resolveImageView")]
    pub resolve_image_view: ImageView,
    #[doc(alias = "resolveImageLayout")]
    pub resolve_image_layout: ImageLayout,
    #[doc(alias = "loadOp")]
    pub load_op: AttachmentLoadOp,
    #[doc(alias = "storeOp")]
    pub store_op: AttachmentStoreOp,
    #[doc(alias = "clearValue")]
    pub clear_value: ClearValue,
}
impl Default for RenderingAttachmentInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::RenderingAttachmentInfo,
            p_next: unsafe { std::mem::zeroed() },
            image_view: unsafe { std::mem::zeroed() },
            image_layout: unsafe { std::mem::zeroed() },
            resolve_mode: unsafe { std::mem::zeroed() },
            resolve_image_view: unsafe { std::mem::zeroed() },
            resolve_image_layout: unsafe { std::mem::zeroed() },
            load_op: unsafe { std::mem::zeroed() },
            store_op: unsafe { std::mem::zeroed() },
            clear_value: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceDynamicRenderingFeatures")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceDynamicRenderingFeatures {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "dynamicRendering")]
    pub dynamic_rendering: Bool32,
}
impl Default for PhysicalDeviceDynamicRenderingFeatures {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceDynamicRenderingFeatures,
            p_next: unsafe { std::mem::zeroed() },
            dynamic_rendering: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkCommandBufferInheritanceRenderingInfo")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct CommandBufferInheritanceRenderingInfo {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub flags: RenderingFlags,
    #[doc(alias = "viewMask")]
    pub view_mask: u32,
    #[doc(alias = "colorAttachmentCount")]
    pub color_attachment_count: u32,
    #[doc(alias = "pColorAttachmentFormats")]
    pub color_attachment_formats: *const Format,
    #[doc(alias = "depthAttachmentFormat")]
    pub depth_attachment_format: Format,
    #[doc(alias = "stencilAttachmentFormat")]
    pub stencil_attachment_format: Format,
    #[doc(alias = "rasterizationSamples")]
    pub rasterization_samples: SampleCountFlagBits,
}
impl Default for CommandBufferInheritanceRenderingInfo {
    fn default() -> Self {
        Self {
            s_type: StructureType::CommandBufferInheritanceRenderingInfo,
            p_next: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
            view_mask: unsafe { std::mem::zeroed() },
            color_attachment_count: unsafe { std::mem::zeroed() },
            color_attachment_formats: unsafe { std::mem::zeroed() },
            depth_attachment_format: unsafe { std::mem::zeroed() },
            stencil_attachment_format: unsafe { std::mem::zeroed() },
            rasterization_samples: unsafe { std::mem::zeroed() },
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(alias = "VkPrivateDataSlot")]
#[repr(transparent)]
pub struct PrivateDataSlot(u64);
impl PrivateDataSlot {
    pub const fn null() -> Self {
        Self(0)
    }
    pub const fn raw(&self) -> u64 {
        self.0
    }
}
impl Default for PrivateDataSlot {
    fn default() -> Self {
        Self::null()
    }
}
pub use crate::common::vulkan1_3::{
    AccessFlagBits2, AccessFlags2, FormatFeatureFlagBits2, FormatFeatureFlags2, PipelineCreationFeedbackFlagBits,
    PipelineCreationFeedbackFlags, PipelineStageFlagBits2, PipelineStageFlags2, PrivateDataSlotCreateFlags,
    RenderingFlagBits, RenderingFlags, SubmitFlagBits, SubmitFlags, ToolPurposeFlagBits, ToolPurposeFlags,
};
#[doc(alias = "vkGetDeviceBufferMemoryRequirements")]
pub type FNGetDeviceBufferMemoryRequirements = unsafe extern "system" fn(
    device: Device,
    p_info: *const DeviceBufferMemoryRequirements,
    p_memory_requirements: *mut MemoryRequirements2,
);
#[doc(alias = "vkGetDeviceImageMemoryRequirements")]
pub type FNGetDeviceImageMemoryRequirements = unsafe extern "system" fn(
    device: Device,
    p_info: *const DeviceImageMemoryRequirements,
    p_memory_requirements: *mut MemoryRequirements2,
);
#[doc(alias = "vkGetDeviceImageSparseMemoryRequirements")]
pub type FNGetDeviceImageSparseMemoryRequirements = unsafe extern "system" fn(
    device: Device,
    p_info: *const DeviceImageMemoryRequirements,
    p_sparse_memory_requirement_count: *mut u32,
    p_sparse_memory_requirements: *mut SparseImageMemoryRequirements2,
);
#[doc(alias = "vkGetPhysicalDeviceToolProperties")]
pub type FNGetPhysicalDeviceToolProperties = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_tool_count: *mut u32,
    p_tool_properties: *mut PhysicalDeviceToolProperties,
) -> VulkanResultCodes;
#[doc(alias = "vkCreatePrivateDataSlot")]
pub type FNCreatePrivateDataSlot = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const PrivateDataSlotCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_private_data_slot: *mut PrivateDataSlot,
) -> VulkanResultCodes;
#[doc(alias = "vkDestroyPrivateDataSlot")]
pub type FNDestroyPrivateDataSlot = unsafe extern "system" fn(
    device: Device,
    private_data_slot: PrivateDataSlot,
    p_allocator: *const AllocationCallbacks,
);
#[doc(alias = "vkSetPrivateData")]
pub type FNSetPrivateData = unsafe extern "system" fn(
    device: Device,
    object_type: ObjectType,
    object_handle: u64,
    private_data_slot: PrivateDataSlot,
    data: u64,
) -> VulkanResultCodes;
#[doc(alias = "vkGetPrivateData")]
pub type FNGetPrivateData = unsafe extern "system" fn(
    device: Device,
    object_type: ObjectType,
    object_handle: u64,
    private_data_slot: PrivateDataSlot,
    p_data: *mut u64,
);
#[doc(alias = "vkQueueSubmit2")]
pub type FNQueueSubmit2 = unsafe extern "system" fn(
    queue: Queue,
    submit_count: u32,
    p_submits: *const SubmitInfo2,
    fence: Fence,
) -> VulkanResultCodes;
#[doc(alias = "vkCmdSetCullMode")]
pub type FNCmdSetCullMode = unsafe extern "system" fn(command_buffer: CommandBuffer, cull_mode: CullModeFlags);
#[doc(alias = "vkCmdSetFrontFace")]
pub type FNCmdSetFrontFace = unsafe extern "system" fn(command_buffer: CommandBuffer, front_face: FrontFace);
#[doc(alias = "vkCmdSetPrimitiveTopology")]
pub type FNCmdSetPrimitiveTopology =
    unsafe extern "system" fn(command_buffer: CommandBuffer, primitive_topology: PrimitiveTopology);
#[doc(alias = "vkCmdSetViewportWithCount")]
pub type FNCmdSetViewportWithCount =
    unsafe extern "system" fn(command_buffer: CommandBuffer, viewport_count: u32, p_viewports: *const Viewport);
#[doc(alias = "vkCmdSetScissorWithCount")]
pub type FNCmdSetScissorWithCount =
    unsafe extern "system" fn(command_buffer: CommandBuffer, scissor_count: u32, p_scissors: *const Rect2D);
#[doc(alias = "vkCmdBindVertexBuffers2")]
pub type FNCmdBindVertexBuffers2 = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    first_binding: u32,
    binding_count: u32,
    p_buffers: *const Buffer,
    p_offsets: *const DeviceSize,
    p_sizes: *const DeviceSize,
    p_strides: *const DeviceSize,
);
#[doc(alias = "vkCmdSetDepthTestEnable")]
pub type FNCmdSetDepthTestEnable = unsafe extern "system" fn(command_buffer: CommandBuffer, depth_test_enable: Bool32);
#[doc(alias = "vkCmdSetDepthWriteEnable")]
pub type FNCmdSetDepthWriteEnable =
    unsafe extern "system" fn(command_buffer: CommandBuffer, depth_write_enable: Bool32);
#[doc(alias = "vkCmdSetDepthCompareOp")]
pub type FNCmdSetDepthCompareOp = unsafe extern "system" fn(command_buffer: CommandBuffer, depth_compare_op: CompareOp);
#[doc(alias = "vkCmdSetDepthBoundsTestEnable")]
pub type FNCmdSetDepthBoundsTestEnable =
    unsafe extern "system" fn(command_buffer: CommandBuffer, depth_bounds_test_enable: Bool32);
#[doc(alias = "vkCmdSetStencilTestEnable")]
pub type FNCmdSetStencilTestEnable =
    unsafe extern "system" fn(command_buffer: CommandBuffer, stencil_test_enable: Bool32);
#[doc(alias = "vkCmdSetStencilOp")]
pub type FNCmdSetStencilOp = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    face_mask: StencilFaceFlags,
    fail_op: StencilOp,
    pass_op: StencilOp,
    depth_fail_op: StencilOp,
    compare_op: CompareOp,
);
#[doc(alias = "vkCmdSetRasterizerDiscardEnable")]
pub type FNCmdSetRasterizerDiscardEnable =
    unsafe extern "system" fn(command_buffer: CommandBuffer, rasterizer_discard_enable: Bool32);
#[doc(alias = "vkCmdSetDepthBiasEnable")]
pub type FNCmdSetDepthBiasEnable = unsafe extern "system" fn(command_buffer: CommandBuffer, depth_bias_enable: Bool32);
#[doc(alias = "vkCmdSetPrimitiveRestartEnable")]
pub type FNCmdSetPrimitiveRestartEnable =
    unsafe extern "system" fn(command_buffer: CommandBuffer, primitive_restart_enable: Bool32);
#[doc(alias = "vkCmdCopyBuffer2")]
pub type FNCmdCopyBuffer2 =
    unsafe extern "system" fn(command_buffer: CommandBuffer, p_copy_buffer_info: *const CopyBufferInfo2);
#[doc(alias = "vkCmdCopyImage2")]
pub type FNCmdCopyImage2 =
    unsafe extern "system" fn(command_buffer: CommandBuffer, p_copy_image_info: *const CopyImageInfo2);
#[doc(alias = "vkCmdBlitImage2")]
pub type FNCmdBlitImage2 =
    unsafe extern "system" fn(command_buffer: CommandBuffer, p_blit_image_info: *const BlitImageInfo2);
#[doc(alias = "vkCmdCopyBufferToImage2")]
pub type FNCmdCopyBufferToImage2 = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_copy_buffer_to_image_info: *const CopyBufferToImageInfo2,
);
#[doc(alias = "vkCmdCopyImageToBuffer2")]
pub type FNCmdCopyImageToBuffer2 = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_copy_image_to_buffer_info: *const CopyImageToBufferInfo2,
);
#[doc(alias = "vkCmdResolveImage2")]
pub type FNCmdResolveImage2 =
    unsafe extern "system" fn(command_buffer: CommandBuffer, p_resolve_image_info: *const ResolveImageInfo2);
#[doc(alias = "vkCmdSetEvent2")]
pub type FNCmdSetEvent2 =
    unsafe extern "system" fn(command_buffer: CommandBuffer, event: Event, p_dependency_info: *const DependencyInfo);
#[doc(alias = "vkCmdResetEvent2")]
pub type FNCmdResetEvent2 =
    unsafe extern "system" fn(command_buffer: CommandBuffer, event: Event, stage_mask: PipelineStageFlags2);
#[doc(alias = "vkCmdWaitEvents2")]
pub type FNCmdWaitEvents2 = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    event_count: u32,
    p_events: *const Event,
    p_dependency_infos: *const DependencyInfo,
);
#[doc(alias = "vkCmdPipelineBarrier2")]
pub type FNCmdPipelineBarrier2 =
    unsafe extern "system" fn(command_buffer: CommandBuffer, p_dependency_info: *const DependencyInfo);
#[doc(alias = "vkCmdWriteTimestamp2")]
pub type FNCmdWriteTimestamp2 = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    stage: PipelineStageFlags2,
    query_pool: QueryPool,
    query: u32,
);
#[doc(alias = "vkCmdBeginRendering")]
pub type FNCmdBeginRendering =
    unsafe extern "system" fn(command_buffer: CommandBuffer, p_rendering_info: *const RenderingInfo);
#[doc(alias = "vkCmdEndRendering")]
pub type FNCmdEndRendering = unsafe extern "system" fn(command_buffer: CommandBuffer);
