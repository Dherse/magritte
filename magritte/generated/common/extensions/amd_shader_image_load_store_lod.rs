use crate::cstr;
use std::ffi::CStr;
#[doc(alias = "VK_AMD_SHADER_IMAGE_LOAD_STORE_LOD_SPEC_VERSION")]
pub const AMD_SHADER_IMAGE_LOAD_STORE_LOD_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_AMD_SHADER_IMAGE_LOAD_STORE_LOD_EXTENSION_NAME")]
pub const AMD_SHADER_IMAGE_LOAD_STORE_LOD_EXTENSION_NAME: &'static CStr = cstr!("VK_AMD_shader_image_load_store_lod");
