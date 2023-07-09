use crate::native::vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, CommandBuffer, StructureType};
#[doc(alias = "VkPhysicalDeviceLineRasterizationFeaturesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceLineRasterizationFeaturesEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "rectangularLines")]
    pub rectangular_lines: Bool32,
    #[doc(alias = "bresenhamLines")]
    pub bresenham_lines: Bool32,
    #[doc(alias = "smoothLines")]
    pub smooth_lines: Bool32,
    #[doc(alias = "stippledRectangularLines")]
    pub stippled_rectangular_lines: Bool32,
    #[doc(alias = "stippledBresenhamLines")]
    pub stippled_bresenham_lines: Bool32,
    #[doc(alias = "stippledSmoothLines")]
    pub stippled_smooth_lines: Bool32,
}
impl Default for PhysicalDeviceLineRasterizationFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceLineRasterizationFeaturesExt,
            p_next: unsafe { std::mem::zeroed() },
            rectangular_lines: unsafe { std::mem::zeroed() },
            bresenham_lines: unsafe { std::mem::zeroed() },
            smooth_lines: unsafe { std::mem::zeroed() },
            stippled_rectangular_lines: unsafe { std::mem::zeroed() },
            stippled_bresenham_lines: unsafe { std::mem::zeroed() },
            stippled_smooth_lines: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceLineRasterizationPropertiesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceLineRasterizationPropertiesEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "lineSubPixelPrecisionBits")]
    pub line_sub_pixel_precision_bits: u32,
}
impl Default for PhysicalDeviceLineRasterizationPropertiesEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceLineRasterizationPropertiesExt,
            p_next: unsafe { std::mem::zeroed() },
            line_sub_pixel_precision_bits: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPipelineRasterizationLineStateCreateInfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PipelineRasterizationLineStateCreateInfoEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "lineRasterizationMode")]
    pub line_rasterization_mode: LineRasterizationModeEXT,
    #[doc(alias = "stippledLineEnable")]
    pub stippled_line_enable: Bool32,
    #[doc(alias = "lineStippleFactor")]
    pub line_stipple_factor: u32,
    #[doc(alias = "lineStipplePattern")]
    pub line_stipple_pattern: u16,
}
impl Default for PipelineRasterizationLineStateCreateInfoEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::PipelineRasterizationLineStateCreateInfoExt,
            p_next: unsafe { std::mem::zeroed() },
            line_rasterization_mode: unsafe { std::mem::zeroed() },
            stippled_line_enable: unsafe { std::mem::zeroed() },
            line_stipple_factor: unsafe { std::mem::zeroed() },
            line_stipple_pattern: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::ext_line_rasterization::{
    LineRasterizationModeEXT, EXT_LINE_RASTERIZATION_EXTENSION_NAME, EXT_LINE_RASTERIZATION_SPEC_VERSION,
};
#[doc(alias = "vkCmdSetLineStippleEXT")]
pub type FNCmdSetLineStippleExt =
    unsafe extern "system" fn(command_buffer: CommandBuffer, line_stipple_factor: u32, line_stipple_pattern: u16);
