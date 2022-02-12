//![VK_EXT_astc_decode_mode](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_astc_decode_mode.html) - device extension
//!# Description
//!The existing specification requires that low dynamic range (LDR) ASTC
//!textures are decompressed to FP16 values per component.
//!In many cases, decompressing LDR textures to a lower precision intermediate
//!result gives acceptable image quality.
//!Source material for LDR textures is typically authored as 8-bit UNORM
//!values, so decoding to FP16 values adds little value.
//!On the other hand, reducing precision of the decoded result reduces the size
//!of the decompressed data, potentially improving texture cache performance
//!and saving power.The goal of this extension is to enable this efficiency gain on existing
//!ASTC texture data.
//!This is achieved by giving the application the ability to select the
//!intermediate decoding precision.Three decoding options are provided:
//! - Decode to `VK_FORMAT_R16G16B16A16_SFLOAT` precision: This is the
//!default, and matches the required behavior in the core API.
//! - Decode to `VK_FORMAT_R8G8B8A8_UNORM` precision: This is provided as
//!an option in LDR mode.
//! - Decode to `VK_FORMAT_E5B9G9R9_UFLOAT_PACK32` precision: This is
//!provided as an option in both LDR and HDR mode.
//!In this mode, negative values cannot be represented and are clamped to
//!zero.
//!The alpha component is ignored, and the results are as if alpha was 1.0.
//!This decode mode is optional and support can be queried via the physical
//!device properties.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_get_physical_device_properties2`]`
//!# Contacts
//! - Jan-Harald Fredriksen [janharaldfredriksen-arm](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_astc_decode_mode]
//!   @janharaldfredriksen-arm%0A<<Here describe the issue or question you have about the
//!   VK_EXT_astc_decode_mode extension>>)
//!# New structures
//! - Extending [`ImageViewCreateInfo`]:
//! - [`ImageViewAstcDecodeModeEXT`]
//!
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:
//! - [`PhysicalDeviceAstcDecodeFeaturesEXT`]
//!# New constants
//! - [`EXT_ASTC_DECODE_MODE_EXTENSION_NAME`]
//! - [`EXT_ASTC_DECODE_MODE_SPEC_VERSION`]
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_IMAGE_VIEW_ASTC_DECODE_MODE_EXT`
//! - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ASTC_DECODE_FEATURES_EXT`
//!# Known issues & F.A.Q
//!1) Are implementations allowed to decode at a higher precision than what is
//!requested?
//!```c
//!RESOLUTION: No.
//!If we allow this, then this extension could be exposed on all
//!implementations that support ASTC.
//!But developers would have no way of knowing what precision was actually
//!used, and thus whether the image quality is sufficient at reduced
//!precision.
//!```
//!2) Should the decode mode be image view state and/or sampler state?
//!```c
//!RESOLUTION: Image view state only.
//!Some implementations treat the different decode modes as different
//!texture formats.
//!```
//!# Version History
//! - Revision 1, 2018-08-07 (Jan-Harald Fredriksen)
//! - Initial revision
//!# Other info
//! * 2018-08-07
//!*
//! - Jan-Harald Fredriksen, Arm
//!# Related
//! - [`ImageViewAstcDecodeModeEXT`]
//! - [`PhysicalDeviceAstcDecodeFeaturesEXT`]
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
#[doc(alias = "VK_EXT_ASTC_DECODE_MODE_SPEC_VERSION")]
pub const EXT_ASTC_DECODE_MODE_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_ASTC_DECODE_MODE_EXTENSION_NAME")]
pub const EXT_ASTC_DECODE_MODE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_astc_decode_mode");
