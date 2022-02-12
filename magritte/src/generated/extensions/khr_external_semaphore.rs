//![VK_KHR_external_semaphore](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_external_semaphore.html) - device extension
//!# Description
//!An application using external memory may wish to synchronize access to that
//!memory using semaphores.
//!This extension enables an application to create semaphores from which
//!non-Vulkan handles that reference the underlying synchronization primitive
//!can be exported.
//!# Revision
//!1
//!# Dependencies
//! - *Promoted* to
//![Vulkan 1.1](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.1-promotions)
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_external_semaphore_capabilities`]`
//!# Contacts
//! - James Jones [cubanismo](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_external_semaphore]
//!   @cubanismo%0A<<Here describe the issue or question you have about the
//!   VK_KHR_external_semaphore extension>>)
//!# New structures
//! - Extending [`SemaphoreCreateInfo`]:
//! - [`ExportSemaphoreCreateInfoKHR`]
//!# New enums
//! - [`SemaphoreImportFlagBitsKHR`]
//!# New bitmasks
//! - [`SemaphoreImportFlagsKHR`]
//!# New constants
//! - [`KHR_EXTERNAL_SEMAPHORE_EXTENSION_NAME`]
//! - [`KHR_EXTERNAL_SEMAPHORE_SPEC_VERSION`]
//! - Extending [`SemaphoreImportFlagBits`]:
//! - `VK_SEMAPHORE_IMPORT_TEMPORARY_BIT_KHR`
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_EXPORT_SEMAPHORE_CREATE_INFO_KHR`
//!# Known issues & F.A.Q
//!1) Should there be restrictions on what side effects can occur when waiting
//!on imported semaphores that are in an invalid state?**RESOLVED**: Yes.
//!Normally, validating such state would be the responsibility of the
//!application, and the implementation would be free to enter an undefined
//!state if valid usage rules were violated.
//!However, this could cause security concerns when using imported semaphores,
//!as it would require the importing application to trust the exporting
//!application to ensure the state is valid.
//!Requiring this level of trust is undesirable for many potential use cases.2) Must
//! implementations validate external handles the application provides
//!as input to semaphore state import operations?**RESOLVED**: Implementations must return an error
//! to the application if the
//!provided semaphore state handle cannot be used to complete the requested
//!import operation.
//!However, implementations need not validate handles are of the exact type
//!specified by the application.
//!# Version History
//! - Revision 1, 2016-10-21 (James Jones)
//! - Initial revision
//!# Other info
//! * 2016-10-21
//! * No known IP claims.
//!*
//! - Promoted to Vulkan 1.1 Core
//!*
//! - Jason Ekstrand, Intel
//! - Jesse Hall, Google
//! - Tobias Hector, Imagination Technologies
//! - James Jones, NVIDIA
//! - Jeff Juliano, NVIDIA
//! - Matthew Netsch, Qualcomm Technologies, Inc.
//! - Ray Smith, ARM
//! - Chad Versace, Google
//!# Related
//! - [`ExportSemaphoreCreateInfoKHR`]
//! - [`SemaphoreImportFlagBitsKHR`]
//! - [`SemaphoreImportFlagsKHR`]
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
#[doc(alias = "VK_KHR_EXTERNAL_SEMAPHORE_SPEC_VERSION")]
pub const KHR_EXTERNAL_SEMAPHORE_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_EXTERNAL_SEMAPHORE_EXTENSION_NAME")]
pub const KHR_EXTERNAL_SEMAPHORE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_external_semaphore");
