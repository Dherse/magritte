//![VK_KHR_display](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_display.html) - instance extension
//!# Description
//!This extension provides the API to enumerate displays and available modes on
//!a given device.
//!# Revision
//!23
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_surface`]`
//!# Contacts
//! - James Jones [cubanismo](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_display]
//!   @cubanismo%0A<<Here describe the issue or question you have about the VK_KHR_display
//!   extension>>)
//! - Norbert Nopper [FslNopper](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_display]
//!   @FslNopper%0A<<Here describe the issue or question you have about the VK_KHR_display
//!   extension>>)
//!# New handles
//! - [`DisplayKHR`]
//! - [`DisplayModeKHR`]
//!# New functions & commands
//! - [`create_display_mode_khr`]
//! - [`create_display_plane_surface_khr`]
//! - [`get_display_mode_properties_khr`]
//! - [`get_display_plane_capabilities_khr`]
//! - [`get_display_plane_supported_displays_khr`]
//! - [`get_physical_device_display_plane_properties_khr`]
//! - [`get_physical_device_display_properties_khr`]
//!# New structures
//! - [`DisplayModeCreateInfoKHR`]
//! - [`DisplayModeParametersKHR`]
//! - [`DisplayModePropertiesKHR`]
//! - [`DisplayPlaneCapabilitiesKHR`]
//! - [`DisplayPlanePropertiesKHR`]
//! - [`DisplayPropertiesKHR`]
//! - [`DisplaySurfaceCreateInfoKHR`]
//!# New enums
//! - [`DisplayPlaneAlphaFlagBitsKHR`]
//!# New bitmasks
//! - [`DisplayModeCreateFlagsKHR`]
//! - [`DisplayPlaneAlphaFlagsKHR`]
//! - [`DisplaySurfaceCreateFlagsKHR`]
//! - [`SurfaceTransformFlagsKHR`]
//!# New constants
//! - [`KHR_DISPLAY_EXTENSION_NAME`]
//! - [`KHR_DISPLAY_SPEC_VERSION`]
//! - Extending [`ObjectType`]:  - `VK_OBJECT_TYPE_DISPLAY_KHR`  - `VK_OBJECT_TYPE_DISPLAY_MODE_KHR`
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_DISPLAY_MODE_CREATE_INFO_KHR`  -
//!   `VK_STRUCTURE_TYPE_DISPLAY_SURFACE_CREATE_INFO_KHR`
//!# Known issues & F.A.Q
//!1) Which properties of a mode should be fixed in the mode information vs.
//!settable in some other function when setting the mode? E.g., do we need to
//!double the size of the mode pool to include both stereo and non-stereo
//!modes? YUV and RGB scanout even if they both take RGB input images? BGR vs.
//!RGB input? etc. **PROPOSED RESOLUTION** : Many modern displays support at most a handful of
//!resolutions and timings natively.
//!Other “modes” are expected to be supported using scaling hardware on the
//!display engine or GPU.
//!Other properties, such as rotation and mirroring should not require
//!duplicating hardware modes just to express all combinations.
//!Further, these properties may be implemented on a per-display or per-overlay
//!granularity.To avoid the exponential growth of modes as mutable properties are added, as
//!was the case with `EGLConfig`/WGL pixel formats/`GLXFBConfig`, this
//!specification should separate out hardware properties and configurable state
//!into separate objects.
//!Modes and overlay planes will express capabilities of the hardware, while a
//!separate structure will allow applications to configure scaling, rotation,
//!mirroring, color keys, LUT values, alpha masks, etc.
//!for a given swapchain independent of the mode in use.
//!Constraints on these settings will be established by properties of the
//!immutable objects.Note the resolution of this issue may affect issue 5 as well.2) What
//! properties of a display itself are useful? **PROPOSED RESOLUTION** : This issue is too broad.
//!It was meant to prompt general discussion, but resolving this issue amounts
//!to completing this specification.
//!All interesting properties should be included.
//!The issue will remain as a placeholder since removing it would make it hard
//!to parse existing discussion notes that refer to issues by number.3) How are multiple overlay
//! planes within a display or mode enumerated? **PROPOSED RESOLUTION** : They are referred to by an
//! index.
//!Each display will report the number of overlay planes it contains.4) Should swapchains be
//! created relative to a mode or a display? **PROPOSED RESOLUTION** : When using this extension,
//! swapchains are created
//!relative to a mode and a plane.
//!The mode implies the display object the swapchain will present to.
//!If the specified mode is not the display’s current mode, the new mode will
//!be applied when the first image is presented to the swapchain, and the
//!default operating system mode, if any, will be restored when the swapchain
//!is destroyed.5) Should users query generic ranges from displays and construct their own
//!modes explicitly using those constraints rather than querying a fixed set of
//!modes (Most monitors only have one real “mode” these days, even though
//!many support relatively arbitrary scaling, either on the monitor side or in
//!the GPU display engine, making “modes” something of a relic/compatibility
//!construct). **PROPOSED RESOLUTION** : Expose both.
//!Display information structures will expose a set of predefined modes, as
//!well as any attributes necessary to construct a customized mode.6) Is it fine if we return the
//! display and display mode handles in the
//!structure used to query their properties? **PROPOSED RESOLUTION** : Yes.7) Is there a
//! possibility that not all displays of a device work with all of
//!the present queues of a device? If yes, how do we determine which displays
//!work with which present queues? **PROPOSED RESOLUTION** : No known hardware has such
//! limitations, but
//!determining such limitations is supported automatically using the existing
//!`[`VK_KHR_surface`]` and `[`VK_KHR_swapchain`]` query mechanisms.8) Should all presentation need
//! to be done relative to an overlay plane, or
//!can a display mode + display be used alone to target an output? **PROPOSED RESOLUTION** :
//! Require specifying a plane explicitly.9) Should displays have an associated window system
//! display, such as an
//!`HDC` or `Display*`? **PROPOSED RESOLUTION** : No.
//!Displays are independent of any windowing system in use on the system.
//!Further, neither `HDC` nor `Display*` refer to a physical display
//!object.10) Are displays queried from a physical GPU or from a device instance? **PROPOSED
//! RESOLUTION** : Developers prefer to query modes directly from the
//!physical GPU so they can use display information as an input to their device
//!selection algorithms prior to device creation.
//!This avoids the need to create placeholder device instances to enumerate
//!displays.This preference must be weighed against the extra initialization that must
//!be done by driver vendors prior to device instance creation to support this
//!usage.11) Should displays and/or modes be dispatchable objects? If functions are
//!to take displays, overlays, or modes as their first parameter, they must be
//!dispatchable objects as defined in Khronos bug 13529.
//!If they are not added to the list of dispatchable objects, functions
//!operating on them must take some higher-level object as their first
//!parameter.
//!There is no performance case against making them dispatchable objects, but
//!they would be the first extension objects to be dispatchable. **PROPOSED RESOLUTION** : Do not
//! make displays or modes dispatchable.
//!They will dispatch based on their associated physical device.12) Should hardware cursor
//! capabilities be exposed? **PROPOSED RESOLUTION** : Defer.
//!This could be a separate extension on top of the base WSI specs.if they are one physical display
//! device to an end user, but may internally
//!be implemented as two side-by-side displays using the same display engine
//!(and sometimes cabling) resources as two physically separate display
//!devices. **RESOLVED** : Tiled displays will appear as a single display object in this
//!API.14) Should the raw EDID data be included in the display information? **RESOLVED** : No.
//!A future extension could be added which reports the EDID if necessary.
//!This may be complicated by the outcome of issue 13.15) Should min and max scaling factor
//! capabilities of overlays be exposed? **RESOLVED** : Yes.
//!This is exposed indirectly by allowing applications to query the min/max
//!position and extent of the source and destination regions from which image
//!contents are fetched by the display engine when using a particular mode and
//!overlay pair.16) Should devices be able to expose planes that can be moved between
//!displays? If so, how? **RESOLVED** : Yes.
//!Applications can determine which displays a given plane supports using
//![`get_display_plane_supported_displays_khr`].17) Should there be a way to destroy display modes?
//! If so, does it support
//!destroying “built in” modes? **RESOLVED** : Not in this extension.
//!A future extension could add this functionality.18) What should the lifetime of display and
//! built-in display mode objects
//!be? **RESOLVED** : The lifetime of the instance.
//!These objects cannot be destroyed.
//!A future extension may be added to expose a way to destroy these objects
//!and/or support display hotplug.19) Should persistent mode for smart panels be enabled/disabled
//! at swapchain
//!creation time, or on a per-present basis. **RESOLVED** : On a per-present basis.
//!# Version History
//! - Revision 1, 2015-02-24 (James Jones)  - Initial draft
//! - Revision 2, 2015-03-12 (Norbert Nopper)  - Added overlay enumeration for a display.
//! - Revision 3, 2015-03-17 (Norbert Nopper)  - Fixed typos and namings as discussed in Bugzilla.
//!   - Reordered and grouped functions.  - Added functions to query count of display, mode and
//!   overlay.  - Added native display handle, which may be needed on some platforms to create a
//!   native Window.
//! - Revision 4, 2015-03-18 (Norbert Nopper)  - Removed primary and virtualPostion members (see
//!   comment of James Jones in Bugzilla).  - Added native overlay handle to information structure.
//!   - Replaced , with ; in struct.
//! - Revision 6, 2015-03-18 (Daniel Rakos)  - Added WSI extension suffix to all items.  - Made the
//!   whole API more “Vulkanish”.  - Replaced all functions with a single vkGetDisplayInfoKHR
//!   function to better match the rest of the API.  - Made the display, display mode, and overlay
//!   objects be first class objects, not subclasses of VkBaseObject as they do not support the
//!   common functions anyways.  - Renamed *Info structures to *Properties.  - Removed overlayIndex
//!   field from VkOverlayProperties as there is an implicit index already as a result of moving to
//!   a “Vulkanish” API.  - Displays are not get through device, but through physical GPU to match
//!   the rest of the Vulkan API. Also this is something ISVs explicitly requested.  - Added issue
//!   (6) and (7).
//! - Revision 7, 2015-03-25 (James Jones)  - Added an issues section  - Added rotation and
//!   mirroring flags
//! - Revision 8, 2015-03-25 (James Jones)  - Combined the duplicate issues sections introduced in
//!   last change.  - Added proposed resolutions to several issues.
//! - Revision 9, 2015-04-01 (Daniel Rakos)  - Rebased extension against Vulkan 0.82.0
//! - Revision 10, 2015-04-01 (James Jones)  - Added issues (10) and (11).  - Added more straw-man
//!   issue resolutions, and cleaned up the proposed resolution for issue (4).  - Updated the
//!   rotation and mirroring enums to have proper bitmask semantics.
//! - Revision 11, 2015-04-15 (James Jones)  - Added proposed resolution for issues (1) and (2).  -
//!   Added issues (12), (13), (14), and (15)  - Removed pNativeHandle field from overlay structure.
//!   - Fixed small compilation errors in example code.
//! - Revision 12, 2015-07-29 (James Jones)  - Rewrote the guts of the extension against the latest
//!   WSI swapchain specifications and the latest Vulkan API.  - Address overlay planes by their
//!   index rather than an object handle and refer to them as “planes” rather than “overlays” to
//!   make it slightly clearer that even a display with no “overlays” still has at least one base
//!   “plane” that images can be displayed on.  - Updated most of the issues.  - Added an “extension
//!   type” section to the specification header.  - Re-used the VK_EXT_KHR_surface surface transform
//!   enumerations rather than redefining them here.  - Updated the example code to use the new
//!   semantics.
//! - Revision 13, 2015-08-21 (Ian Elliott)  - Renamed this extension and all of its enumerations,
//!   types, functions, etc. This makes it compliant with the proposed standard for Vulkan
//!   extensions.  - Switched from “revision” to “version”, including use of the VK_MAKE_VERSION
//!   macro in the header file.
//! - Revision 14, 2015-09-01 (James Jones)  - Restore single-field revision number.
//! - Revision 15, 2015-09-08 (James Jones)  - Added alpha flags enum.  - Added premultiplied alpha
//!   support.
//! - Revision 16, 2015-09-08 (James Jones)  - Added description section to the spec.  - Added
//!   issues 16 - 18.
//! - Revision 17, 2015-10-02 (James Jones)  - Planes are now a property of the entire device rather
//!   than individual displays. This allows planes to be moved between multiple displays on devices
//!   that support it.  - Added a function to create a VkSurfaceKHR object describing a display
//!   plane and mode to align with the new per-platform surface creation conventions.  - Removed
//!   detailed mode timing data. It was agreed that the mode extents and refresh rate are sufficient
//!   for current use cases. Other information could be added back in as an extension if it is
//!   needed in the future.  - Added support for smart/persistent/buffered display devices.
//! - Revision 18, 2015-10-26 (Ian Elliott)  - Renamed from VK_EXT_KHR_display to VK_KHR_display.
//! - Revision 19, 2015-11-02 (James Jones)  - Updated example code to match revision 17 changes.
//! - Revision 20, 2015-11-03 (Daniel Rakos)  - Added allocation callbacks to creation functions.
//! - Revision 21, 2015-11-10 (Jesse Hall)  - Added VK_DISPLAY_PLANE_ALPHA_OPAQUE_BIT_KHR, and use
//!   VkDisplayPlaneAlphaFlagBitsKHR for VkDisplayPlanePropertiesKHR::alphaMode instead of
//!   VkDisplayPlaneAlphaFlagsKHR, since it only represents one mode.  - Added reserved flags
//!   bitmask to VkDisplayPlanePropertiesKHR.  - Use VkSurfaceTransformFlagBitsKHR instead of
//!   obsolete VkSurfaceTransformKHR.  - Renamed vkGetDisplayPlaneSupportedDisplaysKHR parameters
//!   for clarity.
//! - Revision 22, 2015-12-18 (James Jones)  - Added missing “planeIndex” parameter to
//!   vkGetDisplayPlaneSupportedDisplaysKHR()
//! - Revision 23, 2017-03-13 (James Jones)  - Closed all remaining issues. The specification and
//!   implementations have been shipping with the proposed resolutions for some time now.  - Removed
//!   the sample code and noted it has been integrated into the official Vulkan SDK cube demo.
//!# Other info
//! * 2017-03-13
//! * No known IP claims.
//! * - James Jones, NVIDIA  - Norbert Nopper, Freescale  - Jeff Vigil, Qualcomm  - Daniel Rakos,
//!   AMD
//!# Related
//! - [`DisplayKHR`]
//! - [`DisplayModeCreateFlagsKHR`]
//! - [`DisplayModeCreateInfoKHR`]
//! - [`DisplayModeKHR`]
//! - [`DisplayModeParametersKHR`]
//! - [`DisplayModePropertiesKHR`]
//! - [`DisplayPlaneAlphaFlagBitsKHR`]
//! - [`DisplayPlaneAlphaFlagsKHR`]
//! - [`DisplayPlaneCapabilitiesKHR`]
//! - [`DisplayPlanePropertiesKHR`]
//! - [`DisplayPropertiesKHR`]
//! - [`DisplaySurfaceCreateFlagsKHR`]
//! - [`DisplaySurfaceCreateInfoKHR`]
//! - [`SurfaceTransformFlagsKHR`]
//! - [`create_display_mode_khr`]
//! - [`create_display_plane_surface_khr`]
//! - [`get_display_mode_properties_khr`]
//! - [`get_display_plane_capabilities_khr`]
//! - [`get_display_plane_supported_displays_khr`]
//! - [`get_physical_device_display_plane_properties_khr`]
//! - [`get_physical_device_display_properties_khr`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::{
    entry::Entry,
    extensions::khr_surface::{SurfaceKHR, SurfaceTransformFlagBitsKHR},
    vulkan1_0::{
        AllocationCallbacks, BaseInStructure, Bool32, Extent2D, Instance, Offset2D, PhysicalDevice, StructureType,
        VulkanResultCodes,
    },
    AsRaw, Handle, SmallVec, Unique, VulkanResult,
};
#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{
    ffi::CStr,
    iter::{Extend, FromIterator, IntoIterator},
    marker::PhantomData,
    mem::MaybeUninit,
    os::raw::c_char,
};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_DISPLAY_SPEC_VERSION")]
pub const KHR_DISPLAY_SPEC_VERSION: u32 = 23;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_DISPLAY_EXTENSION_NAME")]
pub const KHR_DISPLAY_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_display");
///[vkGetPhysicalDeviceDisplayPropertiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceDisplayPropertiesKHR.html) - Query information about the available displays
///# C Specifications
///Various functions are provided for enumerating the available display devices
///present on a Vulkan physical device.
///To query information about the available displays, call:
///```c
///// Provided by VK_KHR_display
///VkResult vkGetPhysicalDeviceDisplayPropertiesKHR(
///    VkPhysicalDevice                            physicalDevice,
///    uint32_t*                                   pPropertyCount,
///    VkDisplayPropertiesKHR*                     pProperties);
///```
///# Parameters
/// - [`physical_device`] is a physical device.
/// - [`p_property_count`] is a pointer to an integer related to the number of display devices
///   available or queried, as described below.
/// - [`p_properties`] is either `NULL` or a pointer to an array of [`DisplayPropertiesKHR`]
///   structures.
///# Description
///If [`p_properties`] is `NULL`, then the number of display devices available
///for [`physical_device`] is returned in [`p_property_count`].
///Otherwise, [`p_property_count`] **must**  point to a variable set by the user to
///the number of elements in the [`p_properties`] array, and on return the
///variable is overwritten with the number of structures actually written to
///[`p_properties`].
///If the value of [`p_property_count`] is less than the number of display
///devices for [`physical_device`], at most [`p_property_count`] structures
///will be written, and `VK_INCOMPLETE` will be returned instead of
///`VK_SUCCESS`, to indicate that not all the available properties were
///returned.
///## Valid Usage (Implicit)
/// - [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
/// - [`p_property_count`] **must**  be a valid pointer to a `uint32_t` value
/// - If the value referenced by [`p_property_count`] is not `0`, and [`p_properties`] is not
///   `NULL`, [`p_properties`] **must**  be a valid pointer to an array of
///   [`p_property_count`][`DisplayPropertiesKHR`] structures
///
///## Return Codes
/// * - `VK_SUCCESS`  - `VK_INCOMPLETE`
/// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///# Related
/// - [`VK_KHR_display`]
/// - [`DisplayPropertiesKHR`]
/// - [`PhysicalDevice`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkGetPhysicalDeviceDisplayPropertiesKHR")]
pub type FNGetPhysicalDeviceDisplayPropertiesKhr = Option<
    for<'lt> unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_property_count: *mut u32,
        p_properties: *mut DisplayPropertiesKHR<'lt>,
    ) -> VulkanResultCodes,
