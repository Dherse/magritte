use crate::{
    cstr,
    vulkan1_0::{BaseOutStructure, Bool32, CommandBuffer, StructureType},
};
use std::ffi::CStr;
#[doc(alias = "VkMultiDrawInfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct MultiDrawInfoEXT {
    #[doc(alias = "firstVertex")]
    first_vertex: u32,
    #[doc(alias = "vertexCount")]
    vertex_count: u32,
}
#[doc(alias = "VkMultiDrawIndexedInfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct MultiDrawIndexedInfoEXT {
    #[doc(alias = "firstIndex")]
    first_index: u32,
    #[doc(alias = "indexCount")]
    index_count: u32,
    #[doc(alias = "vertexOffset")]
    vertex_offset: i32,
}
#[doc(alias = "VkPhysicalDeviceMultiDrawPropertiesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceMultiDrawPropertiesEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "maxMultiDrawCount")]
    max_multi_draw_count: u32,
}
#[doc(alias = "VkPhysicalDeviceMultiDrawFeaturesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceMultiDrawFeaturesEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "multiDraw")]
    multi_draw: Bool32,
}
#[doc(alias = "VK_EXT_MULTI_DRAW_SPEC_VERSION")]
pub const EXT_MULTI_DRAW_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_EXT_MULTI_DRAW_EXTENSION_NAME")]
pub const EXT_MULTI_DRAW_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_multi_draw");
#[doc(alias = "vkCmdDrawMultiEXT")]
pub type FNCmdDrawMultiExt = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    draw_count: u32,
    p_vertex_info: *const MultiDrawInfoEXT,
    instance_count: u32,
    first_instance: u32,
    stride: u32,
);
#[doc(alias = "vkCmdDrawMultiIndexedEXT")]
pub type FNCmdDrawMultiIndexedExt = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    draw_count: u32,
    p_index_info: *const MultiDrawIndexedInfoEXT,
    instance_count: u32,
    first_instance: u32,
    stride: u32,
    p_vertex_offset: *const i32,
);
