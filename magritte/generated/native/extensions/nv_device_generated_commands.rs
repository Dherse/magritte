//!# [VK_NV_device_generated_commands](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_NV_device_generated_commands.html)
# ! [doc = include_str ! ("../../../../doc/extensions/nv_device_generated_commands/VK_NV_device_generated_commands.md")]
use crate::{
    cstr,
    vulkan1_0::{
        AllocationCallbacks, BaseInStructure, BaseOutStructure, Bool32, Buffer, CommandBuffer, Device, DeviceAddress,
        DeviceSize, IndexType, Pipeline, PipelineBindPoint, PipelineLayout, PipelineShaderStageCreateInfo,
        PipelineTessellationStateCreateInfo, PipelineVertexInputStateCreateInfo, ShaderStageFlags, StructureType,
        VulkanResultCodes,
    },
    vulkan1_1::MemoryRequirements2,
};
use std::ffi::CStr;
///# [VkPhysicalDeviceDeviceGeneratedCommandsFeaturesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDeviceGeneratedCommandsFeaturesNV.html)
# [doc = include_str ! ("../../../../doc/extensions/nv_device_generated_commands/VkPhysicalDeviceDeviceGeneratedCommandsFeaturesNV.md")]
#[doc(alias = "VkPhysicalDeviceDeviceGeneratedCommandsFeaturesNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceDeviceGeneratedCommandsFeaturesNV {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "deviceGeneratedCommands")]
    device_generated_commands: Bool32,
}
///# [VkPhysicalDeviceDeviceGeneratedCommandsPropertiesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDeviceGeneratedCommandsPropertiesNV.html)
# [doc = include_str ! ("../../../../doc/extensions/nv_device_generated_commands/VkPhysicalDeviceDeviceGeneratedCommandsPropertiesNV.md")]
#[doc(alias = "VkPhysicalDeviceDeviceGeneratedCommandsPropertiesNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceDeviceGeneratedCommandsPropertiesNV {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "maxGraphicsShaderGroupCount")]
    max_graphics_shader_group_count: u32,
    #[doc(alias = "maxIndirectSequenceCount")]
    max_indirect_sequence_count: u32,
    #[doc(alias = "maxIndirectCommandsTokenCount")]
    max_indirect_commands_token_count: u32,
    #[doc(alias = "maxIndirectCommandsStreamCount")]
    max_indirect_commands_stream_count: u32,
    #[doc(alias = "maxIndirectCommandsTokenOffset")]
    max_indirect_commands_token_offset: u32,
    #[doc(alias = "maxIndirectCommandsStreamStride")]
    max_indirect_commands_stream_stride: u32,
    #[doc(alias = "minSequencesCountBufferOffsetAlignment")]
    min_sequences_count_buffer_offset_alignment: u32,
    #[doc(alias = "minSequencesIndexBufferOffsetAlignment")]
    min_sequences_index_buffer_offset_alignment: u32,
    #[doc(alias = "minIndirectCommandsBufferOffsetAlignment")]
    min_indirect_commands_buffer_offset_alignment: u32,
}
///# [VkGraphicsShaderGroupCreateInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkGraphicsShaderGroupCreateInfoNV.html)
# [doc = include_str ! ("../../../../doc/extensions/nv_device_generated_commands/VkGraphicsShaderGroupCreateInfoNV.md")]
#[doc(alias = "VkGraphicsShaderGroupCreateInfoNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct GraphicsShaderGroupCreateInfoNV {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "stageCount")]
    stage_count: u32,
    #[doc(alias = "pStages")]
    stages: *const PipelineShaderStageCreateInfo,
    #[doc(alias = "pVertexInputState")]
    vertex_input_state: *const PipelineVertexInputStateCreateInfo,
    #[doc(alias = "pTessellationState")]
    tessellation_state: *const PipelineTessellationStateCreateInfo,
}
///# [VkGraphicsPipelineShaderGroupsCreateInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkGraphicsPipelineShaderGroupsCreateInfoNV.html)
# [doc = include_str ! ("../../../../doc/extensions/nv_device_generated_commands/VkGraphicsPipelineShaderGroupsCreateInfoNV.md")]
#[doc(alias = "VkGraphicsPipelineShaderGroupsCreateInfoNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct GraphicsPipelineShaderGroupsCreateInfoNV {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "groupCount")]
    group_count: u32,
    #[doc(alias = "pGroups")]
    groups: *const GraphicsShaderGroupCreateInfoNV,
    #[doc(alias = "pipelineCount")]
    pipeline_count: u32,
    #[doc(alias = "pPipelines")]
    pipelines: *const Pipeline,
}
///# [VkBindShaderGroupIndirectCommandNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBindShaderGroupIndirectCommandNV.html)
# [doc = include_str ! ("../../../../doc/extensions/nv_device_generated_commands/VkBindShaderGroupIndirectCommandNV.md")]
#[doc(alias = "VkBindShaderGroupIndirectCommandNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct BindShaderGroupIndirectCommandNV {
    #[doc(alias = "groupIndex")]
    group_index: u32,
}
///# [VkBindIndexBufferIndirectCommandNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBindIndexBufferIndirectCommandNV.html)
# [doc = include_str ! ("../../../../doc/extensions/nv_device_generated_commands/VkBindIndexBufferIndirectCommandNV.md")]
#[doc(alias = "VkBindIndexBufferIndirectCommandNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct BindIndexBufferIndirectCommandNV {
    #[doc(alias = "bufferAddress")]
    buffer_address: DeviceAddress,
    size: u32,
    #[doc(alias = "indexType")]
    index_type: IndexType,
}
///# [VkBindVertexBufferIndirectCommandNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBindVertexBufferIndirectCommandNV.html)
# [doc = include_str ! ("../../../../doc/extensions/nv_device_generated_commands/VkBindVertexBufferIndirectCommandNV.md")]
#[doc(alias = "VkBindVertexBufferIndirectCommandNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct BindVertexBufferIndirectCommandNV {
    #[doc(alias = "bufferAddress")]
    buffer_address: DeviceAddress,
    size: u32,
    stride: u32,
}
///# [VkSetStateFlagsIndirectCommandNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSetStateFlagsIndirectCommandNV.html)
# [doc = include_str ! ("../../../../doc/extensions/nv_device_generated_commands/VkSetStateFlagsIndirectCommandNV.md")]
#[doc(alias = "VkSetStateFlagsIndirectCommandNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SetStateFlagsIndirectCommandNV {
    data: u32,
}
///# [VkIndirectCommandsStreamNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkIndirectCommandsStreamNV.html)
# [doc = include_str ! ("../../../../doc/extensions/nv_device_generated_commands/VkIndirectCommandsStreamNV.md")]
#[doc(alias = "VkIndirectCommandsStreamNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct IndirectCommandsStreamNV {
    buffer: Buffer,
    offset: DeviceSize,
}
///# [VkIndirectCommandsLayoutTokenNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkIndirectCommandsLayoutTokenNV.html)
# [doc = include_str ! ("../../../../doc/extensions/nv_device_generated_commands/VkIndirectCommandsLayoutTokenNV.md")]
#[doc(alias = "VkIndirectCommandsLayoutTokenNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct IndirectCommandsLayoutTokenNV {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "tokenType")]
    token_type: IndirectCommandsTokenTypeNV,
    stream: u32,
    offset: u32,
    #[doc(alias = "vertexBindingUnit")]
    vertex_binding_unit: u32,
    #[doc(alias = "vertexDynamicStride")]
    vertex_dynamic_stride: Bool32,
    #[doc(alias = "pushconstantPipelineLayout")]
    pushconstant_pipeline_layout: PipelineLayout,
    #[doc(alias = "pushconstantShaderStageFlags")]
    pushconstant_shader_stage_flags: ShaderStageFlags,
    #[doc(alias = "pushconstantOffset")]
    pushconstant_offset: u32,
    #[doc(alias = "pushconstantSize")]
    pushconstant_size: u32,
    #[doc(alias = "indirectStateFlags")]
    indirect_state_flags: IndirectStateFlagsNV,
    #[doc(alias = "indexTypeCount")]
    index_type_count: u32,
    #[doc(alias = "pIndexTypes")]
    index_types: *const IndexType,
    #[doc(alias = "pIndexTypeValues")]
    index_type_values: *const u32,
}
///# [VkIndirectCommandsLayoutCreateInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkIndirectCommandsLayoutCreateInfoNV.html)
# [doc = include_str ! ("../../../../doc/extensions/nv_device_generated_commands/VkIndirectCommandsLayoutCreateInfoNV.md")]
#[doc(alias = "VkIndirectCommandsLayoutCreateInfoNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct IndirectCommandsLayoutCreateInfoNV {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    flags: IndirectCommandsLayoutUsageFlagsNV,
    #[doc(alias = "pipelineBindPoint")]
    pipeline_bind_point: PipelineBindPoint,
    #[doc(alias = "tokenCount")]
    token_count: u32,
    #[doc(alias = "pTokens")]
    tokens: *const IndirectCommandsLayoutTokenNV,
    #[doc(alias = "streamCount")]
    stream_count: u32,
    #[doc(alias = "pStreamStrides")]
    stream_strides: *const u32,
}
///# [VkGeneratedCommandsInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkGeneratedCommandsInfoNV.html)
# [doc = include_str ! ("../../../../doc/extensions/nv_device_generated_commands/VkGeneratedCommandsInfoNV.md")]
#[doc(alias = "VkGeneratedCommandsInfoNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct GeneratedCommandsInfoNV {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "pipelineBindPoint")]
    pipeline_bind_point: PipelineBindPoint,
    pipeline: Pipeline,
    #[doc(alias = "indirectCommandsLayout")]
    indirect_commands_layout: IndirectCommandsLayoutNV,
    #[doc(alias = "streamCount")]
    stream_count: u32,
    #[doc(alias = "pStreams")]
    streams: *const IndirectCommandsStreamNV,
    #[doc(alias = "sequencesCount")]
    sequences_count: u32,
    #[doc(alias = "preprocessBuffer")]
    preprocess_buffer: Buffer,
    #[doc(alias = "preprocessOffset")]
    preprocess_offset: DeviceSize,
    #[doc(alias = "preprocessSize")]
    preprocess_size: DeviceSize,
    #[doc(alias = "sequencesCountBuffer")]
    sequences_count_buffer: Buffer,
    #[doc(alias = "sequencesCountOffset")]
    sequences_count_offset: DeviceSize,
    #[doc(alias = "sequencesIndexBuffer")]
    sequences_index_buffer: Buffer,
    #[doc(alias = "sequencesIndexOffset")]
    sequences_index_offset: DeviceSize,
}
///# [VkGeneratedCommandsMemoryRequirementsInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkGeneratedCommandsMemoryRequirementsInfoNV.html)
# [doc = include_str ! ("../../../../doc/extensions/nv_device_generated_commands/VkGeneratedCommandsMemoryRequirementsInfoNV.md")]
#[doc(alias = "VkGeneratedCommandsMemoryRequirementsInfoNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct GeneratedCommandsMemoryRequirementsInfoNV {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "pipelineBindPoint")]
    pipeline_bind_point: PipelineBindPoint,
    pipeline: Pipeline,
    #[doc(alias = "indirectCommandsLayout")]
    indirect_commands_layout: IndirectCommandsLayoutNV,
    #[doc(alias = "maxSequencesCount")]
    max_sequences_count: u32,
}
///# [VkIndirectCommandsLayoutNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkIndirectCommandsLayoutNV.html)
# [doc = include_str ! ("../../../../doc/extensions/nv_device_generated_commands/VkIndirectCommandsLayoutNV.md")]
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(alias = "VkIndirectCommandsLayoutNV")]
#[repr(transparent)]
pub struct IndirectCommandsLayoutNV(u64);
impl IndirectCommandsLayoutNV {
    pub const fn null() -> Self {
        Self(0)
    }
}
impl const Default for IndirectCommandsLayoutNV {
    fn default() -> Self {
        Self::null()
    }
}
///# [VkIndirectCommandsLayoutUsageFlagBitsNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkIndirectCommandsLayoutUsageFlagBitsNV.html)
# [doc = include_str ! ("../../../../doc/extensions/nv_device_generated_commands/VkIndirectCommandsLayoutUsageFlagBitsNV.md")]
#[doc(alias = "VkIndirectCommandsLayoutUsageFlagsNV")]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct IndirectCommandsLayoutUsageFlagsNV(u32);
impl IndirectCommandsLayoutUsageFlagsNV {
    #[doc(alias = "VK_INDIRECT_COMMANDS_LAYOUT_USAGE_EXPLICIT_PREPROCESS_BIT_NV")]
    pub const EXPLICIT_PREPROCESS: Self = Self(1);
    #[doc(alias = "VK_INDIRECT_COMMANDS_LAYOUT_USAGE_INDEXED_SEQUENCES_BIT_NV")]
    pub const INDEXED_SEQUENCES: Self = Self(2);
    #[doc(alias = "VK_INDIRECT_COMMANDS_LAYOUT_USAGE_UNORDERED_SEQUENCES_BIT_NV")]
    pub const UNORDERED_SEQUENCES: Self = Self(4);
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
            all |= Self::EXPLICIT_PREPROCESS;
        }
        {
            all |= Self::INDEXED_SEQUENCES;
        }
        {
            all |= Self::UNORDERED_SEQUENCES;
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
impl const std::ops::BitOr for IndirectCommandsLayoutUsageFlagsNV {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl const std::ops::BitOrAssign for IndirectCommandsLayoutUsageFlagsNV {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for IndirectCommandsLayoutUsageFlagsNV {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl const std::ops::BitXorAssign for IndirectCommandsLayoutUsageFlagsNV {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for IndirectCommandsLayoutUsageFlagsNV {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl const std::ops::BitAndAssign for IndirectCommandsLayoutUsageFlagsNV {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for IndirectCommandsLayoutUsageFlagsNV {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl const std::ops::SubAssign for IndirectCommandsLayoutUsageFlagsNV {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for IndirectCommandsLayoutUsageFlagsNV {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<IndirectCommandsLayoutUsageFlagsNV> for IndirectCommandsLayoutUsageFlagsNV {
    fn extend<T: IntoIterator<Item = IndirectCommandsLayoutUsageFlagsNV>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl FromIterator<IndirectCommandsLayoutUsageFlagsNV> for IndirectCommandsLayoutUsageFlagsNV {
    fn from_iter<T: IntoIterator<Item = IndirectCommandsLayoutUsageFlagsNV>>(
        iterator: T,
    ) -> IndirectCommandsLayoutUsageFlagsNV {
        let mut out = Self::empty();
        <Self as Extend<IndirectCommandsLayoutUsageFlagsNV>>::extend(&mut out, iterator);
        out
    }
}
impl const Default for IndirectCommandsLayoutUsageFlagsNV {
    fn default() -> Self {
        Self::empty()
    }
}
impl const From<IndirectCommandsLayoutUsageFlagBitsNV> for IndirectCommandsLayoutUsageFlagsNV {
    fn from(bit: IndirectCommandsLayoutUsageFlagBitsNV) -> Self {
        unsafe { Self::from_bits_unchecked(bit.bits()) }
    }
}
impl Extend<IndirectCommandsLayoutUsageFlagBitsNV> for IndirectCommandsLayoutUsageFlagsNV {
    fn extend<T: IntoIterator<Item = IndirectCommandsLayoutUsageFlagBitsNV>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, Self::from(i));
        }
    }
}
impl FromIterator<IndirectCommandsLayoutUsageFlagBitsNV> for IndirectCommandsLayoutUsageFlagsNV {
    fn from_iter<T: IntoIterator<Item = IndirectCommandsLayoutUsageFlagBitsNV>>(
        iterator: T,
    ) -> IndirectCommandsLayoutUsageFlagsNV {
        let mut out = Self::empty();
        <Self as Extend<IndirectCommandsLayoutUsageFlagBitsNV>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for IndirectCommandsLayoutUsageFlagsNV {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(IndirectCommandsLayoutUsageFlagsNV);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == IndirectCommandsLayoutUsageFlagsNV::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(IndirectCommandsLayoutUsageFlagsNV::EXPLICIT_PREPROCESS) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(EXPLICIT_PREPROCESS))?;
                    }
                    if self.0.contains(IndirectCommandsLayoutUsageFlagsNV::INDEXED_SEQUENCES) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(INDEXED_SEQUENCES))?;
                    }
                    if self.0.contains(IndirectCommandsLayoutUsageFlagsNV::UNORDERED_SEQUENCES) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(UNORDERED_SEQUENCES))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(IndirectCommandsLayoutUsageFlagsNV))
            .field(&Flags(*self))
            .finish()
    }
}
///# [VkIndirectStateFlagBitsNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkIndirectStateFlagBitsNV.html)
# [doc = include_str ! ("../../../../doc/extensions/nv_device_generated_commands/VkIndirectStateFlagBitsNV.md")]
#[doc(alias = "VkIndirectStateFlagsNV")]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct IndirectStateFlagsNV(u32);
impl IndirectStateFlagsNV {
    #[doc(alias = "VK_INDIRECT_STATE_FLAG_FRONTFACE_BIT_NV")]
    pub const FLAG_FRONTFACE: Self = Self(1);
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
            all |= Self::FLAG_FRONTFACE;
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
impl const std::ops::BitOr for IndirectStateFlagsNV {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl const std::ops::BitOrAssign for IndirectStateFlagsNV {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for IndirectStateFlagsNV {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl const std::ops::BitXorAssign for IndirectStateFlagsNV {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for IndirectStateFlagsNV {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl const std::ops::BitAndAssign for IndirectStateFlagsNV {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for IndirectStateFlagsNV {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl const std::ops::SubAssign for IndirectStateFlagsNV {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for IndirectStateFlagsNV {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<IndirectStateFlagsNV> for IndirectStateFlagsNV {
    fn extend<T: IntoIterator<Item = IndirectStateFlagsNV>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl FromIterator<IndirectStateFlagsNV> for IndirectStateFlagsNV {
    fn from_iter<T: IntoIterator<Item = IndirectStateFlagsNV>>(iterator: T) -> IndirectStateFlagsNV {
        let mut out = Self::empty();
        <Self as Extend<IndirectStateFlagsNV>>::extend(&mut out, iterator);
        out
    }
}
impl const Default for IndirectStateFlagsNV {
    fn default() -> Self {
        Self::empty()
    }
}
impl const From<IndirectStateFlagBitsNV> for IndirectStateFlagsNV {
    fn from(bit: IndirectStateFlagBitsNV) -> Self {
        unsafe { Self::from_bits_unchecked(bit.bits()) }
    }
}
impl Extend<IndirectStateFlagBitsNV> for IndirectStateFlagsNV {
    fn extend<T: IntoIterator<Item = IndirectStateFlagBitsNV>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, Self::from(i));
        }
    }
}
impl FromIterator<IndirectStateFlagBitsNV> for IndirectStateFlagsNV {
    fn from_iter<T: IntoIterator<Item = IndirectStateFlagBitsNV>>(iterator: T) -> IndirectStateFlagsNV {
        let mut out = Self::empty();
        <Self as Extend<IndirectStateFlagBitsNV>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for IndirectStateFlagsNV {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(IndirectStateFlagsNV);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == IndirectStateFlagsNV::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(IndirectStateFlagsNV::FLAG_FRONTFACE) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(FLAG_FRONTFACE))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(IndirectStateFlagsNV))
            .field(&Flags(*self))
            .finish()
    }
}
#[doc(alias = "VK_NV_DEVICE_GENERATED_COMMANDS_SPEC_VERSION")]
pub const NV_DEVICE_GENERATED_COMMANDS_SPEC_VERSION: u32 = 3;
#[doc(alias = "VK_NV_DEVICE_GENERATED_COMMANDS_EXTENSION_NAME")]
pub const NV_DEVICE_GENERATED_COMMANDS_EXTENSION_NAME: &'static CStr = cstr!("VK_NV_device_generated_commands");
///# [VkIndirectCommandsLayoutUsageFlagBitsNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkIndirectCommandsLayoutUsageFlagBitsNV.html)
# [doc = include_str ! ("../../../../doc/extensions/nv_device_generated_commands/VkIndirectCommandsLayoutUsageFlagBitsNV.md")]
#[doc(alias = "VkIndirectCommandsLayoutUsageFlagBitsNV")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct IndirectCommandsLayoutUsageFlagBitsNV(u32);
impl IndirectCommandsLayoutUsageFlagBitsNV {
    #[doc(alias = "VK_INDIRECT_COMMANDS_LAYOUT_USAGE_EXPLICIT_PREPROCESS_BIT_NV")]
    pub const EXPLICIT_PREPROCESS: Self = Self(1);
    #[doc(alias = "VK_INDIRECT_COMMANDS_LAYOUT_USAGE_INDEXED_SEQUENCES_BIT_NV")]
    pub const INDEXED_SEQUENCES: Self = Self(2);
    #[doc(alias = "VK_INDIRECT_COMMANDS_LAYOUT_USAGE_UNORDERED_SEQUENCES_BIT_NV")]
    pub const UNORDERED_SEQUENCES: Self = Self(4);
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
            x if x == Self::EXPLICIT_PREPROCESS.bits() => Some(Self(x)),
            x if x == Self::INDEXED_SEQUENCES.bits() => Some(Self(x)),
            x if x == Self::UNORDERED_SEQUENCES.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
}
///# [VkIndirectStateFlagBitsNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkIndirectStateFlagBitsNV.html)
# [doc = include_str ! ("../../../../doc/extensions/nv_device_generated_commands/VkIndirectStateFlagBitsNV.md")]
#[doc(alias = "VkIndirectStateFlagBitsNV")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct IndirectStateFlagBitsNV(u32);
impl IndirectStateFlagBitsNV {
    #[doc(alias = "VK_INDIRECT_STATE_FLAG_FRONTFACE_BIT_NV")]
    pub const FLAG_FRONTFACE: Self = Self(1);
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
            x if x == Self::FLAG_FRONTFACE.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
}
///# [VkIndirectCommandsTokenTypeNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkIndirectCommandsTokenTypeNV.html)
# [doc = include_str ! ("../../../../doc/extensions/nv_device_generated_commands/VkIndirectCommandsTokenTypeNV.md")]
#[doc(alias = "VkIndirectCommandsTokenTypeNV")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct IndirectCommandsTokenTypeNV(i32);
impl IndirectCommandsTokenTypeNV {
    #[doc(alias = "VK_INDIRECT_COMMANDS_TOKEN_TYPE_SHADER_GROUP_NV")]
    pub const SHADER_GROUP: Self = Self(0);
    #[doc(alias = "VK_INDIRECT_COMMANDS_TOKEN_TYPE_STATE_FLAGS_NV")]
    pub const STATE_FLAGS: Self = Self(1);
    #[doc(alias = "VK_INDIRECT_COMMANDS_TOKEN_TYPE_INDEX_BUFFER_NV")]
    pub const INDEX_BUFFER: Self = Self(2);
    #[doc(alias = "VK_INDIRECT_COMMANDS_TOKEN_TYPE_VERTEX_BUFFER_NV")]
    pub const VERTEX_BUFFER: Self = Self(3);
    #[doc(alias = "VK_INDIRECT_COMMANDS_TOKEN_TYPE_PUSH_CONSTANT_NV")]
    pub const PUSH_CONSTANT: Self = Self(4);
    #[doc(alias = "VK_INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_INDEXED_NV")]
    pub const DRAW_INDEXED: Self = Self(5);
    #[doc(alias = "VK_INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_NV")]
    pub const DRAW: Self = Self(6);
    #[doc(alias = "VK_INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_TASKS_NV")]
    pub const DRAW_TASKS: Self = Self(7);
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
            x if x == Self::SHADER_GROUP.bits() => Some(Self(x)),
            x if x == Self::STATE_FLAGS.bits() => Some(Self(x)),
            x if x == Self::INDEX_BUFFER.bits() => Some(Self(x)),
            x if x == Self::VERTEX_BUFFER.bits() => Some(Self(x)),
            x if x == Self::PUSH_CONSTANT.bits() => Some(Self(x)),
            x if x == Self::DRAW_INDEXED.bits() => Some(Self(x)),
            x if x == Self::DRAW.bits() => Some(Self(x)),
            x if x == Self::DRAW_TASKS.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: i32) -> Self {
        Self(bits)
    }
}
///# [vkGetGeneratedCommandsMemoryRequirementsNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetGeneratedCommandsMemoryRequirementsNV.html)
# [doc = include_str ! ("../../../../doc/extensions/nv_device_generated_commands/vkGetGeneratedCommandsMemoryRequirementsNV.md")]
#[doc(alias = "vkGetGeneratedCommandsMemoryRequirementsNV")]
pub type FNGetGeneratedCommandsMemoryRequirementsNv = unsafe extern "system" fn(
    device: Device,
    p_info: *const GeneratedCommandsMemoryRequirementsInfoNV,
    p_memory_requirements: *mut MemoryRequirements2,
);
///# [vkCreateIndirectCommandsLayoutNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateIndirectCommandsLayoutNV.html)
# [doc = include_str ! ("../../../../doc/extensions/nv_device_generated_commands/vkCreateIndirectCommandsLayoutNV.md")]
#[doc(alias = "vkCreateIndirectCommandsLayoutNV")]
pub type FNCreateIndirectCommandsLayoutNv = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const IndirectCommandsLayoutCreateInfoNV,
    p_allocator: *const AllocationCallbacks,
    p_indirect_commands_layout: *mut IndirectCommandsLayoutNV,
) -> VulkanResultCodes;
///# [vkDestroyIndirectCommandsLayoutNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyIndirectCommandsLayoutNV.html)
# [doc = include_str ! ("../../../../doc/extensions/nv_device_generated_commands/vkDestroyIndirectCommandsLayoutNV.md")]
#[doc(alias = "vkDestroyIndirectCommandsLayoutNV")]
pub type FNDestroyIndirectCommandsLayoutNv = unsafe extern "system" fn(
    device: Device,
    indirect_commands_layout: IndirectCommandsLayoutNV,
    p_allocator: *const AllocationCallbacks,
);
///# [vkCmdExecuteGeneratedCommandsNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdExecuteGeneratedCommandsNV.html)
# [doc = include_str ! ("../../../../doc/extensions/nv_device_generated_commands/vkCmdExecuteGeneratedCommandsNV.md")]
#[doc(alias = "vkCmdExecuteGeneratedCommandsNV")]
pub type FNCmdExecuteGeneratedCommandsNv = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    is_preprocessed: Bool32,
    p_generated_commands_info: *const GeneratedCommandsInfoNV,
);
///# [vkCmdPreprocessGeneratedCommandsNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdPreprocessGeneratedCommandsNV.html)
# [doc = include_str ! ("../../../../doc/extensions/nv_device_generated_commands/vkCmdPreprocessGeneratedCommandsNV.md")]
#[doc(alias = "vkCmdPreprocessGeneratedCommandsNV")]
pub type FNCmdPreprocessGeneratedCommandsNv =
    unsafe extern "system" fn(command_buffer: CommandBuffer, p_generated_commands_info: *const GeneratedCommandsInfoNV);
///# [vkCmdBindPipelineShaderGroupNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBindPipelineShaderGroupNV.html)
# [doc = include_str ! ("../../../../doc/extensions/nv_device_generated_commands/vkCmdBindPipelineShaderGroupNV.md")]
#[doc(alias = "vkCmdBindPipelineShaderGroupNV")]
pub type FNCmdBindPipelineShaderGroupNv = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    pipeline_bind_point: PipelineBindPoint,
    pipeline: Pipeline,
    group_index: u32,
);