>;
///[vkGetPhysicalDeviceDisplayPlanePropertiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceDisplayPlanePropertiesKHR.html) - Query the plane properties
///# C Specifications
///Images are presented to individual planes on a display.
///Devices  **must**  support at least one plane on each display.
///Planes  **can**  be stacked and blended to composite multiple images on one
///display.
///Devices  **may**  support only a fixed stacking order and fixed mapping between
///planes and displays, or they  **may**  allow arbitrary application specified
///stacking orders and mappings between planes and displays.
///To query the properties of device display planes, call:
///```c
///// Provided by VK_KHR_display
///VkResult vkGetPhysicalDeviceDisplayPlanePropertiesKHR(
///    VkPhysicalDevice                            physicalDevice,
///    uint32_t*                                   pPropertyCount,
///    VkDisplayPlanePropertiesKHR*                pProperties);
///```
///# Parameters
/// - [`physical_device`] is a physical device.
/// - [`p_property_count`] is a pointer to an integer related to the number of display planes
///   available or queried, as described below.
/// - [`p_properties`] is either `NULL` or a pointer to an array of [`DisplayPlanePropertiesKHR`]
///   structures.
///# Description
///If [`p_properties`] is `NULL`, then the number of display planes available
///for [`physical_device`] is returned in [`p_property_count`].
///Otherwise, [`p_property_count`] **must**  point to a variable set by the user to
///the number of elements in the [`p_properties`] array, and on return the
///variable is overwritten with the number of structures actually written to
///[`p_properties`].
///If the value of [`p_property_count`] is less than the number of display
///planes for [`physical_device`], at most [`p_property_count`] structures
///will be written.
///## Valid Usage (Implicit)
/// - [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
/// - [`p_property_count`] **must**  be a valid pointer to a `uint32_t` value
/// - If the value referenced by [`p_property_count`] is not `0`, and [`p_properties`] is not
///   `NULL`, [`p_properties`] **must**  be a valid pointer to an array of
///   [`p_property_count`][`DisplayPlanePropertiesKHR`] structures
///
///## Return Codes
/// * - `VK_SUCCESS`  - `VK_INCOMPLETE`
/// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///# Related
/// - [`VK_KHR_display`]
/// - [`DisplayPlanePropertiesKHR`]
/// - [`PhysicalDevice`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkGetPhysicalDeviceDisplayPlanePropertiesKHR")]
pub type FNGetPhysicalDeviceDisplayPlanePropertiesKhr = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_property_count: *mut u32,
        p_properties: *mut DisplayPlanePropertiesKHR,
    ) -> VulkanResultCodes,
>;
///[vkGetDisplayPlaneSupportedDisplaysKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDisplayPlaneSupportedDisplaysKHR.html) - Query the list of displays a plane supports
///# C Specifications
///To determine which displays a plane is usable with, call
///```c
///// Provided by VK_KHR_display
///VkResult vkGetDisplayPlaneSupportedDisplaysKHR(
///    VkPhysicalDevice                            physicalDevice,
///    uint32_t                                    planeIndex,
///    uint32_t*                                   pDisplayCount,
///    VkDisplayKHR*                               pDisplays);
///```
///# Parameters
/// - [`physical_device`] is a physical device.
/// - [`plane_index`] is the plane which the application wishes to use, and  **must**  be in the
///   range [0, physical device plane count - 1].
/// - [`p_display_count`] is a pointer to an integer related to the number of displays available or
///   queried, as described below.
/// - [`p_displays`] is either `NULL` or a pointer to an array of [`DisplayKHR`] handles.
///# Description
///If [`p_displays`] is `NULL`, then the number of displays usable with the
///specified [`plane_index`] for [`physical_device`] is returned in
///[`p_display_count`].
///Otherwise, [`p_display_count`] **must**  point to a variable set by the user to
///the number of elements in the [`p_displays`] array, and on return the
///variable is overwritten with the number of handles actually written to
///[`p_displays`].
///If the value of [`p_display_count`] is less than the number of usable
///display-plane pairs for [`physical_device`], at most [`p_display_count`]
///handles will be written, and `VK_INCOMPLETE` will be returned instead of
///`VK_SUCCESS`, to indicate that not all the available pairs were
///returned.
///## Valid Usage
/// - [`plane_index`] **must**  be less than the number of display planes supported by the device as
///   determined by calling [`get_physical_device_display_plane_properties_khr`]
///
///## Valid Usage (Implicit)
/// - [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
/// - [`p_display_count`] **must**  be a valid pointer to a `uint32_t` value
/// - If the value referenced by [`p_display_count`] is not `0`, and [`p_displays`] is not `NULL`,
///   [`p_displays`] **must**  be a valid pointer to an array of [`p_display_count`][`DisplayKHR`]
///   handles
///
///## Return Codes
/// * - `VK_SUCCESS`  - `VK_INCOMPLETE`
/// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///# Related
/// - [`VK_KHR_display`]
/// - [`DisplayKHR`]
/// - [`PhysicalDevice`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkGetDisplayPlaneSupportedDisplaysKHR")]
pub type FNGetDisplayPlaneSupportedDisplaysKhr = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        plane_index: u32,
        p_display_count: *mut u32,
        p_displays: *mut DisplayKHR,
    ) -> VulkanResultCodes,
>;
///[vkGetDisplayModePropertiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDisplayModePropertiesKHR.html) - Query the set of mode properties supported by the display
///# C Specifications
///Each display has one or more supported modes associated with it by default.
///These built-in modes are queried by calling:
///```c
///// Provided by VK_KHR_display
///VkResult vkGetDisplayModePropertiesKHR(
///    VkPhysicalDevice                            physicalDevice,
///    VkDisplayKHR                                display,
///    uint32_t*                                   pPropertyCount,
///    VkDisplayModePropertiesKHR*                 pProperties);
///```
///# Parameters
/// - [`physical_device`] is the physical device associated with [`display`].
/// - [`display`] is the display to query.
/// - [`p_property_count`] is a pointer to an integer related to the number of display modes
///   available or queried, as described below.
/// - [`p_properties`] is either `NULL` or a pointer to an array of [`DisplayModePropertiesKHR`]
///   structures.
///# Description
///If [`p_properties`] is `NULL`, then the number of display modes available
///on the specified [`display`] for [`physical_device`] is returned in
///[`p_property_count`].
///Otherwise, [`p_property_count`] **must**  point to a variable set by the user to
///the number of elements in the [`p_properties`] array, and on return the
///variable is overwritten with the number of structures actually written to
///[`p_properties`].
///If the value of [`p_property_count`] is less than the number of display
///modes for [`physical_device`], at most [`p_property_count`] structures will
///be written, and `VK_INCOMPLETE` will be returned instead of
///`VK_SUCCESS`, to indicate that not all the available display modes were
///returned.
///## Valid Usage (Implicit)
/// - [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
/// - [`display`] **must**  be a valid [`DisplayKHR`] handle
/// - [`p_property_count`] **must**  be a valid pointer to a `uint32_t` value
/// - If the value referenced by [`p_property_count`] is not `0`, and [`p_properties`] is not
///   `NULL`, [`p_properties`] **must**  be a valid pointer to an array of
///   [`p_property_count`][`DisplayModePropertiesKHR`] structures
/// - [`display`] **must**  have been created, allocated, or retrieved from [`physical_device`]
///
///## Return Codes
/// * - `VK_SUCCESS`  - `VK_INCOMPLETE`
/// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///# Related
/// - [`VK_KHR_display`]
/// - [`DisplayKHR`]
/// - [`DisplayModePropertiesKHR`]
/// - [`PhysicalDevice`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkGetDisplayModePropertiesKHR")]
pub type FNGetDisplayModePropertiesKhr = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        display: DisplayKHR,
        p_property_count: *mut u32,
        p_properties: *mut DisplayModePropertiesKHR,
    ) -> VulkanResultCodes,
>;
///[vkCreateDisplayModeKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateDisplayModeKHR.html) - Create a display mode
///# C Specifications
///Additional modes  **may**  also be created by calling:
///```c
///// Provided by VK_KHR_display
///VkResult vkCreateDisplayModeKHR(
///    VkPhysicalDevice                            physicalDevice,
///    VkDisplayKHR                                display,
///    const VkDisplayModeCreateInfoKHR*           pCreateInfo,
///    const VkAllocationCallbacks*                pAllocator,
///    VkDisplayModeKHR*                           pMode);
///```
///# Parameters
/// - [`physical_device`] is the physical device associated with [`display`].
/// - [`display`] is the display to create an additional mode for.
/// - [`p_create_info`] is a pointer to a [`DisplayModeCreateInfoKHR`] structure describing the new
///   mode to create.
/// - [`p_allocator`] is the allocator used for host memory allocated for the display mode object when there is no more specific allocator available (see [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation)).
/// - [`p_mode`] is a pointer to a [`DisplayModeKHR`] handle in which the mode created is returned.
///# Description
///## Valid Usage (Implicit)
/// - [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
/// - [`display`] **must**  be a valid [`DisplayKHR`] handle
/// - [`p_create_info`] **must**  be a valid pointer to a valid [`DisplayModeCreateInfoKHR`]
///   structure
/// - If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid
///   [`AllocationCallbacks`] structure
/// - [`p_mode`] **must**  be a valid pointer to a [`DisplayModeKHR`] handle
/// - [`display`] **must**  have been created, allocated, or retrieved from [`physical_device`]
///
///## Host Synchronization
/// - Host access to [`display`] **must**  be externally synchronized
///
///## Return Codes
/// * - `VK_SUCCESS`
/// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  -
///   `VK_ERROR_INITIALIZATION_FAILED`
///# Related
/// - [`VK_KHR_display`]
/// - [`AllocationCallbacks`]
/// - [`DisplayKHR`]
/// - [`DisplayModeCreateInfoKHR`]
/// - [`DisplayModeKHR`]
/// - [`PhysicalDevice`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkCreateDisplayModeKHR")]
pub type FNCreateDisplayModeKhr = Option<
    for<'lt> unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        display: DisplayKHR,
        p_create_info: *const DisplayModeCreateInfoKHR<'lt>,
        p_allocator: *const AllocationCallbacks<'lt>,
        p_mode: *mut DisplayModeKHR,
    ) -> VulkanResultCodes,
>;
///[vkGetDisplayPlaneCapabilitiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDisplayPlaneCapabilitiesKHR.html) - Query capabilities of a mode and plane combination
///# C Specifications
///Applications that wish to present directly to a display  **must**  select which
///layer, or “plane” of the display they wish to target, and a mode to use
///with the display.
///Each display supports at least one plane.
///The capabilities of a given mode and plane combination are determined by
///calling:
///```c
///// Provided by VK_KHR_display
///VkResult vkGetDisplayPlaneCapabilitiesKHR(
///    VkPhysicalDevice                            physicalDevice,
///    VkDisplayModeKHR                            mode,
///    uint32_t                                    planeIndex,
///    VkDisplayPlaneCapabilitiesKHR*              pCapabilities);
///```
///# Parameters
/// - [`physical_device`] is the physical device associated with the display specified by [`mode`]
/// - [`mode`] is the display mode the application intends to program when using the specified
///   plane. Note this parameter also implicitly specifies a display.
/// - [`plane_index`] is the plane which the application intends to use with the display, and is
///   less than the number of display planes supported by the device.
/// - [`p_capabilities`] is a pointer to a [`DisplayPlaneCapabilitiesKHR`] structure in which the
///   capabilities are returned.
///# Description
///## Valid Usage (Implicit)
/// - [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
/// - [`mode`] **must**  be a valid [`DisplayModeKHR`] handle
/// - [`p_capabilities`] **must**  be a valid pointer to a [`DisplayPlaneCapabilitiesKHR`] structure
///
///## Host Synchronization
/// - Host access to [`mode`] **must**  be externally synchronized
///
///## Return Codes
/// * - `VK_SUCCESS`
/// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///# Related
/// - [`VK_KHR_display`]
/// - [`DisplayModeKHR`]
/// - [`DisplayPlaneCapabilitiesKHR`]
/// - [`PhysicalDevice`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkGetDisplayPlaneCapabilitiesKHR")]
pub type FNGetDisplayPlaneCapabilitiesKhr = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        mode: DisplayModeKHR,
        plane_index: u32,
        p_capabilities: *mut DisplayPlaneCapabilitiesKHR,
    ) -> VulkanResultCodes,
