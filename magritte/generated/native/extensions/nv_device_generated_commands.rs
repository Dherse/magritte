pub use crate::common::extensions::nv_device_generated_commands::{
    BindIndexBufferIndirectCommandNV, BindShaderGroupIndirectCommandNV, BindVertexBufferIndirectCommandNV,
    SetStateFlagsIndirectCommandNV,
};
use crate::native::{
    vulkan1_0::{
        AllocationCallbacks, BaseInStructure, BaseOutStructure, Bool32, Buffer, CommandBuffer, Device, DeviceSize,
        IndexType, Pipeline, PipelineBindPoint, PipelineLayout, PipelineShaderStageCreateInfo,
        PipelineTessellationStateCreateInfo, PipelineVertexInputStateCreateInfo, ShaderStageFlags, StructureType,
        VulkanResultCodes,
    },
    vulkan1_1::MemoryRequirements2,
};
#[doc(alias = "VkPhysicalDeviceDeviceGeneratedCommandsFeaturesNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceDeviceGeneratedCommandsFeaturesNV {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "deviceGeneratedCommands")]
    pub device_generated_commands: Bool32,
}
impl Default for PhysicalDeviceDeviceGeneratedCommandsFeaturesNV {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceDeviceGeneratedCommandsFeaturesNv,
            p_next: unsafe { std::mem::zeroed() },
            device_generated_commands: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceDeviceGeneratedCommandsPropertiesNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceDeviceGeneratedCommandsPropertiesNV {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "maxGraphicsShaderGroupCount")]
    pub max_graphics_shader_group_count: u32,
    #[doc(alias = "maxIndirectSequenceCount")]
    pub max_indirect_sequence_count: u32,
    #[doc(alias = "maxIndirectCommandsTokenCount")]
    pub max_indirect_commands_token_count: u32,
    #[doc(alias = "maxIndirectCommandsStreamCount")]
    pub max_indirect_commands_stream_count: u32,
    #[doc(alias = "maxIndirectCommandsTokenOffset")]
    pub max_indirect_commands_token_offset: u32,
    #[doc(alias = "maxIndirectCommandsStreamStride")]
    pub max_indirect_commands_stream_stride: u32,
    #[doc(alias = "minSequencesCountBufferOffsetAlignment")]
    pub min_sequences_count_buffer_offset_alignment: u32,
    #[doc(alias = "minSequencesIndexBufferOffsetAlignment")]
    pub min_sequences_index_buffer_offset_alignment: u32,
    #[doc(alias = "minIndirectCommandsBufferOffsetAlignment")]
    pub min_indirect_commands_buffer_offset_alignment: u32,
}
impl Default for PhysicalDeviceDeviceGeneratedCommandsPropertiesNV {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceDeviceGeneratedCommandsPropertiesNv,
            p_next: unsafe { std::mem::zeroed() },
            max_graphics_shader_group_count: unsafe { std::mem::zeroed() },
            max_indirect_sequence_count: unsafe { std::mem::zeroed() },
            max_indirect_commands_token_count: unsafe { std::mem::zeroed() },
            max_indirect_commands_stream_count: unsafe { std::mem::zeroed() },
            max_indirect_commands_token_offset: unsafe { std::mem::zeroed() },
            max_indirect_commands_stream_stride: unsafe { std::mem::zeroed() },
            min_sequences_count_buffer_offset_alignment: unsafe { std::mem::zeroed() },
            min_sequences_index_buffer_offset_alignment: unsafe { std::mem::zeroed() },
            min_indirect_commands_buffer_offset_alignment: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkGraphicsShaderGroupCreateInfoNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct GraphicsShaderGroupCreateInfoNV {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "stageCount")]
    pub stage_count: u32,
    #[doc(alias = "pStages")]
    pub stages: *const PipelineShaderStageCreateInfo,
    #[doc(alias = "pVertexInputState")]
    pub vertex_input_state: *const PipelineVertexInputStateCreateInfo,
    #[doc(alias = "pTessellationState")]
    pub tessellation_state: *const PipelineTessellationStateCreateInfo,
}
impl Default for GraphicsShaderGroupCreateInfoNV {
    fn default() -> Self {
        Self {
            s_type: StructureType::GraphicsShaderGroupCreateInfoNv,
            p_next: unsafe { std::mem::zeroed() },
            stage_count: unsafe { std::mem::zeroed() },
            stages: unsafe { std::mem::zeroed() },
            vertex_input_state: unsafe { std::mem::zeroed() },
            tessellation_state: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkGraphicsPipelineShaderGroupsCreateInfoNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct GraphicsPipelineShaderGroupsCreateInfoNV {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "groupCount")]
    pub group_count: u32,
    #[doc(alias = "pGroups")]
    pub groups: *const GraphicsShaderGroupCreateInfoNV,
    #[doc(alias = "pipelineCount")]
    pub pipeline_count: u32,
    #[doc(alias = "pPipelines")]
    pub pipelines: *const Pipeline,
}
impl Default for GraphicsPipelineShaderGroupsCreateInfoNV {
    fn default() -> Self {
        Self {
            s_type: StructureType::GraphicsPipelineShaderGroupsCreateInfoNv,
            p_next: unsafe { std::mem::zeroed() },
            group_count: unsafe { std::mem::zeroed() },
            groups: unsafe { std::mem::zeroed() },
            pipeline_count: unsafe { std::mem::zeroed() },
            pipelines: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkIndirectCommandsStreamNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct IndirectCommandsStreamNV {
    pub buffer: Buffer,
    pub offset: DeviceSize,
}
impl Default for IndirectCommandsStreamNV {
    fn default() -> Self {
        Self {
            buffer: unsafe { std::mem::zeroed() },
            offset: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkIndirectCommandsLayoutTokenNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct IndirectCommandsLayoutTokenNV {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "tokenType")]
    pub token_type: IndirectCommandsTokenTypeNV,
    pub stream: u32,
    pub offset: u32,
    #[doc(alias = "vertexBindingUnit")]
    pub vertex_binding_unit: u32,
    #[doc(alias = "vertexDynamicStride")]
    pub vertex_dynamic_stride: Bool32,
    #[doc(alias = "pushconstantPipelineLayout")]
    pub pushconstant_pipeline_layout: PipelineLayout,
    #[doc(alias = "pushconstantShaderStageFlags")]
    pub pushconstant_shader_stage_flags: ShaderStageFlags,
    #[doc(alias = "pushconstantOffset")]
    pub pushconstant_offset: u32,
    #[doc(alias = "pushconstantSize")]
    pub pushconstant_size: u32,
    #[doc(alias = "indirectStateFlags")]
    pub indirect_state_flags: IndirectStateFlagsNV,
    #[doc(alias = "indexTypeCount")]
    pub index_type_count: u32,
    #[doc(alias = "pIndexTypes")]
    pub index_types: *const IndexType,
    #[doc(alias = "pIndexTypeValues")]
    pub index_type_values: *const u32,
}
impl Default for IndirectCommandsLayoutTokenNV {
    fn default() -> Self {
        Self {
            s_type: StructureType::IndirectCommandsLayoutTokenNv,
            p_next: unsafe { std::mem::zeroed() },
            token_type: unsafe { std::mem::zeroed() },
            stream: unsafe { std::mem::zeroed() },
            offset: unsafe { std::mem::zeroed() },
            vertex_binding_unit: unsafe { std::mem::zeroed() },
            vertex_dynamic_stride: unsafe { std::mem::zeroed() },
            pushconstant_pipeline_layout: unsafe { std::mem::zeroed() },
            pushconstant_shader_stage_flags: unsafe { std::mem::zeroed() },
            pushconstant_offset: unsafe { std::mem::zeroed() },
            pushconstant_size: unsafe { std::mem::zeroed() },
            indirect_state_flags: unsafe { std::mem::zeroed() },
            index_type_count: unsafe { std::mem::zeroed() },
            index_types: unsafe { std::mem::zeroed() },
            index_type_values: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkIndirectCommandsLayoutCreateInfoNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct IndirectCommandsLayoutCreateInfoNV {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub flags: IndirectCommandsLayoutUsageFlagsNV,
    #[doc(alias = "pipelineBindPoint")]
    pub pipeline_bind_point: PipelineBindPoint,
    #[doc(alias = "tokenCount")]
    pub token_count: u32,
    #[doc(alias = "pTokens")]
    pub tokens: *const IndirectCommandsLayoutTokenNV,
    #[doc(alias = "streamCount")]
    pub stream_count: u32,
    #[doc(alias = "pStreamStrides")]
    pub stream_strides: *const u32,
}
impl Default for IndirectCommandsLayoutCreateInfoNV {
    fn default() -> Self {
        Self {
            s_type: StructureType::IndirectCommandsLayoutCreateInfoNv,
            p_next: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
            pipeline_bind_point: unsafe { std::mem::zeroed() },
            token_count: unsafe { std::mem::zeroed() },
            tokens: unsafe { std::mem::zeroed() },
            stream_count: unsafe { std::mem::zeroed() },
            stream_strides: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkGeneratedCommandsInfoNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct GeneratedCommandsInfoNV {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "pipelineBindPoint")]
    pub pipeline_bind_point: PipelineBindPoint,
    pub pipeline: Pipeline,
    #[doc(alias = "indirectCommandsLayout")]
    pub indirect_commands_layout: IndirectCommandsLayoutNV,
    #[doc(alias = "streamCount")]
    pub stream_count: u32,
    #[doc(alias = "pStreams")]
    pub streams: *const IndirectCommandsStreamNV,
    #[doc(alias = "sequencesCount")]
    pub sequences_count: u32,
    #[doc(alias = "preprocessBuffer")]
    pub preprocess_buffer: Buffer,
    #[doc(alias = "preprocessOffset")]
    pub preprocess_offset: DeviceSize,
    #[doc(alias = "preprocessSize")]
    pub preprocess_size: DeviceSize,
    #[doc(alias = "sequencesCountBuffer")]
    pub sequences_count_buffer: Buffer,
    #[doc(alias = "sequencesCountOffset")]
    pub sequences_count_offset: DeviceSize,
    #[doc(alias = "sequencesIndexBuffer")]
    pub sequences_index_buffer: Buffer,
    #[doc(alias = "sequencesIndexOffset")]
    pub sequences_index_offset: DeviceSize,
}
impl Default for GeneratedCommandsInfoNV {
    fn default() -> Self {
        Self {
            s_type: StructureType::GeneratedCommandsInfoNv,
            p_next: unsafe { std::mem::zeroed() },
            pipeline_bind_point: unsafe { std::mem::zeroed() },
            pipeline: unsafe { std::mem::zeroed() },
            indirect_commands_layout: unsafe { std::mem::zeroed() },
            stream_count: unsafe { std::mem::zeroed() },
            streams: unsafe { std::mem::zeroed() },
            sequences_count: unsafe { std::mem::zeroed() },
            preprocess_buffer: unsafe { std::mem::zeroed() },
            preprocess_offset: unsafe { std::mem::zeroed() },
            preprocess_size: unsafe { std::mem::zeroed() },
            sequences_count_buffer: unsafe { std::mem::zeroed() },
            sequences_count_offset: unsafe { std::mem::zeroed() },
            sequences_index_buffer: unsafe { std::mem::zeroed() },
            sequences_index_offset: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkGeneratedCommandsMemoryRequirementsInfoNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct GeneratedCommandsMemoryRequirementsInfoNV {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "pipelineBindPoint")]
    pub pipeline_bind_point: PipelineBindPoint,
    pub pipeline: Pipeline,
    #[doc(alias = "indirectCommandsLayout")]
    pub indirect_commands_layout: IndirectCommandsLayoutNV,
    #[doc(alias = "maxSequencesCount")]
    pub max_sequences_count: u32,
}
impl Default for GeneratedCommandsMemoryRequirementsInfoNV {
    fn default() -> Self {
        Self {
            s_type: StructureType::GeneratedCommandsMemoryRequirementsInfoNv,
            p_next: unsafe { std::mem::zeroed() },
            pipeline_bind_point: unsafe { std::mem::zeroed() },
            pipeline: unsafe { std::mem::zeroed() },
            indirect_commands_layout: unsafe { std::mem::zeroed() },
            max_sequences_count: unsafe { std::mem::zeroed() },
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(alias = "VkIndirectCommandsLayoutNV")]
#[repr(transparent)]
pub struct IndirectCommandsLayoutNV(u64);
impl IndirectCommandsLayoutNV {
    pub const fn null() -> Self {
        Self(0)
    }
    pub const fn raw(&self) -> u64 {
        self.0
    }
}
impl Default for IndirectCommandsLayoutNV {
    fn default() -> Self {
        Self::null()
    }
}
pub use crate::common::extensions::nv_device_generated_commands::{
    IndirectCommandsLayoutUsageFlagBitsNV, IndirectCommandsLayoutUsageFlagsNV, IndirectCommandsTokenTypeNV,
    IndirectStateFlagBitsNV, IndirectStateFlagsNV, NV_DEVICE_GENERATED_COMMANDS_EXTENSION_NAME,
    NV_DEVICE_GENERATED_COMMANDS_SPEC_VERSION,
};
#[doc(alias = "vkGetGeneratedCommandsMemoryRequirementsNV")]
pub type FNGetGeneratedCommandsMemoryRequirementsNv = unsafe extern "system" fn(
    device: Device,
    p_info: *const GeneratedCommandsMemoryRequirementsInfoNV,
    p_memory_requirements: *mut MemoryRequirements2,
);
#[doc(alias = "vkCreateIndirectCommandsLayoutNV")]
pub type FNCreateIndirectCommandsLayoutNv = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const IndirectCommandsLayoutCreateInfoNV,
    p_allocator: *const AllocationCallbacks,
    p_indirect_commands_layout: *mut IndirectCommandsLayoutNV,
) -> VulkanResultCodes;
#[doc(alias = "vkDestroyIndirectCommandsLayoutNV")]
pub type FNDestroyIndirectCommandsLayoutNv = unsafe extern "system" fn(
    device: Device,
    indirect_commands_layout: IndirectCommandsLayoutNV,
    p_allocator: *const AllocationCallbacks,
);
#[doc(alias = "vkCmdExecuteGeneratedCommandsNV")]
pub type FNCmdExecuteGeneratedCommandsNv = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    is_preprocessed: Bool32,
    p_generated_commands_info: *const GeneratedCommandsInfoNV,
);
#[doc(alias = "vkCmdPreprocessGeneratedCommandsNV")]
pub type FNCmdPreprocessGeneratedCommandsNv =
    unsafe extern "system" fn(command_buffer: CommandBuffer, p_generated_commands_info: *const GeneratedCommandsInfoNV);
#[doc(alias = "vkCmdBindPipelineShaderGroupNV")]
pub type FNCmdBindPipelineShaderGroupNv = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    pipeline_bind_point: PipelineBindPoint,
    pipeline: Pipeline,
    group_index: u32,
);
