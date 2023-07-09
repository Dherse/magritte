pub use crate::common::extensions::ext_full_screen_exclusive::{
    FullScreenExclusiveEXT, EXT_FULL_SCREEN_EXCLUSIVE_EXTENSION_NAME, EXT_FULL_SCREEN_EXCLUSIVE_SPEC_VERSION,
};
use crate::{context::Context, vulkan1_0::StructureType};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkSurfaceFullScreenExclusiveInfoEXT")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SurfaceFullScreenExclusiveInfoEXT {
    #[doc(alias = "fullScreenExclusive")]
    pub full_screen_exclusive: FullScreenExclusiveEXT,
}
impl SurfaceFullScreenExclusiveInfoEXT {
    ///Get a reference to the `full_screen_exclusive` field.
    pub fn full_screen_exclusive(&self) -> FullScreenExclusiveEXT {
        self.full_screen_exclusive
    }
    ///Get a mutable reference to the `full_screen_exclusive` field.
    pub fn full_screen_exclusive_mut(&mut self) -> &mut FullScreenExclusiveEXT {
        &mut self.full_screen_exclusive
    }
    ///Sets the `full_screen_exclusive` field.
    pub fn set_full_screen_exclusive(&mut self, full_screen_exclusive: FullScreenExclusiveEXT) -> &mut Self {
        self.full_screen_exclusive = full_screen_exclusive;
        self
    }
    ///Sets the `full_screen_exclusive` field in a builder way.
    pub fn with_full_screen_exclusive(mut self, full_screen_exclusive: FullScreenExclusiveEXT) -> Self {
        self.full_screen_exclusive = full_screen_exclusive;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for SurfaceFullScreenExclusiveInfoEXT {
    type LowLevel = crate::native::extensions::ext_full_screen_exclusive::SurfaceFullScreenExclusiveInfoEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_full_screen_exclusive::SurfaceFullScreenExclusiveInfoEXT {
            s_type: StructureType::SurfaceFullScreenExclusiveInfoExt,
            p_next: std::ptr::null_mut(),
            full_screen_exclusive: self.full_screen_exclusive.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for SurfaceFullScreenExclusiveInfoEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            full_screen_exclusive: crate::conv::FromLowLevel::from_low_level(context, value.full_screen_exclusive),
        }
    }
}
#[doc(alias = "VkSurfaceCapabilitiesFullScreenExclusiveEXT")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SurfaceCapabilitiesFullScreenExclusiveEXT {
    #[doc(alias = "fullScreenExclusiveSupported")]
    pub full_screen_exclusive_supported: bool,
}
impl SurfaceCapabilitiesFullScreenExclusiveEXT {
    ///Get a reference to the `full_screen_exclusive_supported` field.
    pub fn full_screen_exclusive_supported(&self) -> &bool {
        &self.full_screen_exclusive_supported
    }
    ///Get a mutable reference to the `full_screen_exclusive_supported` field.
    pub fn full_screen_exclusive_supported_mut(&mut self) -> &mut bool {
        &mut self.full_screen_exclusive_supported
    }
    ///Sets the `full_screen_exclusive_supported` field.
    pub fn set_full_screen_exclusive_supported(&mut self, full_screen_exclusive_supported: bool) -> &mut Self {
        self.full_screen_exclusive_supported = full_screen_exclusive_supported;
        self
    }
    ///Sets the `full_screen_exclusive_supported` field in a builder way.
    pub fn with_full_screen_exclusive_supported(mut self, full_screen_exclusive_supported: bool) -> Self {
        self.full_screen_exclusive_supported = full_screen_exclusive_supported;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for SurfaceCapabilitiesFullScreenExclusiveEXT {
    type LowLevel = crate::native::extensions::ext_full_screen_exclusive::SurfaceCapabilitiesFullScreenExclusiveEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_full_screen_exclusive::SurfaceCapabilitiesFullScreenExclusiveEXT {
            s_type: StructureType::SurfaceCapabilitiesFullScreenExclusiveExt,
            p_next: std::ptr::null_mut(),
            full_screen_exclusive_supported: self.full_screen_exclusive_supported.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for SurfaceCapabilitiesFullScreenExclusiveEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            full_screen_exclusive_supported: crate::conv::FromLowLevel::from_low_level(
                context,
                value.full_screen_exclusive_supported,
            ),
        }
    }
}
