use crate::{
    cstr,
    vulkan1_0::{
        BaseInStructure, BaseOutStructure, Bool32, PhysicalDevice, SampleCountFlagBits, SampleCountFlags,
        StructureType, VulkanResultCodes,
    },
};
use std::ffi::CStr;
#[doc(alias = "VkPhysicalDeviceCoverageReductionModeFeaturesNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceCoverageReductionModeFeaturesNV {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "coverageReductionMode")]
    coverage_reduction_mode: Bool32,
}
#[doc(alias = "VkPipelineCoverageReductionStateCreateInfoNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PipelineCoverageReductionStateCreateInfoNV {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    flags: PipelineCoverageReductionStateCreateFlagsNV,
    #[doc(alias = "coverageReductionMode")]
    coverage_reduction_mode: CoverageReductionModeNV,
}
#[doc(alias = "VkFramebufferMixedSamplesCombinationNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct FramebufferMixedSamplesCombinationNV {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "coverageReductionMode")]
    coverage_reduction_mode: CoverageReductionModeNV,
    #[doc(alias = "rasterizationSamples")]
    rasterization_samples: SampleCountFlagBits,
    #[doc(alias = "depthStencilSamples")]
    depth_stencil_samples: SampleCountFlags,
    #[doc(alias = "colorSamples")]
    color_samples: SampleCountFlags,
}
#[doc(alias = "VkPipelineCoverageReductionStateCreateFlagsNV")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PipelineCoverageReductionStateCreateFlagsNV(u32);
impl PipelineCoverageReductionStateCreateFlagsNV {
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
}
#[doc(alias = "VK_NV_COVERAGE_REDUCTION_MODE_SPEC_VERSION")]
pub const NV_COVERAGE_REDUCTION_MODE_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_NV_COVERAGE_REDUCTION_MODE_EXTENSION_NAME")]
pub const NV_COVERAGE_REDUCTION_MODE_EXTENSION_NAME: &'static CStr = cstr!("VK_NV_coverage_reduction_mode");
#[doc(alias = "VkCoverageReductionModeNV")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct CoverageReductionModeNV(i32);
impl CoverageReductionModeNV {
    #[doc(alias = "VK_COVERAGE_REDUCTION_MODE_MERGE_NV")]
    pub const MERGE: Self = Self(0);
    #[doc(alias = "VK_COVERAGE_REDUCTION_MODE_TRUNCATE_NV")]
    pub const TRUNCATE: Self = Self(1);
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
            x if x == Self::MERGE.bits() => Some(Self(x)),
            x if x == Self::TRUNCATE.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: i32) -> Self {
        Self(bits)
    }
}
#[doc(alias = "vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV")]
pub type FNGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNv =
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_combination_count: *mut u32,
        p_combinations: *mut FramebufferMixedSamplesCombinationNV,
    ) -> VulkanResultCodes;
