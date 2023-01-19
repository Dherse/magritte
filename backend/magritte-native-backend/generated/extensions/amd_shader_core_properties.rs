//!# [VK_AMD_shader_core_properties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_AMD_shader_core_properties.html)
# ! [doc = include_str ! ("../../../../doc/extensions/amd_shader_core_properties/VK_AMD_shader_core_properties.md")]
use crate::{
    cstr,
    vulkan1_0::{BaseOutStructure, StructureType},
};
use std::ffi::CStr;
///# [VkPhysicalDeviceShaderCorePropertiesAMD](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderCorePropertiesAMD.html)
# [doc = include_str ! ("../../../../doc/extensions/amd_shader_core_properties/VkPhysicalDeviceShaderCorePropertiesAMD.md")]
#[doc(alias = "VkPhysicalDeviceShaderCorePropertiesAMD")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShaderCorePropertiesAMD {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "shaderEngineCount")]
    shader_engine_count: u32,
    #[doc(alias = "shaderArraysPerEngineCount")]
    shader_arrays_per_engine_count: u32,
    #[doc(alias = "computeUnitsPerShaderArray")]
    compute_units_per_shader_array: u32,
    #[doc(alias = "simdPerComputeUnit")]
    simd_per_compute_unit: u32,
    #[doc(alias = "wavefrontsPerSimd")]
    wavefronts_per_simd: u32,
    #[doc(alias = "wavefrontSize")]
    wavefront_size: u32,
    #[doc(alias = "sgprsPerSimd")]
    sgprs_per_simd: u32,
    #[doc(alias = "minSgprAllocation")]
    min_sgpr_allocation: u32,
    #[doc(alias = "maxSgprAllocation")]
    max_sgpr_allocation: u32,
    #[doc(alias = "sgprAllocationGranularity")]
    sgpr_allocation_granularity: u32,
    #[doc(alias = "vgprsPerSimd")]
    vgprs_per_simd: u32,
    #[doc(alias = "minVgprAllocation")]
    min_vgpr_allocation: u32,
    #[doc(alias = "maxVgprAllocation")]
    max_vgpr_allocation: u32,
    #[doc(alias = "vgprAllocationGranularity")]
    vgpr_allocation_granularity: u32,
}
#[doc(alias = "VK_AMD_SHADER_CORE_PROPERTIES_SPEC_VERSION")]
pub const AMD_SHADER_CORE_PROPERTIES_SPEC_VERSION: u32 = 2;
#[doc(alias = "VK_AMD_SHADER_CORE_PROPERTIES_EXTENSION_NAME")]
pub const AMD_SHADER_CORE_PROPERTIES_EXTENSION_NAME: &'static CStr = cstr!("VK_AMD_shader_core_properties");
