use crate::native::vulkan1_0::{BaseOutStructure, Bool32, StructureType};
#[doc(alias = "VkTextureLODGatherFormatPropertiesAMD")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct TextureLodGatherFormatPropertiesAMD {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "supportsTextureGatherLODBiasAMD")]
    pub supports_texture_gather_lod_bias_amd: Bool32,
}
impl Default for TextureLodGatherFormatPropertiesAMD {
    fn default() -> Self {
        Self {
            s_type: StructureType::TextureLodGatherFormatPropertiesAmd,
            p_next: unsafe { std::mem::zeroed() },
            supports_texture_gather_lod_bias_amd: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::amd_texture_gather_bias_lod::{
    AMD_TEXTURE_GATHER_BIAS_LOD_EXTENSION_NAME, AMD_TEXTURE_GATHER_BIAS_LOD_SPEC_VERSION,
};
