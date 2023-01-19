//!# [VK_NV_framebuffer_mixed_samples](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_NV_framebuffer_mixed_samples.html)
# ! [doc = include_str ! ("../../../../doc/extensions/nv_framebuffer_mixed_samples/VK_NV_framebuffer_mixed_samples.md")]
use crate::{
    cstr,
    vulkan1_0::{BaseInStructure, Bool32, StructureType},
};
use std::ffi::CStr;
///# [VkPipelineCoverageModulationStateCreateInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineCoverageModulationStateCreateInfoNV.html)
# [doc = include_str ! ("../../../../doc/extensions/nv_framebuffer_mixed_samples/VkPipelineCoverageModulationStateCreateInfoNV.md")]
#[doc(alias = "VkPipelineCoverageModulationStateCreateInfoNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PipelineCoverageModulationStateCreateInfoNV {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    flags: PipelineCoverageModulationStateCreateFlagsNV,
    #[doc(alias = "coverageModulationMode")]
    coverage_modulation_mode: CoverageModulationModeNV,
    #[doc(alias = "coverageModulationTableEnable")]
    coverage_modulation_table_enable: Bool32,
    #[doc(alias = "coverageModulationTableCount")]
    coverage_modulation_table_count: u32,
    #[doc(alias = "pCoverageModulationTable")]
    coverage_modulation_table: *const f32,
}
#[doc(alias = "VkPipelineCoverageModulationStateCreateFlagsNV")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PipelineCoverageModulationStateCreateFlagsNV(u32);
impl PipelineCoverageModulationStateCreateFlagsNV {
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
}
#[doc(alias = "VK_NV_FRAMEBUFFER_MIXED_SAMPLES_SPEC_VERSION")]
pub const NV_FRAMEBUFFER_MIXED_SAMPLES_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_NV_FRAMEBUFFER_MIXED_SAMPLES_EXTENSION_NAME")]
pub const NV_FRAMEBUFFER_MIXED_SAMPLES_EXTENSION_NAME: &'static CStr = cstr!("VK_NV_framebuffer_mixed_samples");
///# [VkCoverageModulationModeNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCoverageModulationModeNV.html)
# [doc = include_str ! ("../../../../doc/extensions/nv_framebuffer_mixed_samples/VkCoverageModulationModeNV.md")]
#[doc(alias = "VkCoverageModulationModeNV")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct CoverageModulationModeNV(i32);
impl CoverageModulationModeNV {
    #[doc(alias = "VK_COVERAGE_MODULATION_MODE_NONE_NV")]
    pub const NONE: Self = Self(0);
    #[doc(alias = "VK_COVERAGE_MODULATION_MODE_RGB_NV")]
    pub const RGB: Self = Self(1);
    #[doc(alias = "VK_COVERAGE_MODULATION_MODE_ALPHA_NV")]
    pub const ALPHA: Self = Self(2);
    #[doc(alias = "VK_COVERAGE_MODULATION_MODE_RGBA_NV")]
    pub const RGBA: Self = Self(3);
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
            x if x == Self::NONE.bits() => Some(Self(x)),
            x if x == Self::RGB.bits() => Some(Self(x)),
            x if x == Self::ALPHA.bits() => Some(Self(x)),
            x if x == Self::RGBA.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: i32) -> Self {
        Self(bits)
    }
}
