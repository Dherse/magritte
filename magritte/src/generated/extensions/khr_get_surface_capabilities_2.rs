//![VK_KHR_get_surface_capabilities2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_get_surface_capabilities2.html) - instance extension
//!# Description
//!This extension provides new entry points to query device surface
//!capabilities in a way that can be easily extended by other extensions,
//!without introducing any further entry points.
//!This extension can be considered the `[`VK_KHR_surface`]` equivalent of
//!the `[`VK_KHR_get_physical_device_properties2`]` extension.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_surface`]`
//!# Contacts
//! - James Jones [cubanismo](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_get_surface_capabilities2]
//!   @cubanismo%0A<<Here describe the issue or question you have about the
//!   VK_KHR_get_surface_capabilities2 extension>>)
//!# New functions & commands
//! - [`GetPhysicalDeviceSurfaceCapabilities2KHR`]
//! - [`GetPhysicalDeviceSurfaceFormats2KHR`]
//!# New structures
//! - [`PhysicalDeviceSurfaceInfo2KHR`]
//! - [`SurfaceCapabilities2KHR`]
//! - [`SurfaceFormat2KHR`]
//!# New constants
//! - [`KHR_GET_SURFACE_CAPABILITIES_2_EXTENSION_NAME`]
//! - [`KHR_GET_SURFACE_CAPABILITIES_2_SPEC_VERSION`]
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SURFACE_INFO_2_KHR`
//! - `VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES_2_KHR`
//! - `VK_STRUCTURE_TYPE_SURFACE_FORMAT_2_KHR`
//!# Known issues & F.A.Q
//!1) What should this extension be named?**RESOLVED**: [`VK_KHR_get_surface_capabilities2`].
//!Other alternatives:
//! - `VK_KHR_surface2`
//! - One extension, combining a separate display-specific query extension.2) Should additional WSI
//!   query functions be extended?**RESOLVED**:
//! - [`GetPhysicalDeviceSurfaceCapabilitiesKHR`]: Yes.
//!The need for this motivated the extension.
//! - [`GetPhysicalDeviceSurfaceSupportKHR`]: No.
//!Currently only has boolean output.
//!Extensions should instead extend
//![`GetPhysicalDeviceSurfaceCapabilities2KHR`].
//! - [`GetPhysicalDeviceSurfaceFormatsKHR`]: Yes.
//! - [`GetPhysicalDeviceSurfacePresentModesKHR`]: No.
//!Recent discussion concluded this introduced too much variability for
//!applications to deal with.
//!Extensions should instead extend
//![`GetPhysicalDeviceSurfaceCapabilities2KHR`].
//! - [`GetPhysicalDeviceXlibPresentationSupportKHR`]: Not in this
//!extension.
//! - [`GetPhysicalDeviceXcbPresentationSupportKHR`]: Not in this
//!extension.
//! - [`GetPhysicalDeviceWaylandPresentationSupportKHR`]: Not in this
//!extension.
//! - [`GetPhysicalDeviceWin32PresentationSupportKHR`]: Not in this
//!extension.
//!# Version History
//! - Revision 1, 2017-02-27 (James Jones)
//! - Initial draft.
//!# Other info
//! * 2017-02-27
//! * No known IP claims.
//!*
//! - Ian Elliott, Google
//! - James Jones, NVIDIA
//! - Alon Or-bach, Samsung
//!# Related
//! - [`PhysicalDeviceSurfaceInfo2KHR`]
//! - [`SurfaceCapabilities2KHR`]
//! - [`SurfaceFormat2KHR`]
//! - [`GetPhysicalDeviceSurfaceCapabilities2KHR`]
//! - [`GetPhysicalDeviceSurfaceFormats2KHR`]
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
#[doc(alias = "VK_KHR_GET_SURFACE_CAPABILITIES_2_SPEC_VERSION")]
pub const KHR_GET_SURFACE_CAPABILITIES_2_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_GET_SURFACE_CAPABILITIES_2_EXTENSION_NAME")]
pub const KHR_GET_SURFACE_CAPABILITIES_2_EXTENSION_NAME: &'static CStr =
    crate::cstr!("VK_KHR_get_surface_capabilities2");
