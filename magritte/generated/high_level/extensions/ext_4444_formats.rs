pub use crate::common::extensions::ext_4444_formats::{EXT_4444_FORMATS_EXTENSION_NAME, EXT_4444_FORMATS_SPEC_VERSION};
use crate::{context::Context, vulkan1_0::StructureType};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkPhysicalDevice4444FormatsFeaturesEXT")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDevice4444FormatsFeaturesEXT {
    #[doc(alias = "formatA4R4G4B4")]
    pub format_a4r4g4b4: bool,
    #[doc(alias = "formatA4B4G4R4")]
    pub format_a4b4g4r4: bool,
}
impl PhysicalDevice4444FormatsFeaturesEXT {
    ///Get a reference to the `format_a4r4g4b4` field.
    pub fn format_a4r4g4b4(&self) -> &bool {
        &self.format_a4r4g4b4
    }
    ///Get a reference to the `format_a4b4g4r4` field.
    pub fn format_a4b4g4r4(&self) -> &bool {
        &self.format_a4b4g4r4
    }
    ///Get a mutable reference to the `format_a4r4g4b4` field.
    pub fn format_a4r4g4b4_mut(&mut self) -> &mut bool {
        &mut self.format_a4r4g4b4
    }
    ///Get a mutable reference to the `format_a4b4g4r4` field.
    pub fn format_a4b4g4r4_mut(&mut self) -> &mut bool {
        &mut self.format_a4b4g4r4
    }
    ///Sets the `format_a4r4g4b4` field.
    pub fn set_format_a4r4g4b4(&mut self, format_a4r4g4b4: bool) -> &mut Self {
        self.format_a4r4g4b4 = format_a4r4g4b4;
        self
    }
    ///Sets the `format_a4b4g4r4` field.
    pub fn set_format_a4b4g4r4(&mut self, format_a4b4g4r4: bool) -> &mut Self {
        self.format_a4b4g4r4 = format_a4b4g4r4;
        self
    }
    ///Sets the `format_a4r4g4b4` field in a builder way.
    pub fn with_format_a4r4g4b4(mut self, format_a4r4g4b4: bool) -> Self {
        self.format_a4r4g4b4 = format_a4r4g4b4;
        self
    }
    ///Sets the `format_a4b4g4r4` field in a builder way.
    pub fn with_format_a4b4g4r4(mut self, format_a4b4g4r4: bool) -> Self {
        self.format_a4b4g4r4 = format_a4b4g4r4;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDevice4444FormatsFeaturesEXT {
    type LowLevel = crate::native::extensions::ext_4444_formats::PhysicalDevice4444FormatsFeaturesEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_4444_formats::PhysicalDevice4444FormatsFeaturesEXT {
            s_type: StructureType::PhysicalDevice4444FormatsFeaturesExt,
            p_next: std::ptr::null_mut(),
            format_a4r4g4b4: self.format_a4r4g4b4.into_low_level(context, bump),
            format_a4b4g4r4: self.format_a4b4g4r4.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDevice4444FormatsFeaturesEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            format_a4r4g4b4: crate::conv::FromLowLevel::from_low_level(context, value.format_a4r4g4b4),
            format_a4b4g4r4: crate::conv::FromLowLevel::from_low_level(context, value.format_a4b4g4r4),
        }
    }
}
