//![VK_EXT_texture_compression_astc_hdr](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_texture_compression_astc_hdr.html) - device extension
//!# Description
//!This extension adds support for textures compressed using the Adaptive
//!Scalable Texture Compression (ASTC) High Dynamic Range (HDR) profile.When this extension is
//! enabled, the HDR profile is supported for all ASTC
//!formats listed in [ASTC Compressed Image
//!Formats](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#appendix-compressedtex-astc).
//!# Revision
//!1
//!# Dependencies
//! - *Promoted* to [Vulkan 1.3](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.3-promotions)
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_get_physical_device_properties2`]`
//!# Contacts
//! - Jan-Harald Fredriksen [janharaldfredriksen-arm](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_texture_compression_astc_hdr]
//!   @janharaldfredriksen-arm%0A<<Here describe the issue or question you have about the
//!   VK_EXT_texture_compression_astc_hdr extension>>)
//!# New structures
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  -
//!   [`PhysicalDeviceTextureCompressionAstchdrFeaturesEXT`]
//!# New constants
//! - [`EXT_TEXTURE_COMPRESSION_ASTC_HDR_EXTENSION_NAME`]
//! - [`EXT_TEXTURE_COMPRESSION_ASTC_HDR_SPEC_VERSION`]
//! - Extending [`Format`]:  - `VK_FORMAT_ASTC_10x10_SFLOAT_BLOCK_EXT`  -
//!   `VK_FORMAT_ASTC_10x5_SFLOAT_BLOCK_EXT`  - `VK_FORMAT_ASTC_10x6_SFLOAT_BLOCK_EXT`  -
//!   `VK_FORMAT_ASTC_10x8_SFLOAT_BLOCK_EXT`  - `VK_FORMAT_ASTC_12x10_SFLOAT_BLOCK_EXT`  -
//!   `VK_FORMAT_ASTC_12x12_SFLOAT_BLOCK_EXT`  - `VK_FORMAT_ASTC_4x4_SFLOAT_BLOCK_EXT`  -
//!   `VK_FORMAT_ASTC_5x4_SFLOAT_BLOCK_EXT`  - `VK_FORMAT_ASTC_5x5_SFLOAT_BLOCK_EXT`  -
//!   `VK_FORMAT_ASTC_6x5_SFLOAT_BLOCK_EXT`  - `VK_FORMAT_ASTC_6x6_SFLOAT_BLOCK_EXT`  -
//!   `VK_FORMAT_ASTC_8x5_SFLOAT_BLOCK_EXT`  - `VK_FORMAT_ASTC_8x6_SFLOAT_BLOCK_EXT`  -
//!   `VK_FORMAT_ASTC_8x8_SFLOAT_BLOCK_EXT`
//! - Extending [`StructureType`]:  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TEXTURE_COMPRESSION_ASTC_HDR_FEATURES_EXT`
//!# Known issues & F.A.Q
//!1) Should we add a feature or limit for this functionality?Yes.
//!It is consistent with the ASTC LDR support to add a feature like
//!textureCompressionASTC_HDR.The feature is strictly speaking redundant as long as this is just an
//!extension; it would be sufficient to just enable the extension.
//!But adding the feature is more forward-looking if wanted to make this an
//!optional core feature in the future.2) Should we introduce new format enums for HDR?Yes.
//!Vulkan 1.0 describes the ASTC format enums as UNORM, e.g.
//!`VK_FORMAT_ASTC_4x4_UNORM_BLOCK`, so it is confusing to make these
//!contain HDR data.
//!Note that the OpenGL (ES) extensions did not make this distinction because a
//!single ASTC HDR texture may contain both unorm and float blocks.
//!Implementations  **may**  not be able to distinguish between LDR and HDR ASTC
//!textures internally and just treat them as the same format, i.e. if this
//!extension is supported then sampling from a
//!`VK_FORMAT_ASTC_4x4_UNORM_BLOCK` image format  **may**  return HDR results.
//!Applications  **can**  get predictable results by using the appropriate image
//!format.
//!# Version History
//! - Revision 1, 2019-05-28 (Jan-Harald Fredriksen)  - Initial version
//!# Other info
//! * 2019-05-28
//! * - Promoted to Vulkan 1.3 Core
//! * No known issues.
//! * - Jan-Harald Fredriksen, Arm
//!# Related
//! - [`PhysicalDeviceTextureCompressionAstchdrFeaturesEXT`]
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
#[doc(alias = "VK_EXT_TEXTURE_COMPRESSION_ASTC_HDR_SPEC_VERSION")]
pub const EXT_TEXTURE_COMPRESSION_ASTC_HDR_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_TEXTURE_COMPRESSION_ASTC_HDR_EXTENSION_NAME")]
pub const EXT_TEXTURE_COMPRESSION_ASTC_HDR_EXTENSION_NAME: &'static CStr =
    crate::cstr!("VK_EXT_texture_compression_astc_hdr");