>;
///[vkCreateDisplayPlaneSurfaceKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateDisplayPlaneSurfaceKHR.html) - Create a slink:VkSurfaceKHR structure representing a display plane and mode
///# C Specifications
///A complete display configuration includes a mode, one or more display planes
///and any parameters describing their behavior, and parameters describing some
///aspects of the images associated with those planes.
///Display surfaces describe the configuration of a single plane within a
///complete display configuration.
///To create a [`SurfaceKHR`] object for a display plane, call:
///```c
///// Provided by VK_KHR_display
///VkResult vkCreateDisplayPlaneSurfaceKHR(
///    VkInstance                                  instance,
///    const VkDisplaySurfaceCreateInfoKHR*        pCreateInfo,
///    const VkAllocationCallbacks*                pAllocator,
///    VkSurfaceKHR*                               pSurface);
///```
///# Parameters
/// - [`instance`] is the instance corresponding to the physical device the targeted display is on.
/// - [`p_create_info`] is a pointer to a [`DisplaySurfaceCreateInfoKHR`] structure specifying which
///   mode, plane, and other parameters to use, as described below.
/// - [`p_allocator`] is the allocator used for host memory allocated for the surface object when there is no more specific allocator available (see [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation)).
/// - [`p_surface`] is a pointer to a [`SurfaceKHR`] handle in which the created surface is
///   returned.
///# Description
///## Valid Usage (Implicit)
/// - [`instance`] **must**  be a valid [`Instance`] handle
/// - [`p_create_info`] **must**  be a valid pointer to a valid [`DisplaySurfaceCreateInfoKHR`]
///   structure
/// - If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid
///   [`AllocationCallbacks`] structure
/// - [`p_surface`] **must**  be a valid pointer to a [`SurfaceKHR`] handle
///
///## Return Codes
/// * - `VK_SUCCESS`
/// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///# Related
/// - [`VK_KHR_display`]
/// - [`AllocationCallbacks`]
/// - [`DisplaySurfaceCreateInfoKHR`]
/// - [`Instance`]
/// - [`SurfaceKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkCreateDisplayPlaneSurfaceKHR")]
pub type FNCreateDisplayPlaneSurfaceKhr = Option<
    for<'lt> unsafe extern "system" fn(
        instance: Instance,
        p_create_info: *const DisplaySurfaceCreateInfoKHR<'lt>,
        p_allocator: *const AllocationCallbacks<'lt>,
        p_surface: *mut SurfaceKHR,
    ) -> VulkanResultCodes,
