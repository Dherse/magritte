//![VK_EXT_acquire_xlib_display](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_acquire_xlib_display.html) - instance extension
//!# Description
//!This extension allows an application to take exclusive control on a display
//!currently associated with an X11 screen.
//!When control is acquired, the display will be deassociated from the X11
//!screen until control is released or the specified display connection is
//!closed.
//!Essentially, the X11 screen will behave as if the monitor has been unplugged
//!until control is released.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_EXT_direct_mode_display`]`
//!# Contacts
//! - James Jones [cubanismo](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_acquire_xlib_display]
//!   @cubanismo%0A<<Here describe the issue or question you have about the
//!   VK_EXT_acquire_xlib_display extension>>)
//!# New functions & commands
//! - [`AcquireXlibDisplayEXT`]
//! - [`GetRandROutputDisplayEXT`]
//!# New constants
//! - [`EXT_ACQUIRE_XLIB_DISPLAY_EXTENSION_NAME`]
//! - [`EXT_ACQUIRE_XLIB_DISPLAY_SPEC_VERSION`]
//!# Known issues & F.A.Q
//!1) Should [`AcquireXlibDisplayEXT`] take an RandR display ID, or a
//!Vulkan display handle as input? **RESOLVED** : A Vulkan display handle.
//!Otherwise there would be no way to specify handles to displays that had been
//!prevented from being included in the X11 display list by some native
//!platform or vendor-specific mechanism.2) How does an application figure out which RandR display
//! corresponds to a
//!Vulkan display? **RESOLVED** : A new function, [`GetRandROutputDisplayEXT`], is introduced
//!for this purpose.3) Should [`GetRandROutputDisplayEXT`] be part of this extension, or a
//!general Vulkan / RandR or Vulkan / Xlib extension? **RESOLVED** : To avoid yet another
//! extension, include it in this extension.
//!# Version History
//! - Revision 1, 2016-12-13 (James Jones)  - Initial draft
//!# Other info
//! * 2016-12-13
//! * No known IP claims.
//! * - Dave Airlie, Red Hat  - Pierre Boudier, NVIDIA  - James Jones, NVIDIA  - Damien Leone,
//!   NVIDIA  - Pierre-Loup Griffais, Valve  - Liam Middlebrook, NVIDIA  - Daniel Vetter, Intel
//!# Related
//! - [`AcquireXlibDisplayEXT`]
//! - [`GetRandROutputDisplayEXT`]
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
#[doc(alias = "VK_EXT_ACQUIRE_XLIB_DISPLAY_SPEC_VERSION")]
pub const EXT_ACQUIRE_XLIB_DISPLAY_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_ACQUIRE_XLIB_DISPLAY_EXTENSION_NAME")]
pub const EXT_ACQUIRE_XLIB_DISPLAY_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_acquire_xlib_display");
