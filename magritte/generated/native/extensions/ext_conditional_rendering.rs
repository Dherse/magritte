use crate::native::vulkan1_0::{
    BaseInStructure, BaseOutStructure, Bool32, Buffer, CommandBuffer, DeviceSize, StructureType,
};
#[doc(alias = "VkConditionalRenderingBeginInfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ConditionalRenderingBeginInfoEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub buffer: Buffer,
    pub offset: DeviceSize,
    pub flags: ConditionalRenderingFlagsEXT,
}
impl Default for ConditionalRenderingBeginInfoEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::ConditionalRenderingBeginInfoExt,
            p_next: unsafe { std::mem::zeroed() },
            buffer: unsafe { std::mem::zeroed() },
            offset: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkCommandBufferInheritanceConditionalRenderingInfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct CommandBufferInheritanceConditionalRenderingInfoEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "conditionalRenderingEnable")]
    pub conditional_rendering_enable: Bool32,
}
impl Default for CommandBufferInheritanceConditionalRenderingInfoEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::CommandBufferInheritanceConditionalRenderingInfoExt,
            p_next: unsafe { std::mem::zeroed() },
            conditional_rendering_enable: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceConditionalRenderingFeaturesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceConditionalRenderingFeaturesEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "conditionalRendering")]
    pub conditional_rendering: Bool32,
    #[doc(alias = "inheritedConditionalRendering")]
    pub inherited_conditional_rendering: Bool32,
}
impl Default for PhysicalDeviceConditionalRenderingFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceConditionalRenderingFeaturesExt,
            p_next: unsafe { std::mem::zeroed() },
            conditional_rendering: unsafe { std::mem::zeroed() },
            inherited_conditional_rendering: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::ext_conditional_rendering::{
    ConditionalRenderingFlagBitsEXT, ConditionalRenderingFlagsEXT, EXT_CONDITIONAL_RENDERING_EXTENSION_NAME,
    EXT_CONDITIONAL_RENDERING_SPEC_VERSION,
};
#[doc(alias = "vkCmdBeginConditionalRenderingEXT")]
pub type FNCmdBeginConditionalRenderingExt = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_conditional_rendering_begin: *const ConditionalRenderingBeginInfoEXT,
);
#[doc(alias = "vkCmdEndConditionalRenderingEXT")]
pub type FNCmdEndConditionalRenderingExt = unsafe extern "system" fn(command_buffer: CommandBuffer);
