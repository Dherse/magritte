//![VK_FUCHSIA_external_semaphore](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_FUCHSIA_external_semaphore.html) - device extension
//!# Description
//!An application using external memory may wish to synchronize access to that
//!memory using semaphores.
//!This extension enables an application to export semaphore payload to and
//!import semaphore payload from Zircon event handles.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_external_semaphore_capabilities`]`
//! - Requires `[`VK_KHR_external_semaphore`]`
//!# Contacts
//! - John Rosasco [rosasco](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_FUCHSIA_external_semaphore]
//!   @rosasco%0A<<Here describe the issue or question you have about the
//!   VK_FUCHSIA_external_semaphore extension>>)
//!# New functions & commands
//! - [`GetSemaphoreZirconHandleFUCHSIA`]
//! - [`ImportSemaphoreZirconHandleFUCHSIA`]
//!# New structures
//! - [`ImportSemaphoreZirconHandleInfoFUCHSIA`]
//! - [`SemaphoreGetZirconHandleInfoFUCHSIA`]
//!# New constants
//! - [`FUCHSIA_EXTERNAL_SEMAPHORE_EXTENSION_NAME`]
//! - [`FUCHSIA_EXTERNAL_SEMAPHORE_SPEC_VERSION`]
//! - Extending [`ExternalSemaphoreHandleTypeFlagBits`]:
//! - `VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_ZIRCON_EVENT_BIT_FUCHSIA`
//!
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_IMPORT_SEMAPHORE_ZIRCON_HANDLE_INFO_FUCHSIA`
//! - `VK_STRUCTURE_TYPE_SEMAPHORE_GET_ZIRCON_HANDLE_INFO_FUCHSIA`
//!# Known issues & F.A.Q
//!1) Does the application need to close the Zircon event handle returned by
//![`GetSemaphoreZirconHandleFUCHSIA`]?**RESOLVED**: Yes, unless it is passed back in to a driver
//! instance to import
//!the semaphore.
//!A successful get call transfers ownership of the Zircon event handle to the
//!application, and a successful import transfers it back to the driver.
//!Destroying the original semaphore object will not close the Zircon event
//!handle nor remove its reference to the underlying semaphore resource
//!associated with it.
//!# Version History
//! - Revision 1, 2021-03-08 (John Rosasco)
//! - Initial revision
//!# Other info
//! * 2021-03-08
//! * No known IP claims.
//!*
//! - Craig Stout, Google
//! - John Bauman, Google
//! - John Rosasco, Google
//!# Related
//! - [`ImportSemaphoreZirconHandleInfoFUCHSIA`]
//! - [`SemaphoreGetZirconHandleInfoFUCHSIA`]
//! - [`GetSemaphoreZirconHandleFUCHSIA`]
//! - [`ImportSemaphoreZirconHandleFUCHSIA`]
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
#[doc(alias = "VK_FUCHSIA_EXTERNAL_SEMAPHORE_SPEC_VERSION")]
pub const FUCHSIA_EXTERNAL_SEMAPHORE_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_FUCHSIA_EXTERNAL_SEMAPHORE_EXTENSION_NAME")]
pub const FUCHSIA_EXTERNAL_SEMAPHORE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_FUCHSIA_external_semaphore");
