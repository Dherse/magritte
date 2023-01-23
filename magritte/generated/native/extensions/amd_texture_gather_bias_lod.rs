//!# [VK_AMD_texture_gather_bias_lod](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_AMD_texture_gather_bias_lod.html)
# ! [doc = include_str ! ("../../../../doc/extensions/amd_texture_gather_bias_lod/VK_AMD_texture_gather_bias_lod.md")]
use crate::{
    cstr,
    vulkan1_0::{BaseOutStructure, Bool32, StructureType},
};
use std::ffi::CStr;
///# [VkTextureLODGatherFormatPropertiesAMD](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkTextureLODGatherFormatPropertiesAMD.html)
# [doc = include_str ! ("../../../../doc/extensions/amd_texture_gather_bias_lod/VkTextureLODGatherFormatPropertiesAMD.md")]
#[doc(alias = "VkTextureLODGatherFormatPropertiesAMD")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct TextureLodGatherFormatPropertiesAMD {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "supportsTextureGatherLODBiasAMD")]
    supports_texture_gather_lod_bias_amd: Bool32,
}
#[doc(alias = "VK_AMD_TEXTURE_GATHER_BIAS_LOD_SPEC_VERSION")]
pub const AMD_TEXTURE_GATHER_BIAS_LOD_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_AMD_TEXTURE_GATHER_BIAS_LOD_EXTENSION_NAME")]
pub const AMD_TEXTURE_GATHER_BIAS_LOD_EXTENSION_NAME: &'static CStr = cstr!("VK_AMD_texture_gather_bias_lod");
