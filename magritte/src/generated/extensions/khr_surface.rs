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
//! - [`destroy_surface_khr`]
//! - [`get_physical_device_surface_capabilities_khr`]
//! - [`get_physical_device_surface_formats_khr`]
//! - [`get_physical_device_surface_present_modes_khr`]
//! - [`get_physical_device_surface_support_khr`]
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
//!section of the desktop they exist on.2) Should the
//! [`get_physical_device_surface_capabilities_khr`],
//![`get_physical_device_surface_formats_khr`], and
//![`get_physical_device_surface_present_modes_khr`] functions be in this
//!extension and operate on physical devices, rather than being in
//!`[`VK_KHR_swapchain`]` (i.e. device extension) and being dependent on
//![`Device`]? **RESOLVED** : Yes.
//!While it might be useful to depend on [`Device`] (and therefore on
//!enabled extensions and features) for the queries, Vulkan was released only
//!with the [`PhysicalDevice`] versions.
//!Many cases can be resolved by a Valid Usage statement, and/or by a separate
//!`pNext` chain version of the query struct specific to a given extension
//!or parameters, via extensible versions of the queries:
//![`get_physical_device_surface_present_modes2_ext`],
//![`get_physical_device_surface_capabilities2_khr`], and
//![`get_physical_device_surface_formats2_khr`].3) Should Vulkan support Xlib or XCB as the API for
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
//! - [`destroy_surface_khr`]
//! - [`get_physical_device_surface_capabilities_khr`]
//! - [`get_physical_device_surface_formats_khr`]
//! - [`get_physical_device_surface_present_modes_khr`]
//! - [`get_physical_device_surface_support_khr`]
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
    extensions::khr_display::SurfaceTransformFlagsKHR,
    vulkan1_0::{
        AllocationCallbacks, Bool32, Extent2D, Format, ImageUsageFlags, Instance, PhysicalDevice, VulkanResultCodes,
    },
    AsRaw, Handle, SmallVec, Unique, VulkanResult,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{
    ffi::CStr,
    iter::{Extend, FromIterator, IntoIterator},
    mem::MaybeUninit,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_SURFACE_SPEC_VERSION")]
pub const KHR_SURFACE_SPEC_VERSION: u32 = 25;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_SURFACE_EXTENSION_NAME")]
pub const KHR_SURFACE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_surface");
///[vkDestroySurfaceKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroySurfaceKHR.html) - Destroy a VkSurfaceKHR object
///# C Specifications
///To destroy a [`SurfaceKHR`] object, call:
///```c
///// Provided by VK_KHR_surface
///void vkDestroySurfaceKHR(
///    VkInstance                                  instance,
///    VkSurfaceKHR                                surface,
///    const VkAllocationCallbacks*                pAllocator);
///```
///# Parameters
/// - [`instance`] is the instance used to create the surface.
/// - [`surface`] is the surface to destroy.
/// - [`p_allocator`] is the allocator used for host memory allocated for the surface object when there is no more specific allocator available (see [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation)).
///# Description
///Destroying a [`SurfaceKHR`] merely severs the connection between Vulkan
///and the native surface, and does not imply destroying the native surface,
///closing a window, or similar behavior.
///## Valid Usage
/// - All [`SwapchainKHR`] objects created for [`surface`] **must**  have been destroyed prior to
///   destroying [`surface`]
/// - If [`AllocationCallbacks`] were provided when [`surface`] was created, a compatible set of
///   callbacks  **must**  be provided here
/// - If no [`AllocationCallbacks`] were provided when [`surface`] was created, [`p_allocator`]
///   **must**  be `NULL`
///
///## Valid Usage (Implicit)
/// - [`instance`] **must**  be a valid [`Instance`] handle
/// - If [`surface`] is not [`crate::Handle::null`], [`surface`] **must**  be a valid [`SurfaceKHR`]
///   handle
/// - If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid
///   [`AllocationCallbacks`] structure
/// - If [`surface`] is a valid handle, it  **must**  have been created, allocated, or retrieved
///   from [`instance`]
///
///## Host Synchronization
/// - Host access to [`surface`] **must**  be externally synchronized
///# Related
/// - [`VK_KHR_surface`]
/// - [`AllocationCallbacks`]
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
#[doc(alias = "vkDestroySurfaceKHR")]
pub type FNDestroySurfaceKhr = Option<
    for<'lt> unsafe extern "system" fn(
        instance: Instance,
        surface: SurfaceKHR,
        p_allocator: *const AllocationCallbacks<'lt>,
    ),
>;
///[vkGetPhysicalDeviceSurfaceSupportKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfaceSupportKHR.html) - Query if presentation is supported
///# C Specifications
///To determine whether a queue family of a physical device supports
///presentation to a given surface, call:
///```c
///// Provided by VK_KHR_surface
///VkResult vkGetPhysicalDeviceSurfaceSupportKHR(
///    VkPhysicalDevice                            physicalDevice,
///    uint32_t                                    queueFamilyIndex,
///    VkSurfaceKHR                                surface,
///    VkBool32*                                   pSupported);
///```
///# Parameters
/// - [`physical_device`] is the physical device.
/// - [`queue_family_index`] is the queue family.
/// - [`surface`] is the surface.
/// - [`p_supported`] is a pointer to a [`Bool32`], which is set to [`TRUE`] to indicate support,
///   and [`FALSE`] otherwise.
///# Description
///## Valid Usage
/// - [`queue_family_index`] **must**  be less than `pQueueFamilyPropertyCount` returned by
///   [`get_physical_device_queue_family_properties`] for the given [`physical_device`]
///
///## Valid Usage (Implicit)
/// - [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
/// - [`surface`] **must**  be a valid [`SurfaceKHR`] handle
/// - [`p_supported`] **must**  be a valid pointer to a [`Bool32`] value
/// - Both of [`physical_device`], and [`surface`] **must**  have been created, allocated, or
///   retrieved from the same [`Instance`]
///
///## Return Codes
/// * - `VK_SUCCESS`
/// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  -
///   `VK_ERROR_SURFACE_LOST_KHR`
///# Related
/// - [`VK_KHR_surface`]
/// - [`Bool32`]
/// - [`PhysicalDevice`]
/// - [`SurfaceKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkGetPhysicalDeviceSurfaceSupportKHR")]
pub type FNGetPhysicalDeviceSurfaceSupportKhr = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        surface: SurfaceKHR,
        p_supported: *mut Bool32,
    ) -> VulkanResultCodes,
>;
///[vkGetPhysicalDeviceSurfaceCapabilitiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfaceCapabilitiesKHR.html) - Query surface capabilities
///# C Specifications
///To query the basic capabilities of a surface, needed in order to create a
///swapchain, call:
///```c
///// Provided by VK_KHR_surface
///VkResult vkGetPhysicalDeviceSurfaceCapabilitiesKHR(
///    VkPhysicalDevice                            physicalDevice,
///    VkSurfaceKHR                                surface,
///    VkSurfaceCapabilitiesKHR*                   pSurfaceCapabilities);
///```
///# Parameters
/// - [`physical_device`] is the physical device that will be associated with the swapchain to be
///   created, as described for [`create_swapchain_khr`].
/// - [`surface`] is the surface that will be associated with the swapchain.
/// - [`p_surface_capabilities`] is a pointer to a [`SurfaceCapabilitiesKHR`] structure in which the
///   capabilities are returned.
///# Description
///## Valid Usage
/// - [`surface`] **must**  be a valid [`SurfaceKHR`] handle
/// - [`surface`] **must**  be supported by [`physical_device`], as reported by
///   [`get_physical_device_surface_support_khr`] or an equivalent platform-specific mechanism
///
///## Valid Usage (Implicit)
/// - [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
/// - [`surface`] **must**  be a valid [`SurfaceKHR`] handle
/// - [`p_surface_capabilities`] **must**  be a valid pointer to a [`SurfaceCapabilitiesKHR`]
///   structure
/// - Both of [`physical_device`], and [`surface`] **must**  have been created, allocated, or
///   retrieved from the same [`Instance`]
///
///## Return Codes
/// * - `VK_SUCCESS`
/// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  -
///   `VK_ERROR_SURFACE_LOST_KHR`
///# Related
/// - [`VK_KHR_surface`]
/// - [`PhysicalDevice`]
/// - [`SurfaceCapabilitiesKHR`]
/// - [`SurfaceKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkGetPhysicalDeviceSurfaceCapabilitiesKHR")]
pub type FNGetPhysicalDeviceSurfaceCapabilitiesKhr = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        surface: SurfaceKHR,
        p_surface_capabilities: *mut SurfaceCapabilitiesKHR,
    ) -> VulkanResultCodes,
