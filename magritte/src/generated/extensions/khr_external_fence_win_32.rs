//![VK_KHR_external_fence_win32](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_external_fence_win32.html) - device extension
//!# Description
//!An application using external memory may wish to synchronize access to that
//!memory using fences.
//!This extension enables an application to export fence payload to and import
//!fence payload from Windows handles.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_external_fence`]`
//!# Contacts
//! - Jesse Hall [critsec](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_external_fence_win32]
//!   @critsec%0A<<Here describe the issue or question you have about the
//!   VK_KHR_external_fence_win32 extension>>)
//!# New functions & commands
//! - [`GetFenceWin32HandleKHR`]
//! - [`ImportFenceWin32HandleKHR`]
//!# New structures
//! - [`FenceGetWin32HandleInfoKHR`]
//! - [`ImportFenceWin32HandleInfoKHR`]
//! - Extending [`FenceCreateInfo`]:
//! - [`ExportFenceWin32HandleInfoKHR`]
//!# New constants
//! - [`KHR_EXTERNAL_FENCE_WIN32_EXTENSION_NAME`]
//! - [`KHR_EXTERNAL_FENCE_WIN32_SPEC_VERSION`]
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_EXPORT_FENCE_WIN32_HANDLE_INFO_KHR`
//! - `VK_STRUCTURE_TYPE_FENCE_GET_WIN32_HANDLE_INFO_KHR`
//! - `VK_STRUCTURE_TYPE_IMPORT_FENCE_WIN32_HANDLE_INFO_KHR`
//!# Known issues & F.A.Q
//!This extension borrows concepts, semantics, and language from
//!`[`VK_KHR_external_semaphore_win32`]`.
//!That extension’s issues apply equally to this extension.1) Should D3D12 fence handle types be
//! supported, like they are for
//!semaphores?**RESOLVED**: No.
//!Doing so would require extending the fence signal and wait operations to
//!provide values to signal / wait for, like [`D3D12FenceSubmitInfoKHR`]
//!does.
//!A D3D12 fence can be signaled by importing it into a [`Semaphore`]
//!instead of a [`Fence`], and applications can check status or wait on the
//!D3D12 fence using non-Vulkan APIs.
//!The convenience of being able to do these operations on [`Fence`]
//!objects does not justify the extra API complexity.
//!# Version History
//! - Revision 1, 2017-05-08 (Jesse Hall)
//! - Initial revision
//!# Other info
//! * 2017-05-08
//! * No known IP claims.
//!*
//! - Jesse Hall, Google
//! - James Jones, NVIDIA
//! - Jeff Juliano, NVIDIA
//! - Cass Everitt, Oculus
//! - Contributors to `[`VK_KHR_external_semaphore_win32`]`
//!# Related
//! - [`ExportFenceWin32HandleInfoKHR`]
//! - [`FenceGetWin32HandleInfoKHR`]
//! - [`ImportFenceWin32HandleInfoKHR`]
//! - [`GetFenceWin32HandleKHR`]
//! - [`ImportFenceWin32HandleKHR`]
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
#[doc(alias = "VK_KHR_EXTERNAL_FENCE_WIN32_SPEC_VERSION")]
pub const KHR_EXTERNAL_FENCE_WIN32_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_EXTERNAL_FENCE_WIN32_EXTENSION_NAME")]
pub const KHR_EXTERNAL_FENCE_WIN32_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_external_fence_win32");
