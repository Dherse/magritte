//![VK_GOOGLE_user_type](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_GOOGLE_user_type.html) - device extension
//!# Description
//!The [`VK_GOOGLE_user_type`] extension allows use of the `SPV_GOOGLE_user_type`
//!extension in SPIR-V shader modules.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//!# Contacts
//! - Kaye Mason [chaleur](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_GOOGLE_user_type]
//!   @chaleur%0A<<Here describe the issue or question you have about the VK_GOOGLE_user_type
//!   extension>>)
//!# New constants
//! - [`GOOGLE_USER_TYPE_EXTENSION_NAME`]
//! - [`GOOGLE_USER_TYPE_SPEC_VERSION`]
//!# Version History
//! - Revision 1, 2019-09-07 (Kaye Mason)
//! - Initial draft
//!# Other info
//! * 2019-07-09
//! * No known IP claims.
//!*
//! - This extension requires
//![`SPV_GOOGLE_user_type`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/GOOGLE/SPV_GOOGLE_user_type.asciidoc)
//!*
//! - Kaye Mason, Google
//! - Hai Nguyen, Google
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
#[doc(alias = "VK_GOOGLE_USER_TYPE_SPEC_VERSION")]
pub const GOOGLE_USER_TYPE_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_GOOGLE_USER_TYPE_EXTENSION_NAME")]
pub const GOOGLE_USER_TYPE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_GOOGLE_user_type");
