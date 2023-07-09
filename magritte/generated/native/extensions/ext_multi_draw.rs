pub use crate::common::extensions::ext_multi_draw::{MultiDrawIndexedInfoEXT, MultiDrawInfoEXT};
use crate::native::vulkan1_0::{BaseOutStructure, Bool32, CommandBuffer, StructureType};
#[doc(alias = "VkPhysicalDeviceMultiDrawPropertiesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceMultiDrawPropertiesEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "maxMultiDrawCount")]
    pub max_multi_draw_count: u32,
}
impl Default for PhysicalDeviceMultiDrawPropertiesEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceMultiDrawPropertiesExt,
            p_next: unsafe { std::mem::zeroed() },
            max_multi_draw_count: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceMultiDrawFeaturesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceMultiDrawFeaturesEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "multiDraw")]
    pub multi_draw: Bool32,
}
impl Default for PhysicalDeviceMultiDrawFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceMultiDrawFeaturesExt,
            p_next: unsafe { std::mem::zeroed() },
            multi_draw: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::ext_multi_draw::{EXT_MULTI_DRAW_EXTENSION_NAME, EXT_MULTI_DRAW_SPEC_VERSION};
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
