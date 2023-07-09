pub use crate::common::extensions::amd_display_native_hdr::{
    AMD_DISPLAY_NATIVE_HDR_EXTENSION_NAME, AMD_DISPLAY_NATIVE_HDR_SPEC_VERSION,
};
use crate::{context::Context, vulkan1_0::StructureType};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkDisplayNativeHdrSurfaceCapabilitiesAMD")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DisplayNativeHdrSurfaceCapabilitiesAMD {
    #[doc(alias = "localDimmingSupport")]
    pub local_dimming_support: bool,
}
impl DisplayNativeHdrSurfaceCapabilitiesAMD {
    ///Get a reference to the `local_dimming_support` field.
    pub fn local_dimming_support(&self) -> &bool {
        &self.local_dimming_support
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for DisplayNativeHdrSurfaceCapabilitiesAMD {
    type LowLevel = crate::native::extensions::amd_display_native_hdr::DisplayNativeHdrSurfaceCapabilitiesAMD;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::amd_display_native_hdr::DisplayNativeHdrSurfaceCapabilitiesAMD {
            s_type: StructureType::DisplayNativeHdrSurfaceCapabilitiesAmd,
            p_next: std::ptr::null_mut(),
            local_dimming_support: self.local_dimming_support.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for DisplayNativeHdrSurfaceCapabilitiesAMD {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            local_dimming_support: crate::conv::FromLowLevel::from_low_level(context, value.local_dimming_support),
        }
    }
}
#[doc(alias = "VkSwapchainDisplayNativeHdrCreateInfoAMD")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SwapchainDisplayNativeHdrCreateInfoAMD {
    #[doc(alias = "localDimmingEnable")]
    pub local_dimming_enable: bool,
}
impl SwapchainDisplayNativeHdrCreateInfoAMD {
    ///Get a reference to the `local_dimming_enable` field.
    pub fn local_dimming_enable(&self) -> &bool {
        &self.local_dimming_enable
    }
    ///Get a mutable reference to the `local_dimming_enable` field.
    pub fn local_dimming_enable_mut(&mut self) -> &mut bool {
        &mut self.local_dimming_enable
    }
    ///Sets the `local_dimming_enable` field.
    pub fn set_local_dimming_enable(&mut self, local_dimming_enable: bool) -> &mut Self {
        self.local_dimming_enable = local_dimming_enable;
        self
    }
    ///Sets the `local_dimming_enable` field in a builder way.
    pub fn with_local_dimming_enable(mut self, local_dimming_enable: bool) -> Self {
        self.local_dimming_enable = local_dimming_enable;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for SwapchainDisplayNativeHdrCreateInfoAMD {
    type LowLevel = crate::native::extensions::amd_display_native_hdr::SwapchainDisplayNativeHdrCreateInfoAMD;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::amd_display_native_hdr::SwapchainDisplayNativeHdrCreateInfoAMD {
            s_type: StructureType::SwapchainDisplayNativeHdrCreateInfoAmd,
            p_next: std::ptr::null(),
            local_dimming_enable: self.local_dimming_enable.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for SwapchainDisplayNativeHdrCreateInfoAMD {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            local_dimming_enable: crate::conv::FromLowLevel::from_low_level(context, value.local_dimming_enable),
        }
    }
}
