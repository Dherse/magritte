pub use crate::common::extensions::nv_mesh_shader::DrawMeshTasksIndirectCommandNV;
use crate::native::vulkan1_0::{BaseOutStructure, Bool32, Buffer, CommandBuffer, DeviceSize, StructureType};
#[doc(alias = "VkPhysicalDeviceMeshShaderFeaturesNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceMeshShaderFeaturesNV {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "taskShader")]
    pub task_shader: Bool32,
    #[doc(alias = "meshShader")]
    pub mesh_shader: Bool32,
}
impl Default for PhysicalDeviceMeshShaderFeaturesNV {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceMeshShaderFeaturesNv,
            p_next: unsafe { std::mem::zeroed() },
            task_shader: unsafe { std::mem::zeroed() },
            mesh_shader: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceMeshShaderPropertiesNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceMeshShaderPropertiesNV {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
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
impl Default for PhysicalDeviceMeshShaderPropertiesNV {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceMeshShaderPropertiesNv,
            p_next: unsafe { std::mem::zeroed() },
            max_draw_mesh_tasks_count: unsafe { std::mem::zeroed() },
            max_task_work_group_invocations: unsafe { std::mem::zeroed() },
            max_task_work_group_size: unsafe { std::mem::zeroed() },
            max_task_total_memory_size: unsafe { std::mem::zeroed() },
            max_task_output_count: unsafe { std::mem::zeroed() },
            max_mesh_work_group_invocations: unsafe { std::mem::zeroed() },
            max_mesh_work_group_size: unsafe { std::mem::zeroed() },
            max_mesh_total_memory_size: unsafe { std::mem::zeroed() },
            max_mesh_output_vertices: unsafe { std::mem::zeroed() },
            max_mesh_output_primitives: unsafe { std::mem::zeroed() },
            max_mesh_multiview_view_count: unsafe { std::mem::zeroed() },
            mesh_output_per_vertex_granularity: unsafe { std::mem::zeroed() },
            mesh_output_per_primitive_granularity: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::nv_mesh_shader::{NV_MESH_SHADER_EXTENSION_NAME, NV_MESH_SHADER_SPEC_VERSION};
#[doc(alias = "vkCmdDrawMeshTasksNV")]
pub type FNCmdDrawMeshTasksNv =
    unsafe extern "system" fn(command_buffer: CommandBuffer, task_count: u32, first_task: u32);
#[doc(alias = "vkCmdDrawMeshTasksIndirectNV")]
pub type FNCmdDrawMeshTasksIndirectNv = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    buffer: Buffer,
    offset: DeviceSize,
    draw_count: u32,
    stride: u32,
);
#[doc(alias = "vkCmdDrawMeshTasksIndirectCountNV")]
pub type FNCmdDrawMeshTasksIndirectCountNv = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    buffer: Buffer,
    offset: DeviceSize,
    count_buffer: Buffer,
    count_buffer_offset: DeviceSize,
    max_draw_count: u32,
    stride: u32,
);
