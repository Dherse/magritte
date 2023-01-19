//!# [VK_NV_mesh_shader](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_NV_mesh_shader.html)
# ! [doc = include_str ! ("../../../../doc/extensions/nv_mesh_shader/VK_NV_mesh_shader.md")]
use crate::{
    cstr,
    vulkan1_0::{BaseOutStructure, Bool32, Buffer, CommandBuffer, DeviceSize, StructureType},
};
use std::ffi::CStr;
///# [VkPhysicalDeviceMeshShaderFeaturesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMeshShaderFeaturesNV.html)
# [doc = include_str ! ("../../../../doc/extensions/nv_mesh_shader/VkPhysicalDeviceMeshShaderFeaturesNV.md")]
#[doc(alias = "VkPhysicalDeviceMeshShaderFeaturesNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceMeshShaderFeaturesNV {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "taskShader")]
    task_shader: Bool32,
    #[doc(alias = "meshShader")]
    mesh_shader: Bool32,
}
///# [VkPhysicalDeviceMeshShaderPropertiesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMeshShaderPropertiesNV.html)
# [doc = include_str ! ("../../../../doc/extensions/nv_mesh_shader/VkPhysicalDeviceMeshShaderPropertiesNV.md")]
#[doc(alias = "VkPhysicalDeviceMeshShaderPropertiesNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceMeshShaderPropertiesNV {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "maxDrawMeshTasksCount")]
    max_draw_mesh_tasks_count: u32,
    #[doc(alias = "maxTaskWorkGroupInvocations")]
    max_task_work_group_invocations: u32,
    #[doc(alias = "maxTaskWorkGroupSize")]
    max_task_work_group_size: [u32; 3 as usize],
    #[doc(alias = "maxTaskTotalMemorySize")]
    max_task_total_memory_size: u32,
    #[doc(alias = "maxTaskOutputCount")]
    max_task_output_count: u32,
    #[doc(alias = "maxMeshWorkGroupInvocations")]
    max_mesh_work_group_invocations: u32,
    #[doc(alias = "maxMeshWorkGroupSize")]
    max_mesh_work_group_size: [u32; 3 as usize],
    #[doc(alias = "maxMeshTotalMemorySize")]
    max_mesh_total_memory_size: u32,
    #[doc(alias = "maxMeshOutputVertices")]
    max_mesh_output_vertices: u32,
    #[doc(alias = "maxMeshOutputPrimitives")]
    max_mesh_output_primitives: u32,
    #[doc(alias = "maxMeshMultiviewViewCount")]
    max_mesh_multiview_view_count: u32,
    #[doc(alias = "meshOutputPerVertexGranularity")]
    mesh_output_per_vertex_granularity: u32,
    #[doc(alias = "meshOutputPerPrimitiveGranularity")]
    mesh_output_per_primitive_granularity: u32,
}
///# [VkDrawMeshTasksIndirectCommandNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDrawMeshTasksIndirectCommandNV.html)
# [doc = include_str ! ("../../../../doc/extensions/nv_mesh_shader/VkDrawMeshTasksIndirectCommandNV.md")]
#[doc(alias = "VkDrawMeshTasksIndirectCommandNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DrawMeshTasksIndirectCommandNV {
    #[doc(alias = "taskCount")]
    task_count: u32,
    #[doc(alias = "firstTask")]
    first_task: u32,
}
#[doc(alias = "VK_NV_MESH_SHADER_SPEC_VERSION")]
pub const NV_MESH_SHADER_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_NV_MESH_SHADER_EXTENSION_NAME")]
pub const NV_MESH_SHADER_EXTENSION_NAME: &'static CStr = cstr!("VK_NV_mesh_shader");
///# [vkCmdDrawMeshTasksNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDrawMeshTasksNV.html)
# [doc = include_str ! ("../../../../doc/extensions/nv_mesh_shader/vkCmdDrawMeshTasksNV.md")]
#[doc(alias = "vkCmdDrawMeshTasksNV")]
pub type FNCmdDrawMeshTasksNv =
    unsafe extern "system" fn(command_buffer: CommandBuffer, task_count: u32, first_task: u32);
///# [vkCmdDrawMeshTasksIndirectNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDrawMeshTasksIndirectNV.html)
# [doc = include_str ! ("../../../../doc/extensions/nv_mesh_shader/vkCmdDrawMeshTasksIndirectNV.md")]
#[doc(alias = "vkCmdDrawMeshTasksIndirectNV")]
pub type FNCmdDrawMeshTasksIndirectNv = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    buffer: Buffer,
    offset: DeviceSize,
    draw_count: u32,
    stride: u32,
);
///# [vkCmdDrawMeshTasksIndirectCountNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDrawMeshTasksIndirectCountNV.html)
# [doc = include_str ! ("../../../../doc/extensions/nv_mesh_shader/vkCmdDrawMeshTasksIndirectCountNV.md")]
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
