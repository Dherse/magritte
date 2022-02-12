//![VK_KHR_external_memory_fd](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_external_memory_fd.html) - device extension
//!# Description
//!An application may wish to reference device memory in multiple Vulkan
//!logical devices or instances, in multiple processes, and/or in multiple
//!APIs.
//!This extension enables an application to export POSIX file descriptor
//!handles from Vulkan memory objects and to import Vulkan memory objects from
//!POSIX file descriptor handles exported from other Vulkan memory objects or
//!from similar resources in other APIs.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_external_memory`]`
//!# Contacts
//! - James Jones [cubanismo](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_external_memory_fd]
//!   @cubanismo%0A<<Here describe the issue or question you have about the
//!   VK_KHR_external_memory_fd extension>>)
//!# New functions & commands
//! - [`GetMemoryFdKHR`]
//! - [`GetMemoryFdPropertiesKHR`]
//!# New structures
//! - [`MemoryFdPropertiesKHR`]
//! - [`MemoryGetFdInfoKHR`]
//! - Extending [`MemoryAllocateInfo`]:
//! - [`ImportMemoryFdInfoKHR`]
//!# New constants
//! - [`KHR_EXTERNAL_MEMORY_FD_EXTENSION_NAME`]
//! - [`KHR_EXTERNAL_MEMORY_FD_SPEC_VERSION`]
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_IMPORT_MEMORY_FD_INFO_KHR`
//! - `VK_STRUCTURE_TYPE_MEMORY_FD_PROPERTIES_KHR`
//! - `VK_STRUCTURE_TYPE_MEMORY_GET_FD_INFO_KHR`
//!# Known issues & F.A.Q
//!1) Does the application need to close the file descriptor returned by
//![`GetMemoryFdKHR`]?**RESOLVED**: Yes, unless it is passed back in to a driver instance to import
//!the memory.
//!A successful get call transfers ownership of the file descriptor to the
//!application, and a successful import transfers it back to the driver.
//!Destroying the original memory object will not close the file descriptor or
//!remove its reference to the underlying memory resource associated with it.2) Do drivers ever
//! need to expose multiple file descriptors per memory
//!object?**RESOLVED**: No.
//!This would indicate there are actually multiple memory objects, rather than
//!a single memory object.3) How should the valid size and memory type for POSIX file descriptor
//!memory handles created outside of Vulkan be specified?**RESOLVED**: The valid memory types are
//! queried directly from the external
//!handle.
//!The size will be specified by future extensions that introduce such external
//!memory handle types.
//!# Version History
//! - Revision 1, 2016-10-21 (James Jones)
//! - Initial revision
//!# Other info
//! * 2016-10-21
//! * No known IP claims.
//!*
//! - James Jones, NVIDIA
//! - Jeff Juliano, NVIDIA
//!# Related
//! - [`ImportMemoryFdInfoKHR`]
//! - [`MemoryFdPropertiesKHR`]
//! - [`MemoryGetFdInfoKHR`]
//! - [`GetMemoryFdKHR`]
//! - [`GetMemoryFdPropertiesKHR`]
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
#[doc(alias = "VK_KHR_EXTERNAL_MEMORY_FD_SPEC_VERSION")]
pub const KHR_EXTERNAL_MEMORY_FD_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_EXTERNAL_MEMORY_FD_EXTENSION_NAME")]
pub const KHR_EXTERNAL_MEMORY_FD_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_external_memory_fd");
