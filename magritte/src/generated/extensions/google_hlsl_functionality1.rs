//![VK_GOOGLE_hlsl_functionality1](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_GOOGLE_hlsl_functionality1.html) - device extension
//!# Description
//!The [`VK_GOOGLE_hlsl_functionality1`] extension allows use of the
//!`SPV_GOOGLE_hlsl_functionality1` extension in SPIR-V shader modules.
# ! [doc = concat ! ("# " , "Revision")]
//!1
# ! [doc = concat ! ("# " , "Dependencies")]
//! - Requires Vulkan 1.0
# ! [doc = concat ! ("# " , "Contacts")]
//! - Hai Nguyen [chaoticbob](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_GOOGLE_hlsl_functionality1]
//!   @chaoticbob%0A<<Here describe the issue or question you have about the
//!   VK_GOOGLE_hlsl_functionality1 extension>>)
# ! [doc = concat ! ("# " , "New constants")]
//! - [`GOOGLE_HLSL_FUNCTIONALITY1_EXTENSION_NAME`]
//! - [`GOOGLE_HLSL_FUNCTIONALITY1_SPEC_VERSION`]
//! - [`GOOGLE_HLSL_FUNCTIONALITY_1_EXTENSION_NAME`]
//! - [`GOOGLE_HLSL_FUNCTIONALITY_1_SPEC_VERSION`]
# ! [doc = concat ! ("# " , "Version history")]
//! - Revision 1, 2018-07-09 (Neil Henning)  - Initial draft
//!# Other info
//! * 2018-07-09
//! * No known IP claims.
//! * - This extension requires [`SPV_GOOGLE_hlsl_functionality1`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/GOOGLE/SPV_GOOGLE_hlsl_functionality1.html)
//! * - Hai Nguyen, Google  - Neil Henning, AMD
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
#[doc(alias = "VK_GOOGLE_HLSL_FUNCTIONALITY_1_SPEC_VERSION")]
pub const GOOGLE_HLSL_FUNCTIONALITY_1_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_GOOGLE_HLSL_FUNCTIONALITY_1_EXTENSION_NAME")]
pub const GOOGLE_HLSL_FUNCTIONALITY_1_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_GOOGLE_hlsl_functionality1");
