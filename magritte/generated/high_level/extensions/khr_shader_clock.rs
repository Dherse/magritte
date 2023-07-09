pub use crate::common::extensions::khr_shader_clock::{KHR_SHADER_CLOCK_EXTENSION_NAME, KHR_SHADER_CLOCK_SPEC_VERSION};
use crate::{context::Context, vulkan1_0::StructureType};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkPhysicalDeviceShaderClockFeaturesKHR")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceShaderClockFeaturesKHR {
    #[doc(alias = "shaderSubgroupClock")]
    pub shader_subgroup_clock: bool,
    #[doc(alias = "shaderDeviceClock")]
    pub shader_device_clock: bool,
}
impl PhysicalDeviceShaderClockFeaturesKHR {
    ///Get a reference to the `shader_subgroup_clock` field.
    pub fn shader_subgroup_clock(&self) -> &bool {
        &self.shader_subgroup_clock
    }
    ///Get a reference to the `shader_device_clock` field.
    pub fn shader_device_clock(&self) -> &bool {
        &self.shader_device_clock
    }
    ///Get a mutable reference to the `shader_subgroup_clock` field.
    pub fn shader_subgroup_clock_mut(&mut self) -> &mut bool {
        &mut self.shader_subgroup_clock
    }
    ///Get a mutable reference to the `shader_device_clock` field.
    pub fn shader_device_clock_mut(&mut self) -> &mut bool {
        &mut self.shader_device_clock
    }
    ///Sets the `shader_subgroup_clock` field.
    pub fn set_shader_subgroup_clock(&mut self, shader_subgroup_clock: bool) -> &mut Self {
        self.shader_subgroup_clock = shader_subgroup_clock;
        self
    }
    ///Sets the `shader_device_clock` field.
    pub fn set_shader_device_clock(&mut self, shader_device_clock: bool) -> &mut Self {
        self.shader_device_clock = shader_device_clock;
        self
    }
    ///Sets the `shader_subgroup_clock` field in a builder way.
    pub fn with_shader_subgroup_clock(mut self, shader_subgroup_clock: bool) -> Self {
        self.shader_subgroup_clock = shader_subgroup_clock;
        self
    }
    ///Sets the `shader_device_clock` field in a builder way.
    pub fn with_shader_device_clock(mut self, shader_device_clock: bool) -> Self {
        self.shader_device_clock = shader_device_clock;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceShaderClockFeaturesKHR {
    type LowLevel = crate::native::extensions::khr_shader_clock::PhysicalDeviceShaderClockFeaturesKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::khr_shader_clock::PhysicalDeviceShaderClockFeaturesKHR {
            s_type: StructureType::PhysicalDeviceShaderClockFeaturesKhr,
            p_next: std::ptr::null_mut(),
            shader_subgroup_clock: self.shader_subgroup_clock.into_low_level(context, bump),
            shader_device_clock: self.shader_device_clock.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceShaderClockFeaturesKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            shader_subgroup_clock: crate::conv::FromLowLevel::from_low_level(context, value.shader_subgroup_clock),
            shader_device_clock: crate::conv::FromLowLevel::from_low_level(context, value.shader_device_clock),
        }
    }
}
