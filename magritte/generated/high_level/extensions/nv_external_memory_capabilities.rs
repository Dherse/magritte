pub use crate::common::extensions::nv_external_memory_capabilities::{
    ExternalImageFormatPropertiesNV, ExternalMemoryFeatureFlagBitsNV, ExternalMemoryFeatureFlagsNV,
    ExternalMemoryHandleTypeFlagBitsNV, ExternalMemoryHandleTypeFlagsNV,
    NV_EXTERNAL_MEMORY_CAPABILITIES_EXTENSION_NAME, NV_EXTERNAL_MEMORY_CAPABILITIES_SPEC_VERSION,
};
use crate::{context::Context, vulkan1_0::ImageFormatProperties};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
impl ExternalImageFormatPropertiesNV {
    ///Get a reference to the `image_format_properties` field.
    pub fn image_format_properties(&self) -> &ImageFormatProperties {
        &self.image_format_properties
    }
    ///Get a reference to the `external_memory_features` field.
    pub fn external_memory_features(&self) -> ExternalMemoryFeatureFlagsNV {
        self.external_memory_features
    }
    ///Get a reference to the `export_from_imported_handle_types` field.
    pub fn export_from_imported_handle_types(&self) -> ExternalMemoryHandleTypeFlagsNV {
        self.export_from_imported_handle_types
    }
    ///Get a reference to the `compatible_handle_types` field.
    pub fn compatible_handle_types(&self) -> ExternalMemoryHandleTypeFlagsNV {
        self.compatible_handle_types
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for ExternalImageFormatPropertiesNV {
    type LowLevel = crate::native::extensions::nv_external_memory_capabilities::ExternalImageFormatPropertiesNV;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::nv_external_memory_capabilities::ExternalImageFormatPropertiesNV {
            image_format_properties: self.image_format_properties.into_low_level(context, bump),
            external_memory_features: self.external_memory_features.into_low_level(context, bump),
            export_from_imported_handle_types: self.export_from_imported_handle_types.into_low_level(context, bump),
            compatible_handle_types: self.compatible_handle_types.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for ExternalImageFormatPropertiesNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            image_format_properties: crate::conv::FromLowLevel::from_low_level(context, value.image_format_properties),
            external_memory_features: crate::conv::FromLowLevel::from_low_level(
                context,
                value.external_memory_features,
            ),
            export_from_imported_handle_types: crate::conv::FromLowLevel::from_low_level(
                context,
                value.export_from_imported_handle_types,
            ),
            compatible_handle_types: crate::conv::FromLowLevel::from_low_level(context, value.compatible_handle_types),
        }
    }
}
