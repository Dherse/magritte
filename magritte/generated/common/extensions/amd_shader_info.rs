use crate::{common::vulkan1_0::ShaderStageFlags, cstr};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ffi::CStr;
#[doc(alias = "VkShaderResourceUsageAMD")]
#[derive(Clone, Copy, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct ShaderResourceUsageAMD {
    #[doc(alias = "numUsedVgprs")]
    pub num_used_vgprs: u32,
    #[doc(alias = "numUsedSgprs")]
    pub num_used_sgprs: u32,
    #[doc(alias = "ldsSizePerLocalWorkGroup")]
    pub lds_size_per_local_work_group: u32,
    #[doc(alias = "ldsUsageSizeInBytes")]
    pub lds_usage_size_in_bytes: usize,
    #[doc(alias = "scratchMemUsageInBytes")]
    pub scratch_mem_usage_in_bytes: usize,
}
#[doc(alias = "VkShaderStatisticsInfoAMD")]
#[derive(Clone, Copy, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct ShaderStatisticsInfoAMD {
    #[doc(alias = "shaderStageMask")]
    pub shader_stage_mask: ShaderStageFlags,
    #[doc(alias = "resourceUsage")]
    pub resource_usage: ShaderResourceUsageAMD,
    #[doc(alias = "numPhysicalVgprs")]
    pub num_physical_vgprs: u32,
    #[doc(alias = "numPhysicalSgprs")]
    pub num_physical_sgprs: u32,
    #[doc(alias = "numAvailableVgprs")]
    pub num_available_vgprs: u32,
    #[doc(alias = "numAvailableSgprs")]
    pub num_available_sgprs: u32,
    #[doc(alias = "computeWorkGroupSize")]
    pub compute_work_group_size: [u32; 3 as usize],
}
#[doc(alias = "VK_AMD_SHADER_INFO_SPEC_VERSION")]
pub const AMD_SHADER_INFO_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_AMD_SHADER_INFO_EXTENSION_NAME")]
pub const AMD_SHADER_INFO_EXTENSION_NAME: &'static CStr = cstr!("VK_AMD_shader_info");
#[doc(alias = "VkShaderInfoTypeAMD")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub enum ShaderInfoTypeAMD {
    #[doc(alias = "VK_SHADER_INFO_TYPE_STATISTICS_AMD")]
    Statistics = 0,
    #[doc(alias = "VK_SHADER_INFO_TYPE_BINARY_AMD")]
    Binary = 1,
    #[doc(alias = "VK_SHADER_INFO_TYPE_DISASSEMBLY_AMD")]
    Disassembly = 2,
}
impl Default for ShaderInfoTypeAMD {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
impl ShaderInfoTypeAMD {
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        unsafe { Self::from_bits_unchecked(0) }
    }
    ///The bits of this variant
    #[inline]
    pub const fn bits(&self) -> i32 {
        *self as i32
    }
    ///Builds a bitmask from the bits of this variant
    #[inline]
    pub const fn from_bits(bits: i32) -> Option<Self> {
        match bits {
            x if x == Self::Statistics.bits() => Some(Self::Statistics),
            x if x == Self::Binary.bits() => Some(Self::Binary),
            x if x == Self::Disassembly.bits() => Some(Self::Disassembly),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: i32) -> Self {
        std::mem::transmute(bits)
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for ShaderInfoTypeAMD {
    type LowLevel = Self;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        *self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for ShaderInfoTypeAMD {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
