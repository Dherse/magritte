//![VK_EXT_host_query_reset](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_host_query_reset.html) - device extension
//!# Description
//!This extension adds a new function to reset queries from the host.
//!# Revision
//!1
//!# Dependencies
//! - *Promoted* to
//![Vulkan 1.2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.2-promotions)
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_get_physical_device_properties2`]`
//!# Contacts
//! - Bas Nieuwenhuizen [BNieuwenhuizen](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_host_query_reset]
//!   @BNieuwenhuizen%0A<<Here describe the issue or question you have about the
//!   VK_EXT_host_query_reset extension>>)
//!# New functions & commands
//! - [`ResetQueryPoolEXT`]
//!# New structures
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:
//! - [`PhysicalDeviceHostQueryResetFeaturesEXT`]
//!# New constants
//! - [`EXT_HOST_QUERY_RESET_EXTENSION_NAME`]
//! - [`EXT_HOST_QUERY_RESET_SPEC_VERSION`]
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_HOST_QUERY_RESET_FEATURES_EXT`
//!# Version History
//! - Revision 1, 2019-03-12 (Bas Nieuwenhuizen)
//! - Initial draft
//!# Other info
//! * 2019-03-06
//! * No known IP claims.
//!*
//! - Promoted to Vulkan 1.2 Core
//!
//!*
//! - Bas Nieuwenhuizen, Google
//! - Jason Ekstrand, Intel
//! - Jeff Bolz, NVIDIA
//! - Piers Daniell, NVIDIA
//!# Related
//! - [`PhysicalDeviceHostQueryResetFeaturesEXT`]
//! - [`ResetQueryPoolEXT`]
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
#[doc(alias = "VK_EXT_HOST_QUERY_RESET_SPEC_VERSION")]
pub const EXT_HOST_QUERY_RESET_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_HOST_QUERY_RESET_EXTENSION_NAME")]
pub const EXT_HOST_QUERY_RESET_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_host_query_reset");
