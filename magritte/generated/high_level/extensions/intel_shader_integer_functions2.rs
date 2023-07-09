pub use crate::common::extensions::intel_shader_integer_functions2::{
    INTEL_SHADER_INTEGER_FUNCTIONS_2_EXTENSION_NAME, INTEL_SHADER_INTEGER_FUNCTIONS_2_SPEC_VERSION,
};
use crate::{context::Context, vulkan1_0::StructureType};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkPhysicalDeviceShaderIntegerFunctions2FeaturesINTEL")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL {
    #[doc(alias = "shaderIntegerFunctions2")]
    pub shader_integer_functions2: bool,
}
impl PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL {
    ///Get a reference to the `shader_integer_functions2` field.
    pub fn shader_integer_functions2(&self) -> &bool {
        &self.shader_integer_functions2
    }
    ///Get a mutable reference to the `shader_integer_functions2` field.
    pub fn shader_integer_functions2_mut(&mut self) -> &mut bool {
        &mut self.shader_integer_functions2
    }
    ///Sets the `shader_integer_functions2` field.
    pub fn set_shader_integer_functions2(&mut self, shader_integer_functions2: bool) -> &mut Self {
        self.shader_integer_functions2 = shader_integer_functions2;
        self
    }
    ///Sets the `shader_integer_functions2` field in a builder way.
    pub fn with_shader_integer_functions2(mut self, shader_integer_functions2: bool) -> Self {
        self.shader_integer_functions2 = shader_integer_functions2;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL {
    type LowLevel =
        crate::native::extensions::intel_shader_integer_functions2::PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::intel_shader_integer_functions2::PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL {
            s_type: StructureType::PhysicalDeviceShaderIntegerFunctions2FeaturesIntel,
            p_next: std::ptr::null_mut(),
            shader_integer_functions2: self.shader_integer_functions2.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            shader_integer_functions2: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_integer_functions2,
            ),
        }
    }
}
