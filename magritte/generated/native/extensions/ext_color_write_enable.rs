use crate::native::vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, CommandBuffer, StructureType};
#[doc(alias = "VkPhysicalDeviceColorWriteEnableFeaturesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceColorWriteEnableFeaturesEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "colorWriteEnable")]
    pub color_write_enable: Bool32,
}
impl Default for PhysicalDeviceColorWriteEnableFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceColorWriteEnableFeaturesExt,
            p_next: unsafe { std::mem::zeroed() },
            color_write_enable: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPipelineColorWriteCreateInfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PipelineColorWriteCreateInfoEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "attachmentCount")]
    pub attachment_count: u32,
    #[doc(alias = "pColorWriteEnables")]
    pub color_write_enables: *const Bool32,
}
impl Default for PipelineColorWriteCreateInfoEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::PipelineColorWriteCreateInfoExt,
            p_next: unsafe { std::mem::zeroed() },
            attachment_count: unsafe { std::mem::zeroed() },
            color_write_enables: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::ext_color_write_enable::{
    EXT_COLOR_WRITE_ENABLE_EXTENSION_NAME, EXT_COLOR_WRITE_ENABLE_SPEC_VERSION,
};
#[doc(alias = "vkCmdSetColorWriteEnableEXT")]
pub type FNCmdSetColorWriteEnableExt = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    attachment_count: u32,
    p_color_write_enables: *const Bool32,
);
