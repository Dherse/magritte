//![VK_KHR_bind_memory2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_bind_memory2.html) - device extension
//!# Description
//!This extension provides versions of [`BindBufferMemory`] and
//![`BindImageMemory`] that allow multiple bindings to be performed at
//!once, and are extensible.This extension also introduces `VK_IMAGE_CREATE_ALIAS_BIT_KHR`, which
//!allows “identical” images that alias the same memory to interpret the
//!contents consistently, even across image layout changes.
//!# Revision
//!1
//!# Dependencies
//! - *Promoted* to [Vulkan 1.1](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.1-promotions)
//!# Dependencies
//! - Requires Vulkan 1.0
//!# Contacts
//! - Tobias Hector [tobski](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_bind_memory2]
//!   @tobski%0A<<Here describe the issue or question you have about the VK_KHR_bind_memory2
//!   extension>>)
//!# New functions & commands
//! - [`BindBufferMemory2KHR`]
//! - [`BindImageMemory2KHR`]
//!# New structures
//! - [`BindBufferMemoryInfoKHR`]
//! - [`BindImageMemoryInfoKHR`]
//!# New constants
//! - [`KHR_BIND_MEMORY_2_EXTENSION_NAME`]
//! - [`KHR_BIND_MEMORY_2_SPEC_VERSION`]
//! - Extending [`ImageCreateFlagBits`]:  - `VK_IMAGE_CREATE_ALIAS_BIT_KHR`
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_BIND_BUFFER_MEMORY_INFO_KHR`  -
//!   `VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_INFO_KHR`
//!# Version History
//! - Revision 1, 2017-05-19 (Tobias Hector)  - Pulled bind memory functions into their own
//!   extension
//!# Other info
//! * 2017-09-05
//! * No known IP claims.
//! * - Promoted to Vulkan 1.1 Core
//! * - Jeff Bolz, NVIDIA  - Tobias Hector, Imagination Technologies
//!# Related
//! - [`BindBufferMemoryInfoKHR`]
//! - [`BindImageMemoryInfoKHR`]
//! - [`BindBufferMemory2KHR`]
//! - [`BindImageMemory2KHR`]
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
#[doc(alias = "VK_KHR_BIND_MEMORY_2_SPEC_VERSION")]
pub const KHR_BIND_MEMORY_2_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_BIND_MEMORY_2_EXTENSION_NAME")]
pub const KHR_BIND_MEMORY_2_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_bind_memory2");
