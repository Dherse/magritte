pub use crate::common::extensions::ext_physical_device_drm::{
    EXT_PHYSICAL_DEVICE_DRM_EXTENSION_NAME, EXT_PHYSICAL_DEVICE_DRM_SPEC_VERSION,
};
use crate::{context::Context, vulkan1_0::StructureType};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkPhysicalDeviceDrmPropertiesEXT")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceDrmPropertiesEXT {
    #[doc(alias = "hasPrimary")]
    pub has_primary: bool,
    #[doc(alias = "hasRender")]
    pub has_render: bool,
    #[doc(alias = "primaryMajor")]
    pub primary_major: i64,
    #[doc(alias = "primaryMinor")]
    pub primary_minor: i64,
    #[doc(alias = "renderMajor")]
    pub render_major: i64,
    #[doc(alias = "renderMinor")]
    pub render_minor: i64,
}
impl PhysicalDeviceDrmPropertiesEXT {
    ///Get a reference to the `has_primary` field.
    pub fn has_primary(&self) -> &bool {
        &self.has_primary
    }
    ///Get a reference to the `has_render` field.
    pub fn has_render(&self) -> &bool {
        &self.has_render
    }
    ///Get a reference to the `primary_major` field.
    pub fn primary_major(&self) -> i64 {
        self.primary_major
    }
    ///Get a reference to the `primary_minor` field.
    pub fn primary_minor(&self) -> i64 {
        self.primary_minor
    }
    ///Get a reference to the `render_major` field.
    pub fn render_major(&self) -> i64 {
        self.render_major
    }
    ///Get a reference to the `render_minor` field.
    pub fn render_minor(&self) -> i64 {
        self.render_minor
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceDrmPropertiesEXT {
    type LowLevel = crate::native::extensions::ext_physical_device_drm::PhysicalDeviceDrmPropertiesEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_physical_device_drm::PhysicalDeviceDrmPropertiesEXT {
            s_type: StructureType::PhysicalDeviceDrmPropertiesExt,
            p_next: std::ptr::null_mut(),
            has_primary: self.has_primary.into_low_level(context, bump),
            has_render: self.has_render.into_low_level(context, bump),
            primary_major: self.primary_major.into_low_level(context, bump),
            primary_minor: self.primary_minor.into_low_level(context, bump),
            render_major: self.render_major.into_low_level(context, bump),
            render_minor: self.render_minor.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceDrmPropertiesEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            has_primary: crate::conv::FromLowLevel::from_low_level(context, value.has_primary),
            has_render: crate::conv::FromLowLevel::from_low_level(context, value.has_render),
            primary_major: crate::conv::FromLowLevel::from_low_level(context, value.primary_major),
            primary_minor: crate::conv::FromLowLevel::from_low_level(context, value.primary_minor),
            render_major: crate::conv::FromLowLevel::from_low_level(context, value.render_major),
            render_minor: crate::conv::FromLowLevel::from_low_level(context, value.render_minor),
        }
    }
}
