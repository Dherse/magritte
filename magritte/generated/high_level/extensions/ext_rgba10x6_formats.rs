pub use crate::common::extensions::ext_rgba10x6_formats::{
    EXT_RGBA10X6_FORMATS_EXTENSION_NAME, EXT_RGBA10X6_FORMATS_SPEC_VERSION,
};
use crate::{context::Context, vulkan1_0::StructureType};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkPhysicalDeviceRGBA10X6FormatsFeaturesEXT")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceRgba10x6FormatsFeaturesEXT {
    #[doc(alias = "formatRgba10x6WithoutYCbCrSampler")]
    pub format_rgba10x6_without_y_cb_cr_sampler: bool,
}
impl PhysicalDeviceRgba10x6FormatsFeaturesEXT {
    ///Get a reference to the `format_rgba10x6_without_y_cb_cr_sampler` field.
    pub fn format_rgba10x6_without_y_cb_cr_sampler(&self) -> &bool {
        &self.format_rgba10x6_without_y_cb_cr_sampler
    }
    ///Get a mutable reference to the `format_rgba10x6_without_y_cb_cr_sampler` field.
    pub fn format_rgba10x6_without_y_cb_cr_sampler_mut(&mut self) -> &mut bool {
        &mut self.format_rgba10x6_without_y_cb_cr_sampler
    }
    ///Sets the `format_rgba10x6_without_y_cb_cr_sampler` field.
    pub fn set_format_rgba10x6_without_y_cb_cr_sampler(
        &mut self,
        format_rgba10x6_without_y_cb_cr_sampler: bool,
    ) -> &mut Self {
        self.format_rgba10x6_without_y_cb_cr_sampler = format_rgba10x6_without_y_cb_cr_sampler;
        self
    }
    ///Sets the `format_rgba10x6_without_y_cb_cr_sampler` field in a builder way.
    pub fn with_format_rgba10x6_without_y_cb_cr_sampler(
        mut self,
        format_rgba10x6_without_y_cb_cr_sampler: bool,
    ) -> Self {
        self.format_rgba10x6_without_y_cb_cr_sampler = format_rgba10x6_without_y_cb_cr_sampler;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceRgba10x6FormatsFeaturesEXT {
    type LowLevel = crate::native::extensions::ext_rgba10x6_formats::PhysicalDeviceRgba10x6FormatsFeaturesEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_rgba10x6_formats::PhysicalDeviceRgba10x6FormatsFeaturesEXT {
            s_type: StructureType::PhysicalDeviceRgba10x6FormatsFeaturesExt,
            p_next: std::ptr::null_mut(),
            format_rgba10x6_without_y_cb_cr_sampler: self
                .format_rgba10x6_without_y_cb_cr_sampler
                .into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceRgba10x6FormatsFeaturesEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            format_rgba10x6_without_y_cb_cr_sampler: crate::conv::FromLowLevel::from_low_level(
                context,
                value.format_rgba10x6_without_y_cb_cr_sampler,
            ),
        }
    }
}
