//![VK_KHR_wayland_surface](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_wayland_surface.html) - instance extension
//!# Description
//!The [`VK_KHR_wayland_surface`] extension is an instance extension.
//!It provides a mechanism to create a [`SurfaceKHR`] object (defined by
//!the `[`VK_KHR_surface`]` extension) that refers to a Wayland
//![`wl_surface`], as well as a query to determine support for rendering to a
//!Wayland compositor.
//!# Revision
//!6
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_surface`]`
//!# Contacts
//! - Jesse Hall [critsec](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_wayland_surface]
//!   @critsec%0A<<Here describe the issue or question you have about the VK_KHR_wayland_surface
//!   extension>>)
//! - Ian Elliott [ianelliottus](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_wayland_surface]
//!   @ianelliottus%0A<<Here describe the issue or question you have about the
//!   VK_KHR_wayland_surface extension>>)
//!# New functions & commands
//! - [`CreateWaylandSurfaceKHR`]
//! - [`GetPhysicalDeviceWaylandPresentationSupportKHR`]
//!# New structures
//! - [`WaylandSurfaceCreateInfoKHR`]
//!# New bitmasks
//! - [`WaylandSurfaceCreateFlagsKHR`]
//!# New constants
//! - [`KHR_WAYLAND_SURFACE_EXTENSION_NAME`]
//! - [`KHR_WAYLAND_SURFACE_SPEC_VERSION`]
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_WAYLAND_SURFACE_CREATE_INFO_KHR`
//!# Known issues & F.A.Q
//!1) Does Wayland need a way to query for compatibility between a particular
//!physical device and a specific Wayland display? This would be a more general
//!query than [`GetPhysicalDeviceSurfaceSupportKHR`]: if the
//!Wayland-specific query returned [`TRUE`] for a ([`PhysicalDevice`],
//!`struct wl_display*`) pair, then the physical device could be assumed to
//!support presentation to any [`SurfaceKHR`] for surfaces on the display.**RESOLVED**: Yes.
//![`GetPhysicalDeviceWaylandPresentationSupportKHR`] was added to address
//!this issue.2) Should we require surfaces created with [`CreateWaylandSurfaceKHR`]
//!to support the `VK_PRESENT_MODE_MAILBOX_KHR` present mode?**RESOLVED**: Yes.
//!Wayland is an inherently mailbox window system and mailbox support is
//!required for some Wayland compositor interactions to work as expected.
//!While handling these interactions may be possible with
//!`VK_PRESENT_MODE_FIFO_KHR`, it is much more difficult to do without
//!deadlock and requiring all Wayland applications to be able to support
//!implementations which only support `VK_PRESENT_MODE_FIFO_KHR` would be
//!an onerous restriction on application developers.
//!# Version History
//! - Revision 1, 2015-09-23 (Jesse Hall)
//! - Initial draft, based on the previous contents of VK_EXT_KHR_swapchain
//!(later renamed VK_EXT_KHR_surface).
//!
//! - Revision 2, 2015-10-02 (James Jones)
//! - Added vkGetPhysicalDeviceWaylandPresentationSupportKHR() to resolve
//!issue #1.
//! - Adjusted wording of issue #1 to match the agreed-upon solution.
//! - Renamed “window” parameters to “surface” to match Wayland
//!conventions.
//!
//! - Revision 3, 2015-10-26 (Ian Elliott)
//! - Renamed from VK_EXT_KHR_wayland_surface to VK_KHR_wayland_surface.
//!
//! - Revision 4, 2015-11-03 (Daniel Rakos)
//! - Added allocation callbacks to vkCreateWaylandSurfaceKHR.
//!
//! - Revision 5, 2015-11-28 (Daniel Rakos)
//! - Updated the surface create function to take a pCreateInfo structure.
//!
//! - Revision 6, 2017-02-08 (Jason Ekstrand)
//! - Added the requirement that implementations support
//!`VK_PRESENT_MODE_MAILBOX_KHR`.
//! - Added wording about interactions between [`QueuePresentKHR`] and
//!the Wayland requests sent to the compositor.
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
//! - [`WaylandSurfaceCreateFlagsKHR`]
//! - [`WaylandSurfaceCreateInfoKHR`]
//! - [`CreateWaylandSurfaceKHR`]
//! - [`GetPhysicalDeviceWaylandPresentationSupportKHR`]
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
#[doc(alias = "VK_KHR_WAYLAND_SURFACE_SPEC_VERSION")]
pub const KHR_WAYLAND_SURFACE_SPEC_VERSION: u32 = 6;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_WAYLAND_SURFACE_EXTENSION_NAME")]
pub const KHR_WAYLAND_SURFACE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_wayland_surface");
