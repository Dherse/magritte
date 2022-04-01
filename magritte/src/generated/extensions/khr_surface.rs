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
//! - Extending [`ObjectType`]:  - `VK_OBJECT_TYPE_SURFACE_KHR`
//! - Extending [`VulkanResultCodes`]:  - `VK_ERROR_NATIVE_WINDOW_IN_USE_KHR`  -
//!   `VK_ERROR_SURFACE_LOST_KHR`
//!# Known issues & F.A.Q
//!1) Should this extension include a method to query whether a physical device
//!supports presenting to a specific window or native surface on a given
//!platform? **RESOLVED** : Yes.
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
//![`Device`]? **RESOLVED** : Yes.
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
//!System platform? **RESOLVED** : Both.
//!XCB is a more modern and efficient API, but Xlib usage is deeply ingrained
//!in many applications and likely will remain in use for the foreseeable
//!future.
//!Not all drivers necessarily need to support both, but including both as
//!options in the core specification will probably encourage support, which
//!should in turn ease adoption of the Vulkan API in older codebases.
//!Additionally, the performance improvements possible with XCB likely will not
//!have a measurable impact on the performance of Vulkan presentation and other
//!minimal window system interactions defined here.4) Should the GBM platform be included in the
//! list of platform enums? **RESOLVED** : Deferred, and will be addressed with a platform-specific
//!extension to be written in the future.
//!# Version History
//! - Revision 1, 2015-05-20 (James Jones)  - Initial draft, based on LunarG KHR spec, other KHR
//!   specs, patches attached to bugs.
//! - Revision 2, 2015-05-22 (Ian Elliott)  - Created initial Description section.  - Removed query
//!   for whether a platform requires the use of a queue for presentation, since it was decided that
//!   presentation will always be modeled as being part of the queue.  - Fixed typos and other minor
//!   mistakes.
//! - Revision 3, 2015-05-26 (Ian Elliott)  - Improved the Description section.
//! - Revision 4, 2015-05-27 (James Jones)  - Fixed compilation errors in example code.
//! - Revision 5, 2015-06-01 (James Jones)  - Added issues 1 and 2 and made related spec updates.
//! - Revision 6, 2015-06-01 (James Jones)  - Merged the platform type mappings table previously
//!   removed from VK_KHR_swapchain with the platform description table in this spec.  - Added
//!   issues 3 and 4 documenting choices made when building the initial list of native platforms
//!   supported.
//! - Revision 7, 2015-06-11 (Ian Elliott)  - Updated table 1 per input from the KHR TSG.  - Updated
//!   issue 4 (GBM) per discussion with Daniel Stone. He will create a platform-specific extension
//!   sometime in the future.
//! - Revision 8, 2015-06-17 (James Jones)  - Updated enum-extending values using new convention.  -
//!   Fixed the value of VK_SURFACE_PLATFORM_INFO_TYPE_SUPPORTED_KHR.
//! - Revision 9, 2015-06-17 (James Jones)  - Rebased on Vulkan API version 126.
//! - Revision 10, 2015-06-18 (James Jones)  - Marked issues 2 and 3 resolved.
//! - Revision 11, 2015-06-23 (Ian Elliott)  - Examples now show use of function pointers for
//!   extension functions.  - Eliminated extraneous whitespace.
//! - Revision 12, 2015-07-07 (Daniel Rakos)  - Added error section describing when each error is
//!   expected to be reported.  - Replaced the term “queue node index” with “queue family index” in
//!   the spec as that is the agreed term to be used in the latest version of the core header and
//!   spec.  - Replaced bool32_t with VkBool32.
//! - Revision 13, 2015-08-06 (Daniel Rakos)  - Updated spec against latest core API header version.
//! - Revision 14, 2015-08-20 (Ian Elliott)  - Renamed this extension and all of its enumerations,
//!   types, functions, etc. This makes it compliant with the proposed standard for Vulkan
//!   extensions.  - Switched from “revision” to “version”, including use of the VK_MAKE_VERSION
//!   macro in the header file.  - Did miscellaneous cleanup, etc.
//! - Revision 15, 2015-08-20 (Ian Elliott—​porting a 2015-07-29 change from James Jones)  - Moved
//!   the surface transform enums here from VK_WSI_swapchain so they could be reused by
//!   VK_WSI_display.
//! - Revision 16, 2015-09-01 (James Jones)  - Restore single-field revision number.
//! - Revision 17, 2015-09-01 (James Jones)  - Fix example code compilation errors.
//! - Revision 18, 2015-09-26 (Jesse Hall)  - Replaced VkSurfaceDescriptionKHR with the VkSurfaceKHR
//!   object, which is created via layered extensions. Added VkDestroySurfaceKHR.
//! - Revision 19, 2015-09-28 (Jesse Hall)  - Renamed from VK_EXT_KHR_swapchain to
//!   VK_EXT_KHR_surface.
//! - Revision 20, 2015-09-30 (Jeff Vigil)  - Add error result VK_ERROR_SURFACE_LOST_KHR.
//! - Revision 21, 2015-10-15 (Daniel Rakos)  - Updated the resolution of issue #2 and include the
//!   surface capability queries in this extension.  - Renamed SurfaceProperties to
//!   SurfaceCapabilities as it better reflects that the values returned are the capabilities of the
//!   surface on a particular device.  - Other minor cleanup and consistency changes.
//! - Revision 22, 2015-10-26 (Ian Elliott)  - Renamed from VK_EXT_KHR_surface to VK_KHR_surface.
//! - Revision 23, 2015-11-03 (Daniel Rakos)  - Added allocation callbacks to vkDestroySurfaceKHR.
//! - Revision 24, 2015-11-10 (Jesse Hall)  - Removed VkSurfaceTransformKHR. Use
//!   VkSurfaceTransformFlagBitsKHR instead.  - Rename VkSurfaceCapabilitiesKHR member
//!   maxImageArraySize to maxImageArrayLayers.
//! - Revision 25, 2016-01-14 (James Jones)  - Moved VK_ERROR_NATIVE_WINDOW_IN_USE_KHR from the
//!   VK_KHR_android_surface to the VK_KHR_surface extension.
//! - 2016-08-23 (Ian Elliott)  - Update the example code, to not have so many characters per line,
//!   and to split out a new example to show how to obtain function pointers.
//! - 2016-08-25 (Ian Elliott)  - A note was added at the beginning of the example code, stating
//!   that it will be removed from future versions of the appendix.
//!# Other info
//! * 2016-08-25
//! * No known IP claims.
//! * - Patrick Doane, Blizzard  - Ian Elliott, LunarG  - Jesse Hall, Google  - James Jones, NVIDIA
//!   - David Mao, AMD  - Norbert Nopper, Freescale  - Alon Or-bach, Samsung  - Daniel Rakos, AMD  -
//!   Graham Sellers, AMD  - Jeff Vigil, Qualcomm  - Chia-I Wu, LunarG  - Jason Ekstrand, Intel
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
use crate::{
    extensions::khr_display::SurfaceTransformFlagsKHR,
    vulkan1_0::{Extent2D, Format, ImageUsageFlags},
};
#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{
    ffi::CStr,
    iter::{Extend, FromIterator, IntoIterator},
};
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
/// - [`PresentModeImmediateKhr`] specifies that the presentation engine does not wait for a
///   vertical blanking period to update the current image, meaning this mode  **may**  result in
///   visible tearing. No internal queuing of presentation requests is needed, as the requests are
///   applied immediately.
/// - [`PresentModeMailboxKhr`] specifies that the presentation engine waits for the next vertical
///   blanking period to update the current image. Tearing  **cannot**  be observed. An internal
///   single-entry queue is used to hold pending presentation requests. If the queue is full when a
///   new presentation request is received, the new request replaces the existing entry, and any
///   images associated with the prior entry become available for re-use by the application. One
///   request is removed from the queue and processed during each vertical blanking period in which
///   the queue is non-empty.
/// - [`PresentModeFifoKhr`] specifies that the presentation engine waits for the next vertical
///   blanking period to update the current image. Tearing  **cannot**  be observed. An internal
///   queue is used to hold pending presentation requests. New requests are appended to the end of
///   the queue, and one request is removed from the beginning of the queue and processed during
///   each vertical blanking period in which the queue is non-empty. This is the only value of
///   `presentMode` that is  **required**  to be supported.
/// - [`PresentModeFifoRelaxedKhr`] specifies that the presentation engine generally waits for the
///   next vertical blanking period to update the current image. If a vertical blanking period has
///   already passed since the last update of the current image then the presentation engine does
///   not wait for another vertical blanking period for the update, meaning this mode  **may**
///   result in visible tearing in this case. This mode is useful for reducing visual stutter with
///   an application that will mostly present a new image before the next vertical blanking period,
///   but may occasionally be late, and present a new image just after the next vertical blanking
///   period. An internal queue is used to hold pending presentation requests. New requests are
///   appended to the end of the queue, and one request is removed from the beginning of the queue
///   and processed during or after each vertical blanking period in which the queue is non-empty.
/// - [`PresentModeSharedDemandRefreshKhr`] specifies that the presentation engine and application
///   have concurrent access to a single image, which is referred to as a *shared presentable
///   image*. The presentation engine is only required to update the current image after a new
///   presentation request is received. Therefore the application  **must**  make a presentation
///   request whenever an update is required. However, the presentation engine  **may**  update the
///   current image at any point, meaning this mode  **may**  result in visible tearing.
/// - [`PresentModeSharedContinuousRefreshKhr`] specifies that the presentation engine and
///   application have concurrent access to a single image, which is referred to as a *shared
///   presentable image*. The presentation engine periodically updates the current image on its
///   regular refresh cycle. The application is only required to make one initial presentation
///   request, after which the presentation engine  **must**  update the current image without any
///   need for further presentation requests. The application  **can**  indicate the image contents
///   have been updated by making a presentation request, but this does not guarantee the timing of
///   when it will be updated. This mode  **may**  result in visible tearing if rendering to the
///   image is not timed correctly.
///The supported [`ImageUsageFlagBits`] of the presentable images of a
///swapchain created for a surface  **may**  differ depending on the presentation
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
#[repr(i32)]
pub enum PresentModeKHR {
    ///[`PresentModeImmediateKhr`] specifies that the presentation
    ///engine does not wait for a vertical blanking period to update the
    ///current image, meaning this mode  **may**  result in visible tearing.
    ///No internal queuing of presentation requests is needed, as the requests
    ///are applied immediately.
    PresentModeImmediateKhr = 0,
    ///[`PresentModeMailboxKhr`] specifies that the presentation engine
    ///waits for the next vertical blanking period to update the current image.
    ///Tearing  **cannot**  be observed.
    ///An internal single-entry queue is used to hold pending presentation
    ///requests.
    ///If the queue is full when a new presentation request is received, the
    ///new request replaces the existing entry, and any images associated with
    ///the prior entry become available for re-use by the application.
    ///One request is removed from the queue and processed during each vertical
    ///blanking period in which the queue is non-empty.
    PresentModeMailboxKhr = 1,
    ///[`PresentModeFifoKhr`] specifies that the presentation engine
    ///waits for the next vertical blanking period to update the current image.
    ///Tearing  **cannot**  be observed.
    ///An internal queue is used to hold pending presentation requests.
    ///New requests are appended to the end of the queue, and one request is
    ///removed from the beginning of the queue and processed during each
    ///vertical blanking period in which the queue is non-empty.
    ///This is the only value of `presentMode` that is  **required**  to be
    ///supported.
    PresentModeFifoKhr = 2,
    ///[`PresentModeFifoRelaxedKhr`] specifies that the presentation
    ///engine generally waits for the next vertical blanking period to update
    ///the current image.
    ///If a vertical blanking period has already passed since the last update
    ///of the current image then the presentation engine does not wait for
    ///another vertical blanking period for the update, meaning this mode  **may**
    ///result in visible tearing in this case.
    ///This mode is useful for reducing visual stutter with an application that
    ///will mostly present a new image before the next vertical blanking
    ///period, but may occasionally be late, and present a new image just after
    ///the next vertical blanking period.
    ///An internal queue is used to hold pending presentation requests.
    ///New requests are appended to the end of the queue, and one request is
    ///removed from the beginning of the queue and processed during or after
    ///each vertical blanking period in which the queue is non-empty.
    PresentModeFifoRelaxedKhr = 3,
    ///[`PresentModeSharedDemandRefreshKhr`] specifies that the
    ///presentation engine and application have concurrent access to a single
    ///image, which is referred to as a *shared presentable image*.
    ///The presentation engine is only required to update the current image
    ///after a new presentation request is received.
    ///Therefore the application  **must**  make a presentation request whenever an
    ///update is required.
    ///However, the presentation engine  **may**  update the current image at any
    ///point, meaning this mode  **may**  result in visible tearing.
    ///
    ///Provided by [`crate::extensions::khr_shared_presentable_image`]
    #[cfg(feature = "VK_KHR_shared_presentable_image")]
    PresentModeSharedDemandRefreshKhr = 1000111000,
    ///[`PresentModeSharedContinuousRefreshKhr`] specifies that the
    ///presentation engine and application have concurrent access to a single
    ///image, which is referred to as a *shared presentable image*.
    ///The presentation engine periodically updates the current image on its
    ///regular refresh cycle.
    ///The application is only required to make one initial presentation
    ///request, after which the presentation engine  **must**  update the current
    ///image without any need for further presentation requests.
    ///The application  **can**  indicate the image contents have been updated by
    ///making a presentation request, but this does not guarantee the timing of
    ///when it will be updated.
    ///This mode  **may**  result in visible tearing if rendering to the image is
    ///not timed correctly.
    ///
    ///Provided by [`crate::extensions::khr_shared_presentable_image`]
    #[cfg(feature = "VK_KHR_shared_presentable_image")]
    PresentModeSharedContinuousRefreshKhr = 1000111001,
}
impl const Default for PresentModeKHR {
    fn default() -> Self {
        Self::PresentModeImmediateKhr
    }
}
impl PresentModeKHR {
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> i32 {
        *self as i32
    }
    ///Gets a value from a raw underlying value, unchecked and therefore unsafe
    #[inline]
    pub const unsafe fn from_bits(bits: i32) -> i32 {
        std::mem::transmute(bits)
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
/// - [`ColorSpaceSrgbNonlinearKhr`] specifies support for the sRGB color space.
/// - [`ColorSpaceDisplayP3NonlinearExt`] specifies support for the Display-P3 color space to be
///   displayed using an sRGB-like EOTF (defined below).
/// - [`ColorSpaceExtendedSrgbLinearExt`] specifies support for the extended sRGB color space to be
///   displayed using a linear EOTF.
/// - [`ColorSpaceExtendedSrgbNonlinearExt`] specifies support for the extended sRGB color space to
///   be displayed using an sRGB EOTF.
/// - [`ColorSpaceDisplayP3LinearExt`] specifies support for the Display-P3 color space to be
///   displayed using a linear EOTF.
/// - [`ColorSpaceDciP3NonlinearExt`] specifies support for the DCI-P3 color space to be displayed
///   using the DCI-P3 EOTF. Note that values in such an image are interpreted as XYZ encoded color
///   data by the presentation engine.
/// - [`ColorSpaceBt709LinearExt`] specifies support for the BT709 color space to be displayed using
///   a linear EOTF.
/// - [`ColorSpaceBt709NonlinearExt`] specifies support for the BT709 color space to be displayed
///   using the SMPTE 170M EOTF.
/// - [`ColorSpaceBt2020LinearExt`] specifies support for the BT2020 color space to be displayed
///   using a linear EOTF.
/// - [`ColorSpaceHdr10St2084Ext`] specifies support for the HDR10 (BT2020 color) space to be
///   displayed using the SMPTE ST2084 Perceptual Quantizer (PQ) EOTF.
/// - [`ColorSpaceDolbyvisionExt`] specifies support for the Dolby Vision (BT2020 color space),
///   proprietary encoding, to be displayed using the SMPTE ST2084 EOTF.
/// - [`ColorSpaceHdr10HlgExt`] specifies support for the HDR10 (BT2020 color space) to be displayed
///   using the Hybrid Log Gamma (HLG) EOTF.
/// - [`ColorSpaceAdobergbLinearExt`] specifies support for the AdobeRGB color space to be displayed
///   using a linear EOTF.
/// - [`ColorSpaceAdobergbNonlinearExt`] specifies support for the AdobeRGB color space to be
///   displayed using the Gamma 2.2 EOTF.
/// - [`ColorSpacePassThroughExt`] specifies that color components are used “as is”. This is
///   intended to allow applications to supply data for color spaces not described here.
/// - [`ColorSpaceDisplayNativeAmd`] specifies support for the display’s native color space. This
///   matches the color space expectations of AMD’s FreeSync2 standard, for displays supporting it.
///The color components of non-linear color space swap chain images  **must**  have
///had the appropriate transfer function applied.
///The color space selected for the swap chain image will not affect the
///processing of data written into the image by the implementation.
///Vulkan requires that all implementations support the sRGB transfer function
///by use of an SRGB pixel format.
///Other transfer functions, such as SMPTE 170M or SMPTE2084,  **can**  be performed
///by the application shader.
///This extension defines enums for [`ColorSpaceKHR`] that correspond to
///the following color spaces:The transfer functions are described in the “Transfer Functions”
/// chapter
///of the [Khronos Data Format Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#data-format).Except Display-P3 OETF, which is:<span class="katex"><span aria-hidden="true" class="katex-html"><span class="base"><span class="strut" style="height:3.30003em;vertical-align:-1.400015em;"></span><span class="mord"><span class="mtable"><span class="col-align-r"><span class="vlist-t vlist-t2"><span class="vlist-r"><span style="height:1.900015em;" class="vlist"><span style="top:-3.9000150000000002em;"><span class="pstrut" style="height:3.75em;"></span><span class="mord"><span style="margin-right:0.05764em;" class="mord mathdefault">E</span></span></span></span><span class="vlist-s">​</span></span><span class="vlist-r"><span style="height:1.400015em;" class="vlist"><span></span></span></span></span></span><span class="col-align-l"><span class="vlist-t vlist-t2"><span class="vlist-r"><span class="vlist" style="height:1.900015em;"><span style="top:-3.9000150000000002em;"><span class="pstrut" style="height:3.75em;"></span><span class="mord"><span class="mord"></span><span class="mspace" style="margin-right:0.2777777777777778em;"></span><span class="mrel">=</span><span style="margin-right:0.2777777777777778em;" class="mspace"></span><span class="minner"><span style="top:0em;" class="mopen delimcenter"><span class="delimsizing size4">{</span></span><span class="mord"><span class="mtable"><span class="col-align-l"><span class="vlist-t vlist-t2"><span class="vlist-r"><span class="vlist" style="height:1.69em;"><span style="top:-3.69em;"><span style="height:3.008em;" class="pstrut"></span><span class="mord"><span class="mord">1</span><span class="mord">.</span><span class="mord">0</span><span class="mord">5</span><span class="mord">5</span><span style="margin-right:0.2222222222222222em;" class="mspace"></span><span class="mbin">×</span><span class="mspace" style="margin-right:0.2222222222222222em;"></span><span class="mord"><span class="mord mathdefault">L</span><span class="msupsub"><span class="vlist-t"><span class="vlist-r"><span style="height:0.9540200000000001em;" class="vlist"><span style="top:-3.363em;margin-right:0.05em;"><span style="height:3em;" class="pstrut"></span><span class="sizing reset-size6 size3 mtight"><span class="mord mtight"><span class="mord mtight"><span class="mopen nulldelimiter sizing reset-size3 size6"></span><span class="mfrac"><span class="vlist-t vlist-t2"><span class="vlist-r"><span class="vlist" style="height:0.8443142857142858em;"><span style="top:-2.656em;"><span style="height:3em;" class="pstrut"></span><span class="sizing reset-size3 size1 mtight"><span class="mord mtight"><span class="mord mtight">2</span><span class="mord mtight">.</span><span class="mord mtight">4</span></span></span></span><span style="top:-3.2255000000000003em;"><span style="height:3em;" class="pstrut"></span><span class="frac-line mtight" style="border-bottom-width:0.049em;"></span></span><span style="top:-3.384em;"><span class="pstrut" style="height:3em;"></span><span class="sizing reset-size3 size1 mtight"><span class="mord mtight"><span class="mord mtight">1</span></span></span></span></span><span class="vlist-s">​</span></span><span class="vlist-r"><span class="vlist" style="height:0.344em;"><span></span></span></span></span></span><span class="mclose nulldelimiter sizing reset-size3 size6"></span></span></span></span></span></span></span></span></span></span><span class="mspace" style="margin-right:0.2222222222222222em;"></span><span class="mbin">−</span><span class="mspace" style="margin-right:0.2222222222222222em;"></span><span class="mord">0</span><span class="mord">.</span><span class="mord">0</span><span class="mord">5</span><span class="mord">5</span></span></span><span style="top:-2.25em;"><span style="height:3.008em;" class="pstrut"></span><span class="mord"><span class="mord">1</span><span class="mord">2</span><span class="mord">.</span><span class="mord">9</span><span class="mord">2</span><span style="margin-right:0.2222222222222222em;" class="mspace"></span><span class="mbin">×</span><span class="mspace" style="margin-right:0.2222222222222222em;"></span><span class="mord mathdefault">L</span></span></span></span><span class="vlist-s">​</span></span><span class="vlist-r"><span class="vlist" style="height:1.19em;"><span></span></span></span></span></span><span style="width:1em;" class="arraycolsep"></span><span class="col-align-l"><span class="vlist-t vlist-t2"><span class="vlist-r"><span class="vlist" style="height:1.69em;"><span style="top:-3.69em;"><span style="height:3.008em;" class="pstrut"></span><span class="mord"><span class="mord text"><span class="mord">for</span></span><span class="mspace">&nbsp;</span><span class="mord">0</span><span class="mord">.</span><span class="mord">0</span><span class="mord">0</span><span class="mord">3</span><span class="mord">0</span><span class="mord">1</span><span class="mord">8</span><span class="mord">6</span><span class="mspace" style="margin-right:0.2777777777777778em;"></span><span class="mrel">≤</span><span class="mspace" style="margin-right:0.2777777777777778em;"></span><span class="mord mathdefault">L</span><span style="margin-right:0.2777777777777778em;" class="mspace"></span><span class="mrel">≤</span><span style="margin-right:0.2777777777777778em;" class="mspace"></span><span class="mord">1</span></span></span><span style="top:-2.25em;"><span style="height:3.008em;" class="pstrut"></span><span class="mord"><span class="mord text"><span class="mord">for</span></span><span class="mspace">&nbsp;</span><span class="mord">0</span><span class="mspace" style="margin-right:0.2777777777777778em;"></span><span class="mrel">≤</span><span class="mspace" style="margin-right:0.2777777777777778em;"></span><span class="mord mathdefault">L</span><span style="margin-right:0.2777777777777778em;" class="mspace"></span><span class="mrel">&lt;</span><span class="mspace" style="margin-right:0.2777777777777778em;"></span><span class="mord">0</span><span class="mord">.</span><span class="mord">0</span><span class="mord">0</span><span class="mord">3</span><span class="mord">0</span><span class="mord">1</span><span class="mord">8</span><span class="mord">6</span></span></span></span><span class="vlist-s">​</span></span><span class="vlist-r"><span class="vlist" style="height:1.19em;"><span></span></span></span></span></span></span></span><span class="mclose nulldelimiter"></span></span></span></span></span><span class="vlist-s">​</span></span><span class="vlist-r"><span class="vlist" style="height:1.400015em;"><span></span></span></span></span></span></span></span></span></span></span>where L is the linear value of a color component and E is the
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
#[repr(i32)]
pub enum ColorSpaceKHR {
    ///[`ColorSpaceSrgbNonlinearKhr`] specifies support for the sRGB
    ///color space.
    ColorSpaceSrgbNonlinearKhr = 0,
    ///[`ColorSpaceDisplayP3NonlinearExt`] specifies support for the
    ///Display-P3 color space to be displayed using an sRGB-like EOTF (defined
    ///below).
    ///
    ///Provided by [`crate::extensions::ext_swapchain_colorspace`]
    #[cfg(feature = "VK_EXT_swapchain_colorspace")]
    ColorSpaceDisplayP3NonlinearExt = 1000104001,
    ///[`ColorSpaceExtendedSrgbLinearExt`] specifies support for the
    ///extended sRGB color space to be displayed using a linear EOTF.
    ///
    ///Provided by [`crate::extensions::ext_swapchain_colorspace`]
    #[cfg(feature = "VK_EXT_swapchain_colorspace")]
    ColorSpaceExtendedSrgbLinearExt = 1000104002,
    ///[`ColorSpaceDisplayP3LinearExt`] specifies support for the
    ///Display-P3 color space to be displayed using a linear EOTF.
    ///
    ///Provided by [`crate::extensions::ext_swapchain_colorspace`]
    #[cfg(feature = "VK_EXT_swapchain_colorspace")]
    ColorSpaceDisplayP3LinearExt = 1000104003,
    ///[`ColorSpaceDciP3NonlinearExt`] specifies support for the
    ///DCI-P3 color space to be displayed using the DCI-P3 EOTF.
    ///Note that values in such an image are interpreted as XYZ encoded color
    ///data by the presentation engine.
    ///
    ///Provided by [`crate::extensions::ext_swapchain_colorspace`]
    #[cfg(feature = "VK_EXT_swapchain_colorspace")]
    ColorSpaceDciP3NonlinearExt = 1000104004,
    ///[`ColorSpaceBt709LinearExt`] specifies support for the BT709
    ///color space to be displayed using a linear EOTF.
    ///
    ///Provided by [`crate::extensions::ext_swapchain_colorspace`]
    #[cfg(feature = "VK_EXT_swapchain_colorspace")]
    ColorSpaceBt709LinearExt = 1000104005,
    ///[`ColorSpaceBt709NonlinearExt`] specifies support for the BT709
    ///color space to be displayed using the SMPTE 170M EOTF.
    ///
    ///Provided by [`crate::extensions::ext_swapchain_colorspace`]
    #[cfg(feature = "VK_EXT_swapchain_colorspace")]
    ColorSpaceBt709NonlinearExt = 1000104006,
    ///[`ColorSpaceBt2020LinearExt`] specifies support for the BT2020
    ///color space to be displayed using a linear EOTF.
    ///
    ///Provided by [`crate::extensions::ext_swapchain_colorspace`]
    #[cfg(feature = "VK_EXT_swapchain_colorspace")]
    ColorSpaceBt2020LinearExt = 1000104007,
    ///[`ColorSpaceHdr10St2084Ext`] specifies support for the HDR10
    ///(BT2020 color) space to be displayed using the SMPTE ST2084 Perceptual
    ///Quantizer (PQ) EOTF.
    ///
    ///Provided by [`crate::extensions::ext_swapchain_colorspace`]
    #[cfg(feature = "VK_EXT_swapchain_colorspace")]
    ColorSpaceHdr10St2084Ext = 1000104008,
    ///[`ColorSpaceDolbyvisionExt`] specifies support for the Dolby
    ///Vision (BT2020 color space), proprietary encoding, to be displayed using
    ///the SMPTE ST2084 EOTF.
    ///
    ///Provided by [`crate::extensions::ext_swapchain_colorspace`]
    #[cfg(feature = "VK_EXT_swapchain_colorspace")]
    ColorSpaceDolbyvisionExt = 1000104009,
    ///[`ColorSpaceHdr10HlgExt`] specifies support for the HDR10
    ///(BT2020 color space) to be displayed using the Hybrid Log Gamma (HLG)
    ///EOTF.
    ///
    ///Provided by [`crate::extensions::ext_swapchain_colorspace`]
    #[cfg(feature = "VK_EXT_swapchain_colorspace")]
    ColorSpaceHdr10HlgExt = 1000104010,
    ///[`ColorSpaceAdobergbLinearExt`] specifies support for the
    ///AdobeRGB color space to be displayed using a linear EOTF.
    ///
    ///Provided by [`crate::extensions::ext_swapchain_colorspace`]
    #[cfg(feature = "VK_EXT_swapchain_colorspace")]
    ColorSpaceAdobergbLinearExt = 1000104011,
    ///[`ColorSpaceAdobergbNonlinearExt`] specifies support for the
    ///AdobeRGB color space to be displayed using the Gamma 2.2 EOTF.
    ///
    ///Provided by [`crate::extensions::ext_swapchain_colorspace`]
    #[cfg(feature = "VK_EXT_swapchain_colorspace")]
    ColorSpaceAdobergbNonlinearExt = 1000104012,
    ///[`ColorSpacePassThroughExt`] specifies that color components
    ///are used “as is”.
    ///This is intended to allow applications to supply data for color spaces
    ///not described here.
    ///
    ///Provided by [`crate::extensions::ext_swapchain_colorspace`]
    #[cfg(feature = "VK_EXT_swapchain_colorspace")]
    ColorSpacePassThroughExt = 1000104013,
    ///[`ColorSpaceExtendedSrgbNonlinearExt`] specifies support for
    ///the extended sRGB color space to be displayed using an sRGB EOTF.
    ///
    ///Provided by [`crate::extensions::ext_swapchain_colorspace`]
    #[cfg(feature = "VK_EXT_swapchain_colorspace")]
    ColorSpaceExtendedSrgbNonlinearExt = 1000104014,
    ///[`ColorSpaceDisplayNativeAmd`] specifies support for the
    ///display’s native color space.
    ///This matches the color space expectations of AMD’s FreeSync2 standard,
    ///for displays supporting it.
    ///
    ///Provided by [`crate::extensions::amd_display_native_hdr`]
    #[cfg(feature = "VK_AMD_display_native_hdr")]
    ColorSpaceDisplayNativeAmd = 1000213000,
}
impl const Default for ColorSpaceKHR {
    fn default() -> Self {
        Self::ColorSpaceSrgbNonlinearKhr
    }
}
impl ColorSpaceKHR {
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> i32 {
        *self as i32
    }
    ///Gets a value from a raw underlying value, unchecked and therefore unsafe
    #[inline]
    pub const unsafe fn from_bits(bits: i32) -> i32 {
        std::mem::transmute(bits)
    }
}
///[VkCompositeAlphaFlagBitsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCompositeAlphaFlagBitsKHR.html) - Alpha compositing modes supported on a device
///# C Specifications
///The `supportedCompositeAlpha` member is of type
///[`CompositeAlphaFlagBitsKHR`], containing the following values:
///```c
///// Provided by VK_KHR_surface
///typedef enum VkCompositeAlphaFlagBitsKHR {
///    VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR = 0x00000001,
///    VK_COMPOSITE_ALPHA_PRE_MULTIPLIED_BIT_KHR = 0x00000002,
///    VK_COMPOSITE_ALPHA_POST_MULTIPLIED_BIT_KHR = 0x00000004,
///    VK_COMPOSITE_ALPHA_INHERIT_BIT_KHR = 0x00000008,
///} VkCompositeAlphaFlagBitsKHR;
///```
///# Description
///These values are described as follows:
/// - [`CompositeAlphaOpaqueKhr`]: The alpha component, if it exists, of the images is ignored in
///   the compositing process. Instead, the image is treated as if it has a constant alpha of 1.0.
/// - [`CompositeAlphaPreMultipliedKhr`]: The alpha component, if it exists, of the images is
///   respected in the compositing process. The non-alpha components of the image are expected to
///   already be multiplied by the alpha component by the application.
/// - [`CompositeAlphaPostMultipliedKhr`]: The alpha component, if it exists, of the images is
///   respected in the compositing process. The non-alpha components of the image are not expected
///   to already be multiplied by the alpha component by the application; instead, the compositor
///   will multiply the non-alpha components of the image by the alpha component during compositing.
/// - [`CompositeAlphaInheritKhr`]: The way in which the presentation engine treats the alpha
///   component in the images is unknown to the Vulkan API. Instead, the application is responsible
///   for setting the composite alpha blending mode using native window system commands. If the
///   application does not set the blending mode using native window system commands, then a
///   platform-specific default will be used.
///# Related
/// - [`VK_KHR_surface`]
/// - [`CompositeAlphaFlagsKHR`]
/// - [`SwapchainCreateInfoKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkCompositeAlphaFlagBitsKHR")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
#[repr(u32)]
pub enum CompositeAlphaFlagBitsKHR {
    #[doc(hidden)]
    Empty = 0,
    ///[`CompositeAlphaOpaqueKhr`]: The alpha component, if it
    ///exists, of the images is ignored in the compositing process.
    ///Instead, the image is treated as if it has a constant alpha of 1.0.
    CompositeAlphaOpaqueKhr = 1,
    ///[`CompositeAlphaPreMultipliedKhr`]: The alpha component, if
    ///it exists, of the images is respected in the compositing process.
    ///The non-alpha components of the image are expected to already be
    ///multiplied by the alpha component by the application.
    CompositeAlphaPreMultipliedKhr = 2,
    ///[`CompositeAlphaPostMultipliedKhr`]: The alpha component,
    ///if it exists, of the images is respected in the compositing process.
    ///The non-alpha components of the image are not expected to already be
    ///multiplied by the alpha component by the application; instead, the
    ///compositor will multiply the non-alpha components of the image by the
    ///alpha component during compositing.
    CompositeAlphaPostMultipliedKhr = 4,
    ///[`CompositeAlphaInheritKhr`]: The way in which the
    ///presentation engine treats the alpha component in the images is unknown
    ///to the Vulkan API.
    ///Instead, the application is responsible for setting the composite alpha
    ///blending mode using native window system commands.
    ///If the application does not set the blending mode using native window
    ///system commands, then a platform-specific default will be used.
    CompositeAlphaInheritKhr = 8,
}
impl const Default for CompositeAlphaFlagBitsKHR {
    fn default() -> Self {
        Self::Empty
    }
}
impl CompositeAlphaFlagBitsKHR {
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> u32 {
        *self as u32
    }
    ///Gets a value from a raw underlying value, unchecked and therefore unsafe
    #[inline]
    pub const unsafe fn from_bits(bits: u32) -> u32 {
        std::mem::transmute(bits)
    }
}
///[VkSurfaceTransformFlagBitsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSurfaceTransformFlagBitsKHR.html) - Presentation transforms supported on a device
///# C Specifications
///Bits which  **may**  be set in
///[`SurfaceCapabilitiesKHR::supported_transforms`] indicating the
///presentation transforms supported for the surface on the specified device,
///and possible values of
///[`SurfaceCapabilitiesKHR::current_transform`] indicating the
///surface’s current transform relative to the presentation engine’s natural
///orientation, are:
///```c
///// Provided by VK_KHR_surface
///typedef enum VkSurfaceTransformFlagBitsKHR {
///    VK_SURFACE_TRANSFORM_IDENTITY_BIT_KHR = 0x00000001,
///    VK_SURFACE_TRANSFORM_ROTATE_90_BIT_KHR = 0x00000002,
///    VK_SURFACE_TRANSFORM_ROTATE_180_BIT_KHR = 0x00000004,
///    VK_SURFACE_TRANSFORM_ROTATE_270_BIT_KHR = 0x00000008,
///    VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_BIT_KHR = 0x00000010,
///    VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_90_BIT_KHR = 0x00000020,
///    VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_180_BIT_KHR = 0x00000040,
///    VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_270_BIT_KHR = 0x00000080,
///    VK_SURFACE_TRANSFORM_INHERIT_BIT_KHR = 0x00000100,
///} VkSurfaceTransformFlagBitsKHR;
///```
///# Description
/// - [`SurfaceTransformIdentityKhr`] specifies that image content is presented without being
///   transformed.
/// - [`SurfaceTransformRotate90Khr`] specifies that image content is rotated 90 degrees clockwise.
/// - [`SurfaceTransformRotate180Khr`] specifies that image content is rotated 180 degrees
///   clockwise.
/// - [`SurfaceTransformRotate270Khr`] specifies that image content is rotated 270 degrees
///   clockwise.
/// - [`SurfaceTransformHorizontalMirrorKhr`] specifies that image content is mirrored horizontally.
/// - [`SurfaceTransformHorizontalMirrorRotate90Khr`] specifies that image content is mirrored
///   horizontally, then rotated 90 degrees clockwise.
/// - [`SurfaceTransformHorizontalMirrorRotate180Khr`] specifies that image content is mirrored
///   horizontally, then rotated 180 degrees clockwise.
/// - [`SurfaceTransformHorizontalMirrorRotate270Khr`] specifies that image content is mirrored
///   horizontally, then rotated 270 degrees clockwise.
/// - [`SurfaceTransformInheritKhr`] specifies that the presentation transform is not specified, and
///   is instead determined by platform-specific considerations and mechanisms outside Vulkan.
///# Related
/// - [`VK_KHR_surface`]
/// - [`CommandBufferInheritanceRenderPassTransformInfoQCOM`]
/// - [`CopyCommandTransformInfoQCOM`]
/// - [`DisplaySurfaceCreateInfoKHR`]
/// - [`RenderPassTransformBeginInfoQCOM`]
/// - [`SurfaceCapabilities2EXT`]
/// - [`SurfaceCapabilitiesKHR`]
/// - [`SurfaceTransformFlagsKHR`]
/// - [`SwapchainCreateInfoKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkSurfaceTransformFlagBitsKHR")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
#[repr(u32)]
pub enum SurfaceTransformFlagBitsKHR {
    #[doc(hidden)]
    Empty = 0,
    ///[`SurfaceTransformIdentityKhr`] specifies that image content
    ///is presented without being transformed.
    SurfaceTransformIdentityKhr = 1,
    ///[`SurfaceTransformRotate90Khr`] specifies that image
    ///content is rotated 90 degrees clockwise.
    SurfaceTransformRotate90Khr = 2,
    ///[`SurfaceTransformRotate180Khr`] specifies that image
    ///content is rotated 180 degrees clockwise.
    SurfaceTransformRotate180Khr = 4,
    ///[`SurfaceTransformRotate270Khr`] specifies that image
    ///content is rotated 270 degrees clockwise.
    SurfaceTransformRotate270Khr = 8,
    ///[`SurfaceTransformHorizontalMirrorKhr`] specifies that
    ///image content is mirrored horizontally.
    SurfaceTransformHorizontalMirrorKhr = 16,
    ///[`SurfaceTransformHorizontalMirrorRotate90Khr`] specifies
    ///that image content is mirrored horizontally, then rotated 90 degrees
    ///clockwise.
    SurfaceTransformHorizontalMirrorRotate90Khr = 32,
    ///[`SurfaceTransformHorizontalMirrorRotate180Khr`]
    ///specifies that image content is mirrored horizontally, then rotated 180
    ///degrees clockwise.
    SurfaceTransformHorizontalMirrorRotate180Khr = 64,
    ///[`SurfaceTransformHorizontalMirrorRotate270Khr`]
    ///specifies that image content is mirrored horizontally, then rotated 270
    ///degrees clockwise.
    SurfaceTransformHorizontalMirrorRotate270Khr = 128,
    ///[`SurfaceTransformInheritKhr`] specifies that the
    ///presentation transform is not specified, and is instead determined by
    ///platform-specific considerations and mechanisms outside Vulkan.
    SurfaceTransformInheritKhr = 256,
}
impl const Default for SurfaceTransformFlagBitsKHR {
    fn default() -> Self {
        Self::Empty
    }
}
impl SurfaceTransformFlagBitsKHR {
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> u32 {
        *self as u32
    }
    ///Gets a value from a raw underlying value, unchecked and therefore unsafe
    #[inline]
    pub const unsafe fn from_bits(bits: u32) -> u32 {
        std::mem::transmute(bits)
    }
}
///[VkCompositeAlphaFlagBitsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCompositeAlphaFlagBitsKHR.html) - Alpha compositing modes supported on a device
///# C Specifications
///The `supportedCompositeAlpha` member is of type
///[`CompositeAlphaFlagBitsKHR`], containing the following values:
///```c
///// Provided by VK_KHR_surface
///typedef enum VkCompositeAlphaFlagBitsKHR {
///    VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR = 0x00000001,
///    VK_COMPOSITE_ALPHA_PRE_MULTIPLIED_BIT_KHR = 0x00000002,
///    VK_COMPOSITE_ALPHA_POST_MULTIPLIED_BIT_KHR = 0x00000004,
///    VK_COMPOSITE_ALPHA_INHERIT_BIT_KHR = 0x00000008,
///} VkCompositeAlphaFlagBitsKHR;
///```
///# Description
///These values are described as follows:
/// - [`CompositeAlphaOpaqueKhr`]: The alpha component, if it exists, of the images is ignored in
///   the compositing process. Instead, the image is treated as if it has a constant alpha of 1.0.
/// - [`CompositeAlphaPreMultipliedKhr`]: The alpha component, if it exists, of the images is
///   respected in the compositing process. The non-alpha components of the image are expected to
///   already be multiplied by the alpha component by the application.
/// - [`CompositeAlphaPostMultipliedKhr`]: The alpha component, if it exists, of the images is
///   respected in the compositing process. The non-alpha components of the image are not expected
///   to already be multiplied by the alpha component by the application; instead, the compositor
///   will multiply the non-alpha components of the image by the alpha component during compositing.
/// - [`CompositeAlphaInheritKhr`]: The way in which the presentation engine treats the alpha
///   component in the images is unknown to the Vulkan API. Instead, the application is responsible
///   for setting the composite alpha blending mode using native window system commands. If the
///   application does not set the blending mode using native window system commands, then a
///   platform-specific default will be used.
///# Related
/// - [`VK_KHR_surface`]
/// - [`CompositeAlphaFlagsKHR`]
/// - [`SwapchainCreateInfoKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkCompositeAlphaFlagsKHR")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct CompositeAlphaFlagsKHR(u32);
impl const Default for CompositeAlphaFlagsKHR {
    fn default() -> Self {
        Self(0)
    }
}
impl From<CompositeAlphaFlagBitsKHR> for CompositeAlphaFlagsKHR {
    fn from(from: CompositeAlphaFlagBitsKHR) -> Self {
        unsafe { Self::from_bits_unchecked(from as u32) }
    }
}
impl CompositeAlphaFlagsKHR {
    ///[`CompositeAlphaOpaqueKhr`]: The alpha component, if it
    ///exists, of the images is ignored in the compositing process.
    ///Instead, the image is treated as if it has a constant alpha of 1.0.
    pub const COMPOSITE_ALPHA_OPAQUE_KHR: Self = Self(1);
    ///[`CompositeAlphaPreMultipliedKhr`]: The alpha component, if
    ///it exists, of the images is respected in the compositing process.
    ///The non-alpha components of the image are expected to already be
    ///multiplied by the alpha component by the application.
    pub const COMPOSITE_ALPHA_PRE_MULTIPLIED_KHR: Self = Self(2);
    ///[`CompositeAlphaPostMultipliedKhr`]: The alpha component,
    ///if it exists, of the images is respected in the compositing process.
    ///The non-alpha components of the image are not expected to already be
    ///multiplied by the alpha component by the application; instead, the
    ///compositor will multiply the non-alpha components of the image by the
    ///alpha component during compositing.
    pub const COMPOSITE_ALPHA_POST_MULTIPLIED_KHR: Self = Self(4);
    ///[`CompositeAlphaInheritKhr`]: The way in which the
    ///presentation engine treats the alpha component in the images is unknown
    ///to the Vulkan API.
    ///Instead, the application is responsible for setting the composite alpha
    ///blending mode using native window system commands.
    ///If the application does not set the blending mode using native window
    ///system commands, then a platform-specific default will be used.
    pub const COMPOSITE_ALPHA_INHERIT_KHR: Self = Self(8);
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Returns a value with all of the flags enabled
    #[inline]
    #[allow(unused_mut)]
    pub const fn all() -> Self {
        let mut all = Self::empty();
        {
            all |= Self::COMPOSITE_ALPHA_OPAQUE_KHR;
        }
        {
            all |= Self::COMPOSITE_ALPHA_PRE_MULTIPLIED_KHR;
        }
        {
            all |= Self::COMPOSITE_ALPHA_POST_MULTIPLIED_KHR;
        }
        {
            all |= Self::COMPOSITE_ALPHA_INHERIT_KHR;
        }
        all
    }
    ///Returns the raw bits
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0
    }
    ///Convert raw bits into a bit flags checking that only valid
    ///bits are contained.
    #[inline]
    pub const fn from_bits(bits: u32) -> Option<Self> {
        if (bits & !Self::all().bits()) == 0 {
            Some(Self(bits))
        } else {
            None
        }
    }
    ///Convert raw bits into a bit flags truncating all invalid
    ///bits that may be contained.
    #[inline]
    pub const fn from_bits_truncate(bits: u32) -> Self {
        Self(Self::all().0 & bits)
    }
    ///Convert raw bits into a bit preserving all bits
    ///
    ///# Safety
    ///The caller of this function must ensure that all of the bits are valid.
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
    ///Returns `true` if no flags are currently set
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.bits() == Self::empty().bits()
    }
    ///Returns `true` if all flags are currently set
    #[inline]
    pub const fn is_all(&self) -> bool {
        self.bits() == Self::all().bits()
    }
    ///Returns `true` if there are flags in common to `self` and `other`
    #[inline]
    pub const fn intersects(&self, other: Self) -> bool {
        !Self(self.bits() & other.bits()).is_empty()
    }
    ///Returns `true` if all of the flags in `other` are contained `self`
    #[inline]
    pub const fn contains(&self, other: Self) -> bool {
        (self.bits() & other.bits()) == other.bits()
    }
    ///Inserts a set of flags in place
    #[inline]
    pub fn insert(&mut self, other: Self) {
        self.0 |= other.bits()
    }
    ///Removes a set of flags in place
    #[inline]
    pub fn remove(&mut self, other: Self) {
        self.0 &= !other.bits();
    }
    ///Toggles a set of flags in place
    #[inline]
    pub fn toggle(&mut self, other: Self) {
        self.0 ^= other.bits();
    }
    ///Inserts or removes the specified flags depending on the value of `is_insert`
    #[inline]
    pub fn set(&mut self, other: Self, is_insert: bool) {
        if is_insert {
            self.insert(other);
        } else {
            self.remove(other);
        }
    }
    ///Returns the intersection between `self` and `other`
    #[inline]
    pub const fn intersection(self, other: Self) -> Self {
        Self(self.bits() & other.bits())
    }
    ///Returns the union between `self` and `other`
    #[inline]
    pub const fn union(self, other: Self) -> Self {
        Self(self.bits() | other.bits())
    }
    ///Returns the difference between `self` and `other`
    #[inline]
    pub const fn difference(self, other: Self) -> Self {
        Self(self.bits() & !other.bits())
    }
    ///Returns the [symmetric difference][sym-diff] between `self` and `other`
    ///
    ///[sym-diff]: https://en.wikipedia.org/wiki/Symmetric_difference
    #[inline]
    pub const fn symmetric_difference(self, other: Self) -> Self {
        Self(self.bits() ^ other.bits())
    }
    ///Returns the complement of `self`.
    #[inline]
    pub const fn complement(self) -> Self {
        Self::from_bits_truncate(!self.bits())
    }
}
impl const std::ops::BitOr for CompositeAlphaFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl const std::ops::BitOrAssign for CompositeAlphaFlagsKHR {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for CompositeAlphaFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl const std::ops::BitXorAssign for CompositeAlphaFlagsKHR {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for CompositeAlphaFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl const std::ops::BitAndAssign for CompositeAlphaFlagsKHR {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for CompositeAlphaFlagsKHR {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl const std::ops::SubAssign for CompositeAlphaFlagsKHR {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for CompositeAlphaFlagsKHR {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<CompositeAlphaFlagsKHR> for CompositeAlphaFlagsKHR {
    fn extend<T: IntoIterator<Item = CompositeAlphaFlagsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl Extend<CompositeAlphaFlagBitsKHR> for CompositeAlphaFlagsKHR {
    fn extend<T: IntoIterator<Item = CompositeAlphaFlagBitsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, <Self as From<CompositeAlphaFlagBitsKHR>>::from(i));
        }
    }
}
impl FromIterator<CompositeAlphaFlagsKHR> for CompositeAlphaFlagsKHR {
    fn from_iter<T: IntoIterator<Item = CompositeAlphaFlagsKHR>>(iterator: T) -> CompositeAlphaFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<CompositeAlphaFlagsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl FromIterator<CompositeAlphaFlagBitsKHR> for CompositeAlphaFlagsKHR {
    fn from_iter<T: IntoIterator<Item = CompositeAlphaFlagBitsKHR>>(iterator: T) -> CompositeAlphaFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<CompositeAlphaFlagBitsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for CompositeAlphaFlagsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(CompositeAlphaFlagsKHR);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == CompositeAlphaFlagsKHR::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(CompositeAlphaFlagsKHR::COMPOSITE_ALPHA_OPAQUE_KHR) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(COMPOSITE_ALPHA_OPAQUE_KHR))?;
                    }
                    if self
                        .0
                        .contains(CompositeAlphaFlagsKHR::COMPOSITE_ALPHA_PRE_MULTIPLIED_KHR)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(COMPOSITE_ALPHA_PRE_MULTIPLIED_KHR))?;
                    }
                    if self
                        .0
                        .contains(CompositeAlphaFlagsKHR::COMPOSITE_ALPHA_POST_MULTIPLIED_KHR)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(COMPOSITE_ALPHA_POST_MULTIPLIED_KHR))?;
                    }
                    if self.0.contains(CompositeAlphaFlagsKHR::COMPOSITE_ALPHA_INHERIT_KHR) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(COMPOSITE_ALPHA_INHERIT_KHR))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(CompositeAlphaFlagsKHR))
            .field(&Flags(*self))
            .finish()
    }
}
///[VkSurfaceCapabilitiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSurfaceCapabilitiesKHR.html) - Structure describing capabilities of a surface
///# C Specifications
///The [`SurfaceCapabilitiesKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_surface
///typedef struct VkSurfaceCapabilitiesKHR {
///    uint32_t                         minImageCount;
///    uint32_t                         maxImageCount;
///    VkExtent2D                       currentExtent;
///    VkExtent2D                       minImageExtent;
///    VkExtent2D                       maxImageExtent;
///    uint32_t                         maxImageArrayLayers;
///    VkSurfaceTransformFlagsKHR       supportedTransforms;
///    VkSurfaceTransformFlagBitsKHR    currentTransform;
///    VkCompositeAlphaFlagsKHR         supportedCompositeAlpha;
///    VkImageUsageFlags                supportedUsageFlags;
///} VkSurfaceCapabilitiesKHR;
///```
///# Description
/// - [`min_image_count`] is the minimum number of images the specified device supports for a
///   swapchain created for the surface, and will be at least one.
/// - [`max_image_count`] is the maximum number of images the specified device supports for a
///   swapchain created for the surface, and will be either 0, or greater than or equal to
///   [`min_image_count`]. A value of 0 means that there is no limit on the number of images, though
///   there  **may**  be limits related to the total amount of memory used by presentable images.
/// - [`current_extent`] is the current width and height of the surface, or the special value
///   (0xFFFFFFFF, 0xFFFFFFFF) indicating that the surface size will be determined by the extent of
///   a swapchain targeting the surface.
/// - [`min_image_extent`] contains the smallest valid swapchain extent for the surface on the
///   specified device. The `width` and `height` of the extent will each be less than or equal to
///   the corresponding `width` and `height` of [`current_extent`], unless [`current_extent`] has
///   the special value described above.
/// - [`max_image_extent`] contains the largest valid swapchain extent for the surface on the
///   specified device. The `width` and `height` of the extent will each be greater than or equal to
///   the corresponding `width` and `height` of [`min_image_extent`]. The `width` and `height` of
///   the extent will each be greater than or equal to the corresponding `width` and `height` of
///   [`current_extent`], unless [`current_extent`] has the special value described above.
/// - [`max_image_array_layers`] is the maximum number of layers presentable images  **can**  have
///   for a swapchain created for this device and surface, and will be at least one.
/// - [`supported_transforms`] is a bitmask of [`SurfaceTransformFlagBitsKHR`] indicating the
///   presentation transforms supported for the surface on the specified device. At least one bit
///   will be set.
/// - [`current_transform`] is [`SurfaceTransformFlagBitsKHR`] value indicating the surface’s
///   current transform relative to the presentation engine’s natural orientation.
/// - [`supported_composite_alpha`] is a bitmask of [`CompositeAlphaFlagBitsKHR`], representing the
///   alpha compositing modes supported by the presentation engine for the surface on the specified
///   device, and at least one bit will be set. Opaque composition  **can**  be achieved in any
///   alpha compositing mode by either using an image format that has no alpha component, or by
///   ensuring that all pixels in the presentable images have an alpha value of 1.0.
/// - [`supported_usage_flags`] is a bitmask of [`ImageUsageFlagBits`] representing the ways the
///   application  **can**  use the presentable images of a swapchain created with
///   [`PresentModeKHR`] set to `VK_PRESENT_MODE_IMMEDIATE_KHR`, `VK_PRESENT_MODE_MAILBOX_KHR`,
///   `VK_PRESENT_MODE_FIFO_KHR` or `VK_PRESENT_MODE_FIFO_RELAXED_KHR` for the surface on the
///   specified device. `VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT` **must**  be included in the set.
///   Implementations  **may**  support additional usages.
///# Related
/// - [`VK_KHR_surface`]
/// - [`CompositeAlphaFlagsKHR`]
/// - [`Extent2D`]
/// - [`ImageUsageFlags`]
/// - [`SurfaceCapabilities2KHR`]
/// - [`SurfaceTransformFlagBitsKHR`]
/// - [`SurfaceTransformFlagsKHR`]
/// - [`GetPhysicalDeviceSurfaceCapabilitiesKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkSurfaceCapabilitiesKHR")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct SurfaceCapabilitiesKHR {
    ///[`min_image_count`] is the minimum number of images the specified device
    ///supports for a swapchain created for the surface, and will be at least
    ///one.
    pub min_image_count: u32,
    ///[`max_image_count`] is the maximum number of images the specified device
    ///supports for a swapchain created for the surface, and will be either 0,
    ///or greater than or equal to [`min_image_count`].
    ///A value of 0 means that there is no limit on the number of images,
    ///though there  **may**  be limits related to the total amount of memory used
    ///by presentable images.
    pub max_image_count: u32,
    ///[`current_extent`] is the current width and height of the surface, or
    ///the special value (0xFFFFFFFF, 0xFFFFFFFF) indicating that the
    ///surface size will be determined by the extent of a swapchain targeting
    ///the surface.
    pub current_extent: Extent2D,
    ///[`min_image_extent`] contains the smallest valid swapchain extent for
    ///the surface on the specified device.
    ///The `width` and `height` of the extent will each be less than or
    ///equal to the corresponding `width` and `height` of
    ///[`current_extent`], unless [`current_extent`] has the special value
    ///described above.
    pub min_image_extent: Extent2D,
    ///[`max_image_extent`] contains the largest valid swapchain extent for the
    ///surface on the specified device.
    ///The `width` and `height` of the extent will each be greater than
    ///or equal to the corresponding `width` and `height` of
    ///[`min_image_extent`].
    ///The `width` and `height` of the extent will each be greater than
    ///or equal to the corresponding `width` and `height` of
    ///[`current_extent`], unless [`current_extent`] has the special value
    ///described above.
    pub max_image_extent: Extent2D,
    ///[`max_image_array_layers`] is the maximum number of layers presentable
    ///images  **can**  have for a swapchain created for this device and surface,
    ///and will be at least one.
    pub max_image_array_layers: u32,
    ///[`supported_transforms`] is a bitmask of
    ///[`SurfaceTransformFlagBitsKHR`] indicating the presentation
    ///transforms supported for the surface on the specified device.
    ///At least one bit will be set.
    pub supported_transforms: SurfaceTransformFlagsKHR,
    ///[`current_transform`] is [`SurfaceTransformFlagBitsKHR`] value
    ///indicating the surface’s current transform relative to the presentation
    ///engine’s natural orientation.
    pub current_transform: SurfaceTransformFlagBitsKHR,
    ///[`supported_composite_alpha`] is a bitmask of
    ///[`CompositeAlphaFlagBitsKHR`], representing the alpha compositing
    ///modes supported by the presentation engine for the surface on the
    ///specified device, and at least one bit will be set.
    ///Opaque composition  **can**  be achieved in any alpha compositing mode by
    ///either using an image format that has no alpha component, or by ensuring
    ///that all pixels in the presentable images have an alpha value of 1.0.
    pub supported_composite_alpha: CompositeAlphaFlagsKHR,
    ///[`supported_usage_flags`] is a bitmask of [`ImageUsageFlagBits`]
    ///representing the ways the application  **can**  use the presentable images of
    ///a swapchain created
    ///with [`PresentModeKHR`] set to `VK_PRESENT_MODE_IMMEDIATE_KHR`,
    ///`VK_PRESENT_MODE_MAILBOX_KHR`, `VK_PRESENT_MODE_FIFO_KHR` or
    ///`VK_PRESENT_MODE_FIFO_RELAXED_KHR`
    ///for the surface on the specified device.
    ///`VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT` **must**  be included in the set.
    ///Implementations  **may**  support additional usages.
    pub supported_usage_flags: ImageUsageFlags,
}
impl Default for SurfaceCapabilitiesKHR {
    fn default() -> Self {
        Self {
            min_image_count: 0,
            max_image_count: 0,
            current_extent: Default::default(),
            min_image_extent: Default::default(),
            max_image_extent: Default::default(),
            max_image_array_layers: 0,
            supported_transforms: Default::default(),
            current_transform: Default::default(),
            supported_composite_alpha: Default::default(),
            supported_usage_flags: Default::default(),
        }
    }
}
impl SurfaceCapabilitiesKHR {
    ///Gets the value of [`Self::min_image_count`]
    pub fn min_image_count(&self) -> u32 {
        self.min_image_count
    }
    ///Gets the value of [`Self::max_image_count`]
    pub fn max_image_count(&self) -> u32 {
        self.max_image_count
    }
    ///Gets the value of [`Self::current_extent`]
    pub fn current_extent(&self) -> Extent2D {
        self.current_extent
    }
    ///Gets the value of [`Self::min_image_extent`]
    pub fn min_image_extent(&self) -> Extent2D {
        self.min_image_extent
    }
    ///Gets the value of [`Self::max_image_extent`]
    pub fn max_image_extent(&self) -> Extent2D {
        self.max_image_extent
    }
    ///Gets the value of [`Self::max_image_array_layers`]
    pub fn max_image_array_layers(&self) -> u32 {
        self.max_image_array_layers
    }
    ///Gets the value of [`Self::supported_transforms`]
    pub fn supported_transforms(&self) -> SurfaceTransformFlagsKHR {
        self.supported_transforms
    }
    ///Gets the value of [`Self::current_transform`]
    pub fn current_transform(&self) -> SurfaceTransformFlagBitsKHR {
        self.current_transform
    }
    ///Gets the value of [`Self::supported_composite_alpha`]
    pub fn supported_composite_alpha(&self) -> CompositeAlphaFlagsKHR {
        self.supported_composite_alpha
    }
    ///Gets the value of [`Self::supported_usage_flags`]
    pub fn supported_usage_flags(&self) -> ImageUsageFlags {
        self.supported_usage_flags
    }
    ///Gets a mutable reference to the value of [`Self::min_image_count`]
    pub fn min_image_count_mut(&mut self) -> &mut u32 {
        &mut self.min_image_count
    }
    ///Gets a mutable reference to the value of [`Self::max_image_count`]
    pub fn max_image_count_mut(&mut self) -> &mut u32 {
        &mut self.max_image_count
    }
    ///Gets a mutable reference to the value of [`Self::current_extent`]
    pub fn current_extent_mut(&mut self) -> &mut Extent2D {
        &mut self.current_extent
    }
    ///Gets a mutable reference to the value of [`Self::min_image_extent`]
    pub fn min_image_extent_mut(&mut self) -> &mut Extent2D {
        &mut self.min_image_extent
    }
    ///Gets a mutable reference to the value of [`Self::max_image_extent`]
    pub fn max_image_extent_mut(&mut self) -> &mut Extent2D {
        &mut self.max_image_extent
    }
    ///Gets a mutable reference to the value of [`Self::max_image_array_layers`]
    pub fn max_image_array_layers_mut(&mut self) -> &mut u32 {
        &mut self.max_image_array_layers
    }
    ///Gets a mutable reference to the value of [`Self::supported_transforms`]
    pub fn supported_transforms_mut(&mut self) -> &mut SurfaceTransformFlagsKHR {
        &mut self.supported_transforms
    }
    ///Gets a mutable reference to the value of [`Self::current_transform`]
    pub fn current_transform_mut(&mut self) -> &mut SurfaceTransformFlagBitsKHR {
        &mut self.current_transform
    }
    ///Gets a mutable reference to the value of [`Self::supported_composite_alpha`]
    pub fn supported_composite_alpha_mut(&mut self) -> &mut CompositeAlphaFlagsKHR {
        &mut self.supported_composite_alpha
    }
    ///Gets a mutable reference to the value of [`Self::supported_usage_flags`]
    pub fn supported_usage_flags_mut(&mut self) -> &mut ImageUsageFlags {
        &mut self.supported_usage_flags
    }
    ///Sets the raw value of [`Self::min_image_count`]
    pub fn set_min_image_count(&mut self, value: u32) -> &mut Self {
        self.min_image_count = value;
        self
    }
    ///Sets the raw value of [`Self::max_image_count`]
    pub fn set_max_image_count(&mut self, value: u32) -> &mut Self {
        self.max_image_count = value;
        self
    }
    ///Sets the raw value of [`Self::current_extent`]
    pub fn set_current_extent(&mut self, value: crate::vulkan1_0::Extent2D) -> &mut Self {
        self.current_extent = value;
        self
    }
    ///Sets the raw value of [`Self::min_image_extent`]
    pub fn set_min_image_extent(&mut self, value: crate::vulkan1_0::Extent2D) -> &mut Self {
        self.min_image_extent = value;
        self
    }
    ///Sets the raw value of [`Self::max_image_extent`]
    pub fn set_max_image_extent(&mut self, value: crate::vulkan1_0::Extent2D) -> &mut Self {
        self.max_image_extent = value;
        self
    }
    ///Sets the raw value of [`Self::max_image_array_layers`]
    pub fn set_max_image_array_layers(&mut self, value: u32) -> &mut Self {
        self.max_image_array_layers = value;
        self
    }
    ///Sets the raw value of [`Self::supported_transforms`]
    pub fn set_supported_transforms(
        &mut self,
        value: crate::extensions::khr_display::SurfaceTransformFlagsKHR,
    ) -> &mut Self {
        self.supported_transforms = value;
        self
    }
    ///Sets the raw value of [`Self::current_transform`]
    pub fn set_current_transform(
        &mut self,
        value: crate::extensions::khr_surface::SurfaceTransformFlagBitsKHR,
    ) -> &mut Self {
        self.current_transform = value;
        self
    }
    ///Sets the raw value of [`Self::supported_composite_alpha`]
    pub fn set_supported_composite_alpha(
        &mut self,
        value: crate::extensions::khr_surface::CompositeAlphaFlagsKHR,
    ) -> &mut Self {
        self.supported_composite_alpha = value;
        self
    }
    ///Sets the raw value of [`Self::supported_usage_flags`]
    pub fn set_supported_usage_flags(&mut self, value: crate::vulkan1_0::ImageUsageFlags) -> &mut Self {
        self.supported_usage_flags = value;
        self
    }
}
///[VkSurfaceFormatKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSurfaceFormatKHR.html) - Structure describing a supported swapchain format-color space pair
///# C Specifications
///The [`SurfaceFormatKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_surface
///typedef struct VkSurfaceFormatKHR {
///    VkFormat           format;
///    VkColorSpaceKHR    colorSpace;
///} VkSurfaceFormatKHR;
///```
///# Members
/// - [`format`] is a [`Format`] that is compatible with the specified surface.
/// - [`color_space`] is a presentation [`ColorSpaceKHR`] that is compatible with the surface.
///# Related
/// - [`VK_KHR_surface`]
/// - [`ColorSpaceKHR`]
/// - [`Format`]
/// - [`SurfaceFormat2KHR`]
/// - [`GetPhysicalDeviceSurfaceFormatsKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkSurfaceFormatKHR")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct SurfaceFormatKHR {
    ///[`format`] is a [`Format`] that is compatible with the specified
    ///surface.
    pub format: Format,
    ///[`color_space`] is a presentation [`ColorSpaceKHR`] that is
    ///compatible with the surface.
    pub color_space: ColorSpaceKHR,
}
impl Default for SurfaceFormatKHR {
    fn default() -> Self {
        Self {
            format: Default::default(),
            color_space: Default::default(),
        }
    }
}
impl SurfaceFormatKHR {
    ///Gets the value of [`Self::format`]
    pub fn format(&self) -> Format {
        self.format
    }
    ///Gets the value of [`Self::color_space`]
    pub fn color_space(&self) -> ColorSpaceKHR {
        self.color_space
    }
    ///Gets a mutable reference to the value of [`Self::format`]
    pub fn format_mut(&mut self) -> &mut Format {
        &mut self.format
    }
    ///Gets a mutable reference to the value of [`Self::color_space`]
    pub fn color_space_mut(&mut self) -> &mut ColorSpaceKHR {
        &mut self.color_space
    }
    ///Sets the raw value of [`Self::format`]
    pub fn set_format(&mut self, value: crate::vulkan1_0::Format) -> &mut Self {
        self.format = value;
        self
    }
    ///Sets the raw value of [`Self::color_space`]
    pub fn set_color_space(&mut self, value: crate::extensions::khr_surface::ColorSpaceKHR) -> &mut Self {
        self.color_space = value;
        self
    }
}
///[VkSurfaceKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSurfaceKHR.html) - Opaque handle to a surface object
///# C Specifications
///Native platform surface or window objects are abstracted by surface objects,
///which are represented by [`SurfaceKHR`] handles:
///```c
///// Provided by VK_KHR_surface
///VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkSurfaceKHR)
///```
///# Related
/// - [`VK_KHR_surface`]
/// - [`PhysicalDeviceSurfaceInfo2KHR`]
/// - [`SwapchainCreateInfoKHR`]
/// - [`CreateAndroidSurfaceKHR`]
/// - [`CreateDirectFBSurfaceEXT`]
/// - [`CreateDisplayPlaneSurfaceKHR`]
/// - [`CreateHeadlessSurfaceEXT`]
/// - [`CreateIosSurfaceMVK`]
/// - [`CreateImagePipeSurfaceFUCHSIA`]
/// - [`CreateMacOsSurfaceMVK`]
/// - [`CreateMetalSurfaceEXT`]
/// - [`CreateScreenSurfaceQNX`]
/// - [`CreateStreamDescriptorSurfaceGGP`]
/// - [`CreateViSurfaceNN`]
/// - [`CreateWaylandSurfaceKHR`]
/// - [`CreateWin32SurfaceKHR`]
/// - [`CreateXcbSurfaceKHR`]
/// - [`CreateXlibSurfaceKHR`]
/// - [`DestroySurfaceKHR`]
/// - [`GetDeviceGroupSurfacePresentModesKHR`]
/// - [`GetPhysicalDevicePresentRectanglesKHR`]
/// - [`GetPhysicalDeviceSurfaceCapabilities2EXT`]
/// - [`GetPhysicalDeviceSurfaceCapabilitiesKHR`]
/// - [`GetPhysicalDeviceSurfaceFormatsKHR`]
/// - [`GetPhysicalDeviceSurfacePresentModesKHR`]
/// - [`GetPhysicalDeviceSurfaceSupportKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkSurfaceKHR")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(transparent)]
pub struct SurfaceKHR(pub u64);
impl SurfaceKHR {
    ///Creates a new null handle
    #[inline]
    pub const fn null() -> Self {
        Self(0)
    }
    ///Checks if this is a null handle
    #[inline]
    pub fn is_null(&self) -> bool {
        self == &Self::null()
    }
    ///Gets the raw value
    #[inline]
    pub fn raw(&self) -> u64 {
        self.0
    }
}
unsafe impl Send for SurfaceKHR {}
impl Default for SurfaceKHR {
    fn default() -> Self {
        Self::null()
    }
}
