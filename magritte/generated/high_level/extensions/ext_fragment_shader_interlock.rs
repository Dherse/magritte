pub use crate::common::extensions::ext_fragment_shader_interlock::{
    EXT_FRAGMENT_SHADER_INTERLOCK_EXTENSION_NAME, EXT_FRAGMENT_SHADER_INTERLOCK_SPEC_VERSION,
};
use crate::{context::Context, vulkan1_0::StructureType};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkPhysicalDeviceFragmentShaderInterlockFeaturesEXT")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceFragmentShaderInterlockFeaturesEXT {
    #[doc(alias = "fragmentShaderSampleInterlock")]
    pub fragment_shader_sample_interlock: bool,
    #[doc(alias = "fragmentShaderPixelInterlock")]
    pub fragment_shader_pixel_interlock: bool,
    #[doc(alias = "fragmentShaderShadingRateInterlock")]
    pub fragment_shader_shading_rate_interlock: bool,
}
impl PhysicalDeviceFragmentShaderInterlockFeaturesEXT {
    ///Get a reference to the `fragment_shader_sample_interlock` field.
    pub fn fragment_shader_sample_interlock(&self) -> &bool {
        &self.fragment_shader_sample_interlock
    }
    ///Get a reference to the `fragment_shader_pixel_interlock` field.
    pub fn fragment_shader_pixel_interlock(&self) -> &bool {
        &self.fragment_shader_pixel_interlock
    }
    ///Get a reference to the `fragment_shader_shading_rate_interlock` field.
    pub fn fragment_shader_shading_rate_interlock(&self) -> &bool {
        &self.fragment_shader_shading_rate_interlock
    }
    ///Get a mutable reference to the `fragment_shader_sample_interlock` field.
    pub fn fragment_shader_sample_interlock_mut(&mut self) -> &mut bool {
        &mut self.fragment_shader_sample_interlock
    }
    ///Get a mutable reference to the `fragment_shader_pixel_interlock` field.
    pub fn fragment_shader_pixel_interlock_mut(&mut self) -> &mut bool {
        &mut self.fragment_shader_pixel_interlock
    }
    ///Get a mutable reference to the `fragment_shader_shading_rate_interlock` field.
    pub fn fragment_shader_shading_rate_interlock_mut(&mut self) -> &mut bool {
        &mut self.fragment_shader_shading_rate_interlock
    }
    ///Sets the `fragment_shader_sample_interlock` field.
    pub fn set_fragment_shader_sample_interlock(&mut self, fragment_shader_sample_interlock: bool) -> &mut Self {
        self.fragment_shader_sample_interlock = fragment_shader_sample_interlock;
        self
    }
    ///Sets the `fragment_shader_pixel_interlock` field.
    pub fn set_fragment_shader_pixel_interlock(&mut self, fragment_shader_pixel_interlock: bool) -> &mut Self {
        self.fragment_shader_pixel_interlock = fragment_shader_pixel_interlock;
        self
    }
    ///Sets the `fragment_shader_shading_rate_interlock` field.
    pub fn set_fragment_shader_shading_rate_interlock(
        &mut self,
        fragment_shader_shading_rate_interlock: bool,
    ) -> &mut Self {
        self.fragment_shader_shading_rate_interlock = fragment_shader_shading_rate_interlock;
        self
    }
    ///Sets the `fragment_shader_sample_interlock` field in a builder way.
    pub fn with_fragment_shader_sample_interlock(mut self, fragment_shader_sample_interlock: bool) -> Self {
        self.fragment_shader_sample_interlock = fragment_shader_sample_interlock;
        self
    }
    ///Sets the `fragment_shader_pixel_interlock` field in a builder way.
    pub fn with_fragment_shader_pixel_interlock(mut self, fragment_shader_pixel_interlock: bool) -> Self {
        self.fragment_shader_pixel_interlock = fragment_shader_pixel_interlock;
        self
    }
    ///Sets the `fragment_shader_shading_rate_interlock` field in a builder way.
    pub fn with_fragment_shader_shading_rate_interlock(mut self, fragment_shader_shading_rate_interlock: bool) -> Self {
        self.fragment_shader_shading_rate_interlock = fragment_shader_shading_rate_interlock;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceFragmentShaderInterlockFeaturesEXT {
    type LowLevel =
        crate::native::extensions::ext_fragment_shader_interlock::PhysicalDeviceFragmentShaderInterlockFeaturesEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_fragment_shader_interlock::PhysicalDeviceFragmentShaderInterlockFeaturesEXT {
            s_type: StructureType::PhysicalDeviceFragmentShaderInterlockFeaturesExt,
            p_next: std::ptr::null_mut(),
            fragment_shader_sample_interlock: self.fragment_shader_sample_interlock.into_low_level(context, bump),
            fragment_shader_pixel_interlock: self.fragment_shader_pixel_interlock.into_low_level(context, bump),
            fragment_shader_shading_rate_interlock: self
                .fragment_shader_shading_rate_interlock
                .into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceFragmentShaderInterlockFeaturesEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            fragment_shader_sample_interlock: crate::conv::FromLowLevel::from_low_level(
                context,
                value.fragment_shader_sample_interlock,
            ),
            fragment_shader_pixel_interlock: crate::conv::FromLowLevel::from_low_level(
                context,
                value.fragment_shader_pixel_interlock,
            ),
            fragment_shader_shading_rate_interlock: crate::conv::FromLowLevel::from_low_level(
                context,
                value.fragment_shader_shading_rate_interlock,
            ),
        }
    }
}
