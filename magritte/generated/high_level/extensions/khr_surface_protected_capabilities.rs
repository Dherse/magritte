pub use crate::common::extensions::khr_surface_protected_capabilities::{
    KHR_SURFACE_PROTECTED_CAPABILITIES_EXTENSION_NAME, KHR_SURFACE_PROTECTED_CAPABILITIES_SPEC_VERSION,
};
use crate::{context::Context, vulkan1_0::StructureType};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkSurfaceProtectedCapabilitiesKHR")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SurfaceProtectedCapabilitiesKHR {
    #[doc(alias = "supportsProtected")]
    pub supports_protected: bool,
}
impl SurfaceProtectedCapabilitiesKHR {
    ///Get a reference to the `supports_protected` field.
    pub fn supports_protected(&self) -> &bool {
        &self.supports_protected
    }
    ///Get a mutable reference to the `supports_protected` field.
    pub fn supports_protected_mut(&mut self) -> &mut bool {
        &mut self.supports_protected
    }
    ///Sets the `supports_protected` field.
    pub fn set_supports_protected(&mut self, supports_protected: bool) -> &mut Self {
        self.supports_protected = supports_protected;
        self
    }
    ///Sets the `supports_protected` field in a builder way.
    pub fn with_supports_protected(mut self, supports_protected: bool) -> Self {
        self.supports_protected = supports_protected;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for SurfaceProtectedCapabilitiesKHR {
    type LowLevel = crate::native::extensions::khr_surface_protected_capabilities::SurfaceProtectedCapabilitiesKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::khr_surface_protected_capabilities::SurfaceProtectedCapabilitiesKHR {
            s_type: StructureType::SurfaceProtectedCapabilitiesKhr,
            p_next: std::ptr::null(),
            supports_protected: self.supports_protected.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for SurfaceProtectedCapabilitiesKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            supports_protected: crate::conv::FromLowLevel::from_low_level(context, value.supports_protected),
        }
    }
}
