pub use crate::common::extensions::nv_shader_sm_builtins::{
    NV_SHADER_SM_BUILTINS_EXTENSION_NAME, NV_SHADER_SM_BUILTINS_SPEC_VERSION,
};
use crate::{context::Context, vulkan1_0::StructureType};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkPhysicalDeviceShaderSMBuiltinsPropertiesNV")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceShaderSmBuiltinsPropertiesNV {
    #[doc(alias = "shaderSMCount")]
    pub shader_sm_count: u32,
    #[doc(alias = "shaderWarpsPerSM")]
    pub shader_warps_per_sm: u32,
}
impl PhysicalDeviceShaderSmBuiltinsPropertiesNV {
    ///Get a reference to the `shader_sm_count` field.
    pub fn shader_sm_count(&self) -> u32 {
        self.shader_sm_count
    }
    ///Get a reference to the `shader_warps_per_sm` field.
    pub fn shader_warps_per_sm(&self) -> u32 {
        self.shader_warps_per_sm
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceShaderSmBuiltinsPropertiesNV {
    type LowLevel = crate::native::extensions::nv_shader_sm_builtins::PhysicalDeviceShaderSmBuiltinsPropertiesNV;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::nv_shader_sm_builtins::PhysicalDeviceShaderSmBuiltinsPropertiesNV {
            s_type: StructureType::PhysicalDeviceShaderSmBuiltinsPropertiesNv,
            p_next: std::ptr::null_mut(),
            shader_sm_count: self.shader_sm_count.into_low_level(context, bump),
            shader_warps_per_sm: self.shader_warps_per_sm.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceShaderSmBuiltinsPropertiesNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            shader_sm_count: crate::conv::FromLowLevel::from_low_level(context, value.shader_sm_count),
            shader_warps_per_sm: crate::conv::FromLowLevel::from_low_level(context, value.shader_warps_per_sm),
        }
    }
}
#[doc(alias = "VkPhysicalDeviceShaderSMBuiltinsFeaturesNV")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceShaderSmBuiltinsFeaturesNV {
    #[doc(alias = "shaderSMBuiltins")]
    pub shader_sm_builtins: bool,
}
impl PhysicalDeviceShaderSmBuiltinsFeaturesNV {
    ///Get a reference to the `shader_sm_builtins` field.
    pub fn shader_sm_builtins(&self) -> &bool {
        &self.shader_sm_builtins
    }
    ///Get a mutable reference to the `shader_sm_builtins` field.
    pub fn shader_sm_builtins_mut(&mut self) -> &mut bool {
        &mut self.shader_sm_builtins
    }
    ///Sets the `shader_sm_builtins` field.
    pub fn set_shader_sm_builtins(&mut self, shader_sm_builtins: bool) -> &mut Self {
        self.shader_sm_builtins = shader_sm_builtins;
        self
    }
    ///Sets the `shader_sm_builtins` field in a builder way.
    pub fn with_shader_sm_builtins(mut self, shader_sm_builtins: bool) -> Self {
        self.shader_sm_builtins = shader_sm_builtins;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceShaderSmBuiltinsFeaturesNV {
    type LowLevel = crate::native::extensions::nv_shader_sm_builtins::PhysicalDeviceShaderSmBuiltinsFeaturesNV;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::nv_shader_sm_builtins::PhysicalDeviceShaderSmBuiltinsFeaturesNV {
            s_type: StructureType::PhysicalDeviceShaderSmBuiltinsFeaturesNv,
            p_next: std::ptr::null_mut(),
            shader_sm_builtins: self.shader_sm_builtins.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceShaderSmBuiltinsFeaturesNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            shader_sm_builtins: crate::conv::FromLowLevel::from_low_level(context, value.shader_sm_builtins),
        }
    }
}
