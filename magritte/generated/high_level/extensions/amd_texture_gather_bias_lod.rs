pub use crate::common::extensions::amd_texture_gather_bias_lod::{
    AMD_TEXTURE_GATHER_BIAS_LOD_EXTENSION_NAME, AMD_TEXTURE_GATHER_BIAS_LOD_SPEC_VERSION,
};
use crate::{context::Context, vulkan1_0::StructureType};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkTextureLODGatherFormatPropertiesAMD")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TextureLodGatherFormatPropertiesAMD {
    #[doc(alias = "supportsTextureGatherLODBiasAMD")]
    pub supports_texture_gather_lod_bias_amd: bool,
}
impl TextureLodGatherFormatPropertiesAMD {
    ///Get a reference to the `supports_texture_gather_lod_bias_amd` field.
    pub fn supports_texture_gather_lod_bias_amd(&self) -> &bool {
        &self.supports_texture_gather_lod_bias_amd
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for TextureLodGatherFormatPropertiesAMD {
    type LowLevel = crate::native::extensions::amd_texture_gather_bias_lod::TextureLodGatherFormatPropertiesAMD;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::amd_texture_gather_bias_lod::TextureLodGatherFormatPropertiesAMD {
            s_type: StructureType::TextureLodGatherFormatPropertiesAmd,
            p_next: std::ptr::null_mut(),
            supports_texture_gather_lod_bias_amd: self
                .supports_texture_gather_lod_bias_amd
                .into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for TextureLodGatherFormatPropertiesAMD {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            supports_texture_gather_lod_bias_amd: crate::conv::FromLowLevel::from_low_level(
                context,
                value.supports_texture_gather_lod_bias_amd,
            ),
        }
    }
}
