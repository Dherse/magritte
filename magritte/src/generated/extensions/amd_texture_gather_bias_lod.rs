//![VK_AMD_texture_gather_bias_lod](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_AMD_texture_gather_bias_lod.html) - device extension
//!# Description
//!This extension adds two related features.Firstly, support for the following SPIR-V extension in
//! Vulkan is added:
//! - `SPV_AMD_texture_gather_bias_lod`Secondly, the extension allows the application to query which
//!   formats can be
//!used together with the new function prototypes introduced by the SPIR-V
//!extension.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_get_physical_device_properties2`]`
//!# Contacts
//! - Rex Xu [amdrexu](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_AMD_texture_gather_bias_lod]
//!   @amdrexu%0A<<Here describe the issue or question you have about the
//!   VK_AMD_texture_gather_bias_lod extension>>)
//!# New structures
//! - Extending [`ImageFormatProperties2`]:
//! - [`TextureLodGatherFormatPropertiesAMD`]
//!# New constants
//! - [`AMD_TEXTURE_GATHER_BIAS_LOD_EXTENSION_NAME`]
//! - [`AMD_TEXTURE_GATHER_BIAS_LOD_SPEC_VERSION`]
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_TEXTURE_LOD_GATHER_FORMAT_PROPERTIES_AMD`
//!# Version History
//! - Revision 1, 2017-03-21 (Dominik Witczak)
//! - Initial draft
//!# Other info
//! * 2017-03-21
//! * No known IP claims.
//!*
//! - This extension requires
//![`SPV_AMD_texture_gather_bias_lod`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/AMD/SPV_AMD_texture_gather_bias_lod.html)
//! - This extension provides API support for
//![`GL_AMD_texture_gather_bias_lod`](https://www.khronos.org/registry/OpenGL/extensions/AMD/AMD_texture_gather_bias_lod.txt)
//!*
//! - Dominik Witczak, AMD
//! - Daniel Rakos, AMD
//! - Graham Sellers, AMD
//! - Matthaeus G. Chajdas, AMD
//! - Qun Lin, AMD
//! - Rex Xu, AMD
//! - Timothy Lottes, AMD
//!# Related
//! - [`TextureLodGatherFormatPropertiesAMD`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use std::ffi::CStr;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_AMD_TEXTURE_GATHER_BIAS_LOD_SPEC_VERSION")]
pub const AMD_TEXTURE_GATHER_BIAS_LOD_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_AMD_TEXTURE_GATHER_BIAS_LOD_EXTENSION_NAME")]
pub const AMD_TEXTURE_GATHER_BIAS_LOD_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_AMD_texture_gather_bias_lod");
