pub use crate::common::extensions::amd_shader_core_properties2::{
    ShaderCorePropertiesFlagBitsAMD, ShaderCorePropertiesFlagsAMD, AMD_SHADER_CORE_PROPERTIES_2_EXTENSION_NAME,
    AMD_SHADER_CORE_PROPERTIES_2_SPEC_VERSION,
};
use crate::{context::Context, vulkan1_0::StructureType};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkPhysicalDeviceShaderCoreProperties2AMD")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceShaderCoreProperties2AMD {
    #[doc(alias = "shaderCoreFeatures")]
    pub shader_core_features: ShaderCorePropertiesFlagsAMD,
    #[doc(alias = "activeComputeUnitCount")]
    pub active_compute_unit_count: u32,
}
impl PhysicalDeviceShaderCoreProperties2AMD {
    ///Get a reference to the `shader_core_features` field.
    pub fn shader_core_features(&self) -> ShaderCorePropertiesFlagsAMD {
        self.shader_core_features
    }
    ///Get a reference to the `active_compute_unit_count` field.
    pub fn active_compute_unit_count(&self) -> u32 {
        self.active_compute_unit_count
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceShaderCoreProperties2AMD {
    type LowLevel = crate::native::extensions::amd_shader_core_properties2::PhysicalDeviceShaderCoreProperties2AMD;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::amd_shader_core_properties2::PhysicalDeviceShaderCoreProperties2AMD {
            s_type: StructureType::PhysicalDeviceShaderCoreProperties2Amd,
            p_next: std::ptr::null_mut(),
            shader_core_features: self.shader_core_features.into_low_level(context, bump),
            active_compute_unit_count: self.active_compute_unit_count.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceShaderCoreProperties2AMD {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            shader_core_features: crate::conv::FromLowLevel::from_low_level(context, value.shader_core_features),
            active_compute_unit_count: crate::conv::FromLowLevel::from_low_level(
                context,
                value.active_compute_unit_count,
            ),
        }
    }
}
