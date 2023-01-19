//!# [VK_AMD_rasterization_order](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_AMD_rasterization_order.html)
# ! [doc = include_str ! ("../../../../doc/extensions/amd_rasterization_order/VK_AMD_rasterization_order.md")]
use crate::{
    cstr,
    vulkan1_0::{BaseInStructure, StructureType},
};
use std::ffi::CStr;
///# [VkPipelineRasterizationStateRasterizationOrderAMD](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineRasterizationStateRasterizationOrderAMD.html)
# [doc = include_str ! ("../../../../doc/extensions/amd_rasterization_order/VkPipelineRasterizationStateRasterizationOrderAMD.md")]
#[doc(alias = "VkPipelineRasterizationStateRasterizationOrderAMD")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PipelineRasterizationStateRasterizationOrderAMD {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "rasterizationOrder")]
    rasterization_order: RasterizationOrderAMD,
}
#[doc(alias = "VK_AMD_RASTERIZATION_ORDER_SPEC_VERSION")]
pub const AMD_RASTERIZATION_ORDER_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_AMD_RASTERIZATION_ORDER_EXTENSION_NAME")]
pub const AMD_RASTERIZATION_ORDER_EXTENSION_NAME: &'static CStr = cstr!("VK_AMD_rasterization_order");
///# [VkRasterizationOrderAMD](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRasterizationOrderAMD.html)
# [doc = include_str ! ("../../../../doc/extensions/amd_rasterization_order/VkRasterizationOrderAMD.md")]
#[doc(alias = "VkRasterizationOrderAMD")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct RasterizationOrderAMD(i32);
impl RasterizationOrderAMD {
    #[doc(alias = "VK_RASTERIZATION_ORDER_STRICT_AMD")]
    pub const STRICT: Self = Self(0);
    #[doc(alias = "VK_RASTERIZATION_ORDER_RELAXED_AMD")]
    pub const RELAXED: Self = Self(1);
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
            x if x == Self::STRICT.bits() => Some(Self(x)),
            x if x == Self::RELAXED.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: i32) -> Self {
        Self(bits)
    }
}