>;
///[vkGetPhysicalDeviceSurfaceFormatsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfaceFormatsKHR.html) - Query color formats supported by surface
///# C Specifications
///To query the supported swapchain format-color space pairs for a surface,
///call:
///```c
///// Provided by VK_KHR_surface
///VkResult vkGetPhysicalDeviceSurfaceFormatsKHR(
///    VkPhysicalDevice                            physicalDevice,
///    VkSurfaceKHR                                surface,
///    uint32_t*                                   pSurfaceFormatCount,
///    VkSurfaceFormatKHR*                         pSurfaceFormats);
///```
///# Parameters
/// - [`physical_device`] is the physical device that will be associated with the swapchain to be
///   created, as described for [`create_swapchain_khr`].
/// - [`surface`] is the surface that will be associated with the swapchain.
/// - [`p_surface_format_count`] is a pointer to an integer related to the number of format pairs
///   available or queried, as described below.
/// - [`p_surface_formats`] is either `NULL` or a pointer to an array of [`SurfaceFormatKHR`]
///   structures.
///# Description
///If [`p_surface_formats`] is `NULL`, then the number of format pairs
///supported for the given [`surface`] is returned in
///[`p_surface_format_count`].
///Otherwise, [`p_surface_format_count`] **must**  point to a variable set by the
///user to the number of elements in the [`p_surface_formats`] array, and on
///return the variable is overwritten with the number of structures actually
///written to [`p_surface_formats`].
///If the value of [`p_surface_format_count`] is less than the number of format
///pairs supported, at most [`p_surface_format_count`] structures will be
///written, and `VK_INCOMPLETE` will be returned instead of
///`VK_SUCCESS`, to indicate that not all the available format pairs were
///returned.The number of format pairs supported  **must**  be greater than or equal to 1.
///[`p_surface_formats`] **must**  not contain an entry whose value for
///`format` is `VK_FORMAT_UNDEFINED`.If [`p_surface_formats`] includes an entry whose value for
/// `colorSpace`
///is `VK_COLOR_SPACE_SRGB_NONLINEAR_KHR` and whose value for `format`
///is a UNORM (or SRGB) format and the corresponding SRGB (or UNORM) format is
///a color renderable format for `VK_IMAGE_TILING_OPTIMAL`, then
///[`p_surface_formats`] **must**  also contain an entry with the same value for
///`colorSpace` and `format` equal to the corresponding SRGB (or UNORM)
///format.If the `[`VK_GOOGLE_surfaceless_query`]` extension is enabled, the values
///returned in [`p_surface_formats`] will be identical for every valid surface
///created on this physical device, and so [`surface`] **can**  be
///[`crate::Handle::null`].
///## Valid Usage
/// - If the `[`VK_GOOGLE_surfaceless_query`]` extension is not enabled, [`surface`] **must**  be a
///   valid [`SurfaceKHR`] handle
/// - If [`surface`] is not [`crate::Handle::null`], it  **must**  be supported by
///   [`physical_device`], as reported by [`get_physical_device_surface_support_khr`] or an
///   equivalent platform-specific mechanism
///
///## Valid Usage (Implicit)
/// - [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
/// - If [`surface`] is not [`crate::Handle::null`], [`surface`] **must**  be a valid [`SurfaceKHR`]
///   handle
/// - [`p_surface_format_count`] **must**  be a valid pointer to a `uint32_t` value
/// - If the value referenced by [`p_surface_format_count`] is not `0`, and [`p_surface_formats`] is
///   not `NULL`, [`p_surface_formats`] **must**  be a valid pointer to an array of
///   [`p_surface_format_count`][`SurfaceFormatKHR`] structures
/// - Both of [`physical_device`], and [`surface`] that are valid handles of non-ignored parameters
///   **must**  have been created, allocated, or retrieved from the same [`Instance`]
///
///## Return Codes
/// * - `VK_SUCCESS`  - `VK_INCOMPLETE`
/// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  -
///   `VK_ERROR_SURFACE_LOST_KHR`
///# Related
/// - [`VK_KHR_surface`]
/// - [`PhysicalDevice`]
/// - [`SurfaceFormatKHR`]
/// - [`SurfaceKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkGetPhysicalDeviceSurfaceFormatsKHR")]
pub type FNGetPhysicalDeviceSurfaceFormatsKhr = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        surface: SurfaceKHR,
        p_surface_format_count: *mut u32,
        p_surface_formats: *mut SurfaceFormatKHR,
    ) -> VulkanResultCodes,
>;
///[vkGetPhysicalDeviceSurfacePresentModesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfacePresentModesKHR.html) - Query supported presentation modes
///# C Specifications
///To query the supported presentation modes for a surface, call:
///```c
///// Provided by VK_KHR_surface
///VkResult vkGetPhysicalDeviceSurfacePresentModesKHR(
///    VkPhysicalDevice                            physicalDevice,
///    VkSurfaceKHR                                surface,
///    uint32_t*                                   pPresentModeCount,
///    VkPresentModeKHR*                           pPresentModes);
///```
///# Parameters
/// - [`physical_device`] is the physical device that will be associated with the swapchain to be
///   created, as described for [`create_swapchain_khr`].
/// - [`surface`] is the surface that will be associated with the swapchain.
/// - [`p_present_mode_count`] is a pointer to an integer related to the number of presentation
///   modes available or queried, as described below.
/// - [`p_present_modes`] is either `NULL` or a pointer to an array of [`PresentModeKHR`] values,
///   indicating the supported presentation modes.
///# Description
///If [`p_present_modes`] is `NULL`, then the number of presentation modes
///supported for the given [`surface`] is returned in
///[`p_present_mode_count`].
///Otherwise, [`p_present_mode_count`] **must**  point to a variable set by the user
///to the number of elements in the [`p_present_modes`] array, and on return
///the variable is overwritten with the number of values actually written to
///[`p_present_modes`].
///If the value of [`p_present_mode_count`] is less than the number of
///presentation modes supported, at most [`p_present_mode_count`] values will be
///written, and `VK_INCOMPLETE` will be returned instead of
///`VK_SUCCESS`, to indicate that not all the available modes were
///returned.If the `[`VK_GOOGLE_surfaceless_query`]` extension is enabled, the values
///returned in [`p_present_modes`] will be identical for every valid surface
///created on this physical device, and so [`surface`] **can**  be
///[`crate::Handle::null`].
///## Valid Usage
/// - If the `[`VK_GOOGLE_surfaceless_query`]` extension is not enabled, [`surface`] **must**  be a
///   valid [`SurfaceKHR`] handle
/// - If [`surface`] is not [`crate::Handle::null`], it  **must**  be supported by
///   [`physical_device`], as reported by [`get_physical_device_surface_support_khr`] or an
///   equivalent platform-specific mechanism
///
///## Valid Usage (Implicit)
/// - [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
/// - If [`surface`] is not [`crate::Handle::null`], [`surface`] **must**  be a valid [`SurfaceKHR`]
///   handle
/// - [`p_present_mode_count`] **must**  be a valid pointer to a `uint32_t` value
/// - If the value referenced by [`p_present_mode_count`] is not `0`, and [`p_present_modes`] is not
///   `NULL`, [`p_present_modes`] **must**  be a valid pointer to an array of
///   [`p_present_mode_count`][`PresentModeKHR`] values
/// - Both of [`physical_device`], and [`surface`] that are valid handles of non-ignored parameters
///   **must**  have been created, allocated, or retrieved from the same [`Instance`]
///
///## Return Codes
/// * - `VK_SUCCESS`  - `VK_INCOMPLETE`
/// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  -
///   `VK_ERROR_SURFACE_LOST_KHR`
///# Related
/// - [`VK_KHR_surface`]
/// - [`PhysicalDevice`]
/// - [`PresentModeKHR`]
/// - [`SurfaceKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkGetPhysicalDeviceSurfacePresentModesKHR")]
pub type FNGetPhysicalDeviceSurfacePresentModesKhr = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        surface: SurfaceKHR,
        p_present_mode_count: *mut u32,
        p_present_modes: *mut PresentModeKHR,
    ) -> VulkanResultCodes,
