//![VK_KHR_storage_buffer_storage_class](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_storage_buffer_storage_class.html) - device extension
//!# Description
//!This extension adds support for the following SPIR-V extension in Vulkan:
//! - `SPV_KHR_storage_buffer_storage_class`
//!This extension provides a new SPIR-V `StorageBuffer` storage class.
//!A `Block`-decorated object in this class is equivalent to a
//!`BufferBlock`-decorated object in the `Uniform` storage class.
//!# Revision
//!1
//!# Dependencies
//! - *Promoted* to [Vulkan 1.1](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.1-promotions)
//!# Dependencies
//! - Requires Vulkan 1.0
//!# Contacts
//! - Alexander Galazin [alegal-arm](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_storage_buffer_storage_class]
//!   @alegal-arm%0A<<Here describe the issue or question you have about the
//!   VK_KHR_storage_buffer_storage_class extension>>)
//!# New constants
//! - [`KHR_STORAGE_BUFFER_STORAGE_CLASS_EXTENSION_NAME`]
//! - [`KHR_STORAGE_BUFFER_STORAGE_CLASS_SPEC_VERSION`]
//!# Version History
//! - Revision 1, 2017-03-23 (Alexander Galazin)  - Initial draft
//!# Other info
//! * 2017-09-05
//! * No known IP claims.
//! * - This extension requires [`SPV_KHR_storage_buffer_storage_class`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/KHR/SPV_KHR_storage_buffer_storage_class.html)
//!   - Promoted to Vulkan 1.1 Core
//! * - Alexander Galazin, ARM  - David Neto, Google
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
#[doc(alias = "VK_KHR_STORAGE_BUFFER_STORAGE_CLASS_SPEC_VERSION")]
pub const KHR_STORAGE_BUFFER_STORAGE_CLASS_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_STORAGE_BUFFER_STORAGE_CLASS_EXTENSION_NAME")]
pub const KHR_STORAGE_BUFFER_STORAGE_CLASS_EXTENSION_NAME: &'static CStr =
    crate::cstr!("VK_KHR_storage_buffer_storage_class");
