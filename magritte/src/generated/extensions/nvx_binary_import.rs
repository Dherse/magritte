//![VK_NVX_binary_import](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_NVX_binary_import.html) - device extension
//!# Description
//!This extension allows applications to import CuBIN binaries and execute
//!them.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//!# Contacts
//! - Eric Werness [ewerness-nv](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_NVX_binary_import]
//!   @ewerness-nv%0A<<Here describe the issue or question you have about the VK_NVX_binary_import
//!   extension>>)
//! - Liam Middlebrook [liam-middlebrook](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_NVX_binary_import]
//!   @liam-middlebrook%0A<<Here describe the issue or question you have about the
//!   VK_NVX_binary_import extension>>)
//!# New handles
//! - [`CuFunctionNVX`]
//! - [`CuModuleNVX`]
//!# New functions & commands
//! - [`CmdCuLaunchKernelNVX`]
//! - [`CreateCuFunctionNVX`]
//! - [`CreateCuModuleNVX`]
//! - [`DestroyCuFunctionNVX`]
//! - [`DestroyCuModuleNVX`]
//!# New structures
//! - [`CuFunctionCreateInfoNVX`]
//! - [`CuLaunchInfoNVX`]
//! - [`CuModuleCreateInfoNVX`]
//!# New constants
//! - [`NVX_BINARY_IMPORT_EXTENSION_NAME`]
//! - [`NVX_BINARY_IMPORT_SPEC_VERSION`]
//! - Extending [`DebugReportObjectTypeEXT`]:
//! - `VK_DEBUG_REPORT_OBJECT_TYPE_CU_FUNCTION_NVX_EXT`
//! - `VK_DEBUG_REPORT_OBJECT_TYPE_CU_MODULE_NVX_EXT`
//!
//! - Extending [`ObjectType`]:
//! - `VK_OBJECT_TYPE_CU_FUNCTION_NVX`
//! - `VK_OBJECT_TYPE_CU_MODULE_NVX`
//!
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_CU_FUNCTION_CREATE_INFO_NVX`
//! - `VK_STRUCTURE_TYPE_CU_LAUNCH_INFO_NVX`
//! - `VK_STRUCTURE_TYPE_CU_MODULE_CREATE_INFO_NVX`
//!# Version History
//! - Revision 1, 2021-04-09 (Eric Werness)
//! - Internal revisions
//!# Other info
//! * 2021-04-09
//!*
//! - Eric Werness, NVIDIA
//! - Liam Middlebrook, NVIDIA
//!# Related
//! - [`CuFunctionCreateInfoNVX`]
//! - [`CuFunctionNVX`]
//! - [`CuLaunchInfoNVX`]
//! - [`CuModuleCreateInfoNVX`]
//! - [`CuModuleNVX`]
//! - [`CmdCuLaunchKernelNVX`]
//! - [`CreateCuFunctionNVX`]
//! - [`CreateCuModuleNVX`]
//! - [`DestroyCuFunctionNVX`]
//! - [`DestroyCuModuleNVX`]
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
#[doc(alias = "VK_NVX_BINARY_IMPORT_SPEC_VERSION")]
pub const NVX_BINARY_IMPORT_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NVX_BINARY_IMPORT_EXTENSION_NAME")]
pub const NVX_BINARY_IMPORT_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_NVX_binary_import");
