pub use crate::common::extensions::nv_compute_shader_derivatives::{
    NV_COMPUTE_SHADER_DERIVATIVES_EXTENSION_NAME, NV_COMPUTE_SHADER_DERIVATIVES_SPEC_VERSION,
};
use crate::{context::Context, vulkan1_0::StructureType};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkPhysicalDeviceComputeShaderDerivativesFeaturesNV")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceComputeShaderDerivativesFeaturesNV {
    #[doc(alias = "computeDerivativeGroupQuads")]
    pub compute_derivative_group_quads: bool,
    #[doc(alias = "computeDerivativeGroupLinear")]
    pub compute_derivative_group_linear: bool,
}
impl PhysicalDeviceComputeShaderDerivativesFeaturesNV {
    ///Get a reference to the `compute_derivative_group_quads` field.
    pub fn compute_derivative_group_quads(&self) -> &bool {
        &self.compute_derivative_group_quads
    }
    ///Get a reference to the `compute_derivative_group_linear` field.
    pub fn compute_derivative_group_linear(&self) -> &bool {
        &self.compute_derivative_group_linear
    }
    ///Get a mutable reference to the `compute_derivative_group_quads` field.
    pub fn compute_derivative_group_quads_mut(&mut self) -> &mut bool {
        &mut self.compute_derivative_group_quads
    }
    ///Get a mutable reference to the `compute_derivative_group_linear` field.
    pub fn compute_derivative_group_linear_mut(&mut self) -> &mut bool {
        &mut self.compute_derivative_group_linear
    }
    ///Sets the `compute_derivative_group_quads` field.
    pub fn set_compute_derivative_group_quads(&mut self, compute_derivative_group_quads: bool) -> &mut Self {
        self.compute_derivative_group_quads = compute_derivative_group_quads;
        self
    }
    ///Sets the `compute_derivative_group_linear` field.
    pub fn set_compute_derivative_group_linear(&mut self, compute_derivative_group_linear: bool) -> &mut Self {
        self.compute_derivative_group_linear = compute_derivative_group_linear;
        self
    }
    ///Sets the `compute_derivative_group_quads` field in a builder way.
    pub fn with_compute_derivative_group_quads(mut self, compute_derivative_group_quads: bool) -> Self {
        self.compute_derivative_group_quads = compute_derivative_group_quads;
        self
    }
    ///Sets the `compute_derivative_group_linear` field in a builder way.
    pub fn with_compute_derivative_group_linear(mut self, compute_derivative_group_linear: bool) -> Self {
        self.compute_derivative_group_linear = compute_derivative_group_linear;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceComputeShaderDerivativesFeaturesNV {
    type LowLevel =
        crate::native::extensions::nv_compute_shader_derivatives::PhysicalDeviceComputeShaderDerivativesFeaturesNV;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::nv_compute_shader_derivatives::PhysicalDeviceComputeShaderDerivativesFeaturesNV {
            s_type: StructureType::PhysicalDeviceComputeShaderDerivativesFeaturesNv,
            p_next: std::ptr::null_mut(),
            compute_derivative_group_quads: self.compute_derivative_group_quads.into_low_level(context, bump),
            compute_derivative_group_linear: self.compute_derivative_group_linear.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceComputeShaderDerivativesFeaturesNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            compute_derivative_group_quads: crate::conv::FromLowLevel::from_low_level(
                context,
                value.compute_derivative_group_quads,
            ),
            compute_derivative_group_linear: crate::conv::FromLowLevel::from_low_level(
                context,
                value.compute_derivative_group_linear,
            ),
        }
    }
}
