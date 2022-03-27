use crate::vulkan1_0::{
    BaseInStructure, BaseOutStructure, Bool32, Buffer, DeviceAddress, DeviceSize, IndexType, Pipeline,
    PipelineBindPoint, PipelineLayout, PipelineShaderStageCreateInfo, PipelineTessellationStateCreateInfo,
    PipelineVertexInputStateCreateInfo, ShaderStageFlags, StructureType,
};
#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_DEVICE_GENERATED_COMMANDS_SPEC_VERSION")]
pub const NV_DEVICE_GENERATED_COMMANDS_SPEC_VERSION: u32 = 3;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_DEVICE_GENERATED_COMMANDS_EXTENSION_NAME")]
pub const NV_DEVICE_GENERATED_COMMANDS_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_NV_device_generated_commands");
///[VkIndirectCommandsTokenTypeNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkIndirectCommandsTokenTypeNV.html) - Enum specifying token commands
///# C Specifications
///Possible values of those elements of the
///[`IndirectCommandsLayoutCreateInfoNV::p_tokens`] array specifying
///command tokens (other elements of the array specify command parameters) are:
///```c
///// Provided by VK_NV_device_generated_commands
///typedef enum VkIndirectCommandsTokenTypeNV {
///    VK_INDIRECT_COMMANDS_TOKEN_TYPE_SHADER_GROUP_NV = 0,
///    VK_INDIRECT_COMMANDS_TOKEN_TYPE_STATE_FLAGS_NV = 1,
///    VK_INDIRECT_COMMANDS_TOKEN_TYPE_INDEX_BUFFER_NV = 2,
///    VK_INDIRECT_COMMANDS_TOKEN_TYPE_VERTEX_BUFFER_NV = 3,
///    VK_INDIRECT_COMMANDS_TOKEN_TYPE_PUSH_CONSTANT_NV = 4,
///    VK_INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_INDEXED_NV = 5,
///    VK_INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_NV = 6,
///    VK_INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_TASKS_NV = 7,
///} VkIndirectCommandsTokenTypeNV;
///```
///# Related
/// - [`VK_NV_device_generated_commands`]
/// - [`IndirectCommandsLayoutTokenNV`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkIndirectCommandsTokenTypeNV")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(i32)]
pub enum IndirectCommandsTokenTypeNV {
    ///No documentation found
    IndirectCommandsTokenTypeShaderGroupNv = 0,
    ///No documentation found
    IndirectCommandsTokenTypeStateFlagsNv = 1,
    ///No documentation found
    IndirectCommandsTokenTypeIndexBufferNv = 2,
    ///No documentation found
    IndirectCommandsTokenTypeVertexBufferNv = 3,
    ///No documentation found
    IndirectCommandsTokenTypePushConstantNv = 4,
    ///No documentation found
    IndirectCommandsTokenTypeDrawIndexedNv = 5,
    ///No documentation found
    IndirectCommandsTokenTypeDrawNv = 6,
    ///No documentation found
    IndirectCommandsTokenTypeDrawTasksNv = 7,
}
impl const Default for IndirectCommandsTokenTypeNV {
    fn default() -> Self {
        IndirectCommandsTokenTypeShaderGroupNv
    }
}
impl IndirectCommandsTokenTypeNV {
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> i32 {
        self as i32
    }
    ///Gets a value from a raw underlying value, unchecked and therefore unsafe
    #[inline]
    pub const unsafe fn from_bits(bits: i32) -> i32 {
        std::mem::transmute(bits)
    }
}
///[VkPhysicalDeviceDeviceGeneratedCommandsFeaturesNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDeviceGeneratedCommandsFeaturesNV.html) - Structure describing the device-generated commands features that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceDeviceGeneratedCommandsFeaturesNV`] structure is
///defined as:
///```c
///// Provided by VK_NV_device_generated_commands
///typedef struct VkPhysicalDeviceDeviceGeneratedCommandsFeaturesNV {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           deviceGeneratedCommands;
///} VkPhysicalDeviceDeviceGeneratedCommandsFeaturesNV;
///```
///# Members
///This structure describes the following feature:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`device_generated_commands`] indicates whether the implementation supports functionality to generate commands on the device. See [Device-Generated Commands](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#device-generated-commands).
///If the [`PhysicalDeviceDeviceGeneratedCommandsFeaturesNV`] structure is included in the
/// [`p_next`] chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceDeviceGeneratedCommandsFeaturesNV`]**can** also be used in the [`p_next`] chain
/// of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage (Implicit)
/// - [`s_type`]**must** be
///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_FEATURES_NV`
///# Related
/// - [`VK_NV_device_generated_commands`]
/// - [`Bool32`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct PhysicalDeviceDeviceGeneratedCommandsFeaturesNV<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`device_generated_commands`]
    ///indicates whether the implementation supports functionality to generate
    ///commands on the device.
    ///See [Device-Generated Commands](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#device-generated-commands).
    device_generated_commands: Bool32,
}
///[VkPhysicalDeviceDeviceGeneratedCommandsPropertiesNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDeviceGeneratedCommandsPropertiesNV.html) - Structure describing push descriptor limits that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceDeviceGeneratedCommandsPropertiesNV`] structure is
///defined as:
///```c
///// Provided by VK_NV_device_generated_commands
///typedef struct VkPhysicalDeviceDeviceGeneratedCommandsPropertiesNV {
///    VkStructureType    sType;
///    void*              pNext;
///    uint32_t           maxGraphicsShaderGroupCount;
///    uint32_t           maxIndirectSequenceCount;
///    uint32_t           maxIndirectCommandsTokenCount;
///    uint32_t           maxIndirectCommandsStreamCount;
///    uint32_t           maxIndirectCommandsTokenOffset;
///    uint32_t           maxIndirectCommandsStreamStride;
///    uint32_t           minSequencesCountBufferOffsetAlignment;
///    uint32_t           minSequencesIndexBufferOffsetAlignment;
///    uint32_t           minIndirectCommandsBufferOffsetAlignment;
///} VkPhysicalDeviceDeviceGeneratedCommandsPropertiesNV;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`max_graphics_shader_group_count`] is the maximum number of shader groups in
///   [`GraphicsPipelineShaderGroupsCreateInfoNV`].
/// - [`max_indirect_sequence_count`] is the maximum number of sequences in
///   [`GeneratedCommandsInfoNV`] and in [`GeneratedCommandsMemoryRequirementsInfoNV`].
/// - [`max_indirect_commands_token_count`] is the maximum number of tokens in
///   [`IndirectCommandsLayoutCreateInfoNV`].
/// - [`max_indirect_commands_stream_count`] is the maximum number of streams in
///   [`IndirectCommandsLayoutCreateInfoNV`].
/// - [`max_indirect_commands_token_offset`] is the maximum offset in
///   [`IndirectCommandsLayoutTokenNV`].
/// - [`max_indirect_commands_stream_stride`] is the maximum stream stride in
///   [`IndirectCommandsLayoutCreateInfoNV`].
/// - [`min_sequences_count_buffer_offset_alignment`] is the minimum alignment for memory addresses
///   which **can** be used in [`GeneratedCommandsInfoNV`].
/// - [`min_sequences_index_buffer_offset_alignment`] is the minimum alignment for memory addresses
///   which **can** be used in [`GeneratedCommandsInfoNV`].
/// - [`min_indirect_commands_buffer_offset_alignment`] is the minimum alignment for memory
///   addresses used in [`IndirectCommandsStreamNV`], and as preprocess buffer in
///   [`GeneratedCommandsInfoNV`].
///# Description
///If the [`PhysicalDeviceDeviceGeneratedCommandsPropertiesNV`] structure is included in the
/// [`p_next`] chain of the
///[`PhysicalDeviceProperties2`] structure passed to
///[`GetPhysicalDeviceProperties2`], it is filled in with each
///corresponding implementation-dependent property.Valid Usage (Implicit)
/// - [`s_type`]**must** be
///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_PROPERTIES_NV`
///# Related
/// - [`VK_NV_device_generated_commands`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct PhysicalDeviceDeviceGeneratedCommandsPropertiesNV<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`max_graphics_shader_group_count`] is the maximum number of shader groups
    ///in [`GraphicsPipelineShaderGroupsCreateInfoNV`].
    max_graphics_shader_group_count: u32,
    ///[`max_indirect_sequence_count`] is the maximum number of sequences in
    ///[`GeneratedCommandsInfoNV`] and in
    ///[`GeneratedCommandsMemoryRequirementsInfoNV`].
    max_indirect_sequence_count: u32,
    ///[`max_indirect_commands_token_count`] is the maximum number of tokens in
    ///[`IndirectCommandsLayoutCreateInfoNV`].
    max_indirect_commands_token_count: u32,
    ///[`max_indirect_commands_stream_count`] is the maximum number of streams in
    ///[`IndirectCommandsLayoutCreateInfoNV`].
    max_indirect_commands_stream_count: u32,
    ///[`max_indirect_commands_token_offset`] is the maximum offset in
    ///[`IndirectCommandsLayoutTokenNV`].
    max_indirect_commands_token_offset: u32,
    ///[`max_indirect_commands_stream_stride`] is the maximum stream stride in
    ///[`IndirectCommandsLayoutCreateInfoNV`].
    max_indirect_commands_stream_stride: u32,
    ///[`min_sequences_count_buffer_offset_alignment`] is the minimum alignment
    ///for memory addresses which **can** be used in
    ///[`GeneratedCommandsInfoNV`].
    min_sequences_count_buffer_offset_alignment: u32,
    ///[`min_sequences_index_buffer_offset_alignment`] is the minimum alignment
    ///for memory addresses which **can** be used in
    ///[`GeneratedCommandsInfoNV`].
    min_sequences_index_buffer_offset_alignment: u32,
    ///[`min_indirect_commands_buffer_offset_alignment`] is the minimum alignment
    ///for memory addresses used in [`IndirectCommandsStreamNV`], and as
    ///preprocess buffer in [`GeneratedCommandsInfoNV`].
    min_indirect_commands_buffer_offset_alignment: u32,
}
///[VkGraphicsShaderGroupCreateInfoNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkGraphicsShaderGroupCreateInfoNV.html) - Structure specifying override parameters for each shader group
///# C Specifications
///The [`GraphicsShaderGroupCreateInfoNV`] structure provides the state
///overrides for each shader group.
///Each shader group behaves like a pipeline that was created from its state as
///well as the remaining parent’s state.
///It is defined as:
///```c
///// Provided by VK_NV_device_generated_commands
///typedef struct VkGraphicsShaderGroupCreateInfoNV {
///    VkStructureType                                 sType;
///    const void*                                     pNext;
///    uint32_t                                        stageCount;
///    const VkPipelineShaderStageCreateInfo*          pStages;
///    const VkPipelineVertexInputStateCreateInfo*     pVertexInputState;
///    const VkPipelineTessellationStateCreateInfo*    pTessellationState;
///} VkGraphicsShaderGroupCreateInfoNV;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`stage_count`] is the number of entries in the [`p_stages`] array.
/// - [`p_stages`] is a pointer to an array [`PipelineShaderStageCreateInfo`] structures specifying
///   the set of the shader stages to be included in this shader group.
/// - [`p_vertex_input_state`] is a pointer to a [`PipelineVertexInputStateCreateInfo`] structure.
/// - [`p_tessellation_state`] is a pointer to a [`PipelineTessellationStateCreateInfo`] structure,
///   and is ignored if the shader group does not include a tessellation control shader stage and
///   tessellation evaluation shader stage.
///# Description
///Valid Usage
/// - For [`stage_count`], the same restrictions as in
///   [`GraphicsPipelineCreateInfo`]::[`stage_count`] apply
/// - For [`p_stages`], the same restrictions as in [`GraphicsPipelineCreateInfo`]::[`p_stages`]
///   apply
/// - For [`p_vertex_input_state`], the same restrictions as in
///   [`GraphicsPipelineCreateInfo`]::[`p_vertex_input_state`] apply
/// - For [`p_tessellation_state`], the same restrictions as in
///   [`GraphicsPipelineCreateInfo`]::[`p_tessellation_state`] apply
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_GRAPHICS_SHADER_GROUP_CREATE_INFO_NV`
/// - [`p_next`]**must** be `NULL`
/// - [`p_stages`]**must** be a valid pointer to an array of [`stage_count`] valid
///   [`PipelineShaderStageCreateInfo`] structures
/// - [`stage_count`]**must** be greater than `0`
///# Related
/// - [`VK_NV_device_generated_commands`]
/// - [`GraphicsPipelineShaderGroupsCreateInfoNV`]
/// - [`PipelineShaderStageCreateInfo`]
/// - [`PipelineTessellationStateCreateInfo`]
/// - [`PipelineVertexInputStateCreateInfo`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct GraphicsShaderGroupCreateInfoNV<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`stage_count`] is the number of entries in the [`p_stages`] array.
    stage_count: u32,
    ///[`p_stages`] is a pointer to an array
    ///[`PipelineShaderStageCreateInfo`] structures specifying the set of
    ///the shader stages to be included in this shader group.
    p_stages: *mut PipelineShaderStageCreateInfo<'lt>,
    ///[`p_vertex_input_state`] is a pointer to a
    ///[`PipelineVertexInputStateCreateInfo`] structure.
    p_vertex_input_state: *mut PipelineVertexInputStateCreateInfo<'lt>,
    ///[`p_tessellation_state`] is a pointer to a
    ///[`PipelineTessellationStateCreateInfo`] structure, and is ignored if
    ///the shader group does not include a tessellation control shader stage
    ///and tessellation evaluation shader stage.
    p_tessellation_state: *mut PipelineTessellationStateCreateInfo<'lt>,
}
///[VkGraphicsPipelineShaderGroupsCreateInfoNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkGraphicsPipelineShaderGroupsCreateInfoNV.html) - Structure specifying parameters of a newly created multi shader group pipeline
///# C Specifications
///The [`GraphicsPipelineShaderGroupsCreateInfoNV`] structure is defined
///as:
///```c
///// Provided by VK_NV_device_generated_commands
///typedef struct VkGraphicsPipelineShaderGroupsCreateInfoNV {
///    VkStructureType                             sType;
///    const void*                                 pNext;
///    uint32_t                                    groupCount;
///    const VkGraphicsShaderGroupCreateInfoNV*    pGroups;
///    uint32_t                                    pipelineCount;
///    const VkPipeline*                           pPipelines;
///} VkGraphicsPipelineShaderGroupsCreateInfoNV;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`group_count`] is the number of elements in the [`p_groups`] array.
/// - [`p_groups`] is a pointer to an array of [`GraphicsShaderGroupCreateInfoNV`] structures
///   specifying which state of the original [`GraphicsPipelineCreateInfo`] each shader group
///   overrides.
/// - [`pipeline_count`] is the number of elements in the [`p_pipelines`] array.
/// - [`p_pipelines`] is a pointer to an array of graphics [`Pipeline`] structures which are
///   referenced within the created pipeline, including all their shader groups.
///# Description
///When referencing shader groups by index, groups defined in the referenced
///pipelines are treated as if they were defined as additional entries in
///[`p_groups`].
///They are appended in the order they appear in the [`p_pipelines`] array and
///in the [`p_groups`] array when those pipelines were defined.The application **must** maintain
/// the lifetime of all such referenced pipelines
///based on the pipelines that make use of them.Valid Usage
/// - [`group_count`]**must** be at least `1` and as maximum
///   [`PhysicalDeviceDeviceGeneratedCommandsPropertiesNV::max_graphics_shader_group_count`]
/// - The sum of [`group_count`] including those groups added from referenced
///   [`p_pipelines`]**must** also be as maximum
///   [`PhysicalDeviceDeviceGeneratedCommandsPropertiesNV::max_graphics_shader_group_count`]
/// - The state of the first element of [`p_groups`]**must** match its equivalent within the
///   parent’s [`GraphicsPipelineCreateInfo`]
/// - Each element of [`p_groups`]**must** in combination with the rest of the pipeline state yield
///   a valid state configuration
/// - All elements of [`p_groups`]**must** use the same shader stage combinations unless any mesh
///   shader stage is used, then either combination of task and mesh or just mesh shader is valid
/// - Mesh and regular primitive shading stages cannot be mixed across [`p_groups`]
/// - Each element of [`p_pipelines`]**must** have been created with identical state to the pipeline
///   currently created except the state that can be overridden by
///   [`GraphicsShaderGroupCreateInfoNV`]
/// - The [[`PhysicalDeviceDeviceGeneratedCommandsFeaturesNV::device_generated_commands`]](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-deviceGeneratedCommands)
///   feature **must** be enabled
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_SHADER_GROUPS_CREATE_INFO_NV`
/// - [`p_groups`]**must** be a valid pointer to an array of [`group_count`] valid
///   [`GraphicsShaderGroupCreateInfoNV`] structures
/// - If [`pipeline_count`] is not `0`, [`p_pipelines`]**must** be a valid pointer to an array of
///   [`pipeline_count`] valid [`Pipeline`] handles
/// - [`group_count`]**must** be greater than `0`
///# Related
/// - [`VK_NV_device_generated_commands`]
/// - [`GraphicsShaderGroupCreateInfoNV`]
/// - [`Pipeline`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct GraphicsPipelineShaderGroupsCreateInfoNV<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`group_count`] is the number of elements in the [`p_groups`] array.
    group_count: u32,
    ///[`p_groups`] is a pointer to an array of
    ///[`GraphicsShaderGroupCreateInfoNV`] structures specifying which
    ///state of the original [`GraphicsPipelineCreateInfo`] each shader
    ///group overrides.
    p_groups: *mut GraphicsShaderGroupCreateInfoNV<'lt>,
    ///[`pipeline_count`] is the number of elements in the [`p_pipelines`]
    ///array.
    pipeline_count: u32,
    ///[`p_pipelines`] is a pointer to an array of graphics [`Pipeline`]
    ///structures which are referenced within the created pipeline, including
    ///all their shader groups.
    p_pipelines: *mut Pipeline,
}
///[VkBindShaderGroupIndirectCommandNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBindShaderGroupIndirectCommandNV.html) - Structure specifying input data for a single shader group command token
///# C Specifications
///The [`BindShaderGroupIndirectCommandNV`] structure specifies the input
///data for the `VK_INDIRECT_COMMANDS_TOKEN_TYPE_SHADER_GROUP_NV` token.
///```c
///// Provided by VK_NV_device_generated_commands
///typedef struct VkBindShaderGroupIndirectCommandNV {
///    uint32_t    groupIndex;
///} VkBindShaderGroupIndirectCommandNV;
///```
///# Members
/// - `index` specifies which shader group of the current bound graphics pipeline is used.
///# Description
///Valid Usage
/// - The current bound graphics pipeline, as well as the pipelines it may reference, **must** have
///   been created with `VK_PIPELINE_CREATE_INDIRECT_BINDABLE_BIT_NV`
/// - The `index`**must** be within range of the accessible shader groups of the current bound
///   graphics pipeline. See [`CmdBindPipelineShaderGroupNV`] for further details
///# Related
/// - [`VK_NV_device_generated_commands`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct BindShaderGroupIndirectCommandNV {
    ///No documentation found
    group_index: u32,
}
///[VkBindIndexBufferIndirectCommandNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBindIndexBufferIndirectCommandNV.html) - Structure specifying input data for a single index buffer command token
///# C Specifications
///The [`BindIndexBufferIndirectCommandNV`] structure specifies the input
///data for the `VK_INDIRECT_COMMANDS_TOKEN_TYPE_INDEX_BUFFER_NV` token.
///```c
///// Provided by VK_NV_device_generated_commands
///typedef struct VkBindIndexBufferIndirectCommandNV {
///    VkDeviceAddress    bufferAddress;
///    uint32_t           size;
///    VkIndexType        indexType;
///} VkBindIndexBufferIndirectCommandNV;
///```
///# Members
/// - [`buffer_address`] specifies a physical address of the [`Buffer`] used as index buffer.
/// - [`size`] is the byte size range which is available for this operation from the provided
///   address.
/// - [`index_type`] is a [`IndexType`] value specifying how indices are treated. Instead of the
///   Vulkan enum values, a custom `uint32_t` value **can** be mapped to an [`IndexType`] by
///   specifying the [`IndirectCommandsLayoutTokenNV::p_index_types`] and
///   [`IndirectCommandsLayoutTokenNV::p_index_type_values`] arrays.
///# Description
///Valid Usage
/// - The buffer’s usage flag from which the address was acquired **must** have the
///   `VK_BUFFER_USAGE_INDEX_BUFFER_BIT` bit set
/// - The [`buffer_address`]**must** be aligned to the [`index_type`] used
/// - Each element of the buffer from which the address was acquired and that is non-sparse **must**
///   be bound completely and contiguously to a single [`DeviceMemory`] object
///Valid Usage (Implicit)
/// - [`index_type`]**must** be a valid [`IndexType`] value
///# Related
/// - [`VK_NV_device_generated_commands`]
/// - [`DeviceAddress`]
/// - [`IndexType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct BindIndexBufferIndirectCommandNV {
    ///[`buffer_address`] specifies a physical address of the [`Buffer`]
    ///used as index buffer.
    buffer_address: DeviceAddress,
    ///[`size`] is the byte size range which is available for this operation
    ///from the provided address.
    size: u32,
    ///[`index_type`] is a [`IndexType`] value specifying how indices are
    ///treated.
    ///Instead of the Vulkan enum values, a custom `uint32_t` value **can** be
    ///mapped to an [`IndexType`] by specifying the
    ///[`IndirectCommandsLayoutTokenNV`]::`pIndexTypes` and
    ///[`IndirectCommandsLayoutTokenNV`]::`pIndexTypeValues` arrays.
    index_type: IndexType,
}
///[VkBindVertexBufferIndirectCommandNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBindVertexBufferIndirectCommandNV.html) - Structure specifying input data for a single vertex buffer command token
///# C Specifications
///The [`BindVertexBufferIndirectCommandNV`] structure specifies the input
///data for the `VK_INDIRECT_COMMANDS_TOKEN_TYPE_VERTEX_BUFFER_NV` token.
///```c
///// Provided by VK_NV_device_generated_commands
///typedef struct VkBindVertexBufferIndirectCommandNV {
///    VkDeviceAddress    bufferAddress;
///    uint32_t           size;
///    uint32_t           stride;
///} VkBindVertexBufferIndirectCommandNV;
///```
///# Members
/// - [`buffer_address`] specifies a physical address of the [`Buffer`] used as vertex input
///   binding.
/// - [`size`] is the byte size range which is available for this operation from the provided
///   address.
/// - [`stride`] is the byte size stride for this vertex input binding as in
///   [`VertexInputBindingDescription`]::[`stride`]. It is only used if
///   [`IndirectCommandsLayoutTokenNV::vertex_dynamic_stride`] was set, otherwise the stride is
///   inherited from the current bound graphics pipeline.
///# Description
///Valid Usage
/// - The buffer’s usage flag from which the address was acquired **must** have the
///   `VK_BUFFER_USAGE_VERTEX_BUFFER_BIT` bit set
/// - Each element of the buffer from which the address was acquired and that is non-sparse **must**
///   be bound completely and contiguously to a single [`DeviceMemory`] object
///# Related
/// - [`VK_NV_device_generated_commands`]
/// - [`DeviceAddress`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct BindVertexBufferIndirectCommandNV {
    ///[`buffer_address`] specifies a physical address of the [`Buffer`]
    ///used as vertex input binding.
    buffer_address: DeviceAddress,
    ///[`size`] is the byte size range which is available for this operation
    ///from the provided address.
    size: u32,
    ///[`stride`] is the byte size stride for this vertex input binding as in
    ///[`VertexInputBindingDescription`]::[`stride`].
    ///It is only used if
    ///[`IndirectCommandsLayoutTokenNV`]::`vertexDynamicStride` was
    ///set, otherwise the stride is inherited from the current bound graphics
    ///pipeline.
    stride: u32,
}
///[VkSetStateFlagsIndirectCommandNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSetStateFlagsIndirectCommandNV.html) - Structure specifying input data for a single state flag command token
///# C Specifications
///The [`SetStateFlagsIndirectCommandNV`] structure specifies the input
///data for the `VK_INDIRECT_COMMANDS_TOKEN_TYPE_STATE_FLAGS_NV` token.
///Which state is changed depends on the [`IndirectStateFlagBitsNV`]
///specified at [`IndirectCommandsLayoutNV`] creation time.
///```c
///// Provided by VK_NV_device_generated_commands
///typedef struct VkSetStateFlagsIndirectCommandNV {
///    uint32_t    data;
///} VkSetStateFlagsIndirectCommandNV;
///```
///# Members
/// - [`data`] encodes packed state that this command alters.  - Bit `0`: If set represents
///   `VK_FRONT_FACE_CLOCKWISE`, otherwise `VK_FRONT_FACE_COUNTER_CLOCKWISE`
///# Related
/// - [`VK_NV_device_generated_commands`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct SetStateFlagsIndirectCommandNV {
    ///[`data`] encodes packed state that this command alters.
    /// - Bit `0`: If set represents `VK_FRONT_FACE_CLOCKWISE`, otherwise
    ///   `VK_FRONT_FACE_COUNTER_CLOCKWISE`
    data: u32,
}
///[VkIndirectCommandsStreamNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkIndirectCommandsStreamNV.html) - Structure specifying input streams for generated command tokens
///# C Specifications
///The [`IndirectCommandsStreamNV`] structure specifies the input data for
///one or more tokens at processing time.
///```c
///// Provided by VK_NV_device_generated_commands
///typedef struct VkIndirectCommandsStreamNV {
///    VkBuffer        buffer;
///    VkDeviceSize    offset;
///} VkIndirectCommandsStreamNV;
///```
///# Members
/// - [`buffer`] specifies the [`Buffer`] storing the functional arguments for each sequence. These
///   arguments **can** be written by the device.
/// - [`offset`] specified an offset into [`buffer`] where the arguments start.
///# Description
///Valid Usage
/// - The [`buffer`]’s usage flag **must** have the `VK_BUFFER_USAGE_INDIRECT_BUFFER_BIT` bit set
/// - The [`offset`]**must** be aligned to
///   [`PhysicalDeviceDeviceGeneratedCommandsPropertiesNV::
///   min_indirect_commands_buffer_offset_alignment`]
/// - If [`buffer`] is non-sparse then it **must** be bound completely and contiguously to a single
///   [`DeviceMemory`] object
///Valid Usage (Implicit)
/// - [`buffer`]**must** be a valid [`Buffer`] handle
///# Related
/// - [`VK_NV_device_generated_commands`]
/// - [`Buffer`]
/// - [`DeviceSize`]
/// - [`GeneratedCommandsInfoNV`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct IndirectCommandsStreamNV {
    ///[`buffer`] specifies the [`Buffer`] storing the functional
    ///arguments for each sequence.
    ///These arguments **can** be written by the device.
    buffer: Buffer,
    ///[`offset`] specified an offset into [`buffer`] where the arguments
    ///start.
    offset: DeviceSize,
}
///[VkIndirectCommandsLayoutTokenNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkIndirectCommandsLayoutTokenNV.html) - Struct specifying the details of an indirect command layout token
///# C Specifications
///The [`IndirectCommandsLayoutTokenNV`] structure specifies details to the
///function arguments that need to be known at layout creation time:
///```c
///// Provided by VK_NV_device_generated_commands
///typedef struct VkIndirectCommandsLayoutTokenNV {
///    VkStructureType                  sType;
///    const void*                      pNext;
///    VkIndirectCommandsTokenTypeNV    tokenType;
///    uint32_t                         stream;
///    uint32_t                         offset;
///    uint32_t                         vertexBindingUnit;
///    VkBool32                         vertexDynamicStride;
///    VkPipelineLayout                 pushconstantPipelineLayout;
///    VkShaderStageFlags               pushconstantShaderStageFlags;
///    uint32_t                         pushconstantOffset;
///    uint32_t                         pushconstantSize;
///    VkIndirectStateFlagsNV           indirectStateFlags;
///    uint32_t                         indexTypeCount;
///    const VkIndexType*               pIndexTypes;
///    const uint32_t*                  pIndexTypeValues;
///} VkIndirectCommandsLayoutTokenNV;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`token_type`] specifies the token command type.
/// - [`stream`] is the index of the input stream containing the token argument data.
/// - [`offset`] is a relative starting offset within the input stream memory for the token argument
///   data.
/// - [`vertex_binding_unit`] is used for the vertex buffer binding command.
/// - [`vertex_dynamic_stride`] sets if the vertex buffer stride is provided by the binding command
///   rather than the current bound graphics pipeline state.
/// - [`pushconstant_pipeline_layout`] is the [`PipelineLayout`] used for the push constant command.
/// - [`pushconstant_shader_stage_flags`] are the shader stage flags used for the push constant
///   command.
/// - [`pushconstant_offset`] is the offset used for the push constant command.
/// - [`pushconstant_size`] is the size used for the push constant command.
/// - [`indirect_state_flags`] are the active states for the state flag command.
/// - [`index_type_count`] is the optional size of the [`p_index_types`] and [`p_index_type_values`]
///   array pairings. If not zero, it allows to register a custom `uint32_t` value to be treated as
///   specific [`IndexType`].
/// - [`p_index_types`] is the used [`IndexType`] for the corresponding `uint32_t` value entry in
///   [`p_index_type_values`].
///# Description
///Valid Usage
/// - [`stream`]**must** be smaller than [`IndirectCommandsLayoutCreateInfoNV::stream_count`]
/// - [`offset`]**must** be less than or equal to
///   [`PhysicalDeviceDeviceGeneratedCommandsPropertiesNV::max_indirect_commands_token_offset`]
/// - If [`token_type`] is `VK_INDIRECT_COMMANDS_TOKEN_TYPE_VERTEX_BUFFER_NV`,
///   [`vertex_binding_unit`]**must** stay within device supported limits for the appropriate
///   commands
/// - If [`token_type`] is `VK_INDIRECT_COMMANDS_TOKEN_TYPE_PUSH_CONSTANT_NV`,
///   [`pushconstant_pipeline_layout`]**must** be valid
/// - If [`token_type`] is `VK_INDIRECT_COMMANDS_TOKEN_TYPE_PUSH_CONSTANT_NV`,
///   [`pushconstant_offset`]**must** be a multiple of `4`
/// - If [`token_type`] is `VK_INDIRECT_COMMANDS_TOKEN_TYPE_PUSH_CONSTANT_NV`,
///   [`pushconstant_size`]**must** be a multiple of `4`
/// - If [`token_type`] is `VK_INDIRECT_COMMANDS_TOKEN_TYPE_PUSH_CONSTANT_NV`,
///   [`pushconstant_offset`]**must** be less than [`PhysicalDeviceLimits::max_push_constants_size`]
/// - If [`token_type`] is `VK_INDIRECT_COMMANDS_TOKEN_TYPE_PUSH_CONSTANT_NV`,
///   [`pushconstant_size`]**must** be less than or equal to
///   [`PhysicalDeviceLimits::max_push_constants_size`] minus [`pushconstant_offset`]
/// - If [`token_type`] is `VK_INDIRECT_COMMANDS_TOKEN_TYPE_PUSH_CONSTANT_NV`, for each byte in the
///   range specified by [`pushconstant_offset`] and [`pushconstant_size`] and for each shader stage
///   in [`pushconstant_shader_stage_flags`], there **must** be a push constant range in
///   [`pushconstant_pipeline_layout`] that includes that byte and that stage
/// - If [`token_type`] is `VK_INDIRECT_COMMANDS_TOKEN_TYPE_PUSH_CONSTANT_NV`, for each byte in the
///   range specified by [`pushconstant_offset`] and [`pushconstant_size`] and for each push
///   constant range that overlaps that byte, [`pushconstant_shader_stage_flags`]**must** include
///   all stages in that push constant range’s [`PushConstantRange::stage_flags`]
/// - If [`token_type`] is `VK_INDIRECT_COMMANDS_TOKEN_TYPE_STATE_FLAGS_NV`,
///   [`indirect_state_flags`]**must** not be `0`
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_INDIRECT_COMMANDS_LAYOUT_TOKEN_NV`
/// - [`p_next`]**must** be `NULL`
/// - [`token_type`]**must** be a valid [`IndirectCommandsTokenTypeNV`] value
/// - If [`pushconstant_pipeline_layout`] is not [`crate::utils::Handle::null`],
///   [`pushconstant_pipeline_layout`]**must** be a valid [`PipelineLayout`] handle
/// - [`pushconstant_shader_stage_flags`]**must** be a valid combination of [`ShaderStageFlagBits`]
///   values
/// - [`indirect_state_flags`]**must** be a valid combination of [`IndirectStateFlagBitsNV`] values
/// - If [`index_type_count`] is not `0`, [`p_index_types`]**must** be a valid pointer to an array
///   of [`index_type_count`] valid [`IndexType`] values
/// - If [`index_type_count`] is not `0`, [`p_index_type_values`]**must** be a valid pointer to an
///   array of [`index_type_count`]`uint32_t` values
///# Related
/// - [`VK_NV_device_generated_commands`]
/// - [`Bool32`]
/// - [`IndexType`]
/// - [`IndirectCommandsLayoutCreateInfoNV`]
/// - [`IndirectCommandsTokenTypeNV`]
/// - [`IndirectStateFlagsNV`]
/// - [`PipelineLayout`]
/// - [`ShaderStageFlags`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct IndirectCommandsLayoutTokenNV<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`token_type`] specifies the token command type.
    token_type: IndirectCommandsTokenTypeNV,
    ///[`stream`] is the index of the input stream containing the token
    ///argument data.
    stream: u32,
    ///[`offset`] is a relative starting offset within the input stream
    ///memory for the token argument data.
    offset: u32,
    ///[`vertex_binding_unit`] is used for the vertex buffer binding command.
    vertex_binding_unit: u32,
    ///[`vertex_dynamic_stride`] sets if the vertex buffer stride is provided
    ///by the binding command rather than the current bound graphics pipeline
    ///state.
    vertex_dynamic_stride: Bool32,
    ///[`pushconstant_pipeline_layout`] is the [`PipelineLayout`] used for
    ///the push constant command.
    pushconstant_pipeline_layout: PipelineLayout,
    ///[`pushconstant_shader_stage_flags`] are the shader stage flags used for
    ///the push constant command.
    pushconstant_shader_stage_flags: ShaderStageFlags,
    ///[`pushconstant_offset`] is the offset used for the push constant
    ///command.
    pushconstant_offset: u32,
    ///[`pushconstant_size`] is the size used for the push constant command.
    pushconstant_size: u32,
    ///[`indirect_state_flags`] are the active states for the state flag
    ///command.
    indirect_state_flags: IndirectStateFlagsNV,
    ///[`index_type_count`] is the optional size of the [`p_index_types`] and
    ///[`p_index_type_values`] array pairings.
    ///If not zero, it allows to register a custom `uint32_t` value to be
    ///treated as specific [`IndexType`].
    index_type_count: u32,
    ///[`p_index_types`] is the used [`IndexType`] for the corresponding
    ///`uint32_t` value entry in [`p_index_type_values`].
    p_index_types: *mut IndexType,
    ///No documentation found
    p_index_type_values: *mut u32,
}
///[VkIndirectCommandsLayoutCreateInfoNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkIndirectCommandsLayoutCreateInfoNV.html) - Structure specifying the parameters of a newly created indirect commands layout object
///# C Specifications
///The [`IndirectCommandsLayoutCreateInfoNV`] structure is defined as:
///```c
///// Provided by VK_NV_device_generated_commands
///typedef struct VkIndirectCommandsLayoutCreateInfoNV {
///    VkStructureType                           sType;
///    const void*                               pNext;
///    VkIndirectCommandsLayoutUsageFlagsNV      flags;
///    VkPipelineBindPoint                       pipelineBindPoint;
///    uint32_t                                  tokenCount;
///    const VkIndirectCommandsLayoutTokenNV*    pTokens;
///    uint32_t                                  streamCount;
///    const uint32_t*                           pStreamStrides;
///} VkIndirectCommandsLayoutCreateInfoNV;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`pipeline_bind_point`] is the [`PipelineBindPoint`] that this layout targets.
/// - [`flags`] is a bitmask of [`IndirectCommandsLayoutUsageFlagBitsNV`] specifying usage hints of
///   this layout.
/// - [`token_count`] is the length of the individual command sequence.
/// - [`p_tokens`] is an array describing each command token in detail. See
///   [`IndirectCommandsTokenTypeNV`] and [`IndirectCommandsLayoutTokenNV`] below for details.
/// - [`stream_count`] is the number of streams used to provide the token inputs.
/// - [`p_stream_strides`] is an array defining the byte stride for each input stream.
///# Description
///The following code illustrates some of the flags:
///```c
///void cmdProcessAllSequences(cmd, pipeline, indirectCommandsLayout, pIndirectCommandsTokens,
/// sequencesCount, indexbuffer, indexbufferOffset)
///{
///  for (s = 0; s < sequencesCount; s++)
///  {
///    sUsed = s;
///
///    if (indirectCommandsLayout.flags &
/// VK_INDIRECT_COMMANDS_LAYOUT_USAGE_INDEXED_SEQUENCES_BIT_NV) {
///      sUsed = indexbuffer.load_uint32( sUsed * sizeof(uint32_t) + indexbufferOffset);
///    }
///
///    if (indirectCommandsLayout.flags &
/// VK_INDIRECT_COMMANDS_LAYOUT_USAGE_UNORDERED_SEQUENCES_BIT_NV) {
///      sUsed = incoherent_implementation_dependent_permutation[ sUsed ];
///    }
///
///    cmdProcessSequence( cmd, pipeline, indirectCommandsLayout, pIndirectCommandsTokens, sUsed );
///  }
///}
///```
///Valid Usage
/// - The [`pipeline_bind_point`]**must** be `VK_PIPELINE_BIND_POINT_GRAPHICS`
/// - [`token_count`]**must** be greater than `0` and less than or equal to
///   [`PhysicalDeviceDeviceGeneratedCommandsPropertiesNV::max_indirect_commands_token_count`]
/// - If [`p_tokens`] contains an entry of `VK_INDIRECT_COMMANDS_TOKEN_TYPE_SHADER_GROUP_NV` it
///   **must** be the first element of the array and there **must** be only a single element of such
///   token type
/// - If [`p_tokens`] contains an entry of `VK_INDIRECT_COMMANDS_TOKEN_TYPE_STATE_FLAGS_NV` there
///   **must** be only a single element of such token type
/// - All state tokens in [`p_tokens`]**must** occur prior work provoking tokens
///   (`VK_INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_NV`, `VK_INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_INDEXED_NV`,
///   `VK_INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_TASKS_NV`)
/// - The content of [`p_tokens`]**must** include one single work provoking token that is compatible
///   with the [`pipeline_bind_point`]
/// - [`stream_count`]**must** be greater than `0` and less or equal to
///   [`PhysicalDeviceDeviceGeneratedCommandsPropertiesNV::max_indirect_commands_stream_count`]
/// - each element of [`p_stream_strides`]**must** be greater than `0`and less than or equal to
///   [`PhysicalDeviceDeviceGeneratedCommandsPropertiesNV::max_indirect_commands_stream_stride`].
///   Furthermore the alignment of each token input **must** be ensured
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_INDIRECT_COMMANDS_LAYOUT_CREATE_INFO_NV`
/// - [`p_next`]**must** be `NULL`
/// - [`flags`]**must** be a valid combination of [`IndirectCommandsLayoutUsageFlagBitsNV`] values
/// - [`pipeline_bind_point`]**must** be a valid [`PipelineBindPoint`] value
/// - [`p_tokens`]**must** be a valid pointer to an array of [`token_count`] valid
///   [`IndirectCommandsLayoutTokenNV`] structures
/// - [`p_stream_strides`]**must** be a valid pointer to an array of [`stream_count`]`uint32_t`
///   values
/// - [`token_count`]**must** be greater than `0`
/// - [`stream_count`]**must** be greater than `0`
///# Related
/// - [`VK_NV_device_generated_commands`]
/// - [`IndirectCommandsLayoutTokenNV`]
/// - [`IndirectCommandsLayoutUsageFlagsNV`]
/// - [`PipelineBindPoint`]
/// - [`StructureType`]
/// - [`CreateIndirectCommandsLayoutNV`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct IndirectCommandsLayoutCreateInfoNV<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`flags`] is a bitmask of
    ///[`IndirectCommandsLayoutUsageFlagBitsNV`] specifying usage hints of
    ///this layout.
    flags: IndirectCommandsLayoutUsageFlagsNV,
    ///[`pipeline_bind_point`] is the [`PipelineBindPoint`] that this
    ///layout targets.
    pipeline_bind_point: PipelineBindPoint,
    ///[`token_count`] is the length of the individual command sequence.
    token_count: u32,
    ///[`p_tokens`] is an array describing each command token in detail.
    ///See [`IndirectCommandsTokenTypeNV`] and
    ///[`IndirectCommandsLayoutTokenNV`] below for details.
    p_tokens: *mut IndirectCommandsLayoutTokenNV<'lt>,
    ///[`stream_count`] is the number of streams used to provide the token
    ///inputs.
    stream_count: u32,
    ///[`p_stream_strides`] is an array defining the byte stride for each input
    ///stream.
    p_stream_strides: *mut u32,
}
///[VkGeneratedCommandsInfoNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkGeneratedCommandsInfoNV.html) - Structure specifying parameters for the generation of commands
///# C Specifications
///```c
///// Provided by VK_NV_device_generated_commands
///typedef struct VkGeneratedCommandsInfoNV {
///    VkStructureType                      sType;
///    const void*                          pNext;
///    VkPipelineBindPoint                  pipelineBindPoint;
///    VkPipeline                           pipeline;
///    VkIndirectCommandsLayoutNV           indirectCommandsLayout;
///    uint32_t                             streamCount;
///    const VkIndirectCommandsStreamNV*    pStreams;
///    uint32_t                             sequencesCount;
///    VkBuffer                             preprocessBuffer;
///    VkDeviceSize                         preprocessOffset;
///    VkDeviceSize                         preprocessSize;
///    VkBuffer                             sequencesCountBuffer;
///    VkDeviceSize                         sequencesCountOffset;
///    VkBuffer                             sequencesIndexBuffer;
///    VkDeviceSize                         sequencesIndexOffset;
///} VkGeneratedCommandsInfoNV;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`pipeline_bind_point`] is the [`PipelineBindPoint`] used for the [`pipeline`].
/// - [`pipeline`] is the [`Pipeline`] used in the generation and execution process.
/// - [`indirect_commands_layout`] is the [`IndirectCommandsLayoutNV`] that provides the command
///   sequence to generate.
/// - [`stream_count`] defines the number of input streams
/// - [`p_streams`] is a pointer to an array of [`stream_count`][`IndirectCommandsStreamNV`]
///   structures providing the input data for the tokens used in [`indirect_commands_layout`].
/// - [`sequences_count`] is the maximum number of sequences to reserve. If
///   [`sequences_count_buffer`] is [`crate::utils::Handle::null`], this is also the actual number
///   of sequences generated.
/// - [`preprocess_buffer`] is the [`Buffer`] that is used for preprocessing the input data for
///   execution. If this structure is used with [`CmdExecuteGeneratedCommandsNV`] with its
///   `isPreprocessed` set to [`TRUE`], then the preprocessing step is skipped and data is only read
///   from this buffer.
/// - [`preprocess_offset`] is the byte offset into [`preprocess_buffer`] where the preprocessed
///   data is stored.
/// - [`preprocess_size`] is the maximum byte size within the [`preprocess_buffer`] after the
///   [`preprocess_offset`] that is available for preprocessing.
/// - [`sequences_count_buffer`] is a [`Buffer`] in which the actual number of sequences is provided
///   as single `uint32_t` value.
/// - [`sequences_count_offset`] is the byte offset into [`sequences_count_buffer`] where the count
///   value is stored.
/// - [`sequences_index_buffer`] is a [`Buffer`] that encodes the used sequence indices as
///   `uint32_t` array.
/// - [`sequences_index_offset`] is the byte offset into [`sequences_index_buffer`] where the index
///   values start.
///# Description
///Valid Usage
/// - The provided [`pipeline`]**must** match the pipeline bound at execution time
/// - If the [`indirect_commands_layout`] uses a token of
///   `VK_INDIRECT_COMMANDS_TOKEN_TYPE_SHADER_GROUP_NV`, then the [`pipeline`]**must** have been
///   created with multiple shader groups
/// - If the [`indirect_commands_layout`] uses a token of
///   `VK_INDIRECT_COMMANDS_TOKEN_TYPE_SHADER_GROUP_NV`, then the [`pipeline`]**must** have been
///   created with `VK_PIPELINE_CREATE_INDIRECT_BINDABLE_BIT_NV` set in
///   [`GraphicsPipelineCreateInfo::flags`]
/// - If the [`indirect_commands_layout`] uses a token of
///   `VK_INDIRECT_COMMANDS_TOKEN_TYPE_PUSH_CONSTANT_NV`, then the [`pipeline`]`s
///   [`PipelineLayout`]**must** match the
///   [`IndirectCommandsLayoutTokenNV::pushconstant_pipeline_layout`]
/// - [`stream_count`]**must** match the [`indirect_commands_layout`]’s [`stream_count`]
/// - [`sequences_count`]**must** be less or equal to
///   [`PhysicalDeviceDeviceGeneratedCommandsPropertiesNV::max_indirect_sequence_count`] and
///   [`GeneratedCommandsMemoryRequirementsInfoNV::max_sequences_count`] that was used to determine
///   the [`preprocess_size`]
/// - [`preprocess_buffer`]**must** have the `VK_BUFFER_USAGE_INDIRECT_BUFFER_BIT` bit set in its
///   usage flag
/// - [`preprocess_offset`]**must** be aligned to
///   [`PhysicalDeviceDeviceGeneratedCommandsPropertiesNV::
///   min_indirect_commands_buffer_offset_alignment`]
/// - If [`preprocess_buffer`] is non-sparse then it **must** be bound completely and contiguously
///   to a single [`DeviceMemory`] object
/// - [`preprocess_size`]**must** be at least equal to the memory requirement`s size returned by
///   [`GetGeneratedCommandsMemoryRequirementsNV`] using the matching inputs
///   ([`indirect_commands_layout`], …​) as within this structure
/// - [`sequences_count_buffer`]**can** be set if the actual used count of sequences is sourced from
///   the provided buffer. In that case the [`sequences_count`] serves as upper bound
/// - If [`sequences_count_buffer`] is not [`crate::utils::Handle::null`], its usage flag **must**
///   have the `VK_BUFFER_USAGE_INDIRECT_BUFFER_BIT` bit set
/// - If [`sequences_count_buffer`] is not [`crate::utils::Handle::null`],
///   [`sequences_count_offset`]**must** be aligned to
///   [`PhysicalDeviceDeviceGeneratedCommandsPropertiesNV::
///   min_sequences_count_buffer_offset_alignment`]
/// - If [`sequences_count_buffer`] is not [`crate::utils::Handle::null`] and is non-sparse then it
///   **must** be bound completely and contiguously to a single [`DeviceMemory`] object
/// - If [`indirect_commands_layout`]’s `VK_INDIRECT_COMMANDS_LAYOUT_USAGE_INDEXED_SEQUENCES_BIT_NV`
///   is set, [`sequences_index_buffer`]**must** be set otherwise it **must** be
///   [`crate::utils::Handle::null`]
/// - If [`sequences_index_buffer`] is not [`crate::utils::Handle::null`], its usage flag **must**
///   have the `VK_BUFFER_USAGE_INDIRECT_BUFFER_BIT` bit set
/// - If [`sequences_index_buffer`] is not [`crate::utils::Handle::null`],
///   [`sequences_index_offset`]**must** be aligned to
///   [`PhysicalDeviceDeviceGeneratedCommandsPropertiesNV::
///   min_sequences_index_buffer_offset_alignment`]
/// - If [`sequences_index_buffer`] is not [`crate::utils::Handle::null`] and is non-sparse then it
///   **must** be bound completely and contiguously to a single [`DeviceMemory`] object
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_GENERATED_COMMANDS_INFO_NV`
/// - [`p_next`]**must** be `NULL`
/// - [`pipeline_bind_point`]**must** be a valid [`PipelineBindPoint`] value
/// - [`pipeline`]**must** be a valid [`Pipeline`] handle
/// - [`indirect_commands_layout`]**must** be a valid [`IndirectCommandsLayoutNV`] handle
/// - [`p_streams`]**must** be a valid pointer to an array of [`stream_count`] valid
///   [`IndirectCommandsStreamNV`] structures
/// - [`preprocess_buffer`]**must** be a valid [`Buffer`] handle
/// - If [`sequences_count_buffer`] is not [`crate::utils::Handle::null`],
///   [`sequences_count_buffer`]**must** be a valid [`Buffer`] handle
/// - If [`sequences_index_buffer`] is not [`crate::utils::Handle::null`],
///   [`sequences_index_buffer`]**must** be a valid [`Buffer`] handle
/// - [`stream_count`]**must** be greater than `0`
/// - Each of [`indirect_commands_layout`], [`pipeline`], [`preprocess_buffer`],
///   [`sequences_count_buffer`], and [`sequences_index_buffer`] that are valid handles of
///   non-ignored parameters **must** have been created, allocated, or retrieved from the same
///   [`Device`]
///# Related
/// - [`VK_NV_device_generated_commands`]
/// - [`Buffer`]
/// - [`DeviceSize`]
/// - [`IndirectCommandsLayoutNV`]
/// - [`IndirectCommandsStreamNV`]
/// - [`Pipeline`]
/// - [`PipelineBindPoint`]
/// - [`StructureType`]
/// - [`CmdExecuteGeneratedCommandsNV`]
/// - [`CmdPreprocessGeneratedCommandsNV`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct GeneratedCommandsInfoNV<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`pipeline_bind_point`] is the [`PipelineBindPoint`] used for the
    ///[`pipeline`].
    pipeline_bind_point: PipelineBindPoint,
    ///[`pipeline`] is the [`Pipeline`] used in the generation and
    ///execution process.
    pipeline: Pipeline,
    ///[`indirect_commands_layout`] is the [`IndirectCommandsLayoutNV`]
    ///that provides the command sequence to generate.
    indirect_commands_layout: IndirectCommandsLayoutNV,
    ///[`stream_count`] defines the number of input streams
    stream_count: u32,
    ///[`p_streams`] is a pointer to an array of [`stream_count`][`IndirectCommandsStreamNV`]
    /// structures providing the input data for the tokens used in [`indirect_commands_layout`].
    p_streams: *mut IndirectCommandsStreamNV,
    ///[`sequences_count`] is the maximum number of sequences to reserve.
    ///If [`sequences_count_buffer`] is [`crate::utils::Handle::null`], this is also the
    ///actual number of sequences generated.
    sequences_count: u32,
    ///[`preprocess_buffer`] is the [`Buffer`] that is used for
    ///preprocessing the input data for execution.
    ///If this structure is used with [`CmdExecuteGeneratedCommandsNV`]
    ///with its `isPreprocessed` set to [`TRUE`], then the preprocessing
    ///step is skipped and data is only read from this buffer.
    preprocess_buffer: Buffer,
    ///[`preprocess_offset`] is the byte offset into [`preprocess_buffer`]
    ///where the preprocessed data is stored.
    preprocess_offset: DeviceSize,
    ///[`preprocess_size`] is the maximum byte size within the
    ///[`preprocess_buffer`] after the [`preprocess_offset`] that is
    ///available for preprocessing.
    preprocess_size: DeviceSize,
    ///[`sequences_count_buffer`] is a [`Buffer`] in which the actual
    ///number of sequences is provided as single `uint32_t` value.
    sequences_count_buffer: Buffer,
    ///[`sequences_count_offset`] is the byte offset into
    ///[`sequences_count_buffer`] where the count value is stored.
    sequences_count_offset: DeviceSize,
    ///[`sequences_index_buffer`] is a [`Buffer`] that encodes the used
    ///sequence indices as `uint32_t` array.
    sequences_index_buffer: Buffer,
    ///[`sequences_index_offset`] is the byte offset into
    ///[`sequences_index_buffer`] where the index values start.
    sequences_index_offset: DeviceSize,
}
///[VkGeneratedCommandsMemoryRequirementsInfoNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkGeneratedCommandsMemoryRequirementsInfoNV.html) - Structure specifying parameters for the reservation of preprocess buffer space
///# C Specifications
///```c
///// Provided by VK_NV_device_generated_commands
///typedef struct VkGeneratedCommandsMemoryRequirementsInfoNV {
///    VkStructureType               sType;
///    const void*                   pNext;
///    VkPipelineBindPoint           pipelineBindPoint;
///    VkPipeline                    pipeline;
///    VkIndirectCommandsLayoutNV    indirectCommandsLayout;
///    uint32_t                      maxSequencesCount;
///} VkGeneratedCommandsMemoryRequirementsInfoNV;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`pipeline_bind_point`] is the [`PipelineBindPoint`] of the [`pipeline`] that this buffer
///   memory is intended to be used with during the execution.
/// - [`pipeline`] is the [`Pipeline`] that this buffer memory is intended to be used with during
///   the execution.
/// - [`indirect_commands_layout`] is the [`IndirectCommandsLayoutNV`] that this buffer memory is
///   intended to be used with.
/// - [`max_sequences_count`] is the maximum number of sequences that this buffer memory in
///   combination with the other state provided **can** be used with.
///# Description
///Valid Usage
/// - [`max_sequences_count`]**must** be less or equal to
///   [`PhysicalDeviceDeviceGeneratedCommandsPropertiesNV::max_indirect_sequence_count`]
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_GENERATED_COMMANDS_MEMORY_REQUIREMENTS_INFO_NV`
/// - [`p_next`]**must** be `NULL`
/// - [`pipeline_bind_point`]**must** be a valid [`PipelineBindPoint`] value
/// - [`pipeline`]**must** be a valid [`Pipeline`] handle
/// - [`indirect_commands_layout`]**must** be a valid [`IndirectCommandsLayoutNV`] handle
/// - Both of [`indirect_commands_layout`], and [`pipeline`]**must** have been created, allocated,
///   or retrieved from the same [`Device`]
///# Related
/// - [`VK_NV_device_generated_commands`]
/// - [`IndirectCommandsLayoutNV`]
/// - [`Pipeline`]
/// - [`PipelineBindPoint`]
/// - [`StructureType`]
/// - [`GetGeneratedCommandsMemoryRequirementsNV`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct GeneratedCommandsMemoryRequirementsInfoNV<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`pipeline_bind_point`] is the [`PipelineBindPoint`] of the
    ///[`pipeline`] that this buffer memory is intended to be used with
    ///during the execution.
    pipeline_bind_point: PipelineBindPoint,
    ///[`pipeline`] is the [`Pipeline`] that this buffer memory is
    ///intended to be used with during the execution.
    pipeline: Pipeline,
    ///[`indirect_commands_layout`] is the [`IndirectCommandsLayoutNV`]
    ///that this buffer memory is intended to be used with.
    indirect_commands_layout: IndirectCommandsLayoutNV,
    ///[`max_sequences_count`] is the maximum number of sequences that this
    ///buffer memory in combination with the other state provided **can** be used
    ///with.
    max_sequences_count: u32,
}
