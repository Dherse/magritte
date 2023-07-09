pub use crate::common::extensions::khr_get_display_properties2::{
    KHR_GET_DISPLAY_PROPERTIES_2_EXTENSION_NAME, KHR_GET_DISPLAY_PROPERTIES_2_SPEC_VERSION,
};
use crate::{
    context::Context,
    extensions::khr_display::{
        DisplayModeKHR, DisplayModePropertiesKHR, DisplayPlaneCapabilitiesKHR, DisplayPlanePropertiesKHR,
        DisplayPropertiesKHR,
    },
    vulkan1_0::StructureType,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkDisplayProperties2KHR")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DisplayProperties2KHR {
    #[doc(alias = "displayProperties")]
    pub display_properties: DisplayPropertiesKHR,
}
impl DisplayProperties2KHR {
    ///Get a reference to the `display_properties` field.
    pub fn display_properties(&self) -> &DisplayPropertiesKHR {
        &self.display_properties
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for DisplayProperties2KHR {
    type LowLevel = crate::native::extensions::khr_get_display_properties2::DisplayProperties2KHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::khr_get_display_properties2::DisplayProperties2KHR {
            s_type: StructureType::DisplayProperties2Khr,
            p_next: std::ptr::null_mut(),
            display_properties: self.display_properties.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for DisplayProperties2KHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            display_properties: crate::conv::FromLowLevel::from_low_level(context, value.display_properties),
        }
    }
}
#[doc(alias = "VkDisplayPlaneProperties2KHR")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DisplayPlaneProperties2KHR {
    #[doc(alias = "displayPlaneProperties")]
    pub display_plane_properties: DisplayPlanePropertiesKHR,
}
impl DisplayPlaneProperties2KHR {
    ///Get a reference to the `display_plane_properties` field.
    pub fn display_plane_properties(&self) -> &DisplayPlanePropertiesKHR {
        &self.display_plane_properties
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for DisplayPlaneProperties2KHR {
    type LowLevel = crate::native::extensions::khr_get_display_properties2::DisplayPlaneProperties2KHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::khr_get_display_properties2::DisplayPlaneProperties2KHR {
            s_type: StructureType::DisplayPlaneProperties2Khr,
            p_next: std::ptr::null_mut(),
            display_plane_properties: self.display_plane_properties.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for DisplayPlaneProperties2KHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            display_plane_properties: crate::conv::FromLowLevel::from_low_level(
                context,
                value.display_plane_properties,
            ),
        }
    }
}
#[doc(alias = "VkDisplayModeProperties2KHR")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DisplayModeProperties2KHR {
    #[doc(alias = "displayModeProperties")]
    pub display_mode_properties: DisplayModePropertiesKHR,
}
impl DisplayModeProperties2KHR {
    ///Get a reference to the `display_mode_properties` field.
    pub fn display_mode_properties(&self) -> &DisplayModePropertiesKHR {
        &self.display_mode_properties
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for DisplayModeProperties2KHR {
    type LowLevel = crate::native::extensions::khr_get_display_properties2::DisplayModeProperties2KHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::khr_get_display_properties2::DisplayModeProperties2KHR {
            s_type: StructureType::DisplayModeProperties2Khr,
            p_next: std::ptr::null_mut(),
            display_mode_properties: self.display_mode_properties.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for DisplayModeProperties2KHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            display_mode_properties: crate::conv::FromLowLevel::from_low_level(context, value.display_mode_properties),
        }
    }
}
#[doc(alias = "VkDisplayPlaneInfo2KHR")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DisplayPlaneInfo2KHR {
    pub mode: DisplayModeKHR,
    #[doc(alias = "planeIndex")]
    pub plane_index: u32,
}
impl DisplayPlaneInfo2KHR {
    ///Get a reference to the `mode` field.
    pub fn mode(&self) -> &DisplayModeKHR {
        &self.mode
    }
    ///Get a reference to the `plane_index` field.
    pub fn plane_index(&self) -> u32 {
        self.plane_index
    }
    ///Get a mutable reference to the `mode` field.
    pub fn mode_mut(&mut self) -> &mut DisplayModeKHR {
        &mut self.mode
    }
    ///Get a mutable reference to the `plane_index` field.
    pub fn plane_index_mut(&mut self) -> &mut u32 {
        &mut self.plane_index
    }
    ///Sets the `mode` field.
    pub fn set_mode(&mut self, mode: DisplayModeKHR) -> &mut Self {
        self.mode = mode;
        self
    }
    ///Sets the `plane_index` field.
    pub fn set_plane_index(&mut self, plane_index: u32) -> &mut Self {
        self.plane_index = plane_index;
        self
    }
    ///Sets the `mode` field in a builder way.
    pub fn with_mode(mut self, mode: DisplayModeKHR) -> Self {
        self.mode = mode;
        self
    }
    ///Sets the `plane_index` field in a builder way.
    pub fn with_plane_index(mut self, plane_index: u32) -> Self {
        self.plane_index = plane_index;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for DisplayPlaneInfo2KHR {
    type LowLevel = crate::native::extensions::khr_get_display_properties2::DisplayPlaneInfo2KHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::khr_get_display_properties2::DisplayPlaneInfo2KHR {
            s_type: StructureType::DisplayPlaneInfo2Khr,
            p_next: std::ptr::null(),
            mode: self.mode.into_low_level(context, bump),
            plane_index: self.plane_index.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for DisplayPlaneInfo2KHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            mode: crate::conv::FromLowLevel::from_low_level(context, value.mode),
            plane_index: crate::conv::FromLowLevel::from_low_level(context, value.plane_index),
        }
    }
}
#[doc(alias = "VkDisplayPlaneCapabilities2KHR")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DisplayPlaneCapabilities2KHR {
    pub capabilities: DisplayPlaneCapabilitiesKHR,
}
impl DisplayPlaneCapabilities2KHR {
    ///Get a reference to the `capabilities` field.
    pub fn capabilities(&self) -> DisplayPlaneCapabilitiesKHR {
        self.capabilities
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for DisplayPlaneCapabilities2KHR {
    type LowLevel = crate::native::extensions::khr_get_display_properties2::DisplayPlaneCapabilities2KHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::khr_get_display_properties2::DisplayPlaneCapabilities2KHR {
            s_type: StructureType::DisplayPlaneCapabilities2Khr,
            p_next: std::ptr::null_mut(),
            capabilities: self.capabilities.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for DisplayPlaneCapabilities2KHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            capabilities: crate::conv::FromLowLevel::from_low_level(context, value.capabilities),
        }
    }
}
