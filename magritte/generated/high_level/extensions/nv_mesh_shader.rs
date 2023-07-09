pub use crate::common::extensions::nv_mesh_shader::{
    DrawMeshTasksIndirectCommandNV, NV_MESH_SHADER_EXTENSION_NAME, NV_MESH_SHADER_SPEC_VERSION,
};
use crate::{context::Context, vulkan1_0::StructureType};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkPhysicalDeviceMeshShaderFeaturesNV")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceMeshShaderFeaturesNV {
    #[doc(alias = "taskShader")]
    pub task_shader: bool,
    #[doc(alias = "meshShader")]
    pub mesh_shader: bool,
}
impl PhysicalDeviceMeshShaderFeaturesNV {
    ///Get a reference to the `task_shader` field.
    pub fn task_shader(&self) -> &bool {
        &self.task_shader
    }
    ///Get a reference to the `mesh_shader` field.
    pub fn mesh_shader(&self) -> &bool {
        &self.mesh_shader
    }
    ///Get a mutable reference to the `task_shader` field.
    pub fn task_shader_mut(&mut self) -> &mut bool {
        &mut self.task_shader
    }
    ///Get a mutable reference to the `mesh_shader` field.
    pub fn mesh_shader_mut(&mut self) -> &mut bool {
        &mut self.mesh_shader
    }
    ///Sets the `task_shader` field.
    pub fn set_task_shader(&mut self, task_shader: bool) -> &mut Self {
        self.task_shader = task_shader;
        self
    }
    ///Sets the `mesh_shader` field.
    pub fn set_mesh_shader(&mut self, mesh_shader: bool) -> &mut Self {
        self.mesh_shader = mesh_shader;
        self
    }
    ///Sets the `task_shader` field in a builder way.
    pub fn with_task_shader(mut self, task_shader: bool) -> Self {
        self.task_shader = task_shader;
        self
    }
    ///Sets the `mesh_shader` field in a builder way.
    pub fn with_mesh_shader(mut self, mesh_shader: bool) -> Self {
        self.mesh_shader = mesh_shader;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceMeshShaderFeaturesNV {
    type LowLevel = crate::native::extensions::nv_mesh_shader::PhysicalDeviceMeshShaderFeaturesNV;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::nv_mesh_shader::PhysicalDeviceMeshShaderFeaturesNV {
            s_type: StructureType::PhysicalDeviceMeshShaderFeaturesNv,
            p_next: std::ptr::null_mut(),
            task_shader: self.task_shader.into_low_level(context, bump),
            mesh_shader: self.mesh_shader.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceMeshShaderFeaturesNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            task_shader: crate::conv::FromLowLevel::from_low_level(context, value.task_shader),
            mesh_shader: crate::conv::FromLowLevel::from_low_level(context, value.mesh_shader),
        }
    }
}
#[doc(alias = "VkPhysicalDeviceMeshShaderPropertiesNV")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceMeshShaderPropertiesNV {
    #[doc(alias = "maxDrawMeshTasksCount")]
    pub max_draw_mesh_tasks_count: u32,
    #[doc(alias = "maxTaskWorkGroupInvocations")]
    pub max_task_work_group_invocations: u32,
    #[doc(alias = "maxTaskWorkGroupSize")]
    pub max_task_work_group_size: [u32; 3 as usize],
    #[doc(alias = "maxTaskTotalMemorySize")]
    pub max_task_total_memory_size: u32,
    #[doc(alias = "maxTaskOutputCount")]
    pub max_task_output_count: u32,
    #[doc(alias = "maxMeshWorkGroupInvocations")]
    pub max_mesh_work_group_invocations: u32,
    #[doc(alias = "maxMeshWorkGroupSize")]
    pub max_mesh_work_group_size: [u32; 3 as usize],
    #[doc(alias = "maxMeshTotalMemorySize")]
    pub max_mesh_total_memory_size: u32,
    #[doc(alias = "maxMeshOutputVertices")]
    pub max_mesh_output_vertices: u32,
    #[doc(alias = "maxMeshOutputPrimitives")]
    pub max_mesh_output_primitives: u32,
    #[doc(alias = "maxMeshMultiviewViewCount")]
    pub max_mesh_multiview_view_count: u32,
    #[doc(alias = "meshOutputPerVertexGranularity")]
    pub mesh_output_per_vertex_granularity: u32,
    #[doc(alias = "meshOutputPerPrimitiveGranularity")]
    pub mesh_output_per_primitive_granularity: u32,
}
impl PhysicalDeviceMeshShaderPropertiesNV {
    ///Get a reference to the `max_draw_mesh_tasks_count` field.
    pub fn max_draw_mesh_tasks_count(&self) -> u32 {
        self.max_draw_mesh_tasks_count
    }
    ///Get a reference to the `max_task_work_group_invocations` field.
    pub fn max_task_work_group_invocations(&self) -> u32 {
        self.max_task_work_group_invocations
    }
    ///Get a reference to the `max_task_work_group_size` field.
    pub fn max_task_work_group_size(&self) -> [u32; 3 as usize] {
        self.max_task_work_group_size
    }
    ///Get a reference to the `max_task_total_memory_size` field.
    pub fn max_task_total_memory_size(&self) -> u32 {
        self.max_task_total_memory_size
    }
    ///Get a reference to the `max_task_output_count` field.
    pub fn max_task_output_count(&self) -> u32 {
        self.max_task_output_count
    }
    ///Get a reference to the `max_mesh_work_group_invocations` field.
    pub fn max_mesh_work_group_invocations(&self) -> u32 {
        self.max_mesh_work_group_invocations
    }
    ///Get a reference to the `max_mesh_work_group_size` field.
    pub fn max_mesh_work_group_size(&self) -> [u32; 3 as usize] {
        self.max_mesh_work_group_size
    }
    ///Get a reference to the `max_mesh_total_memory_size` field.
    pub fn max_mesh_total_memory_size(&self) -> u32 {
        self.max_mesh_total_memory_size
    }
    ///Get a reference to the `max_mesh_output_vertices` field.
    pub fn max_mesh_output_vertices(&self) -> u32 {
        self.max_mesh_output_vertices
    }
    ///Get a reference to the `max_mesh_output_primitives` field.
    pub fn max_mesh_output_primitives(&self) -> u32 {
        self.max_mesh_output_primitives
    }
    ///Get a reference to the `max_mesh_multiview_view_count` field.
    pub fn max_mesh_multiview_view_count(&self) -> u32 {
        self.max_mesh_multiview_view_count
    }
    ///Get a reference to the `mesh_output_per_vertex_granularity` field.
    pub fn mesh_output_per_vertex_granularity(&self) -> u32 {
        self.mesh_output_per_vertex_granularity
    }
    ///Get a reference to the `mesh_output_per_primitive_granularity` field.
    pub fn mesh_output_per_primitive_granularity(&self) -> u32 {
        self.mesh_output_per_primitive_granularity
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceMeshShaderPropertiesNV {
    type LowLevel = crate::native::extensions::nv_mesh_shader::PhysicalDeviceMeshShaderPropertiesNV;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::nv_mesh_shader::PhysicalDeviceMeshShaderPropertiesNV {
            s_type: StructureType::PhysicalDeviceMeshShaderPropertiesNv,
            p_next: std::ptr::null_mut(),
            max_draw_mesh_tasks_count: self.max_draw_mesh_tasks_count.into_low_level(context, bump),
            max_task_work_group_invocations: self.max_task_work_group_invocations.into_low_level(context, bump),
            max_task_work_group_size: self.max_task_work_group_size.into_low_level(context, bump),
            max_task_total_memory_size: self.max_task_total_memory_size.into_low_level(context, bump),
            max_task_output_count: self.max_task_output_count.into_low_level(context, bump),
            max_mesh_work_group_invocations: self.max_mesh_work_group_invocations.into_low_level(context, bump),
            max_mesh_work_group_size: self.max_mesh_work_group_size.into_low_level(context, bump),
            max_mesh_total_memory_size: self.max_mesh_total_memory_size.into_low_level(context, bump),
            max_mesh_output_vertices: self.max_mesh_output_vertices.into_low_level(context, bump),
            max_mesh_output_primitives: self.max_mesh_output_primitives.into_low_level(context, bump),
            max_mesh_multiview_view_count: self.max_mesh_multiview_view_count.into_low_level(context, bump),
            mesh_output_per_vertex_granularity: self.mesh_output_per_vertex_granularity.into_low_level(context, bump),
            mesh_output_per_primitive_granularity: self
                .mesh_output_per_primitive_granularity
                .into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceMeshShaderPropertiesNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            max_draw_mesh_tasks_count: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_draw_mesh_tasks_count,
            ),
            max_task_work_group_invocations: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_task_work_group_invocations,
            ),
            max_task_work_group_size: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_task_work_group_size,
            ),
            max_task_total_memory_size: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_task_total_memory_size,
            ),
            max_task_output_count: crate::conv::FromLowLevel::from_low_level(context, value.max_task_output_count),
            max_mesh_work_group_invocations: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_mesh_work_group_invocations,
            ),
            max_mesh_work_group_size: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_mesh_work_group_size,
            ),
            max_mesh_total_memory_size: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_mesh_total_memory_size,
            ),
            max_mesh_output_vertices: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_mesh_output_vertices,
            ),
            max_mesh_output_primitives: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_mesh_output_primitives,
            ),
            max_mesh_multiview_view_count: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_mesh_multiview_view_count,
            ),
            mesh_output_per_vertex_granularity: crate::conv::FromLowLevel::from_low_level(
                context,
                value.mesh_output_per_vertex_granularity,
            ),
            mesh_output_per_primitive_granularity: crate::conv::FromLowLevel::from_low_level(
                context,
                value.mesh_output_per_primitive_granularity,
            ),
        }
    }
}
impl DrawMeshTasksIndirectCommandNV {
    ///Get a reference to the `task_count` field.
    pub fn task_count(&self) -> u32 {
        self.task_count
    }
    ///Get a reference to the `first_task` field.
    pub fn first_task(&self) -> u32 {
        self.first_task
    }
    ///Get a mutable reference to the `task_count` field.
    pub fn task_count_mut(&mut self) -> &mut u32 {
        &mut self.task_count
    }
    ///Get a mutable reference to the `first_task` field.
    pub fn first_task_mut(&mut self) -> &mut u32 {
        &mut self.first_task
    }
    ///Sets the `task_count` field.
    pub fn set_task_count(&mut self, task_count: u32) -> &mut Self {
        self.task_count = task_count;
        self
    }
    ///Sets the `first_task` field.
    pub fn set_first_task(&mut self, first_task: u32) -> &mut Self {
        self.first_task = first_task;
        self
    }
    ///Sets the `task_count` field in a builder way.
    pub fn with_task_count(mut self, task_count: u32) -> Self {
        self.task_count = task_count;
        self
    }
    ///Sets the `first_task` field in a builder way.
    pub fn with_first_task(mut self, first_task: u32) -> Self {
        self.first_task = first_task;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for DrawMeshTasksIndirectCommandNV {
    type LowLevel = crate::native::extensions::nv_mesh_shader::DrawMeshTasksIndirectCommandNV;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::nv_mesh_shader::DrawMeshTasksIndirectCommandNV {
            task_count: self.task_count.into_low_level(context, bump),
            first_task: self.first_task.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for DrawMeshTasksIndirectCommandNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            task_count: crate::conv::FromLowLevel::from_low_level(context, value.task_count),
            first_task: crate::conv::FromLowLevel::from_low_level(context, value.first_task),
        }
    }
}
