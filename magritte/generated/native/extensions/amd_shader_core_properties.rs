use crate::native::vulkan1_0::{BaseOutStructure, StructureType};
#[doc(alias = "VkPhysicalDeviceShaderCorePropertiesAMD")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShaderCorePropertiesAMD {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "shaderEngineCount")]
    pub shader_engine_count: u32,
    #[doc(alias = "shaderArraysPerEngineCount")]
    pub shader_arrays_per_engine_count: u32,
    #[doc(alias = "computeUnitsPerShaderArray")]
    pub compute_units_per_shader_array: u32,
    #[doc(alias = "simdPerComputeUnit")]
    pub simd_per_compute_unit: u32,
    #[doc(alias = "wavefrontsPerSimd")]
    pub wavefronts_per_simd: u32,
    #[doc(alias = "wavefrontSize")]
    pub wavefront_size: u32,
    #[doc(alias = "sgprsPerSimd")]
    pub sgprs_per_simd: u32,
    #[doc(alias = "minSgprAllocation")]
    pub min_sgpr_allocation: u32,
    #[doc(alias = "maxSgprAllocation")]
    pub max_sgpr_allocation: u32,
    #[doc(alias = "sgprAllocationGranularity")]
    pub sgpr_allocation_granularity: u32,
    #[doc(alias = "vgprsPerSimd")]
    pub vgprs_per_simd: u32,
    #[doc(alias = "minVgprAllocation")]
    pub min_vgpr_allocation: u32,
    #[doc(alias = "maxVgprAllocation")]
    pub max_vgpr_allocation: u32,
    #[doc(alias = "vgprAllocationGranularity")]
    pub vgpr_allocation_granularity: u32,
}
impl Default for PhysicalDeviceShaderCorePropertiesAMD {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceShaderCorePropertiesAmd,
            p_next: unsafe { std::mem::zeroed() },
            shader_engine_count: unsafe { std::mem::zeroed() },
            shader_arrays_per_engine_count: unsafe { std::mem::zeroed() },
            compute_units_per_shader_array: unsafe { std::mem::zeroed() },
            simd_per_compute_unit: unsafe { std::mem::zeroed() },
            wavefronts_per_simd: unsafe { std::mem::zeroed() },
            wavefront_size: unsafe { std::mem::zeroed() },
            sgprs_per_simd: unsafe { std::mem::zeroed() },
            min_sgpr_allocation: unsafe { std::mem::zeroed() },
            max_sgpr_allocation: unsafe { std::mem::zeroed() },
            sgpr_allocation_granularity: unsafe { std::mem::zeroed() },
            vgprs_per_simd: unsafe { std::mem::zeroed() },
            min_vgpr_allocation: unsafe { std::mem::zeroed() },
            max_vgpr_allocation: unsafe { std::mem::zeroed() },
            vgpr_allocation_granularity: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::amd_shader_core_properties::{
    AMD_SHADER_CORE_PROPERTIES_EXTENSION_NAME, AMD_SHADER_CORE_PROPERTIES_SPEC_VERSION,
};
