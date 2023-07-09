use crate::native::vulkan1_0::{
    BaseInStructure, BaseOutStructure, Bool32, PhysicalDevice, SampleCountFlagBits, SampleCountFlags, StructureType,
    VulkanResultCodes,
};
#[doc(alias = "VkPhysicalDeviceCoverageReductionModeFeaturesNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceCoverageReductionModeFeaturesNV {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "coverageReductionMode")]
    pub coverage_reduction_mode: Bool32,
}
impl Default for PhysicalDeviceCoverageReductionModeFeaturesNV {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceCoverageReductionModeFeaturesNv,
            p_next: unsafe { std::mem::zeroed() },
            coverage_reduction_mode: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPipelineCoverageReductionStateCreateInfoNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PipelineCoverageReductionStateCreateInfoNV {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub flags: PipelineCoverageReductionStateCreateFlagsNV,
    #[doc(alias = "coverageReductionMode")]
    pub coverage_reduction_mode: CoverageReductionModeNV,
}
impl Default for PipelineCoverageReductionStateCreateInfoNV {
    fn default() -> Self {
        Self {
            s_type: StructureType::PipelineCoverageReductionStateCreateInfoNv,
            p_next: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
            coverage_reduction_mode: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkFramebufferMixedSamplesCombinationNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct FramebufferMixedSamplesCombinationNV {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "coverageReductionMode")]
    pub coverage_reduction_mode: CoverageReductionModeNV,
    #[doc(alias = "rasterizationSamples")]
    pub rasterization_samples: SampleCountFlagBits,
    #[doc(alias = "depthStencilSamples")]
    pub depth_stencil_samples: SampleCountFlags,
    #[doc(alias = "colorSamples")]
    pub color_samples: SampleCountFlags,
}
impl Default for FramebufferMixedSamplesCombinationNV {
    fn default() -> Self {
        Self {
            s_type: StructureType::FramebufferMixedSamplesCombinationNv,
            p_next: unsafe { std::mem::zeroed() },
            coverage_reduction_mode: unsafe { std::mem::zeroed() },
            rasterization_samples: unsafe { std::mem::zeroed() },
            depth_stencil_samples: unsafe { std::mem::zeroed() },
            color_samples: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::nv_coverage_reduction_mode::{
    CoverageReductionModeNV, PipelineCoverageReductionStateCreateFlagsNV, NV_COVERAGE_REDUCTION_MODE_EXTENSION_NAME,
    NV_COVERAGE_REDUCTION_MODE_SPEC_VERSION,
};
#[doc(alias = "vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV")]
pub type FNGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNv =
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_combination_count: *mut u32,
        p_combinations: *mut FramebufferMixedSamplesCombinationNV,
    ) -> VulkanResultCodes;
