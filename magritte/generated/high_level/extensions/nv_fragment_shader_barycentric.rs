pub use crate::common::extensions::nv_fragment_shader_barycentric::{
    NV_FRAGMENT_SHADER_BARYCENTRIC_EXTENSION_NAME, NV_FRAGMENT_SHADER_BARYCENTRIC_SPEC_VERSION,
};
use crate::{context::Context, vulkan1_0::StructureType};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkPhysicalDeviceFragmentShaderBarycentricFeaturesNV")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceFragmentShaderBarycentricFeaturesNV {
    #[doc(alias = "fragmentShaderBarycentric")]
    pub fragment_shader_barycentric: bool,
}
impl PhysicalDeviceFragmentShaderBarycentricFeaturesNV {
    ///Get a reference to the `fragment_shader_barycentric` field.
    pub fn fragment_shader_barycentric(&self) -> &bool {
        &self.fragment_shader_barycentric
    }
    ///Get a mutable reference to the `fragment_shader_barycentric` field.
    pub fn fragment_shader_barycentric_mut(&mut self) -> &mut bool {
        &mut self.fragment_shader_barycentric
    }
    ///Sets the `fragment_shader_barycentric` field.
    pub fn set_fragment_shader_barycentric(&mut self, fragment_shader_barycentric: bool) -> &mut Self {
        self.fragment_shader_barycentric = fragment_shader_barycentric;
        self
    }
    ///Sets the `fragment_shader_barycentric` field in a builder way.
    pub fn with_fragment_shader_barycentric(mut self, fragment_shader_barycentric: bool) -> Self {
        self.fragment_shader_barycentric = fragment_shader_barycentric;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceFragmentShaderBarycentricFeaturesNV {
    type LowLevel =
        crate::native::extensions::nv_fragment_shader_barycentric::PhysicalDeviceFragmentShaderBarycentricFeaturesNV;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::nv_fragment_shader_barycentric::PhysicalDeviceFragmentShaderBarycentricFeaturesNV {
            s_type: StructureType::PhysicalDeviceFragmentShaderBarycentricFeaturesNv,
            p_next: std::ptr::null_mut(),
            fragment_shader_barycentric: self.fragment_shader_barycentric.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceFragmentShaderBarycentricFeaturesNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            fragment_shader_barycentric: crate::conv::FromLowLevel::from_low_level(
                context,
                value.fragment_shader_barycentric,
            ),
        }
    }
}
