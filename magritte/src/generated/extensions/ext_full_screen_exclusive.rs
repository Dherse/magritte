//![VK_EXT_full_screen_exclusive](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_full_screen_exclusive.html) - device extension
//!# Description
//!This extension allows applications to set the policy for swapchain creation
//!and presentation mechanisms relating to full-screen access.
//!Implementations may be able to acquire exclusive access to a particular
//!display for an application window that covers the whole screen.
//!This can increase performance on some systems by bypassing composition,
//!however it can also result in disruptive or expensive transitions in the
//!underlying windowing system when a change occurs.Applications can choose between explicitly
//! disallowing or allowing this
//!behavior, letting the implementation decide, or managing this mode of
//!operation directly using the new [`AcquireFullScreenExclusiveModeEXT`]
//!and [`ReleaseFullScreenExclusiveModeEXT`] commands.
//!# Revision
//!4
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_get_physical_device_properties2`]`
//! - Requires `[`VK_KHR_surface`]`
//! - Requires `[`VK_KHR_get_surface_capabilities2`]`
//! - Requires `[`VK_KHR_swapchain`]`
//!# Contacts
//! - James Jones [cubanismo](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_full_screen_exclusive]
//!   @cubanismo%0A<<Here describe the issue or question you have about the
//!   VK_EXT_full_screen_exclusive extension>>)
//!# New functions & commands
//! - [`AcquireFullScreenExclusiveModeEXT`]
//! - [`GetPhysicalDeviceSurfacePresentModes2EXT`]
//! - [`ReleaseFullScreenExclusiveModeEXT`]If [`VK_KHR_device_group`] is supported:
//! - [`GetDeviceGroupSurfacePresentModes2EXT`]If [Version 1.1]() is supported:
//! - [`GetDeviceGroupSurfacePresentModes2EXT`]
//!# New structures
//! - Extending [`PhysicalDeviceSurfaceInfo2KHR`], [`SwapchainCreateInfoKHR`]:
//! - [`SurfaceFullScreenExclusiveInfoEXT`]
//! - Extending [`SurfaceCapabilities2KHR`]:
//! - [`SurfaceCapabilitiesFullScreenExclusiveEXT`]If [`VK_KHR_win32_surface`] is supported:
//! - Extending [`PhysicalDeviceSurfaceInfo2KHR`], [`SwapchainCreateInfoKHR`]:
//! - [`SurfaceFullScreenExclusiveWin32InfoEXT`]
//!# New enums
//! - [`FullScreenExclusiveEXT`]
//!# New constants
//! - [`EXT_FULL_SCREEN_EXCLUSIVE_EXTENSION_NAME`]
//! - [`EXT_FULL_SCREEN_EXCLUSIVE_SPEC_VERSION`]
//! - Extending [`VulkanResultCodes`]:
//! - `VK_ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT`
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES_FULL_SCREEN_EXCLUSIVE_EXT`
//! - `VK_STRUCTURE_TYPE_SURFACE_FULL_SCREEN_EXCLUSIVE_INFO_EXT`If [`VK_KHR_win32_surface`] is
//!   supported:
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_SURFACE_FULL_SCREEN_EXCLUSIVE_WIN32_INFO_EXT`
//!# Known issues & F.A.Q
//!1) What should the extension & flag be called?**RESOLVED**: VK_EXT_full_screen_exclusive.Other
//! options considered (prior to the app-controlled mode) were:
//! - VK_EXT_smooth_fullscreen_transition
//! - VK_EXT_fullscreen_behavior
//! - VK_EXT_fullscreen_preference
//! - VK_EXT_fullscreen_hint
//! - VK_EXT_fast_fullscreen_transition
//! - VK_EXT_avoid_fullscreen_exclusive2) Do we need more than a boolean toggle?**RESOLVED**:
//!   Yes.Using an enum with default/allowed/disallowed/app-controlled enables
//!applications to accept driver default behavior, specifically override it in
//!either direction without implying the driver is ever required to use
//!full-screen exclusive mechanisms, or manage this mode explicitly.3) Should this be a KHR or EXT
//! extension?**RESOLVED**: EXT, in order to allow it to be shipped faster.4) Can the fullscreen
//! hint affect the surface capabilities, and if so,
//!should the hint also be specified as input when querying the surface
//!capabilities?**RESOLVED**: Yes on both accounts.While the hint does not guarantee a particular
//! fullscreen mode will be used
//!when the swapchain is created, it can sometimes imply particular modes will
//!NOT be used.
//!If the driver determines that it will opt-out of using a particular mode
//!based on the policy, and knows it can only support certain capabilities if
//!that mode is used, it would be confusing at best to the application to
//!report those capabilities in such cases.
//!Not allowing implementations to report this state to applications could
//!result in situations where applications are unable to determine why
//!swapchain creation fails when they specify certain hint values, which could
//!result in never- terminating surface creation loops.5) Should full-screen be one word or
//! two?**RESOLVED**: Two words."Fullscreen" is not in my dictionary, and web searches did not turn
//! up
//!definitive proof that it is a colloquially accepted compound word.
//!Documentation for the corresponding Windows API mechanisms dithers.
//!The text consistently uses a hyphen, but none-the-less, there is a
//!SetFullscreenState method in the DXGI swapchain object.
//!Given this inconclusive external guidance, it is best to adhere to the
//!Vulkan style guidelines and avoid inventing new compound words.
//!# Version History
//! - Revision 4, 2019-03-12 (Tobias Hector)
//! - Added application-controlled mode, and related functions
//! - Tidied up appendix
//! - Revision 3, 2019-01-03 (James Jones)
//! - Renamed to VK_EXT_full_screen_exclusive
//! - Made related adjustments to the tri-state enumerant names.
//! - Revision 2, 2018-11-27 (James Jones)
//! - Renamed to VK_KHR_fullscreen_behavior
//! - Switched from boolean flag to tri-state enum
//! - Revision 1, 2018-11-06 (James Jones)
//! - Internal revision
//!# Other info
//! * 2019-03-12
//! * No known IP claims.
//!*
//! - Interacts with Vulkan 1.1
//! - Interacts with `[`VK_KHR_device_group`]`
//! - Interacts with `[`VK_KHR_win32_surface`]`
//!*
//! - Hans-Kristian Arntzen, ARM
//! - Slawomir Grajewski, Intel
//! - Tobias Hector, AMD
//! - James Jones, NVIDIA
//! - Daniel Rakos, AMD
//! - Jeff Juliano, NVIDIA
//! - Joshua Schnarr, NVIDIA
//! - Aaron Hagan, AMD
//!# Related
//! - [`FullScreenExclusiveEXT`]
//! - [`SurfaceCapabilitiesFullScreenExclusiveEXT`]
//! - [`SurfaceFullScreenExclusiveInfoEXT`]
//! - [`AcquireFullScreenExclusiveModeEXT`]
//! - [`GetPhysicalDeviceSurfacePresentModes2EXT`]
//! - [`ReleaseFullScreenExclusiveModeEXT`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ffi::CStr;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_FULL_SCREEN_EXCLUSIVE_SPEC_VERSION")]
pub const EXT_FULL_SCREEN_EXCLUSIVE_SPEC_VERSION: u32 = 4;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_FULL_SCREEN_EXCLUSIVE_EXTENSION_NAME")]
pub const EXT_FULL_SCREEN_EXCLUSIVE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_full_screen_exclusive");
///[VkFullScreenExclusiveEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFullScreenExclusiveEXT.html) - Hint values an application can specify affecting full-screen transition behavior
///# C Specifications
///Possible values of
///[`SurfaceFullScreenExclusiveInfoEXT::full_screen_exclusive`] are:
///```c
///// Provided by VK_EXT_full_screen_exclusive
///typedef enum VkFullScreenExclusiveEXT {
///    VK_FULL_SCREEN_EXCLUSIVE_DEFAULT_EXT = 0,
///    VK_FULL_SCREEN_EXCLUSIVE_ALLOWED_EXT = 1,
///    VK_FULL_SCREEN_EXCLUSIVE_DISALLOWED_EXT = 2,
///    VK_FULL_SCREEN_EXCLUSIVE_APPLICATION_CONTROLLED_EXT = 3,
///} VkFullScreenExclusiveEXT;
///```
///# Description
/// - [`FULL_SCREEN_EXCLUSIVE_DEFAULT`] indicates the implementation
///**should** determine the appropriate full-screen method by whatever means
///it deems appropriate.
/// - [`FULL_SCREEN_EXCLUSIVE_ALLOWED`] indicates the implementation
///**may** use full-screen exclusive mechanisms when available.
///Such mechanisms **may** result in better performance and/or the
///availability of different presentation capabilities, but **may** require a
///more disruptive transition during swapchain initialization, first
///presentation and/or destruction.
/// - [`FULL_SCREEN_EXCLUSIVE_DISALLOWED`] indicates the
///implementation **should** avoid using full-screen mechanisms which rely on
///disruptive transitions.
/// - [`FULL_SCREEN_EXCLUSIVE_APPLICATION_CONTROLLED`] indicates the
///application will manage full-screen exclusive mode by using the
///[`AcquireFullScreenExclusiveModeEXT`] and
///[`ReleaseFullScreenExclusiveModeEXT`] commands.
///# Related
/// - [`VK_EXT_full_screen_exclusive`]
/// - [`SurfaceFullScreenExclusiveInfoEXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkFullScreenExclusiveEXT")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct FullScreenExclusiveEXT(i32);
impl const Default for FullScreenExclusiveEXT {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for FullScreenExclusiveEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple("FullScreenExclusiveEXT")
            .field(match *self {
                Self::FULL_SCREEN_EXCLUSIVE_DEFAULT => &"FULL_SCREEN_EXCLUSIVE_DEFAULT",
                Self::FULL_SCREEN_EXCLUSIVE_ALLOWED => &"FULL_SCREEN_EXCLUSIVE_ALLOWED",
                Self::FULL_SCREEN_EXCLUSIVE_DISALLOWED => &"FULL_SCREEN_EXCLUSIVE_DISALLOWED",
                Self::FULL_SCREEN_EXCLUSIVE_APPLICATION_CONTROLLED => &"FULL_SCREEN_EXCLUSIVE_APPLICATION_CONTROLLED",
                other => unreachable!("invalid value for `FullScreenExclusiveEXT`: {:?}", other),
            })
            .finish()
    }
}
impl FullScreenExclusiveEXT {
    ///[`FULL_SCREEN_EXCLUSIVE_DEFAULT`] indicates the implementation
    ///**should** determine the appropriate full-screen method by whatever means
    ///it deems appropriate.
    pub const FULL_SCREEN_EXCLUSIVE_DEFAULT: Self = Self(0);
    ///[`FULL_SCREEN_EXCLUSIVE_ALLOWED`] indicates the implementation
    ///**may** use full-screen exclusive mechanisms when available.
    ///Such mechanisms **may** result in better performance and/or the
    ///availability of different presentation capabilities, but **may** require a
    ///more disruptive transition during swapchain initialization, first
    ///presentation and/or destruction.
    pub const FULL_SCREEN_EXCLUSIVE_ALLOWED: Self = Self(1);
    ///[`FULL_SCREEN_EXCLUSIVE_DISALLOWED`] indicates the
    ///implementation **should** avoid using full-screen mechanisms which rely on
    ///disruptive transitions.
    pub const FULL_SCREEN_EXCLUSIVE_DISALLOWED: Self = Self(2);
    ///[`FULL_SCREEN_EXCLUSIVE_APPLICATION_CONTROLLED`] indicates the
    ///application will manage full-screen exclusive mode by using the
    ///[`AcquireFullScreenExclusiveModeEXT`] and
    ///[`ReleaseFullScreenExclusiveModeEXT`] commands.
    pub const FULL_SCREEN_EXCLUSIVE_APPLICATION_CONTROLLED: Self = Self(3);
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> i32 {
        self.0
    }
}
