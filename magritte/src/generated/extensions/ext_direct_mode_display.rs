//![VK_EXT_direct_mode_display](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_direct_mode_display.html) - instance extension
//!# Description
//!This is extension, along with related platform extensions, allows
//!applications to take exclusive control of displays associated with a native
//!windowing system.
//!This is especially useful for virtual reality applications that wish to hide
//!HMDs (head mounted displays) from the native platform’s display management
//!system, desktop, and/or other applications.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_display`]`
//!# Contacts
//! - James Jones [cubanismo](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_direct_mode_display]
//!   @cubanismo%0A<<Here describe the issue or question you have about the
//!   VK_EXT_direct_mode_display extension>>)
//!# New functions & commands
//! - [`ReleaseDisplayEXT`]
//!# New constants
//! - [`EXT_DIRECT_MODE_DISPLAY_EXTENSION_NAME`]
//! - [`EXT_DIRECT_MODE_DISPLAY_SPEC_VERSION`]
//!# Known issues & F.A.Q
//!1) Should this extension and its related platform-specific extensions
//!leverage `[`VK_KHR_display`]`, or provide separate equivalent interfaces. **RESOLVED** : Use
//! `[`VK_KHR_display`]` concepts and objects.
//!`[`VK_KHR_display`]` can be used to enumerate all displays on the system,
//!including those attached to/in use by a window system or native platform,
//!but `[`VK_KHR_display_swapchain`]` will fail to create a swapchain on
//!in-use displays.
//!This extension and its platform-specific children will allow applications to
//!grab in-use displays away from window systems and/or native platforms,
//!allowing them to be used with `[`VK_KHR_display_swapchain`]`.2) Are separate calls needed to
//! acquire displays and enable direct mode? **RESOLVED** : No, these operations happen in one
//! combined command.
//!Acquiring a display puts it into direct mode.
//!# Version History
//! - Revision 1, 2016-12-13 (James Jones)  - Initial draft
//!# Other info
//! * 2016-12-13
//! * No known IP claims.
//! * - Pierre Boudier, NVIDIA  - James Jones, NVIDIA  - Damien Leone, NVIDIA  - Pierre-Loup
//!   Griffais, Valve  - Liam Middlebrook, NVIDIA
//!# Related
//! - [`ReleaseDisplayEXT`]
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
#[doc(alias = "VK_EXT_DIRECT_MODE_DISPLAY_SPEC_VERSION")]
pub const EXT_DIRECT_MODE_DISPLAY_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_DIRECT_MODE_DISPLAY_EXTENSION_NAME")]
pub const EXT_DIRECT_MODE_DISPLAY_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_direct_mode_display");
