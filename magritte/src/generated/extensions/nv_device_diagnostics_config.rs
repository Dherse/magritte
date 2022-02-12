//![VK_NV_device_diagnostics_config](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_NV_device_diagnostics_config.html) - device extension
//!# Description
//!Applications using Nvidia Nsight<sup>™</sup> Aftermath SDK for Vulkan to integrate
//!device crash dumps into their error reporting mechanisms, **may** use this
//!extension to configure options related to device crash dump creation.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_get_physical_device_properties2`]`
//!# Contacts
//! - Kedarnath Thangudu [kthangudu](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_NV_device_diagnostics_config]
//!   @kthangudu%0A<<Here describe the issue or question you have about the
//!   VK_NV_device_diagnostics_config extension>>)
//!# New structures
//! - Extending [`DeviceCreateInfo`]:
//! - [`DeviceDiagnosticsConfigCreateInfoNV`]
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:
//! - [`PhysicalDeviceDiagnosticsConfigFeaturesNV`]
//!# New enums
//! - [`DeviceDiagnosticsConfigFlagBitsNV`]
//!# New bitmasks
//! - [`DeviceDiagnosticsConfigFlagsNV`]
//!# New constants
//! - [`NV_DEVICE_DIAGNOSTICS_CONFIG_EXTENSION_NAME`]
//! - [`NV_DEVICE_DIAGNOSTICS_CONFIG_SPEC_VERSION`]
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_DEVICE_DIAGNOSTICS_CONFIG_CREATE_INFO_NV`
//! - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DIAGNOSTICS_CONFIG_FEATURES_NV`
//!# Version History
//! - Revision 1, 2019-11-21 (Kedarnath Thangudu)
//! - Internal revisions
//!# Other info
//! * 2019-12-15
//!*
//! - Kedarnath Thangudu, NVIDIA
//! - Thomas Klein, NVIDIA
//!# Related
//! - [`DeviceDiagnosticsConfigCreateInfoNV`]
//! - [`DeviceDiagnosticsConfigFlagBitsNV`]
//! - [`DeviceDiagnosticsConfigFlagsNV`]
//! - [`PhysicalDeviceDiagnosticsConfigFeaturesNV`]
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
#[doc(alias = "VK_NV_DEVICE_DIAGNOSTICS_CONFIG_SPEC_VERSION")]
pub const NV_DEVICE_DIAGNOSTICS_CONFIG_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_DEVICE_DIAGNOSTICS_CONFIG_EXTENSION_NAME")]
pub const NV_DEVICE_DIAGNOSTICS_CONFIG_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_NV_device_diagnostics_config");
