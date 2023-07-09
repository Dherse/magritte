use crate::cstr;
use std::ffi::CStr;
#[doc(alias = "VkDrawMeshTasksIndirectCommandNV")]
#[derive(Clone, Copy, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct DrawMeshTasksIndirectCommandNV {
    #[doc(alias = "taskCount")]
    pub task_count: u32,
    #[doc(alias = "firstTask")]
    pub first_task: u32,
}
#[doc(alias = "VK_NV_MESH_SHADER_SPEC_VERSION")]
pub const NV_MESH_SHADER_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_NV_MESH_SHADER_EXTENSION_NAME")]
pub const NV_MESH_SHADER_EXTENSION_NAME: &'static CStr = cstr!("VK_NV_mesh_shader");
