use crate::{cstr, vulkan1_3::PhysicalDeviceTextureCompressionAstchdrFeatures};
use std::ffi::CStr;
///See [`PhysicalDeviceTextureCompressionAstchdrFeatures`]
#[doc(alias = "VkPhysicalDeviceTextureCompressionASTCHDRFeaturesEXT")]
pub type PhysicalDeviceTextureCompressionAstchdrFeaturesEXT = PhysicalDeviceTextureCompressionAstchdrFeatures;
#[doc(alias = "VK_EXT_TEXTURE_COMPRESSION_ASTC_HDR_SPEC_VERSION")]
pub const EXT_TEXTURE_COMPRESSION_ASTC_HDR_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_EXT_TEXTURE_COMPRESSION_ASTC_HDR_EXTENSION_NAME")]
pub const EXT_TEXTURE_COMPRESSION_ASTC_HDR_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_texture_compression_astc_hdr");
