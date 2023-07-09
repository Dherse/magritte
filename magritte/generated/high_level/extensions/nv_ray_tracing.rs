pub use crate::common::extensions::nv_ray_tracing::{
    AccelerationStructureMemoryRequirementsTypeNV, NV_RAY_TRACING_EXTENSION_NAME, NV_RAY_TRACING_SPEC_VERSION,
    SHADER_UNUSED_NV,
};
#[cfg(feature = "VK_KHR_acceleration_structure")]
use crate::extensions::khr_acceleration_structure::AabbPositionsKHR;
#[cfg(feature = "VK_KHR_acceleration_structure")]
use crate::extensions::khr_acceleration_structure::AccelerationStructureInstanceKHR;
#[cfg(feature = "VK_KHR_acceleration_structure")]
use crate::extensions::khr_acceleration_structure::AccelerationStructureTypeKHR;
#[cfg(feature = "VK_KHR_acceleration_structure")]
use crate::extensions::khr_acceleration_structure::BuildAccelerationStructureFlagBitsKHR;
#[cfg(feature = "VK_KHR_acceleration_structure")]
use crate::extensions::khr_acceleration_structure::BuildAccelerationStructureFlagsKHR;
#[cfg(feature = "VK_KHR_acceleration_structure")]
use crate::extensions::khr_acceleration_structure::CopyAccelerationStructureModeKHR;
#[cfg(feature = "VK_KHR_acceleration_structure")]
use crate::extensions::khr_acceleration_structure::GeometryFlagBitsKHR;
#[cfg(feature = "VK_KHR_acceleration_structure")]
use crate::extensions::khr_acceleration_structure::GeometryFlagsKHR;
#[cfg(feature = "VK_KHR_acceleration_structure")]
use crate::extensions::khr_acceleration_structure::GeometryInstanceFlagBitsKHR;
#[cfg(feature = "VK_KHR_acceleration_structure")]
use crate::extensions::khr_acceleration_structure::GeometryInstanceFlagsKHR;
#[cfg(feature = "VK_KHR_acceleration_structure")]
use crate::extensions::khr_acceleration_structure::GeometryTypeKHR;
#[cfg(feature = "VK_KHR_acceleration_structure")]
use crate::extensions::khr_acceleration_structure::TransformMatrixKHR;
#[cfg(feature = "VK_KHR_ray_tracing_pipeline")]
use crate::extensions::khr_ray_tracing_pipeline::RayTracingShaderGroupTypeKHR;
use crate::{
    context::{Container, Context, Error, ObjectId},
    vulkan1_0::{
        Buffer, DeviceMemory, DeviceSize, Format, IndexType, Pipeline, PipelineCreateFlags, PipelineLayout,
        PipelineShaderStageCreateInfo, StructureType,
    },
    vulkan1_1::MemoryRequirements2,
    vulkan1_3::PipelineCreationFeedbackCreateInfo,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use std::sync::Arc;
#[doc(alias = "VkGeometryFlagsNV")]
pub type GeometryFlagsNV = GeometryFlagsKHR;
#[doc(alias = "VkGeometryInstanceFlagsNV")]
pub type GeometryInstanceFlagsNV = GeometryInstanceFlagsKHR;
#[doc(alias = "VkBuildAccelerationStructureFlagsNV")]
pub type BuildAccelerationStructureFlagsNV = BuildAccelerationStructureFlagsKHR;
#[doc(alias = "VkGeometryFlagBitsNV")]
pub type GeometryFlagBitsNV = GeometryFlagBitsKHR;
#[doc(alias = "VkGeometryInstanceFlagBitsNV")]
pub type GeometryInstanceFlagBitsNV = GeometryInstanceFlagBitsKHR;
#[doc(alias = "VkBuildAccelerationStructureFlagBitsNV")]
pub type BuildAccelerationStructureFlagBitsNV = BuildAccelerationStructureFlagBitsKHR;
#[doc(alias = "VkCopyAccelerationStructureModeNV")]
pub type CopyAccelerationStructureModeNV = CopyAccelerationStructureModeKHR;
#[doc(alias = "VkAccelerationStructureTypeNV")]
pub type AccelerationStructureTypeNV = AccelerationStructureTypeKHR;
#[doc(alias = "VkGeometryTypeNV")]
pub type GeometryTypeNV = GeometryTypeKHR;
#[doc(alias = "VkRayTracingShaderGroupTypeNV")]
pub type RayTracingShaderGroupTypeNV = RayTracingShaderGroupTypeKHR;
#[doc(alias = "VkMemoryRequirements2KHR")]
pub type MemoryRequirements2KHR = MemoryRequirements2;
#[doc(alias = "VkAabbPositionsNV")]
pub type AabbPositionsNV = AabbPositionsKHR;
#[doc(alias = "VkTransformMatrixNV")]
pub type TransformMatrixNV = TransformMatrixKHR;
#[doc(alias = "VkAccelerationStructureInstanceNV")]
pub type AccelerationStructureInstanceNV = AccelerationStructureInstanceKHR;
#[doc(alias = "VkRayTracingShaderGroupCreateInfoNV")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RayTracingShaderGroupCreateInfoNV {
    #[doc(alias = "type")]
    pub type_: RayTracingShaderGroupTypeKHR,
    #[doc(alias = "generalShader")]
    pub general_shader: u32,
    #[doc(alias = "closestHitShader")]
    pub closest_hit_shader: u32,
    #[doc(alias = "anyHitShader")]
    pub any_hit_shader: u32,
    #[doc(alias = "intersectionShader")]
    pub intersection_shader: u32,
}
impl RayTracingShaderGroupCreateInfoNV {
    ///Get a reference to the `type_` field.
    pub fn type_(&self) -> RayTracingShaderGroupTypeKHR {
        self.type_
    }
    ///Get a reference to the `general_shader` field.
    pub fn general_shader(&self) -> u32 {
        self.general_shader
    }
    ///Get a reference to the `closest_hit_shader` field.
    pub fn closest_hit_shader(&self) -> u32 {
        self.closest_hit_shader
    }
    ///Get a reference to the `any_hit_shader` field.
    pub fn any_hit_shader(&self) -> u32 {
        self.any_hit_shader
    }
    ///Get a reference to the `intersection_shader` field.
    pub fn intersection_shader(&self) -> u32 {
        self.intersection_shader
    }
    ///Get a mutable reference to the `type_` field.
    pub fn type__mut(&mut self) -> &mut RayTracingShaderGroupTypeKHR {
        &mut self.type_
    }
    ///Get a mutable reference to the `general_shader` field.
    pub fn general_shader_mut(&mut self) -> &mut u32 {
        &mut self.general_shader
    }
    ///Get a mutable reference to the `closest_hit_shader` field.
    pub fn closest_hit_shader_mut(&mut self) -> &mut u32 {
        &mut self.closest_hit_shader
    }
    ///Get a mutable reference to the `any_hit_shader` field.
    pub fn any_hit_shader_mut(&mut self) -> &mut u32 {
        &mut self.any_hit_shader
    }
    ///Get a mutable reference to the `intersection_shader` field.
    pub fn intersection_shader_mut(&mut self) -> &mut u32 {
        &mut self.intersection_shader
    }
    ///Sets the `type_` field.
    pub fn set_type_(&mut self, type_: RayTracingShaderGroupTypeKHR) -> &mut Self {
        self.type_ = type_;
        self
    }
    ///Sets the `general_shader` field.
    pub fn set_general_shader(&mut self, general_shader: u32) -> &mut Self {
        self.general_shader = general_shader;
        self
    }
    ///Sets the `closest_hit_shader` field.
    pub fn set_closest_hit_shader(&mut self, closest_hit_shader: u32) -> &mut Self {
        self.closest_hit_shader = closest_hit_shader;
        self
    }
    ///Sets the `any_hit_shader` field.
    pub fn set_any_hit_shader(&mut self, any_hit_shader: u32) -> &mut Self {
        self.any_hit_shader = any_hit_shader;
        self
    }
    ///Sets the `intersection_shader` field.
    pub fn set_intersection_shader(&mut self, intersection_shader: u32) -> &mut Self {
        self.intersection_shader = intersection_shader;
        self
    }
    ///Sets the `type_` field in a builder way.
    pub fn with_type_(mut self, type_: RayTracingShaderGroupTypeKHR) -> Self {
        self.type_ = type_;
        self
    }
    ///Sets the `general_shader` field in a builder way.
    pub fn with_general_shader(mut self, general_shader: u32) -> Self {
        self.general_shader = general_shader;
        self
    }
    ///Sets the `closest_hit_shader` field in a builder way.
    pub fn with_closest_hit_shader(mut self, closest_hit_shader: u32) -> Self {
        self.closest_hit_shader = closest_hit_shader;
        self
    }
    ///Sets the `any_hit_shader` field in a builder way.
    pub fn with_any_hit_shader(mut self, any_hit_shader: u32) -> Self {
        self.any_hit_shader = any_hit_shader;
        self
    }
    ///Sets the `intersection_shader` field in a builder way.
    pub fn with_intersection_shader(mut self, intersection_shader: u32) -> Self {
        self.intersection_shader = intersection_shader;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for RayTracingShaderGroupCreateInfoNV {
    type LowLevel = crate::native::extensions::nv_ray_tracing::RayTracingShaderGroupCreateInfoNV;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::nv_ray_tracing::RayTracingShaderGroupCreateInfoNV {
            s_type: StructureType::RayTracingShaderGroupCreateInfoNv,
            p_next: std::ptr::null(),
            type_: self.type_.into_low_level(context, bump),
            general_shader: self.general_shader.into_low_level(context, bump),
            closest_hit_shader: self.closest_hit_shader.into_low_level(context, bump),
            any_hit_shader: self.any_hit_shader.into_low_level(context, bump),
            intersection_shader: self.intersection_shader.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for RayTracingShaderGroupCreateInfoNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            type_: crate::conv::FromLowLevel::from_low_level(context, value.type_),
            general_shader: crate::conv::FromLowLevel::from_low_level(context, value.general_shader),
            closest_hit_shader: crate::conv::FromLowLevel::from_low_level(context, value.closest_hit_shader),
            any_hit_shader: crate::conv::FromLowLevel::from_low_level(context, value.any_hit_shader),
            intersection_shader: crate::conv::FromLowLevel::from_low_level(context, value.intersection_shader),
        }
    }
}
#[doc(alias = "VkRayTracingPipelineCreateInfoNV")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RayTracingPipelineCreateInfoNV {
    #[doc(alias = "pNext")]
    pub extensions: SmallVec<[RayTracingPipelineCreateInfoNVExtension; 1]>,
    pub flags: PipelineCreateFlags,
    #[doc(alias = "pStages")]
    pub stages: SmallVec<[PipelineShaderStageCreateInfo; 8]>,
    #[doc(alias = "pGroups")]
    pub groups: SmallVec<[RayTracingShaderGroupCreateInfoNV; 8]>,
    #[doc(alias = "maxRecursionDepth")]
    pub max_recursion_depth: u32,
    pub layout: PipelineLayout,
    #[doc(alias = "basePipelineHandle")]
    pub base_pipeline_handle: Option<Pipeline>,
    #[doc(alias = "basePipelineIndex")]
    pub base_pipeline_index: i32,
}
impl RayTracingPipelineCreateInfoNV {
    ///Adds an extension to the struct
    pub fn with_extension(mut self, ext: impl Into<RayTracingPipelineCreateInfoNVExtension>) -> Self {
        self.extensions.push(ext.into());
        self
    }
    ///Get a reference to the `extensions` field.
    pub fn extensions(&self) -> &SmallVec<[RayTracingPipelineCreateInfoNVExtension; 1]> {
        &self.extensions
    }
    ///Get a reference to the `flags` field.
    pub fn flags(&self) -> PipelineCreateFlags {
        self.flags
    }
    ///Get a reference to the `stages` field.
    pub fn stages(&self) -> &SmallVec<[PipelineShaderStageCreateInfo; 8]> {
        &self.stages
    }
    ///Get a reference to the `groups` field.
    pub fn groups(&self) -> &SmallVec<[RayTracingShaderGroupCreateInfoNV; 8]> {
        &self.groups
    }
    ///Get a reference to the `max_recursion_depth` field.
    pub fn max_recursion_depth(&self) -> u32 {
        self.max_recursion_depth
    }
    ///Get a reference to the `layout` field.
    pub fn layout(&self) -> &PipelineLayout {
        &self.layout
    }
    ///Get a reference to the `base_pipeline_handle` field.
    pub fn base_pipeline_handle(&self) -> &Option<Pipeline> {
        &self.base_pipeline_handle
    }
    ///Get a reference to the `base_pipeline_index` field.
    pub fn base_pipeline_index(&self) -> i32 {
        self.base_pipeline_index
    }
    ///Get a mutable reference to the `extensions` field.
    pub fn extensions_mut(&mut self) -> &mut SmallVec<[RayTracingPipelineCreateInfoNVExtension; 1]> {
        &mut self.extensions
    }
    ///Get a mutable reference to the `flags` field.
    pub fn flags_mut(&mut self) -> &mut PipelineCreateFlags {
        &mut self.flags
    }
    ///Get a mutable reference to the `stages` field.
    pub fn stages_mut(&mut self) -> &mut SmallVec<[PipelineShaderStageCreateInfo; 8]> {
        &mut self.stages
    }
    ///Get a mutable reference to the `groups` field.
    pub fn groups_mut(&mut self) -> &mut SmallVec<[RayTracingShaderGroupCreateInfoNV; 8]> {
        &mut self.groups
    }
    ///Get a mutable reference to the `max_recursion_depth` field.
    pub fn max_recursion_depth_mut(&mut self) -> &mut u32 {
        &mut self.max_recursion_depth
    }
    ///Get a mutable reference to the `layout` field.
    pub fn layout_mut(&mut self) -> &mut PipelineLayout {
        &mut self.layout
    }
    ///Get a mutable reference to the `base_pipeline_handle` field.
    pub fn base_pipeline_handle_mut(&mut self) -> &mut Option<Pipeline> {
        &mut self.base_pipeline_handle
    }
    ///Get a mutable reference to the `base_pipeline_index` field.
    pub fn base_pipeline_index_mut(&mut self) -> &mut i32 {
        &mut self.base_pipeline_index
    }
    ///Sets the `extensions` field.
    pub fn set_extensions(&mut self, extensions: SmallVec<[RayTracingPipelineCreateInfoNVExtension; 1]>) -> &mut Self {
        self.extensions = extensions;
        self
    }
    ///Sets the `flags` field.
    pub fn set_flags(&mut self, flags: PipelineCreateFlags) -> &mut Self {
        self.flags = flags;
        self
    }
    ///Sets the `stages` field.
    pub fn set_stages(&mut self, stages: SmallVec<[PipelineShaderStageCreateInfo; 8]>) -> &mut Self {
        self.stages = stages;
        self
    }
    ///Sets the `groups` field.
    pub fn set_groups(&mut self, groups: SmallVec<[RayTracingShaderGroupCreateInfoNV; 8]>) -> &mut Self {
        self.groups = groups;
        self
    }
    ///Sets the `max_recursion_depth` field.
    pub fn set_max_recursion_depth(&mut self, max_recursion_depth: u32) -> &mut Self {
        self.max_recursion_depth = max_recursion_depth;
        self
    }
    ///Sets the `layout` field.
    pub fn set_layout(&mut self, layout: PipelineLayout) -> &mut Self {
        self.layout = layout;
        self
    }
    ///Sets the `base_pipeline_handle` field.
    pub fn set_base_pipeline_handle(&mut self, base_pipeline_handle: Option<Pipeline>) -> &mut Self {
        self.base_pipeline_handle = base_pipeline_handle;
        self
    }
    ///Sets the `base_pipeline_index` field.
    pub fn set_base_pipeline_index(&mut self, base_pipeline_index: i32) -> &mut Self {
        self.base_pipeline_index = base_pipeline_index;
        self
    }
    ///Sets the `extensions` field in a builder way.
    pub fn with_extensions(mut self, extensions: SmallVec<[RayTracingPipelineCreateInfoNVExtension; 1]>) -> Self {
        self.extensions = extensions;
        self
    }
    ///Sets the `flags` field in a builder way.
    pub fn with_flags(mut self, flags: PipelineCreateFlags) -> Self {
        self.flags = flags;
        self
    }
    ///Sets the `stages` field in a builder way.
    pub fn with_stages(mut self, stages: SmallVec<[PipelineShaderStageCreateInfo; 8]>) -> Self {
        self.stages = stages;
        self
    }
    ///Sets the `groups` field in a builder way.
    pub fn with_groups(mut self, groups: SmallVec<[RayTracingShaderGroupCreateInfoNV; 8]>) -> Self {
        self.groups = groups;
        self
    }
    ///Sets the `max_recursion_depth` field in a builder way.
    pub fn with_max_recursion_depth(mut self, max_recursion_depth: u32) -> Self {
        self.max_recursion_depth = max_recursion_depth;
        self
    }
    ///Sets the `layout` field in a builder way.
    pub fn with_layout(mut self, layout: PipelineLayout) -> Self {
        self.layout = layout;
        self
    }
    ///Sets the `base_pipeline_handle` field in a builder way.
    pub fn with_base_pipeline_handle(mut self, base_pipeline_handle: Option<Pipeline>) -> Self {
        self.base_pipeline_handle = base_pipeline_handle;
        self
    }
    ///Sets the `base_pipeline_index` field in a builder way.
    pub fn with_base_pipeline_index(mut self, base_pipeline_index: i32) -> Self {
        self.base_pipeline_index = base_pipeline_index;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for RayTracingPipelineCreateInfoNV {
    type LowLevel = crate::native::extensions::nv_ray_tracing::RayTracingPipelineCreateInfoNV;
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
        let len_stages = self.stages.len() as u32;
        let stages = bump
            .alloc_slice_fill_iter(self.stages.iter().map(|x| x.into_low_level(context, bump)))
            .as_ptr()
            .cast();
        let len_groups = self.groups.len() as u32;
        let groups = bump
            .alloc_slice_fill_iter(self.groups.iter().map(|x| x.into_low_level(context, bump)))
            .as_ptr()
            .cast();
        crate::native::extensions::nv_ray_tracing::RayTracingPipelineCreateInfoNV {
            s_type: StructureType::RayTracingPipelineCreateInfoNv,
            p_next: next,
            flags: self.flags.into_low_level(context, bump),
            stage_count: len_stages,
            stages: stages,
            group_count: len_groups,
            groups: groups,
            max_recursion_depth: self.max_recursion_depth.into_low_level(context, bump),
            layout: self.layout.into_low_level(context, bump),
            base_pipeline_handle: self
                .base_pipeline_handle
                .as_ref()
                .map(|v| v.into_low_level(context, bump))
                .unwrap_or_else(Default::default),
            base_pipeline_index: self.base_pipeline_index.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for RayTracingPipelineCreateInfoNV {
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
        let stages_len = value.stage_count;
        let mut stages = SmallVec::with_capacity(stages_len as usize);
        for i in 0..stages_len {
            stages.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.stages.add(i as usize).read(),
            ));
        }
        let groups_len = value.group_count;
        let mut groups = SmallVec::with_capacity(groups_len as usize);
        for i in 0..groups_len {
            groups.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.groups.add(i as usize).read(),
            ));
        }
        Self {
            extensions: extensions,
            flags: crate::conv::FromLowLevel::from_low_level(context, value.flags),
            stages: stages,
            groups: groups,
            max_recursion_depth: crate::conv::FromLowLevel::from_low_level(context, value.max_recursion_depth),
            layout: crate::conv::FromLowLevel::from_low_level(context, value.layout),
            base_pipeline_handle: if value.base_pipeline_handle == crate::native::vulkan1_0::Pipeline::null() {
                None
            } else {
                Some(crate::conv::FromLowLevel::from_low_level(
                    context,
                    value.base_pipeline_handle,
                ))
            },
            base_pipeline_index: crate::conv::FromLowLevel::from_low_level(context, value.base_pipeline_index),
        }
    }
}
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
///Extensions for [`RayTracingPipelineCreateInfoNV`]
pub enum RayTracingPipelineCreateInfoNVExtension {
    ///Contains a type [`PipelineCreationFeedbackCreateInfo`] for extending
    /// [`RayTracingPipelineCreateInfoNV`]
    PipelineCreationFeedbackCreateInfo(PipelineCreationFeedbackCreateInfo),
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for RayTracingPipelineCreateInfoNVExtension {
    type LowLevel = *const crate::native::vulkan1_0::BaseInStructure;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        match self {
            Self::PipelineCreationFeedbackCreateInfo(ext) => (bump.alloc(ext.into_low_level(context, bump))
                as *mut crate::native::vulkan1_3::PipelineCreationFeedbackCreateInfo)
                .cast(),
            other => unreachable!("unexpected variant {:?}", other),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for RayTracingPipelineCreateInfoNVExtension {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        assert!(!value.is_null());
        match (*value).s_type {
            crate::native::vulkan1_0::StructureType::PipelineCreationFeedbackCreateInfo => {
                Self::PipelineCreationFeedbackCreateInfo(PipelineCreationFeedbackCreateInfo::from_low_level(
                    context,
                    std::ptr::read(value.cast::<crate::native::vulkan1_3::PipelineCreationFeedbackCreateInfo>()),
                ))
            },
            other => panic!(
                "Structure type {:?} is not a member of {}",
                other,
                stringify!(RayTracingPipelineCreateInfoNV)
            ),
        }
    }
}
impl From<PipelineCreationFeedbackCreateInfo> for RayTracingPipelineCreateInfoNVExtension {
    fn from(ext: PipelineCreationFeedbackCreateInfo) -> Self {
        Self::PipelineCreationFeedbackCreateInfo(ext)
    }
}
impl TryInto<PipelineCreationFeedbackCreateInfo> for RayTracingPipelineCreateInfoNVExtension {
    type Error = RayTracingPipelineCreateInfoNVExtension;
    fn try_into(self) -> Result<PipelineCreationFeedbackCreateInfo, Self::Error> {
        match self {
            Self::PipelineCreationFeedbackCreateInfo(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[doc(alias = "VkGeometryTrianglesNV")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GeometryTrianglesNV {
    #[doc(alias = "vertexData")]
    pub vertex_data: Option<Buffer>,
    #[doc(alias = "vertexOffset")]
    pub vertex_offset: DeviceSize,
    #[doc(alias = "vertexCount")]
    pub vertex_count: u32,
    #[doc(alias = "vertexStride")]
    pub vertex_stride: DeviceSize,
    #[doc(alias = "vertexFormat")]
    pub vertex_format: Format,
    #[doc(alias = "indexData")]
    pub index_data: Option<Buffer>,
    #[doc(alias = "indexOffset")]
    pub index_offset: DeviceSize,
    #[doc(alias = "indexCount")]
    pub index_count: u32,
    #[doc(alias = "indexType")]
    pub index_type: IndexType,
    #[doc(alias = "transformData")]
    pub transform_data: Option<Buffer>,
    #[doc(alias = "transformOffset")]
    pub transform_offset: DeviceSize,
}
impl GeometryTrianglesNV {
    ///Get a reference to the `vertex_data` field.
    pub fn vertex_data(&self) -> &Option<Buffer> {
        &self.vertex_data
    }
    ///Get a reference to the `vertex_offset` field.
    pub fn vertex_offset(&self) -> &DeviceSize {
        &self.vertex_offset
    }
    ///Get a reference to the `vertex_count` field.
    pub fn vertex_count(&self) -> u32 {
        self.vertex_count
    }
    ///Get a reference to the `vertex_stride` field.
    pub fn vertex_stride(&self) -> &DeviceSize {
        &self.vertex_stride
    }
    ///Get a reference to the `vertex_format` field.
    pub fn vertex_format(&self) -> Format {
        self.vertex_format
    }
    ///Get a reference to the `index_data` field.
    pub fn index_data(&self) -> &Option<Buffer> {
        &self.index_data
    }
    ///Get a reference to the `index_offset` field.
    pub fn index_offset(&self) -> &DeviceSize {
        &self.index_offset
    }
    ///Get a reference to the `index_count` field.
    pub fn index_count(&self) -> u32 {
        self.index_count
    }
    ///Get a reference to the `index_type` field.
    pub fn index_type(&self) -> IndexType {
        self.index_type
    }
    ///Get a reference to the `transform_data` field.
    pub fn transform_data(&self) -> &Option<Buffer> {
        &self.transform_data
    }
    ///Get a reference to the `transform_offset` field.
    pub fn transform_offset(&self) -> &DeviceSize {
        &self.transform_offset
    }
    ///Get a mutable reference to the `vertex_data` field.
    pub fn vertex_data_mut(&mut self) -> &mut Option<Buffer> {
        &mut self.vertex_data
    }
    ///Get a mutable reference to the `vertex_offset` field.
    pub fn vertex_offset_mut(&mut self) -> &mut DeviceSize {
        &mut self.vertex_offset
    }
    ///Get a mutable reference to the `vertex_count` field.
    pub fn vertex_count_mut(&mut self) -> &mut u32 {
        &mut self.vertex_count
    }
    ///Get a mutable reference to the `vertex_stride` field.
    pub fn vertex_stride_mut(&mut self) -> &mut DeviceSize {
        &mut self.vertex_stride
    }
    ///Get a mutable reference to the `vertex_format` field.
    pub fn vertex_format_mut(&mut self) -> &mut Format {
        &mut self.vertex_format
    }
    ///Get a mutable reference to the `index_data` field.
    pub fn index_data_mut(&mut self) -> &mut Option<Buffer> {
        &mut self.index_data
    }
    ///Get a mutable reference to the `index_offset` field.
    pub fn index_offset_mut(&mut self) -> &mut DeviceSize {
        &mut self.index_offset
    }
    ///Get a mutable reference to the `index_count` field.
    pub fn index_count_mut(&mut self) -> &mut u32 {
        &mut self.index_count
    }
    ///Get a mutable reference to the `index_type` field.
    pub fn index_type_mut(&mut self) -> &mut IndexType {
        &mut self.index_type
    }
    ///Get a mutable reference to the `transform_data` field.
    pub fn transform_data_mut(&mut self) -> &mut Option<Buffer> {
        &mut self.transform_data
    }
    ///Get a mutable reference to the `transform_offset` field.
    pub fn transform_offset_mut(&mut self) -> &mut DeviceSize {
        &mut self.transform_offset
    }
    ///Sets the `vertex_data` field.
    pub fn set_vertex_data(&mut self, vertex_data: Option<Buffer>) -> &mut Self {
        self.vertex_data = vertex_data;
        self
    }
    ///Sets the `vertex_offset` field.
    pub fn set_vertex_offset(&mut self, vertex_offset: DeviceSize) -> &mut Self {
        self.vertex_offset = vertex_offset;
        self
    }
    ///Sets the `vertex_count` field.
    pub fn set_vertex_count(&mut self, vertex_count: u32) -> &mut Self {
        self.vertex_count = vertex_count;
        self
    }
    ///Sets the `vertex_stride` field.
    pub fn set_vertex_stride(&mut self, vertex_stride: DeviceSize) -> &mut Self {
        self.vertex_stride = vertex_stride;
        self
    }
    ///Sets the `vertex_format` field.
    pub fn set_vertex_format(&mut self, vertex_format: Format) -> &mut Self {
        self.vertex_format = vertex_format;
        self
    }
    ///Sets the `index_data` field.
    pub fn set_index_data(&mut self, index_data: Option<Buffer>) -> &mut Self {
        self.index_data = index_data;
        self
    }
    ///Sets the `index_offset` field.
    pub fn set_index_offset(&mut self, index_offset: DeviceSize) -> &mut Self {
        self.index_offset = index_offset;
        self
    }
    ///Sets the `index_count` field.
    pub fn set_index_count(&mut self, index_count: u32) -> &mut Self {
        self.index_count = index_count;
        self
    }
    ///Sets the `index_type` field.
    pub fn set_index_type(&mut self, index_type: IndexType) -> &mut Self {
        self.index_type = index_type;
        self
    }
    ///Sets the `transform_data` field.
    pub fn set_transform_data(&mut self, transform_data: Option<Buffer>) -> &mut Self {
        self.transform_data = transform_data;
        self
    }
    ///Sets the `transform_offset` field.
    pub fn set_transform_offset(&mut self, transform_offset: DeviceSize) -> &mut Self {
        self.transform_offset = transform_offset;
        self
    }
    ///Sets the `vertex_data` field in a builder way.
    pub fn with_vertex_data(mut self, vertex_data: Option<Buffer>) -> Self {
        self.vertex_data = vertex_data;
        self
    }
    ///Sets the `vertex_offset` field in a builder way.
    pub fn with_vertex_offset(mut self, vertex_offset: DeviceSize) -> Self {
        self.vertex_offset = vertex_offset;
        self
    }
    ///Sets the `vertex_count` field in a builder way.
    pub fn with_vertex_count(mut self, vertex_count: u32) -> Self {
        self.vertex_count = vertex_count;
        self
    }
    ///Sets the `vertex_stride` field in a builder way.
    pub fn with_vertex_stride(mut self, vertex_stride: DeviceSize) -> Self {
        self.vertex_stride = vertex_stride;
        self
    }
    ///Sets the `vertex_format` field in a builder way.
    pub fn with_vertex_format(mut self, vertex_format: Format) -> Self {
        self.vertex_format = vertex_format;
        self
    }
    ///Sets the `index_data` field in a builder way.
    pub fn with_index_data(mut self, index_data: Option<Buffer>) -> Self {
        self.index_data = index_data;
        self
    }
    ///Sets the `index_offset` field in a builder way.
    pub fn with_index_offset(mut self, index_offset: DeviceSize) -> Self {
        self.index_offset = index_offset;
        self
    }
    ///Sets the `index_count` field in a builder way.
    pub fn with_index_count(mut self, index_count: u32) -> Self {
        self.index_count = index_count;
        self
    }
    ///Sets the `index_type` field in a builder way.
    pub fn with_index_type(mut self, index_type: IndexType) -> Self {
        self.index_type = index_type;
        self
    }
    ///Sets the `transform_data` field in a builder way.
    pub fn with_transform_data(mut self, transform_data: Option<Buffer>) -> Self {
        self.transform_data = transform_data;
        self
    }
    ///Sets the `transform_offset` field in a builder way.
    pub fn with_transform_offset(mut self, transform_offset: DeviceSize) -> Self {
        self.transform_offset = transform_offset;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for GeometryTrianglesNV {
    type LowLevel = crate::native::extensions::nv_ray_tracing::GeometryTrianglesNV;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::nv_ray_tracing::GeometryTrianglesNV {
            s_type: StructureType::GeometryTrianglesNv,
            p_next: std::ptr::null(),
            vertex_data: self
                .vertex_data
                .as_ref()
                .map(|v| v.into_low_level(context, bump))
                .unwrap_or_else(Default::default),
            vertex_offset: self.vertex_offset.into_low_level(context, bump),
            vertex_count: self.vertex_count.into_low_level(context, bump),
            vertex_stride: self.vertex_stride.into_low_level(context, bump),
            vertex_format: self.vertex_format.into_low_level(context, bump),
            index_data: self
                .index_data
                .as_ref()
                .map(|v| v.into_low_level(context, bump))
                .unwrap_or_else(Default::default),
            index_offset: self.index_offset.into_low_level(context, bump),
            index_count: self.index_count.into_low_level(context, bump),
            index_type: self.index_type.into_low_level(context, bump),
            transform_data: self
                .transform_data
                .as_ref()
                .map(|v| v.into_low_level(context, bump))
                .unwrap_or_else(Default::default),
            transform_offset: self.transform_offset.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for GeometryTrianglesNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            vertex_data: if value.vertex_data == crate::native::vulkan1_0::Buffer::null() {
                None
            } else {
                Some(crate::conv::FromLowLevel::from_low_level(context, value.vertex_data))
            },
            vertex_offset: crate::conv::FromLowLevel::from_low_level(context, value.vertex_offset),
            vertex_count: crate::conv::FromLowLevel::from_low_level(context, value.vertex_count),
            vertex_stride: crate::conv::FromLowLevel::from_low_level(context, value.vertex_stride),
            vertex_format: crate::conv::FromLowLevel::from_low_level(context, value.vertex_format),
            index_data: if value.index_data == crate::native::vulkan1_0::Buffer::null() {
                None
            } else {
                Some(crate::conv::FromLowLevel::from_low_level(context, value.index_data))
            },
            index_offset: crate::conv::FromLowLevel::from_low_level(context, value.index_offset),
            index_count: crate::conv::FromLowLevel::from_low_level(context, value.index_count),
            index_type: crate::conv::FromLowLevel::from_low_level(context, value.index_type),
            transform_data: if value.transform_data == crate::native::vulkan1_0::Buffer::null() {
                None
            } else {
                Some(crate::conv::FromLowLevel::from_low_level(context, value.transform_data))
            },
            transform_offset: crate::conv::FromLowLevel::from_low_level(context, value.transform_offset),
        }
    }
}
#[doc(alias = "VkGeometryAABBNV")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GeometryAabbNV {
    #[doc(alias = "aabbData")]
    pub aabb_data: Option<Buffer>,
    #[doc(alias = "numAABBs")]
    pub num_aab_bs: u32,
    pub stride: u32,
    pub offset: DeviceSize,
}
impl GeometryAabbNV {
    ///Get a reference to the `aabb_data` field.
    pub fn aabb_data(&self) -> &Option<Buffer> {
        &self.aabb_data
    }
    ///Get a reference to the `num_aab_bs` field.
    pub fn num_aab_bs(&self) -> u32 {
        self.num_aab_bs
    }
    ///Get a reference to the `stride` field.
    pub fn stride(&self) -> u32 {
        self.stride
    }
    ///Get a reference to the `offset` field.
    pub fn offset(&self) -> &DeviceSize {
        &self.offset
    }
    ///Get a mutable reference to the `aabb_data` field.
    pub fn aabb_data_mut(&mut self) -> &mut Option<Buffer> {
        &mut self.aabb_data
    }
    ///Get a mutable reference to the `num_aab_bs` field.
    pub fn num_aab_bs_mut(&mut self) -> &mut u32 {
        &mut self.num_aab_bs
    }
    ///Get a mutable reference to the `stride` field.
    pub fn stride_mut(&mut self) -> &mut u32 {
        &mut self.stride
    }
    ///Get a mutable reference to the `offset` field.
    pub fn offset_mut(&mut self) -> &mut DeviceSize {
        &mut self.offset
    }
    ///Sets the `aabb_data` field.
    pub fn set_aabb_data(&mut self, aabb_data: Option<Buffer>) -> &mut Self {
        self.aabb_data = aabb_data;
        self
    }
    ///Sets the `num_aab_bs` field.
    pub fn set_num_aab_bs(&mut self, num_aab_bs: u32) -> &mut Self {
        self.num_aab_bs = num_aab_bs;
        self
    }
    ///Sets the `stride` field.
    pub fn set_stride(&mut self, stride: u32) -> &mut Self {
        self.stride = stride;
        self
    }
    ///Sets the `offset` field.
    pub fn set_offset(&mut self, offset: DeviceSize) -> &mut Self {
        self.offset = offset;
        self
    }
    ///Sets the `aabb_data` field in a builder way.
    pub fn with_aabb_data(mut self, aabb_data: Option<Buffer>) -> Self {
        self.aabb_data = aabb_data;
        self
    }
    ///Sets the `num_aab_bs` field in a builder way.
    pub fn with_num_aab_bs(mut self, num_aab_bs: u32) -> Self {
        self.num_aab_bs = num_aab_bs;
        self
    }
    ///Sets the `stride` field in a builder way.
    pub fn with_stride(mut self, stride: u32) -> Self {
        self.stride = stride;
        self
    }
    ///Sets the `offset` field in a builder way.
    pub fn with_offset(mut self, offset: DeviceSize) -> Self {
        self.offset = offset;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for GeometryAabbNV {
    type LowLevel = crate::native::extensions::nv_ray_tracing::GeometryAabbNV;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::nv_ray_tracing::GeometryAabbNV {
            s_type: StructureType::GeometryAabbNv,
            p_next: std::ptr::null(),
            aabb_data: self
                .aabb_data
                .as_ref()
                .map(|v| v.into_low_level(context, bump))
                .unwrap_or_else(Default::default),
            num_aab_bs: self.num_aab_bs.into_low_level(context, bump),
            stride: self.stride.into_low_level(context, bump),
            offset: self.offset.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for GeometryAabbNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            aabb_data: if value.aabb_data == crate::native::vulkan1_0::Buffer::null() {
                None
            } else {
                Some(crate::conv::FromLowLevel::from_low_level(context, value.aabb_data))
            },
            num_aab_bs: crate::conv::FromLowLevel::from_low_level(context, value.num_aab_bs),
            stride: crate::conv::FromLowLevel::from_low_level(context, value.stride),
            offset: crate::conv::FromLowLevel::from_low_level(context, value.offset),
        }
    }
}
#[doc(alias = "VkGeometryDataNV")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GeometryDataNV {
    pub triangles: GeometryTrianglesNV,
    pub aabbs: GeometryAabbNV,
}
impl GeometryDataNV {
    ///Get a reference to the `triangles` field.
    pub fn triangles(&self) -> &GeometryTrianglesNV {
        &self.triangles
    }
    ///Get a reference to the `aabbs` field.
    pub fn aabbs(&self) -> &GeometryAabbNV {
        &self.aabbs
    }
    ///Get a mutable reference to the `triangles` field.
    pub fn triangles_mut(&mut self) -> &mut GeometryTrianglesNV {
        &mut self.triangles
    }
    ///Get a mutable reference to the `aabbs` field.
    pub fn aabbs_mut(&mut self) -> &mut GeometryAabbNV {
        &mut self.aabbs
    }
    ///Sets the `triangles` field.
    pub fn set_triangles(&mut self, triangles: GeometryTrianglesNV) -> &mut Self {
        self.triangles = triangles;
        self
    }
    ///Sets the `aabbs` field.
    pub fn set_aabbs(&mut self, aabbs: GeometryAabbNV) -> &mut Self {
        self.aabbs = aabbs;
        self
    }
    ///Sets the `triangles` field in a builder way.
    pub fn with_triangles(mut self, triangles: GeometryTrianglesNV) -> Self {
        self.triangles = triangles;
        self
    }
    ///Sets the `aabbs` field in a builder way.
    pub fn with_aabbs(mut self, aabbs: GeometryAabbNV) -> Self {
        self.aabbs = aabbs;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for GeometryDataNV {
    type LowLevel = crate::native::extensions::nv_ray_tracing::GeometryDataNV;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::nv_ray_tracing::GeometryDataNV {
            triangles: self.triangles.into_low_level(context, bump),
            aabbs: self.aabbs.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for GeometryDataNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            triangles: crate::conv::FromLowLevel::from_low_level(context, value.triangles),
            aabbs: crate::conv::FromLowLevel::from_low_level(context, value.aabbs),
        }
    }
}
#[doc(alias = "VkGeometryNV")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GeometryNV {
    #[doc(alias = "geometryType")]
    pub geometry_type: GeometryTypeKHR,
    pub geometry: GeometryDataNV,
    pub flags: GeometryFlagsKHR,
}
impl GeometryNV {
    ///Get a reference to the `geometry_type` field.
    pub fn geometry_type(&self) -> GeometryTypeKHR {
        self.geometry_type
    }
    ///Get a reference to the `geometry` field.
    pub fn geometry(&self) -> &GeometryDataNV {
        &self.geometry
    }
    ///Get a reference to the `flags` field.
    pub fn flags(&self) -> GeometryFlagsKHR {
        self.flags
    }
    ///Get a mutable reference to the `geometry_type` field.
    pub fn geometry_type_mut(&mut self) -> &mut GeometryTypeKHR {
        &mut self.geometry_type
    }
    ///Get a mutable reference to the `geometry` field.
    pub fn geometry_mut(&mut self) -> &mut GeometryDataNV {
        &mut self.geometry
    }
    ///Get a mutable reference to the `flags` field.
    pub fn flags_mut(&mut self) -> &mut GeometryFlagsKHR {
        &mut self.flags
    }
    ///Sets the `geometry_type` field.
    pub fn set_geometry_type(&mut self, geometry_type: GeometryTypeKHR) -> &mut Self {
        self.geometry_type = geometry_type;
        self
    }
    ///Sets the `geometry` field.
    pub fn set_geometry(&mut self, geometry: GeometryDataNV) -> &mut Self {
        self.geometry = geometry;
        self
    }
    ///Sets the `flags` field.
    pub fn set_flags(&mut self, flags: GeometryFlagsKHR) -> &mut Self {
        self.flags = flags;
        self
    }
    ///Sets the `geometry_type` field in a builder way.
    pub fn with_geometry_type(mut self, geometry_type: GeometryTypeKHR) -> Self {
        self.geometry_type = geometry_type;
        self
    }
    ///Sets the `geometry` field in a builder way.
    pub fn with_geometry(mut self, geometry: GeometryDataNV) -> Self {
        self.geometry = geometry;
        self
    }
    ///Sets the `flags` field in a builder way.
    pub fn with_flags(mut self, flags: GeometryFlagsKHR) -> Self {
        self.flags = flags;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for GeometryNV {
    type LowLevel = crate::native::extensions::nv_ray_tracing::GeometryNV;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::nv_ray_tracing::GeometryNV {
            s_type: StructureType::GeometryNv,
            p_next: std::ptr::null(),
            geometry_type: self.geometry_type.into_low_level(context, bump),
            geometry: self.geometry.into_low_level(context, bump),
            flags: self.flags.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for GeometryNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            geometry_type: crate::conv::FromLowLevel::from_low_level(context, value.geometry_type),
            geometry: crate::conv::FromLowLevel::from_low_level(context, value.geometry),
            flags: crate::conv::FromLowLevel::from_low_level(context, value.flags),
        }
    }
}
#[doc(alias = "VkAccelerationStructureInfoNV")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AccelerationStructureInfoNV {
    #[doc(alias = "type")]
    pub type_: AccelerationStructureTypeNV,
    pub flags: BuildAccelerationStructureFlagsNV,
    #[doc(alias = "instanceCount")]
    pub instance_count: u32,
    #[doc(alias = "pGeometries")]
    pub geometries: SmallVec<[GeometryNV; 8]>,
}
impl AccelerationStructureInfoNV {
    ///Get a reference to the `type_` field.
    pub fn type_(&self) -> AccelerationStructureTypeNV {
        self.type_
    }
    ///Get a reference to the `flags` field.
    pub fn flags(&self) -> BuildAccelerationStructureFlagsNV {
        self.flags
    }
    ///Get a reference to the `instance_count` field.
    pub fn instance_count(&self) -> u32 {
        self.instance_count
    }
    ///Get a reference to the `geometries` field.
    pub fn geometries(&self) -> &SmallVec<[GeometryNV; 8]> {
        &self.geometries
    }
    ///Get a mutable reference to the `type_` field.
    pub fn type__mut(&mut self) -> &mut AccelerationStructureTypeNV {
        &mut self.type_
    }
    ///Get a mutable reference to the `flags` field.
    pub fn flags_mut(&mut self) -> &mut BuildAccelerationStructureFlagsNV {
        &mut self.flags
    }
    ///Get a mutable reference to the `instance_count` field.
    pub fn instance_count_mut(&mut self) -> &mut u32 {
        &mut self.instance_count
    }
    ///Get a mutable reference to the `geometries` field.
    pub fn geometries_mut(&mut self) -> &mut SmallVec<[GeometryNV; 8]> {
        &mut self.geometries
    }
    ///Sets the `type_` field.
    pub fn set_type_(&mut self, type_: AccelerationStructureTypeNV) -> &mut Self {
        self.type_ = type_;
        self
    }
    ///Sets the `flags` field.
    pub fn set_flags(&mut self, flags: BuildAccelerationStructureFlagsNV) -> &mut Self {
        self.flags = flags;
        self
    }
    ///Sets the `instance_count` field.
    pub fn set_instance_count(&mut self, instance_count: u32) -> &mut Self {
        self.instance_count = instance_count;
        self
    }
    ///Sets the `geometries` field.
    pub fn set_geometries(&mut self, geometries: SmallVec<[GeometryNV; 8]>) -> &mut Self {
        self.geometries = geometries;
        self
    }
    ///Sets the `type_` field in a builder way.
    pub fn with_type_(mut self, type_: AccelerationStructureTypeNV) -> Self {
        self.type_ = type_;
        self
    }
    ///Sets the `flags` field in a builder way.
    pub fn with_flags(mut self, flags: BuildAccelerationStructureFlagsNV) -> Self {
        self.flags = flags;
        self
    }
    ///Sets the `instance_count` field in a builder way.
    pub fn with_instance_count(mut self, instance_count: u32) -> Self {
        self.instance_count = instance_count;
        self
    }
    ///Sets the `geometries` field in a builder way.
    pub fn with_geometries(mut self, geometries: SmallVec<[GeometryNV; 8]>) -> Self {
        self.geometries = geometries;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for AccelerationStructureInfoNV {
    type LowLevel = crate::native::extensions::nv_ray_tracing::AccelerationStructureInfoNV;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let len_geometries = self.geometries.len() as u32;
        let geometries = bump
            .alloc_slice_fill_iter(self.geometries.iter().map(|x| x.into_low_level(context, bump)))
            .as_ptr()
            .cast();
        crate::native::extensions::nv_ray_tracing::AccelerationStructureInfoNV {
            s_type: StructureType::AccelerationStructureInfoNv,
            p_next: std::ptr::null(),
            type_: self.type_.into_low_level(context, bump),
            flags: self.flags.into_low_level(context, bump),
            instance_count: self.instance_count.into_low_level(context, bump),
            geometry_count: len_geometries,
            geometries: geometries,
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for AccelerationStructureInfoNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let geometries_len = value.geometry_count;
        let mut geometries = SmallVec::with_capacity(geometries_len as usize);
        for i in 0..geometries_len {
            geometries.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.geometries.add(i as usize).read(),
            ));
        }
        Self {
            type_: crate::conv::FromLowLevel::from_low_level(context, value.type_),
            flags: crate::conv::FromLowLevel::from_low_level(context, value.flags),
            instance_count: crate::conv::FromLowLevel::from_low_level(context, value.instance_count),
            geometries: geometries,
        }
    }
}
#[doc(alias = "VkAccelerationStructureCreateInfoNV")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AccelerationStructureCreateInfoNV {
    #[doc(alias = "compactedSize")]
    pub compacted_size: DeviceSize,
    pub info: AccelerationStructureInfoNV,
}
impl AccelerationStructureCreateInfoNV {
    ///Get a reference to the `compacted_size` field.
    pub fn compacted_size(&self) -> &DeviceSize {
        &self.compacted_size
    }
    ///Get a reference to the `info` field.
    pub fn info(&self) -> &AccelerationStructureInfoNV {
        &self.info
    }
    ///Get a mutable reference to the `compacted_size` field.
    pub fn compacted_size_mut(&mut self) -> &mut DeviceSize {
        &mut self.compacted_size
    }
    ///Get a mutable reference to the `info` field.
    pub fn info_mut(&mut self) -> &mut AccelerationStructureInfoNV {
        &mut self.info
    }
    ///Sets the `compacted_size` field.
    pub fn set_compacted_size(&mut self, compacted_size: DeviceSize) -> &mut Self {
        self.compacted_size = compacted_size;
        self
    }
    ///Sets the `info` field.
    pub fn set_info(&mut self, info: AccelerationStructureInfoNV) -> &mut Self {
        self.info = info;
        self
    }
    ///Sets the `compacted_size` field in a builder way.
    pub fn with_compacted_size(mut self, compacted_size: DeviceSize) -> Self {
        self.compacted_size = compacted_size;
        self
    }
    ///Sets the `info` field in a builder way.
    pub fn with_info(mut self, info: AccelerationStructureInfoNV) -> Self {
        self.info = info;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for AccelerationStructureCreateInfoNV {
    type LowLevel = crate::native::extensions::nv_ray_tracing::AccelerationStructureCreateInfoNV;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::nv_ray_tracing::AccelerationStructureCreateInfoNV {
            s_type: StructureType::AccelerationStructureCreateInfoNv,
            p_next: std::ptr::null(),
            compacted_size: self.compacted_size.into_low_level(context, bump),
            info: self.info.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for AccelerationStructureCreateInfoNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            compacted_size: crate::conv::FromLowLevel::from_low_level(context, value.compacted_size),
            info: crate::conv::FromLowLevel::from_low_level(context, value.info),
        }
    }
}
#[doc(alias = "VkBindAccelerationStructureMemoryInfoNV")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BindAccelerationStructureMemoryInfoNV {
    #[doc(alias = "accelerationStructure")]
    pub acceleration_structure: AccelerationStructureNV,
    pub memory: DeviceMemory,
    #[doc(alias = "memoryOffset")]
    pub memory_offset: DeviceSize,
    #[doc(alias = "pDeviceIndices")]
    pub device_indices: SmallVec<[u32; 8]>,
}
impl BindAccelerationStructureMemoryInfoNV {
    ///Get a reference to the `acceleration_structure` field.
    pub fn acceleration_structure(&self) -> &AccelerationStructureNV {
        &self.acceleration_structure
    }
    ///Get a reference to the `memory` field.
    pub fn memory(&self) -> &DeviceMemory {
        &self.memory
    }
    ///Get a reference to the `memory_offset` field.
    pub fn memory_offset(&self) -> &DeviceSize {
        &self.memory_offset
    }
    ///Get a reference to the `device_indices` field.
    pub fn device_indices(&self) -> &SmallVec<[u32; 8]> {
        &self.device_indices
    }
    ///Get a mutable reference to the `acceleration_structure` field.
    pub fn acceleration_structure_mut(&mut self) -> &mut AccelerationStructureNV {
        &mut self.acceleration_structure
    }
    ///Get a mutable reference to the `memory` field.
    pub fn memory_mut(&mut self) -> &mut DeviceMemory {
        &mut self.memory
    }
    ///Get a mutable reference to the `memory_offset` field.
    pub fn memory_offset_mut(&mut self) -> &mut DeviceSize {
        &mut self.memory_offset
    }
    ///Get a mutable reference to the `device_indices` field.
    pub fn device_indices_mut(&mut self) -> &mut SmallVec<[u32; 8]> {
        &mut self.device_indices
    }
    ///Sets the `acceleration_structure` field.
    pub fn set_acceleration_structure(&mut self, acceleration_structure: AccelerationStructureNV) -> &mut Self {
        self.acceleration_structure = acceleration_structure;
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
    ///Sets the `device_indices` field.
    pub fn set_device_indices(&mut self, device_indices: SmallVec<[u32; 8]>) -> &mut Self {
        self.device_indices = device_indices;
        self
    }
    ///Sets the `acceleration_structure` field in a builder way.
    pub fn with_acceleration_structure(mut self, acceleration_structure: AccelerationStructureNV) -> Self {
        self.acceleration_structure = acceleration_structure;
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
    ///Sets the `device_indices` field in a builder way.
    pub fn with_device_indices(mut self, device_indices: SmallVec<[u32; 8]>) -> Self {
        self.device_indices = device_indices;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for BindAccelerationStructureMemoryInfoNV {
    type LowLevel = crate::native::extensions::nv_ray_tracing::BindAccelerationStructureMemoryInfoNV;
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
        crate::native::extensions::nv_ray_tracing::BindAccelerationStructureMemoryInfoNV {
            s_type: StructureType::BindAccelerationStructureMemoryInfoNv,
            p_next: std::ptr::null(),
            acceleration_structure: self.acceleration_structure.into_low_level(context, bump),
            memory: self.memory.into_low_level(context, bump),
            memory_offset: self.memory_offset.into_low_level(context, bump),
            device_index_count: len_device_indices,
            device_indices: device_indices,
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for BindAccelerationStructureMemoryInfoNV {
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
            acceleration_structure: crate::conv::FromLowLevel::from_low_level(context, value.acceleration_structure),
            memory: crate::conv::FromLowLevel::from_low_level(context, value.memory),
            memory_offset: crate::conv::FromLowLevel::from_low_level(context, value.memory_offset),
            device_indices: device_indices,
        }
    }
}
#[doc(alias = "VkWriteDescriptorSetAccelerationStructureNV")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct WriteDescriptorSetAccelerationStructureNV {
    #[doc(alias = "pAccelerationStructures")]
    pub acceleration_structures: SmallVec<[AccelerationStructureNV; 8]>,
}
impl WriteDescriptorSetAccelerationStructureNV {
    ///Get a reference to the `acceleration_structures` field.
    pub fn acceleration_structures(&self) -> &SmallVec<[AccelerationStructureNV; 8]> {
        &self.acceleration_structures
    }
    ///Get a mutable reference to the `acceleration_structures` field.
    pub fn acceleration_structures_mut(&mut self) -> &mut SmallVec<[AccelerationStructureNV; 8]> {
        &mut self.acceleration_structures
    }
    ///Sets the `acceleration_structures` field.
    pub fn set_acceleration_structures(
        &mut self,
        acceleration_structures: SmallVec<[AccelerationStructureNV; 8]>,
    ) -> &mut Self {
        self.acceleration_structures = acceleration_structures;
        self
    }
    ///Sets the `acceleration_structures` field in a builder way.
    pub fn with_acceleration_structures(
        mut self,
        acceleration_structures: SmallVec<[AccelerationStructureNV; 8]>,
    ) -> Self {
        self.acceleration_structures = acceleration_structures;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for WriteDescriptorSetAccelerationStructureNV {
    type LowLevel = crate::native::extensions::nv_ray_tracing::WriteDescriptorSetAccelerationStructureNV;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let len_acceleration_structures = self.acceleration_structures.len() as u32;
        let acceleration_structures = bump
            .alloc_slice_fill_iter(
                self.acceleration_structures
                    .iter()
                    .map(|x| x.into_low_level(context, bump)),
            )
            .as_ptr()
            .cast();
        crate::native::extensions::nv_ray_tracing::WriteDescriptorSetAccelerationStructureNV {
            s_type: StructureType::WriteDescriptorSetAccelerationStructureNv,
            p_next: std::ptr::null(),
            acceleration_structure_count: len_acceleration_structures,
            acceleration_structures: acceleration_structures,
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for WriteDescriptorSetAccelerationStructureNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let acceleration_structures_len = value.acceleration_structure_count;
        let mut acceleration_structures = SmallVec::with_capacity(acceleration_structures_len as usize);
        for i in 0..acceleration_structures_len {
            acceleration_structures.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.acceleration_structures.add(i as usize).read(),
            ));
        }
        Self {
            acceleration_structures: acceleration_structures,
        }
    }
}
#[doc(alias = "VkAccelerationStructureMemoryRequirementsInfoNV")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AccelerationStructureMemoryRequirementsInfoNV {
    #[doc(alias = "type")]
    pub type_: AccelerationStructureMemoryRequirementsTypeNV,
    #[doc(alias = "accelerationStructure")]
    pub acceleration_structure: AccelerationStructureNV,
}
impl AccelerationStructureMemoryRequirementsInfoNV {
    ///Get a reference to the `type_` field.
    pub fn type_(&self) -> AccelerationStructureMemoryRequirementsTypeNV {
        self.type_
    }
    ///Get a reference to the `acceleration_structure` field.
    pub fn acceleration_structure(&self) -> &AccelerationStructureNV {
        &self.acceleration_structure
    }
    ///Get a mutable reference to the `type_` field.
    pub fn type__mut(&mut self) -> &mut AccelerationStructureMemoryRequirementsTypeNV {
        &mut self.type_
    }
    ///Get a mutable reference to the `acceleration_structure` field.
    pub fn acceleration_structure_mut(&mut self) -> &mut AccelerationStructureNV {
        &mut self.acceleration_structure
    }
    ///Sets the `type_` field.
    pub fn set_type_(&mut self, type_: AccelerationStructureMemoryRequirementsTypeNV) -> &mut Self {
        self.type_ = type_;
        self
    }
    ///Sets the `acceleration_structure` field.
    pub fn set_acceleration_structure(&mut self, acceleration_structure: AccelerationStructureNV) -> &mut Self {
        self.acceleration_structure = acceleration_structure;
        self
    }
    ///Sets the `type_` field in a builder way.
    pub fn with_type_(mut self, type_: AccelerationStructureMemoryRequirementsTypeNV) -> Self {
        self.type_ = type_;
        self
    }
    ///Sets the `acceleration_structure` field in a builder way.
    pub fn with_acceleration_structure(mut self, acceleration_structure: AccelerationStructureNV) -> Self {
        self.acceleration_structure = acceleration_structure;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for AccelerationStructureMemoryRequirementsInfoNV {
    type LowLevel = crate::native::extensions::nv_ray_tracing::AccelerationStructureMemoryRequirementsInfoNV;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::nv_ray_tracing::AccelerationStructureMemoryRequirementsInfoNV {
            s_type: StructureType::AccelerationStructureMemoryRequirementsInfoNv,
            p_next: std::ptr::null(),
            type_: self.type_.into_low_level(context, bump),
            acceleration_structure: self.acceleration_structure.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for AccelerationStructureMemoryRequirementsInfoNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            type_: crate::conv::FromLowLevel::from_low_level(context, value.type_),
            acceleration_structure: crate::conv::FromLowLevel::from_low_level(context, value.acceleration_structure),
        }
    }
}
#[doc(alias = "VkPhysicalDeviceRayTracingPropertiesNV")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceRayTracingPropertiesNV {
    #[doc(alias = "shaderGroupHandleSize")]
    pub shader_group_handle_size: u32,
    #[doc(alias = "maxRecursionDepth")]
    pub max_recursion_depth: u32,
    #[doc(alias = "maxShaderGroupStride")]
    pub max_shader_group_stride: u32,
    #[doc(alias = "shaderGroupBaseAlignment")]
    pub shader_group_base_alignment: u32,
    #[doc(alias = "maxGeometryCount")]
    pub max_geometry_count: u64,
    #[doc(alias = "maxInstanceCount")]
    pub max_instance_count: u64,
    #[doc(alias = "maxTriangleCount")]
    pub max_triangle_count: u64,
    #[doc(alias = "maxDescriptorSetAccelerationStructures")]
    pub max_descriptor_set_acceleration_structures: u32,
}
impl PhysicalDeviceRayTracingPropertiesNV {
    ///Get a reference to the `shader_group_handle_size` field.
    pub fn shader_group_handle_size(&self) -> u32 {
        self.shader_group_handle_size
    }
    ///Get a reference to the `max_recursion_depth` field.
    pub fn max_recursion_depth(&self) -> u32 {
        self.max_recursion_depth
    }
    ///Get a reference to the `max_shader_group_stride` field.
    pub fn max_shader_group_stride(&self) -> u32 {
        self.max_shader_group_stride
    }
    ///Get a reference to the `shader_group_base_alignment` field.
    pub fn shader_group_base_alignment(&self) -> u32 {
        self.shader_group_base_alignment
    }
    ///Get a reference to the `max_geometry_count` field.
    pub fn max_geometry_count(&self) -> u64 {
        self.max_geometry_count
    }
    ///Get a reference to the `max_instance_count` field.
    pub fn max_instance_count(&self) -> u64 {
        self.max_instance_count
    }
    ///Get a reference to the `max_triangle_count` field.
    pub fn max_triangle_count(&self) -> u64 {
        self.max_triangle_count
    }
    ///Get a reference to the `max_descriptor_set_acceleration_structures` field.
    pub fn max_descriptor_set_acceleration_structures(&self) -> u32 {
        self.max_descriptor_set_acceleration_structures
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceRayTracingPropertiesNV {
    type LowLevel = crate::native::extensions::nv_ray_tracing::PhysicalDeviceRayTracingPropertiesNV;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::nv_ray_tracing::PhysicalDeviceRayTracingPropertiesNV {
            s_type: StructureType::PhysicalDeviceRayTracingPropertiesNv,
            p_next: std::ptr::null_mut(),
            shader_group_handle_size: self.shader_group_handle_size.into_low_level(context, bump),
            max_recursion_depth: self.max_recursion_depth.into_low_level(context, bump),
            max_shader_group_stride: self.max_shader_group_stride.into_low_level(context, bump),
            shader_group_base_alignment: self.shader_group_base_alignment.into_low_level(context, bump),
            max_geometry_count: self.max_geometry_count.into_low_level(context, bump),
            max_instance_count: self.max_instance_count.into_low_level(context, bump),
            max_triangle_count: self.max_triangle_count.into_low_level(context, bump),
            max_descriptor_set_acceleration_structures: self
                .max_descriptor_set_acceleration_structures
                .into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceRayTracingPropertiesNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            shader_group_handle_size: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_group_handle_size,
            ),
            max_recursion_depth: crate::conv::FromLowLevel::from_low_level(context, value.max_recursion_depth),
            max_shader_group_stride: crate::conv::FromLowLevel::from_low_level(context, value.max_shader_group_stride),
            shader_group_base_alignment: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_group_base_alignment,
            ),
            max_geometry_count: crate::conv::FromLowLevel::from_low_level(context, value.max_geometry_count),
            max_instance_count: crate::conv::FromLowLevel::from_low_level(context, value.max_instance_count),
            max_triangle_count: crate::conv::FromLowLevel::from_low_level(context, value.max_triangle_count),
            max_descriptor_set_acceleration_structures: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_descriptor_set_acceleration_structures,
            ),
        }
    }
}
#[doc(alias = "VkAccelerationStructureNV")]
#[derive(Debug)]
pub struct AccelerationStructureNV {
    context: Arc<Context>,
    id: ObjectId,
}
impl Clone for AccelerationStructureNV {
    fn clone(&self) -> Self {
        self.context.clone_acceleration_structure_nv(self.id);
        Self {
            context: Arc::clone(&self.context),
            id: self.id,
        }
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for AccelerationStructureNV {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.id.serialize(serializer)
    }
}
impl<'de> Deserialize<'de> for AccelerationStructureNV {
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
impl Drop for AccelerationStructureNV {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            self.context.drop_acceleration_structure_nv(&self.id);
        }
    }
}
impl PartialEq for AccelerationStructureNV {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl AccelerationStructureNV {
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
unsafe impl crate::conv::IntoLowLevel for AccelerationStructureNV {
    type LowLevel = crate::native::extensions::nv_ray_tracing::AccelerationStructureNV;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        *context
            .acceleration_structure_nv()
            .get(&self.id)
            .expect("unknwon handle")
            .handle()
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for AccelerationStructureNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let object_id = ObjectId::random();
        context
            .acceleration_structure_nv()
            .insert(object_id, Container::new(value));
        Self {
            context: context.clone(),
            id: object_id,
        }
    }
}
