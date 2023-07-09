pub use crate::common::extensions::ext_headless_surface::{
    HeadlessSurfaceCreateFlagsEXT, EXT_HEADLESS_SURFACE_EXTENSION_NAME, EXT_HEADLESS_SURFACE_SPEC_VERSION,
};
use crate::{context::Context, vulkan1_0::StructureType};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkHeadlessSurfaceCreateInfoEXT")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct HeadlessSurfaceCreateInfoEXT {
    pub flags: HeadlessSurfaceCreateFlagsEXT,
}
impl HeadlessSurfaceCreateInfoEXT {
    ///Get a reference to the `flags` field.
    pub fn flags(&self) -> HeadlessSurfaceCreateFlagsEXT {
        self.flags
    }
    ///Get a mutable reference to the `flags` field.
    pub fn flags_mut(&mut self) -> &mut HeadlessSurfaceCreateFlagsEXT {
        &mut self.flags
    }
    ///Sets the `flags` field.
    pub fn set_flags(&mut self, flags: HeadlessSurfaceCreateFlagsEXT) -> &mut Self {
        self.flags = flags;
        self
    }
    ///Sets the `flags` field in a builder way.
    pub fn with_flags(mut self, flags: HeadlessSurfaceCreateFlagsEXT) -> Self {
        self.flags = flags;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for HeadlessSurfaceCreateInfoEXT {
    type LowLevel = crate::native::extensions::ext_headless_surface::HeadlessSurfaceCreateInfoEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_headless_surface::HeadlessSurfaceCreateInfoEXT {
            s_type: StructureType::HeadlessSurfaceCreateInfoExt,
            p_next: std::ptr::null(),
            flags: self.flags.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for HeadlessSurfaceCreateInfoEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            flags: crate::conv::FromLowLevel::from_low_level(context, value.flags),
        }
    }
}
