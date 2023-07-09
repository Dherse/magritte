pub use crate::common::extensions::nv_device_generated_commands::{
    BindIndexBufferIndirectCommandNV, BindShaderGroupIndirectCommandNV, BindVertexBufferIndirectCommandNV,
    IndirectCommandsLayoutUsageFlagBitsNV, IndirectCommandsLayoutUsageFlagsNV, IndirectCommandsTokenTypeNV,
    IndirectStateFlagBitsNV, IndirectStateFlagsNV, SetStateFlagsIndirectCommandNV,
    NV_DEVICE_GENERATED_COMMANDS_EXTENSION_NAME, NV_DEVICE_GENERATED_COMMANDS_SPEC_VERSION,
};
use crate::{
    context::{Container, Context, Error, ObjectId},
    vulkan1_0::{
        Buffer, DeviceAddress, DeviceSize, IndexType, Pipeline, PipelineBindPoint, PipelineLayout,
        PipelineShaderStageCreateInfo, PipelineTessellationStateCreateInfo, PipelineVertexInputStateCreateInfo,
        ShaderStageFlags, StructureType,
    },
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use std::sync::Arc;
#[doc(alias = "VkPhysicalDeviceDeviceGeneratedCommandsFeaturesNV")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceDeviceGeneratedCommandsFeaturesNV {
    #[doc(alias = "deviceGeneratedCommands")]
    pub device_generated_commands: bool,
}
impl PhysicalDeviceDeviceGeneratedCommandsFeaturesNV {
    ///Get a reference to the `device_generated_commands` field.
    pub fn device_generated_commands(&self) -> &bool {
        &self.device_generated_commands
    }
    ///Get a mutable reference to the `device_generated_commands` field.
    pub fn device_generated_commands_mut(&mut self) -> &mut bool {
        &mut self.device_generated_commands
    }
    ///Sets the `device_generated_commands` field.
    pub fn set_device_generated_commands(&mut self, device_generated_commands: bool) -> &mut Self {
        self.device_generated_commands = device_generated_commands;
        self
    }
    ///Sets the `device_generated_commands` field in a builder way.
    pub fn with_device_generated_commands(mut self, device_generated_commands: bool) -> Self {
        self.device_generated_commands = device_generated_commands;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceDeviceGeneratedCommandsFeaturesNV {
    type LowLevel =
        crate::native::extensions::nv_device_generated_commands::PhysicalDeviceDeviceGeneratedCommandsFeaturesNV;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::nv_device_generated_commands::PhysicalDeviceDeviceGeneratedCommandsFeaturesNV {
            s_type: StructureType::PhysicalDeviceDeviceGeneratedCommandsFeaturesNv,
            p_next: std::ptr::null_mut(),
            device_generated_commands: self.device_generated_commands.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceDeviceGeneratedCommandsFeaturesNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            device_generated_commands: crate::conv::FromLowLevel::from_low_level(
                context,
                value.device_generated_commands,
            ),
        }
    }
}
#[doc(alias = "VkPhysicalDeviceDeviceGeneratedCommandsPropertiesNV")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceDeviceGeneratedCommandsPropertiesNV {
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
impl PhysicalDeviceDeviceGeneratedCommandsPropertiesNV {
    ///Get a reference to the `max_graphics_shader_group_count` field.
    pub fn max_graphics_shader_group_count(&self) -> u32 {
        self.max_graphics_shader_group_count
    }
    ///Get a reference to the `max_indirect_sequence_count` field.
    pub fn max_indirect_sequence_count(&self) -> u32 {
        self.max_indirect_sequence_count
    }
    ///Get a reference to the `max_indirect_commands_token_count` field.
    pub fn max_indirect_commands_token_count(&self) -> u32 {
        self.max_indirect_commands_token_count
    }
    ///Get a reference to the `max_indirect_commands_stream_count` field.
    pub fn max_indirect_commands_stream_count(&self) -> u32 {
        self.max_indirect_commands_stream_count
    }
    ///Get a reference to the `max_indirect_commands_token_offset` field.
    pub fn max_indirect_commands_token_offset(&self) -> u32 {
        self.max_indirect_commands_token_offset
    }
    ///Get a reference to the `max_indirect_commands_stream_stride` field.
    pub fn max_indirect_commands_stream_stride(&self) -> u32 {
        self.max_indirect_commands_stream_stride
    }
    ///Get a reference to the `min_sequences_count_buffer_offset_alignment` field.
    pub fn min_sequences_count_buffer_offset_alignment(&self) -> u32 {
        self.min_sequences_count_buffer_offset_alignment
    }
    ///Get a reference to the `min_sequences_index_buffer_offset_alignment` field.
    pub fn min_sequences_index_buffer_offset_alignment(&self) -> u32 {
        self.min_sequences_index_buffer_offset_alignment
    }
    ///Get a reference to the `min_indirect_commands_buffer_offset_alignment` field.
    pub fn min_indirect_commands_buffer_offset_alignment(&self) -> u32 {
        self.min_indirect_commands_buffer_offset_alignment
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceDeviceGeneratedCommandsPropertiesNV {
    type LowLevel =
        crate::native::extensions::nv_device_generated_commands::PhysicalDeviceDeviceGeneratedCommandsPropertiesNV;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::nv_device_generated_commands::PhysicalDeviceDeviceGeneratedCommandsPropertiesNV {
            s_type: StructureType::PhysicalDeviceDeviceGeneratedCommandsPropertiesNv,
            p_next: std::ptr::null_mut(),
            max_graphics_shader_group_count: self.max_graphics_shader_group_count.into_low_level(context, bump),
            max_indirect_sequence_count: self.max_indirect_sequence_count.into_low_level(context, bump),
            max_indirect_commands_token_count: self.max_indirect_commands_token_count.into_low_level(context, bump),
            max_indirect_commands_stream_count: self.max_indirect_commands_stream_count.into_low_level(context, bump),
            max_indirect_commands_token_offset: self.max_indirect_commands_token_offset.into_low_level(context, bump),
            max_indirect_commands_stream_stride: self.max_indirect_commands_stream_stride.into_low_level(context, bump),
            min_sequences_count_buffer_offset_alignment: self
                .min_sequences_count_buffer_offset_alignment
                .into_low_level(context, bump),
            min_sequences_index_buffer_offset_alignment: self
                .min_sequences_index_buffer_offset_alignment
                .into_low_level(context, bump),
            min_indirect_commands_buffer_offset_alignment: self
                .min_indirect_commands_buffer_offset_alignment
                .into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceDeviceGeneratedCommandsPropertiesNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            max_graphics_shader_group_count: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_graphics_shader_group_count,
            ),
            max_indirect_sequence_count: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_indirect_sequence_count,
            ),
            max_indirect_commands_token_count: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_indirect_commands_token_count,
            ),
            max_indirect_commands_stream_count: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_indirect_commands_stream_count,
            ),
            max_indirect_commands_token_offset: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_indirect_commands_token_offset,
            ),
            max_indirect_commands_stream_stride: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_indirect_commands_stream_stride,
            ),
            min_sequences_count_buffer_offset_alignment: crate::conv::FromLowLevel::from_low_level(
                context,
                value.min_sequences_count_buffer_offset_alignment,
            ),
            min_sequences_index_buffer_offset_alignment: crate::conv::FromLowLevel::from_low_level(
                context,
                value.min_sequences_index_buffer_offset_alignment,
            ),
            min_indirect_commands_buffer_offset_alignment: crate::conv::FromLowLevel::from_low_level(
                context,
                value.min_indirect_commands_buffer_offset_alignment,
            ),
        }
    }
}
#[doc(alias = "VkGraphicsShaderGroupCreateInfoNV")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GraphicsShaderGroupCreateInfoNV {
    #[doc(alias = "pStages")]
    pub stages: SmallVec<[PipelineShaderStageCreateInfo; 8]>,
    #[doc(alias = "pVertexInputState")]
    pub vertex_input_state: Option<PipelineVertexInputStateCreateInfo>,
    #[doc(alias = "pTessellationState")]
    pub tessellation_state: Option<PipelineTessellationStateCreateInfo>,
}
impl GraphicsShaderGroupCreateInfoNV {
    ///Get a reference to the `stages` field.
    pub fn stages(&self) -> &SmallVec<[PipelineShaderStageCreateInfo; 8]> {
        &self.stages
    }
    ///Get a reference to the `vertex_input_state` field.
    pub fn vertex_input_state(&self) -> &Option<PipelineVertexInputStateCreateInfo> {
        &self.vertex_input_state
    }
    ///Get a reference to the `tessellation_state` field.
    pub fn tessellation_state(&self) -> &Option<PipelineTessellationStateCreateInfo> {
        &self.tessellation_state
    }
    ///Get a mutable reference to the `stages` field.
    pub fn stages_mut(&mut self) -> &mut SmallVec<[PipelineShaderStageCreateInfo; 8]> {
        &mut self.stages
    }
    ///Get a mutable reference to the `vertex_input_state` field.
    pub fn vertex_input_state_mut(&mut self) -> &mut Option<PipelineVertexInputStateCreateInfo> {
        &mut self.vertex_input_state
    }
    ///Get a mutable reference to the `tessellation_state` field.
    pub fn tessellation_state_mut(&mut self) -> &mut Option<PipelineTessellationStateCreateInfo> {
        &mut self.tessellation_state
    }
    ///Sets the `stages` field.
    pub fn set_stages(&mut self, stages: SmallVec<[PipelineShaderStageCreateInfo; 8]>) -> &mut Self {
        self.stages = stages;
        self
    }
    ///Sets the `vertex_input_state` field.
    pub fn set_vertex_input_state(
        &mut self,
        vertex_input_state: Option<PipelineVertexInputStateCreateInfo>,
    ) -> &mut Self {
        self.vertex_input_state = vertex_input_state;
        self
    }
    ///Sets the `tessellation_state` field.
    pub fn set_tessellation_state(
        &mut self,
        tessellation_state: Option<PipelineTessellationStateCreateInfo>,
    ) -> &mut Self {
        self.tessellation_state = tessellation_state;
        self
    }
    ///Sets the `stages` field in a builder way.
    pub fn with_stages(mut self, stages: SmallVec<[PipelineShaderStageCreateInfo; 8]>) -> Self {
        self.stages = stages;
        self
    }
    ///Sets the `vertex_input_state` field in a builder way.
    pub fn with_vertex_input_state(mut self, vertex_input_state: Option<PipelineVertexInputStateCreateInfo>) -> Self {
        self.vertex_input_state = vertex_input_state;
        self
    }
    ///Sets the `tessellation_state` field in a builder way.
    pub fn with_tessellation_state(mut self, tessellation_state: Option<PipelineTessellationStateCreateInfo>) -> Self {
        self.tessellation_state = tessellation_state;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for GraphicsShaderGroupCreateInfoNV {
    type LowLevel = crate::native::extensions::nv_device_generated_commands::GraphicsShaderGroupCreateInfoNV;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let len_stages = self.stages.len() as u32;
        let stages = bump
            .alloc_slice_fill_iter(self.stages.iter().map(|x| x.into_low_level(context, bump)))
            .as_ptr()
            .cast();
        crate::native::extensions::nv_device_generated_commands::GraphicsShaderGroupCreateInfoNV {
            s_type: StructureType::GraphicsShaderGroupCreateInfoNv,
            p_next: std::ptr::null(),
            stage_count: len_stages,
            stages: stages,
            vertex_input_state: self
                .vertex_input_state
                .as_ref()
                .map(|v| bump.alloc(v.into_low_level(context, bump)) as *const _)
                .unwrap_or_else(std::ptr::null),
            tessellation_state: self
                .tessellation_state
                .as_ref()
                .map(|v| bump.alloc(v.into_low_level(context, bump)) as *const _)
                .unwrap_or_else(std::ptr::null),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for GraphicsShaderGroupCreateInfoNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let stages_len = value.stage_count;
        let mut stages = SmallVec::with_capacity(stages_len as usize);
        for i in 0..stages_len {
            stages.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.stages.add(i as usize).read(),
            ));
        }
        Self {
            stages: stages,
            vertex_input_state: crate::conv::FromLowLevel::from_low_level(context, *value.vertex_input_state),
            tessellation_state: crate::conv::FromLowLevel::from_low_level(context, *value.tessellation_state),
        }
    }
}
#[doc(alias = "VkGraphicsPipelineShaderGroupsCreateInfoNV")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GraphicsPipelineShaderGroupsCreateInfoNV {
    #[doc(alias = "pGroups")]
    pub groups: SmallVec<[GraphicsShaderGroupCreateInfoNV; 8]>,
    #[doc(alias = "pPipelines")]
    pub pipelines: SmallVec<[Pipeline; 8]>,
}
impl GraphicsPipelineShaderGroupsCreateInfoNV {
    ///Get a reference to the `groups` field.
    pub fn groups(&self) -> &SmallVec<[GraphicsShaderGroupCreateInfoNV; 8]> {
        &self.groups
    }
    ///Get a reference to the `pipelines` field.
    pub fn pipelines(&self) -> &SmallVec<[Pipeline; 8]> {
        &self.pipelines
    }
    ///Get a mutable reference to the `groups` field.
    pub fn groups_mut(&mut self) -> &mut SmallVec<[GraphicsShaderGroupCreateInfoNV; 8]> {
        &mut self.groups
    }
    ///Get a mutable reference to the `pipelines` field.
    pub fn pipelines_mut(&mut self) -> &mut SmallVec<[Pipeline; 8]> {
        &mut self.pipelines
    }
    ///Sets the `groups` field.
    pub fn set_groups(&mut self, groups: SmallVec<[GraphicsShaderGroupCreateInfoNV; 8]>) -> &mut Self {
        self.groups = groups;
        self
    }
    ///Sets the `pipelines` field.
    pub fn set_pipelines(&mut self, pipelines: SmallVec<[Pipeline; 8]>) -> &mut Self {
        self.pipelines = pipelines;
        self
    }
    ///Sets the `groups` field in a builder way.
    pub fn with_groups(mut self, groups: SmallVec<[GraphicsShaderGroupCreateInfoNV; 8]>) -> Self {
        self.groups = groups;
        self
    }
    ///Sets the `pipelines` field in a builder way.
    pub fn with_pipelines(mut self, pipelines: SmallVec<[Pipeline; 8]>) -> Self {
        self.pipelines = pipelines;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for GraphicsPipelineShaderGroupsCreateInfoNV {
    type LowLevel = crate::native::extensions::nv_device_generated_commands::GraphicsPipelineShaderGroupsCreateInfoNV;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let len_groups = self.groups.len() as u32;
        let groups = bump
            .alloc_slice_fill_iter(self.groups.iter().map(|x| x.into_low_level(context, bump)))
            .as_ptr()
            .cast();
        let len_pipelines = self.pipelines.len() as u32;
        let pipelines = bump
            .alloc_slice_fill_iter(self.pipelines.iter().map(|x| x.into_low_level(context, bump)))
            .as_ptr()
            .cast();
        crate::native::extensions::nv_device_generated_commands::GraphicsPipelineShaderGroupsCreateInfoNV {
            s_type: StructureType::GraphicsPipelineShaderGroupsCreateInfoNv,
            p_next: std::ptr::null(),
            group_count: len_groups,
            groups: groups,
            pipeline_count: len_pipelines,
            pipelines: pipelines,
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for GraphicsPipelineShaderGroupsCreateInfoNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let groups_len = value.group_count;
        let mut groups = SmallVec::with_capacity(groups_len as usize);
        for i in 0..groups_len {
            groups.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.groups.add(i as usize).read(),
            ));
        }
        let pipelines_len = value.pipeline_count;
        let mut pipelines = SmallVec::with_capacity(pipelines_len as usize);
        for i in 0..pipelines_len {
            pipelines.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.pipelines.add(i as usize).read(),
            ));
        }
        Self {
            groups: groups,
            pipelines: pipelines,
        }
    }
}
impl BindShaderGroupIndirectCommandNV {
    ///Get a reference to the `group_index` field.
    pub fn group_index(&self) -> u32 {
        self.group_index
    }
    ///Get a mutable reference to the `group_index` field.
    pub fn group_index_mut(&mut self) -> &mut u32 {
        &mut self.group_index
    }
    ///Sets the `group_index` field.
    pub fn set_group_index(&mut self, group_index: u32) -> &mut Self {
        self.group_index = group_index;
        self
    }
    ///Sets the `group_index` field in a builder way.
    pub fn with_group_index(mut self, group_index: u32) -> Self {
        self.group_index = group_index;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for BindShaderGroupIndirectCommandNV {
    type LowLevel = crate::native::extensions::nv_device_generated_commands::BindShaderGroupIndirectCommandNV;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::nv_device_generated_commands::BindShaderGroupIndirectCommandNV {
            group_index: self.group_index.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for BindShaderGroupIndirectCommandNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            group_index: crate::conv::FromLowLevel::from_low_level(context, value.group_index),
        }
    }
}
impl BindIndexBufferIndirectCommandNV {
    ///Get a reference to the `buffer_address` field.
    pub fn buffer_address(&self) -> &DeviceAddress {
        &self.buffer_address
    }
    ///Get a reference to the `size` field.
    pub fn size(&self) -> u32 {
        self.size
    }
    ///Get a reference to the `index_type` field.
    pub fn index_type(&self) -> IndexType {
        self.index_type
    }
    ///Get a mutable reference to the `buffer_address` field.
    pub fn buffer_address_mut(&mut self) -> &mut DeviceAddress {
        &mut self.buffer_address
    }
    ///Get a mutable reference to the `size` field.
    pub fn size_mut(&mut self) -> &mut u32 {
        &mut self.size
    }
    ///Get a mutable reference to the `index_type` field.
    pub fn index_type_mut(&mut self) -> &mut IndexType {
        &mut self.index_type
    }
    ///Sets the `buffer_address` field.
    pub fn set_buffer_address(&mut self, buffer_address: DeviceAddress) -> &mut Self {
        self.buffer_address = buffer_address;
        self
    }
    ///Sets the `size` field.
    pub fn set_size(&mut self, size: u32) -> &mut Self {
        self.size = size;
        self
    }
    ///Sets the `index_type` field.
    pub fn set_index_type(&mut self, index_type: IndexType) -> &mut Self {
        self.index_type = index_type;
        self
    }
    ///Sets the `buffer_address` field in a builder way.
    pub fn with_buffer_address(mut self, buffer_address: DeviceAddress) -> Self {
        self.buffer_address = buffer_address;
        self
    }
    ///Sets the `size` field in a builder way.
    pub fn with_size(mut self, size: u32) -> Self {
        self.size = size;
        self
    }
    ///Sets the `index_type` field in a builder way.
    pub fn with_index_type(mut self, index_type: IndexType) -> Self {
        self.index_type = index_type;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for BindIndexBufferIndirectCommandNV {
    type LowLevel = crate::native::extensions::nv_device_generated_commands::BindIndexBufferIndirectCommandNV;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::nv_device_generated_commands::BindIndexBufferIndirectCommandNV {
            buffer_address: self.buffer_address.into_low_level(context, bump),
            size: self.size.into_low_level(context, bump),
            index_type: self.index_type.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for BindIndexBufferIndirectCommandNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            buffer_address: crate::conv::FromLowLevel::from_low_level(context, value.buffer_address),
            size: crate::conv::FromLowLevel::from_low_level(context, value.size),
            index_type: crate::conv::FromLowLevel::from_low_level(context, value.index_type),
        }
    }
}
impl BindVertexBufferIndirectCommandNV {
    ///Get a reference to the `buffer_address` field.
    pub fn buffer_address(&self) -> &DeviceAddress {
        &self.buffer_address
    }
    ///Get a reference to the `size` field.
    pub fn size(&self) -> u32 {
        self.size
    }
    ///Get a reference to the `stride` field.
    pub fn stride(&self) -> u32 {
        self.stride
    }
    ///Get a mutable reference to the `buffer_address` field.
    pub fn buffer_address_mut(&mut self) -> &mut DeviceAddress {
        &mut self.buffer_address
    }
    ///Get a mutable reference to the `size` field.
    pub fn size_mut(&mut self) -> &mut u32 {
        &mut self.size
    }
    ///Get a mutable reference to the `stride` field.
    pub fn stride_mut(&mut self) -> &mut u32 {
        &mut self.stride
    }
    ///Sets the `buffer_address` field.
    pub fn set_buffer_address(&mut self, buffer_address: DeviceAddress) -> &mut Self {
        self.buffer_address = buffer_address;
        self
    }
    ///Sets the `size` field.
    pub fn set_size(&mut self, size: u32) -> &mut Self {
        self.size = size;
        self
    }
    ///Sets the `stride` field.
    pub fn set_stride(&mut self, stride: u32) -> &mut Self {
        self.stride = stride;
        self
    }
    ///Sets the `buffer_address` field in a builder way.
    pub fn with_buffer_address(mut self, buffer_address: DeviceAddress) -> Self {
        self.buffer_address = buffer_address;
        self
    }
    ///Sets the `size` field in a builder way.
    pub fn with_size(mut self, size: u32) -> Self {
        self.size = size;
        self
    }
    ///Sets the `stride` field in a builder way.
    pub fn with_stride(mut self, stride: u32) -> Self {
        self.stride = stride;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for BindVertexBufferIndirectCommandNV {
    type LowLevel = crate::native::extensions::nv_device_generated_commands::BindVertexBufferIndirectCommandNV;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::nv_device_generated_commands::BindVertexBufferIndirectCommandNV {
            buffer_address: self.buffer_address.into_low_level(context, bump),
            size: self.size.into_low_level(context, bump),
            stride: self.stride.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for BindVertexBufferIndirectCommandNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            buffer_address: crate::conv::FromLowLevel::from_low_level(context, value.buffer_address),
            size: crate::conv::FromLowLevel::from_low_level(context, value.size),
            stride: crate::conv::FromLowLevel::from_low_level(context, value.stride),
        }
    }
}
impl SetStateFlagsIndirectCommandNV {
    ///Get a reference to the `data` field.
    pub fn data(&self) -> u32 {
        self.data
    }
    ///Get a mutable reference to the `data` field.
    pub fn data_mut(&mut self) -> &mut u32 {
        &mut self.data
    }
    ///Sets the `data` field.
    pub fn set_data(&mut self, data: u32) -> &mut Self {
        self.data = data;
        self
    }
    ///Sets the `data` field in a builder way.
    pub fn with_data(mut self, data: u32) -> Self {
        self.data = data;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for SetStateFlagsIndirectCommandNV {
    type LowLevel = crate::native::extensions::nv_device_generated_commands::SetStateFlagsIndirectCommandNV;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::nv_device_generated_commands::SetStateFlagsIndirectCommandNV {
            data: self.data.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for SetStateFlagsIndirectCommandNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            data: crate::conv::FromLowLevel::from_low_level(context, value.data),
        }
    }
}
#[doc(alias = "VkIndirectCommandsStreamNV")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct IndirectCommandsStreamNV {
    pub buffer: Buffer,
    pub offset: DeviceSize,
}
impl IndirectCommandsStreamNV {
    ///Get a reference to the `buffer` field.
    pub fn buffer(&self) -> &Buffer {
        &self.buffer
    }
    ///Get a reference to the `offset` field.
    pub fn offset(&self) -> &DeviceSize {
        &self.offset
    }
    ///Get a mutable reference to the `buffer` field.
    pub fn buffer_mut(&mut self) -> &mut Buffer {
        &mut self.buffer
    }
    ///Get a mutable reference to the `offset` field.
    pub fn offset_mut(&mut self) -> &mut DeviceSize {
        &mut self.offset
    }
    ///Sets the `buffer` field.
    pub fn set_buffer(&mut self, buffer: Buffer) -> &mut Self {
        self.buffer = buffer;
        self
    }
    ///Sets the `offset` field.
    pub fn set_offset(&mut self, offset: DeviceSize) -> &mut Self {
        self.offset = offset;
        self
    }
    ///Sets the `buffer` field in a builder way.
    pub fn with_buffer(mut self, buffer: Buffer) -> Self {
        self.buffer = buffer;
        self
    }
    ///Sets the `offset` field in a builder way.
    pub fn with_offset(mut self, offset: DeviceSize) -> Self {
        self.offset = offset;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for IndirectCommandsStreamNV {
    type LowLevel = crate::native::extensions::nv_device_generated_commands::IndirectCommandsStreamNV;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::nv_device_generated_commands::IndirectCommandsStreamNV {
            buffer: self.buffer.into_low_level(context, bump),
            offset: self.offset.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for IndirectCommandsStreamNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            buffer: crate::conv::FromLowLevel::from_low_level(context, value.buffer),
            offset: crate::conv::FromLowLevel::from_low_level(context, value.offset),
        }
    }
}
#[doc(alias = "VkIndirectCommandsLayoutTokenNV")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct IndirectCommandsLayoutTokenNV {
    #[doc(alias = "tokenType")]
    pub token_type: IndirectCommandsTokenTypeNV,
    pub stream: u32,
    pub offset: u32,
    #[doc(alias = "vertexBindingUnit")]
    pub vertex_binding_unit: u32,
    #[doc(alias = "vertexDynamicStride")]
    pub vertex_dynamic_stride: bool,
    #[doc(alias = "pushconstantPipelineLayout")]
    pub pushconstant_pipeline_layout: Option<PipelineLayout>,
    #[doc(alias = "pushconstantShaderStageFlags")]
    pub pushconstant_shader_stage_flags: ShaderStageFlags,
    #[doc(alias = "pushconstantOffset")]
    pub pushconstant_offset: u32,
    #[doc(alias = "pushconstantSize")]
    pub pushconstant_size: u32,
    #[doc(alias = "indirectStateFlags")]
    pub indirect_state_flags: IndirectStateFlagsNV,
    #[doc(alias = "pIndexTypes")]
    pub index_types: SmallVec<[IndexType; 8]>,
    #[doc(alias = "pIndexTypeValues")]
    pub index_type_values: SmallVec<[u32; 8]>,
}
impl IndirectCommandsLayoutTokenNV {
    ///Get a reference to the `token_type` field.
    pub fn token_type(&self) -> IndirectCommandsTokenTypeNV {
        self.token_type
    }
    ///Get a reference to the `stream` field.
    pub fn stream(&self) -> u32 {
        self.stream
    }
    ///Get a reference to the `offset` field.
    pub fn offset(&self) -> u32 {
        self.offset
    }
    ///Get a reference to the `vertex_binding_unit` field.
    pub fn vertex_binding_unit(&self) -> u32 {
        self.vertex_binding_unit
    }
    ///Get a reference to the `vertex_dynamic_stride` field.
    pub fn vertex_dynamic_stride(&self) -> &bool {
        &self.vertex_dynamic_stride
    }
    ///Get a reference to the `pushconstant_pipeline_layout` field.
    pub fn pushconstant_pipeline_layout(&self) -> &Option<PipelineLayout> {
        &self.pushconstant_pipeline_layout
    }
    ///Get a reference to the `pushconstant_shader_stage_flags` field.
    pub fn pushconstant_shader_stage_flags(&self) -> ShaderStageFlags {
        self.pushconstant_shader_stage_flags
    }
    ///Get a reference to the `pushconstant_offset` field.
    pub fn pushconstant_offset(&self) -> u32 {
        self.pushconstant_offset
    }
    ///Get a reference to the `pushconstant_size` field.
    pub fn pushconstant_size(&self) -> u32 {
        self.pushconstant_size
    }
    ///Get a reference to the `indirect_state_flags` field.
    pub fn indirect_state_flags(&self) -> IndirectStateFlagsNV {
        self.indirect_state_flags
    }
    ///Get a reference to the `index_types` field.
    pub fn index_types(&self) -> &SmallVec<[IndexType; 8]> {
        &self.index_types
    }
    ///Get a reference to the `index_type_values` field.
    pub fn index_type_values(&self) -> &SmallVec<[u32; 8]> {
        &self.index_type_values
    }
    ///Get a mutable reference to the `token_type` field.
    pub fn token_type_mut(&mut self) -> &mut IndirectCommandsTokenTypeNV {
        &mut self.token_type
    }
    ///Get a mutable reference to the `stream` field.
    pub fn stream_mut(&mut self) -> &mut u32 {
        &mut self.stream
    }
    ///Get a mutable reference to the `offset` field.
    pub fn offset_mut(&mut self) -> &mut u32 {
        &mut self.offset
    }
    ///Get a mutable reference to the `vertex_binding_unit` field.
    pub fn vertex_binding_unit_mut(&mut self) -> &mut u32 {
        &mut self.vertex_binding_unit
    }
    ///Get a mutable reference to the `vertex_dynamic_stride` field.
    pub fn vertex_dynamic_stride_mut(&mut self) -> &mut bool {
        &mut self.vertex_dynamic_stride
    }
    ///Get a mutable reference to the `pushconstant_pipeline_layout` field.
    pub fn pushconstant_pipeline_layout_mut(&mut self) -> &mut Option<PipelineLayout> {
        &mut self.pushconstant_pipeline_layout
    }
    ///Get a mutable reference to the `pushconstant_shader_stage_flags` field.
    pub fn pushconstant_shader_stage_flags_mut(&mut self) -> &mut ShaderStageFlags {
        &mut self.pushconstant_shader_stage_flags
    }
    ///Get a mutable reference to the `pushconstant_offset` field.
    pub fn pushconstant_offset_mut(&mut self) -> &mut u32 {
        &mut self.pushconstant_offset
    }
    ///Get a mutable reference to the `pushconstant_size` field.
    pub fn pushconstant_size_mut(&mut self) -> &mut u32 {
        &mut self.pushconstant_size
    }
    ///Get a mutable reference to the `indirect_state_flags` field.
    pub fn indirect_state_flags_mut(&mut self) -> &mut IndirectStateFlagsNV {
        &mut self.indirect_state_flags
    }
    ///Get a mutable reference to the `index_types` field.
    pub fn index_types_mut(&mut self) -> &mut SmallVec<[IndexType; 8]> {
        &mut self.index_types
    }
    ///Get a mutable reference to the `index_type_values` field.
    pub fn index_type_values_mut(&mut self) -> &mut SmallVec<[u32; 8]> {
        &mut self.index_type_values
    }
    ///Sets the `token_type` field.
    pub fn set_token_type(&mut self, token_type: IndirectCommandsTokenTypeNV) -> &mut Self {
        self.token_type = token_type;
        self
    }
    ///Sets the `stream` field.
    pub fn set_stream(&mut self, stream: u32) -> &mut Self {
        self.stream = stream;
        self
    }
    ///Sets the `offset` field.
    pub fn set_offset(&mut self, offset: u32) -> &mut Self {
        self.offset = offset;
        self
    }
    ///Sets the `vertex_binding_unit` field.
    pub fn set_vertex_binding_unit(&mut self, vertex_binding_unit: u32) -> &mut Self {
        self.vertex_binding_unit = vertex_binding_unit;
        self
    }
    ///Sets the `vertex_dynamic_stride` field.
    pub fn set_vertex_dynamic_stride(&mut self, vertex_dynamic_stride: bool) -> &mut Self {
        self.vertex_dynamic_stride = vertex_dynamic_stride;
        self
    }
    ///Sets the `pushconstant_pipeline_layout` field.
    pub fn set_pushconstant_pipeline_layout(
        &mut self,
        pushconstant_pipeline_layout: Option<PipelineLayout>,
    ) -> &mut Self {
        self.pushconstant_pipeline_layout = pushconstant_pipeline_layout;
        self
    }
    ///Sets the `pushconstant_shader_stage_flags` field.
    pub fn set_pushconstant_shader_stage_flags(
        &mut self,
        pushconstant_shader_stage_flags: ShaderStageFlags,
    ) -> &mut Self {
        self.pushconstant_shader_stage_flags = pushconstant_shader_stage_flags;
        self
    }
    ///Sets the `pushconstant_offset` field.
    pub fn set_pushconstant_offset(&mut self, pushconstant_offset: u32) -> &mut Self {
        self.pushconstant_offset = pushconstant_offset;
        self
    }
    ///Sets the `pushconstant_size` field.
    pub fn set_pushconstant_size(&mut self, pushconstant_size: u32) -> &mut Self {
        self.pushconstant_size = pushconstant_size;
        self
    }
    ///Sets the `indirect_state_flags` field.
    pub fn set_indirect_state_flags(&mut self, indirect_state_flags: IndirectStateFlagsNV) -> &mut Self {
        self.indirect_state_flags = indirect_state_flags;
        self
    }
    ///Sets the `index_types` field.
    pub fn set_index_types(&mut self, index_types: SmallVec<[IndexType; 8]>) -> &mut Self {
        self.index_types = index_types;
        self
    }
    ///Sets the `index_type_values` field.
    pub fn set_index_type_values(&mut self, index_type_values: SmallVec<[u32; 8]>) -> &mut Self {
        self.index_type_values = index_type_values;
        self
    }
    ///Sets the `token_type` field in a builder way.
    pub fn with_token_type(mut self, token_type: IndirectCommandsTokenTypeNV) -> Self {
        self.token_type = token_type;
        self
    }
    ///Sets the `stream` field in a builder way.
    pub fn with_stream(mut self, stream: u32) -> Self {
        self.stream = stream;
        self
    }
    ///Sets the `offset` field in a builder way.
    pub fn with_offset(mut self, offset: u32) -> Self {
        self.offset = offset;
        self
    }
    ///Sets the `vertex_binding_unit` field in a builder way.
    pub fn with_vertex_binding_unit(mut self, vertex_binding_unit: u32) -> Self {
        self.vertex_binding_unit = vertex_binding_unit;
        self
    }
    ///Sets the `vertex_dynamic_stride` field in a builder way.
    pub fn with_vertex_dynamic_stride(mut self, vertex_dynamic_stride: bool) -> Self {
        self.vertex_dynamic_stride = vertex_dynamic_stride;
        self
    }
    ///Sets the `pushconstant_pipeline_layout` field in a builder way.
    pub fn with_pushconstant_pipeline_layout(mut self, pushconstant_pipeline_layout: Option<PipelineLayout>) -> Self {
        self.pushconstant_pipeline_layout = pushconstant_pipeline_layout;
        self
    }
    ///Sets the `pushconstant_shader_stage_flags` field in a builder way.
    pub fn with_pushconstant_shader_stage_flags(mut self, pushconstant_shader_stage_flags: ShaderStageFlags) -> Self {
        self.pushconstant_shader_stage_flags = pushconstant_shader_stage_flags;
        self
    }
    ///Sets the `pushconstant_offset` field in a builder way.
    pub fn with_pushconstant_offset(mut self, pushconstant_offset: u32) -> Self {
        self.pushconstant_offset = pushconstant_offset;
        self
    }
    ///Sets the `pushconstant_size` field in a builder way.
    pub fn with_pushconstant_size(mut self, pushconstant_size: u32) -> Self {
        self.pushconstant_size = pushconstant_size;
        self
    }
    ///Sets the `indirect_state_flags` field in a builder way.
    pub fn with_indirect_state_flags(mut self, indirect_state_flags: IndirectStateFlagsNV) -> Self {
        self.indirect_state_flags = indirect_state_flags;
        self
    }
    ///Sets the `index_types` field in a builder way.
    pub fn with_index_types(mut self, index_types: SmallVec<[IndexType; 8]>) -> Self {
        self.index_types = index_types;
        self
    }
    ///Sets the `index_type_values` field in a builder way.
    pub fn with_index_type_values(mut self, index_type_values: SmallVec<[u32; 8]>) -> Self {
        self.index_type_values = index_type_values;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for IndirectCommandsLayoutTokenNV {
    type LowLevel = crate::native::extensions::nv_device_generated_commands::IndirectCommandsLayoutTokenNV;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let len_index_types = self.index_types.len() as u32;
        let index_types = bump
            .alloc_slice_fill_iter(self.index_types.iter().map(|x| x.into_low_level(context, bump)))
            .as_ptr()
            .cast();
        let index_type_values = bump
            .alloc_slice_fill_iter(self.index_type_values.iter().map(|x| x.into_low_level(context, bump)))
            .as_ptr()
            .cast();
        crate::native::extensions::nv_device_generated_commands::IndirectCommandsLayoutTokenNV {
            s_type: StructureType::IndirectCommandsLayoutTokenNv,
            p_next: std::ptr::null(),
            token_type: self.token_type.into_low_level(context, bump),
            stream: self.stream.into_low_level(context, bump),
            offset: self.offset.into_low_level(context, bump),
            vertex_binding_unit: self.vertex_binding_unit.into_low_level(context, bump),
            vertex_dynamic_stride: self.vertex_dynamic_stride.into_low_level(context, bump),
            pushconstant_pipeline_layout: self
                .pushconstant_pipeline_layout
                .as_ref()
                .map(|v| v.into_low_level(context, bump))
                .unwrap_or_else(Default::default),
            pushconstant_shader_stage_flags: self.pushconstant_shader_stage_flags.into_low_level(context, bump),
            pushconstant_offset: self.pushconstant_offset.into_low_level(context, bump),
            pushconstant_size: self.pushconstant_size.into_low_level(context, bump),
            indirect_state_flags: self.indirect_state_flags.into_low_level(context, bump),
            index_type_count: len_index_types,
            index_types: index_types,
            index_type_values: index_type_values,
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for IndirectCommandsLayoutTokenNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let index_types_len = value.index_type_count;
        let mut index_types = SmallVec::with_capacity(index_types_len as usize);
        for i in 0..index_types_len {
            index_types.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.index_types.add(i as usize).read(),
            ));
        }
        let index_type_values_len = value.index_type_count;
        let mut index_type_values = SmallVec::with_capacity(index_type_values_len as usize);
        for i in 0..index_type_values_len {
            index_type_values.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.index_type_values.add(i as usize).read(),
            ));
        }
        Self {
            token_type: crate::conv::FromLowLevel::from_low_level(context, value.token_type),
            stream: crate::conv::FromLowLevel::from_low_level(context, value.stream),
            offset: crate::conv::FromLowLevel::from_low_level(context, value.offset),
            vertex_binding_unit: crate::conv::FromLowLevel::from_low_level(context, value.vertex_binding_unit),
            vertex_dynamic_stride: crate::conv::FromLowLevel::from_low_level(context, value.vertex_dynamic_stride),
            pushconstant_pipeline_layout: if value.pushconstant_pipeline_layout
                == crate::native::vulkan1_0::PipelineLayout::null()
            {
                None
            } else {
                Some(crate::conv::FromLowLevel::from_low_level(
                    context,
                    value.pushconstant_pipeline_layout,
                ))
            },
            pushconstant_shader_stage_flags: crate::conv::FromLowLevel::from_low_level(
                context,
                value.pushconstant_shader_stage_flags,
            ),
            pushconstant_offset: crate::conv::FromLowLevel::from_low_level(context, value.pushconstant_offset),
            pushconstant_size: crate::conv::FromLowLevel::from_low_level(context, value.pushconstant_size),
            indirect_state_flags: crate::conv::FromLowLevel::from_low_level(context, value.indirect_state_flags),
            index_types: index_types,
            index_type_values: index_type_values,
        }
    }
}
#[doc(alias = "VkIndirectCommandsLayoutCreateInfoNV")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct IndirectCommandsLayoutCreateInfoNV {
    pub flags: IndirectCommandsLayoutUsageFlagsNV,
    #[doc(alias = "pipelineBindPoint")]
    pub pipeline_bind_point: PipelineBindPoint,
    #[doc(alias = "pTokens")]
    pub tokens: SmallVec<[IndirectCommandsLayoutTokenNV; 8]>,
    #[doc(alias = "pStreamStrides")]
    pub stream_strides: SmallVec<[u32; 8]>,
}
impl IndirectCommandsLayoutCreateInfoNV {
    ///Get a reference to the `flags` field.
    pub fn flags(&self) -> IndirectCommandsLayoutUsageFlagsNV {
        self.flags
    }
    ///Get a reference to the `pipeline_bind_point` field.
    pub fn pipeline_bind_point(&self) -> PipelineBindPoint {
        self.pipeline_bind_point
    }
    ///Get a reference to the `tokens` field.
    pub fn tokens(&self) -> &SmallVec<[IndirectCommandsLayoutTokenNV; 8]> {
        &self.tokens
    }
    ///Get a reference to the `stream_strides` field.
    pub fn stream_strides(&self) -> &SmallVec<[u32; 8]> {
        &self.stream_strides
    }
    ///Get a mutable reference to the `flags` field.
    pub fn flags_mut(&mut self) -> &mut IndirectCommandsLayoutUsageFlagsNV {
        &mut self.flags
    }
    ///Get a mutable reference to the `pipeline_bind_point` field.
    pub fn pipeline_bind_point_mut(&mut self) -> &mut PipelineBindPoint {
        &mut self.pipeline_bind_point
    }
    ///Get a mutable reference to the `tokens` field.
    pub fn tokens_mut(&mut self) -> &mut SmallVec<[IndirectCommandsLayoutTokenNV; 8]> {
        &mut self.tokens
    }
    ///Get a mutable reference to the `stream_strides` field.
    pub fn stream_strides_mut(&mut self) -> &mut SmallVec<[u32; 8]> {
        &mut self.stream_strides
    }
    ///Sets the `flags` field.
    pub fn set_flags(&mut self, flags: IndirectCommandsLayoutUsageFlagsNV) -> &mut Self {
        self.flags = flags;
        self
    }
    ///Sets the `pipeline_bind_point` field.
    pub fn set_pipeline_bind_point(&mut self, pipeline_bind_point: PipelineBindPoint) -> &mut Self {
        self.pipeline_bind_point = pipeline_bind_point;
        self
    }
    ///Sets the `tokens` field.
    pub fn set_tokens(&mut self, tokens: SmallVec<[IndirectCommandsLayoutTokenNV; 8]>) -> &mut Self {
        self.tokens = tokens;
        self
    }
    ///Sets the `stream_strides` field.
    pub fn set_stream_strides(&mut self, stream_strides: SmallVec<[u32; 8]>) -> &mut Self {
        self.stream_strides = stream_strides;
        self
    }
    ///Sets the `flags` field in a builder way.
    pub fn with_flags(mut self, flags: IndirectCommandsLayoutUsageFlagsNV) -> Self {
        self.flags = flags;
        self
    }
    ///Sets the `pipeline_bind_point` field in a builder way.
    pub fn with_pipeline_bind_point(mut self, pipeline_bind_point: PipelineBindPoint) -> Self {
        self.pipeline_bind_point = pipeline_bind_point;
        self
    }
    ///Sets the `tokens` field in a builder way.
    pub fn with_tokens(mut self, tokens: SmallVec<[IndirectCommandsLayoutTokenNV; 8]>) -> Self {
        self.tokens = tokens;
        self
    }
    ///Sets the `stream_strides` field in a builder way.
    pub fn with_stream_strides(mut self, stream_strides: SmallVec<[u32; 8]>) -> Self {
        self.stream_strides = stream_strides;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for IndirectCommandsLayoutCreateInfoNV {
    type LowLevel = crate::native::extensions::nv_device_generated_commands::IndirectCommandsLayoutCreateInfoNV;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let len_tokens = self.tokens.len() as u32;
        let tokens = bump
            .alloc_slice_fill_iter(self.tokens.iter().map(|x| x.into_low_level(context, bump)))
            .as_ptr()
            .cast();
        let len_stream_strides = self.stream_strides.len() as u32;
        let stream_strides = bump
            .alloc_slice_fill_iter(self.stream_strides.iter().map(|x| x.into_low_level(context, bump)))
            .as_ptr()
            .cast();
        crate::native::extensions::nv_device_generated_commands::IndirectCommandsLayoutCreateInfoNV {
            s_type: StructureType::IndirectCommandsLayoutCreateInfoNv,
            p_next: std::ptr::null(),
            flags: self.flags.into_low_level(context, bump),
            pipeline_bind_point: self.pipeline_bind_point.into_low_level(context, bump),
            token_count: len_tokens,
            tokens: tokens,
            stream_count: len_stream_strides,
            stream_strides: stream_strides,
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for IndirectCommandsLayoutCreateInfoNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let tokens_len = value.token_count;
        let mut tokens = SmallVec::with_capacity(tokens_len as usize);
        for i in 0..tokens_len {
            tokens.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.tokens.add(i as usize).read(),
            ));
        }
        let stream_strides_len = value.stream_count;
        let mut stream_strides = SmallVec::with_capacity(stream_strides_len as usize);
        for i in 0..stream_strides_len {
            stream_strides.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.stream_strides.add(i as usize).read(),
            ));
        }
        Self {
            flags: crate::conv::FromLowLevel::from_low_level(context, value.flags),
            pipeline_bind_point: crate::conv::FromLowLevel::from_low_level(context, value.pipeline_bind_point),
            tokens: tokens,
            stream_strides: stream_strides,
        }
    }
}
#[doc(alias = "VkGeneratedCommandsInfoNV")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GeneratedCommandsInfoNV {
    #[doc(alias = "pipelineBindPoint")]
    pub pipeline_bind_point: PipelineBindPoint,
    pub pipeline: Pipeline,
    #[doc(alias = "indirectCommandsLayout")]
    pub indirect_commands_layout: IndirectCommandsLayoutNV,
    #[doc(alias = "pStreams")]
    pub streams: SmallVec<[IndirectCommandsStreamNV; 8]>,
    #[doc(alias = "sequencesCount")]
    pub sequences_count: u32,
    #[doc(alias = "preprocessBuffer")]
    pub preprocess_buffer: Buffer,
    #[doc(alias = "preprocessOffset")]
    pub preprocess_offset: DeviceSize,
    #[doc(alias = "preprocessSize")]
    pub preprocess_size: DeviceSize,
    #[doc(alias = "sequencesCountBuffer")]
    pub sequences_count_buffer: Option<Buffer>,
    #[doc(alias = "sequencesCountOffset")]
    pub sequences_count_offset: DeviceSize,
    #[doc(alias = "sequencesIndexBuffer")]
    pub sequences_index_buffer: Option<Buffer>,
    #[doc(alias = "sequencesIndexOffset")]
    pub sequences_index_offset: DeviceSize,
}
impl GeneratedCommandsInfoNV {
    ///Get a reference to the `pipeline_bind_point` field.
    pub fn pipeline_bind_point(&self) -> PipelineBindPoint {
        self.pipeline_bind_point
    }
    ///Get a reference to the `pipeline` field.
    pub fn pipeline(&self) -> &Pipeline {
        &self.pipeline
    }
    ///Get a reference to the `indirect_commands_layout` field.
    pub fn indirect_commands_layout(&self) -> &IndirectCommandsLayoutNV {
        &self.indirect_commands_layout
    }
    ///Get a reference to the `streams` field.
    pub fn streams(&self) -> &SmallVec<[IndirectCommandsStreamNV; 8]> {
        &self.streams
    }
    ///Get a reference to the `sequences_count` field.
    pub fn sequences_count(&self) -> u32 {
        self.sequences_count
    }
    ///Get a reference to the `preprocess_buffer` field.
    pub fn preprocess_buffer(&self) -> &Buffer {
        &self.preprocess_buffer
    }
    ///Get a reference to the `preprocess_offset` field.
    pub fn preprocess_offset(&self) -> &DeviceSize {
        &self.preprocess_offset
    }
    ///Get a reference to the `preprocess_size` field.
    pub fn preprocess_size(&self) -> &DeviceSize {
        &self.preprocess_size
    }
    ///Get a reference to the `sequences_count_buffer` field.
    pub fn sequences_count_buffer(&self) -> &Option<Buffer> {
        &self.sequences_count_buffer
    }
    ///Get a reference to the `sequences_count_offset` field.
    pub fn sequences_count_offset(&self) -> &DeviceSize {
        &self.sequences_count_offset
    }
    ///Get a reference to the `sequences_index_buffer` field.
    pub fn sequences_index_buffer(&self) -> &Option<Buffer> {
        &self.sequences_index_buffer
    }
    ///Get a reference to the `sequences_index_offset` field.
    pub fn sequences_index_offset(&self) -> &DeviceSize {
        &self.sequences_index_offset
    }
    ///Get a mutable reference to the `pipeline_bind_point` field.
    pub fn pipeline_bind_point_mut(&mut self) -> &mut PipelineBindPoint {
        &mut self.pipeline_bind_point
    }
    ///Get a mutable reference to the `pipeline` field.
    pub fn pipeline_mut(&mut self) -> &mut Pipeline {
        &mut self.pipeline
    }
    ///Get a mutable reference to the `indirect_commands_layout` field.
    pub fn indirect_commands_layout_mut(&mut self) -> &mut IndirectCommandsLayoutNV {
        &mut self.indirect_commands_layout
    }
    ///Get a mutable reference to the `streams` field.
    pub fn streams_mut(&mut self) -> &mut SmallVec<[IndirectCommandsStreamNV; 8]> {
        &mut self.streams
    }
    ///Get a mutable reference to the `sequences_count` field.
    pub fn sequences_count_mut(&mut self) -> &mut u32 {
        &mut self.sequences_count
    }
    ///Get a mutable reference to the `preprocess_buffer` field.
    pub fn preprocess_buffer_mut(&mut self) -> &mut Buffer {
        &mut self.preprocess_buffer
    }
    ///Get a mutable reference to the `preprocess_offset` field.
    pub fn preprocess_offset_mut(&mut self) -> &mut DeviceSize {
        &mut self.preprocess_offset
    }
    ///Get a mutable reference to the `preprocess_size` field.
    pub fn preprocess_size_mut(&mut self) -> &mut DeviceSize {
        &mut self.preprocess_size
    }
    ///Get a mutable reference to the `sequences_count_buffer` field.
    pub fn sequences_count_buffer_mut(&mut self) -> &mut Option<Buffer> {
        &mut self.sequences_count_buffer
    }
    ///Get a mutable reference to the `sequences_count_offset` field.
    pub fn sequences_count_offset_mut(&mut self) -> &mut DeviceSize {
        &mut self.sequences_count_offset
    }
    ///Get a mutable reference to the `sequences_index_buffer` field.
    pub fn sequences_index_buffer_mut(&mut self) -> &mut Option<Buffer> {
        &mut self.sequences_index_buffer
    }
    ///Get a mutable reference to the `sequences_index_offset` field.
    pub fn sequences_index_offset_mut(&mut self) -> &mut DeviceSize {
        &mut self.sequences_index_offset
    }
    ///Sets the `pipeline_bind_point` field.
    pub fn set_pipeline_bind_point(&mut self, pipeline_bind_point: PipelineBindPoint) -> &mut Self {
        self.pipeline_bind_point = pipeline_bind_point;
        self
    }
    ///Sets the `pipeline` field.
    pub fn set_pipeline(&mut self, pipeline: Pipeline) -> &mut Self {
        self.pipeline = pipeline;
        self
    }
    ///Sets the `indirect_commands_layout` field.
    pub fn set_indirect_commands_layout(&mut self, indirect_commands_layout: IndirectCommandsLayoutNV) -> &mut Self {
        self.indirect_commands_layout = indirect_commands_layout;
        self
    }
    ///Sets the `streams` field.
    pub fn set_streams(&mut self, streams: SmallVec<[IndirectCommandsStreamNV; 8]>) -> &mut Self {
        self.streams = streams;
        self
    }
    ///Sets the `sequences_count` field.
    pub fn set_sequences_count(&mut self, sequences_count: u32) -> &mut Self {
        self.sequences_count = sequences_count;
        self
    }
    ///Sets the `preprocess_buffer` field.
    pub fn set_preprocess_buffer(&mut self, preprocess_buffer: Buffer) -> &mut Self {
        self.preprocess_buffer = preprocess_buffer;
        self
    }
    ///Sets the `preprocess_offset` field.
    pub fn set_preprocess_offset(&mut self, preprocess_offset: DeviceSize) -> &mut Self {
        self.preprocess_offset = preprocess_offset;
        self
    }
    ///Sets the `preprocess_size` field.
    pub fn set_preprocess_size(&mut self, preprocess_size: DeviceSize) -> &mut Self {
        self.preprocess_size = preprocess_size;
        self
    }
    ///Sets the `sequences_count_buffer` field.
    pub fn set_sequences_count_buffer(&mut self, sequences_count_buffer: Option<Buffer>) -> &mut Self {
        self.sequences_count_buffer = sequences_count_buffer;
        self
    }
    ///Sets the `sequences_count_offset` field.
    pub fn set_sequences_count_offset(&mut self, sequences_count_offset: DeviceSize) -> &mut Self {
        self.sequences_count_offset = sequences_count_offset;
        self
    }
    ///Sets the `sequences_index_buffer` field.
    pub fn set_sequences_index_buffer(&mut self, sequences_index_buffer: Option<Buffer>) -> &mut Self {
        self.sequences_index_buffer = sequences_index_buffer;
        self
    }
    ///Sets the `sequences_index_offset` field.
    pub fn set_sequences_index_offset(&mut self, sequences_index_offset: DeviceSize) -> &mut Self {
        self.sequences_index_offset = sequences_index_offset;
        self
    }
    ///Sets the `pipeline_bind_point` field in a builder way.
    pub fn with_pipeline_bind_point(mut self, pipeline_bind_point: PipelineBindPoint) -> Self {
        self.pipeline_bind_point = pipeline_bind_point;
        self
    }
    ///Sets the `pipeline` field in a builder way.
    pub fn with_pipeline(mut self, pipeline: Pipeline) -> Self {
        self.pipeline = pipeline;
        self
    }
    ///Sets the `indirect_commands_layout` field in a builder way.
    pub fn with_indirect_commands_layout(mut self, indirect_commands_layout: IndirectCommandsLayoutNV) -> Self {
        self.indirect_commands_layout = indirect_commands_layout;
        self
    }
    ///Sets the `streams` field in a builder way.
    pub fn with_streams(mut self, streams: SmallVec<[IndirectCommandsStreamNV; 8]>) -> Self {
        self.streams = streams;
        self
    }
    ///Sets the `sequences_count` field in a builder way.
    pub fn with_sequences_count(mut self, sequences_count: u32) -> Self {
        self.sequences_count = sequences_count;
        self
    }
    ///Sets the `preprocess_buffer` field in a builder way.
    pub fn with_preprocess_buffer(mut self, preprocess_buffer: Buffer) -> Self {
        self.preprocess_buffer = preprocess_buffer;
        self
    }
    ///Sets the `preprocess_offset` field in a builder way.
    pub fn with_preprocess_offset(mut self, preprocess_offset: DeviceSize) -> Self {
        self.preprocess_offset = preprocess_offset;
        self
    }
    ///Sets the `preprocess_size` field in a builder way.
    pub fn with_preprocess_size(mut self, preprocess_size: DeviceSize) -> Self {
        self.preprocess_size = preprocess_size;
        self
    }
    ///Sets the `sequences_count_buffer` field in a builder way.
    pub fn with_sequences_count_buffer(mut self, sequences_count_buffer: Option<Buffer>) -> Self {
        self.sequences_count_buffer = sequences_count_buffer;
        self
    }
    ///Sets the `sequences_count_offset` field in a builder way.
    pub fn with_sequences_count_offset(mut self, sequences_count_offset: DeviceSize) -> Self {
        self.sequences_count_offset = sequences_count_offset;
        self
    }
    ///Sets the `sequences_index_buffer` field in a builder way.
    pub fn with_sequences_index_buffer(mut self, sequences_index_buffer: Option<Buffer>) -> Self {
        self.sequences_index_buffer = sequences_index_buffer;
        self
    }
    ///Sets the `sequences_index_offset` field in a builder way.
    pub fn with_sequences_index_offset(mut self, sequences_index_offset: DeviceSize) -> Self {
        self.sequences_index_offset = sequences_index_offset;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for GeneratedCommandsInfoNV {
    type LowLevel = crate::native::extensions::nv_device_generated_commands::GeneratedCommandsInfoNV;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let len_streams = self.streams.len() as u32;
        let streams = bump
            .alloc_slice_fill_iter(self.streams.iter().map(|x| x.into_low_level(context, bump)))
            .as_ptr()
            .cast();
        crate::native::extensions::nv_device_generated_commands::GeneratedCommandsInfoNV {
            s_type: StructureType::GeneratedCommandsInfoNv,
            p_next: std::ptr::null(),
            pipeline_bind_point: self.pipeline_bind_point.into_low_level(context, bump),
            pipeline: self.pipeline.into_low_level(context, bump),
            indirect_commands_layout: self.indirect_commands_layout.into_low_level(context, bump),
            stream_count: len_streams,
            streams: streams,
            sequences_count: self.sequences_count.into_low_level(context, bump),
            preprocess_buffer: self.preprocess_buffer.into_low_level(context, bump),
            preprocess_offset: self.preprocess_offset.into_low_level(context, bump),
            preprocess_size: self.preprocess_size.into_low_level(context, bump),
            sequences_count_buffer: self
                .sequences_count_buffer
                .as_ref()
                .map(|v| v.into_low_level(context, bump))
                .unwrap_or_else(Default::default),
            sequences_count_offset: self.sequences_count_offset.into_low_level(context, bump),
            sequences_index_buffer: self
                .sequences_index_buffer
                .as_ref()
                .map(|v| v.into_low_level(context, bump))
                .unwrap_or_else(Default::default),
            sequences_index_offset: self.sequences_index_offset.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for GeneratedCommandsInfoNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let streams_len = value.stream_count;
        let mut streams = SmallVec::with_capacity(streams_len as usize);
        for i in 0..streams_len {
            streams.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.streams.add(i as usize).read(),
            ));
        }
        Self {
            pipeline_bind_point: crate::conv::FromLowLevel::from_low_level(context, value.pipeline_bind_point),
            pipeline: crate::conv::FromLowLevel::from_low_level(context, value.pipeline),
            indirect_commands_layout: crate::conv::FromLowLevel::from_low_level(
                context,
                value.indirect_commands_layout,
            ),
            streams: streams,
            sequences_count: crate::conv::FromLowLevel::from_low_level(context, value.sequences_count),
            preprocess_buffer: crate::conv::FromLowLevel::from_low_level(context, value.preprocess_buffer),
            preprocess_offset: crate::conv::FromLowLevel::from_low_level(context, value.preprocess_offset),
            preprocess_size: crate::conv::FromLowLevel::from_low_level(context, value.preprocess_size),
            sequences_count_buffer: if value.sequences_count_buffer == crate::native::vulkan1_0::Buffer::null() {
                None
            } else {
                Some(crate::conv::FromLowLevel::from_low_level(
                    context,
                    value.sequences_count_buffer,
                ))
            },
            sequences_count_offset: crate::conv::FromLowLevel::from_low_level(context, value.sequences_count_offset),
            sequences_index_buffer: if value.sequences_index_buffer == crate::native::vulkan1_0::Buffer::null() {
                None
            } else {
                Some(crate::conv::FromLowLevel::from_low_level(
                    context,
                    value.sequences_index_buffer,
                ))
            },
            sequences_index_offset: crate::conv::FromLowLevel::from_low_level(context, value.sequences_index_offset),
        }
    }
}
#[doc(alias = "VkGeneratedCommandsMemoryRequirementsInfoNV")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GeneratedCommandsMemoryRequirementsInfoNV {
    #[doc(alias = "pipelineBindPoint")]
    pub pipeline_bind_point: PipelineBindPoint,
    pub pipeline: Pipeline,
    #[doc(alias = "indirectCommandsLayout")]
    pub indirect_commands_layout: IndirectCommandsLayoutNV,
    #[doc(alias = "maxSequencesCount")]
    pub max_sequences_count: u32,
}
impl GeneratedCommandsMemoryRequirementsInfoNV {
    ///Get a reference to the `pipeline_bind_point` field.
    pub fn pipeline_bind_point(&self) -> PipelineBindPoint {
        self.pipeline_bind_point
    }
    ///Get a reference to the `pipeline` field.
    pub fn pipeline(&self) -> &Pipeline {
        &self.pipeline
    }
    ///Get a reference to the `indirect_commands_layout` field.
    pub fn indirect_commands_layout(&self) -> &IndirectCommandsLayoutNV {
        &self.indirect_commands_layout
    }
    ///Get a reference to the `max_sequences_count` field.
    pub fn max_sequences_count(&self) -> u32 {
        self.max_sequences_count
    }
    ///Get a mutable reference to the `pipeline_bind_point` field.
    pub fn pipeline_bind_point_mut(&mut self) -> &mut PipelineBindPoint {
        &mut self.pipeline_bind_point
    }
    ///Get a mutable reference to the `pipeline` field.
    pub fn pipeline_mut(&mut self) -> &mut Pipeline {
        &mut self.pipeline
    }
    ///Get a mutable reference to the `indirect_commands_layout` field.
    pub fn indirect_commands_layout_mut(&mut self) -> &mut IndirectCommandsLayoutNV {
        &mut self.indirect_commands_layout
    }
    ///Get a mutable reference to the `max_sequences_count` field.
    pub fn max_sequences_count_mut(&mut self) -> &mut u32 {
        &mut self.max_sequences_count
    }
    ///Sets the `pipeline_bind_point` field.
    pub fn set_pipeline_bind_point(&mut self, pipeline_bind_point: PipelineBindPoint) -> &mut Self {
        self.pipeline_bind_point = pipeline_bind_point;
        self
    }
    ///Sets the `pipeline` field.
    pub fn set_pipeline(&mut self, pipeline: Pipeline) -> &mut Self {
        self.pipeline = pipeline;
        self
    }
    ///Sets the `indirect_commands_layout` field.
    pub fn set_indirect_commands_layout(&mut self, indirect_commands_layout: IndirectCommandsLayoutNV) -> &mut Self {
        self.indirect_commands_layout = indirect_commands_layout;
        self
    }
    ///Sets the `max_sequences_count` field.
    pub fn set_max_sequences_count(&mut self, max_sequences_count: u32) -> &mut Self {
        self.max_sequences_count = max_sequences_count;
        self
    }
    ///Sets the `pipeline_bind_point` field in a builder way.
    pub fn with_pipeline_bind_point(mut self, pipeline_bind_point: PipelineBindPoint) -> Self {
        self.pipeline_bind_point = pipeline_bind_point;
        self
    }
    ///Sets the `pipeline` field in a builder way.
    pub fn with_pipeline(mut self, pipeline: Pipeline) -> Self {
        self.pipeline = pipeline;
        self
    }
    ///Sets the `indirect_commands_layout` field in a builder way.
    pub fn with_indirect_commands_layout(mut self, indirect_commands_layout: IndirectCommandsLayoutNV) -> Self {
        self.indirect_commands_layout = indirect_commands_layout;
        self
    }
    ///Sets the `max_sequences_count` field in a builder way.
    pub fn with_max_sequences_count(mut self, max_sequences_count: u32) -> Self {
        self.max_sequences_count = max_sequences_count;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for GeneratedCommandsMemoryRequirementsInfoNV {
    type LowLevel = crate::native::extensions::nv_device_generated_commands::GeneratedCommandsMemoryRequirementsInfoNV;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::nv_device_generated_commands::GeneratedCommandsMemoryRequirementsInfoNV {
            s_type: StructureType::GeneratedCommandsMemoryRequirementsInfoNv,
            p_next: std::ptr::null(),
            pipeline_bind_point: self.pipeline_bind_point.into_low_level(context, bump),
            pipeline: self.pipeline.into_low_level(context, bump),
            indirect_commands_layout: self.indirect_commands_layout.into_low_level(context, bump),
            max_sequences_count: self.max_sequences_count.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for GeneratedCommandsMemoryRequirementsInfoNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            pipeline_bind_point: crate::conv::FromLowLevel::from_low_level(context, value.pipeline_bind_point),
            pipeline: crate::conv::FromLowLevel::from_low_level(context, value.pipeline),
            indirect_commands_layout: crate::conv::FromLowLevel::from_low_level(
                context,
                value.indirect_commands_layout,
            ),
            max_sequences_count: crate::conv::FromLowLevel::from_low_level(context, value.max_sequences_count),
        }
    }
}
#[doc(alias = "VkIndirectCommandsLayoutNV")]
#[derive(Debug)]
pub struct IndirectCommandsLayoutNV {
    context: Arc<Context>,
    id: ObjectId,
}
impl Clone for IndirectCommandsLayoutNV {
    fn clone(&self) -> Self {
        self.context.clone_indirect_commands_layout_nv(self.id);
        Self {
            context: Arc::clone(&self.context),
            id: self.id,
        }
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for IndirectCommandsLayoutNV {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.id.serialize(serializer)
    }
}
impl<'de> Deserialize<'de> for IndirectCommandsLayoutNV {
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
impl Drop for IndirectCommandsLayoutNV {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            self.context.drop_indirect_commands_layout_nv(&self.id);
        }
    }
}
impl PartialEq for IndirectCommandsLayoutNV {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl IndirectCommandsLayoutNV {
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
unsafe impl crate::conv::IntoLowLevel for IndirectCommandsLayoutNV {
    type LowLevel = crate::native::extensions::nv_device_generated_commands::IndirectCommandsLayoutNV;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        *context
            .indirect_commands_layout_nv()
            .get(&self.id)
            .expect("unknwon handle")
            .handle()
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for IndirectCommandsLayoutNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let object_id = ObjectId::random();
        context
            .indirect_commands_layout_nv()
            .insert(object_id, Container::new(value));
        Self {
            context: context.clone(),
            id: object_id,
        }
    }
}