>;
///[VkPresentModeKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPresentModeKHR.html) - Presentation mode supported for a surface
///# C Specifications
///Possible values of elements of the
///[`get_physical_device_surface_present_modes_khr`]`::pPresentModes` array,
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
/// - [`IMMEDIATE`] specifies that the presentation engine does not wait for a vertical blanking
///   period to update the current image, meaning this mode  **may**  result in visible tearing. No
///   internal queuing of presentation requests is needed, as the requests are applied immediately.
/// - [`MAILBOX`] specifies that the presentation engine waits for the next vertical blanking period
///   to update the current image. Tearing  **cannot**  be observed. An internal single-entry queue
///   is used to hold pending presentation requests. If the queue is full when a new presentation
///   request is received, the new request replaces the existing entry, and any images associated
///   with the prior entry become available for re-use by the application. One request is removed
///   from the queue and processed during each vertical blanking period in which the queue is
///   non-empty.
/// - [`FIFO`] specifies that the presentation engine waits for the next vertical blanking period to
///   update the current image. Tearing  **cannot**  be observed. An internal queue is used to hold
///   pending presentation requests. New requests are appended to the end of the queue, and one
///   request is removed from the beginning of the queue and processed during each vertical blanking
///   period in which the queue is non-empty. This is the only value of `presentMode` that is
///   **required**  to be supported.
/// - [`FIFO_RELAXED`] specifies that the presentation engine generally waits for the next vertical
///   blanking period to update the current image. If a vertical blanking period has already passed
///   since the last update of the current image then the presentation engine does not wait for
///   another vertical blanking period for the update, meaning this mode  **may**  result in visible
///   tearing in this case. This mode is useful for reducing visual stutter with an application that
///   will mostly present a new image before the next vertical blanking period, but may occasionally
///   be late, and present a new image just after the next vertical blanking period. An internal
///   queue is used to hold pending presentation requests. New requests are appended to the end of
///   the queue, and one request is removed from the beginning of the queue and processed during or
///   after each vertical blanking period in which the queue is non-empty.
/// - [`SHARED_DEMAND_REFRESH`] specifies that the presentation engine and application have
///   concurrent access to a single image, which is referred to as a *shared presentable image*. The
///   presentation engine is only required to update the current image after a new presentation
///   request is received. Therefore the application  **must**  make a presentation request whenever
///   an update is required. However, the presentation engine  **may**  update the current image at
///   any point, meaning this mode  **may**  result in visible tearing.
/// - [`SHARED_CONTINUOUS_REFRESH`] specifies that the presentation engine and application have
///   concurrent access to a single image, which is referred to as a *shared presentable image*. The
///   presentation engine periodically updates the current image on its regular refresh cycle. The
///   application is only required to make one initial presentation request, after which the
///   presentation engine  **must**  update the current image without any need for further
///   presentation requests. The application  **can**  indicate the image contents have been updated
///   by making a presentation request, but this does not guarantee the timing of when it will be
///   updated. This mode  **may**  result in visible tearing if rendering to the image is not timed
///   correctly.
///The supported [`ImageUsageFlagBits`] of the presentable images of a
///swapchain created for a surface  **may**  differ depending on the presentation
///mode, and can be determined as per the table below:
///# Related
/// - [`VK_KHR_surface`]
/// - [`SwapchainCreateInfoKHR`]
/// - [`get_physical_device_surface_present_modes2_ext`]
/// - [`get_physical_device_surface_present_modes_khr`]
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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct PresentModeKHR(i32);
impl const Default for PresentModeKHR {
    fn default() -> Self {
        Self(0)
    }
}
impl PresentModeKHR {
    ///[`IMMEDIATE`] specifies that the presentation
    ///engine does not wait for a vertical blanking period to update the
    ///current image, meaning this mode  **may**  result in visible tearing.
    ///No internal queuing of presentation requests is needed, as the requests
    ///are applied immediately.
    pub const IMMEDIATE: Self = Self(0);
    ///[`MAILBOX`] specifies that the presentation engine
    ///waits for the next vertical blanking period to update the current image.
    ///Tearing  **cannot**  be observed.
    ///An internal single-entry queue is used to hold pending presentation
    ///requests.
    ///If the queue is full when a new presentation request is received, the
    ///new request replaces the existing entry, and any images associated with
    ///the prior entry become available for re-use by the application.
    ///One request is removed from the queue and processed during each vertical
    ///blanking period in which the queue is non-empty.
    pub const MAILBOX: Self = Self(1);
    ///[`FIFO`] specifies that the presentation engine
    ///waits for the next vertical blanking period to update the current image.
    ///Tearing  **cannot**  be observed.
    ///An internal queue is used to hold pending presentation requests.
    ///New requests are appended to the end of the queue, and one request is
    ///removed from the beginning of the queue and processed during each
    ///vertical blanking period in which the queue is non-empty.
    ///This is the only value of `presentMode` that is  **required**  to be
    ///supported.
    pub const FIFO: Self = Self(2);
    ///[`FIFO_RELAXED`] specifies that the presentation
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
    pub const FIFO_RELAXED: Self = Self(3);
    ///[`SHARED_DEMAND_REFRESH`] specifies that the
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
    pub const SHARED_DEMAND_REFRESH: Self = Self(1000111000);
    ///[`SHARED_CONTINUOUS_REFRESH`] specifies that the
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
    pub const SHARED_CONTINUOUS_REFRESH: Self = Self(1000111001);
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
    ///Gets a value from a raw underlying value, unchecked and therefore unsafe.
    ///
    ///# Safety
    ///The caller of this function must ensure that all of the bits are valid.
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: i32) -> Self {
        Self(bits)
    }
}
impl std::fmt::Debug for PresentModeKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple(stringify!(PresentModeKHR))
            .field(match *self {
                Self::IMMEDIATE => &"IMMEDIATE",
                Self::MAILBOX => &"MAILBOX",
                Self::FIFO => &"FIFO",
                Self::FIFO_RELAXED => &"FIFO_RELAXED",
                #[cfg(feature = "VK_KHR_shared_presentable_image")]
                Self::SHARED_DEMAND_REFRESH => &"SHARED_DEMAND_REFRESH",
                #[cfg(feature = "VK_KHR_shared_presentable_image")]
                Self::SHARED_CONTINUOUS_REFRESH => &"SHARED_CONTINUOUS_REFRESH",
                other => unreachable!(
                    concat!("invalid value for", stringify!(PresentModeKHR), ": {:?}"),
                    other
                ),
            })
            .finish()
    }
}
impl std::fmt::Display for PresentModeKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str(match *self {
            Self::IMMEDIATE => &"IMMEDIATE",
            Self::MAILBOX => &"MAILBOX",
            Self::FIFO => &"FIFO",
            Self::FIFO_RELAXED => &"FIFO_RELAXED",
            #[cfg(feature = "VK_KHR_shared_presentable_image")]
            Self::SHARED_DEMAND_REFRESH => &"SHARED_DEMAND_REFRESH",
            #[cfg(feature = "VK_KHR_shared_presentable_image")]
            Self::SHARED_CONTINUOUS_REFRESH => &"SHARED_CONTINUOUS_REFRESH",
            other => unreachable!(
                concat!("invalid value for", stringify!(PresentModeKHR), ": {:?}"),
                other
            ),
        })
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
/// - [`SRGB_NONLINEAR`] specifies support for the sRGB color space.
/// - [`DISPLAY_P3_NONLINEAR_EXT`] specifies support for the Display-P3 color space to be displayed
///   using an sRGB-like EOTF (defined below).
/// - [`EXTENDED_SRGB_LINEAR_EXT`] specifies support for the extended sRGB color space to be
///   displayed using a linear EOTF.
/// - [`EXTENDED_SRGB_NONLINEAR_EXT`] specifies support for the extended sRGB color space to be
///   displayed using an sRGB EOTF.
/// - [`DISPLAY_P3_LINEAR_EXT`] specifies support for the Display-P3 color space to be displayed
///   using a linear EOTF.
/// - [`DCI_P3_NONLINEAR_EXT`] specifies support for the DCI-P3 color space to be displayed using
///   the DCI-P3 EOTF. Note that values in such an image are interpreted as XYZ encoded color data
///   by the presentation engine.
/// - [`BT709_LINEAR_EXT`] specifies support for the BT709 color space to be displayed using a
///   linear EOTF.
/// - [`BT709_NONLINEAR_EXT`] specifies support for the BT709 color space to be displayed using the
///   SMPTE 170M EOTF.
/// - [`BT2020_LINEAR_EXT`] specifies support for the BT2020 color space to be displayed using a
///   linear EOTF.
/// - [`HDR10_ST2084_EXT`] specifies support for the HDR10 (BT2020 color) space to be displayed
///   using the SMPTE ST2084 Perceptual Quantizer (PQ) EOTF.
/// - [`DOLBYVISION_EXT`] specifies support for the Dolby Vision (BT2020 color space), proprietary
///   encoding, to be displayed using the SMPTE ST2084 EOTF.
/// - [`HDR10_HLG_EXT`] specifies support for the HDR10 (BT2020 color space) to be displayed using
///   the Hybrid Log Gamma (HLG) EOTF.
/// - [`ADOBERGB_LINEAR_EXT`] specifies support for the AdobeRGB color space to be displayed using a
///   linear EOTF.
/// - [`ADOBERGB_NONLINEAR_EXT`] specifies support for the AdobeRGB color space to be displayed
///   using the Gamma 2.2 EOTF.
/// - [`PASS_THROUGH_EXT`] specifies that color components are used “as is”. This is intended to
///   allow applications to supply data for color spaces not described here.
/// - [`DISPLAY_NATIVE_AMD`] specifies support for the display’s native color space. This matches
///   the color space expectations of AMD’s FreeSync2 standard, for displays supporting it.
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
///of the [Khronos Data Format Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#data-format).Except Display-P3 OETF, which is:<span class="katex"><span class="katex-html" aria-hidden="true"><span class="base"><span style="height:3.30003em;vertical-align:-1.400015em;" class="strut"></span><span class="mord"><span class="mtable"><span class="col-align-r"><span class="vlist-t vlist-t2"><span class="vlist-r"><span class="vlist" style="height:1.900015em;"><span style="top:-3.9000150000000002em;"><span style="height:3.75em;" class="pstrut"></span><span class="mord"><span class="mord mathdefault" style="margin-right:0.05764em;">E</span></span></span></span><span class="vlist-s">​</span></span><span class="vlist-r"><span class="vlist" style="height:1.400015em;"><span></span></span></span></span></span><span class="col-align-l"><span class="vlist-t vlist-t2"><span class="vlist-r"><span class="vlist" style="height:1.900015em;"><span style="top:-3.9000150000000002em;"><span style="height:3.75em;" class="pstrut"></span><span class="mord"><span class="mord"></span><span class="mspace" style="margin-right:0.2777777777777778em;"></span><span class="mrel">=</span><span style="margin-right:0.2777777777777778em;" class="mspace"></span><span class="minner"><span class="mopen delimcenter" style="top:0em;"><span class="delimsizing size4">{</span></span><span class="mord"><span class="mtable"><span class="col-align-l"><span class="vlist-t vlist-t2"><span class="vlist-r"><span style="height:1.69em;" class="vlist"><span style="top:-3.69em;"><span class="pstrut" style="height:3.008em;"></span><span class="mord"><span class="mord">1</span><span class="mord">.</span><span class="mord">0</span><span class="mord">5</span><span class="mord">5</span><span class="mspace" style="margin-right:0.2222222222222222em;"></span><span class="mbin">×</span><span class="mspace" style="margin-right:0.2222222222222222em;"></span><span class="mord"><span class="mord mathdefault">L</span><span class="msupsub"><span class="vlist-t"><span class="vlist-r"><span style="height:0.9540200000000001em;" class="vlist"><span style="top:-3.363em;margin-right:0.05em;"><span style="height:3em;" class="pstrut"></span><span class="sizing reset-size6 size3 mtight"><span class="mord mtight"><span class="mord mtight"><span class="mopen nulldelimiter sizing reset-size3 size6"></span><span class="mfrac"><span class="vlist-t vlist-t2"><span class="vlist-r"><span class="vlist" style="height:0.8443142857142858em;"><span style="top:-2.656em;"><span class="pstrut" style="height:3em;"></span><span class="sizing reset-size3 size1 mtight"><span class="mord mtight"><span class="mord mtight">2</span><span class="mord mtight">.</span><span class="mord mtight">4</span></span></span></span><span style="top:-3.2255000000000003em;"><span class="pstrut" style="height:3em;"></span><span style="border-bottom-width:0.049em;" class="frac-line mtight"></span></span><span style="top:-3.384em;"><span class="pstrut" style="height:3em;"></span><span class="sizing reset-size3 size1 mtight"><span class="mord mtight"><span class="mord mtight">1</span></span></span></span></span><span class="vlist-s">​</span></span><span class="vlist-r"><span style="height:0.344em;" class="vlist"><span></span></span></span></span></span><span class="mclose nulldelimiter sizing reset-size3 size6"></span></span></span></span></span></span></span></span></span></span><span style="margin-right:0.2222222222222222em;" class="mspace"></span><span class="mbin">−</span><span class="mspace" style="margin-right:0.2222222222222222em;"></span><span class="mord">0</span><span class="mord">.</span><span class="mord">0</span><span class="mord">5</span><span class="mord">5</span></span></span><span style="top:-2.25em;"><span class="pstrut" style="height:3.008em;"></span><span class="mord"><span class="mord">1</span><span class="mord">2</span><span class="mord">.</span><span class="mord">9</span><span class="mord">2</span><span class="mspace" style="margin-right:0.2222222222222222em;"></span><span class="mbin">×</span><span class="mspace" style="margin-right:0.2222222222222222em;"></span><span class="mord mathdefault">L</span></span></span></span><span class="vlist-s">​</span></span><span class="vlist-r"><span style="height:1.19em;" class="vlist"><span></span></span></span></span></span><span style="width:1em;" class="arraycolsep"></span><span class="col-align-l"><span class="vlist-t vlist-t2"><span class="vlist-r"><span class="vlist" style="height:1.69em;"><span style="top:-3.69em;"><span style="height:3.008em;" class="pstrut"></span><span class="mord"><span class="mord text"><span class="mord">for</span></span><span class="mspace">&nbsp;</span><span class="mord">0</span><span class="mord">.</span><span class="mord">0</span><span class="mord">0</span><span class="mord">3</span><span class="mord">0</span><span class="mord">1</span><span class="mord">8</span><span class="mord">6</span><span class="mspace" style="margin-right:0.2777777777777778em;"></span><span class="mrel">≤</span><span style="margin-right:0.2777777777777778em;" class="mspace"></span><span class="mord mathdefault">L</span><span class="mspace" style="margin-right:0.2777777777777778em;"></span><span class="mrel">≤</span><span class="mspace" style="margin-right:0.2777777777777778em;"></span><span class="mord">1</span></span></span><span style="top:-2.25em;"><span class="pstrut" style="height:3.008em;"></span><span class="mord"><span class="mord text"><span class="mord">for</span></span><span class="mspace">&nbsp;</span><span class="mord">0</span><span style="margin-right:0.2777777777777778em;" class="mspace"></span><span class="mrel">≤</span><span class="mspace" style="margin-right:0.2777777777777778em;"></span><span class="mord mathdefault">L</span><span class="mspace" style="margin-right:0.2777777777777778em;"></span><span class="mrel">&lt;</span><span class="mspace" style="margin-right:0.2777777777777778em;"></span><span class="mord">0</span><span class="mord">.</span><span class="mord">0</span><span class="mord">0</span><span class="mord">3</span><span class="mord">0</span><span class="mord">1</span><span class="mord">8</span><span class="mord">6</span></span></span></span><span class="vlist-s">​</span></span><span class="vlist-r"><span class="vlist" style="height:1.19em;"><span></span></span></span></span></span></span></span><span class="mclose nulldelimiter"></span></span></span></span></span><span class="vlist-s">​</span></span><span class="vlist-r"><span class="vlist" style="height:1.400015em;"><span></span></span></span></span></span></span></span></span></span></span>where L is the linear value of a color component and E is the
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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct ColorSpaceKHR(i32);
impl const Default for ColorSpaceKHR {
    fn default() -> Self {
        Self(0)
    }
}
impl ColorSpaceKHR {
    ///[`SRGB_NONLINEAR`] specifies support for the sRGB
    ///color space.
    pub const SRGB_NONLINEAR: Self = Self(0);
    ///[`DISPLAY_P3_NONLINEAR_EXT`] specifies support for the
    ///Display-P3 color space to be displayed using an sRGB-like EOTF (defined
    ///below).
    ///
    ///Provided by [`crate::extensions::ext_swapchain_colorspace`]
    #[cfg(feature = "VK_EXT_swapchain_colorspace")]
    pub const DISPLAY_P3_NONLINEAR_EXT: Self = Self(1000104001);
    ///[`EXTENDED_SRGB_LINEAR_EXT`] specifies support for the
    ///extended sRGB color space to be displayed using a linear EOTF.
    ///
    ///Provided by [`crate::extensions::ext_swapchain_colorspace`]
    #[cfg(feature = "VK_EXT_swapchain_colorspace")]
    pub const EXTENDED_SRGB_LINEAR_EXT: Self = Self(1000104002);
    ///[`DISPLAY_P3_LINEAR_EXT`] specifies support for the
    ///Display-P3 color space to be displayed using a linear EOTF.
    ///
    ///Provided by [`crate::extensions::ext_swapchain_colorspace`]
    #[cfg(feature = "VK_EXT_swapchain_colorspace")]
    pub const DISPLAY_P3_LINEAR_EXT: Self = Self(1000104003);
    ///[`DCI_P3_NONLINEAR_EXT`] specifies support for the
    ///DCI-P3 color space to be displayed using the DCI-P3 EOTF.
    ///Note that values in such an image are interpreted as XYZ encoded color
    ///data by the presentation engine.
    ///
    ///Provided by [`crate::extensions::ext_swapchain_colorspace`]
    #[cfg(feature = "VK_EXT_swapchain_colorspace")]
    pub const DCI_P3_NONLINEAR_EXT: Self = Self(1000104004);
    ///[`BT709_LINEAR_EXT`] specifies support for the BT709
    ///color space to be displayed using a linear EOTF.
    ///
    ///Provided by [`crate::extensions::ext_swapchain_colorspace`]
    #[cfg(feature = "VK_EXT_swapchain_colorspace")]
    pub const BT709_LINEAR_EXT: Self = Self(1000104005);
    ///[`BT709_NONLINEAR_EXT`] specifies support for the BT709
    ///color space to be displayed using the SMPTE 170M EOTF.
    ///
    ///Provided by [`crate::extensions::ext_swapchain_colorspace`]
    #[cfg(feature = "VK_EXT_swapchain_colorspace")]
    pub const BT709_NONLINEAR_EXT: Self = Self(1000104006);
    ///[`BT2020_LINEAR_EXT`] specifies support for the BT2020
    ///color space to be displayed using a linear EOTF.
    ///
    ///Provided by [`crate::extensions::ext_swapchain_colorspace`]
    #[cfg(feature = "VK_EXT_swapchain_colorspace")]
    pub const BT2020_LINEAR_EXT: Self = Self(1000104007);
    ///[`HDR10_ST2084_EXT`] specifies support for the HDR10
    ///(BT2020 color) space to be displayed using the SMPTE ST2084 Perceptual
    ///Quantizer (PQ) EOTF.
    ///
    ///Provided by [`crate::extensions::ext_swapchain_colorspace`]
    #[cfg(feature = "VK_EXT_swapchain_colorspace")]
    pub const HDR10_ST2084_EXT: Self = Self(1000104008);
    ///[`DOLBYVISION_EXT`] specifies support for the Dolby
    ///Vision (BT2020 color space), proprietary encoding, to be displayed using
    ///the SMPTE ST2084 EOTF.
    ///
    ///Provided by [`crate::extensions::ext_swapchain_colorspace`]
    #[cfg(feature = "VK_EXT_swapchain_colorspace")]
    pub const DOLBYVISION_EXT: Self = Self(1000104009);
    ///[`HDR10_HLG_EXT`] specifies support for the HDR10
    ///(BT2020 color space) to be displayed using the Hybrid Log Gamma (HLG)
    ///EOTF.
    ///
    ///Provided by [`crate::extensions::ext_swapchain_colorspace`]
    #[cfg(feature = "VK_EXT_swapchain_colorspace")]
    pub const HDR10_HLG_EXT: Self = Self(1000104010);
    ///[`ADOBERGB_LINEAR_EXT`] specifies support for the
    ///AdobeRGB color space to be displayed using a linear EOTF.
    ///
    ///Provided by [`crate::extensions::ext_swapchain_colorspace`]
    #[cfg(feature = "VK_EXT_swapchain_colorspace")]
    pub const ADOBERGB_LINEAR_EXT: Self = Self(1000104011);
    ///[`ADOBERGB_NONLINEAR_EXT`] specifies support for the
    ///AdobeRGB color space to be displayed using the Gamma 2.2 EOTF.
    ///
    ///Provided by [`crate::extensions::ext_swapchain_colorspace`]
    #[cfg(feature = "VK_EXT_swapchain_colorspace")]
    pub const ADOBERGB_NONLINEAR_EXT: Self = Self(1000104012);
    ///[`PASS_THROUGH_EXT`] specifies that color components
    ///are used “as is”.
    ///This is intended to allow applications to supply data for color spaces
    ///not described here.
    ///
    ///Provided by [`crate::extensions::ext_swapchain_colorspace`]
    #[cfg(feature = "VK_EXT_swapchain_colorspace")]
    pub const PASS_THROUGH_EXT: Self = Self(1000104013);
    ///[`EXTENDED_SRGB_NONLINEAR_EXT`] specifies support for
    ///the extended sRGB color space to be displayed using an sRGB EOTF.
    ///
    ///Provided by [`crate::extensions::ext_swapchain_colorspace`]
    #[cfg(feature = "VK_EXT_swapchain_colorspace")]
    pub const EXTENDED_SRGB_NONLINEAR_EXT: Self = Self(1000104014);
    ///[`DISPLAY_NATIVE_AMD`] specifies support for the
    ///display’s native color space.
    ///This matches the color space expectations of AMD’s FreeSync2 standard,
    ///for displays supporting it.
    ///
    ///Provided by [`crate::extensions::amd_display_native_hdr`]
    #[cfg(feature = "VK_AMD_display_native_hdr")]
    pub const DISPLAY_NATIVE_AMD: Self = Self(1000213000);
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
    ///Gets a value from a raw underlying value, unchecked and therefore unsafe.
    ///
    ///# Safety
    ///The caller of this function must ensure that all of the bits are valid.
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: i32) -> Self {
        Self(bits)
    }
}
impl std::fmt::Debug for ColorSpaceKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple(stringify!(ColorSpaceKHR))
            .field(match *self {
                Self::SRGB_NONLINEAR => &"SRGB_NONLINEAR",
                #[cfg(feature = "VK_EXT_swapchain_colorspace")]
                Self::DISPLAY_P3_NONLINEAR_EXT => &"DISPLAY_P3_NONLINEAR_EXT",
                #[cfg(feature = "VK_EXT_swapchain_colorspace")]
                Self::EXTENDED_SRGB_LINEAR_EXT => &"EXTENDED_SRGB_LINEAR_EXT",
                #[cfg(feature = "VK_EXT_swapchain_colorspace")]
                Self::DISPLAY_P3_LINEAR_EXT => &"DISPLAY_P3_LINEAR_EXT",
                #[cfg(feature = "VK_EXT_swapchain_colorspace")]
                Self::DCI_P3_NONLINEAR_EXT => &"DCI_P3_NONLINEAR_EXT",
                #[cfg(feature = "VK_EXT_swapchain_colorspace")]
                Self::BT709_LINEAR_EXT => &"BT709_LINEAR_EXT",
                #[cfg(feature = "VK_EXT_swapchain_colorspace")]
                Self::BT709_NONLINEAR_EXT => &"BT709_NONLINEAR_EXT",
                #[cfg(feature = "VK_EXT_swapchain_colorspace")]
                Self::BT2020_LINEAR_EXT => &"BT2020_LINEAR_EXT",
                #[cfg(feature = "VK_EXT_swapchain_colorspace")]
                Self::HDR10_ST2084_EXT => &"HDR10_ST2084_EXT",
                #[cfg(feature = "VK_EXT_swapchain_colorspace")]
                Self::DOLBYVISION_EXT => &"DOLBYVISION_EXT",
                #[cfg(feature = "VK_EXT_swapchain_colorspace")]
                Self::HDR10_HLG_EXT => &"HDR10_HLG_EXT",
                #[cfg(feature = "VK_EXT_swapchain_colorspace")]
                Self::ADOBERGB_LINEAR_EXT => &"ADOBERGB_LINEAR_EXT",
                #[cfg(feature = "VK_EXT_swapchain_colorspace")]
                Self::ADOBERGB_NONLINEAR_EXT => &"ADOBERGB_NONLINEAR_EXT",
                #[cfg(feature = "VK_EXT_swapchain_colorspace")]
                Self::PASS_THROUGH_EXT => &"PASS_THROUGH_EXT",
                #[cfg(feature = "VK_EXT_swapchain_colorspace")]
                Self::EXTENDED_SRGB_NONLINEAR_EXT => &"EXTENDED_SRGB_NONLINEAR_EXT",
                #[cfg(feature = "VK_AMD_display_native_hdr")]
                Self::DISPLAY_NATIVE_AMD => &"DISPLAY_NATIVE_AMD",
                other => unreachable!(concat!("invalid value for", stringify!(ColorSpaceKHR), ": {:?}"), other),
            })
            .finish()
    }
}
impl std::fmt::Display for ColorSpaceKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str(match *self {
            Self::SRGB_NONLINEAR => &"SRGB_NONLINEAR",
            #[cfg(feature = "VK_EXT_swapchain_colorspace")]
            Self::DISPLAY_P3_NONLINEAR_EXT => &"DISPLAY_P3_NONLINEAR_EXT",
            #[cfg(feature = "VK_EXT_swapchain_colorspace")]
            Self::EXTENDED_SRGB_LINEAR_EXT => &"EXTENDED_SRGB_LINEAR_EXT",
            #[cfg(feature = "VK_EXT_swapchain_colorspace")]
            Self::DISPLAY_P3_LINEAR_EXT => &"DISPLAY_P3_LINEAR_EXT",
            #[cfg(feature = "VK_EXT_swapchain_colorspace")]
            Self::DCI_P3_NONLINEAR_EXT => &"DCI_P3_NONLINEAR_EXT",
            #[cfg(feature = "VK_EXT_swapchain_colorspace")]
            Self::BT709_LINEAR_EXT => &"BT709_LINEAR_EXT",
            #[cfg(feature = "VK_EXT_swapchain_colorspace")]
            Self::BT709_NONLINEAR_EXT => &"BT709_NONLINEAR_EXT",
            #[cfg(feature = "VK_EXT_swapchain_colorspace")]
            Self::BT2020_LINEAR_EXT => &"BT2020_LINEAR_EXT",
            #[cfg(feature = "VK_EXT_swapchain_colorspace")]
            Self::HDR10_ST2084_EXT => &"HDR10_ST2084_EXT",
            #[cfg(feature = "VK_EXT_swapchain_colorspace")]
            Self::DOLBYVISION_EXT => &"DOLBYVISION_EXT",
            #[cfg(feature = "VK_EXT_swapchain_colorspace")]
            Self::HDR10_HLG_EXT => &"HDR10_HLG_EXT",
            #[cfg(feature = "VK_EXT_swapchain_colorspace")]
            Self::ADOBERGB_LINEAR_EXT => &"ADOBERGB_LINEAR_EXT",
            #[cfg(feature = "VK_EXT_swapchain_colorspace")]
            Self::ADOBERGB_NONLINEAR_EXT => &"ADOBERGB_NONLINEAR_EXT",
            #[cfg(feature = "VK_EXT_swapchain_colorspace")]
            Self::PASS_THROUGH_EXT => &"PASS_THROUGH_EXT",
            #[cfg(feature = "VK_EXT_swapchain_colorspace")]
            Self::EXTENDED_SRGB_NONLINEAR_EXT => &"EXTENDED_SRGB_NONLINEAR_EXT",
            #[cfg(feature = "VK_AMD_display_native_hdr")]
            Self::DISPLAY_NATIVE_AMD => &"DISPLAY_NATIVE_AMD",
            other => unreachable!(concat!("invalid value for", stringify!(ColorSpaceKHR), ": {:?}"), other),
        })
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
/// - [`OPAQUE`]: The alpha component, if it exists, of the images is ignored in the compositing
///   process. Instead, the image is treated as if it has a constant alpha of 1.0.
/// - [`PRE_MULTIPLIED`]: The alpha component, if it exists, of the images is respected in the
///   compositing process. The non-alpha components of the image are expected to already be
///   multiplied by the alpha component by the application.
/// - [`POST_MULTIPLIED`]: The alpha component, if it exists, of the images is respected in the
///   compositing process. The non-alpha components of the image are not expected to already be
///   multiplied by the alpha component by the application; instead, the compositor will multiply
///   the non-alpha components of the image by the alpha component during compositing.
/// - [`INHERIT`]: The way in which the presentation engine treats the alpha component in the images
///   is unknown to the Vulkan API. Instead, the application is responsible for setting the
///   composite alpha blending mode using native window system commands. If the application does not
///   set the blending mode using native window system commands, then a platform-specific default
///   will be used.
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
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct CompositeAlphaFlagBitsKHR(u32);
impl const Default for CompositeAlphaFlagBitsKHR {
    fn default() -> Self {
        Self(0)
    }
}
impl CompositeAlphaFlagBitsKHR {
    ///[`OPAQUE`]: The alpha component, if it
    ///exists, of the images is ignored in the compositing process.
    ///Instead, the image is treated as if it has a constant alpha of 1.0.
    pub const OPAQUE: Self = Self(1);
    ///[`PRE_MULTIPLIED`]: The alpha component, if
    ///it exists, of the images is respected in the compositing process.
    ///The non-alpha components of the image are expected to already be
    ///multiplied by the alpha component by the application.
    pub const PRE_MULTIPLIED: Self = Self(2);
    ///[`POST_MULTIPLIED`]: The alpha component,
    ///if it exists, of the images is respected in the compositing process.
    ///The non-alpha components of the image are not expected to already be
    ///multiplied by the alpha component by the application; instead, the
    ///compositor will multiply the non-alpha components of the image by the
    ///alpha component during compositing.
    pub const POST_MULTIPLIED: Self = Self(4);
    ///[`INHERIT`]: The way in which the
    ///presentation engine treats the alpha component in the images is unknown
    ///to the Vulkan API.
    ///Instead, the application is responsible for setting the composite alpha
    ///blending mode using native window system commands.
    ///If the application does not set the blending mode using native window
    ///system commands, then a platform-specific default will be used.
    pub const INHERIT: Self = Self(8);
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0
    }
    ///Gets a value from a raw underlying value, unchecked and therefore unsafe.
    ///
    ///# Safety
    ///The caller of this function must ensure that all of the bits are valid.
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
}
impl std::fmt::Debug for CompositeAlphaFlagBitsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple(stringify!(CompositeAlphaFlagBitsKHR))
            .field(match *self {
                Self::OPAQUE => &"OPAQUE",
                Self::PRE_MULTIPLIED => &"PRE_MULTIPLIED",
                Self::POST_MULTIPLIED => &"POST_MULTIPLIED",
                Self::INHERIT => &"INHERIT",
                other => unreachable!(
                    concat!("invalid value for", stringify!(CompositeAlphaFlagBitsKHR), ": {:?}"),
                    other
                ),
            })
            .finish()
    }
}
impl std::fmt::Display for CompositeAlphaFlagBitsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str(match *self {
            Self::OPAQUE => &"OPAQUE",
            Self::PRE_MULTIPLIED => &"PRE_MULTIPLIED",
            Self::POST_MULTIPLIED => &"POST_MULTIPLIED",
            Self::INHERIT => &"INHERIT",
            other => unreachable!(
                concat!("invalid value for", stringify!(CompositeAlphaFlagBitsKHR), ": {:?}"),
                other
            ),
        })
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
/// - [`IDENTITY`] specifies that image content is presented without being transformed.
/// - [`ROTATE90`] specifies that image content is rotated 90 degrees clockwise.
/// - [`ROTATE180`] specifies that image content is rotated 180 degrees clockwise.
/// - [`ROTATE270`] specifies that image content is rotated 270 degrees clockwise.
/// - [`HORIZONTAL_MIRROR`] specifies that image content is mirrored horizontally.
/// - [`HORIZONTAL_MIRROR_ROTATE90`] specifies that image content is mirrored horizontally, then
///   rotated 90 degrees clockwise.
/// - [`HORIZONTAL_MIRROR_ROTATE180`] specifies that image content is mirrored horizontally, then
///   rotated 180 degrees clockwise.
/// - [`HORIZONTAL_MIRROR_ROTATE270`] specifies that image content is mirrored horizontally, then
///   rotated 270 degrees clockwise.
/// - [`INHERIT`] specifies that the presentation transform is not specified, and is instead
///   determined by platform-specific considerations and mechanisms outside Vulkan.
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
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct SurfaceTransformFlagBitsKHR(u32);
impl const Default for SurfaceTransformFlagBitsKHR {
    fn default() -> Self {
        Self(0)
    }
}
impl SurfaceTransformFlagBitsKHR {
    ///[`IDENTITY`] specifies that image content
    ///is presented without being transformed.
    pub const IDENTITY: Self = Self(1);
    ///[`ROTATE90`] specifies that image
    ///content is rotated 90 degrees clockwise.
    pub const ROTATE90: Self = Self(2);
    ///[`ROTATE180`] specifies that image
    ///content is rotated 180 degrees clockwise.
    pub const ROTATE180: Self = Self(4);
    ///[`ROTATE270`] specifies that image
    ///content is rotated 270 degrees clockwise.
    pub const ROTATE270: Self = Self(8);
    ///[`HORIZONTAL_MIRROR`] specifies that
    ///image content is mirrored horizontally.
    pub const HORIZONTAL_MIRROR: Self = Self(16);
    ///[`HORIZONTAL_MIRROR_ROTATE90`] specifies
    ///that image content is mirrored horizontally, then rotated 90 degrees
    ///clockwise.
    pub const HORIZONTAL_MIRROR_ROTATE90: Self = Self(32);
    ///[`HORIZONTAL_MIRROR_ROTATE180`]
    ///specifies that image content is mirrored horizontally, then rotated 180
    ///degrees clockwise.
    pub const HORIZONTAL_MIRROR_ROTATE180: Self = Self(64);
    ///[`HORIZONTAL_MIRROR_ROTATE270`]
    ///specifies that image content is mirrored horizontally, then rotated 270
    ///degrees clockwise.
    pub const HORIZONTAL_MIRROR_ROTATE270: Self = Self(128);
    ///[`INHERIT`] specifies that the
    ///presentation transform is not specified, and is instead determined by
    ///platform-specific considerations and mechanisms outside Vulkan.
    pub const INHERIT: Self = Self(256);
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0
    }
    ///Gets a value from a raw underlying value, unchecked and therefore unsafe.
    ///
    ///# Safety
    ///The caller of this function must ensure that all of the bits are valid.
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
}
impl std::fmt::Debug for SurfaceTransformFlagBitsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple(stringify!(SurfaceTransformFlagBitsKHR))
            .field(match *self {
                Self::IDENTITY => &"IDENTITY",
                Self::ROTATE90 => &"ROTATE90",
                Self::ROTATE180 => &"ROTATE180",
                Self::ROTATE270 => &"ROTATE270",
                Self::HORIZONTAL_MIRROR => &"HORIZONTAL_MIRROR",
                Self::HORIZONTAL_MIRROR_ROTATE90 => &"HORIZONTAL_MIRROR_ROTATE90",
                Self::HORIZONTAL_MIRROR_ROTATE180 => &"HORIZONTAL_MIRROR_ROTATE180",
                Self::HORIZONTAL_MIRROR_ROTATE270 => &"HORIZONTAL_MIRROR_ROTATE270",
                Self::INHERIT => &"INHERIT",
                other => unreachable!(
                    concat!("invalid value for", stringify!(SurfaceTransformFlagBitsKHR), ": {:?}"),
                    other
                ),
            })
            .finish()
    }
}
impl std::fmt::Display for SurfaceTransformFlagBitsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str(match *self {
            Self::IDENTITY => &"IDENTITY",
            Self::ROTATE90 => &"ROTATE90",
            Self::ROTATE180 => &"ROTATE180",
            Self::ROTATE270 => &"ROTATE270",
            Self::HORIZONTAL_MIRROR => &"HORIZONTAL_MIRROR",
            Self::HORIZONTAL_MIRROR_ROTATE90 => &"HORIZONTAL_MIRROR_ROTATE90",
            Self::HORIZONTAL_MIRROR_ROTATE180 => &"HORIZONTAL_MIRROR_ROTATE180",
            Self::HORIZONTAL_MIRROR_ROTATE270 => &"HORIZONTAL_MIRROR_ROTATE270",
            Self::INHERIT => &"INHERIT",
            other => unreachable!(
                concat!("invalid value for", stringify!(SurfaceTransformFlagBitsKHR), ": {:?}"),
                other
            ),
        })
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
/// - [`OPAQUE`]: The alpha component, if it exists, of the images is ignored in the compositing
///   process. Instead, the image is treated as if it has a constant alpha of 1.0.
/// - [`PRE_MULTIPLIED`]: The alpha component, if it exists, of the images is respected in the
///   compositing process. The non-alpha components of the image are expected to already be
///   multiplied by the alpha component by the application.
/// - [`POST_MULTIPLIED`]: The alpha component, if it exists, of the images is respected in the
///   compositing process. The non-alpha components of the image are not expected to already be
///   multiplied by the alpha component by the application; instead, the compositor will multiply
///   the non-alpha components of the image by the alpha component during compositing.
/// - [`INHERIT`]: The way in which the presentation engine treats the alpha component in the images
///   is unknown to the Vulkan API. Instead, the application is responsible for setting the
///   composite alpha blending mode using native window system commands. If the application does not
///   set the blending mode using native window system commands, then a platform-specific default
///   will be used.
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
        unsafe { Self::from_bits_unchecked(from.bits()) }
    }
}
impl CompositeAlphaFlagsKHR {
    ///[`OPAQUE`]: The alpha component, if it
    ///exists, of the images is ignored in the compositing process.
    ///Instead, the image is treated as if it has a constant alpha of 1.0.
    pub const OPAQUE: Self = Self(1);
    ///[`PRE_MULTIPLIED`]: The alpha component, if
    ///it exists, of the images is respected in the compositing process.
    ///The non-alpha components of the image are expected to already be
    ///multiplied by the alpha component by the application.
    pub const PRE_MULTIPLIED: Self = Self(2);
    ///[`POST_MULTIPLIED`]: The alpha component,
    ///if it exists, of the images is respected in the compositing process.
    ///The non-alpha components of the image are not expected to already be
    ///multiplied by the alpha component by the application; instead, the
    ///compositor will multiply the non-alpha components of the image by the
    ///alpha component during compositing.
    pub const POST_MULTIPLIED: Self = Self(4);
    ///[`INHERIT`]: The way in which the
    ///presentation engine treats the alpha component in the images is unknown
    ///to the Vulkan API.
    ///Instead, the application is responsible for setting the composite alpha
    ///blending mode using native window system commands.
    ///If the application does not set the blending mode using native window
    ///system commands, then a platform-specific default will be used.
    pub const INHERIT: Self = Self(8);
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
            all |= Self::OPAQUE;
        }
        {
            all |= Self::PRE_MULTIPLIED;
        }
        {
            all |= Self::POST_MULTIPLIED;
        }
        {
            all |= Self::INHERIT;
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
                    if self.0.contains(CompositeAlphaFlagsKHR::OPAQUE) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(OPAQUE))?;
                    }
                    if self.0.contains(CompositeAlphaFlagsKHR::PRE_MULTIPLIED) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(PRE_MULTIPLIED))?;
                    }
                    if self.0.contains(CompositeAlphaFlagsKHR::POST_MULTIPLIED) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(POST_MULTIPLIED))?;
                    }
                    if self.0.contains(CompositeAlphaFlagsKHR::INHERIT) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(INHERIT))?;
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
/// - [`get_physical_device_surface_capabilities_khr`]
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
    ///Sets the value of [`Self::min_image_count`]
    pub fn set_min_image_count(mut self, value: u32) -> Self {
        self.min_image_count = value;
        self
    }
    ///Sets the value of [`Self::max_image_count`]
    pub fn set_max_image_count(mut self, value: u32) -> Self {
        self.max_image_count = value;
        self
    }
    ///Sets the value of [`Self::current_extent`]
    pub fn set_current_extent(mut self, value: crate::vulkan1_0::Extent2D) -> Self {
        self.current_extent = value;
        self
    }
    ///Sets the value of [`Self::min_image_extent`]
    pub fn set_min_image_extent(mut self, value: crate::vulkan1_0::Extent2D) -> Self {
        self.min_image_extent = value;
        self
    }
    ///Sets the value of [`Self::max_image_extent`]
    pub fn set_max_image_extent(mut self, value: crate::vulkan1_0::Extent2D) -> Self {
        self.max_image_extent = value;
        self
    }
    ///Sets the value of [`Self::max_image_array_layers`]
    pub fn set_max_image_array_layers(mut self, value: u32) -> Self {
        self.max_image_array_layers = value;
        self
    }
    ///Sets the value of [`Self::supported_transforms`]
    pub fn set_supported_transforms(mut self, value: crate::extensions::khr_display::SurfaceTransformFlagsKHR) -> Self {
        self.supported_transforms = value;
        self
    }
    ///Sets the value of [`Self::current_transform`]
    pub fn set_current_transform(mut self, value: crate::extensions::khr_surface::SurfaceTransformFlagBitsKHR) -> Self {
        self.current_transform = value;
        self
    }
    ///Sets the value of [`Self::supported_composite_alpha`]
    pub fn set_supported_composite_alpha(
        mut self,
        value: crate::extensions::khr_surface::CompositeAlphaFlagsKHR,
    ) -> Self {
        self.supported_composite_alpha = value;
        self
    }
    ///Sets the value of [`Self::supported_usage_flags`]
    pub fn set_supported_usage_flags(mut self, value: crate::vulkan1_0::ImageUsageFlags) -> Self {
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
/// - [`get_physical_device_surface_formats_khr`]
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
    ///Sets the value of [`Self::format`]
    pub fn set_format(mut self, value: crate::vulkan1_0::Format) -> Self {
        self.format = value;
        self
    }
    ///Sets the value of [`Self::color_space`]
    pub fn set_color_space(mut self, value: crate::extensions::khr_surface::ColorSpaceKHR) -> Self {
        self.color_space = value;
        self
    }
}
impl Instance {
    ///[vkDestroySurfaceKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroySurfaceKHR.html) - Destroy a VkSurfaceKHR object
    ///# C Specifications
    ///To destroy a [`SurfaceKHR`] object, call:
    ///```c
    ///// Provided by VK_KHR_surface
    ///void vkDestroySurfaceKHR(
    ///    VkInstance                                  instance,
    ///    VkSurfaceKHR                                surface,
    ///    const VkAllocationCallbacks*                pAllocator);
    ///```
    ///# Parameters
    /// - [`instance`] is the instance used to create the surface.
    /// - [`surface`] is the surface to destroy.
    /// - [`p_allocator`] is the allocator used for host memory allocated for the surface object when there is no more specific allocator available (see [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation)).
    ///# Description
    ///Destroying a [`SurfaceKHR`] merely severs the connection between Vulkan
    ///and the native surface, and does not imply destroying the native surface,
    ///closing a window, or similar behavior.
    ///## Valid Usage
    /// - All [`SwapchainKHR`] objects created for [`surface`] **must**  have been destroyed prior
    ///   to destroying [`surface`]
    /// - If [`AllocationCallbacks`] were provided when [`surface`] was created, a compatible set of
    ///   callbacks  **must**  be provided here
    /// - If no [`AllocationCallbacks`] were provided when [`surface`] was created, [`p_allocator`]
    ///   **must**  be `NULL`
    ///
    ///## Valid Usage (Implicit)
    /// - [`instance`] **must**  be a valid [`Instance`] handle
    /// - If [`surface`] is not [`crate::Handle::null`], [`surface`] **must**  be a valid
    ///   [`SurfaceKHR`] handle
    /// - If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid
    ///   [`AllocationCallbacks`] structure
    /// - If [`surface`] is a valid handle, it  **must**  have been created, allocated, or retrieved
    ///   from [`instance`]
    ///
    ///## Host Synchronization
    /// - Host access to [`surface`] **must**  be externally synchronized
    ///# Related
    /// - [`VK_KHR_surface`]
    /// - [`AllocationCallbacks`]
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
    #[doc(alias = "vkDestroySurfaceKHR")]
    #[track_caller]
    #[inline]
    pub unsafe fn destroy_surface_khr<'lt>(
        self: &Unique<Instance>,
        surface: Option<SurfaceKHR>,
        p_allocator: Option<&AllocationCallbacks<'lt>>,
    ) -> () {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .khr_surface()
            .and_then(|vtable| vtable.destroy_surface_khr())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .khr_surface()
            .and_then(|vtable| vtable.destroy_surface_khr())
            .unwrap_unchecked();
        let _return = _function(
            self.as_raw(),
            surface.unwrap_or_default(),
            p_allocator
                .map(|v| v as *const AllocationCallbacks<'lt>)
                .unwrap_or_else(std::ptr::null),
        );
        ()
    }
}
impl PhysicalDevice {
    ///[vkGetPhysicalDeviceSurfaceSupportKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfaceSupportKHR.html) - Query if presentation is supported
    ///# C Specifications
    ///To determine whether a queue family of a physical device supports
    ///presentation to a given surface, call:
    ///```c
    ///// Provided by VK_KHR_surface
    ///VkResult vkGetPhysicalDeviceSurfaceSupportKHR(
    ///    VkPhysicalDevice                            physicalDevice,
    ///    uint32_t                                    queueFamilyIndex,
    ///    VkSurfaceKHR                                surface,
    ///    VkBool32*                                   pSupported);
    ///```
    ///# Parameters
    /// - [`physical_device`] is the physical device.
    /// - [`queue_family_index`] is the queue family.
    /// - [`surface`] is the surface.
    /// - [`p_supported`] is a pointer to a [`Bool32`], which is set to [`TRUE`] to indicate
    ///   support, and [`FALSE`] otherwise.
    ///# Description
    ///## Valid Usage
    /// - [`queue_family_index`] **must**  be less than `pQueueFamilyPropertyCount` returned by
    ///   [`get_physical_device_queue_family_properties`] for the given [`physical_device`]
    ///
    ///## Valid Usage (Implicit)
    /// - [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
    /// - [`surface`] **must**  be a valid [`SurfaceKHR`] handle
    /// - [`p_supported`] **must**  be a valid pointer to a [`Bool32`] value
    /// - Both of [`physical_device`], and [`surface`] **must**  have been created, allocated, or
    ///   retrieved from the same [`Instance`]
    ///
    ///## Return Codes
    /// * - `VK_SUCCESS`
    /// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  -
    ///   `VK_ERROR_SURFACE_LOST_KHR`
    ///# Related
    /// - [`VK_KHR_surface`]
    /// - [`Bool32`]
    /// - [`PhysicalDevice`]
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
    #[doc(alias = "vkGetPhysicalDeviceSurfaceSupportKHR")]
    #[track_caller]
    #[inline]
    pub unsafe fn get_physical_device_surface_support_khr(
        self: &Unique<PhysicalDevice>,
        queue_family_index: Option<u32>,
        surface: SurfaceKHR,
    ) -> VulkanResult<bool> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .instance()
            .vtable()
            .khr_surface()
            .and_then(|vtable| vtable.get_physical_device_surface_support_khr())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .instance()
            .vtable()
            .khr_surface()
            .and_then(|vtable| vtable.get_physical_device_surface_support_khr())
            .unwrap_unchecked();
        let mut p_supported: u32 = 0;
        let _return = _function(
            self.as_raw(),
            queue_family_index.unwrap_or_default() as _,
            surface,
            &mut p_supported,
        );
        match _return {
            VulkanResultCodes::SUCCESS => VulkanResult::Success(_return, p_supported != 0),
            e => VulkanResult::Err(e),
        }
    }
}
impl PhysicalDevice {
    ///[vkGetPhysicalDeviceSurfaceCapabilitiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfaceCapabilitiesKHR.html) - Query surface capabilities
    ///# C Specifications
    ///To query the basic capabilities of a surface, needed in order to create a
    ///swapchain, call:
    ///```c
    ///// Provided by VK_KHR_surface
    ///VkResult vkGetPhysicalDeviceSurfaceCapabilitiesKHR(
    ///    VkPhysicalDevice                            physicalDevice,
    ///    VkSurfaceKHR                                surface,
    ///    VkSurfaceCapabilitiesKHR*                   pSurfaceCapabilities);
    ///```
    ///# Parameters
    /// - [`physical_device`] is the physical device that will be associated with the swapchain to
    ///   be created, as described for [`create_swapchain_khr`].
    /// - [`surface`] is the surface that will be associated with the swapchain.
    /// - [`p_surface_capabilities`] is a pointer to a [`SurfaceCapabilitiesKHR`] structure in which
    ///   the capabilities are returned.
    ///# Description
    ///## Valid Usage
    /// - [`surface`] **must**  be a valid [`SurfaceKHR`] handle
    /// - [`surface`] **must**  be supported by [`physical_device`], as reported by
    ///   [`get_physical_device_surface_support_khr`] or an equivalent platform-specific mechanism
    ///
    ///## Valid Usage (Implicit)
    /// - [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
    /// - [`surface`] **must**  be a valid [`SurfaceKHR`] handle
    /// - [`p_surface_capabilities`] **must**  be a valid pointer to a [`SurfaceCapabilitiesKHR`]
    ///   structure
    /// - Both of [`physical_device`], and [`surface`] **must**  have been created, allocated, or
    ///   retrieved from the same [`Instance`]
    ///
    ///## Return Codes
    /// * - `VK_SUCCESS`
    /// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  -
    ///   `VK_ERROR_SURFACE_LOST_KHR`
    ///# Related
    /// - [`VK_KHR_surface`]
    /// - [`PhysicalDevice`]
    /// - [`SurfaceCapabilitiesKHR`]
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
    #[doc(alias = "vkGetPhysicalDeviceSurfaceCapabilitiesKHR")]
    #[track_caller]
    #[inline]
    pub unsafe fn get_physical_device_surface_capabilities_khr(
        self: &Unique<PhysicalDevice>,
        surface: SurfaceKHR,
    ) -> VulkanResult<SurfaceCapabilitiesKHR> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .instance()
            .vtable()
            .khr_surface()
            .and_then(|vtable| vtable.get_physical_device_surface_capabilities_khr())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .instance()
            .vtable()
            .khr_surface()
            .and_then(|vtable| vtable.get_physical_device_surface_capabilities_khr())
            .unwrap_unchecked();
        let mut p_surface_capabilities = MaybeUninit::<SurfaceCapabilitiesKHR>::uninit();
        let _return = _function(self.as_raw(), surface, p_surface_capabilities.as_mut_ptr());
        match _return {
            VulkanResultCodes::SUCCESS => VulkanResult::Success(_return, p_surface_capabilities.assume_init()),
            e => VulkanResult::Err(e),
        }
    }
}
impl PhysicalDevice {
    ///[vkGetPhysicalDeviceSurfaceFormatsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfaceFormatsKHR.html) - Query color formats supported by surface
    ///# C Specifications
    ///To query the supported swapchain format-color space pairs for a surface,
    ///call:
    ///```c
    ///// Provided by VK_KHR_surface
    ///VkResult vkGetPhysicalDeviceSurfaceFormatsKHR(
    ///    VkPhysicalDevice                            physicalDevice,
    ///    VkSurfaceKHR                                surface,
    ///    uint32_t*                                   pSurfaceFormatCount,
    ///    VkSurfaceFormatKHR*                         pSurfaceFormats);
    ///```
    ///# Parameters
    /// - [`physical_device`] is the physical device that will be associated with the swapchain to
    ///   be created, as described for [`create_swapchain_khr`].
    /// - [`surface`] is the surface that will be associated with the swapchain.
    /// - [`p_surface_format_count`] is a pointer to an integer related to the number of format
    ///   pairs available or queried, as described below.
    /// - [`p_surface_formats`] is either `NULL` or a pointer to an array of [`SurfaceFormatKHR`]
    ///   structures.
    ///# Description
    ///If [`p_surface_formats`] is `NULL`, then the number of format pairs
    ///supported for the given [`surface`] is returned in
    ///[`p_surface_format_count`].
    ///Otherwise, [`p_surface_format_count`] **must**  point to a variable set by the
    ///user to the number of elements in the [`p_surface_formats`] array, and on
    ///return the variable is overwritten with the number of structures actually
    ///written to [`p_surface_formats`].
    ///If the value of [`p_surface_format_count`] is less than the number of format
    ///pairs supported, at most [`p_surface_format_count`] structures will be
    ///written, and `VK_INCOMPLETE` will be returned instead of
    ///`VK_SUCCESS`, to indicate that not all the available format pairs were
    ///returned.The number of format pairs supported  **must**  be greater than or equal to 1.
    ///[`p_surface_formats`] **must**  not contain an entry whose value for
    ///`format` is `VK_FORMAT_UNDEFINED`.If [`p_surface_formats`] includes an entry whose value for
    /// `colorSpace`
    ///is `VK_COLOR_SPACE_SRGB_NONLINEAR_KHR` and whose value for `format`
    ///is a UNORM (or SRGB) format and the corresponding SRGB (or UNORM) format is
    ///a color renderable format for `VK_IMAGE_TILING_OPTIMAL`, then
    ///[`p_surface_formats`] **must**  also contain an entry with the same value for
    ///`colorSpace` and `format` equal to the corresponding SRGB (or UNORM)
    ///format.If the `[`VK_GOOGLE_surfaceless_query`]` extension is enabled, the values
    ///returned in [`p_surface_formats`] will be identical for every valid surface
    ///created on this physical device, and so [`surface`] **can**  be
    ///[`crate::Handle::null`].
    ///## Valid Usage
    /// - If the `[`VK_GOOGLE_surfaceless_query`]` extension is not enabled, [`surface`] **must**
    ///   be a valid [`SurfaceKHR`] handle
    /// - If [`surface`] is not [`crate::Handle::null`], it  **must**  be supported by
    ///   [`physical_device`], as reported by [`get_physical_device_surface_support_khr`] or an
    ///   equivalent platform-specific mechanism
    ///
    ///## Valid Usage (Implicit)
    /// - [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
    /// - If [`surface`] is not [`crate::Handle::null`], [`surface`] **must**  be a valid
    ///   [`SurfaceKHR`] handle
    /// - [`p_surface_format_count`] **must**  be a valid pointer to a `uint32_t` value
    /// - If the value referenced by [`p_surface_format_count`] is not `0`, and
    ///   [`p_surface_formats`] is not `NULL`, [`p_surface_formats`] **must**  be a valid pointer to
    ///   an array of [`p_surface_format_count`][`SurfaceFormatKHR`] structures
    /// - Both of [`physical_device`], and [`surface`] that are valid handles of non-ignored
    ///   parameters  **must**  have been created, allocated, or retrieved from the same
    ///   [`Instance`]
    ///
    ///## Return Codes
    /// * - `VK_SUCCESS`  - `VK_INCOMPLETE`
    /// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  -
    ///   `VK_ERROR_SURFACE_LOST_KHR`
    ///# Related
    /// - [`VK_KHR_surface`]
    /// - [`PhysicalDevice`]
    /// - [`SurfaceFormatKHR`]
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
    #[doc(alias = "vkGetPhysicalDeviceSurfaceFormatsKHR")]
    #[track_caller]
    #[inline]
    pub unsafe fn get_physical_device_surface_formats_khr(
        self: &Unique<PhysicalDevice>,
        surface: Option<SurfaceKHR>,
        p_surface_format_count: Option<usize>,
    ) -> VulkanResult<SmallVec<SurfaceFormatKHR>> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .instance()
            .vtable()
            .khr_surface()
            .and_then(|vtable| vtable.get_physical_device_surface_formats_khr())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .instance()
            .vtable()
            .khr_surface()
            .and_then(|vtable| vtable.get_physical_device_surface_formats_khr())
            .unwrap_unchecked();
        let mut p_surface_format_count = match p_surface_format_count {
            Some(v) => v as _,
            None => {
                let mut v = 0;
                _function(self.as_raw(), surface.unwrap_or_default(), &mut v, std::ptr::null_mut());
                v
            },
        };
        let mut p_surface_formats =
            SmallVec::<SurfaceFormatKHR>::from_elem(Default::default(), p_surface_format_count as usize);
        let _return = _function(
            self.as_raw(),
            surface.unwrap_or_default(),
            &mut p_surface_format_count,
            p_surface_formats.as_mut_ptr(),
        );
        match _return {
            VulkanResultCodes::SUCCESS | VulkanResultCodes::INCOMPLETE => {
                VulkanResult::Success(_return, p_surface_formats)
            },
            e => VulkanResult::Err(e),
        }
    }
}
impl PhysicalDevice {
    ///[vkGetPhysicalDeviceSurfacePresentModesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfacePresentModesKHR.html) - Query supported presentation modes
    ///# C Specifications
    ///To query the supported presentation modes for a surface, call:
    ///```c
    ///// Provided by VK_KHR_surface
    ///VkResult vkGetPhysicalDeviceSurfacePresentModesKHR(
    ///    VkPhysicalDevice                            physicalDevice,
    ///    VkSurfaceKHR                                surface,
    ///    uint32_t*                                   pPresentModeCount,
    ///    VkPresentModeKHR*                           pPresentModes);
    ///```
    ///# Parameters
    /// - [`physical_device`] is the physical device that will be associated with the swapchain to
    ///   be created, as described for [`create_swapchain_khr`].
    /// - [`surface`] is the surface that will be associated with the swapchain.
    /// - [`p_present_mode_count`] is a pointer to an integer related to the number of presentation
    ///   modes available or queried, as described below.
    /// - [`p_present_modes`] is either `NULL` or a pointer to an array of [`PresentModeKHR`]
    ///   values, indicating the supported presentation modes.
    ///# Description
    ///If [`p_present_modes`] is `NULL`, then the number of presentation modes
    ///supported for the given [`surface`] is returned in
    ///[`p_present_mode_count`].
    ///Otherwise, [`p_present_mode_count`] **must**  point to a variable set by the user
    ///to the number of elements in the [`p_present_modes`] array, and on return
    ///the variable is overwritten with the number of values actually written to
    ///[`p_present_modes`].
    ///If the value of [`p_present_mode_count`] is less than the number of
    ///presentation modes supported, at most [`p_present_mode_count`] values will be
    ///written, and `VK_INCOMPLETE` will be returned instead of
    ///`VK_SUCCESS`, to indicate that not all the available modes were
    ///returned.If the `[`VK_GOOGLE_surfaceless_query`]` extension is enabled, the values
    ///returned in [`p_present_modes`] will be identical for every valid surface
    ///created on this physical device, and so [`surface`] **can**  be
    ///[`crate::Handle::null`].
    ///## Valid Usage
    /// - If the `[`VK_GOOGLE_surfaceless_query`]` extension is not enabled, [`surface`] **must**
    ///   be a valid [`SurfaceKHR`] handle
    /// - If [`surface`] is not [`crate::Handle::null`], it  **must**  be supported by
    ///   [`physical_device`], as reported by [`get_physical_device_surface_support_khr`] or an
    ///   equivalent platform-specific mechanism
    ///
    ///## Valid Usage (Implicit)
    /// - [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
    /// - If [`surface`] is not [`crate::Handle::null`], [`surface`] **must**  be a valid
    ///   [`SurfaceKHR`] handle
    /// - [`p_present_mode_count`] **must**  be a valid pointer to a `uint32_t` value
    /// - If the value referenced by [`p_present_mode_count`] is not `0`, and [`p_present_modes`] is
    ///   not `NULL`, [`p_present_modes`] **must**  be a valid pointer to an array of
    ///   [`p_present_mode_count`][`PresentModeKHR`] values
    /// - Both of [`physical_device`], and [`surface`] that are valid handles of non-ignored
    ///   parameters  **must**  have been created, allocated, or retrieved from the same
    ///   [`Instance`]
    ///
    ///## Return Codes
    /// * - `VK_SUCCESS`  - `VK_INCOMPLETE`
    /// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  -
    ///   `VK_ERROR_SURFACE_LOST_KHR`
    ///# Related
    /// - [`VK_KHR_surface`]
    /// - [`PhysicalDevice`]
    /// - [`PresentModeKHR`]
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
    #[doc(alias = "vkGetPhysicalDeviceSurfacePresentModesKHR")]
    #[track_caller]
    #[inline]
    pub unsafe fn get_physical_device_surface_present_modes_khr(
        self: &Unique<PhysicalDevice>,
        surface: Option<SurfaceKHR>,
        p_present_mode_count: Option<usize>,
    ) -> VulkanResult<SmallVec<PresentModeKHR>> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .instance()
            .vtable()
            .khr_surface()
            .and_then(|vtable| vtable.get_physical_device_surface_present_modes_khr())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .instance()
            .vtable()
            .khr_surface()
            .and_then(|vtable| vtable.get_physical_device_surface_present_modes_khr())
            .unwrap_unchecked();
        let mut p_present_mode_count = match p_present_mode_count {
            Some(v) => v as _,
            None => {
                let mut v = 0;
                _function(self.as_raw(), surface.unwrap_or_default(), &mut v, std::ptr::null_mut());
                v
            },
        };
        let mut p_present_modes =
            SmallVec::<PresentModeKHR>::from_elem(Default::default(), p_present_mode_count as usize);
        let _return = _function(
            self.as_raw(),
            surface.unwrap_or_default(),
            &mut p_present_mode_count,
            p_present_modes.as_mut_ptr(),
        );
        match _return {
            VulkanResultCodes::SUCCESS | VulkanResultCodes::INCOMPLETE => {
                VulkanResult::Success(_return, p_present_modes)
            },
            e => VulkanResult::Err(e),
        }
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
/// - [`create_android_surface_khr`]
/// - [`create_direct_fb_surface_ext`]
/// - [`create_display_plane_surface_khr`]
/// - [`create_headless_surface_ext`]
/// - [`create_ios_surface_mvk`]
/// - [`create_image_pipe_surface_fuchsia`]
/// - [`create_mac_os_surface_mvk`]
/// - [`create_metal_surface_ext`]
/// - [`create_screen_surface_qnx`]
/// - [`create_stream_descriptor_surface_ggp`]
/// - [`create_vi_surface_nn`]
/// - [`create_wayland_surface_khr`]
/// - [`create_win32_surface_khr`]
/// - [`create_xcb_surface_khr`]
/// - [`create_xlib_surface_khr`]
/// - [`destroy_surface_khr`]
/// - [`get_device_group_surface_present_modes_khr`]
/// - [`get_physical_device_present_rectangles_khr`]
/// - [`get_physical_device_surface_capabilities2_ext`]
/// - [`get_physical_device_surface_capabilities_khr`]
/// - [`get_physical_device_surface_formats_khr`]
/// - [`get_physical_device_surface_present_modes_khr`]
/// - [`get_physical_device_surface_support_khr`]
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
impl Handle for SurfaceKHR {
    type Parent = Unique<Instance>;
    type VTable = ();
    type Metadata = AtomicBool;
    type Raw = u64;
    #[inline]
    fn as_raw(self) -> Self::Raw {
        self.0
    }
    #[inline]
    unsafe fn from_raw(this: Self::Raw) -> Self {
        Self(this)
    }
    #[inline]
    #[track_caller]
    unsafe fn destroy(self: &mut Unique<Self>) {
        if !self.metadata().load(Ordering::Acquire) {
            self.instance().destroy_surface_khr(Some(self.as_raw().coerce()), None);
        }
    }
    #[inline]
    unsafe fn load_vtable(&self, _: &Self::Parent, _: &Self::Metadata) -> Self::VTable {}
}
impl Unique<SurfaceKHR> {
    ///Gets the reference to the [`Entry`]
    #[inline]
    pub fn entry(&self) -> &Arc<Entry> {
        self.parent().parent()
    }
    ///Gets the reference to the [`Instance`]
    #[inline]
    pub fn instance(&self) -> &Unique<Instance> {
        self.parent()
    }
    ///Disables the base dropping behaviour of this handle
    #[inline]
    pub fn disable_drop(&self) {
        self.metadata().store(true, Ordering::Relaxed);
    }
}
///The V-table of [`Instance`] for functions from `VK_KHR_surface`
pub struct InstanceKhrSurfaceVTable {
    ///See [`FNDestroySurfaceKhr`] for more information.
    pub destroy_surface_khr: FNDestroySurfaceKhr,
    ///See [`FNGetPhysicalDeviceSurfaceSupportKhr`] for more information.
    pub get_physical_device_surface_support_khr: FNGetPhysicalDeviceSurfaceSupportKhr,
    ///See [`FNGetPhysicalDeviceSurfaceCapabilitiesKhr`] for more information.
    pub get_physical_device_surface_capabilities_khr: FNGetPhysicalDeviceSurfaceCapabilitiesKhr,
    ///See [`FNGetPhysicalDeviceSurfaceFormatsKhr`] for more information.
    pub get_physical_device_surface_formats_khr: FNGetPhysicalDeviceSurfaceFormatsKhr,
    ///See [`FNGetPhysicalDeviceSurfacePresentModesKhr`] for more information.
    pub get_physical_device_surface_present_modes_khr: FNGetPhysicalDeviceSurfacePresentModesKhr,
}
impl InstanceKhrSurfaceVTable {
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
            destroy_surface_khr: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkDestroySurfaceKHR").as_ptr()))
            },
            get_physical_device_surface_support_khr: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkGetPhysicalDeviceSurfaceSupportKHR").as_ptr(),
                ))
            },
            get_physical_device_surface_capabilities_khr: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkGetPhysicalDeviceSurfaceCapabilitiesKHR").as_ptr(),
                ))
            },
            get_physical_device_surface_formats_khr: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkGetPhysicalDeviceSurfaceFormatsKHR").as_ptr(),
                ))
            },
            get_physical_device_surface_present_modes_khr: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkGetPhysicalDeviceSurfacePresentModesKHR").as_ptr(),
                ))
            },
        }
    }
    ///Gets [`Self::destroy_surface_khr`]. See [`FNDestroySurfaceKhr`] for more information.
    pub fn destroy_surface_khr(&self) -> FNDestroySurfaceKhr {
        self.destroy_surface_khr
    }
    ///Gets [`Self::get_physical_device_surface_support_khr`]. See
    /// [`FNGetPhysicalDeviceSurfaceSupportKhr`] for more information.
    pub fn get_physical_device_surface_support_khr(&self) -> FNGetPhysicalDeviceSurfaceSupportKhr {
        self.get_physical_device_surface_support_khr
    }
    ///Gets [`Self::get_physical_device_surface_capabilities_khr`]. See
    /// [`FNGetPhysicalDeviceSurfaceCapabilitiesKhr`] for more information.
    pub fn get_physical_device_surface_capabilities_khr(&self) -> FNGetPhysicalDeviceSurfaceCapabilitiesKhr {
        self.get_physical_device_surface_capabilities_khr
    }
    ///Gets [`Self::get_physical_device_surface_formats_khr`]. See
    /// [`FNGetPhysicalDeviceSurfaceFormatsKhr`] for more information.
    pub fn get_physical_device_surface_formats_khr(&self) -> FNGetPhysicalDeviceSurfaceFormatsKhr {
        self.get_physical_device_surface_formats_khr
    }
    ///Gets [`Self::get_physical_device_surface_present_modes_khr`]. See
    /// [`FNGetPhysicalDeviceSurfacePresentModesKhr`] for more information.
    pub fn get_physical_device_surface_present_modes_khr(&self) -> FNGetPhysicalDeviceSurfacePresentModesKhr {
        self.get_physical_device_surface_present_modes_khr
    }
}
