use crate::native::vulkan1_0::{BaseOutStructure, Bool32, CommandBuffer, Format, StructureType, VertexInputRate};
#[doc(alias = "VkPhysicalDeviceVertexInputDynamicStateFeaturesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceVertexInputDynamicStateFeaturesEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "vertexInputDynamicState")]
    pub vertex_input_dynamic_state: Bool32,
}
impl Default for PhysicalDeviceVertexInputDynamicStateFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceVertexInputDynamicStateFeaturesExt,
            p_next: unsafe { std::mem::zeroed() },
            vertex_input_dynamic_state: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkVertexInputBindingDescription2EXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VertexInputBindingDescription2EXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    pub binding: u32,
    pub stride: u32,
    #[doc(alias = "inputRate")]
    pub input_rate: VertexInputRate,
    pub divisor: u32,
}
impl Default for VertexInputBindingDescription2EXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::VertexInputBindingDescription2Ext,
            p_next: unsafe { std::mem::zeroed() },
            binding: unsafe { std::mem::zeroed() },
            stride: unsafe { std::mem::zeroed() },
            input_rate: unsafe { std::mem::zeroed() },
            divisor: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkVertexInputAttributeDescription2EXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VertexInputAttributeDescription2EXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    pub location: u32,
    pub binding: u32,
    pub format: Format,
    pub offset: u32,
}
impl Default for VertexInputAttributeDescription2EXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::VertexInputAttributeDescription2Ext,
            p_next: unsafe { std::mem::zeroed() },
            location: unsafe { std::mem::zeroed() },
            binding: unsafe { std::mem::zeroed() },
            format: unsafe { std::mem::zeroed() },
            offset: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::ext_vertex_input_dynamic_state::{
    EXT_VERTEX_INPUT_DYNAMIC_STATE_EXTENSION_NAME, EXT_VERTEX_INPUT_DYNAMIC_STATE_SPEC_VERSION,
};
#[doc(alias = "vkCmdSetVertexInputEXT")]
pub type FNCmdSetVertexInputExt = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    vertex_binding_description_count: u32,
    p_vertex_binding_descriptions: *const VertexInputBindingDescription2EXT,
    vertex_attribute_description_count: u32,
    p_vertex_attribute_descriptions: *const VertexInputAttributeDescription2EXT,
);