>;
///[VkDisplayPlaneAlphaFlagBitsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDisplayPlaneAlphaFlagBitsKHR.html) - Alpha blending type
///# C Specifications
///Bits which  **can**  be set in
///[`DisplaySurfaceCreateInfoKHR::alpha_mode`], specifying the type of
///alpha blending to use on a display, are:
///```c
///// Provided by VK_KHR_display
///typedef enum VkDisplayPlaneAlphaFlagBitsKHR {
///    VK_DISPLAY_PLANE_ALPHA_OPAQUE_BIT_KHR = 0x00000001,
///    VK_DISPLAY_PLANE_ALPHA_GLOBAL_BIT_KHR = 0x00000002,
///    VK_DISPLAY_PLANE_ALPHA_PER_PIXEL_BIT_KHR = 0x00000004,
///    VK_DISPLAY_PLANE_ALPHA_PER_PIXEL_PREMULTIPLIED_BIT_KHR = 0x00000008,
///} VkDisplayPlaneAlphaFlagBitsKHR;
///```
///# Description
/// - [`DisplayPlaneAlphaOpaqueKhr`] specifies that the source image will be treated as opaque.
/// - [`DisplayPlaneAlphaGlobalKhr`] specifies that a global alpha value  **must**  be specified
///   that will be applied to all pixels in the source image.
/// - [`DisplayPlaneAlphaPerPixelKhr`] specifies that the alpha value will be determined by the
///   alpha component of the source image’s pixels. If the source format contains no alpha values,
///   no blending will be applied. The source alpha values are not premultiplied into the source
///   image’s other color components.
/// - [`DisplayPlaneAlphaPerPixelPremultipliedKhr`] is equivalent to
///   [`DisplayPlaneAlphaPerPixelKhr`], except the source alpha values are assumed to be
///   premultiplied into the source image’s other color components.
///# Related
/// - [`VK_KHR_display`]
/// - [`DisplayPlaneAlphaFlagsKHR`]
/// - [`DisplaySurfaceCreateInfoKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkDisplayPlaneAlphaFlagBitsKHR")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
#[repr(u32)]
pub enum DisplayPlaneAlphaFlagBitsKHR {
    #[doc(hidden)]
    Empty = 0,
    ///[`DisplayPlaneAlphaOpaqueKhr`] specifies that the source
    ///image will be treated as opaque.
    DisplayPlaneAlphaOpaqueKhr = 1,
    ///[`DisplayPlaneAlphaGlobalKhr`] specifies that a global
    ///alpha value  **must**  be specified that will be applied to all pixels in the
    ///source image.
    DisplayPlaneAlphaGlobalKhr = 2,
    ///[`DisplayPlaneAlphaPerPixelKhr`] specifies that the alpha
    ///value will be determined by the alpha component of the source image’s
    ///pixels.
    ///If the source format contains no alpha values, no blending will be
    ///applied.
    ///The source alpha values are not premultiplied into the source image’s
    ///other color components.
    DisplayPlaneAlphaPerPixelKhr = 4,
    ///[`DisplayPlaneAlphaPerPixelPremultipliedKhr`] is
    ///equivalent to [`DisplayPlaneAlphaPerPixelKhr`], except the
    ///source alpha values are assumed to be premultiplied into the source
    ///image’s other color components.
    DisplayPlaneAlphaPerPixelPremultipliedKhr = 8,
}
impl const Default for DisplayPlaneAlphaFlagBitsKHR {
    fn default() -> Self {
        Self::Empty
    }
}
impl DisplayPlaneAlphaFlagBitsKHR {
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
///[VkDisplayPlaneAlphaFlagBitsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDisplayPlaneAlphaFlagBitsKHR.html) - Alpha blending type
///# C Specifications
///Bits which  **can**  be set in
///[`DisplaySurfaceCreateInfoKHR::alpha_mode`], specifying the type of
///alpha blending to use on a display, are:
///```c
///// Provided by VK_KHR_display
///typedef enum VkDisplayPlaneAlphaFlagBitsKHR {
///    VK_DISPLAY_PLANE_ALPHA_OPAQUE_BIT_KHR = 0x00000001,
///    VK_DISPLAY_PLANE_ALPHA_GLOBAL_BIT_KHR = 0x00000002,
///    VK_DISPLAY_PLANE_ALPHA_PER_PIXEL_BIT_KHR = 0x00000004,
///    VK_DISPLAY_PLANE_ALPHA_PER_PIXEL_PREMULTIPLIED_BIT_KHR = 0x00000008,
///} VkDisplayPlaneAlphaFlagBitsKHR;
///```
///# Description
/// - [`DisplayPlaneAlphaOpaqueKhr`] specifies that the source image will be treated as opaque.
/// - [`DisplayPlaneAlphaGlobalKhr`] specifies that a global alpha value  **must**  be specified
///   that will be applied to all pixels in the source image.
/// - [`DisplayPlaneAlphaPerPixelKhr`] specifies that the alpha value will be determined by the
///   alpha component of the source image’s pixels. If the source format contains no alpha values,
///   no blending will be applied. The source alpha values are not premultiplied into the source
///   image’s other color components.
/// - [`DisplayPlaneAlphaPerPixelPremultipliedKhr`] is equivalent to
///   [`DisplayPlaneAlphaPerPixelKhr`], except the source alpha values are assumed to be
///   premultiplied into the source image’s other color components.
///# Related
/// - [`VK_KHR_display`]
/// - [`DisplayPlaneAlphaFlagsKHR`]
/// - [`DisplaySurfaceCreateInfoKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkDisplayPlaneAlphaFlagsKHR")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct DisplayPlaneAlphaFlagsKHR(u32);
impl const Default for DisplayPlaneAlphaFlagsKHR {
    fn default() -> Self {
        Self(0)
    }
}
impl From<DisplayPlaneAlphaFlagBitsKHR> for DisplayPlaneAlphaFlagsKHR {
    fn from(from: DisplayPlaneAlphaFlagBitsKHR) -> Self {
        unsafe { Self::from_bits_unchecked(from as u32) }
    }
}
impl DisplayPlaneAlphaFlagsKHR {
    ///[`DisplayPlaneAlphaOpaqueKhr`] specifies that the source
    ///image will be treated as opaque.
    pub const DISPLAY_PLANE_ALPHA_OPAQUE_KHR: Self = Self(1);
    ///[`DisplayPlaneAlphaGlobalKhr`] specifies that a global
    ///alpha value  **must**  be specified that will be applied to all pixels in the
    ///source image.
    pub const DISPLAY_PLANE_ALPHA_GLOBAL_KHR: Self = Self(2);
    ///[`DisplayPlaneAlphaPerPixelKhr`] specifies that the alpha
    ///value will be determined by the alpha component of the source image’s
    ///pixels.
    ///If the source format contains no alpha values, no blending will be
    ///applied.
    ///The source alpha values are not premultiplied into the source image’s
    ///other color components.
    pub const DISPLAY_PLANE_ALPHA_PER_PIXEL_KHR: Self = Self(4);
    ///[`DisplayPlaneAlphaPerPixelPremultipliedKhr`] is
    ///equivalent to [`DisplayPlaneAlphaPerPixelKhr`], except the
    ///source alpha values are assumed to be premultiplied into the source
    ///image’s other color components.
    pub const DISPLAY_PLANE_ALPHA_PER_PIXEL_PREMULTIPLIED_KHR: Self = Self(8);
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
            all |= Self::DISPLAY_PLANE_ALPHA_OPAQUE_KHR;
        }
        {
            all |= Self::DISPLAY_PLANE_ALPHA_GLOBAL_KHR;
        }
        {
            all |= Self::DISPLAY_PLANE_ALPHA_PER_PIXEL_KHR;
        }
        {
            all |= Self::DISPLAY_PLANE_ALPHA_PER_PIXEL_PREMULTIPLIED_KHR;
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
impl const std::ops::BitOr for DisplayPlaneAlphaFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl const std::ops::BitOrAssign for DisplayPlaneAlphaFlagsKHR {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for DisplayPlaneAlphaFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl const std::ops::BitXorAssign for DisplayPlaneAlphaFlagsKHR {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for DisplayPlaneAlphaFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl const std::ops::BitAndAssign for DisplayPlaneAlphaFlagsKHR {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for DisplayPlaneAlphaFlagsKHR {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl const std::ops::SubAssign for DisplayPlaneAlphaFlagsKHR {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for DisplayPlaneAlphaFlagsKHR {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<DisplayPlaneAlphaFlagsKHR> for DisplayPlaneAlphaFlagsKHR {
    fn extend<T: IntoIterator<Item = DisplayPlaneAlphaFlagsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl Extend<DisplayPlaneAlphaFlagBitsKHR> for DisplayPlaneAlphaFlagsKHR {
    fn extend<T: IntoIterator<Item = DisplayPlaneAlphaFlagBitsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, <Self as From<DisplayPlaneAlphaFlagBitsKHR>>::from(i));
        }
    }
}
impl FromIterator<DisplayPlaneAlphaFlagsKHR> for DisplayPlaneAlphaFlagsKHR {
    fn from_iter<T: IntoIterator<Item = DisplayPlaneAlphaFlagsKHR>>(iterator: T) -> DisplayPlaneAlphaFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<DisplayPlaneAlphaFlagsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl FromIterator<DisplayPlaneAlphaFlagBitsKHR> for DisplayPlaneAlphaFlagsKHR {
    fn from_iter<T: IntoIterator<Item = DisplayPlaneAlphaFlagBitsKHR>>(iterator: T) -> DisplayPlaneAlphaFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<DisplayPlaneAlphaFlagBitsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for DisplayPlaneAlphaFlagsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(DisplayPlaneAlphaFlagsKHR);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == DisplayPlaneAlphaFlagsKHR::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self
                        .0
                        .contains(DisplayPlaneAlphaFlagsKHR::DISPLAY_PLANE_ALPHA_OPAQUE_KHR)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(DISPLAY_PLANE_ALPHA_OPAQUE_KHR))?;
                    }
                    if self
                        .0
                        .contains(DisplayPlaneAlphaFlagsKHR::DISPLAY_PLANE_ALPHA_GLOBAL_KHR)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(DISPLAY_PLANE_ALPHA_GLOBAL_KHR))?;
                    }
                    if self
                        .0
                        .contains(DisplayPlaneAlphaFlagsKHR::DISPLAY_PLANE_ALPHA_PER_PIXEL_KHR)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(DISPLAY_PLANE_ALPHA_PER_PIXEL_KHR))?;
                    }
                    if self
                        .0
                        .contains(DisplayPlaneAlphaFlagsKHR::DISPLAY_PLANE_ALPHA_PER_PIXEL_PREMULTIPLIED_KHR)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(DISPLAY_PLANE_ALPHA_PER_PIXEL_PREMULTIPLIED_KHR))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(DisplayPlaneAlphaFlagsKHR))
            .field(&Flags(*self))
            .finish()
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
#[doc(alias = "VkSurfaceTransformFlagsKHR")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct SurfaceTransformFlagsKHR(u32);
impl const Default for SurfaceTransformFlagsKHR {
    fn default() -> Self {
        Self(0)
    }
}
impl From<SurfaceTransformFlagBitsKHR> for SurfaceTransformFlagsKHR {
    fn from(from: SurfaceTransformFlagBitsKHR) -> Self {
        unsafe { Self::from_bits_unchecked(from as u32) }
    }
}
impl SurfaceTransformFlagsKHR {
    ///[`SurfaceTransformIdentityKhr`] specifies that image content
    ///is presented without being transformed.
    ///
    ///Provided by [`crate::extensions::khr_surface`]
    pub const SURFACE_TRANSFORM_IDENTITY_KHR: Self = Self(1);
    ///[`SurfaceTransformRotate90Khr`] specifies that image
    ///content is rotated 90 degrees clockwise.
    ///
    ///Provided by [`crate::extensions::khr_surface`]
    pub const SURFACE_TRANSFORM_ROTATE_90_KHR: Self = Self(2);
    ///[`SurfaceTransformRotate180Khr`] specifies that image
    ///content is rotated 180 degrees clockwise.
    ///
    ///Provided by [`crate::extensions::khr_surface`]
    pub const SURFACE_TRANSFORM_ROTATE_180_KHR: Self = Self(4);
    ///[`SurfaceTransformRotate270Khr`] specifies that image
    ///content is rotated 270 degrees clockwise.
    ///
    ///Provided by [`crate::extensions::khr_surface`]
    pub const SURFACE_TRANSFORM_ROTATE_270_KHR: Self = Self(8);
    ///[`SurfaceTransformHorizontalMirrorKhr`] specifies that
    ///image content is mirrored horizontally.
    ///
    ///Provided by [`crate::extensions::khr_surface`]
    pub const SURFACE_TRANSFORM_HORIZONTAL_MIRROR_KHR: Self = Self(16);
    ///[`SurfaceTransformHorizontalMirrorRotate90Khr`] specifies
    ///that image content is mirrored horizontally, then rotated 90 degrees
    ///clockwise.
    ///
    ///Provided by [`crate::extensions::khr_surface`]
    pub const SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_90_KHR: Self = Self(32);
    ///[`SurfaceTransformHorizontalMirrorRotate180Khr`]
    ///specifies that image content is mirrored horizontally, then rotated 180
    ///degrees clockwise.
    ///
    ///Provided by [`crate::extensions::khr_surface`]
    pub const SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_180_KHR: Self = Self(64);
    ///[`SurfaceTransformHorizontalMirrorRotate270Khr`]
    ///specifies that image content is mirrored horizontally, then rotated 270
    ///degrees clockwise.
    ///
    ///Provided by [`crate::extensions::khr_surface`]
    pub const SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_270_KHR: Self = Self(128);
    ///[`SurfaceTransformInheritKhr`] specifies that the
    ///presentation transform is not specified, and is instead determined by
    ///platform-specific considerations and mechanisms outside Vulkan.
    ///
    ///Provided by [`crate::extensions::khr_surface`]
    pub const SURFACE_TRANSFORM_INHERIT_KHR: Self = Self(256);
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
            all |= Self::SURFACE_TRANSFORM_IDENTITY_KHR;
        }
        {
            all |= Self::SURFACE_TRANSFORM_ROTATE_90_KHR;
        }
        {
            all |= Self::SURFACE_TRANSFORM_ROTATE_180_KHR;
        }
        {
            all |= Self::SURFACE_TRANSFORM_ROTATE_270_KHR;
        }
        {
            all |= Self::SURFACE_TRANSFORM_HORIZONTAL_MIRROR_KHR;
        }
        {
            all |= Self::SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_90_KHR;
        }
        {
            all |= Self::SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_180_KHR;
        }
        {
            all |= Self::SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_270_KHR;
        }
        {
            all |= Self::SURFACE_TRANSFORM_INHERIT_KHR;
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
impl const std::ops::BitOr for SurfaceTransformFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl const std::ops::BitOrAssign for SurfaceTransformFlagsKHR {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for SurfaceTransformFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl const std::ops::BitXorAssign for SurfaceTransformFlagsKHR {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for SurfaceTransformFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl const std::ops::BitAndAssign for SurfaceTransformFlagsKHR {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for SurfaceTransformFlagsKHR {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl const std::ops::SubAssign for SurfaceTransformFlagsKHR {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for SurfaceTransformFlagsKHR {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<SurfaceTransformFlagsKHR> for SurfaceTransformFlagsKHR {
    fn extend<T: IntoIterator<Item = SurfaceTransformFlagsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl Extend<SurfaceTransformFlagBitsKHR> for SurfaceTransformFlagsKHR {
    fn extend<T: IntoIterator<Item = SurfaceTransformFlagBitsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, <Self as From<SurfaceTransformFlagBitsKHR>>::from(i));
        }
    }
}
impl FromIterator<SurfaceTransformFlagsKHR> for SurfaceTransformFlagsKHR {
    fn from_iter<T: IntoIterator<Item = SurfaceTransformFlagsKHR>>(iterator: T) -> SurfaceTransformFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<SurfaceTransformFlagsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl FromIterator<SurfaceTransformFlagBitsKHR> for SurfaceTransformFlagsKHR {
    fn from_iter<T: IntoIterator<Item = SurfaceTransformFlagBitsKHR>>(iterator: T) -> SurfaceTransformFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<SurfaceTransformFlagBitsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for SurfaceTransformFlagsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(SurfaceTransformFlagsKHR);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == SurfaceTransformFlagsKHR::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self
                        .0
                        .contains(SurfaceTransformFlagsKHR::SURFACE_TRANSFORM_IDENTITY_KHR)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(SURFACE_TRANSFORM_IDENTITY_KHR))?;
                    }
                    if self
                        .0
                        .contains(SurfaceTransformFlagsKHR::SURFACE_TRANSFORM_ROTATE_90_KHR)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(SURFACE_TRANSFORM_ROTATE_90_KHR))?;
                    }
                    if self
                        .0
                        .contains(SurfaceTransformFlagsKHR::SURFACE_TRANSFORM_ROTATE_180_KHR)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(SURFACE_TRANSFORM_ROTATE_180_KHR))?;
                    }
                    if self
                        .0
                        .contains(SurfaceTransformFlagsKHR::SURFACE_TRANSFORM_ROTATE_270_KHR)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(SURFACE_TRANSFORM_ROTATE_270_KHR))?;
                    }
                    if self
                        .0
                        .contains(SurfaceTransformFlagsKHR::SURFACE_TRANSFORM_HORIZONTAL_MIRROR_KHR)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(SURFACE_TRANSFORM_HORIZONTAL_MIRROR_KHR))?;
                    }
                    if self
                        .0
                        .contains(SurfaceTransformFlagsKHR::SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_90_KHR)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_90_KHR))?;
                    }
                    if self
                        .0
                        .contains(SurfaceTransformFlagsKHR::SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_180_KHR)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_180_KHR))?;
                    }
                    if self
                        .0
                        .contains(SurfaceTransformFlagsKHR::SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_270_KHR)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_270_KHR))?;
                    }
                    if self.0.contains(SurfaceTransformFlagsKHR::SURFACE_TRANSFORM_INHERIT_KHR) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(SURFACE_TRANSFORM_INHERIT_KHR))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(SurfaceTransformFlagsKHR))
            .field(&Flags(*self))
            .finish()
    }
}
///[VkDisplayModeCreateFlagsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDisplayModeCreateFlagsKHR.html) - Reserved for future use
///# C Specifications
///```c
///// Provided by VK_KHR_display
///typedef VkFlags VkDisplayModeCreateFlagsKHR;
///```
///# Related
/// - [`VK_KHR_display`]
/// - [`DisplayModeCreateInfoKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct DisplayModeCreateFlagsKHR(u32);
impl const Default for DisplayModeCreateFlagsKHR {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for DisplayModeCreateFlagsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple(stringify!(DisplayModeCreateFlagsKHR))
            .field(&self.0)
            .finish()
    }
}
///[VkDisplaySurfaceCreateFlagsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDisplaySurfaceCreateFlagsKHR.html) - Reserved for future use
///# C Specifications
///```c
///// Provided by VK_KHR_display
///typedef VkFlags VkDisplaySurfaceCreateFlagsKHR;
///```
///# Related
/// - [`VK_KHR_display`]
/// - [`DisplaySurfaceCreateInfoKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct DisplaySurfaceCreateFlagsKHR(u32);
impl const Default for DisplaySurfaceCreateFlagsKHR {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for DisplaySurfaceCreateFlagsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple(stringify!(DisplaySurfaceCreateFlagsKHR))
            .field(&self.0)
            .finish()
    }
}
///[VkDisplayPropertiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDisplayPropertiesKHR.html) - Structure describing an available display device
///# C Specifications
///The [`DisplayPropertiesKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_display
///typedef struct VkDisplayPropertiesKHR {
///    VkDisplayKHR                  display;
///    const char*                   displayName;
///    VkExtent2D                    physicalDimensions;
///    VkExtent2D                    physicalResolution;
///    VkSurfaceTransformFlagsKHR    supportedTransforms;
///    VkBool32                      planeReorderPossible;
///    VkBool32                      persistentContent;
///} VkDisplayPropertiesKHR;
///```
///# Members
/// - [`display`] is a handle that is used to refer to the display described here. This handle will
///   be valid for the lifetime of the Vulkan instance.
/// - [`display_name`] is `NULL` or a pointer to a null-terminated UTF-8 string containing the name
///   of the display. Generally, this will be the name provided by the display’s EDID. If `NULL`, no
///   suitable name is available. If not `NULL`, the string pointed to  **must**  remain accessible
///   and unmodified as long as [`display`] is valid.
/// - [`physical_dimensions`] describes the physical width and height of the visible portion of the
///   display, in millimeters.
/// - [`physical_resolution`] describes the physical, native, or preferred resolution of the
///   display.
///# Description
/// - [`supported_transforms`] is a bitmask of [`SurfaceTransformFlagBitsKHR`] describing which
///   transforms are supported by this display.
/// - [`plane_reorder_possible`] tells whether the planes on this display  **can**  have their z
///   order changed. If this is [`TRUE`], the application  **can**  re-arrange the planes on this
///   display in any order relative to each other.
/// - [`persistent_content`] tells whether the display supports self-refresh/internal buffering. If
///   this is true, the application  **can**  submit persistent present operations on swapchains
///   created against this display.
///# Related
/// - [`VK_KHR_display`]
/// - [`Bool32`]
/// - [`DisplayKHR`]
/// - [`DisplayProperties2KHR`]
/// - [`Extent2D`]
/// - [`SurfaceTransformFlagsKHR`]
/// - [`get_physical_device_display_properties_khr`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkDisplayPropertiesKHR")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct DisplayPropertiesKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`display`] is a handle that is used to refer to the display described
    ///here.
    ///This handle will be valid for the lifetime of the Vulkan instance.
    pub display: DisplayKHR,
    ///[`display_name`] is `NULL` or a pointer to a null-terminated UTF-8
    ///string containing the name of the display.
    ///Generally, this will be the name provided by the display’s EDID.
    ///If `NULL`, no suitable name is available.
    ///If not `NULL`, the string pointed to  **must**  remain accessible and
    ///unmodified as long as [`display`] is valid.
    pub display_name: *const c_char,
    ///[`physical_dimensions`] describes the physical width and height of the
    ///visible portion of the display, in millimeters.
    pub physical_dimensions: Extent2D,
    ///[`physical_resolution`] describes the physical, native, or preferred
    ///resolution of the display.
    pub physical_resolution: Extent2D,
    ///No documentation found
    pub supported_transforms: SurfaceTransformFlagsKHR,
    ///No documentation found
    pub plane_reorder_possible: Bool32,
    ///No documentation found
    pub persistent_content: Bool32,
}
impl<'lt> Default for DisplayPropertiesKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            display: Default::default(),
            display_name: std::ptr::null(),
            physical_dimensions: Default::default(),
            physical_resolution: Default::default(),
            supported_transforms: Default::default(),
            plane_reorder_possible: 0,
            persistent_content: 0,
        }
    }
}
impl<'lt> DisplayPropertiesKHR<'lt> {
    ///Gets the raw value of [`Self::display_name`]
    pub fn display_name_raw(&self) -> *const c_char {
        self.display_name
    }
    ///Gets the raw value of [`Self::plane_reorder_possible`]
    pub fn plane_reorder_possible_raw(&self) -> Bool32 {
        self.plane_reorder_possible
    }
    ///Gets the raw value of [`Self::persistent_content`]
    pub fn persistent_content_raw(&self) -> Bool32 {
        self.persistent_content
    }
    ///Sets the raw value of [`Self::display_name`]
    pub fn set_display_name_raw(mut self, value: *const c_char) -> Self {
        self.display_name = value;
        self
    }
    ///Sets the raw value of [`Self::plane_reorder_possible`]
    pub fn set_plane_reorder_possible_raw(mut self, value: Bool32) -> Self {
        self.plane_reorder_possible = value;
        self
    }
    ///Sets the raw value of [`Self::persistent_content`]
    pub fn set_persistent_content_raw(mut self, value: Bool32) -> Self {
        self.persistent_content = value;
        self
    }
    ///Gets the value of [`Self::display`]
    pub fn display(&self) -> DisplayKHR {
        self.display
    }
    ///Gets the value of [`Self::display_name`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn display_name(&self) -> &'lt CStr {
        CStr::from_ptr(self.display_name)
    }
    ///Gets the value of [`Self::physical_dimensions`]
    pub fn physical_dimensions(&self) -> Extent2D {
        self.physical_dimensions
    }
    ///Gets the value of [`Self::physical_resolution`]
    pub fn physical_resolution(&self) -> Extent2D {
        self.physical_resolution
    }
    ///Gets the value of [`Self::supported_transforms`]
    pub fn supported_transforms(&self) -> SurfaceTransformFlagsKHR {
        self.supported_transforms
    }
    ///Gets the value of [`Self::plane_reorder_possible`]
    pub fn plane_reorder_possible(&self) -> bool {
        unsafe { std::mem::transmute(self.plane_reorder_possible as u8) }
    }
    ///Gets the value of [`Self::persistent_content`]
    pub fn persistent_content(&self) -> bool {
        unsafe { std::mem::transmute(self.persistent_content as u8) }
    }
    ///Gets a mutable reference to the value of [`Self::display`]
    pub fn display_mut(&mut self) -> &mut DisplayKHR {
        &mut self.display
    }
    ///Gets a mutable reference to the value of [`Self::physical_dimensions`]
    pub fn physical_dimensions_mut(&mut self) -> &mut Extent2D {
        &mut self.physical_dimensions
    }
    ///Gets a mutable reference to the value of [`Self::physical_resolution`]
    pub fn physical_resolution_mut(&mut self) -> &mut Extent2D {
        &mut self.physical_resolution
    }
    ///Gets a mutable reference to the value of [`Self::supported_transforms`]
    pub fn supported_transforms_mut(&mut self) -> &mut SurfaceTransformFlagsKHR {
        &mut self.supported_transforms
    }
    ///Gets a mutable reference to the value of [`Self::plane_reorder_possible`]
    pub fn plane_reorder_possible_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.plane_reorder_possible as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.plane_reorder_possible as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::persistent_content`]
    pub fn persistent_content_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.persistent_content as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.persistent_content as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Sets the value of [`Self::display`]
    pub fn set_display(mut self, value: crate::extensions::khr_display::DisplayKHR) -> Self {
        self.display = value;
        self
    }
    ///Sets the value of [`Self::display_name`]
    pub fn set_display_name(mut self, value: *const std::os::raw::c_char) -> Self {
        self.display_name = value;
        self
    }
    ///Sets the value of [`Self::physical_dimensions`]
    pub fn set_physical_dimensions(mut self, value: crate::vulkan1_0::Extent2D) -> Self {
        self.physical_dimensions = value;
        self
    }
    ///Sets the value of [`Self::physical_resolution`]
    pub fn set_physical_resolution(mut self, value: crate::vulkan1_0::Extent2D) -> Self {
        self.physical_resolution = value;
        self
    }
    ///Sets the value of [`Self::supported_transforms`]
    pub fn set_supported_transforms(mut self, value: crate::extensions::khr_display::SurfaceTransformFlagsKHR) -> Self {
        self.supported_transforms = value;
        self
    }
    ///Sets the value of [`Self::plane_reorder_possible`]
    pub fn set_plane_reorder_possible(mut self, value: bool) -> Self {
        self.plane_reorder_possible = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::persistent_content`]
    pub fn set_persistent_content(mut self, value: bool) -> Self {
        self.persistent_content = value as u8 as u32;
        self
    }
}
///[VkDisplayPlanePropertiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDisplayPlanePropertiesKHR.html) - Structure describing display plane properties
///# C Specifications
///The [`DisplayPlanePropertiesKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_display
///typedef struct VkDisplayPlanePropertiesKHR {
///    VkDisplayKHR    currentDisplay;
///    uint32_t        currentStackIndex;
///} VkDisplayPlanePropertiesKHR;
///```
///# Members
/// - [`current_display`] is the handle of the display the plane is currently associated with. If
///   the plane is not currently attached to any displays, this will be [`crate::Handle::null`].
/// - [`current_stack_index`] is the current z-order of the plane. This will be between 0 and the
///   value returned by [`get_physical_device_display_plane_properties_khr`] in `pPropertyCount`.
///# Related
/// - [`VK_KHR_display`]
/// - [`DisplayKHR`]
/// - [`DisplayPlaneProperties2KHR`]
/// - [`get_physical_device_display_plane_properties_khr`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkDisplayPlanePropertiesKHR")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct DisplayPlanePropertiesKHR {
    ///[`current_display`] is the handle of the display the plane is currently
    ///associated with.
    ///If the plane is not currently attached to any displays, this will be
    ///[`crate::Handle::null`].
    pub current_display: DisplayKHR,
    ///[`current_stack_index`] is the current z-order of the plane.
    ///This will be between 0 and the value returned by
    ///[`get_physical_device_display_plane_properties_khr`] in
    ///`pPropertyCount`.
    pub current_stack_index: u32,
}
impl Default for DisplayPlanePropertiesKHR {
    fn default() -> Self {
        Self {
            current_display: Default::default(),
            current_stack_index: 0,
        }
    }
}
impl DisplayPlanePropertiesKHR {
    ///Gets the value of [`Self::current_display`]
    pub fn current_display(&self) -> DisplayKHR {
        self.current_display
    }
    ///Gets the value of [`Self::current_stack_index`]
    pub fn current_stack_index(&self) -> u32 {
        self.current_stack_index
    }
    ///Gets a mutable reference to the value of [`Self::current_display`]
    pub fn current_display_mut(&mut self) -> &mut DisplayKHR {
        &mut self.current_display
    }
    ///Gets a mutable reference to the value of [`Self::current_stack_index`]
    pub fn current_stack_index_mut(&mut self) -> &mut u32 {
        &mut self.current_stack_index
    }
    ///Sets the value of [`Self::current_display`]
    pub fn set_current_display(mut self, value: crate::extensions::khr_display::DisplayKHR) -> Self {
        self.current_display = value;
        self
    }
    ///Sets the value of [`Self::current_stack_index`]
    pub fn set_current_stack_index(mut self, value: u32) -> Self {
        self.current_stack_index = value;
        self
    }
}
///[VkDisplayModeParametersKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDisplayModeParametersKHR.html) - Structure describing display parameters associated with a display mode
///# C Specifications
///The [`DisplayModeParametersKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_display
///typedef struct VkDisplayModeParametersKHR {
///    VkExtent2D    visibleRegion;
///    uint32_t      refreshRate;
///} VkDisplayModeParametersKHR;
///```
///# Members
/// - [`visible_region`] is the 2D extents of the visible region.
/// - [`refresh_rate`] is a `uint32_t` that is the number of times the display is refreshed each
///   second multiplied by 1000.
///# Description
///## Valid Usage
/// - The `width` member of [`visible_region`] **must**  be greater than `0`
/// - The `height` member of [`visible_region`] **must**  be greater than `0`
/// - [`refresh_rate`] **must**  be greater than `0`
///# Related
/// - [`VK_KHR_display`]
/// - [`DisplayModeCreateInfoKHR`]
/// - [`DisplayModePropertiesKHR`]
/// - [`Extent2D`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkDisplayModeParametersKHR")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct DisplayModeParametersKHR {
    ///[`visible_region`] is the 2D extents of the visible region.
    pub visible_region: Extent2D,
    ///[`refresh_rate`] is a `uint32_t` that is the number of times the
    ///display is refreshed each second multiplied by 1000.
    pub refresh_rate: u32,
}
impl Default for DisplayModeParametersKHR {
    fn default() -> Self {
        Self {
            visible_region: Default::default(),
            refresh_rate: 0,
        }
    }
}
impl DisplayModeParametersKHR {
    ///Gets the value of [`Self::visible_region`]
    pub fn visible_region(&self) -> Extent2D {
        self.visible_region
    }
    ///Gets the value of [`Self::refresh_rate`]
    pub fn refresh_rate(&self) -> u32 {
        self.refresh_rate
    }
    ///Gets a mutable reference to the value of [`Self::visible_region`]
    pub fn visible_region_mut(&mut self) -> &mut Extent2D {
        &mut self.visible_region
    }
    ///Gets a mutable reference to the value of [`Self::refresh_rate`]
    pub fn refresh_rate_mut(&mut self) -> &mut u32 {
        &mut self.refresh_rate
    }
    ///Sets the value of [`Self::visible_region`]
    pub fn set_visible_region(mut self, value: crate::vulkan1_0::Extent2D) -> Self {
        self.visible_region = value;
        self
    }
    ///Sets the value of [`Self::refresh_rate`]
    pub fn set_refresh_rate(mut self, value: u32) -> Self {
        self.refresh_rate = value;
        self
    }
}
///[VkDisplayModePropertiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDisplayModePropertiesKHR.html) - Structure describing display mode properties
///# C Specifications
///The [`DisplayModePropertiesKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_display
///typedef struct VkDisplayModePropertiesKHR {
///    VkDisplayModeKHR              displayMode;
///    VkDisplayModeParametersKHR    parameters;
///} VkDisplayModePropertiesKHR;
///```
///# Members
/// - [`display_mode`] is a handle to the display mode described in this structure. This handle will
///   be valid for the lifetime of the Vulkan instance.
/// - [`parameters`] is a [`DisplayModeParametersKHR`] structure describing the display parameters
///   associated with [`display_mode`].
///# Related
/// - [`VK_KHR_display`]
/// - [`DisplayModeKHR`]
/// - [`DisplayModeParametersKHR`]
/// - [`DisplayModeProperties2KHR`]
/// - [`get_display_mode_properties_khr`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkDisplayModePropertiesKHR")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct DisplayModePropertiesKHR {
    ///[`display_mode`] is a handle to the display mode described in this
    ///structure.
    ///This handle will be valid for the lifetime of the Vulkan instance.
    pub display_mode: DisplayModeKHR,
    ///[`parameters`] is a [`DisplayModeParametersKHR`] structure
    ///describing the display parameters associated with [`display_mode`].
    pub parameters: DisplayModeParametersKHR,
}
impl Default for DisplayModePropertiesKHR {
    fn default() -> Self {
        Self {
            display_mode: Default::default(),
            parameters: Default::default(),
        }
    }
}
impl DisplayModePropertiesKHR {
    ///Gets the value of [`Self::display_mode`]
    pub fn display_mode(&self) -> DisplayModeKHR {
        self.display_mode
    }
    ///Gets the value of [`Self::parameters`]
    pub fn parameters(&self) -> DisplayModeParametersKHR {
        self.parameters
    }
    ///Gets a mutable reference to the value of [`Self::display_mode`]
    pub fn display_mode_mut(&mut self) -> &mut DisplayModeKHR {
        &mut self.display_mode
    }
    ///Gets a mutable reference to the value of [`Self::parameters`]
    pub fn parameters_mut(&mut self) -> &mut DisplayModeParametersKHR {
        &mut self.parameters
    }
    ///Sets the value of [`Self::display_mode`]
    pub fn set_display_mode(mut self, value: crate::extensions::khr_display::DisplayModeKHR) -> Self {
        self.display_mode = value;
        self
    }
    ///Sets the value of [`Self::parameters`]
    pub fn set_parameters(mut self, value: crate::extensions::khr_display::DisplayModeParametersKHR) -> Self {
        self.parameters = value;
        self
    }
}
///[VkDisplayModeCreateInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDisplayModeCreateInfoKHR.html) - Structure specifying parameters of a newly created display mode object
///# C Specifications
///The [`DisplayModeCreateInfoKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_display
///typedef struct VkDisplayModeCreateInfoKHR {
///    VkStructureType                sType;
///    const void*                    pNext;
///    VkDisplayModeCreateFlagsKHR    flags;
///    VkDisplayModeParametersKHR     parameters;
///} VkDisplayModeCreateInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is reserved for future use, and  **must**  be zero.
/// - [`parameters`] is a [`DisplayModeParametersKHR`] structure describing the display parameters
///   to use in creating the new mode. If the parameters are not compatible with the specified
///   display, the implementation  **must**  return `VK_ERROR_INITIALIZATION_FAILED`.
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DISPLAY_MODE_CREATE_INFO_KHR`
/// - [`p_next`] **must**  be `NULL`
/// - [`flags`] **must**  be `0`
/// - [`parameters`] **must**  be a valid [`DisplayModeParametersKHR`] structure
///# Related
/// - [`VK_KHR_display`]
/// - [`DisplayModeCreateFlagsKHR`]
/// - [`DisplayModeParametersKHR`]
/// - [`StructureType`]
/// - [`create_display_mode_khr`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkDisplayModeCreateInfoKHR")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct DisplayModeCreateInfoKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`flags`] is reserved for future use, and  **must**  be zero.
    pub flags: DisplayModeCreateFlagsKHR,
    ///[`parameters`] is a [`DisplayModeParametersKHR`] structure
    ///describing the display parameters to use in creating the new mode.
    ///If the parameters are not compatible with the specified display, the
    ///implementation  **must**  return `VK_ERROR_INITIALIZATION_FAILED`.
    pub parameters: DisplayModeParametersKHR,
}
impl<'lt> Default for DisplayModeCreateInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::DisplayModeCreateInfoKhr,
            p_next: std::ptr::null(),
            flags: Default::default(),
            parameters: Default::default(),
        }
    }
}
impl<'lt> DisplayModeCreateInfoKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Gets the value of [`Self::s_type`]
    pub fn s_type(&self) -> StructureType {
        self.s_type
    }
    ///Gets the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next(&self) -> &BaseInStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::flags`]
    pub fn flags(&self) -> DisplayModeCreateFlagsKHR {
        self.flags
    }
    ///Gets the value of [`Self::parameters`]
    pub fn parameters(&self) -> DisplayModeParametersKHR {
        self.parameters
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut DisplayModeCreateFlagsKHR {
        &mut self.flags
    }
    ///Gets a mutable reference to the value of [`Self::parameters`]
    pub fn parameters_mut(&mut self) -> &mut DisplayModeParametersKHR {
        &mut self.parameters
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::flags`]
    pub fn set_flags(mut self, value: crate::extensions::khr_display::DisplayModeCreateFlagsKHR) -> Self {
        self.flags = value;
        self
    }
    ///Sets the value of [`Self::parameters`]
    pub fn set_parameters(mut self, value: crate::extensions::khr_display::DisplayModeParametersKHR) -> Self {
        self.parameters = value;
        self
    }
}
///[VkDisplayPlaneCapabilitiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDisplayPlaneCapabilitiesKHR.html) - Structure describing capabilities of a mode and plane combination
///# C Specifications
///The [`DisplayPlaneCapabilitiesKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_display
///typedef struct VkDisplayPlaneCapabilitiesKHR {
///    VkDisplayPlaneAlphaFlagsKHR    supportedAlpha;
///    VkOffset2D                     minSrcPosition;
///    VkOffset2D                     maxSrcPosition;
///    VkExtent2D                     minSrcExtent;
///    VkExtent2D                     maxSrcExtent;
///    VkOffset2D                     minDstPosition;
///    VkOffset2D                     maxDstPosition;
///    VkExtent2D                     minDstExtent;
///    VkExtent2D                     maxDstExtent;
///} VkDisplayPlaneCapabilitiesKHR;
///```
///# Members
/// - [`supported_alpha`] is a bitmask of [`DisplayPlaneAlphaFlagBitsKHR`] describing the supported
///   alpha blending modes.
/// - [`min_src_position`] is the minimum source rectangle offset supported by this plane using the
///   specified mode.
/// - [`max_src_position`] is the maximum source rectangle offset supported by this plane using the
///   specified mode. The `x` and `y` components of [`max_src_position`] **must**  each be greater
///   than or equal to the `x` and `y` components of [`min_src_position`], respectively.
/// - [`min_src_extent`] is the minimum source rectangle size supported by this plane using the
///   specified mode.
/// - [`max_src_extent`] is the maximum source rectangle size supported by this plane using the
///   specified mode.
/// - [`min_dst_position`], [`max_dst_position`], [`min_dst_extent`], [`max_dst_extent`] all have
///   similar semantics to their corresponding `*Src*` equivalents, but apply to the output region
///   within the mode rather than the input region within the source image. Unlike the `*Src*`
///   offsets, [`min_dst_position`] and [`max_dst_position`] **may**  contain negative values.
///# Description
///The minimum and maximum position and extent fields describe the
///implementation limits, if any, as they apply to the specified display mode
///and plane.
///Vendors  **may**  support displaying a subset of a swapchain’s presentable images
///on the specified display plane.
///This is expressed by returning [`min_src_position`], [`max_src_position`],
///[`min_src_extent`], and [`max_src_extent`] values that indicate a range of
///possible positions and sizes which  **may**  be used to specify the region within
///the presentable images that source pixels will be read from when creating a
///swapchain on the specified display mode and plane.Vendors  **may**  also support mapping the
/// presentable images’ content to a
///subset or superset of the visible region in the specified display mode.
///This is expressed by returning [`min_dst_position`], [`max_dst_position`],
///[`min_dst_extent`] and [`max_dst_extent`] values that indicate a range of
///possible positions and sizes which  **may**  be used to describe the region
///within the display mode that the source pixels will be mapped to.Other vendors  **may**  support
/// only a 1-1 mapping between pixels in the
///presentable images and the display mode.
///This  **may**  be indicated by returning (0,0) for [`min_src_position`],
///[`max_src_position`], [`min_dst_position`], and [`max_dst_position`], and
///(display mode width, display mode height) for [`min_src_extent`],
///[`max_src_extent`], [`min_dst_extent`], and [`max_dst_extent`].The value [`supported_alpha`]
/// **must**  contain at least one valid
///[`DisplayPlaneAlphaFlagBitsKHR`] bit.These values indicate the limits of the implementation’s
/// individual fields.
///Not all combinations of values within the offset and extent ranges returned
///in [`DisplayPlaneCapabilitiesKHR`] are guaranteed to be supported.
///Presentation requests specifying unsupported combinations  **may**  fail.
///# Related
/// - [`VK_KHR_display`]
/// - [`DisplayPlaneAlphaFlagsKHR`]
/// - [`DisplayPlaneCapabilities2KHR`]
/// - [`Extent2D`]
/// - [`Offset2D`]
/// - [`get_display_plane_capabilities_khr`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkDisplayPlaneCapabilitiesKHR")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct DisplayPlaneCapabilitiesKHR {
    ///[`supported_alpha`] is a bitmask of
    ///[`DisplayPlaneAlphaFlagBitsKHR`] describing the supported alpha
    ///blending modes.
    pub supported_alpha: DisplayPlaneAlphaFlagsKHR,
    ///[`min_src_position`] is the minimum source rectangle offset supported by
    ///this plane using the specified mode.
    pub min_src_position: Offset2D,
    ///[`max_src_position`] is the maximum source rectangle offset supported by
    ///this plane using the specified mode.
    ///The `x` and `y` components of [`max_src_position`] **must**  each be
    ///greater than or equal to the `x` and `y` components of
    ///[`min_src_position`], respectively.
    pub max_src_position: Offset2D,
    ///[`min_src_extent`] is the minimum source rectangle size supported by
    ///this plane using the specified mode.
    pub min_src_extent: Extent2D,
    ///[`max_src_extent`] is the maximum source rectangle size supported by
    ///this plane using the specified mode.
    pub max_src_extent: Extent2D,
    ///[`min_dst_position`], [`max_dst_position`], [`min_dst_extent`],
    ///[`max_dst_extent`] all have similar semantics to their corresponding
    ///`*Src*` equivalents, but apply to the output region within the mode
    ///rather than the input region within the source image.
    ///Unlike the `*Src*` offsets, [`min_dst_position`] and
    ///[`max_dst_position`] **may**  contain negative values.
    pub min_dst_position: Offset2D,
    ///No documentation found
    pub max_dst_position: Offset2D,
    ///No documentation found
    pub min_dst_extent: Extent2D,
    ///No documentation found
    pub max_dst_extent: Extent2D,
}
impl Default for DisplayPlaneCapabilitiesKHR {
    fn default() -> Self {
        Self {
            supported_alpha: Default::default(),
            min_src_position: Default::default(),
            max_src_position: Default::default(),
            min_src_extent: Default::default(),
            max_src_extent: Default::default(),
            min_dst_position: Default::default(),
            max_dst_position: Default::default(),
            min_dst_extent: Default::default(),
            max_dst_extent: Default::default(),
        }
    }
}
impl DisplayPlaneCapabilitiesKHR {
    ///Gets the value of [`Self::supported_alpha`]
    pub fn supported_alpha(&self) -> DisplayPlaneAlphaFlagsKHR {
        self.supported_alpha
    }
    ///Gets the value of [`Self::min_src_position`]
    pub fn min_src_position(&self) -> Offset2D {
        self.min_src_position
    }
    ///Gets the value of [`Self::max_src_position`]
    pub fn max_src_position(&self) -> Offset2D {
        self.max_src_position
    }
    ///Gets the value of [`Self::min_src_extent`]
    pub fn min_src_extent(&self) -> Extent2D {
        self.min_src_extent
    }
    ///Gets the value of [`Self::max_src_extent`]
    pub fn max_src_extent(&self) -> Extent2D {
        self.max_src_extent
    }
    ///Gets the value of [`Self::min_dst_position`]
    pub fn min_dst_position(&self) -> Offset2D {
        self.min_dst_position
    }
    ///Gets the value of [`Self::max_dst_position`]
    pub fn max_dst_position(&self) -> Offset2D {
        self.max_dst_position
    }
    ///Gets the value of [`Self::min_dst_extent`]
    pub fn min_dst_extent(&self) -> Extent2D {
        self.min_dst_extent
    }
    ///Gets the value of [`Self::max_dst_extent`]
    pub fn max_dst_extent(&self) -> Extent2D {
        self.max_dst_extent
    }
    ///Gets a mutable reference to the value of [`Self::supported_alpha`]
    pub fn supported_alpha_mut(&mut self) -> &mut DisplayPlaneAlphaFlagsKHR {
        &mut self.supported_alpha
    }
    ///Gets a mutable reference to the value of [`Self::min_src_position`]
    pub fn min_src_position_mut(&mut self) -> &mut Offset2D {
        &mut self.min_src_position
    }
    ///Gets a mutable reference to the value of [`Self::max_src_position`]
    pub fn max_src_position_mut(&mut self) -> &mut Offset2D {
        &mut self.max_src_position
    }
    ///Gets a mutable reference to the value of [`Self::min_src_extent`]
    pub fn min_src_extent_mut(&mut self) -> &mut Extent2D {
        &mut self.min_src_extent
    }
    ///Gets a mutable reference to the value of [`Self::max_src_extent`]
    pub fn max_src_extent_mut(&mut self) -> &mut Extent2D {
        &mut self.max_src_extent
    }
    ///Gets a mutable reference to the value of [`Self::min_dst_position`]
    pub fn min_dst_position_mut(&mut self) -> &mut Offset2D {
        &mut self.min_dst_position
    }
    ///Gets a mutable reference to the value of [`Self::max_dst_position`]
    pub fn max_dst_position_mut(&mut self) -> &mut Offset2D {
        &mut self.max_dst_position
    }
    ///Gets a mutable reference to the value of [`Self::min_dst_extent`]
    pub fn min_dst_extent_mut(&mut self) -> &mut Extent2D {
        &mut self.min_dst_extent
    }
    ///Gets a mutable reference to the value of [`Self::max_dst_extent`]
    pub fn max_dst_extent_mut(&mut self) -> &mut Extent2D {
        &mut self.max_dst_extent
    }
    ///Sets the value of [`Self::supported_alpha`]
    pub fn set_supported_alpha(mut self, value: crate::extensions::khr_display::DisplayPlaneAlphaFlagsKHR) -> Self {
        self.supported_alpha = value;
        self
    }
    ///Sets the value of [`Self::min_src_position`]
    pub fn set_min_src_position(mut self, value: crate::vulkan1_0::Offset2D) -> Self {
        self.min_src_position = value;
        self
    }
    ///Sets the value of [`Self::max_src_position`]
    pub fn set_max_src_position(mut self, value: crate::vulkan1_0::Offset2D) -> Self {
        self.max_src_position = value;
        self
    }
    ///Sets the value of [`Self::min_src_extent`]
    pub fn set_min_src_extent(mut self, value: crate::vulkan1_0::Extent2D) -> Self {
        self.min_src_extent = value;
        self
    }
    ///Sets the value of [`Self::max_src_extent`]
    pub fn set_max_src_extent(mut self, value: crate::vulkan1_0::Extent2D) -> Self {
        self.max_src_extent = value;
        self
    }
    ///Sets the value of [`Self::min_dst_position`]
    pub fn set_min_dst_position(mut self, value: crate::vulkan1_0::Offset2D) -> Self {
        self.min_dst_position = value;
        self
    }
    ///Sets the value of [`Self::max_dst_position`]
    pub fn set_max_dst_position(mut self, value: crate::vulkan1_0::Offset2D) -> Self {
        self.max_dst_position = value;
        self
    }
    ///Sets the value of [`Self::min_dst_extent`]
    pub fn set_min_dst_extent(mut self, value: crate::vulkan1_0::Extent2D) -> Self {
        self.min_dst_extent = value;
        self
    }
    ///Sets the value of [`Self::max_dst_extent`]
    pub fn set_max_dst_extent(mut self, value: crate::vulkan1_0::Extent2D) -> Self {
        self.max_dst_extent = value;
        self
    }
}
///[VkDisplaySurfaceCreateInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDisplaySurfaceCreateInfoKHR.html) - Structure specifying parameters of a newly created display plane surface object
///# C Specifications
///The [`DisplaySurfaceCreateInfoKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_display
///typedef struct VkDisplaySurfaceCreateInfoKHR {
///    VkStructureType                   sType;
///    const void*                       pNext;
///    VkDisplaySurfaceCreateFlagsKHR    flags;
///    VkDisplayModeKHR                  displayMode;
///    uint32_t                          planeIndex;
///    uint32_t                          planeStackIndex;
///    VkSurfaceTransformFlagBitsKHR     transform;
///    float                             globalAlpha;
///    VkDisplayPlaneAlphaFlagBitsKHR    alphaMode;
///    VkExtent2D                        imageExtent;
///} VkDisplaySurfaceCreateInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is reserved for future use, and  **must**  be zero.
/// - [`display_mode`] is a [`DisplayModeKHR`] handle specifying the mode to use when displaying
///   this surface.
/// - [`plane_index`] is the plane on which this surface appears.
/// - [`plane_stack_index`] is the z-order of the plane.
/// - [`transform`] is a [`SurfaceTransformFlagBitsKHR`] value specifying the transformation to
///   apply to images as part of the scanout operation.
/// - [`global_alpha`] is the global alpha value. This value is ignored if [`alpha_mode`] is not
///   `VK_DISPLAY_PLANE_ALPHA_GLOBAL_BIT_KHR`.
/// - [`alpha_mode`] is a [`DisplayPlaneAlphaFlagBitsKHR`] value specifying the type of alpha
///   blending to use.
/// - [`image_extent`] is the size of the presentable images to use with the surface.
///# Description
///## Valid Usage
/// - [`plane_index`] **must**  be less than the number of display planes supported by the device as
///   determined by calling [`get_physical_device_display_plane_properties_khr`]
/// - If the `planeReorderPossible` member of the [`DisplayPropertiesKHR`] structure returned by
///   [`get_physical_device_display_properties_khr`] for the display corresponding to
///   [`display_mode`] is [`TRUE`] then [`plane_stack_index`] **must**  be less than the number of
///   display planes supported by the device as determined by calling
///   [`get_physical_device_display_plane_properties_khr`]; otherwise [`plane_stack_index`] **must**
///   equal the `currentStackIndex` member of [`DisplayPlanePropertiesKHR`] returned by
///   [`get_physical_device_display_plane_properties_khr`] for the display plane corresponding to
///   [`display_mode`]
/// - If [`alpha_mode`] is `VK_DISPLAY_PLANE_ALPHA_GLOBAL_BIT_KHR` then [`global_alpha`] **must**
///   be between `0` and `1`, inclusive
/// - [`alpha_mode`] **must**  be one of the bits present in the `supportedAlpha` member of
///   [`DisplayPlaneCapabilitiesKHR`] for the display plane corresponding to [`display_mode`]
/// - The `width` and `height` members of [`image_extent`] **must**  be less than or equal to
///   [`PhysicalDeviceLimits::max_image_dimension_2_d`]
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DISPLAY_SURFACE_CREATE_INFO_KHR`
/// - [`p_next`] **must**  be `NULL`
/// - [`flags`] **must**  be `0`
/// - [`display_mode`] **must**  be a valid [`DisplayModeKHR`] handle
/// - [`transform`] **must**  be a valid [`SurfaceTransformFlagBitsKHR`] value
/// - [`alpha_mode`] **must**  be a valid [`DisplayPlaneAlphaFlagBitsKHR`] value
///# Related
/// - [`VK_KHR_display`]
/// - [`DisplayModeKHR`]
/// - [`DisplayPlaneAlphaFlagBitsKHR`]
/// - [`DisplaySurfaceCreateFlagsKHR`]
/// - [`Extent2D`]
/// - [`StructureType`]
/// - [`SurfaceTransformFlagBitsKHR`]
/// - [`create_display_plane_surface_khr`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkDisplaySurfaceCreateInfoKHR")]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct DisplaySurfaceCreateInfoKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`flags`] is reserved for future use, and  **must**  be zero.
    pub flags: DisplaySurfaceCreateFlagsKHR,
    ///[`display_mode`] is a [`DisplayModeKHR`] handle specifying the mode
    ///to use when displaying this surface.
    pub display_mode: DisplayModeKHR,
    ///[`plane_index`] is the plane on which this surface appears.
    pub plane_index: u32,
    ///[`plane_stack_index`] is the z-order of the plane.
    pub plane_stack_index: u32,
    ///[`transform`] is a [`SurfaceTransformFlagBitsKHR`] value
    ///specifying the transformation to apply to images as part of the scanout
    ///operation.
    pub transform: SurfaceTransformFlagBitsKHR,
    ///[`global_alpha`] is the global alpha value.
    ///This value is ignored if [`alpha_mode`] is not
    ///`VK_DISPLAY_PLANE_ALPHA_GLOBAL_BIT_KHR`.
    pub global_alpha: f32,
    ///[`alpha_mode`] is a [`DisplayPlaneAlphaFlagBitsKHR`] value
    ///specifying the type of alpha blending to use.
    pub alpha_mode: DisplayPlaneAlphaFlagBitsKHR,
    ///[`image_extent`] is the size of the presentable images to use with the
    ///surface.
    pub image_extent: Extent2D,
}
impl<'lt> Default for DisplaySurfaceCreateInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::DisplaySurfaceCreateInfoKhr,
            p_next: std::ptr::null(),
            flags: Default::default(),
            display_mode: Default::default(),
            plane_index: 0,
            plane_stack_index: 0,
            transform: Default::default(),
            global_alpha: 0.0,
            alpha_mode: Default::default(),
            image_extent: Default::default(),
        }
    }
}
impl<'lt> DisplaySurfaceCreateInfoKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Gets the value of [`Self::s_type`]
    pub fn s_type(&self) -> StructureType {
        self.s_type
    }
    ///Gets the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next(&self) -> &BaseInStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::flags`]
    pub fn flags(&self) -> DisplaySurfaceCreateFlagsKHR {
        self.flags
    }
    ///Gets the value of [`Self::display_mode`]
    pub fn display_mode(&self) -> DisplayModeKHR {
        self.display_mode
    }
    ///Gets the value of [`Self::plane_index`]
    pub fn plane_index(&self) -> u32 {
        self.plane_index
    }
    ///Gets the value of [`Self::plane_stack_index`]
    pub fn plane_stack_index(&self) -> u32 {
        self.plane_stack_index
    }
    ///Gets the value of [`Self::transform`]
    pub fn transform(&self) -> SurfaceTransformFlagBitsKHR {
        self.transform
    }
    ///Gets the value of [`Self::global_alpha`]
    pub fn global_alpha(&self) -> f32 {
        self.global_alpha
    }
    ///Gets the value of [`Self::alpha_mode`]
    pub fn alpha_mode(&self) -> DisplayPlaneAlphaFlagBitsKHR {
        self.alpha_mode
    }
    ///Gets the value of [`Self::image_extent`]
    pub fn image_extent(&self) -> Extent2D {
        self.image_extent
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut DisplaySurfaceCreateFlagsKHR {
        &mut self.flags
    }
    ///Gets a mutable reference to the value of [`Self::display_mode`]
    pub fn display_mode_mut(&mut self) -> &mut DisplayModeKHR {
        &mut self.display_mode
    }
    ///Gets a mutable reference to the value of [`Self::plane_index`]
    pub fn plane_index_mut(&mut self) -> &mut u32 {
        &mut self.plane_index
    }
    ///Gets a mutable reference to the value of [`Self::plane_stack_index`]
    pub fn plane_stack_index_mut(&mut self) -> &mut u32 {
        &mut self.plane_stack_index
    }
    ///Gets a mutable reference to the value of [`Self::transform`]
    pub fn transform_mut(&mut self) -> &mut SurfaceTransformFlagBitsKHR {
        &mut self.transform
    }
    ///Gets a mutable reference to the value of [`Self::global_alpha`]
    pub fn global_alpha_mut(&mut self) -> &mut f32 {
        &mut self.global_alpha
    }
    ///Gets a mutable reference to the value of [`Self::alpha_mode`]
    pub fn alpha_mode_mut(&mut self) -> &mut DisplayPlaneAlphaFlagBitsKHR {
        &mut self.alpha_mode
    }
    ///Gets a mutable reference to the value of [`Self::image_extent`]
    pub fn image_extent_mut(&mut self) -> &mut Extent2D {
        &mut self.image_extent
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::flags`]
    pub fn set_flags(mut self, value: crate::extensions::khr_display::DisplaySurfaceCreateFlagsKHR) -> Self {
        self.flags = value;
        self
    }
    ///Sets the value of [`Self::display_mode`]
    pub fn set_display_mode(mut self, value: crate::extensions::khr_display::DisplayModeKHR) -> Self {
        self.display_mode = value;
        self
    }
    ///Sets the value of [`Self::plane_index`]
    pub fn set_plane_index(mut self, value: u32) -> Self {
        self.plane_index = value;
        self
    }
    ///Sets the value of [`Self::plane_stack_index`]
    pub fn set_plane_stack_index(mut self, value: u32) -> Self {
        self.plane_stack_index = value;
        self
    }
    ///Sets the value of [`Self::transform`]
    pub fn set_transform(mut self, value: crate::extensions::khr_surface::SurfaceTransformFlagBitsKHR) -> Self {
        self.transform = value;
        self
    }
    ///Sets the value of [`Self::global_alpha`]
    pub fn set_global_alpha(mut self, value: f32) -> Self {
        self.global_alpha = value;
        self
    }
    ///Sets the value of [`Self::alpha_mode`]
    pub fn set_alpha_mode(mut self, value: crate::extensions::khr_display::DisplayPlaneAlphaFlagBitsKHR) -> Self {
        self.alpha_mode = value;
        self
    }
    ///Sets the value of [`Self::image_extent`]
    pub fn set_image_extent(mut self, value: crate::vulkan1_0::Extent2D) -> Self {
        self.image_extent = value;
        self
    }
}
impl Instance {
    ///[vkCreateDisplayPlaneSurfaceKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateDisplayPlaneSurfaceKHR.html) - Create a slink:VkSurfaceKHR structure representing a display plane and mode
    ///# C Specifications
    ///A complete display configuration includes a mode, one or more display planes
    ///and any parameters describing their behavior, and parameters describing some
    ///aspects of the images associated with those planes.
    ///Display surfaces describe the configuration of a single plane within a
    ///complete display configuration.
    ///To create a [`SurfaceKHR`] object for a display plane, call:
    ///```c
    ///// Provided by VK_KHR_display
    ///VkResult vkCreateDisplayPlaneSurfaceKHR(
    ///    VkInstance                                  instance,
    ///    const VkDisplaySurfaceCreateInfoKHR*        pCreateInfo,
    ///    const VkAllocationCallbacks*                pAllocator,
    ///    VkSurfaceKHR*                               pSurface);
    ///```
    ///# Parameters
    /// - [`instance`] is the instance corresponding to the physical device the targeted display is
    ///   on.
    /// - [`p_create_info`] is a pointer to a [`DisplaySurfaceCreateInfoKHR`] structure specifying
    ///   which mode, plane, and other parameters to use, as described below.
    /// - [`p_allocator`] is the allocator used for host memory allocated for the surface object when there is no more specific allocator available (see [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation)).
    /// - [`p_surface`] is a pointer to a [`SurfaceKHR`] handle in which the created surface is
    ///   returned.
    ///# Description
    ///## Valid Usage (Implicit)
    /// - [`instance`] **must**  be a valid [`Instance`] handle
    /// - [`p_create_info`] **must**  be a valid pointer to a valid [`DisplaySurfaceCreateInfoKHR`]
    ///   structure
    /// - If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid
    ///   [`AllocationCallbacks`] structure
    /// - [`p_surface`] **must**  be a valid pointer to a [`SurfaceKHR`] handle
    ///
    ///## Return Codes
    /// * - `VK_SUCCESS`
    /// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    ///# Related
    /// - [`VK_KHR_display`]
    /// - [`AllocationCallbacks`]
    /// - [`DisplaySurfaceCreateInfoKHR`]
    /// - [`Instance`]
    /// - [`SurfaceKHR`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkCreateDisplayPlaneSurfaceKHR")]
    #[track_caller]
    #[inline]
    pub unsafe fn create_display_plane_surface_khr<'a: 'this, 'this, 'lt>(
        self: &'this Unique<'a, Instance>,
        p_create_info: &DisplaySurfaceCreateInfoKHR<'lt>,
        p_allocator: Option<&AllocationCallbacks<'lt>>,
    ) -> VulkanResult<Unique<'this, SurfaceKHR>> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .khr_display()
            .expect("extension/version not loaded")
            .create_display_plane_surface_khr()
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .khr_display()
            .unwrap_unchecked()
            .create_display_plane_surface_khr()
            .unwrap_unchecked();
        let mut p_surface = MaybeUninit::<SurfaceKHR>::uninit();
        let _return = _function(
            self.as_raw(),
            p_create_info as *const DisplaySurfaceCreateInfoKHR<'lt>,
            p_allocator
                .map(|v| v as *const AllocationCallbacks<'lt>)
                .unwrap_or_else(std::ptr::null),
            p_surface.as_mut_ptr(),
        );
        match _return {
            VulkanResultCodes::Success => {
                VulkanResult::Success(_return, Unique::new(self, p_surface.assume_init(), ()))
            },
            e => VulkanResult::Err(e),
        }
    }
}
impl PhysicalDevice {
    ///[vkGetPhysicalDeviceDisplayPropertiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceDisplayPropertiesKHR.html) - Query information about the available displays
    ///# C Specifications
    ///Various functions are provided for enumerating the available display devices
    ///present on a Vulkan physical device.
    ///To query information about the available displays, call:
    ///```c
    ///// Provided by VK_KHR_display
    ///VkResult vkGetPhysicalDeviceDisplayPropertiesKHR(
    ///    VkPhysicalDevice                            physicalDevice,
    ///    uint32_t*                                   pPropertyCount,
    ///    VkDisplayPropertiesKHR*                     pProperties);
    ///```
    ///# Parameters
    /// - [`physical_device`] is a physical device.
    /// - [`p_property_count`] is a pointer to an integer related to the number of display devices
    ///   available or queried, as described below.
    /// - [`p_properties`] is either `NULL` or a pointer to an array of [`DisplayPropertiesKHR`]
    ///   structures.
    ///# Description
    ///If [`p_properties`] is `NULL`, then the number of display devices available
    ///for [`physical_device`] is returned in [`p_property_count`].
    ///Otherwise, [`p_property_count`] **must**  point to a variable set by the user to
    ///the number of elements in the [`p_properties`] array, and on return the
    ///variable is overwritten with the number of structures actually written to
    ///[`p_properties`].
    ///If the value of [`p_property_count`] is less than the number of display
    ///devices for [`physical_device`], at most [`p_property_count`] structures
    ///will be written, and `VK_INCOMPLETE` will be returned instead of
    ///`VK_SUCCESS`, to indicate that not all the available properties were
    ///returned.
    ///## Valid Usage (Implicit)
    /// - [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
    /// - [`p_property_count`] **must**  be a valid pointer to a `uint32_t` value
    /// - If the value referenced by [`p_property_count`] is not `0`, and [`p_properties`] is not
    ///   `NULL`, [`p_properties`] **must**  be a valid pointer to an array of
    ///   [`p_property_count`][`DisplayPropertiesKHR`] structures
    ///
    ///## Return Codes
    /// * - `VK_SUCCESS`  - `VK_INCOMPLETE`
    /// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    ///# Related
    /// - [`VK_KHR_display`]
    /// - [`DisplayPropertiesKHR`]
    /// - [`PhysicalDevice`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkGetPhysicalDeviceDisplayPropertiesKHR")]
    #[track_caller]
    #[inline]
    pub unsafe fn get_physical_device_display_properties_khr<'a: 'this, 'this, 'lt>(
        self: &'this Unique<'a, PhysicalDevice>,
        p_property_count: Option<usize>,
    ) -> VulkanResult<SmallVec<DisplayPropertiesKHR<'lt>>> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .instance()
            .vtable()
            .khr_display()
            .expect("extension/version not loaded")
            .get_physical_device_display_properties_khr()
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .instance()
            .vtable()
            .khr_display()
            .unwrap_unchecked()
            .get_physical_device_display_properties_khr()
            .unwrap_unchecked();
        let mut p_property_count = match p_property_count {
            Some(v) => v as _,
            None => {
                let mut v = 0;
                _function(self.as_raw(), &mut v, std::ptr::null_mut());
                v
            },
        };
        let mut p_properties =
            SmallVec::<DisplayPropertiesKHR<'lt>>::from_elem(Default::default(), p_property_count as usize);
        let _return = _function(self.as_raw(), &mut p_property_count, p_properties.as_mut_ptr());
        match _return {
            VulkanResultCodes::Success | VulkanResultCodes::Incomplete => VulkanResult::Success(_return, p_properties),
            e => VulkanResult::Err(e),
        }
    }
}
impl PhysicalDevice {
    ///[vkGetPhysicalDeviceDisplayPlanePropertiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceDisplayPlanePropertiesKHR.html) - Query the plane properties
    ///# C Specifications
    ///Images are presented to individual planes on a display.
    ///Devices  **must**  support at least one plane on each display.
    ///Planes  **can**  be stacked and blended to composite multiple images on one
    ///display.
    ///Devices  **may**  support only a fixed stacking order and fixed mapping between
    ///planes and displays, or they  **may**  allow arbitrary application specified
    ///stacking orders and mappings between planes and displays.
    ///To query the properties of device display planes, call:
    ///```c
    ///// Provided by VK_KHR_display
    ///VkResult vkGetPhysicalDeviceDisplayPlanePropertiesKHR(
    ///    VkPhysicalDevice                            physicalDevice,
    ///    uint32_t*                                   pPropertyCount,
    ///    VkDisplayPlanePropertiesKHR*                pProperties);
    ///```
    ///# Parameters
    /// - [`physical_device`] is a physical device.
    /// - [`p_property_count`] is a pointer to an integer related to the number of display planes
    ///   available or queried, as described below.
    /// - [`p_properties`] is either `NULL` or a pointer to an array of
    ///   [`DisplayPlanePropertiesKHR`] structures.
    ///# Description
    ///If [`p_properties`] is `NULL`, then the number of display planes available
    ///for [`physical_device`] is returned in [`p_property_count`].
    ///Otherwise, [`p_property_count`] **must**  point to a variable set by the user to
    ///the number of elements in the [`p_properties`] array, and on return the
    ///variable is overwritten with the number of structures actually written to
    ///[`p_properties`].
    ///If the value of [`p_property_count`] is less than the number of display
    ///planes for [`physical_device`], at most [`p_property_count`] structures
    ///will be written.
    ///## Valid Usage (Implicit)
    /// - [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
    /// - [`p_property_count`] **must**  be a valid pointer to a `uint32_t` value
    /// - If the value referenced by [`p_property_count`] is not `0`, and [`p_properties`] is not
    ///   `NULL`, [`p_properties`] **must**  be a valid pointer to an array of
    ///   [`p_property_count`][`DisplayPlanePropertiesKHR`] structures
    ///
    ///## Return Codes
    /// * - `VK_SUCCESS`  - `VK_INCOMPLETE`
    /// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    ///# Related
    /// - [`VK_KHR_display`]
    /// - [`DisplayPlanePropertiesKHR`]
    /// - [`PhysicalDevice`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkGetPhysicalDeviceDisplayPlanePropertiesKHR")]
    #[track_caller]
    #[inline]
    pub unsafe fn get_physical_device_display_plane_properties_khr<'a: 'this, 'this>(
        self: &'this Unique<'a, PhysicalDevice>,
        p_property_count: Option<usize>,
    ) -> VulkanResult<SmallVec<DisplayPlanePropertiesKHR>> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .instance()
            .vtable()
            .khr_display()
            .expect("extension/version not loaded")
            .get_physical_device_display_plane_properties_khr()
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .instance()
            .vtable()
            .khr_display()
            .unwrap_unchecked()
            .get_physical_device_display_plane_properties_khr()
            .unwrap_unchecked();
        let mut p_property_count = match p_property_count {
            Some(v) => v as _,
            None => {
                let mut v = 0;
                _function(self.as_raw(), &mut v, std::ptr::null_mut());
                v
            },
        };
        let mut p_properties =
            SmallVec::<DisplayPlanePropertiesKHR>::from_elem(Default::default(), p_property_count as usize);
        let _return = _function(self.as_raw(), &mut p_property_count, p_properties.as_mut_ptr());
        match _return {
            VulkanResultCodes::Success | VulkanResultCodes::Incomplete => VulkanResult::Success(_return, p_properties),
            e => VulkanResult::Err(e),
        }
    }
}
impl PhysicalDevice {
    ///[vkGetDisplayPlaneSupportedDisplaysKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDisplayPlaneSupportedDisplaysKHR.html) - Query the list of displays a plane supports
    ///# C Specifications
    ///To determine which displays a plane is usable with, call
    ///```c
    ///// Provided by VK_KHR_display
    ///VkResult vkGetDisplayPlaneSupportedDisplaysKHR(
    ///    VkPhysicalDevice                            physicalDevice,
    ///    uint32_t                                    planeIndex,
    ///    uint32_t*                                   pDisplayCount,
    ///    VkDisplayKHR*                               pDisplays);
    ///```
    ///# Parameters
    /// - [`physical_device`] is a physical device.
    /// - [`plane_index`] is the plane which the application wishes to use, and  **must**  be in the
    ///   range [0, physical device plane count - 1].
    /// - [`p_display_count`] is a pointer to an integer related to the number of displays available
    ///   or queried, as described below.
    /// - [`p_displays`] is either `NULL` or a pointer to an array of [`DisplayKHR`] handles.
    ///# Description
    ///If [`p_displays`] is `NULL`, then the number of displays usable with the
    ///specified [`plane_index`] for [`physical_device`] is returned in
    ///[`p_display_count`].
    ///Otherwise, [`p_display_count`] **must**  point to a variable set by the user to
    ///the number of elements in the [`p_displays`] array, and on return the
    ///variable is overwritten with the number of handles actually written to
    ///[`p_displays`].
    ///If the value of [`p_display_count`] is less than the number of usable
    ///display-plane pairs for [`physical_device`], at most [`p_display_count`]
    ///handles will be written, and `VK_INCOMPLETE` will be returned instead of
    ///`VK_SUCCESS`, to indicate that not all the available pairs were
    ///returned.
    ///## Valid Usage
    /// - [`plane_index`] **must**  be less than the number of display planes supported by the
    ///   device as determined by calling [`get_physical_device_display_plane_properties_khr`]
    ///
    ///## Valid Usage (Implicit)
    /// - [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
    /// - [`p_display_count`] **must**  be a valid pointer to a `uint32_t` value
    /// - If the value referenced by [`p_display_count`] is not `0`, and [`p_displays`] is not
    ///   `NULL`, [`p_displays`] **must**  be a valid pointer to an array of
    ///   [`p_display_count`][`DisplayKHR`] handles
    ///
    ///## Return Codes
    /// * - `VK_SUCCESS`  - `VK_INCOMPLETE`
    /// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    ///# Related
    /// - [`VK_KHR_display`]
    /// - [`DisplayKHR`]
    /// - [`PhysicalDevice`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkGetDisplayPlaneSupportedDisplaysKHR")]
    #[track_caller]
    #[inline]
    pub unsafe fn get_display_plane_supported_displays_khr<'a: 'this, 'this>(
        self: &'this Unique<'a, PhysicalDevice>,
        plane_index: Option<u32>,
        p_display_count: Option<usize>,
    ) -> VulkanResult<SmallVec<Unique<'this, DisplayKHR>>> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .instance()
            .vtable()
            .khr_display()
            .expect("extension/version not loaded")
            .get_display_plane_supported_displays_khr()
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .instance()
            .vtable()
            .khr_display()
            .unwrap_unchecked()
            .get_display_plane_supported_displays_khr()
            .unwrap_unchecked();
        let mut p_display_count = match p_display_count {
            Some(v) => v as _,
            None => {
                let mut v = 0;
                _function(
                    self.as_raw(),
                    plane_index.unwrap_or_default() as _,
                    &mut v,
                    std::ptr::null_mut(),
                );
                v
            },
        };
        let mut p_displays = SmallVec::<DisplayKHR>::from_elem(Default::default(), p_display_count as usize);
        let _return = _function(
            self.as_raw(),
            plane_index.unwrap_or_default() as _,
            &mut p_display_count,
            p_displays.as_mut_ptr(),
        );
        match _return {
            VulkanResultCodes::Success | VulkanResultCodes::Incomplete => VulkanResult::Success(
                _return,
                p_displays.into_iter().map(|i| Unique::new(self, i, ())).collect(),
            ),
            e => VulkanResult::Err(e),
        }
    }
}
impl PhysicalDevice {
    ///[vkGetDisplayModePropertiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDisplayModePropertiesKHR.html) - Query the set of mode properties supported by the display
    ///# C Specifications
    ///Each display has one or more supported modes associated with it by default.
    ///These built-in modes are queried by calling:
    ///```c
    ///// Provided by VK_KHR_display
    ///VkResult vkGetDisplayModePropertiesKHR(
    ///    VkPhysicalDevice                            physicalDevice,
    ///    VkDisplayKHR                                display,
    ///    uint32_t*                                   pPropertyCount,
    ///    VkDisplayModePropertiesKHR*                 pProperties);
    ///```
    ///# Parameters
    /// - [`physical_device`] is the physical device associated with [`display`].
    /// - [`display`] is the display to query.
    /// - [`p_property_count`] is a pointer to an integer related to the number of display modes
    ///   available or queried, as described below.
    /// - [`p_properties`] is either `NULL` or a pointer to an array of [`DisplayModePropertiesKHR`]
    ///   structures.
    ///# Description
    ///If [`p_properties`] is `NULL`, then the number of display modes available
    ///on the specified [`display`] for [`physical_device`] is returned in
    ///[`p_property_count`].
    ///Otherwise, [`p_property_count`] **must**  point to a variable set by the user to
    ///the number of elements in the [`p_properties`] array, and on return the
    ///variable is overwritten with the number of structures actually written to
    ///[`p_properties`].
    ///If the value of [`p_property_count`] is less than the number of display
    ///modes for [`physical_device`], at most [`p_property_count`] structures will
    ///be written, and `VK_INCOMPLETE` will be returned instead of
    ///`VK_SUCCESS`, to indicate that not all the available display modes were
    ///returned.
    ///## Valid Usage (Implicit)
    /// - [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
    /// - [`display`] **must**  be a valid [`DisplayKHR`] handle
    /// - [`p_property_count`] **must**  be a valid pointer to a `uint32_t` value
    /// - If the value referenced by [`p_property_count`] is not `0`, and [`p_properties`] is not
    ///   `NULL`, [`p_properties`] **must**  be a valid pointer to an array of
    ///   [`p_property_count`][`DisplayModePropertiesKHR`] structures
    /// - [`display`] **must**  have been created, allocated, or retrieved from [`physical_device`]
    ///
    ///## Return Codes
    /// * - `VK_SUCCESS`  - `VK_INCOMPLETE`
    /// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    ///# Related
    /// - [`VK_KHR_display`]
    /// - [`DisplayKHR`]
    /// - [`DisplayModePropertiesKHR`]
    /// - [`PhysicalDevice`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkGetDisplayModePropertiesKHR")]
    #[track_caller]
    #[inline]
    pub unsafe fn get_display_mode_properties_khr<'a: 'this, 'this>(
        self: &'this Unique<'a, PhysicalDevice>,
        display: DisplayKHR,
        p_property_count: Option<usize>,
    ) -> VulkanResult<SmallVec<DisplayModePropertiesKHR>> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .instance()
            .vtable()
            .khr_display()
            .expect("extension/version not loaded")
            .get_display_mode_properties_khr()
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .instance()
            .vtable()
            .khr_display()
            .unwrap_unchecked()
            .get_display_mode_properties_khr()
            .unwrap_unchecked();
        let mut p_property_count = match p_property_count {
            Some(v) => v as _,
            None => {
                let mut v = 0;
                _function(self.as_raw(), display, &mut v, std::ptr::null_mut());
                v
            },
        };
        let mut p_properties =
            SmallVec::<DisplayModePropertiesKHR>::from_elem(Default::default(), p_property_count as usize);
        let _return = _function(self.as_raw(), display, &mut p_property_count, p_properties.as_mut_ptr());
        match _return {
            VulkanResultCodes::Success | VulkanResultCodes::Incomplete => VulkanResult::Success(_return, p_properties),
            e => VulkanResult::Err(e),
        }
    }
}
impl PhysicalDevice {
    ///[vkCreateDisplayModeKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateDisplayModeKHR.html) - Create a display mode
    ///# C Specifications
    ///Additional modes  **may**  also be created by calling:
    ///```c
    ///// Provided by VK_KHR_display
    ///VkResult vkCreateDisplayModeKHR(
    ///    VkPhysicalDevice                            physicalDevice,
    ///    VkDisplayKHR                                display,
    ///    const VkDisplayModeCreateInfoKHR*           pCreateInfo,
    ///    const VkAllocationCallbacks*                pAllocator,
    ///    VkDisplayModeKHR*                           pMode);
    ///```
    ///# Parameters
    /// - [`physical_device`] is the physical device associated with [`display`].
    /// - [`display`] is the display to create an additional mode for.
    /// - [`p_create_info`] is a pointer to a [`DisplayModeCreateInfoKHR`] structure describing the
    ///   new mode to create.
    /// - [`p_allocator`] is the allocator used for host memory allocated for the display mode object when there is no more specific allocator available (see [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation)).
    /// - [`p_mode`] is a pointer to a [`DisplayModeKHR`] handle in which the mode created is
    ///   returned.
    ///# Description
    ///## Valid Usage (Implicit)
    /// - [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
    /// - [`display`] **must**  be a valid [`DisplayKHR`] handle
    /// - [`p_create_info`] **must**  be a valid pointer to a valid [`DisplayModeCreateInfoKHR`]
    ///   structure
    /// - If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid
    ///   [`AllocationCallbacks`] structure
    /// - [`p_mode`] **must**  be a valid pointer to a [`DisplayModeKHR`] handle
    /// - [`display`] **must**  have been created, allocated, or retrieved from [`physical_device`]
    ///
    ///## Host Synchronization
    /// - Host access to [`display`] **must**  be externally synchronized
    ///
    ///## Return Codes
    /// * - `VK_SUCCESS`
    /// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  -
    ///   `VK_ERROR_INITIALIZATION_FAILED`
    ///# Related
    /// - [`VK_KHR_display`]
    /// - [`AllocationCallbacks`]
    /// - [`DisplayKHR`]
    /// - [`DisplayModeCreateInfoKHR`]
    /// - [`DisplayModeKHR`]
    /// - [`PhysicalDevice`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkCreateDisplayModeKHR")]
    #[track_caller]
    #[inline]
    pub unsafe fn create_display_mode_khr<'a: 'this, 'this: 'parent, 'parent, 'lt>(
        self: &'this Unique<'a, PhysicalDevice>,
        display: DisplayKHR,
        p_create_info: &DisplayModeCreateInfoKHR<'lt>,
        p_allocator: Option<&AllocationCallbacks<'lt>>,
        parent: &'parent Unique<'this, DisplayKHR>,
    ) -> VulkanResult<Unique<'parent, DisplayModeKHR>> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .instance()
            .vtable()
            .khr_display()
            .expect("extension/version not loaded")
            .create_display_mode_khr()
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .instance()
            .vtable()
            .khr_display()
            .unwrap_unchecked()
            .create_display_mode_khr()
            .unwrap_unchecked();
        let mut p_mode = MaybeUninit::<DisplayModeKHR>::uninit();
        let _return = _function(
            self.as_raw(),
            display,
            p_create_info as *const DisplayModeCreateInfoKHR<'lt>,
            p_allocator
                .map(|v| v as *const AllocationCallbacks<'lt>)
                .unwrap_or_else(std::ptr::null),
            p_mode.as_mut_ptr(),
        );
        match _return {
            VulkanResultCodes::Success => VulkanResult::Success(_return, Unique::new(parent, p_mode.assume_init(), ())),
            e => VulkanResult::Err(e),
        }
    }
}
impl PhysicalDevice {
    ///[vkGetDisplayPlaneCapabilitiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDisplayPlaneCapabilitiesKHR.html) - Query capabilities of a mode and plane combination
    ///# C Specifications
    ///Applications that wish to present directly to a display  **must**  select which
    ///layer, or “plane” of the display they wish to target, and a mode to use
    ///with the display.
    ///Each display supports at least one plane.
    ///The capabilities of a given mode and plane combination are determined by
    ///calling:
    ///```c
    ///// Provided by VK_KHR_display
    ///VkResult vkGetDisplayPlaneCapabilitiesKHR(
    ///    VkPhysicalDevice                            physicalDevice,
    ///    VkDisplayModeKHR                            mode,
    ///    uint32_t                                    planeIndex,
    ///    VkDisplayPlaneCapabilitiesKHR*              pCapabilities);
    ///```
    ///# Parameters
    /// - [`physical_device`] is the physical device associated with the display specified by
    ///   [`mode`]
    /// - [`mode`] is the display mode the application intends to program when using the specified
    ///   plane. Note this parameter also implicitly specifies a display.
    /// - [`plane_index`] is the plane which the application intends to use with the display, and is
    ///   less than the number of display planes supported by the device.
    /// - [`p_capabilities`] is a pointer to a [`DisplayPlaneCapabilitiesKHR`] structure in which
    ///   the capabilities are returned.
    ///# Description
    ///## Valid Usage (Implicit)
    /// - [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
    /// - [`mode`] **must**  be a valid [`DisplayModeKHR`] handle
    /// - [`p_capabilities`] **must**  be a valid pointer to a [`DisplayPlaneCapabilitiesKHR`]
    ///   structure
    ///
    ///## Host Synchronization
    /// - Host access to [`mode`] **must**  be externally synchronized
    ///
    ///## Return Codes
    /// * - `VK_SUCCESS`
    /// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    ///# Related
    /// - [`VK_KHR_display`]
    /// - [`DisplayModeKHR`]
    /// - [`DisplayPlaneCapabilitiesKHR`]
    /// - [`PhysicalDevice`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkGetDisplayPlaneCapabilitiesKHR")]
    #[track_caller]
    #[inline]
    pub unsafe fn get_display_plane_capabilities_khr<'a: 'this, 'this>(
        self: &'this Unique<'a, PhysicalDevice>,
        mode: DisplayModeKHR,
        plane_index: Option<u32>,
    ) -> VulkanResult<DisplayPlaneCapabilitiesKHR> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .instance()
            .vtable()
            .khr_display()
            .expect("extension/version not loaded")
            .get_display_plane_capabilities_khr()
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .instance()
            .vtable()
            .khr_display()
            .unwrap_unchecked()
            .get_display_plane_capabilities_khr()
            .unwrap_unchecked();
        let mut p_capabilities = MaybeUninit::<DisplayPlaneCapabilitiesKHR>::uninit();
        let _return = _function(
            self.as_raw(),
            mode,
            plane_index.unwrap_or_default() as _,
            p_capabilities.as_mut_ptr(),
        );
        match _return {
            VulkanResultCodes::Success => VulkanResult::Success(_return, p_capabilities.assume_init()),
            e => VulkanResult::Err(e),
        }
    }
}
///[VkDisplayKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDisplayKHR.html) - Opaque handle to a display object
///# C Specifications
///Displays are represented by [`DisplayKHR`] handles:
///```c
///// Provided by VK_KHR_display
///VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkDisplayKHR)
///```
///# Related
/// - [`VK_KHR_display`]
/// - [`DisplayPlanePropertiesKHR`]
/// - [`DisplayPropertiesKHR`]
/// - [`acquire_drm_display_ext`]
/// - [`acquire_winrt_display_nv`]
/// - [`acquire_xlib_display_ext`]
/// - [`create_display_mode_khr`]
/// - [`display_power_control_ext`]
/// - [`get_display_mode_properties2_khr`]
/// - [`get_display_mode_properties_khr`]
/// - [`get_display_plane_supported_displays_khr`]
/// - [`get_drm_display_ext`]
/// - [`get_rand_r_output_display_ext`]
/// - [`get_winrt_display_nv`]
/// - [`register_display_event_ext`]
/// - [`release_display_ext`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkDisplayKHR")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(transparent)]
pub struct DisplayKHR(pub u64);
impl DisplayKHR {
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
unsafe impl Send for DisplayKHR {}
impl Default for DisplayKHR {
    fn default() -> Self {
        Self::null()
    }
}
impl Handle for DisplayKHR {
    type Parent<'a> = Unique<'a, PhysicalDevice>;
    type VTable = ();
    type Metadata = ();
    #[inline]
    #[track_caller]
    unsafe fn destroy<'a>(self: &mut Unique<'a, Self>) {
        #[cfg(feature = "VK_EXT_direct_mode_display")]
        self.parent().release_display_ext(self.as_raw());
    }
    #[inline]
    unsafe fn load_vtable<'a>(&self, parent: &Self::Parent<'a>, metadata: &Self::Metadata) -> Self::VTable {
        ()
    }
}
impl<'a> Unique<'a, DisplayKHR> {
    ///Gets the reference to the [`Entry`]
    #[inline]
    pub fn entry(&self) -> &'a Entry {
        self.parent().parent().parent()
    }
    ///Gets the reference to the [`Instance`]
    #[inline]
    pub fn instance(&self) -> &'a Unique<'a, Instance> {
        self.parent().parent()
    }
    ///Gets the reference to the [`PhysicalDevice`]
    #[inline]
    pub fn physical_device(&self) -> &'a Unique<'a, PhysicalDevice> {
        self.parent()
    }
}
///[VkDisplayModeKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDisplayModeKHR.html) - Opaque handle to a display mode object
///# C Specifications
///Display modes are represented by [`DisplayModeKHR`] handles:
///```c
///// Provided by VK_KHR_display
///VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkDisplayModeKHR)
///```
///# Related
/// - [`VK_KHR_display`]
/// - [`DisplayModePropertiesKHR`]
/// - [`DisplayPlaneInfo2KHR`]
/// - [`DisplaySurfaceCreateInfoKHR`]
/// - [`create_display_mode_khr`]
/// - [`get_display_plane_capabilities_khr`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkDisplayModeKHR")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(transparent)]
pub struct DisplayModeKHR(pub u64);
impl DisplayModeKHR {
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
unsafe impl Send for DisplayModeKHR {}
impl Default for DisplayModeKHR {
    fn default() -> Self {
        Self::null()
    }
}
impl Handle for DisplayModeKHR {
    type Parent<'a> = Unique<'a, DisplayKHR>;
    type VTable = ();
    type Metadata = ();
    #[inline]
    #[track_caller]
    unsafe fn destroy<'a>(self: &mut Unique<'a, Self>) {}
    #[inline]
    unsafe fn load_vtable<'a>(&self, parent: &Self::Parent<'a>, metadata: &Self::Metadata) -> Self::VTable {
        ()
    }
}
impl<'a> Unique<'a, DisplayModeKHR> {
    ///Gets the reference to the [`Entry`]
    #[inline]
    pub fn entry(&self) -> &'a Entry {
        self.parent().parent().parent().parent()
    }
    ///Gets the reference to the [`Instance`]
    #[inline]
    pub fn instance(&self) -> &'a Unique<'a, Instance> {
        self.parent().parent().parent()
    }
    ///Gets the reference to the [`PhysicalDevice`]
    #[inline]
    pub fn physical_device(&self) -> &'a Unique<'a, PhysicalDevice> {
        self.parent().parent()
    }
    ///Gets the reference to the [`DisplayKHR`]
    #[inline]
    pub fn display_khr(&self) -> &'a Unique<'a, DisplayKHR> {
        self.parent()
    }
}
///The V-table of [`Instance`] for functions from `VK_KHR_display`
pub struct InstanceKhrDisplayVTable {
    ///See [`FNGetPhysicalDeviceDisplayPropertiesKhr`] for more information.
    pub get_physical_device_display_properties_khr: FNGetPhysicalDeviceDisplayPropertiesKhr,
    ///See [`FNGetPhysicalDeviceDisplayPlanePropertiesKhr`] for more information.
    pub get_physical_device_display_plane_properties_khr: FNGetPhysicalDeviceDisplayPlanePropertiesKhr,
    ///See [`FNGetDisplayPlaneSupportedDisplaysKhr`] for more information.
    pub get_display_plane_supported_displays_khr: FNGetDisplayPlaneSupportedDisplaysKhr,
    ///See [`FNGetDisplayModePropertiesKhr`] for more information.
    pub get_display_mode_properties_khr: FNGetDisplayModePropertiesKhr,
    ///See [`FNCreateDisplayModeKhr`] for more information.
    pub create_display_mode_khr: FNCreateDisplayModeKhr,
    ///See [`FNGetDisplayPlaneCapabilitiesKhr`] for more information.
    pub get_display_plane_capabilities_khr: FNGetDisplayPlaneCapabilitiesKhr,
    ///See [`FNCreateDisplayPlaneSurfaceKhr`] for more information.
    pub create_display_plane_surface_khr: FNCreateDisplayPlaneSurfaceKhr,
}
impl InstanceKhrDisplayVTable {
    ///Loads the VTable from the owner and the names
    #[track_caller]
    pub fn load(
        loader_fn: unsafe extern "system" fn(
            Instance,
            *const std::os::raw::c_char,
        ) -> Option<unsafe extern "system" fn()>,
        loader: Instance,
    ) -> Self {
        Self {
            get_physical_device_display_properties_khr: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkGetPhysicalDeviceDisplayPropertiesKHR").as_ptr(),
                ))
            },
            get_physical_device_display_plane_properties_khr: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkGetPhysicalDeviceDisplayPlanePropertiesKHR").as_ptr(),
                ))
            },
            get_display_plane_supported_displays_khr: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkGetDisplayPlaneSupportedDisplaysKHR").as_ptr(),
                ))
            },
            get_display_mode_properties_khr: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkGetDisplayModePropertiesKHR").as_ptr(),
                ))
            },
            create_display_mode_khr: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkCreateDisplayModeKHR").as_ptr()))
            },
            get_display_plane_capabilities_khr: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkGetDisplayPlaneCapabilitiesKHR").as_ptr(),
                ))
            },
            create_display_plane_surface_khr: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkCreateDisplayPlaneSurfaceKHR").as_ptr(),
                ))
            },
        }
    }
    ///Gets [`Self::get_physical_device_display_properties_khr`]. See
    /// [`FNGetPhysicalDeviceDisplayPropertiesKhr`] for more information.
    pub fn get_physical_device_display_properties_khr(&self) -> FNGetPhysicalDeviceDisplayPropertiesKhr {
        self.get_physical_device_display_properties_khr
    }
    ///Gets [`Self::get_physical_device_display_plane_properties_khr`]. See
    /// [`FNGetPhysicalDeviceDisplayPlanePropertiesKhr`] for more information.
    pub fn get_physical_device_display_plane_properties_khr(&self) -> FNGetPhysicalDeviceDisplayPlanePropertiesKhr {
        self.get_physical_device_display_plane_properties_khr
    }
    ///Gets [`Self::get_display_plane_supported_displays_khr`]. See
    /// [`FNGetDisplayPlaneSupportedDisplaysKhr`] for more information.
    pub fn get_display_plane_supported_displays_khr(&self) -> FNGetDisplayPlaneSupportedDisplaysKhr {
        self.get_display_plane_supported_displays_khr
    }
    ///Gets [`Self::get_display_mode_properties_khr`]. See [`FNGetDisplayModePropertiesKhr`] for
    /// more information.
    pub fn get_display_mode_properties_khr(&self) -> FNGetDisplayModePropertiesKhr {
        self.get_display_mode_properties_khr
    }
    ///Gets [`Self::create_display_mode_khr`]. See [`FNCreateDisplayModeKhr`] for more information.
    pub fn create_display_mode_khr(&self) -> FNCreateDisplayModeKhr {
        self.create_display_mode_khr
    }
    ///Gets [`Self::get_display_plane_capabilities_khr`]. See [`FNGetDisplayPlaneCapabilitiesKhr`]
    /// for more information.
    pub fn get_display_plane_capabilities_khr(&self) -> FNGetDisplayPlaneCapabilitiesKhr {
        self.get_display_plane_capabilities_khr
    }
    ///Gets [`Self::create_display_plane_surface_khr`]. See [`FNCreateDisplayPlaneSurfaceKhr`] for
    /// more information.
    pub fn create_display_plane_surface_khr(&self) -> FNCreateDisplayPlaneSurfaceKhr {
        self.create_display_plane_surface_khr
    }
}
