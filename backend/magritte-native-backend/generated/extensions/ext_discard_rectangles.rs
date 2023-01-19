//!# [VK_EXT_discard_rectangles](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_discard_rectangles.html)
# ! [doc = include_str ! ("../../../../doc/extensions/ext_discard_rectangles/VK_EXT_discard_rectangles.md")]
use crate::{
    cstr,
    vulkan1_0::{BaseInStructure, BaseOutStructure, CommandBuffer, Rect2D, StructureType},
};
use std::ffi::CStr;
///# [VkPhysicalDeviceDiscardRectanglePropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDiscardRectanglePropertiesEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_discard_rectangles/VkPhysicalDeviceDiscardRectanglePropertiesEXT.md")]
#[doc(alias = "VkPhysicalDeviceDiscardRectanglePropertiesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceDiscardRectanglePropertiesEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "maxDiscardRectangles")]
    max_discard_rectangles: u32,
}
///# [VkPipelineDiscardRectangleStateCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineDiscardRectangleStateCreateInfoEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_discard_rectangles/VkPipelineDiscardRectangleStateCreateInfoEXT.md")]
#[doc(alias = "VkPipelineDiscardRectangleStateCreateInfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PipelineDiscardRectangleStateCreateInfoEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    flags: PipelineDiscardRectangleStateCreateFlagsEXT,
    #[doc(alias = "discardRectangleMode")]
    discard_rectangle_mode: DiscardRectangleModeEXT,
    #[doc(alias = "discardRectangleCount")]
    discard_rectangle_count: u32,
    #[doc(alias = "pDiscardRectangles")]
    discard_rectangles: *const Rect2D,
}
#[doc(alias = "VkPipelineDiscardRectangleStateCreateFlagsEXT")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PipelineDiscardRectangleStateCreateFlagsEXT(u32);
impl PipelineDiscardRectangleStateCreateFlagsEXT {
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
}
#[doc(alias = "VK_EXT_DISCARD_RECTANGLES_SPEC_VERSION")]
pub const EXT_DISCARD_RECTANGLES_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_EXT_DISCARD_RECTANGLES_EXTENSION_NAME")]
pub const EXT_DISCARD_RECTANGLES_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_discard_rectangles");
///# [VkDiscardRectangleModeEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDiscardRectangleModeEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_discard_rectangles/VkDiscardRectangleModeEXT.md")]
#[doc(alias = "VkDiscardRectangleModeEXT")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct DiscardRectangleModeEXT(i32);
impl DiscardRectangleModeEXT {
    #[doc(alias = "VK_DISCARD_RECTANGLE_MODE_INCLUSIVE_EXT")]
    pub const INCLUSIVE: Self = Self(0);
    #[doc(alias = "VK_DISCARD_RECTANGLE_MODE_EXCLUSIVE_EXT")]
    pub const EXCLUSIVE: Self = Self(1);
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
            x if x == Self::INCLUSIVE.bits() => Some(Self(x)),
            x if x == Self::EXCLUSIVE.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: i32) -> Self {
        Self(bits)
    }
}
///# [vkCmdSetDiscardRectangleEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetDiscardRectangleEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_discard_rectangles/vkCmdSetDiscardRectangleEXT.md")]
#[doc(alias = "vkCmdSetDiscardRectangleEXT")]
pub type FNCmdSetDiscardRectangleExt = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    first_discard_rectangle: u32,
    discard_rectangle_count: u32,
    p_discard_rectangles: *const Rect2D,
);
