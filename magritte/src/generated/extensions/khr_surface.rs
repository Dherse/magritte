//![VK_KHR_surface](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_surface.html) - instance extension
//!# Description
//!The [`VK_KHR_surface`] extension is an instance extension.
//!It introduces [`SurfaceKHR`] objects, which abstract native platform
//!surface or window objects for use with Vulkan.
//!It also provides a way to determine whether a queue family in a physical
//!device supports presenting to particular surface.Separate extensions for each platform provide
//! the mechanisms for creating
//![`SurfaceKHR`] objects, but once created they may be used in this and
//!other platform-independent extensions, in particular the
//!`[`VK_KHR_swapchain`]` extension.
//!# Revision
//!25
//!# Dependencies
//! - Requires Vulkan 1.0
//!# Contacts
//! - James Jones [cubanismo](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_surface]
//!   @cubanismo%0A<<Here describe the issue or question you have about the VK_KHR_surface
//!   extension>>)
//! - Ian Elliott [ianelliottus](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_surface]
//!   @ianelliottus%0A<<Here describe the issue or question you have about the VK_KHR_surface
//!   extension>>)
//!# New handles
//! - [`SurfaceKHR`]
//!# New functions & commands
//! - [`DestroySurfaceKHR`]
//! - [`GetPhysicalDeviceSurfaceCapabilitiesKHR`]
//! - [`GetPhysicalDeviceSurfaceFormatsKHR`]
//! - [`GetPhysicalDeviceSurfacePresentModesKHR`]
//! - [`GetPhysicalDeviceSurfaceSupportKHR`]
//!# New structures
//! - [`SurfaceCapabilitiesKHR`]
//! - [`SurfaceFormatKHR`]
//!# New enums
//! - [`ColorSpaceKHR`]
//! - [`CompositeAlphaFlagBitsKHR`]
//! - [`PresentModeKHR`]
//! - [`SurfaceTransformFlagBitsKHR`]
//!# New bitmasks
//! - [`CompositeAlphaFlagsKHR`]
//!# New constants
//! - [`KHR_SURFACE_EXTENSION_NAME`]
//! - [`KHR_SURFACE_SPEC_VERSION`]
//! - Extending [`ObjectType`]:
//! - `VK_OBJECT_TYPE_SURFACE_KHR`
//!
//! - Extending [`VulkanResultCodes`]:
//! - `VK_ERROR_NATIVE_WINDOW_IN_USE_KHR`
//! - `VK_ERROR_SURFACE_LOST_KHR`
//!# Known issues & F.A.Q
//!1) Should this extension include a method to query whether a physical device
//!supports presenting to a specific window or native surface on a given
//!platform?**RESOLVED**: Yes.
//!Without this, applications would need to create a device instance to
//!determine whether a particular window can be presented to.
//!Knowing that a device supports presentation to a platform in general is not
//!sufficient, as a single machine might support multiple seats, or instances
//!of the platform that each use different underlying physical devices.
//!Additionally, on some platforms, such as the X Window System, different
//!drivers and devices might be used for different windows depending on which
//!section of the desktop they exist on.2) Should the [`GetPhysicalDeviceSurfaceCapabilitiesKHR`],
//![`GetPhysicalDeviceSurfaceFormatsKHR`], and
//![`GetPhysicalDeviceSurfacePresentModesKHR`] functions be in this
//!extension and operate on physical devices, rather than being in
//!`[`VK_KHR_swapchain`]` (i.e. device extension) and being dependent on
//![`Device`]?**RESOLVED**: Yes.
//!While it might be useful to depend on [`Device`] (and therefore on
//!enabled extensions and features) for the queries, Vulkan was released only
//!with the [`PhysicalDevice`] versions.
//!Many cases can be resolved by a Valid Usage statement, and/or by a separate
//!`pNext` chain version of the query struct specific to a given extension
//!or parameters, via extensible versions of the queries:
//![`GetPhysicalDeviceSurfacePresentModes2EXT`],
//![`GetPhysicalDeviceSurfaceCapabilities2KHR`], and
//![`GetPhysicalDeviceSurfaceFormats2KHR`].3) Should Vulkan support Xlib or XCB as the API for
//! accessing the X Window
//!System platform?**RESOLVED**: Both.
//!XCB is a more modern and efficient API, but Xlib usage is deeply ingrained
//!in many applications and likely will remain in use for the foreseeable
//!future.
//!Not all drivers necessarily need to support both, but including both as
//!options in the core specification will probably encourage support, which
//!should in turn ease adoption of the Vulkan API in older codebases.
//!Additionally, the performance improvements possible with XCB likely will not
//!have a measurable impact on the performance of Vulkan presentation and other
//!minimal window system interactions defined here.4) Should the GBM platform be included in the
//! list of platform enums?**RESOLVED**: Deferred, and will be addressed with a platform-specific
//!extension to be written in the future.
//!# Version History
//! - Revision 1, 2015-05-20 (James Jones)
//! - Initial draft, based on LunarG KHR spec, other KHR specs, patches
//!attached to bugs.
//!
//! - Revision 2, 2015-05-22 (Ian Elliott)
//! - Created initial Description section.
//! - Removed query for whether a platform requires the use of a queue for
//!presentation, since it was decided that presentation will always be
//!modeled as being part of the queue.
//! - Fixed typos and other minor mistakes.
//!
//! - Revision 3, 2015-05-26 (Ian Elliott)
//! - Improved the Description section.
//!
//! - Revision 4, 2015-05-27 (James Jones)
//! - Fixed compilation errors in example code.
//!
//! - Revision 5, 2015-06-01 (James Jones)
//! - Added issues 1 and 2 and made related spec updates.
//!
//! - Revision 6, 2015-06-01 (James Jones)
//! - Merged the platform type mappings table previously removed from
//!VK_KHR_swapchain with the platform description table in this spec.
//! - Added issues 3 and 4 documenting choices made when building the
//!initial list of native platforms supported.
//!
//! - Revision 7, 2015-06-11 (Ian Elliott)
//! - Updated table 1 per input from the KHR TSG.
//! - Updated issue 4 (GBM) per discussion with Daniel Stone.
//!He will create a platform-specific extension sometime in the future.
//!
//! - Revision 8, 2015-06-17 (James Jones)
//! - Updated enum-extending values using new convention.
//! - Fixed the value of VK_SURFACE_PLATFORM_INFO_TYPE_SUPPORTED_KHR.
//!
//! - Revision 9, 2015-06-17 (James Jones)
//! - Rebased on Vulkan API version 126.
//!
//! - Revision 10, 2015-06-18 (James Jones)
//! - Marked issues 2 and 3 resolved.
//!
//! - Revision 11, 2015-06-23 (Ian Elliott)
//! - Examples now show use of function pointers for extension functions.
//! - Eliminated extraneous whitespace.
//!
//! - Revision 12, 2015-07-07 (Daniel Rakos)
//! - Added error section describing when each error is expected to be
//!reported.
//! - Replaced the term “queue node index” with “queue family index” in
//!the spec as that is the agreed term to be used in the latest version
//!of the core header and spec.
//! - Replaced bool32_t with VkBool32.
//!
//! - Revision 13, 2015-08-06 (Daniel Rakos)
//! - Updated spec against latest core API header version.
//!
//! - Revision 14, 2015-08-20 (Ian Elliott)
//! - Renamed this extension and all of its enumerations, types, functions,
//!etc.
//!This makes it compliant with the proposed standard for Vulkan
//!extensions.
//! - Switched from “revision” to “version”, including use of the
//!VK_MAKE_VERSION macro in the header file.
//! - Did miscellaneous cleanup, etc.
//!
//! - Revision 15, 2015-08-20 (Ian Elliott—​porting a 2015-07-29 change from
//!James Jones)
//! - Moved the surface transform enums here from VK_WSI_swapchain so they
//!could be reused by VK_WSI_display.
//!
//! - Revision 16, 2015-09-01 (James Jones)
//! - Restore single-field revision number.
//!
//! - Revision 17, 2015-09-01 (James Jones)
//! - Fix example code compilation errors.
//!
//! - Revision 18, 2015-09-26 (Jesse Hall)
//! - Replaced VkSurfaceDescriptionKHR with the VkSurfaceKHR object, which
//!is created via layered extensions.
//!Added VkDestroySurfaceKHR.
//!
//! - Revision 19, 2015-09-28 (Jesse Hall)
//! - Renamed from VK_EXT_KHR_swapchain to VK_EXT_KHR_surface.
//!
//! - Revision 20, 2015-09-30 (Jeff Vigil)
//! - Add error result VK_ERROR_SURFACE_LOST_KHR.
//!
//! - Revision 21, 2015-10-15 (Daniel Rakos)
//! - Updated the resolution of issue #2 and include the surface capability
//!queries in this extension.
//! - Renamed SurfaceProperties to SurfaceCapabilities as it better reflects
//!that the values returned are the capabilities of the surface on a
//!particular device.
//! - Other minor cleanup and consistency changes.
//!
//! - Revision 22, 2015-10-26 (Ian Elliott)
//! - Renamed from VK_EXT_KHR_surface to VK_KHR_surface.
//!
//! - Revision 23, 2015-11-03 (Daniel Rakos)
//! - Added allocation callbacks to vkDestroySurfaceKHR.
//!
//! - Revision 24, 2015-11-10 (Jesse Hall)
//! - Removed VkSurfaceTransformKHR.
//!Use VkSurfaceTransformFlagBitsKHR instead.
//! - Rename VkSurfaceCapabilitiesKHR member maxImageArraySize to
//!maxImageArrayLayers.
//!
//! - Revision 25, 2016-01-14 (James Jones)
//! - Moved VK_ERROR_NATIVE_WINDOW_IN_USE_KHR from the
//!VK_KHR_android_surface to the VK_KHR_surface extension.
//!
//! - 2016-08-23 (Ian Elliott)
//! - Update the example code, to not have so many characters per line, and
//!to split out a new example to show how to obtain function pointers.
//!
//! - 2016-08-25 (Ian Elliott)
//! - A note was added at the beginning of the example code, stating that it
//!will be removed from future versions of the appendix.
//!# Other info
//! * 2016-08-25
//! * No known IP claims.
//!*
//! - Patrick Doane, Blizzard
//! - Ian Elliott, LunarG
//! - Jesse Hall, Google
//! - James Jones, NVIDIA
//! - David Mao, AMD
//! - Norbert Nopper, Freescale
//! - Alon Or-bach, Samsung
//! - Daniel Rakos, AMD
//! - Graham Sellers, AMD
//! - Jeff Vigil, Qualcomm
//! - Chia-I Wu, LunarG
//! - Jason Ekstrand, Intel
//!# Related
//! - [`ColorSpaceKHR`]
//! - [`CompositeAlphaFlagBitsKHR`]
//! - [`CompositeAlphaFlagsKHR`]
//! - [`PresentModeKHR`]
//! - [`SurfaceCapabilitiesKHR`]
//! - [`SurfaceFormatKHR`]
//! - [`SurfaceKHR`]
//! - [`SurfaceTransformFlagBitsKHR`]
//! - [`DestroySurfaceKHR`]
//! - [`GetPhysicalDeviceSurfaceCapabilitiesKHR`]
//! - [`GetPhysicalDeviceSurfaceFormatsKHR`]
//! - [`GetPhysicalDeviceSurfacePresentModesKHR`]
//! - [`GetPhysicalDeviceSurfaceSupportKHR`]
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
#[doc(alias = "VK_KHR_SURFACE_SPEC_VERSION")]
pub const KHR_SURFACE_SPEC_VERSION: u32 = 25;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_SURFACE_EXTENSION_NAME")]
pub const KHR_SURFACE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_surface");
///[VkPresentModeKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPresentModeKHR.html) - Presentation mode supported for a surface
///# C Specifications
///Possible values of elements of the
///[`GetPhysicalDeviceSurfacePresentModesKHR`]`::pPresentModes` array,
///indicating the supported presentation modes for a surface, are:
///```c
///// Provided by VK_KHR_surface
///typedef enum VkPresentModeKHR {
///    VK_PRESENT_MODE_IMMEDIATE_KHR = 0,
///    VK_PRESENT_MODE_MAILBOX_KHR = 1,
///    VK_PRESENT_MODE_FIFO_KHR = 2,
///    VK_PRESENT_MODE_FIFO_RELAXED_KHR = 3,
///  // Provided by VK_KHR_shared_presentable_image
///    VK_PRESENT_MODE_SHARED_DEMAND_REFRESH_KHR = 1000111000,
///  // Provided by VK_KHR_shared_presentable_image
///    VK_PRESENT_MODE_SHARED_CONTINUOUS_REFRESH_KHR = 1000111001,
///} VkPresentModeKHR;
///```
///# Description
/// - [`PRESENT_MODE_IMMEDIATE`] specifies that the presentation
///engine does not wait for a vertical blanking period to update the
///current image, meaning this mode **may** result in visible tearing.
///No internal queuing of presentation requests is needed, as the requests
///are applied immediately.
/// - [`PRESENT_MODE_MAILBOX`] specifies that the presentation engine
///waits for the next vertical blanking period to update the current image.
///Tearing **cannot** be observed.
///An internal single-entry queue is used to hold pending presentation
///requests.
///If the queue is full when a new presentation request is received, the
///new request replaces the existing entry, and any images associated with
///the prior entry become available for re-use by the application.
///One request is removed from the queue and processed during each vertical
///blanking period in which the queue is non-empty.
/// - [`PRESENT_MODE_FIFO`] specifies that the presentation engine
///waits for the next vertical blanking period to update the current image.
///Tearing **cannot** be observed.
///An internal queue is used to hold pending presentation requests.
///New requests are appended to the end of the queue, and one request is
///removed from the beginning of the queue and processed during each
///vertical blanking period in which the queue is non-empty.
///This is the only value of `presentMode` that is **required** to be
///supported.
/// - [`PRESENT_MODE_FIFO_RELAXED`] specifies that the presentation
///engine generally waits for the next vertical blanking period to update
///the current image.
///If a vertical blanking period has already passed since the last update
///of the current image then the presentation engine does not wait for
///another vertical blanking period for the update, meaning this mode **may**
///result in visible tearing in this case.
///This mode is useful for reducing visual stutter with an application that
///will mostly present a new image before the next vertical blanking
///period, but may occasionally be late, and present a new image just after
///the next vertical blanking period.
///An internal queue is used to hold pending presentation requests.
///New requests are appended to the end of the queue, and one request is
///removed from the beginning of the queue and processed during or after
///each vertical blanking period in which the queue is non-empty.
/// - [`PRESENT_MODE_SHARED_DEMAND_REFRESH`] specifies that the
///presentation engine and application have concurrent access to a single
///image, which is referred to as a *shared presentable image*.
///The presentation engine is only required to update the current image
///after a new presentation request is received.
///Therefore the application **must** make a presentation request whenever an
///update is required.
///However, the presentation engine **may** update the current image at any
///point, meaning this mode **may** result in visible tearing.
/// - [`PRESENT_MODE_SHARED_CONTINUOUS_REFRESH`] specifies that the
///presentation engine and application have concurrent access to a single
///image, which is referred to as a *shared presentable image*.
///The presentation engine periodically updates the current image on its
///regular refresh cycle.
///The application is only required to make one initial presentation
///request, after which the presentation engine **must** update the current
///image without any need for further presentation requests.
///The application **can** indicate the image contents have been updated by
///making a presentation request, but this does not guarantee the timing of
///when it will be updated.
///This mode **may** result in visible tearing if rendering to the image is
///not timed correctly.
///The supported [`ImageUsageFlagBits`] of the presentable images of a
///swapchain created for a surface **may** differ depending on the presentation
///mode, and can be determined as per the table below:
///# Related
/// - [`VK_KHR_surface`]
/// - [`SwapchainCreateInfoKHR`]
/// - [`GetPhysicalDeviceSurfacePresentModes2EXT`]
/// - [`GetPhysicalDeviceSurfacePresentModesKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPresentModeKHR")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct PresentModeKHR(i32);
impl const Default for PresentModeKHR {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for PresentModeKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple("PresentModeKHR")
            .field(match *self {
                Self::PRESENT_MODE_IMMEDIATE => &"PRESENT_MODE_IMMEDIATE",
                Self::PRESENT_MODE_MAILBOX => &"PRESENT_MODE_MAILBOX",
                Self::PRESENT_MODE_FIFO => &"PRESENT_MODE_FIFO",
                Self::PRESENT_MODE_FIFO_RELAXED => &"PRESENT_MODE_FIFO_RELAXED",
                Self::PRESENT_MODE_SHARED_DEMAND_REFRESH => &"PRESENT_MODE_SHARED_DEMAND_REFRESH",
                Self::PRESENT_MODE_SHARED_CONTINUOUS_REFRESH => &"PRESENT_MODE_SHARED_CONTINUOUS_REFRESH",
                other => unreachable!("invalid value for `PresentModeKHR`: {:?}", other),
            })
            .finish()
    }
}
impl PresentModeKHR {
    ///[`PRESENT_MODE_IMMEDIATE`] specifies that the presentation
    ///engine does not wait for a vertical blanking period to update the
    ///current image, meaning this mode **may** result in visible tearing.
    ///No internal queuing of presentation requests is needed, as the requests
    ///are applied immediately.
    pub const PRESENT_MODE_IMMEDIATE: Self = Self(0);
    ///[`PRESENT_MODE_MAILBOX`] specifies that the presentation engine
    ///waits for the next vertical blanking period to update the current image.
    ///Tearing **cannot** be observed.
    ///An internal single-entry queue is used to hold pending presentation
    ///requests.
    ///If the queue is full when a new presentation request is received, the
    ///new request replaces the existing entry, and any images associated with
    ///the prior entry become available for re-use by the application.
    ///One request is removed from the queue and processed during each vertical
    ///blanking period in which the queue is non-empty.
    pub const PRESENT_MODE_MAILBOX: Self = Self(1);
    ///[`PRESENT_MODE_FIFO`] specifies that the presentation engine
    ///waits for the next vertical blanking period to update the current image.
    ///Tearing **cannot** be observed.
    ///An internal queue is used to hold pending presentation requests.
    ///New requests are appended to the end of the queue, and one request is
    ///removed from the beginning of the queue and processed during each
    ///vertical blanking period in which the queue is non-empty.
    ///This is the only value of `presentMode` that is **required** to be
    ///supported.
    pub const PRESENT_MODE_FIFO: Self = Self(2);
    ///[`PRESENT_MODE_FIFO_RELAXED`] specifies that the presentation
    ///engine generally waits for the next vertical blanking period to update
    ///the current image.
    ///If a vertical blanking period has already passed since the last update
    ///of the current image then the presentation engine does not wait for
    ///another vertical blanking period for the update, meaning this mode **may**
    ///result in visible tearing in this case.
    ///This mode is useful for reducing visual stutter with an application that
    ///will mostly present a new image before the next vertical blanking
    ///period, but may occasionally be late, and present a new image just after
    ///the next vertical blanking period.
    ///An internal queue is used to hold pending presentation requests.
    ///New requests are appended to the end of the queue, and one request is
    ///removed from the beginning of the queue and processed during or after
    ///each vertical blanking period in which the queue is non-empty.
    pub const PRESENT_MODE_FIFO_RELAXED: Self = Self(3);
    ///[`PRESENT_MODE_SHARED_DEMAND_REFRESH`] specifies that the
    ///presentation engine and application have concurrent access to a single
    ///image, which is referred to as a *shared presentable image*.
    ///The presentation engine is only required to update the current image
    ///after a new presentation request is received.
    ///Therefore the application **must** make a presentation request whenever an
    ///update is required.
    ///However, the presentation engine **may** update the current image at any
    ///point, meaning this mode **may** result in visible tearing.
    ///
    ///Provided by [`crate::extensions::khr_shared_presentable_image`]
    pub const PRESENT_MODE_SHARED_DEMAND_REFRESH: Self = Self(1000111000);
    ///[`PRESENT_MODE_SHARED_CONTINUOUS_REFRESH`] specifies that the
    ///presentation engine and application have concurrent access to a single
    ///image, which is referred to as a *shared presentable image*.
    ///The presentation engine periodically updates the current image on its
    ///regular refresh cycle.
    ///The application is only required to make one initial presentation
    ///request, after which the presentation engine **must** update the current
    ///image without any need for further presentation requests.
    ///The application **can** indicate the image contents have been updated by
    ///making a presentation request, but this does not guarantee the timing of
    ///when it will be updated.
    ///This mode **may** result in visible tearing if rendering to the image is
    ///not timed correctly.
    ///
    ///Provided by [`crate::extensions::khr_shared_presentable_image`]
    pub const PRESENT_MODE_SHARED_CONTINUOUS_REFRESH: Self = Self(1000111001);
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
///[VkColorSpaceKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkColorSpaceKHR.html) - Supported color space of the presentation engine
///# C Specifications
///Possible values of [`SurfaceFormatKHR::color_space`], specifying
///supported color spaces of a presentation engine, are:
///```c
///// Provided by VK_KHR_surface
///typedef enum VkColorSpaceKHR {
///    VK_COLOR_SPACE_SRGB_NONLINEAR_KHR = 0,
///  // Provided by VK_EXT_swapchain_colorspace
///    VK_COLOR_SPACE_DISPLAY_P3_NONLINEAR_EXT = 1000104001,
///  // Provided by VK_EXT_swapchain_colorspace
///    VK_COLOR_SPACE_EXTENDED_SRGB_LINEAR_EXT = 1000104002,
///  // Provided by VK_EXT_swapchain_colorspace
///    VK_COLOR_SPACE_DISPLAY_P3_LINEAR_EXT = 1000104003,
///  // Provided by VK_EXT_swapchain_colorspace
///    VK_COLOR_SPACE_DCI_P3_NONLINEAR_EXT = 1000104004,
///  // Provided by VK_EXT_swapchain_colorspace
///    VK_COLOR_SPACE_BT709_LINEAR_EXT = 1000104005,
///  // Provided by VK_EXT_swapchain_colorspace
///    VK_COLOR_SPACE_BT709_NONLINEAR_EXT = 1000104006,
///  // Provided by VK_EXT_swapchain_colorspace
///    VK_COLOR_SPACE_BT2020_LINEAR_EXT = 1000104007,
///  // Provided by VK_EXT_swapchain_colorspace
///    VK_COLOR_SPACE_HDR10_ST2084_EXT = 1000104008,
///  // Provided by VK_EXT_swapchain_colorspace
///    VK_COLOR_SPACE_DOLBYVISION_EXT = 1000104009,
///  // Provided by VK_EXT_swapchain_colorspace
///    VK_COLOR_SPACE_HDR10_HLG_EXT = 1000104010,
///  // Provided by VK_EXT_swapchain_colorspace
///    VK_COLOR_SPACE_ADOBERGB_LINEAR_EXT = 1000104011,
///  // Provided by VK_EXT_swapchain_colorspace
///    VK_COLOR_SPACE_ADOBERGB_NONLINEAR_EXT = 1000104012,
///  // Provided by VK_EXT_swapchain_colorspace
///    VK_COLOR_SPACE_PASS_THROUGH_EXT = 1000104013,
///  // Provided by VK_EXT_swapchain_colorspace
///    VK_COLOR_SPACE_EXTENDED_SRGB_NONLINEAR_EXT = 1000104014,
///  // Provided by VK_AMD_display_native_hdr
///    VK_COLOR_SPACE_DISPLAY_NATIVE_AMD = 1000213000,
///    VK_COLORSPACE_SRGB_NONLINEAR_KHR = VK_COLOR_SPACE_SRGB_NONLINEAR_KHR,
///  // Provided by VK_EXT_swapchain_colorspace
///    VK_COLOR_SPACE_DCI_P3_LINEAR_EXT = VK_COLOR_SPACE_DISPLAY_P3_LINEAR_EXT,
///} VkColorSpaceKHR;
///```
///# Description
/// - [`COLOR_SPACE_SRGB_NONLINEAR`] specifies support for the sRGB
///color space.
/// - [`COLOR_SPACE_DISPLAY_P3_NONLINEAR_EXT`] specifies support for the
///Display-P3 color space to be displayed using an sRGB-like EOTF (defined
///below).
/// - [`COLOR_SPACE_EXTENDED_SRGB_LINEAR_EXT`] specifies support for the
///extended sRGB color space to be displayed using a linear EOTF.
/// - [`COLOR_SPACE_EXTENDED_SRGB_NONLINEAR_EXT`] specifies support for
///the extended sRGB color space to be displayed using an sRGB EOTF.
/// - [`COLOR_SPACE_DISPLAY_P3_LINEAR_EXT`] specifies support for the
///Display-P3 color space to be displayed using a linear EOTF.
/// - [`COLOR_SPACE_DCI_P3_NONLINEAR_EXT`] specifies support for the
///DCI-P3 color space to be displayed using the DCI-P3 EOTF.
///Note that values in such an image are interpreted as XYZ encoded color
///data by the presentation engine.
/// - [`COLOR_SPACE_BT709_LINEAR_EXT`] specifies support for the BT709
///color space to be displayed using a linear EOTF.
/// - [`COLOR_SPACE_BT709_NONLINEAR_EXT`] specifies support for the BT709
///color space to be displayed using the SMPTE 170M EOTF.
/// - [`COLOR_SPACE_BT2020_LINEAR_EXT`] specifies support for the BT2020
///color space to be displayed using a linear EOTF.
/// - [`COLOR_SPACE_HDR10_ST2084_EXT`] specifies support for the HDR10
///(BT2020 color) space to be displayed using the SMPTE ST2084 Perceptual
///Quantizer (PQ) EOTF.
/// - [`COLOR_SPACE_DOLBYVISION_EXT`] specifies support for the Dolby
///Vision (BT2020 color space), proprietary encoding, to be displayed using
///the SMPTE ST2084 EOTF.
/// - [`COLOR_SPACE_HDR10_HLG_EXT`] specifies support for the HDR10
///(BT2020 color space) to be displayed using the Hybrid Log Gamma (HLG)
///EOTF.
/// - [`COLOR_SPACE_ADOBERGB_LINEAR_EXT`] specifies support for the
///AdobeRGB color space to be displayed using a linear EOTF.
/// - [`COLOR_SPACE_ADOBERGB_NONLINEAR_EXT`] specifies support for the
///AdobeRGB color space to be displayed using the Gamma 2.2 EOTF.
/// - [`COLOR_SPACE_PASS_THROUGH_EXT`] specifies that color components
///are used “as is”.
///This is intended to allow applications to supply data for color spaces
///not described here.
/// - [`COLOR_SPACE_DISPLAY_NATIVE_AMD`] specifies support for the
///display’s native color space.
///This matches the color space expectations of AMD’s FreeSync2 standard,
///for displays supporting it.
///The color components of non-linear color space swap chain images **must** have
///had the appropriate transfer function applied.
///The color space selected for the swap chain image will not affect the
///processing of data written into the image by the implementation.
///Vulkan requires that all implementations support the sRGB transfer function
///by use of an SRGB pixel format.
///Other transfer functions, such as SMPTE 170M or SMPTE2084, **can** be performed
///by the application shader.
///This extension defines enums for [`ColorSpaceKHR`] that correspond to
///the following color spaces:The transfer functions are described in the “Transfer Functions”
/// chapter
///of the [Khronos Data Format Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#data-format).Except Display-P3 OETF, which is:<span class="katex"><span class="katex-html" aria-hidden="true"><span class="base"><span class="strut" style="height:3.30003em;vertical-align:-1.400015em;"></span><span class="mord"><span class="mtable"><span class="col-align-r"><span class="vlist-t vlist-t2"><span class="vlist-r"><span style="height:1.900015em;" class="vlist"><span style="top:-3.9000150000000002em;"><span style="height:3.75em;" class="pstrut"></span><span class="mord"><span style="margin-right:0.05764em;" class="mord mathdefault">E</span></span></span></span><span class="vlist-s">​</span></span><span class="vlist-r"><span style="height:1.400015em;" class="vlist"><span></span></span></span></span></span><span class="col-align-l"><span class="vlist-t vlist-t2"><span class="vlist-r"><span style="height:1.900015em;" class="vlist"><span style="top:-3.9000150000000002em;"><span class="pstrut" style="height:3.75em;"></span><span class="mord"><span class="mord"></span><span style="margin-right:0.2777777777777778em;" class="mspace"></span><span class="mrel">=</span><span class="mspace" style="margin-right:0.2777777777777778em;"></span><span class="minner"><span style="top:0em;" class="mopen delimcenter"><span class="delimsizing size4">{</span></span><span class="mord"><span class="mtable"><span class="col-align-l"><span class="vlist-t vlist-t2"><span class="vlist-r"><span style="height:1.69em;" class="vlist"><span style="top:-3.69em;"><span class="pstrut" style="height:3.008em;"></span><span class="mord"><span class="mord">1</span><span class="mord">.</span><span class="mord">0</span><span class="mord">5</span><span class="mord">5</span><span style="margin-right:0.2222222222222222em;" class="mspace"></span><span class="mbin">×</span><span class="mspace" style="margin-right:0.2222222222222222em;"></span><span class="mord"><span class="mord mathdefault">L</span><span class="msupsub"><span class="vlist-t"><span class="vlist-r"><span style="height:0.9540200000000001em;" class="vlist"><span style="top:-3.363em;margin-right:0.05em;"><span class="pstrut" style="height:3em;"></span><span class="sizing reset-size6 size3 mtight"><span class="mord mtight"><span class="mord mtight"><span class="mopen nulldelimiter sizing reset-size3 size6"></span><span class="mfrac"><span class="vlist-t vlist-t2"><span class="vlist-r"><span class="vlist" style="height:0.8443142857142858em;"><span style="top:-2.656em;"><span style="height:3em;" class="pstrut"></span><span class="sizing reset-size3 size1 mtight"><span class="mord mtight"><span class="mord mtight">2</span><span class="mord mtight">.</span><span class="mord mtight">4</span></span></span></span><span style="top:-3.2255000000000003em;"><span class="pstrut" style="height:3em;"></span><span style="border-bottom-width:0.049em;" class="frac-line mtight"></span></span><span style="top:-3.384em;"><span class="pstrut" style="height:3em;"></span><span class="sizing reset-size3 size1 mtight"><span class="mord mtight"><span class="mord mtight">1</span></span></span></span></span><span class="vlist-s">​</span></span><span class="vlist-r"><span style="height:0.344em;" class="vlist"><span></span></span></span></span></span><span class="mclose nulldelimiter sizing reset-size3 size6"></span></span></span></span></span></span></span></span></span></span><span class="mspace" style="margin-right:0.2222222222222222em;"></span><span class="mbin">−</span><span style="margin-right:0.2222222222222222em;" class="mspace"></span><span class="mord">0</span><span class="mord">.</span><span class="mord">0</span><span class="mord">5</span><span class="mord">5</span></span></span><span style="top:-2.25em;"><span class="pstrut" style="height:3.008em;"></span><span class="mord"><span class="mord">1</span><span class="mord">2</span><span class="mord">.</span><span class="mord">9</span><span class="mord">2</span><span class="mspace" style="margin-right:0.2222222222222222em;"></span><span class="mbin">×</span><span style="margin-right:0.2222222222222222em;" class="mspace"></span><span class="mord mathdefault">L</span></span></span></span><span class="vlist-s">​</span></span><span class="vlist-r"><span class="vlist" style="height:1.19em;"><span></span></span></span></span></span><span class="arraycolsep" style="width:1em;"></span><span class="col-align-l"><span class="vlist-t vlist-t2"><span class="vlist-r"><span class="vlist" style="height:1.69em;"><span style="top:-3.69em;"><span class="pstrut" style="height:3.008em;"></span><span class="mord"><span class="mord text"><span class="mord">for</span></span><span class="mspace">&nbsp;</span><span class="mord">0</span><span class="mord">.</span><span class="mord">0</span><span class="mord">0</span><span class="mord">3</span><span class="mord">0</span><span class="mord">1</span><span class="mord">8</span><span class="mord">6</span><span style="margin-right:0.2777777777777778em;" class="mspace"></span><span class="mrel">≤</span><span class="mspace" style="margin-right:0.2777777777777778em;"></span><span class="mord mathdefault">L</span><span style="margin-right:0.2777777777777778em;" class="mspace"></span><span class="mrel">≤</span><span class="mspace" style="margin-right:0.2777777777777778em;"></span><span class="mord">1</span></span></span><span style="top:-2.25em;"><span class="pstrut" style="height:3.008em;"></span><span class="mord"><span class="mord text"><span class="mord">for</span></span><span class="mspace">&nbsp;</span><span class="mord">0</span><span class="mspace" style="margin-right:0.2777777777777778em;"></span><span class="mrel">≤</span><span class="mspace" style="margin-right:0.2777777777777778em;"></span><span class="mord mathdefault">L</span><span class="mspace" style="margin-right:0.2777777777777778em;"></span><span class="mrel">&lt;</span><span class="mspace" style="margin-right:0.2777777777777778em;"></span><span class="mord">0</span><span class="mord">.</span><span class="mord">0</span><span class="mord">0</span><span class="mord">3</span><span class="mord">0</span><span class="mord">1</span><span class="mord">8</span><span class="mord">6</span></span></span></span><span class="vlist-s">​</span></span><span class="vlist-r"><span style="height:1.19em;" class="vlist"><span></span></span></span></span></span></span></span><span class="mclose nulldelimiter"></span></span></span></span></span><span class="vlist-s">​</span></span><span class="vlist-r"><span class="vlist" style="height:1.400015em;"><span></span></span></span></span></span></span></span></span></span></span>where L is the linear value of a color component and E is the
///encoded value (as stored in the image in memory).
///# Related
/// - [`VK_KHR_surface`]
/// - [`SurfaceFormatKHR`]
/// - [`SwapchainCreateInfoKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkColorSpaceKHR")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct ColorSpaceKHR(i32);
impl const Default for ColorSpaceKHR {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for ColorSpaceKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple("ColorSpaceKHR")
            .field(match *self {
                Self::COLOR_SPACE_SRGB_NONLINEAR => &"COLOR_SPACE_SRGB_NONLINEAR",
                Self::COLOR_SPACE_DISPLAY_P3_NONLINEAR_EXT => &"COLOR_SPACE_DISPLAY_P3_NONLINEAR_EXT",
                Self::COLOR_SPACE_EXTENDED_SRGB_LINEAR_EXT => &"COLOR_SPACE_EXTENDED_SRGB_LINEAR_EXT",
                Self::COLOR_SPACE_DISPLAY_P3_LINEAR_EXT => &"COLOR_SPACE_DISPLAY_P3_LINEAR_EXT",
                Self::COLOR_SPACE_DCI_P3_NONLINEAR_EXT => &"COLOR_SPACE_DCI_P3_NONLINEAR_EXT",
                Self::COLOR_SPACE_BT709_LINEAR_EXT => &"COLOR_SPACE_BT709_LINEAR_EXT",
                Self::COLOR_SPACE_BT709_NONLINEAR_EXT => &"COLOR_SPACE_BT709_NONLINEAR_EXT",
                Self::COLOR_SPACE_BT2020_LINEAR_EXT => &"COLOR_SPACE_BT2020_LINEAR_EXT",
                Self::COLOR_SPACE_HDR10_ST2084_EXT => &"COLOR_SPACE_HDR10_ST2084_EXT",
                Self::COLOR_SPACE_DOLBYVISION_EXT => &"COLOR_SPACE_DOLBYVISION_EXT",
                Self::COLOR_SPACE_HDR10_HLG_EXT => &"COLOR_SPACE_HDR10_HLG_EXT",
                Self::COLOR_SPACE_ADOBERGB_LINEAR_EXT => &"COLOR_SPACE_ADOBERGB_LINEAR_EXT",
                Self::COLOR_SPACE_ADOBERGB_NONLINEAR_EXT => &"COLOR_SPACE_ADOBERGB_NONLINEAR_EXT",
                Self::COLOR_SPACE_PASS_THROUGH_EXT => &"COLOR_SPACE_PASS_THROUGH_EXT",
                Self::COLOR_SPACE_EXTENDED_SRGB_NONLINEAR_EXT => &"COLOR_SPACE_EXTENDED_SRGB_NONLINEAR_EXT",
                Self::COLOR_SPACE_DISPLAY_NATIVE_AMD => &"COLOR_SPACE_DISPLAY_NATIVE_AMD",
                other => unreachable!("invalid value for `ColorSpaceKHR`: {:?}", other),
            })
            .finish()
    }
}
impl ColorSpaceKHR {
    ///[`COLOR_SPACE_SRGB_NONLINEAR`] specifies support for the sRGB
    ///color space.
    pub const COLOR_SPACE_SRGB_NONLINEAR: Self = Self(0);
    ///[`COLOR_SPACE_DISPLAY_P3_NONLINEAR_EXT`] specifies support for the
    ///Display-P3 color space to be displayed using an sRGB-like EOTF (defined
    ///below).
    ///
    ///Provided by [`crate::extensions::ext_swapchain_colorspace`]
    pub const COLOR_SPACE_DISPLAY_P3_NONLINEAR_EXT: Self = Self(1000104001);
    ///[`COLOR_SPACE_EXTENDED_SRGB_LINEAR_EXT`] specifies support for the
    ///extended sRGB color space to be displayed using a linear EOTF.
    ///
    ///Provided by [`crate::extensions::ext_swapchain_colorspace`]
    pub const COLOR_SPACE_EXTENDED_SRGB_LINEAR_EXT: Self = Self(1000104002);
    ///[`COLOR_SPACE_DISPLAY_P3_LINEAR_EXT`] specifies support for the
    ///Display-P3 color space to be displayed using a linear EOTF.
    ///
    ///Provided by [`crate::extensions::ext_swapchain_colorspace`]
    pub const COLOR_SPACE_DISPLAY_P3_LINEAR_EXT: Self = Self(1000104003);
    ///[`COLOR_SPACE_DCI_P3_NONLINEAR_EXT`] specifies support for the
    ///DCI-P3 color space to be displayed using the DCI-P3 EOTF.
    ///Note that values in such an image are interpreted as XYZ encoded color
    ///data by the presentation engine.
    ///
    ///Provided by [`crate::extensions::ext_swapchain_colorspace`]
    pub const COLOR_SPACE_DCI_P3_NONLINEAR_EXT: Self = Self(1000104004);
    ///[`COLOR_SPACE_BT709_LINEAR_EXT`] specifies support for the BT709
    ///color space to be displayed using a linear EOTF.
    ///
    ///Provided by [`crate::extensions::ext_swapchain_colorspace`]
    pub const COLOR_SPACE_BT709_LINEAR_EXT: Self = Self(1000104005);
    ///[`COLOR_SPACE_BT709_NONLINEAR_EXT`] specifies support for the BT709
    ///color space to be displayed using the SMPTE 170M EOTF.
    ///
    ///Provided by [`crate::extensions::ext_swapchain_colorspace`]
    pub const COLOR_SPACE_BT709_NONLINEAR_EXT: Self = Self(1000104006);
    ///[`COLOR_SPACE_BT2020_LINEAR_EXT`] specifies support for the BT2020
    ///color space to be displayed using a linear EOTF.
    ///
    ///Provided by [`crate::extensions::ext_swapchain_colorspace`]
    pub const COLOR_SPACE_BT2020_LINEAR_EXT: Self = Self(1000104007);
    ///[`COLOR_SPACE_HDR10_ST2084_EXT`] specifies support for the HDR10
    ///(BT2020 color) space to be displayed using the SMPTE ST2084 Perceptual
    ///Quantizer (PQ) EOTF.
    ///
    ///Provided by [`crate::extensions::ext_swapchain_colorspace`]
    pub const COLOR_SPACE_HDR10_ST2084_EXT: Self = Self(1000104008);
    ///[`COLOR_SPACE_DOLBYVISION_EXT`] specifies support for the Dolby
    ///Vision (BT2020 color space), proprietary encoding, to be displayed using
    ///the SMPTE ST2084 EOTF.
    ///
    ///Provided by [`crate::extensions::ext_swapchain_colorspace`]
    pub const COLOR_SPACE_DOLBYVISION_EXT: Self = Self(1000104009);
    ///[`COLOR_SPACE_HDR10_HLG_EXT`] specifies support for the HDR10
    ///(BT2020 color space) to be displayed using the Hybrid Log Gamma (HLG)
    ///EOTF.
    ///
    ///Provided by [`crate::extensions::ext_swapchain_colorspace`]
    pub const COLOR_SPACE_HDR10_HLG_EXT: Self = Self(1000104010);
    ///[`COLOR_SPACE_ADOBERGB_LINEAR_EXT`] specifies support for the
    ///AdobeRGB color space to be displayed using a linear EOTF.
    ///
    ///Provided by [`crate::extensions::ext_swapchain_colorspace`]
    pub const COLOR_SPACE_ADOBERGB_LINEAR_EXT: Self = Self(1000104011);
    ///[`COLOR_SPACE_ADOBERGB_NONLINEAR_EXT`] specifies support for the
    ///AdobeRGB color space to be displayed using the Gamma 2.2 EOTF.
    ///
    ///Provided by [`crate::extensions::ext_swapchain_colorspace`]
    pub const COLOR_SPACE_ADOBERGB_NONLINEAR_EXT: Self = Self(1000104012);
    ///[`COLOR_SPACE_PASS_THROUGH_EXT`] specifies that color components
    ///are used “as is”.
    ///This is intended to allow applications to supply data for color spaces
    ///not described here.
    ///
    ///Provided by [`crate::extensions::ext_swapchain_colorspace`]
    pub const COLOR_SPACE_PASS_THROUGH_EXT: Self = Self(1000104013);
    ///[`COLOR_SPACE_EXTENDED_SRGB_NONLINEAR_EXT`] specifies support for
    ///the extended sRGB color space to be displayed using an sRGB EOTF.
    ///
    ///Provided by [`crate::extensions::ext_swapchain_colorspace`]
    pub const COLOR_SPACE_EXTENDED_SRGB_NONLINEAR_EXT: Self = Self(1000104014);
    ///[`COLOR_SPACE_DISPLAY_NATIVE_AMD`] specifies support for the
    ///display’s native color space.
    ///This matches the color space expectations of AMD’s FreeSync2 standard,
    ///for displays supporting it.
    ///
    ///Provided by [`crate::extensions::amd_display_native_hdr`]
    pub const COLOR_SPACE_DISPLAY_NATIVE_AMD: Self = Self(1000213000);
    ///No documentation found
    pub const COLORSPACE_SRGB_NONLINEAR: Self = Self::COLOR_SPACE_SRGB_NONLINEAR;
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_swapchain_colorspace`]
    pub const COLOR_SPACE_DCI_P3_LINEAR_EXT: Self = Self::COLOR_SPACE_DISPLAY_P3_LINEAR_EXT;
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
