use crate::native::{
    extensions::khr_fragment_shading_rate::FragmentShadingRateCombinerOpKHR,
    vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, CommandBuffer, SampleCountFlagBits, StructureType},
};
#[doc(alias = "VkPhysicalDeviceFragmentShadingRateEnumsFeaturesNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceFragmentShadingRateEnumsFeaturesNV {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "fragmentShadingRateEnums")]
    pub fragment_shading_rate_enums: Bool32,
    #[doc(alias = "supersampleFragmentShadingRates")]
    pub supersample_fragment_shading_rates: Bool32,
    #[doc(alias = "noInvocationFragmentShadingRates")]
    pub no_invocation_fragment_shading_rates: Bool32,
}
impl Default for PhysicalDeviceFragmentShadingRateEnumsFeaturesNV {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceFragmentShadingRateEnumsFeaturesNv,
            p_next: unsafe { std::mem::zeroed() },
            fragment_shading_rate_enums: unsafe { std::mem::zeroed() },
            supersample_fragment_shading_rates: unsafe { std::mem::zeroed() },
            no_invocation_fragment_shading_rates: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceFragmentShadingRateEnumsPropertiesNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceFragmentShadingRateEnumsPropertiesNV {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "maxFragmentShadingRateInvocationCount")]
    pub max_fragment_shading_rate_invocation_count: SampleCountFlagBits,
}
impl Default for PhysicalDeviceFragmentShadingRateEnumsPropertiesNV {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceFragmentShadingRateEnumsPropertiesNv,
            p_next: unsafe { std::mem::zeroed() },
            max_fragment_shading_rate_invocation_count: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPipelineFragmentShadingRateEnumStateCreateInfoNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PipelineFragmentShadingRateEnumStateCreateInfoNV {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "shadingRateType")]
    pub shading_rate_type: FragmentShadingRateTypeNV,
    #[doc(alias = "shadingRate")]
    pub shading_rate: FragmentShadingRateNV,
    #[doc(alias = "combinerOps")]
    pub combiner_ops: [FragmentShadingRateCombinerOpKHR; 2 as usize],
}
impl Default for PipelineFragmentShadingRateEnumStateCreateInfoNV {
    fn default() -> Self {
        Self {
            s_type: StructureType::PipelineFragmentShadingRateEnumStateCreateInfoNv,
            p_next: unsafe { std::mem::zeroed() },
            shading_rate_type: unsafe { std::mem::zeroed() },
            shading_rate: unsafe { std::mem::zeroed() },
            combiner_ops: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::nv_fragment_shading_rate_enums::{
    FragmentShadingRateNV, FragmentShadingRateTypeNV, NV_FRAGMENT_SHADING_RATE_ENUMS_EXTENSION_NAME,
    NV_FRAGMENT_SHADING_RATE_ENUMS_SPEC_VERSION,
};
#[doc(alias = "vkCmdSetFragmentShadingRateEnumNV")]
pub type FNCmdSetFragmentShadingRateEnumNv = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    shading_rate: FragmentShadingRateNV,
    combiner_ops: [FragmentShadingRateCombinerOpKHR; 2 as usize],
);
