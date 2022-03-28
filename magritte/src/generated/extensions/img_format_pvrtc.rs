//![VK_IMG_format_pvrtc](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_IMG_format_pvrtc.html) - device extension
//!# Description
//![`VK_IMG_format_pvrtc`] provides additional texture compression functionality
//!specific to Imagination Technologies PowerVR Texture compression format
//!(called PVRTC).
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//!# Contacts
//! - Stuart Smith
//!# New constants
//! - [`IMG_FORMAT_PVRTC_EXTENSION_NAME`]
//! - [`IMG_FORMAT_PVRTC_SPEC_VERSION`]
//! - Extending [`Format`]:  - `VK_FORMAT_PVRTC1_2BPP_SRGB_BLOCK_IMG`  -
//!   `VK_FORMAT_PVRTC1_2BPP_UNORM_BLOCK_IMG`  - `VK_FORMAT_PVRTC1_4BPP_SRGB_BLOCK_IMG`  -
//!   `VK_FORMAT_PVRTC1_4BPP_UNORM_BLOCK_IMG`  - `VK_FORMAT_PVRTC2_2BPP_SRGB_BLOCK_IMG`  -
//!   `VK_FORMAT_PVRTC2_2BPP_UNORM_BLOCK_IMG`  - `VK_FORMAT_PVRTC2_4BPP_SRGB_BLOCK_IMG`  -
//!   `VK_FORMAT_PVRTC2_4BPP_UNORM_BLOCK_IMG`
//!# Version History
//! - Revision 1, 2019-09-02 (Stuart Smith)  - Initial version
//!# Other info
//! * 2019-09-02
//! * Imagination Technologies Proprietary
//! * - Stuart Smith, Imagination Technologies
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
#[doc(alias = "VK_IMG_FORMAT_PVRTC_SPEC_VERSION")]
pub const IMG_FORMAT_PVRTC_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_IMG_FORMAT_PVRTC_EXTENSION_NAME")]
pub const IMG_FORMAT_PVRTC_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_IMG_format_pvrtc");
