use crate::{
    cstr,
    vulkan1_0::{Device, Pipeline, ShaderStageFlagBits, ShaderStageFlags, VulkanResultCodes},
};
use std::ffi::CStr;
#[doc(alias = "VkShaderResourceUsageAMD")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ShaderResourceUsageAMD {
    #[doc(alias = "numUsedVgprs")]
    num_used_vgprs: u32,
    #[doc(alias = "numUsedSgprs")]
    num_used_sgprs: u32,
    #[doc(alias = "ldsSizePerLocalWorkGroup")]
    lds_size_per_local_work_group: u32,
    #[doc(alias = "ldsUsageSizeInBytes")]
    lds_usage_size_in_bytes: usize,
    #[doc(alias = "scratchMemUsageInBytes")]
    scratch_mem_usage_in_bytes: usize,
}
#[doc(alias = "VkShaderStatisticsInfoAMD")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ShaderStatisticsInfoAMD {
    #[doc(alias = "shaderStageMask")]
    shader_stage_mask: ShaderStageFlags,
    #[doc(alias = "resourceUsage")]
    resource_usage: ShaderResourceUsageAMD,
    #[doc(alias = "numPhysicalVgprs")]
    num_physical_vgprs: u32,
    #[doc(alias = "numPhysicalSgprs")]
    num_physical_sgprs: u32,
    #[doc(alias = "numAvailableVgprs")]
    num_available_vgprs: u32,
    #[doc(alias = "numAvailableSgprs")]
    num_available_sgprs: u32,
    #[doc(alias = "computeWorkGroupSize")]
    compute_work_group_size: [u32; 3 as usize],
}
#[doc(alias = "VK_AMD_SHADER_INFO_SPEC_VERSION")]
pub const AMD_SHADER_INFO_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_AMD_SHADER_INFO_EXTENSION_NAME")]
pub const AMD_SHADER_INFO_EXTENSION_NAME: &'static CStr = cstr!("VK_AMD_shader_info");
#[doc(alias = "VkShaderInfoTypeAMD")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct ShaderInfoTypeAMD(i32);
impl ShaderInfoTypeAMD {
    #[doc(alias = "VK_SHADER_INFO_TYPE_STATISTICS_AMD")]
    pub const STATISTICS: Self = Self(0);
    #[doc(alias = "VK_SHADER_INFO_TYPE_BINARY_AMD")]
    pub const BINARY: Self = Self(1);
    #[doc(alias = "VK_SHADER_INFO_TYPE_DISASSEMBLY_AMD")]
    pub const DISASSEMBLY: Self = Self(2);
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
            x if x == Self::STATISTICS.bits() => Some(Self(x)),
            x if x == Self::BINARY.bits() => Some(Self(x)),
            x if x == Self::DISASSEMBLY.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: i32) -> Self {
        Self(bits)
    }
}
#[doc(alias = "vkGetShaderInfoAMD")]
pub type FNGetShaderInfoAmd = unsafe extern "system" fn(
    device: Device,
    pipeline: Pipeline,
    shader_stage: ShaderStageFlagBits,
    info_type: ShaderInfoTypeAMD,
    p_info_size: *mut usize,
    p_info: *mut std::ffi::c_void,
) -> VulkanResultCodes;
