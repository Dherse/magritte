use crate::{
    cstr,
    vulkan1_0::{BaseOutStructure, Bool32, CommandBuffer, Format, StructureType, VertexInputRate},
};
use std::ffi::CStr;
#[doc(alias = "VkPhysicalDeviceVertexInputDynamicStateFeaturesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceVertexInputDynamicStateFeaturesEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "vertexInputDynamicState")]
    vertex_input_dynamic_state: Bool32,
}
#[doc(alias = "VkVertexInputBindingDescription2EXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VertexInputBindingDescription2EXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    binding: u32,
    stride: u32,
    #[doc(alias = "inputRate")]
    input_rate: VertexInputRate,
    divisor: u32,
}
#[doc(alias = "VkVertexInputAttributeDescription2EXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VertexInputAttributeDescription2EXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    location: u32,
    binding: u32,
    format: Format,
    offset: u32,
}
#[doc(alias = "VK_EXT_VERTEX_INPUT_DYNAMIC_STATE_SPEC_VERSION")]
pub const EXT_VERTEX_INPUT_DYNAMIC_STATE_SPEC_VERSION: u32 = 2;
#[doc(alias = "VK_EXT_VERTEX_INPUT_DYNAMIC_STATE_EXTENSION_NAME")]
pub const EXT_VERTEX_INPUT_DYNAMIC_STATE_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_vertex_input_dynamic_state");
#[doc(alias = "vkCmdSetVertexInputEXT")]
pub type FNCmdSetVertexInputExt = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    vertex_binding_description_count: u32,
    p_vertex_binding_descriptions: *const VertexInputBindingDescription2EXT,
    vertex_attribute_description_count: u32,
    p_vertex_attribute_descriptions: *const VertexInputAttributeDescription2EXT,
);
