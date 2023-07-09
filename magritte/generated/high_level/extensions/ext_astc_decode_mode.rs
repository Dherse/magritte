pub use crate::common::extensions::ext_astc_decode_mode::{
    EXT_ASTC_DECODE_MODE_EXTENSION_NAME, EXT_ASTC_DECODE_MODE_SPEC_VERSION,
};
use crate::{
    context::Context,
    vulkan1_0::{Format, StructureType},
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkImageViewASTCDecodeModeEXT")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ImageViewAstcDecodeModeEXT {
    #[doc(alias = "decodeMode")]
    pub decode_mode: Format,
}
impl ImageViewAstcDecodeModeEXT {
    ///Get a reference to the `decode_mode` field.
    pub fn decode_mode(&self) -> Format {
        self.decode_mode
    }
    ///Get a mutable reference to the `decode_mode` field.
    pub fn decode_mode_mut(&mut self) -> &mut Format {
        &mut self.decode_mode
    }
    ///Sets the `decode_mode` field.
    pub fn set_decode_mode(&mut self, decode_mode: Format) -> &mut Self {
        self.decode_mode = decode_mode;
        self
    }
    ///Sets the `decode_mode` field in a builder way.
    pub fn with_decode_mode(mut self, decode_mode: Format) -> Self {
        self.decode_mode = decode_mode;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for ImageViewAstcDecodeModeEXT {
    type LowLevel = crate::native::extensions::ext_astc_decode_mode::ImageViewAstcDecodeModeEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_astc_decode_mode::ImageViewAstcDecodeModeEXT {
            s_type: StructureType::ImageViewAstcDecodeModeExt,
            p_next: std::ptr::null(),
            decode_mode: self.decode_mode.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for ImageViewAstcDecodeModeEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            decode_mode: crate::conv::FromLowLevel::from_low_level(context, value.decode_mode),
        }
    }
}
#[doc(alias = "VkPhysicalDeviceASTCDecodeFeaturesEXT")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceAstcDecodeFeaturesEXT {
    #[doc(alias = "decodeModeSharedExponent")]
    pub decode_mode_shared_exponent: bool,
}
impl PhysicalDeviceAstcDecodeFeaturesEXT {
    ///Get a reference to the `decode_mode_shared_exponent` field.
    pub fn decode_mode_shared_exponent(&self) -> &bool {
        &self.decode_mode_shared_exponent
    }
    ///Get a mutable reference to the `decode_mode_shared_exponent` field.
    pub fn decode_mode_shared_exponent_mut(&mut self) -> &mut bool {
        &mut self.decode_mode_shared_exponent
    }
    ///Sets the `decode_mode_shared_exponent` field.
    pub fn set_decode_mode_shared_exponent(&mut self, decode_mode_shared_exponent: bool) -> &mut Self {
        self.decode_mode_shared_exponent = decode_mode_shared_exponent;
        self
    }
    ///Sets the `decode_mode_shared_exponent` field in a builder way.
    pub fn with_decode_mode_shared_exponent(mut self, decode_mode_shared_exponent: bool) -> Self {
        self.decode_mode_shared_exponent = decode_mode_shared_exponent;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceAstcDecodeFeaturesEXT {
    type LowLevel = crate::native::extensions::ext_astc_decode_mode::PhysicalDeviceAstcDecodeFeaturesEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_astc_decode_mode::PhysicalDeviceAstcDecodeFeaturesEXT {
            s_type: StructureType::PhysicalDeviceAstcDecodeFeaturesExt,
            p_next: std::ptr::null_mut(),
            decode_mode_shared_exponent: self.decode_mode_shared_exponent.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceAstcDecodeFeaturesEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            decode_mode_shared_exponent: crate::conv::FromLowLevel::from_low_level(
                context,
                value.decode_mode_shared_exponent,
            ),
        }
    }
}
