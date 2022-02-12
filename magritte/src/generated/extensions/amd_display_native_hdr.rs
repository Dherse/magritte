//![VK_AMD_display_native_hdr](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_AMD_display_native_hdr.html) - device extension
//!# Description
//!This extension introduces the following display native HDR features to
//!Vulkan:
//! - A new [`ColorSpaceKHR`] enum for setting the native display
//!colorspace.
//!For example, this color space would be set by the swapchain to use the
//!native color space in Freesync2 displays.
//! - Local dimming control
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_get_physical_device_properties2`]`
//! - Requires `[`VK_KHR_get_surface_capabilities2`]`
//! - Requires `[`VK_KHR_swapchain`]`
//!# Contacts
//! - Matthaeus G. Chajdas [anteru](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_AMD_display_native_hdr]
//!   @anteru%0A<<Here describe the issue or question you have about the VK_AMD_display_native_hdr
//!   extension>>)
//!# New functions & commands
//! - [`SetLocalDimmingAMD`]
//!# New structures
//! - Extending [`SurfaceCapabilities2KHR`]:
//! - [`DisplayNativeHdrSurfaceCapabilitiesAMD`]
//! - Extending [`SwapchainCreateInfoKHR`]:
//! - [`SwapchainDisplayNativeHdrCreateInfoAMD`]
//!# New constants
//! - [`AMD_DISPLAY_NATIVE_HDR_EXTENSION_NAME`]
//! - [`AMD_DISPLAY_NATIVE_HDR_SPEC_VERSION`]
//! - Extending [`ColorSpaceKHR`]:
//! - `VK_COLOR_SPACE_DISPLAY_NATIVE_AMD`
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_DISPLAY_NATIVE_HDR_SURFACE_CAPABILITIES_AMD`
//! - `VK_STRUCTURE_TYPE_SWAPCHAIN_DISPLAY_NATIVE_HDR_CREATE_INFO_AMD`
//!# Known issues & F.A.Q
//!None.
//!# Version History
//! - Revision 1, 2018-12-18 (Daniel Rakos)
//! - Initial revision
//!# Other info
//! * 2018-12-18
//! * No known IP claims.
//!*
//! - Matthaeus G. Chajdas, AMD
//! - Aaron Hagan, AMD
//! - Aric Cyr, AMD
//! - Timothy Lottes, AMD
//! - Derrick Owens, AMD
//! - Daniel Rakos, AMD
//!# Related
//! - [`DisplayNativeHdrSurfaceCapabilitiesAMD`]
//! - [`SwapchainDisplayNativeHdrCreateInfoAMD`]
//! - [`SetLocalDimmingAMD`]
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
#[doc(alias = "VK_AMD_DISPLAY_NATIVE_HDR_SPEC_VERSION")]
pub const AMD_DISPLAY_NATIVE_HDR_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_AMD_DISPLAY_NATIVE_HDR_EXTENSION_NAME")]
pub const AMD_DISPLAY_NATIVE_HDR_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_AMD_display_native_hdr");
