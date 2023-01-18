use crate::{
    cstr,
    vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, CommandBuffer, StructureType},
};
use std::ffi::CStr;
#[doc(alias = "VkPhysicalDeviceLineRasterizationFeaturesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceLineRasterizationFeaturesEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "rectangularLines")]
    rectangular_lines: Bool32,
    #[doc(alias = "bresenhamLines")]
    bresenham_lines: Bool32,
    #[doc(alias = "smoothLines")]
    smooth_lines: Bool32,
    #[doc(alias = "stippledRectangularLines")]
    stippled_rectangular_lines: Bool32,
    #[doc(alias = "stippledBresenhamLines")]
    stippled_bresenham_lines: Bool32,
    #[doc(alias = "stippledSmoothLines")]
    stippled_smooth_lines: Bool32,
}
#[doc(alias = "VkPhysicalDeviceLineRasterizationPropertiesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceLineRasterizationPropertiesEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "lineSubPixelPrecisionBits")]
    line_sub_pixel_precision_bits: u32,
}
#[doc(alias = "VkPipelineRasterizationLineStateCreateInfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PipelineRasterizationLineStateCreateInfoEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "lineRasterizationMode")]
    line_rasterization_mode: LineRasterizationModeEXT,
    #[doc(alias = "stippledLineEnable")]
    stippled_line_enable: Bool32,
    #[doc(alias = "lineStippleFactor")]
    line_stipple_factor: u32,
    #[doc(alias = "lineStipplePattern")]
    line_stipple_pattern: u16,
}
#[doc(alias = "VK_EXT_LINE_RASTERIZATION_SPEC_VERSION")]
pub const EXT_LINE_RASTERIZATION_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_EXT_LINE_RASTERIZATION_EXTENSION_NAME")]
pub const EXT_LINE_RASTERIZATION_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_line_rasterization");
#[doc(alias = "VkLineRasterizationModeEXT")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct LineRasterizationModeEXT(i32);
impl LineRasterizationModeEXT {
    #[doc(alias = "VK_LINE_RASTERIZATION_MODE_DEFAULT_EXT")]
    pub const DEFAULT: Self = Self(0);
    #[doc(alias = "VK_LINE_RASTERIZATION_MODE_RECTANGULAR_EXT")]
    pub const RECTANGULAR: Self = Self(1);
    #[doc(alias = "VK_LINE_RASTERIZATION_MODE_BRESENHAM_EXT")]
    pub const BRESENHAM: Self = Self(2);
    #[doc(alias = "VK_LINE_RASTERIZATION_MODE_RECTANGULAR_SMOOTH_EXT")]
    pub const RECTANGULAR_SMOOTH: Self = Self(3);
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
    ///The bits of this variant
    #[inline]
    pub const fn bits(&self) -> i32 {
        self.0
    }
    ///Builds a bitmask from the bits of this variant
    #[inline]
    pub const fn from_bits(bits: i32) -> Option<Self> {
        match bits {
            x if x == Self::DEFAULT.bits() => Some(Self(x)),
            x if x == Self::RECTANGULAR.bits() => Some(Self(x)),
            x if x == Self::BRESENHAM.bits() => Some(Self(x)),
            x if x == Self::RECTANGULAR_SMOOTH.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: i32) -> Self {
        Self(bits)
    }
}
#[doc(alias = "vkCmdSetLineStippleEXT")]
pub type FNCmdSetLineStippleExt =
    unsafe extern "system" fn(command_buffer: CommandBuffer, line_stipple_factor: u32, line_stipple_pattern: u16);
