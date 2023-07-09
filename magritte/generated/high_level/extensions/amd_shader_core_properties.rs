pub use crate::common::extensions::amd_shader_core_properties::{
    AMD_SHADER_CORE_PROPERTIES_EXTENSION_NAME, AMD_SHADER_CORE_PROPERTIES_SPEC_VERSION,
};
use crate::{context::Context, vulkan1_0::StructureType};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkPhysicalDeviceShaderCorePropertiesAMD")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceShaderCorePropertiesAMD {
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
impl PhysicalDeviceShaderCorePropertiesAMD {
    ///Get a reference to the `shader_engine_count` field.
    pub fn shader_engine_count(&self) -> u32 {
        self.shader_engine_count
    }
    ///Get a reference to the `shader_arrays_per_engine_count` field.
    pub fn shader_arrays_per_engine_count(&self) -> u32 {
        self.shader_arrays_per_engine_count
    }
    ///Get a reference to the `compute_units_per_shader_array` field.
    pub fn compute_units_per_shader_array(&self) -> u32 {
        self.compute_units_per_shader_array
    }
    ///Get a reference to the `simd_per_compute_unit` field.
    pub fn simd_per_compute_unit(&self) -> u32 {
        self.simd_per_compute_unit
    }
    ///Get a reference to the `wavefronts_per_simd` field.
    pub fn wavefronts_per_simd(&self) -> u32 {
        self.wavefronts_per_simd
    }
    ///Get a reference to the `wavefront_size` field.
    pub fn wavefront_size(&self) -> u32 {
        self.wavefront_size
    }
    ///Get a reference to the `sgprs_per_simd` field.
    pub fn sgprs_per_simd(&self) -> u32 {
        self.sgprs_per_simd
    }
    ///Get a reference to the `min_sgpr_allocation` field.
    pub fn min_sgpr_allocation(&self) -> u32 {
        self.min_sgpr_allocation
    }
    ///Get a reference to the `max_sgpr_allocation` field.
    pub fn max_sgpr_allocation(&self) -> u32 {
        self.max_sgpr_allocation
    }
    ///Get a reference to the `sgpr_allocation_granularity` field.
    pub fn sgpr_allocation_granularity(&self) -> u32 {
        self.sgpr_allocation_granularity
    }
    ///Get a reference to the `vgprs_per_simd` field.
    pub fn vgprs_per_simd(&self) -> u32 {
        self.vgprs_per_simd
    }
    ///Get a reference to the `min_vgpr_allocation` field.
    pub fn min_vgpr_allocation(&self) -> u32 {
        self.min_vgpr_allocation
    }
    ///Get a reference to the `max_vgpr_allocation` field.
    pub fn max_vgpr_allocation(&self) -> u32 {
        self.max_vgpr_allocation
    }
    ///Get a reference to the `vgpr_allocation_granularity` field.
    pub fn vgpr_allocation_granularity(&self) -> u32 {
        self.vgpr_allocation_granularity
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceShaderCorePropertiesAMD {
    type LowLevel = crate::native::extensions::amd_shader_core_properties::PhysicalDeviceShaderCorePropertiesAMD;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::amd_shader_core_properties::PhysicalDeviceShaderCorePropertiesAMD {
            s_type: StructureType::PhysicalDeviceShaderCorePropertiesAmd,
            p_next: std::ptr::null_mut(),
            shader_engine_count: self.shader_engine_count.into_low_level(context, bump),
            shader_arrays_per_engine_count: self.shader_arrays_per_engine_count.into_low_level(context, bump),
            compute_units_per_shader_array: self.compute_units_per_shader_array.into_low_level(context, bump),
            simd_per_compute_unit: self.simd_per_compute_unit.into_low_level(context, bump),
            wavefronts_per_simd: self.wavefronts_per_simd.into_low_level(context, bump),
            wavefront_size: self.wavefront_size.into_low_level(context, bump),
            sgprs_per_simd: self.sgprs_per_simd.into_low_level(context, bump),
            min_sgpr_allocation: self.min_sgpr_allocation.into_low_level(context, bump),
            max_sgpr_allocation: self.max_sgpr_allocation.into_low_level(context, bump),
            sgpr_allocation_granularity: self.sgpr_allocation_granularity.into_low_level(context, bump),
            vgprs_per_simd: self.vgprs_per_simd.into_low_level(context, bump),
            min_vgpr_allocation: self.min_vgpr_allocation.into_low_level(context, bump),
            max_vgpr_allocation: self.max_vgpr_allocation.into_low_level(context, bump),
            vgpr_allocation_granularity: self.vgpr_allocation_granularity.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceShaderCorePropertiesAMD {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            shader_engine_count: crate::conv::FromLowLevel::from_low_level(context, value.shader_engine_count),
            shader_arrays_per_engine_count: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_arrays_per_engine_count,
            ),
            compute_units_per_shader_array: crate::conv::FromLowLevel::from_low_level(
                context,
                value.compute_units_per_shader_array,
            ),
            simd_per_compute_unit: crate::conv::FromLowLevel::from_low_level(context, value.simd_per_compute_unit),
            wavefronts_per_simd: crate::conv::FromLowLevel::from_low_level(context, value.wavefronts_per_simd),
            wavefront_size: crate::conv::FromLowLevel::from_low_level(context, value.wavefront_size),
            sgprs_per_simd: crate::conv::FromLowLevel::from_low_level(context, value.sgprs_per_simd),
            min_sgpr_allocation: crate::conv::FromLowLevel::from_low_level(context, value.min_sgpr_allocation),
            max_sgpr_allocation: crate::conv::FromLowLevel::from_low_level(context, value.max_sgpr_allocation),
            sgpr_allocation_granularity: crate::conv::FromLowLevel::from_low_level(
                context,
                value.sgpr_allocation_granularity,
            ),
            vgprs_per_simd: crate::conv::FromLowLevel::from_low_level(context, value.vgprs_per_simd),
            min_vgpr_allocation: crate::conv::FromLowLevel::from_low_level(context, value.min_vgpr_allocation),
            max_vgpr_allocation: crate::conv::FromLowLevel::from_low_level(context, value.max_vgpr_allocation),
            vgpr_allocation_granularity: crate::conv::FromLowLevel::from_low_level(
                context,
                value.vgpr_allocation_granularity,
            ),
        }
    }
}
