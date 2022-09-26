//![VK_AMD_shader_fragment_mask](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_AMD_shader_fragment_mask.html) - device extension
//!# Description
//!This extension provides efficient read access to the fragment mask in
//!compressed multisampled color surfaces.
//!The fragment mask is a lookup table that associates color samples with color
//!fragment values.From a shader, the fragment mask can be fetched with a call to
//!`fragmentMaskFetchAMD`, which returns a single `uint` where each
//!subsequent four bits specify the color fragment index corresponding to the
//!color sample, starting from the least significant bit.
//!For example, when eight color samples are used, the color fragment index for
//!color sample 0 will be in bits 0-3 of the fragment mask, for color sample 7
//!the index will be in bits 28-31.The color fragment for a particular color sample may then be
//! fetched with
//!the corresponding fragment mask value using the `fragmentFetchAMD` shader
//!function.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//!# Contacts
//! - Aaron Hagan [AaronHaganAMD](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_AMD_shader_fragment_mask]
//!   @AaronHaganAMD%0A<<Here describe the issue or question you have about the
//!   VK_AMD_shader_fragment_mask extension>>)
//!# New constants
//! - [`AMD_SHADER_FRAGMENT_MASK_EXTENSION_NAME`]
//! - [`AMD_SHADER_FRAGMENT_MASK_SPEC_VERSION`]
//!# Version history
//! - Revision 1, 2017-08-16 (Aaron Hagan)  - Initial draft
//!# Other information
//! * 2017-08-16
//! * No known IP claims.
//! * - This extension requires [`SPV_AMD_shader_fragment_mask`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/AMD/SPV_AMD_shader_fragment_mask.html)
//!   - This extension provides API support for [`GL_AMD_shader_fragment_mask`](https://github.com/KhronosGroup/GLSL/blob/master/extensions/amd/GL_AMD_shader_fragment_mask.txt)
//! * - Aaron Hagan, AMD  - Daniel Rakos, AMD  - Timothy Lottes, AMD
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
#[doc(alias = "VK_AMD_SHADER_FRAGMENT_MASK_SPEC_VERSION")]
pub const AMD_SHADER_FRAGMENT_MASK_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_AMD_SHADER_FRAGMENT_MASK_EXTENSION_NAME")]
pub const AMD_SHADER_FRAGMENT_MASK_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_AMD_shader_fragment_mask");
