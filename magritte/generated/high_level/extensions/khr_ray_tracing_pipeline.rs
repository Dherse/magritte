pub use crate::common::extensions::khr_ray_tracing_pipeline::{
    RayTracingShaderGroupTypeKHR, ShaderGroupShaderKHR, StridedDeviceAddressRegionKHR, TraceRaysIndirectCommandKHR,
    KHR_RAY_TRACING_PIPELINE_EXTENSION_NAME, KHR_RAY_TRACING_PIPELINE_SPEC_VERSION,
};
#[cfg(feature = "VK_KHR_pipeline_library")]
use crate::extensions::khr_pipeline_library::PipelineLibraryCreateInfoKHR;
use crate::{
    context::Context,
    vulkan1_0::{
        DeviceAddress, DeviceSize, Pipeline, PipelineCreateFlags, PipelineDynamicStateCreateInfo, PipelineLayout,
        PipelineShaderStageCreateInfo, StructureType,
    },
    vulkan1_3::PipelineCreationFeedbackCreateInfo,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use std::sync::Arc;
#[doc(alias = "VkRayTracingShaderGroupCreateInfoKHR")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RayTracingShaderGroupCreateInfoKHR {
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
impl RayTracingShaderGroupCreateInfoKHR {
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
unsafe impl crate::conv::IntoLowLevel for RayTracingShaderGroupCreateInfoKHR {
    type LowLevel = crate::native::extensions::khr_ray_tracing_pipeline::RayTracingShaderGroupCreateInfoKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::khr_ray_tracing_pipeline::RayTracingShaderGroupCreateInfoKHR {
            s_type: StructureType::RayTracingShaderGroupCreateInfoKhr,
            p_next: std::ptr::null(),
            type_: self.type_.into_low_level(context, bump),
            general_shader: self.general_shader.into_low_level(context, bump),
            closest_hit_shader: self.closest_hit_shader.into_low_level(context, bump),
            any_hit_shader: self.any_hit_shader.into_low_level(context, bump),
            intersection_shader: self.intersection_shader.into_low_level(context, bump),
            shader_group_capture_replay_handle: self.shader_group_capture_replay_handle.as_ptr().cast(),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for RayTracingShaderGroupCreateInfoKHR {
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
#[doc(alias = "VkRayTracingPipelineCreateInfoKHR")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RayTracingPipelineCreateInfoKHR {
    #[doc(alias = "pNext")]
    pub extensions: SmallVec<[RayTracingPipelineCreateInfoKHRExtension; 1]>,
    pub flags: PipelineCreateFlags,
    #[doc(alias = "pStages")]
    pub stages: SmallVec<[PipelineShaderStageCreateInfo; 8]>,
    #[doc(alias = "pGroups")]
    pub groups: SmallVec<[RayTracingShaderGroupCreateInfoKHR; 8]>,
    #[doc(alias = "maxPipelineRayRecursionDepth")]
    pub max_pipeline_ray_recursion_depth: u32,
    #[doc(alias = "pLibraryInfo")]
    pub library_info: Option<PipelineLibraryCreateInfoKHR>,
    #[doc(alias = "pLibraryInterface")]
    pub library_interface: Option<RayTracingPipelineInterfaceCreateInfoKHR>,
    #[doc(alias = "pDynamicState")]
    pub dynamic_state: Option<PipelineDynamicStateCreateInfo>,
    pub layout: PipelineLayout,
    #[doc(alias = "basePipelineHandle")]
    pub base_pipeline_handle: Option<Pipeline>,
    #[doc(alias = "basePipelineIndex")]
    pub base_pipeline_index: i32,
}
impl RayTracingPipelineCreateInfoKHR {
    ///Adds an extension to the struct
    pub fn with_extension(mut self, ext: impl Into<RayTracingPipelineCreateInfoKHRExtension>) -> Self {
        self.extensions.push(ext.into());
        self
    }
    ///Get a reference to the `extensions` field.
    pub fn extensions(&self) -> &SmallVec<[RayTracingPipelineCreateInfoKHRExtension; 1]> {
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
    pub fn groups(&self) -> &SmallVec<[RayTracingShaderGroupCreateInfoKHR; 8]> {
        &self.groups
    }
    ///Get a reference to the `max_pipeline_ray_recursion_depth` field.
    pub fn max_pipeline_ray_recursion_depth(&self) -> u32 {
        self.max_pipeline_ray_recursion_depth
    }
    ///Get a reference to the `library_info` field.
    pub fn library_info(&self) -> &Option<PipelineLibraryCreateInfoKHR> {
        &self.library_info
    }
    ///Get a reference to the `library_interface` field.
    pub fn library_interface(&self) -> &Option<RayTracingPipelineInterfaceCreateInfoKHR> {
        &self.library_interface
    }
    ///Get a reference to the `dynamic_state` field.
    pub fn dynamic_state(&self) -> &Option<PipelineDynamicStateCreateInfo> {
        &self.dynamic_state
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
    pub fn extensions_mut(&mut self) -> &mut SmallVec<[RayTracingPipelineCreateInfoKHRExtension; 1]> {
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
    pub fn groups_mut(&mut self) -> &mut SmallVec<[RayTracingShaderGroupCreateInfoKHR; 8]> {
        &mut self.groups
    }
    ///Get a mutable reference to the `max_pipeline_ray_recursion_depth` field.
    pub fn max_pipeline_ray_recursion_depth_mut(&mut self) -> &mut u32 {
        &mut self.max_pipeline_ray_recursion_depth
    }
    ///Get a mutable reference to the `library_info` field.
    pub fn library_info_mut(&mut self) -> &mut Option<PipelineLibraryCreateInfoKHR> {
        &mut self.library_info
    }
    ///Get a mutable reference to the `library_interface` field.
    pub fn library_interface_mut(&mut self) -> &mut Option<RayTracingPipelineInterfaceCreateInfoKHR> {
        &mut self.library_interface
    }
    ///Get a mutable reference to the `dynamic_state` field.
    pub fn dynamic_state_mut(&mut self) -> &mut Option<PipelineDynamicStateCreateInfo> {
        &mut self.dynamic_state
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
    pub fn set_extensions(&mut self, extensions: SmallVec<[RayTracingPipelineCreateInfoKHRExtension; 1]>) -> &mut Self {
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
    pub fn set_groups(&mut self, groups: SmallVec<[RayTracingShaderGroupCreateInfoKHR; 8]>) -> &mut Self {
        self.groups = groups;
        self
    }
    ///Sets the `max_pipeline_ray_recursion_depth` field.
    pub fn set_max_pipeline_ray_recursion_depth(&mut self, max_pipeline_ray_recursion_depth: u32) -> &mut Self {
        self.max_pipeline_ray_recursion_depth = max_pipeline_ray_recursion_depth;
        self
    }
    ///Sets the `library_info` field.
    pub fn set_library_info(&mut self, library_info: Option<PipelineLibraryCreateInfoKHR>) -> &mut Self {
        self.library_info = library_info;
        self
    }
    ///Sets the `library_interface` field.
    pub fn set_library_interface(
        &mut self,
        library_interface: Option<RayTracingPipelineInterfaceCreateInfoKHR>,
    ) -> &mut Self {
        self.library_interface = library_interface;
        self
    }
    ///Sets the `dynamic_state` field.
    pub fn set_dynamic_state(&mut self, dynamic_state: Option<PipelineDynamicStateCreateInfo>) -> &mut Self {
        self.dynamic_state = dynamic_state;
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
    pub fn with_extensions(mut self, extensions: SmallVec<[RayTracingPipelineCreateInfoKHRExtension; 1]>) -> Self {
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
    pub fn with_groups(mut self, groups: SmallVec<[RayTracingShaderGroupCreateInfoKHR; 8]>) -> Self {
        self.groups = groups;
        self
    }
    ///Sets the `max_pipeline_ray_recursion_depth` field in a builder way.
    pub fn with_max_pipeline_ray_recursion_depth(mut self, max_pipeline_ray_recursion_depth: u32) -> Self {
        self.max_pipeline_ray_recursion_depth = max_pipeline_ray_recursion_depth;
        self
    }
    ///Sets the `library_info` field in a builder way.
    pub fn with_library_info(mut self, library_info: Option<PipelineLibraryCreateInfoKHR>) -> Self {
        self.library_info = library_info;
        self
    }
    ///Sets the `library_interface` field in a builder way.
    pub fn with_library_interface(
        mut self,
        library_interface: Option<RayTracingPipelineInterfaceCreateInfoKHR>,
    ) -> Self {
        self.library_interface = library_interface;
        self
    }
    ///Sets the `dynamic_state` field in a builder way.
    pub fn with_dynamic_state(mut self, dynamic_state: Option<PipelineDynamicStateCreateInfo>) -> Self {
        self.dynamic_state = dynamic_state;
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
unsafe impl crate::conv::IntoLowLevel for RayTracingPipelineCreateInfoKHR {
    type LowLevel = crate::native::extensions::khr_ray_tracing_pipeline::RayTracingPipelineCreateInfoKHR;
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
        crate::native::extensions::khr_ray_tracing_pipeline::RayTracingPipelineCreateInfoKHR {
            s_type: StructureType::RayTracingPipelineCreateInfoKhr,
            p_next: next,
            flags: self.flags.into_low_level(context, bump),
            stage_count: len_stages,
            stages: stages,
            group_count: len_groups,
            groups: groups,
            max_pipeline_ray_recursion_depth: self.max_pipeline_ray_recursion_depth.into_low_level(context, bump),
            library_info: self
                .library_info
                .as_ref()
                .map(|v| bump.alloc(v.into_low_level(context, bump)) as *const _)
                .unwrap_or_else(std::ptr::null),
            library_interface: self
                .library_interface
                .as_ref()
                .map(|v| bump.alloc(v.into_low_level(context, bump)) as *const _)
                .unwrap_or_else(std::ptr::null),
            dynamic_state: self
                .dynamic_state
                .as_ref()
                .map(|v| bump.alloc(v.into_low_level(context, bump)) as *const _)
                .unwrap_or_else(std::ptr::null),
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
unsafe impl crate::conv::FromLowLevel for RayTracingPipelineCreateInfoKHR {
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
            max_pipeline_ray_recursion_depth: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_pipeline_ray_recursion_depth,
            ),
            library_info: crate::conv::FromLowLevel::from_low_level(context, *value.library_info),
            library_interface: crate::conv::FromLowLevel::from_low_level(context, *value.library_interface),
            dynamic_state: crate::conv::FromLowLevel::from_low_level(context, *value.dynamic_state),
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
///Extensions for [`RayTracingPipelineCreateInfoKHR`]
pub enum RayTracingPipelineCreateInfoKHRExtension {
    ///Contains a type [`PipelineCreationFeedbackCreateInfo`] for extending
    /// [`RayTracingPipelineCreateInfoKHR`]
    PipelineCreationFeedbackCreateInfo(PipelineCreationFeedbackCreateInfo),
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for RayTracingPipelineCreateInfoKHRExtension {
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
unsafe impl crate::conv::FromLowLevel for RayTracingPipelineCreateInfoKHRExtension {
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
                stringify!(RayTracingPipelineCreateInfoKHR)
            ),
        }
    }
}
impl From<PipelineCreationFeedbackCreateInfo> for RayTracingPipelineCreateInfoKHRExtension {
    fn from(ext: PipelineCreationFeedbackCreateInfo) -> Self {
        Self::PipelineCreationFeedbackCreateInfo(ext)
    }
}
impl TryInto<PipelineCreationFeedbackCreateInfo> for RayTracingPipelineCreateInfoKHRExtension {
    type Error = RayTracingPipelineCreateInfoKHRExtension;
    fn try_into(self) -> Result<PipelineCreationFeedbackCreateInfo, Self::Error> {
        match self {
            Self::PipelineCreationFeedbackCreateInfo(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[doc(alias = "VkPhysicalDeviceRayTracingPipelineFeaturesKHR")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceRayTracingPipelineFeaturesKHR {
    #[doc(alias = "rayTracingPipeline")]
    pub ray_tracing_pipeline: bool,
    #[doc(alias = "rayTracingPipelineShaderGroupHandleCaptureReplay")]
    pub ray_tracing_pipeline_shader_group_handle_capture_replay: bool,
    #[doc(alias = "rayTracingPipelineShaderGroupHandleCaptureReplayMixed")]
    pub ray_tracing_pipeline_shader_group_handle_capture_replay_mixed: bool,
    #[doc(alias = "rayTracingPipelineTraceRaysIndirect")]
    pub ray_tracing_pipeline_trace_rays_indirect: bool,
    #[doc(alias = "rayTraversalPrimitiveCulling")]
    pub ray_traversal_primitive_culling: bool,
}
impl PhysicalDeviceRayTracingPipelineFeaturesKHR {
    ///Get a reference to the `ray_tracing_pipeline` field.
    pub fn ray_tracing_pipeline(&self) -> &bool {
        &self.ray_tracing_pipeline
    }
    ///Get a reference to the `ray_tracing_pipeline_shader_group_handle_capture_replay` field.
    pub fn ray_tracing_pipeline_shader_group_handle_capture_replay(&self) -> &bool {
        &self.ray_tracing_pipeline_shader_group_handle_capture_replay
    }
    ///Get a reference to the `ray_tracing_pipeline_shader_group_handle_capture_replay_mixed`
    /// field.
    pub fn ray_tracing_pipeline_shader_group_handle_capture_replay_mixed(&self) -> &bool {
        &self.ray_tracing_pipeline_shader_group_handle_capture_replay_mixed
    }
    ///Get a reference to the `ray_tracing_pipeline_trace_rays_indirect` field.
    pub fn ray_tracing_pipeline_trace_rays_indirect(&self) -> &bool {
        &self.ray_tracing_pipeline_trace_rays_indirect
    }
    ///Get a reference to the `ray_traversal_primitive_culling` field.
    pub fn ray_traversal_primitive_culling(&self) -> &bool {
        &self.ray_traversal_primitive_culling
    }
    ///Get a mutable reference to the `ray_tracing_pipeline` field.
    pub fn ray_tracing_pipeline_mut(&mut self) -> &mut bool {
        &mut self.ray_tracing_pipeline
    }
    ///Get a mutable reference to the `ray_tracing_pipeline_shader_group_handle_capture_replay`
    /// field.
    pub fn ray_tracing_pipeline_shader_group_handle_capture_replay_mut(&mut self) -> &mut bool {
        &mut self.ray_tracing_pipeline_shader_group_handle_capture_replay
    }
    ///Get a mutable reference to the
    /// `ray_tracing_pipeline_shader_group_handle_capture_replay_mixed` field.
    pub fn ray_tracing_pipeline_shader_group_handle_capture_replay_mixed_mut(&mut self) -> &mut bool {
        &mut self.ray_tracing_pipeline_shader_group_handle_capture_replay_mixed
    }
    ///Get a mutable reference to the `ray_tracing_pipeline_trace_rays_indirect` field.
    pub fn ray_tracing_pipeline_trace_rays_indirect_mut(&mut self) -> &mut bool {
        &mut self.ray_tracing_pipeline_trace_rays_indirect
    }
    ///Get a mutable reference to the `ray_traversal_primitive_culling` field.
    pub fn ray_traversal_primitive_culling_mut(&mut self) -> &mut bool {
        &mut self.ray_traversal_primitive_culling
    }
    ///Sets the `ray_tracing_pipeline` field.
    pub fn set_ray_tracing_pipeline(&mut self, ray_tracing_pipeline: bool) -> &mut Self {
        self.ray_tracing_pipeline = ray_tracing_pipeline;
        self
    }
    ///Sets the `ray_tracing_pipeline_shader_group_handle_capture_replay` field.
    pub fn set_ray_tracing_pipeline_shader_group_handle_capture_replay(
        &mut self,
        ray_tracing_pipeline_shader_group_handle_capture_replay: bool,
    ) -> &mut Self {
        self.ray_tracing_pipeline_shader_group_handle_capture_replay =
            ray_tracing_pipeline_shader_group_handle_capture_replay;
        self
    }
    ///Sets the `ray_tracing_pipeline_shader_group_handle_capture_replay_mixed` field.
    pub fn set_ray_tracing_pipeline_shader_group_handle_capture_replay_mixed(
        &mut self,
        ray_tracing_pipeline_shader_group_handle_capture_replay_mixed: bool,
    ) -> &mut Self {
        self.ray_tracing_pipeline_shader_group_handle_capture_replay_mixed =
            ray_tracing_pipeline_shader_group_handle_capture_replay_mixed;
        self
    }
    ///Sets the `ray_tracing_pipeline_trace_rays_indirect` field.
    pub fn set_ray_tracing_pipeline_trace_rays_indirect(
        &mut self,
        ray_tracing_pipeline_trace_rays_indirect: bool,
    ) -> &mut Self {
        self.ray_tracing_pipeline_trace_rays_indirect = ray_tracing_pipeline_trace_rays_indirect;
        self
    }
    ///Sets the `ray_traversal_primitive_culling` field.
    pub fn set_ray_traversal_primitive_culling(&mut self, ray_traversal_primitive_culling: bool) -> &mut Self {
        self.ray_traversal_primitive_culling = ray_traversal_primitive_culling;
        self
    }
    ///Sets the `ray_tracing_pipeline` field in a builder way.
    pub fn with_ray_tracing_pipeline(mut self, ray_tracing_pipeline: bool) -> Self {
        self.ray_tracing_pipeline = ray_tracing_pipeline;
        self
    }
    ///Sets the `ray_tracing_pipeline_shader_group_handle_capture_replay` field in a builder way.
    pub fn with_ray_tracing_pipeline_shader_group_handle_capture_replay(
        mut self,
        ray_tracing_pipeline_shader_group_handle_capture_replay: bool,
    ) -> Self {
        self.ray_tracing_pipeline_shader_group_handle_capture_replay =
            ray_tracing_pipeline_shader_group_handle_capture_replay;
        self
    }
    ///Sets the `ray_tracing_pipeline_shader_group_handle_capture_replay_mixed` field in a builder
    /// way.
    pub fn with_ray_tracing_pipeline_shader_group_handle_capture_replay_mixed(
        mut self,
        ray_tracing_pipeline_shader_group_handle_capture_replay_mixed: bool,
    ) -> Self {
        self.ray_tracing_pipeline_shader_group_handle_capture_replay_mixed =
            ray_tracing_pipeline_shader_group_handle_capture_replay_mixed;
        self
    }
    ///Sets the `ray_tracing_pipeline_trace_rays_indirect` field in a builder way.
    pub fn with_ray_tracing_pipeline_trace_rays_indirect(
        mut self,
        ray_tracing_pipeline_trace_rays_indirect: bool,
    ) -> Self {
        self.ray_tracing_pipeline_trace_rays_indirect = ray_tracing_pipeline_trace_rays_indirect;
        self
    }
    ///Sets the `ray_traversal_primitive_culling` field in a builder way.
    pub fn with_ray_traversal_primitive_culling(mut self, ray_traversal_primitive_culling: bool) -> Self {
        self.ray_traversal_primitive_culling = ray_traversal_primitive_culling;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceRayTracingPipelineFeaturesKHR {
    type LowLevel = crate::native::extensions::khr_ray_tracing_pipeline::PhysicalDeviceRayTracingPipelineFeaturesKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::khr_ray_tracing_pipeline::PhysicalDeviceRayTracingPipelineFeaturesKHR {
            s_type: StructureType::PhysicalDeviceRayTracingPipelineFeaturesKhr,
            p_next: std::ptr::null_mut(),
            ray_tracing_pipeline: self.ray_tracing_pipeline.into_low_level(context, bump),
            ray_tracing_pipeline_shader_group_handle_capture_replay: self
                .ray_tracing_pipeline_shader_group_handle_capture_replay
                .into_low_level(context, bump),
            ray_tracing_pipeline_shader_group_handle_capture_replay_mixed: self
                .ray_tracing_pipeline_shader_group_handle_capture_replay_mixed
                .into_low_level(context, bump),
            ray_tracing_pipeline_trace_rays_indirect: self
                .ray_tracing_pipeline_trace_rays_indirect
                .into_low_level(context, bump),
            ray_traversal_primitive_culling: self.ray_traversal_primitive_culling.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceRayTracingPipelineFeaturesKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            ray_tracing_pipeline: crate::conv::FromLowLevel::from_low_level(context, value.ray_tracing_pipeline),
            ray_tracing_pipeline_shader_group_handle_capture_replay: crate::conv::FromLowLevel::from_low_level(
                context,
                value.ray_tracing_pipeline_shader_group_handle_capture_replay,
            ),
            ray_tracing_pipeline_shader_group_handle_capture_replay_mixed: crate::conv::FromLowLevel::from_low_level(
                context,
                value.ray_tracing_pipeline_shader_group_handle_capture_replay_mixed,
            ),
            ray_tracing_pipeline_trace_rays_indirect: crate::conv::FromLowLevel::from_low_level(
                context,
                value.ray_tracing_pipeline_trace_rays_indirect,
            ),
            ray_traversal_primitive_culling: crate::conv::FromLowLevel::from_low_level(
                context,
                value.ray_traversal_primitive_culling,
            ),
        }
    }
}
#[doc(alias = "VkPhysicalDeviceRayTracingPipelinePropertiesKHR")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceRayTracingPipelinePropertiesKHR {
    #[doc(alias = "shaderGroupHandleSize")]
    pub shader_group_handle_size: u32,
    #[doc(alias = "maxRayRecursionDepth")]
    pub max_ray_recursion_depth: u32,
    #[doc(alias = "maxShaderGroupStride")]
    pub max_shader_group_stride: u32,
    #[doc(alias = "shaderGroupBaseAlignment")]
    pub shader_group_base_alignment: u32,
    #[doc(alias = "shaderGroupHandleCaptureReplaySize")]
    pub shader_group_handle_capture_replay_size: u32,
    #[doc(alias = "maxRayDispatchInvocationCount")]
    pub max_ray_dispatch_invocation_count: u32,
    #[doc(alias = "shaderGroupHandleAlignment")]
    pub shader_group_handle_alignment: u32,
    #[doc(alias = "maxRayHitAttributeSize")]
    pub max_ray_hit_attribute_size: u32,
}
impl PhysicalDeviceRayTracingPipelinePropertiesKHR {
    ///Get a reference to the `shader_group_handle_size` field.
    pub fn shader_group_handle_size(&self) -> u32 {
        self.shader_group_handle_size
    }
    ///Get a reference to the `max_ray_recursion_depth` field.
    pub fn max_ray_recursion_depth(&self) -> u32 {
        self.max_ray_recursion_depth
    }
    ///Get a reference to the `max_shader_group_stride` field.
    pub fn max_shader_group_stride(&self) -> u32 {
        self.max_shader_group_stride
    }
    ///Get a reference to the `shader_group_base_alignment` field.
    pub fn shader_group_base_alignment(&self) -> u32 {
        self.shader_group_base_alignment
    }
    ///Get a reference to the `shader_group_handle_capture_replay_size` field.
    pub fn shader_group_handle_capture_replay_size(&self) -> u32 {
        self.shader_group_handle_capture_replay_size
    }
    ///Get a reference to the `max_ray_dispatch_invocation_count` field.
    pub fn max_ray_dispatch_invocation_count(&self) -> u32 {
        self.max_ray_dispatch_invocation_count
    }
    ///Get a reference to the `shader_group_handle_alignment` field.
    pub fn shader_group_handle_alignment(&self) -> u32 {
        self.shader_group_handle_alignment
    }
    ///Get a reference to the `max_ray_hit_attribute_size` field.
    pub fn max_ray_hit_attribute_size(&self) -> u32 {
        self.max_ray_hit_attribute_size
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceRayTracingPipelinePropertiesKHR {
    type LowLevel = crate::native::extensions::khr_ray_tracing_pipeline::PhysicalDeviceRayTracingPipelinePropertiesKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::khr_ray_tracing_pipeline::PhysicalDeviceRayTracingPipelinePropertiesKHR {
            s_type: StructureType::PhysicalDeviceRayTracingPipelinePropertiesKhr,
            p_next: std::ptr::null_mut(),
            shader_group_handle_size: self.shader_group_handle_size.into_low_level(context, bump),
            max_ray_recursion_depth: self.max_ray_recursion_depth.into_low_level(context, bump),
            max_shader_group_stride: self.max_shader_group_stride.into_low_level(context, bump),
            shader_group_base_alignment: self.shader_group_base_alignment.into_low_level(context, bump),
            shader_group_handle_capture_replay_size: self
                .shader_group_handle_capture_replay_size
                .into_low_level(context, bump),
            max_ray_dispatch_invocation_count: self.max_ray_dispatch_invocation_count.into_low_level(context, bump),
            shader_group_handle_alignment: self.shader_group_handle_alignment.into_low_level(context, bump),
            max_ray_hit_attribute_size: self.max_ray_hit_attribute_size.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceRayTracingPipelinePropertiesKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            shader_group_handle_size: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_group_handle_size,
            ),
            max_ray_recursion_depth: crate::conv::FromLowLevel::from_low_level(context, value.max_ray_recursion_depth),
            max_shader_group_stride: crate::conv::FromLowLevel::from_low_level(context, value.max_shader_group_stride),
            shader_group_base_alignment: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_group_base_alignment,
            ),
            shader_group_handle_capture_replay_size: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_group_handle_capture_replay_size,
            ),
            max_ray_dispatch_invocation_count: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_ray_dispatch_invocation_count,
            ),
            shader_group_handle_alignment: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_group_handle_alignment,
            ),
            max_ray_hit_attribute_size: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_ray_hit_attribute_size,
            ),
        }
    }
}
impl StridedDeviceAddressRegionKHR {
    ///Get a reference to the `device_address` field.
    pub fn device_address(&self) -> &DeviceAddress {
        &self.device_address
    }
    ///Get a reference to the `stride` field.
    pub fn stride(&self) -> &DeviceSize {
        &self.stride
    }
    ///Get a reference to the `size` field.
    pub fn size(&self) -> &DeviceSize {
        &self.size
    }
    ///Get a mutable reference to the `device_address` field.
    pub fn device_address_mut(&mut self) -> &mut DeviceAddress {
        &mut self.device_address
    }
    ///Get a mutable reference to the `stride` field.
    pub fn stride_mut(&mut self) -> &mut DeviceSize {
        &mut self.stride
    }
    ///Get a mutable reference to the `size` field.
    pub fn size_mut(&mut self) -> &mut DeviceSize {
        &mut self.size
    }
    ///Sets the `device_address` field.
    pub fn set_device_address(&mut self, device_address: DeviceAddress) -> &mut Self {
        self.device_address = device_address;
        self
    }
    ///Sets the `stride` field.
    pub fn set_stride(&mut self, stride: DeviceSize) -> &mut Self {
        self.stride = stride;
        self
    }
    ///Sets the `size` field.
    pub fn set_size(&mut self, size: DeviceSize) -> &mut Self {
        self.size = size;
        self
    }
    ///Sets the `device_address` field in a builder way.
    pub fn with_device_address(mut self, device_address: DeviceAddress) -> Self {
        self.device_address = device_address;
        self
    }
    ///Sets the `stride` field in a builder way.
    pub fn with_stride(mut self, stride: DeviceSize) -> Self {
        self.stride = stride;
        self
    }
    ///Sets the `size` field in a builder way.
    pub fn with_size(mut self, size: DeviceSize) -> Self {
        self.size = size;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for StridedDeviceAddressRegionKHR {
    type LowLevel = crate::native::extensions::khr_ray_tracing_pipeline::StridedDeviceAddressRegionKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::khr_ray_tracing_pipeline::StridedDeviceAddressRegionKHR {
            device_address: self.device_address.into_low_level(context, bump),
            stride: self.stride.into_low_level(context, bump),
            size: self.size.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for StridedDeviceAddressRegionKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            device_address: crate::conv::FromLowLevel::from_low_level(context, value.device_address),
            stride: crate::conv::FromLowLevel::from_low_level(context, value.stride),
            size: crate::conv::FromLowLevel::from_low_level(context, value.size),
        }
    }
}
impl TraceRaysIndirectCommandKHR {
    ///Get a reference to the `width` field.
    pub fn width(&self) -> u32 {
        self.width
    }
    ///Get a reference to the `height` field.
    pub fn height(&self) -> u32 {
        self.height
    }
    ///Get a reference to the `depth` field.
    pub fn depth(&self) -> u32 {
        self.depth
    }
    ///Get a mutable reference to the `width` field.
    pub fn width_mut(&mut self) -> &mut u32 {
        &mut self.width
    }
    ///Get a mutable reference to the `height` field.
    pub fn height_mut(&mut self) -> &mut u32 {
        &mut self.height
    }
    ///Get a mutable reference to the `depth` field.
    pub fn depth_mut(&mut self) -> &mut u32 {
        &mut self.depth
    }
    ///Sets the `width` field.
    pub fn set_width(&mut self, width: u32) -> &mut Self {
        self.width = width;
        self
    }
    ///Sets the `height` field.
    pub fn set_height(&mut self, height: u32) -> &mut Self {
        self.height = height;
        self
    }
    ///Sets the `depth` field.
    pub fn set_depth(&mut self, depth: u32) -> &mut Self {
        self.depth = depth;
        self
    }
    ///Sets the `width` field in a builder way.
    pub fn with_width(mut self, width: u32) -> Self {
        self.width = width;
        self
    }
    ///Sets the `height` field in a builder way.
    pub fn with_height(mut self, height: u32) -> Self {
        self.height = height;
        self
    }
    ///Sets the `depth` field in a builder way.
    pub fn with_depth(mut self, depth: u32) -> Self {
        self.depth = depth;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for TraceRaysIndirectCommandKHR {
    type LowLevel = crate::native::extensions::khr_ray_tracing_pipeline::TraceRaysIndirectCommandKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::khr_ray_tracing_pipeline::TraceRaysIndirectCommandKHR {
            width: self.width.into_low_level(context, bump),
            height: self.height.into_low_level(context, bump),
            depth: self.depth.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for TraceRaysIndirectCommandKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            width: crate::conv::FromLowLevel::from_low_level(context, value.width),
            height: crate::conv::FromLowLevel::from_low_level(context, value.height),
            depth: crate::conv::FromLowLevel::from_low_level(context, value.depth),
        }
    }
}
#[doc(alias = "VkRayTracingPipelineInterfaceCreateInfoKHR")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RayTracingPipelineInterfaceCreateInfoKHR {
    #[doc(alias = "maxPipelineRayPayloadSize")]
    pub max_pipeline_ray_payload_size: u32,
    #[doc(alias = "maxPipelineRayHitAttributeSize")]
    pub max_pipeline_ray_hit_attribute_size: u32,
}
impl RayTracingPipelineInterfaceCreateInfoKHR {
    ///Get a reference to the `max_pipeline_ray_payload_size` field.
    pub fn max_pipeline_ray_payload_size(&self) -> u32 {
        self.max_pipeline_ray_payload_size
    }
    ///Get a reference to the `max_pipeline_ray_hit_attribute_size` field.
    pub fn max_pipeline_ray_hit_attribute_size(&self) -> u32 {
        self.max_pipeline_ray_hit_attribute_size
    }
    ///Get a mutable reference to the `max_pipeline_ray_payload_size` field.
    pub fn max_pipeline_ray_payload_size_mut(&mut self) -> &mut u32 {
        &mut self.max_pipeline_ray_payload_size
    }
    ///Get a mutable reference to the `max_pipeline_ray_hit_attribute_size` field.
    pub fn max_pipeline_ray_hit_attribute_size_mut(&mut self) -> &mut u32 {
        &mut self.max_pipeline_ray_hit_attribute_size
    }
    ///Sets the `max_pipeline_ray_payload_size` field.
    pub fn set_max_pipeline_ray_payload_size(&mut self, max_pipeline_ray_payload_size: u32) -> &mut Self {
        self.max_pipeline_ray_payload_size = max_pipeline_ray_payload_size;
        self
    }
    ///Sets the `max_pipeline_ray_hit_attribute_size` field.
    pub fn set_max_pipeline_ray_hit_attribute_size(&mut self, max_pipeline_ray_hit_attribute_size: u32) -> &mut Self {
        self.max_pipeline_ray_hit_attribute_size = max_pipeline_ray_hit_attribute_size;
        self
    }
    ///Sets the `max_pipeline_ray_payload_size` field in a builder way.
    pub fn with_max_pipeline_ray_payload_size(mut self, max_pipeline_ray_payload_size: u32) -> Self {
        self.max_pipeline_ray_payload_size = max_pipeline_ray_payload_size;
        self
    }
    ///Sets the `max_pipeline_ray_hit_attribute_size` field in a builder way.
    pub fn with_max_pipeline_ray_hit_attribute_size(mut self, max_pipeline_ray_hit_attribute_size: u32) -> Self {
        self.max_pipeline_ray_hit_attribute_size = max_pipeline_ray_hit_attribute_size;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for RayTracingPipelineInterfaceCreateInfoKHR {
    type LowLevel = crate::native::extensions::khr_ray_tracing_pipeline::RayTracingPipelineInterfaceCreateInfoKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::khr_ray_tracing_pipeline::RayTracingPipelineInterfaceCreateInfoKHR {
            s_type: StructureType::RayTracingPipelineInterfaceCreateInfoKhr,
            p_next: std::ptr::null(),
            max_pipeline_ray_payload_size: self.max_pipeline_ray_payload_size.into_low_level(context, bump),
            max_pipeline_ray_hit_attribute_size: self.max_pipeline_ray_hit_attribute_size.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for RayTracingPipelineInterfaceCreateInfoKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            max_pipeline_ray_payload_size: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_pipeline_ray_payload_size,
            ),
            max_pipeline_ray_hit_attribute_size: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_pipeline_ray_hit_attribute_size,
            ),
        }
    }
}
