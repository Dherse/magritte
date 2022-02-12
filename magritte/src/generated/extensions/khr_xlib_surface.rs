//![VK_KHR_xlib_surface](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_xlib_surface.html) - instance extension
//!# Description
//!The [`VK_KHR_xlib_surface`] extension is an instance extension.
//!It provides a mechanism to create a [`SurfaceKHR`] object (defined by
//!the `[`VK_KHR_surface`]` extension) that refers to an X11 [`Window`],
//!using the Xlib client-side library, as well as a query to determine support
//!for rendering via Xlib.
//!# Revision
//!6
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_surface`]`
//!# Contacts
//! - Jesse Hall [critsec](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_xlib_surface]
//!   @critsec%0A<<Here describe the issue or question you have about the VK_KHR_xlib_surface
//!   extension>>)
//! - Ian Elliott [ianelliottus](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_xlib_surface]
//!   @ianelliottus%0A<<Here describe the issue or question you have about the VK_KHR_xlib_surface
//!   extension>>)
//!# New functions & commands
//! - [`CreateXlibSurfaceKHR`]
//! - [`GetPhysicalDeviceXlibPresentationSupportKHR`]
//!# New structures
//! - [`XlibSurfaceCreateInfoKHR`]
//!# New bitmasks
//! - [`XlibSurfaceCreateFlagsKHR`]
//!# New constants
//! - [`KHR_XLIB_SURFACE_EXTENSION_NAME`]
//! - [`KHR_XLIB_SURFACE_SPEC_VERSION`]
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_XLIB_SURFACE_CREATE_INFO_KHR`
//!# Known issues & F.A.Q
//!1) Does X11 need a way to query for compatibility between a particular
//!physical device and a specific screen? This would be a more general query
//!than [`GetPhysicalDeviceSurfaceSupportKHR`]; if it returned
//![`TRUE`], then the physical device could be assumed to support
//!presentation to any window on that screen.**RESOLVED**: Yes, this is needed for toolkits that
//! want to create a
//![`Device`] before creating a window.
//!To ensure the query is reliable, it must be made against a particular X
//!visual rather than the screen in general.
//!# Version History
//! - Revision 1, 2015-09-23 (Jesse Hall)
//! - Initial draft, based on the previous contents of VK_EXT_KHR_swapchain
//!(later renamed VK_EXT_KHR_surface).
//! - Revision 2, 2015-10-02 (James Jones)
//! - Added presentation support query for (Display*, VisualID) pair.
//! - Removed “root” parameter from CreateXlibSurfaceKHR(), as it is
//!redundant when a window on the same screen is specified as well.
//! - Added appropriate X errors.
//! - Adjusted wording of issue #1 and added agreed upon resolution.
//! - Revision 3, 2015-10-14 (Ian Elliott)
//! - Renamed this extension from VK_EXT_KHR_x11_surface to
//!VK_EXT_KHR_xlib_surface.
//! - Revision 4, 2015-10-26 (Ian Elliott)
//! - Renamed from VK_EXT_KHR_xlib_surface to VK_KHR_xlib_surface.
//! - Revision 5, 2015-11-03 (Daniel Rakos)
//! - Added allocation callbacks to vkCreateXlibSurfaceKHR.
//! - Revision 6, 2015-11-28 (Daniel Rakos)
//! - Updated the surface create function to take a pCreateInfo structure.
//!# Other info
//! * 2015-11-28
//! * No known IP claims.
//!*
//! - Patrick Doane, Blizzard
//! - Jason Ekstrand, Intel
//! - Ian Elliott, LunarG
//! - Courtney Goeltzenleuchter, LunarG
//! - Jesse Hall, Google
//! - James Jones, NVIDIA
//! - Antoine Labour, Google
//! - Jon Leech, Khronos
//! - David Mao, AMD
//! - Norbert Nopper, Freescale
//! - Alon Or-bach, Samsung
//! - Daniel Rakos, AMD
//! - Graham Sellers, AMD
//! - Ray Smith, ARM
//! - Jeff Vigil, Qualcomm
//! - Chia-I Wu, LunarG
//!# Related
//! - [`XlibSurfaceCreateFlagsKHR`]
//! - [`XlibSurfaceCreateInfoKHR`]
//! - [`CreateXlibSurfaceKHR`]
//! - [`GetPhysicalDeviceXlibPresentationSupportKHR`]
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
#[doc(alias = "VK_KHR_XLIB_SURFACE_SPEC_VERSION")]
pub const KHR_XLIB_SURFACE_SPEC_VERSION: u32 = 6;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_XLIB_SURFACE_EXTENSION_NAME")]
pub const KHR_XLIB_SURFACE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_xlib_surface");
