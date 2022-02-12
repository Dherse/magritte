//![VK_FUCHSIA_external_memory](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_FUCHSIA_external_memory.html) - device extension
//!# Description
//!Vulkan apps may wish to export or import device memory handles to or from
//!other logical devices, instances or APIs.This memory sharing can eliminate copies of memory
//! buffers when different
//!subsystems need to interoperate on them.
//!Sharing memory buffers may also facilitate a better distribution of
//!processing workload for more complex memory manipulation pipelines.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_external_memory_capabilities`]`
//! - Requires `[`VK_KHR_external_memory`]`
//!# Contacts
//! - John Rosasco [rosasco](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_FUCHSIA_external_memory]
//!   @rosasco%0A<<Here describe the issue or question you have about the VK_FUCHSIA_external_memory
//!   extension>>)
//!# New functions & commands
//! - [`GetMemoryZirconHandleFUCHSIA`]
//! - [`GetMemoryZirconHandlePropertiesFUCHSIA`]
//!# New structures
//! - [`MemoryGetZirconHandleInfoFUCHSIA`]
//! - [`MemoryZirconHandlePropertiesFUCHSIA`]
//! - Extending [`MemoryAllocateInfo`]:
//! - [`ImportMemoryZirconHandleInfoFUCHSIA`]
//!# New constants
//! - [`FUCHSIA_EXTERNAL_MEMORY_EXTENSION_NAME`]
//! - [`FUCHSIA_EXTERNAL_MEMORY_SPEC_VERSION`]
//! - Extending [`ExternalMemoryHandleTypeFlagBits`]:
//! - `VK_EXTERNAL_MEMORY_HANDLE_TYPE_ZIRCON_VMO_BIT_FUCHSIA`
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_IMPORT_MEMORY_ZIRCON_HANDLE_INFO_FUCHSIA`
//! - `VK_STRUCTURE_TYPE_MEMORY_GET_ZIRCON_HANDLE_INFO_FUCHSIA`
//! - `VK_STRUCTURE_TYPE_MEMORY_ZIRCON_HANDLE_PROPERTIES_FUCHSIA`
//!# Known issues & F.A.Q
//!See `[`VK_KHR_external_memory`]` issues list for further information.
//!# Version History
//! - Revision 1, 2021-03-01 (John Rosasco)
//! - Initial draft
//!# Other info
//! * 2021-03-01
//! * No known IP claims.
//!*
//! - Craig Stout, Google
//! - John Bauman, Google
//! - John Rosasco, Google
//!# Related
//! - [`ImportMemoryZirconHandleInfoFUCHSIA`]
//! - [`MemoryGetZirconHandleInfoFUCHSIA`]
//! - [`MemoryZirconHandlePropertiesFUCHSIA`]
//! - [`GetMemoryZirconHandleFUCHSIA`]
//! - [`GetMemoryZirconHandlePropertiesFUCHSIA`]
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
#[doc(alias = "VK_FUCHSIA_EXTERNAL_MEMORY_SPEC_VERSION")]
pub const FUCHSIA_EXTERNAL_MEMORY_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_FUCHSIA_EXTERNAL_MEMORY_EXTENSION_NAME")]
pub const FUCHSIA_EXTERNAL_MEMORY_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_FUCHSIA_external_memory");
