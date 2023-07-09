use crate::native::vulkan1_0::{BaseInStructure, BaseOutStructure, CommandBuffer, Rect2D, StructureType};
#[doc(alias = "VkPhysicalDeviceDiscardRectanglePropertiesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceDiscardRectanglePropertiesEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "maxDiscardRectangles")]
    pub max_discard_rectangles: u32,
}
impl Default for PhysicalDeviceDiscardRectanglePropertiesEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceDiscardRectanglePropertiesExt,
            p_next: unsafe { std::mem::zeroed() },
            max_discard_rectangles: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPipelineDiscardRectangleStateCreateInfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PipelineDiscardRectangleStateCreateInfoEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub flags: PipelineDiscardRectangleStateCreateFlagsEXT,
    #[doc(alias = "discardRectangleMode")]
    pub discard_rectangle_mode: DiscardRectangleModeEXT,
    #[doc(alias = "discardRectangleCount")]
    pub discard_rectangle_count: u32,
    #[doc(alias = "pDiscardRectangles")]
    pub discard_rectangles: *const Rect2D,
}
impl Default for PipelineDiscardRectangleStateCreateInfoEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::PipelineDiscardRectangleStateCreateInfoExt,
            p_next: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
            discard_rectangle_mode: unsafe { std::mem::zeroed() },
            discard_rectangle_count: unsafe { std::mem::zeroed() },
            discard_rectangles: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::ext_discard_rectangles::{
    DiscardRectangleModeEXT, PipelineDiscardRectangleStateCreateFlagsEXT, EXT_DISCARD_RECTANGLES_EXTENSION_NAME,
    EXT_DISCARD_RECTANGLES_SPEC_VERSION,
};
#[doc(alias = "vkCmdSetDiscardRectangleEXT")]
pub type FNCmdSetDiscardRectangleExt = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    first_discard_rectangle: u32,
    discard_rectangle_count: u32,
    p_discard_rectangles: *const Rect2D,
);
