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
//!operation directly using the new [`acquire_full_screen_exclusive_mode_ext`]
//!and [`release_full_screen_exclusive_mode_ext`] commands.
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
//! - [`acquire_full_screen_exclusive_mode_ext`]
//! - [`get_physical_device_surface_present_modes2_ext`]
//! - [`release_full_screen_exclusive_mode_ext`]
//!If [`VK_KHR_device_group`] is supported:
//! - [`get_device_group_surface_present_modes2_ext`]
//!If [Version 1.1]() is supported:
//! - [`get_device_group_surface_present_modes2_ext`]
//!# New structures
//! - Extending [`PhysicalDeviceSurfaceInfo2KHR`], [`SwapchainCreateInfoKHR`]:  -
//!   [`SurfaceFullScreenExclusiveInfoEXT`]
//! - Extending [`SurfaceCapabilities2KHR`]:  - [`SurfaceCapabilitiesFullScreenExclusiveEXT`]
//!If [`VK_KHR_win32_surface`] is supported:
//! - Extending [`PhysicalDeviceSurfaceInfo2KHR`], [`SwapchainCreateInfoKHR`]:  -
//!   [`SurfaceFullScreenExclusiveWin32InfoEXT`]
//!# New enums
//! - [`FullScreenExclusiveEXT`]
//!# New constants
//! - [`EXT_FULL_SCREEN_EXCLUSIVE_EXTENSION_NAME`]
//! - [`EXT_FULL_SCREEN_EXCLUSIVE_SPEC_VERSION`]
//! - Extending [`VulkanResultCodes`]:  - `VK_ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT`
//! - Extending [`StructureType`]:  -
//!   `VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES_FULL_SCREEN_EXCLUSIVE_EXT`  -
//!   `VK_STRUCTURE_TYPE_SURFACE_FULL_SCREEN_EXCLUSIVE_INFO_EXT`
//!If [`VK_KHR_win32_surface`] is supported:
//! - Extending [`StructureType`]:  -
//!   `VK_STRUCTURE_TYPE_SURFACE_FULL_SCREEN_EXCLUSIVE_WIN32_INFO_EXT`
//!# Known issues & F.A.Q
//!1) What should the extension & flag be called? **RESOLVED** : VK_EXT_full_screen_exclusive.Other
//! options considered (prior to the app-controlled mode) were:
//! - VK_EXT_smooth_fullscreen_transition
//! - VK_EXT_fullscreen_behavior
//! - VK_EXT_fullscreen_preference
//! - VK_EXT_fullscreen_hint
//! - VK_EXT_fast_fullscreen_transition
//! - VK_EXT_avoid_fullscreen_exclusive
//!2) Do we need more than a boolean toggle? **RESOLVED** : Yes.Using an enum with
//! default/allowed/disallowed/app-controlled enables
//!applications to accept driver default behavior, specifically override it in
//!either direction without implying the driver is ever required to use
//!full-screen exclusive mechanisms, or manage this mode explicitly.3) Should this be a KHR or EXT
//! extension? **RESOLVED** : EXT, in order to allow it to be shipped faster.4) Can the fullscreen
//! hint affect the surface capabilities, and if so,
//!should the hint also be specified as input when querying the surface
//!capabilities? **RESOLVED** : Yes on both accounts.While the hint does not guarantee a particular
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
//!result in never- terminating surface creation loops.5) Should full-screen be one word or two?
//! **RESOLVED** : Two words."Fullscreen" is not in my dictionary, and web searches did not turn up
//!definitive proof that it is a colloquially accepted compound word.
//!Documentation for the corresponding Windows API mechanisms dithers.
//!The text consistently uses a hyphen, but none-the-less, there is a
//!SetFullscreenState method in the DXGI swapchain object.
//!Given this inconclusive external guidance, it is best to adhere to the
//!Vulkan style guidelines and avoid inventing new compound words.
//!# Version History
//! - Revision 4, 2019-03-12 (Tobias Hector)  - Added application-controlled mode, and related
//!   functions  - Tidied up appendix
//! - Revision 3, 2019-01-03 (James Jones)  - Renamed to VK_EXT_full_screen_exclusive  - Made
//!   related adjustments to the tri-state enumerant names.
//! - Revision 2, 2018-11-27 (James Jones)  - Renamed to VK_KHR_fullscreen_behavior  - Switched from
//!   boolean flag to tri-state enum
//! - Revision 1, 2018-11-06 (James Jones)  - Internal revision
//!# Other info
//! * 2019-03-12
//! * No known IP claims.
//! * - Interacts with Vulkan 1.1  - Interacts with `[`VK_KHR_device_group`]`  - Interacts with
//!   `[`VK_KHR_win32_surface`]`
//! * - Hans-Kristian Arntzen, ARM  - Slawomir Grajewski, Intel  - Tobias Hector, AMD  - James
//!   Jones, NVIDIA  - Daniel Rakos, AMD  - Jeff Juliano, NVIDIA  - Joshua Schnarr, NVIDIA  - Aaron
//!   Hagan, AMD
//!# Related
//! - [`FullScreenExclusiveEXT`]
//! - [`SurfaceCapabilitiesFullScreenExclusiveEXT`]
//! - [`SurfaceFullScreenExclusiveInfoEXT`]
//! - [`acquire_full_screen_exclusive_mode_ext`]
//! - [`get_physical_device_surface_present_modes2_ext`]
//! - [`release_full_screen_exclusive_mode_ext`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::{
    extensions::{
        khr_device_group::DeviceGroupPresentModeFlagsKHR, khr_get_surface_capabilities2::PhysicalDeviceSurfaceInfo2KHR,
        khr_surface::PresentModeKHR, khr_swapchain::SwapchainKHR,
    },
    native::HMONITOR,
    vulkan1_0::{
        BaseInStructure, BaseOutStructure, Bool32, Device, Instance, PhysicalDevice, StructureType, VulkanResultCodes,
    },
    AsRaw, SmallVec, Unique, VulkanResult,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_FULL_SCREEN_EXCLUSIVE_SPEC_VERSION")]
pub const EXT_FULL_SCREEN_EXCLUSIVE_SPEC_VERSION: u32 = 4;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_FULL_SCREEN_EXCLUSIVE_EXTENSION_NAME")]
pub const EXT_FULL_SCREEN_EXCLUSIVE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_full_screen_exclusive");
///[vkGetPhysicalDeviceSurfacePresentModes2EXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfacePresentModes2EXT.html) - Query supported presentation modes
///# C Specifications
///Alternatively, to query the supported presentation modes for a surface
///combined with select other fixed swapchain creation parameters, call:
///```c
///// Provided by VK_EXT_full_screen_exclusive
///VkResult vkGetPhysicalDeviceSurfacePresentModes2EXT(
///    VkPhysicalDevice                            physicalDevice,
///    const VkPhysicalDeviceSurfaceInfo2KHR*      pSurfaceInfo,
///    uint32_t*                                   pPresentModeCount,
///    VkPresentModeKHR*                           pPresentModes);
///```
///# Parameters
/// - [`physical_device`] is the physical device that will be associated with the swapchain to be
///   created, as described for [`create_swapchain_khr`].
/// - [`p_surface_info`] is a pointer to a [`PhysicalDeviceSurfaceInfo2KHR`] structure describing
///   the surface and other fixed parameters that would be consumed by [`create_swapchain_khr`].
/// - [`p_present_mode_count`] is a pointer to an integer related to the number of presentation
///   modes available or queried, as described below.
/// - [`p_present_modes`] is either `NULL` or a pointer to an array of [`PresentModeKHR`] values,
///   indicating the supported presentation modes.
///# Description
///[`get_physical_device_surface_present_modes2_ext`] behaves similarly to
///[`get_physical_device_surface_present_modes_khr`], with the ability to specify
///extended inputs via chained input structures.
///## Valid Usage
/// - If the `[`VK_GOOGLE_surfaceless_query`]` extension is not enabled, `pSurfaceInfo->surface`
///   **must**  be a valid [`SurfaceKHR`] handle
/// - If `pSurfaceInfo->surface` is not [`crate::Handle::null`], it  **must**  be supported by
///   [`physical_device`], as reported by [`get_physical_device_surface_support_khr`] or an
///   equivalent platform-specific mechanism
///
///## Valid Usage (Implicit)
/// - [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
/// - [`p_surface_info`] **must**  be a valid pointer to a valid [`PhysicalDeviceSurfaceInfo2KHR`]
///   structure
/// - [`p_present_mode_count`] **must**  be a valid pointer to a `uint32_t` value
/// - If the value referenced by [`p_present_mode_count`] is not `0`, and [`p_present_modes`] is not
///   `NULL`, [`p_present_modes`] **must**  be a valid pointer to an array of
///   [`p_present_mode_count`][`PresentModeKHR`] values
///
///## Return Codes
/// * - `VK_SUCCESS`  - `VK_INCOMPLETE`
/// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  -
///   `VK_ERROR_SURFACE_LOST_KHR`
///# Related
/// - [`VK_EXT_full_screen_exclusive`]
/// - [`PhysicalDevice`]
/// - [`PhysicalDeviceSurfaceInfo2KHR`]
/// - [`PresentModeKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkGetPhysicalDeviceSurfacePresentModes2EXT")]
pub type FNGetPhysicalDeviceSurfacePresentModes2Ext = Option<
    for<'lt> unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_surface_info: *const PhysicalDeviceSurfaceInfo2KHR<'lt>,
        p_present_mode_count: *mut u32,
        p_present_modes: *mut PresentModeKHR,
    ) -> VulkanResultCodes,
>;
///[vkGetDeviceGroupSurfacePresentModes2EXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceGroupSurfacePresentModes2EXT.html) - Query device group present capabilities for a surface
///# C Specifications
///Alternatively, to query the supported device group presentation modes for a
///surface combined with select other fixed swapchain creation parameters,
///call:
///```c
///// Provided by VK_VERSION_1_1 with VK_EXT_full_screen_exclusive, VK_KHR_device_group with
///// VK_EXT_full_screen_exclusive
///VkResult vkGetDeviceGroupSurfacePresentModes2EXT(
///    VkDevice                                    device,
///    const VkPhysicalDeviceSurfaceInfo2KHR*      pSurfaceInfo,
///    VkDeviceGroupPresentModeFlagsKHR*           pModes);
///```
///# Parameters
/// - [`device`] is the logical device.
/// - [`p_surface_info`] is a pointer to a [`PhysicalDeviceSurfaceInfo2KHR`] structure describing
///   the surface and other fixed parameters that would be consumed by [`create_swapchain_khr`].
/// - [`p_modes`] is a pointer to a [`DeviceGroupPresentModeFlagsKHR`] in which the supported device
///   group present modes for the surface are returned.
///# Description
///[`get_device_group_surface_present_modes2_ext`] behaves similarly to
///[`get_device_group_surface_present_modes_khr`], with the ability to specify
///extended inputs via chained input structures.
///## Valid Usage
/// - `pSurfaceInfo->surface` **must**  be supported by all physical devices associated with
///   [`device`], as reported by [`get_physical_device_surface_support_khr`] or an equivalent
///   platform-specific mechanism
///
///## Valid Usage (Implicit)
/// - [`device`] **must**  be a valid [`Device`] handle
/// - [`p_surface_info`] **must**  be a valid pointer to a valid [`PhysicalDeviceSurfaceInfo2KHR`]
///   structure
/// - [`p_modes`] **must**  be a valid pointer to a [`DeviceGroupPresentModeFlagsKHR`] value
///
///## Return Codes
/// * - `VK_SUCCESS`
/// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  -
///   `VK_ERROR_SURFACE_LOST_KHR`
///# Related
/// - [`VK_EXT_full_screen_exclusive`]
/// - [`VK_KHR_device_group`]
/// - [`crate::vulkan1_1`]
/// - [`Device`]
/// - [`DeviceGroupPresentModeFlagsKHR`]
/// - [`PhysicalDeviceSurfaceInfo2KHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkGetDeviceGroupSurfacePresentModes2EXT")]
pub type FNGetDeviceGroupSurfacePresentModes2Ext = Option<
    for<'lt> unsafe extern "system" fn(
        device: Device,
        p_surface_info: *const PhysicalDeviceSurfaceInfo2KHR<'lt>,
        p_modes: *mut DeviceGroupPresentModeFlagsKHR,
    ) -> VulkanResultCodes,
>;
///[vkAcquireFullScreenExclusiveModeEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAcquireFullScreenExclusiveModeEXT.html) - Acquire full-screen exclusive mode for a swapchain
///# C Specifications
///To acquire exclusive full-screen access for a swapchain, call:
///```c
///// Provided by VK_EXT_full_screen_exclusive
///VkResult vkAcquireFullScreenExclusiveModeEXT(
///    VkDevice                                    device,
///    VkSwapchainKHR                              swapchain);
///```
///# Parameters
/// - [`device`] is the device associated with [`swapchain`].
/// - [`swapchain`] is the swapchain to acquire exclusive full-screen access for.
///# Description
///## Valid Usage
/// - [`swapchain`] **must**  not be in the retired state
/// - [`swapchain`] **must**  be a swapchain created with a [`SurfaceFullScreenExclusiveInfoEXT`]
///   structure, with `fullScreenExclusive` set to
///   `VK_FULL_SCREEN_EXCLUSIVE_APPLICATION_CONTROLLED_EXT`
/// - [`swapchain`] **must**  not currently have exclusive full-screen access
///A return value of `VK_SUCCESS` indicates that the [`swapchain`]
///successfully acquired exclusive full-screen access.
///The swapchain will retain this exclusivity until either the application
///releases exclusive full-screen access with
///[`release_full_screen_exclusive_mode_ext`], destroys the swapchain, or if any
///of the swapchain commands return
///`VK_ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT` indicating that the mode
///was lost because of platform-specific changes.If the swapchain was unable to acquire exclusive
/// full-screen access to the
///display then `VK_ERROR_INITIALIZATION_FAILED` is returned.
///An application  **can**  attempt to acquire exclusive full-screen access again
///for the same swapchain even if this command fails, or if
///`VK_ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT` has been returned by a
///swapchain command.
///## Valid Usage (Implicit)
/// - [`device`] **must**  be a valid [`Device`] handle
/// - [`swapchain`] **must**  be a valid [`SwapchainKHR`] handle
/// - Both of [`device`], and [`swapchain`] **must**  have been created, allocated, or retrieved
///   from the same [`Instance`]
///
///## Return Codes
/// * - `VK_SUCCESS`
/// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  -
///   `VK_ERROR_INITIALIZATION_FAILED`  - `VK_ERROR_SURFACE_LOST_KHR`
///# Related
/// - [`VK_EXT_full_screen_exclusive`]
/// - [`Device`]
/// - [`SwapchainKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkAcquireFullScreenExclusiveModeEXT")]
pub type FNAcquireFullScreenExclusiveModeExt =
    Option<unsafe extern "system" fn(device: Device, swapchain: SwapchainKHR) -> VulkanResultCodes>;
///[vkReleaseFullScreenExclusiveModeEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkReleaseFullScreenExclusiveModeEXT.html) - Release full-screen exclusive mode from a swapchain
///# C Specifications
///To release exclusive full-screen access from a swapchain, call:
///```c
///// Provided by VK_EXT_full_screen_exclusive
///VkResult vkReleaseFullScreenExclusiveModeEXT(
///    VkDevice                                    device,
///    VkSwapchainKHR                              swapchain);
///```
///# Parameters
/// - [`device`] is the device associated with [`swapchain`].
/// - [`swapchain`] is the swapchain to release exclusive full-screen access from.
///# Description
///## Valid Usage
/// - [`swapchain`] **must**  not be in the retired state
/// - [`swapchain`] **must**  be a swapchain created with a [`SurfaceFullScreenExclusiveInfoEXT`]
///   structure, with `fullScreenExclusive` set to
///   `VK_FULL_SCREEN_EXCLUSIVE_APPLICATION_CONTROLLED_EXT`
///# Related
/// - [`VK_EXT_full_screen_exclusive`]
/// - [`Device`]
/// - [`SwapchainKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkReleaseFullScreenExclusiveModeEXT")]
pub type FNReleaseFullScreenExclusiveModeExt =
    Option<unsafe extern "system" fn(device: Device, swapchain: SwapchainKHR) -> VulkanResultCodes>;
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
/// - [`DEFAULT`] indicates the implementation  **should**  determine the appropriate full-screen
///   method by whatever means it deems appropriate.
/// - [`ALLOWED`] indicates the implementation  **may**  use full-screen exclusive mechanisms when
///   available. Such mechanisms  **may**  result in better performance and/or the availability of
///   different presentation capabilities, but  **may**  require a more disruptive transition during
///   swapchain initialization, first presentation and/or destruction.
/// - [`DISALLOWED`] indicates the implementation  **should**  avoid using full-screen mechanisms
///   which rely on disruptive transitions.
/// - [`APPLICATION_CONTROLLED`] indicates the application will manage full-screen exclusive mode by
///   using the [`acquire_full_screen_exclusive_mode_ext`] and
///   [`release_full_screen_exclusive_mode_ext`] commands.
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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct FullScreenExclusiveEXT(i32);
impl const Default for FullScreenExclusiveEXT {
    fn default() -> Self {
        Self(0)
    }
}
impl FullScreenExclusiveEXT {
    ///[`DEFAULT`] indicates the implementation
    /// **should**  determine the appropriate full-screen method by whatever means
    ///it deems appropriate.
    pub const DEFAULT: Self = Self(0);
    ///[`ALLOWED`] indicates the implementation
    /// **may**  use full-screen exclusive mechanisms when available.
    ///Such mechanisms  **may**  result in better performance and/or the
    ///availability of different presentation capabilities, but  **may**  require a
    ///more disruptive transition during swapchain initialization, first
    ///presentation and/or destruction.
    pub const ALLOWED: Self = Self(1);
    ///[`DISALLOWED`] indicates the
    ///implementation  **should**  avoid using full-screen mechanisms which rely on
    ///disruptive transitions.
    pub const DISALLOWED: Self = Self(2);
    ///[`APPLICATION_CONTROLLED`] indicates the
    ///application will manage full-screen exclusive mode by using the
    ///[`acquire_full_screen_exclusive_mode_ext`] and
    ///[`release_full_screen_exclusive_mode_ext`] commands.
    pub const APPLICATION_CONTROLLED: Self = Self(3);
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
impl std::fmt::Debug for FullScreenExclusiveEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple(stringify!(FullScreenExclusiveEXT))
            .field(match *self {
                Self::DEFAULT => &"DEFAULT",
                Self::ALLOWED => &"ALLOWED",
                Self::DISALLOWED => &"DISALLOWED",
                Self::APPLICATION_CONTROLLED => &"APPLICATION_CONTROLLED",
                other => unreachable!(
                    concat!("invalid value for", stringify!(FullScreenExclusiveEXT), ": {:?}"),
                    other
                ),
            })
            .finish()
    }
}
impl std::fmt::Display for FullScreenExclusiveEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str(match *self {
            Self::DEFAULT => &"DEFAULT",
            Self::ALLOWED => &"ALLOWED",
            Self::DISALLOWED => &"DISALLOWED",
            Self::APPLICATION_CONTROLLED => &"APPLICATION_CONTROLLED",
            other => unreachable!(
                concat!("invalid value for", stringify!(FullScreenExclusiveEXT), ": {:?}"),
                other
            ),
        })
    }
}
///[VkSurfaceFullScreenExclusiveInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSurfaceFullScreenExclusiveInfoEXT.html) - Structure specifying the preferred full-screen transition behavior
///# C Specifications
///If the [`p_next`] chain of [`SwapchainCreateInfoKHR`] includes a
///[`SurfaceFullScreenExclusiveInfoEXT`] structure, then that structure
///specifies the application’s preferred full-screen transition behavior.The
/// [`SurfaceFullScreenExclusiveInfoEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_full_screen_exclusive
///typedef struct VkSurfaceFullScreenExclusiveInfoEXT {
///    VkStructureType             sType;
///    void*                       pNext;
///    VkFullScreenExclusiveEXT    fullScreenExclusive;
///} VkSurfaceFullScreenExclusiveInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`full_screen_exclusive`] is a [`FullScreenExclusiveEXT`] value specifying the preferred
///   full-screen transition behavior.
///# Description
///If this structure is not present, [`full_screen_exclusive`] is considered to
///be `VK_FULL_SCREEN_EXCLUSIVE_DEFAULT_EXT`.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_SURFACE_FULL_SCREEN_EXCLUSIVE_INFO_EXT`
/// - [`full_screen_exclusive`] **must**  be a valid [`FullScreenExclusiveEXT`] value
///# Related
/// - [`VK_EXT_full_screen_exclusive`]
/// - [`FullScreenExclusiveEXT`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkSurfaceFullScreenExclusiveInfoEXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct SurfaceFullScreenExclusiveInfoEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`full_screen_exclusive`] is a [`FullScreenExclusiveEXT`] value
    ///specifying the preferred full-screen transition behavior.
    pub full_screen_exclusive: FullScreenExclusiveEXT,
}
impl<'lt> Default for SurfaceFullScreenExclusiveInfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::SURFACE_FULL_SCREEN_EXCLUSIVE_INFO_EXT,
            p_next: std::ptr::null_mut(),
            full_screen_exclusive: Default::default(),
        }
    }
}
impl<'lt> SurfaceFullScreenExclusiveInfoEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *mut BaseOutStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *mut BaseOutStructure<'lt>) -> Self {
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
    pub unsafe fn p_next(&self) -> &BaseOutStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::full_screen_exclusive`]
    pub fn full_screen_exclusive(&self) -> FullScreenExclusiveEXT {
        self.full_screen_exclusive
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next_mut(&mut self) -> &mut BaseOutStructure<'lt> {
        &mut *self.p_next
    }
    ///Gets a mutable reference to the value of [`Self::full_screen_exclusive`]
    pub fn full_screen_exclusive_mut(&mut self) -> &mut FullScreenExclusiveEXT {
        &mut self.full_screen_exclusive
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the value of [`Self::full_screen_exclusive`]
    pub fn set_full_screen_exclusive(
        mut self,
        value: crate::extensions::ext_full_screen_exclusive::FullScreenExclusiveEXT,
    ) -> Self {
        self.full_screen_exclusive = value;
        self
    }
}
///[VkSurfaceFullScreenExclusiveWin32InfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSurfaceFullScreenExclusiveWin32InfoEXT.html) - Structure specifying additional creation parameters specific to Win32 fullscreen exclusive mode
///# C Specifications
///The [`SurfaceFullScreenExclusiveWin32InfoEXT`] structure is defined as:
///```c
///// Provided by VK_KHR_win32_surface with VK_EXT_full_screen_exclusive
///typedef struct VkSurfaceFullScreenExclusiveWin32InfoEXT {
///    VkStructureType    sType;
///    const void*        pNext;
///    HMONITOR           hmonitor;
///} VkSurfaceFullScreenExclusiveWin32InfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`hmonitor`] is the Win32 [`HMONITOR`] handle identifying the display to create the surface
///   with.
///# Description
///## Valid Usage
/// - [`hmonitor`] **must**  be a valid [`HMONITOR`]
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_SURFACE_FULL_SCREEN_EXCLUSIVE_WIN32_INFO_EXT`
///# Related
/// - [`VK_EXT_full_screen_exclusive`]
/// - [`VK_KHR_win32_surface`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkSurfaceFullScreenExclusiveWin32InfoEXT")]
#[repr(C)]
pub struct SurfaceFullScreenExclusiveWin32InfoEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`hmonitor`] is the Win32 [`HMONITOR`] handle identifying the display
    ///to create the surface with.
    pub hmonitor: HMONITOR,
}
impl<'lt> Default for SurfaceFullScreenExclusiveWin32InfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::SURFACE_FULL_SCREEN_EXCLUSIVE_WIN32_INFO_EXT,
            p_next: std::ptr::null(),
            hmonitor: unsafe { std::mem::zeroed() },
        }
    }
}
impl<'lt> SurfaceFullScreenExclusiveWin32InfoEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::hmonitor`]
    pub fn hmonitor_raw(&self) -> &HMONITOR {
        &self.hmonitor
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::hmonitor`]
    pub fn set_hmonitor_raw(mut self, value: HMONITOR) -> Self {
        self.hmonitor = value;
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
    ///Gets the value of [`Self::hmonitor`]
    pub fn hmonitor(&self) -> HMONITOR {
        self.hmonitor
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::hmonitor`]
    pub fn hmonitor_mut(&mut self) -> &mut HMONITOR {
        &mut self.hmonitor
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
    ///Sets the value of [`Self::hmonitor`]
    pub fn set_hmonitor(mut self, value: crate::native::HMONITOR) -> Self {
        self.hmonitor = value;
        self
    }
}
///[VkSurfaceCapabilitiesFullScreenExclusiveEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSurfaceCapabilitiesFullScreenExclusiveEXT.html) - Structure describing full screen exclusive capabilities of a surface
///# C Specifications
///The [`SurfaceCapabilitiesFullScreenExclusiveEXT`] structure is defined
///as:
///```c
///// Provided by VK_EXT_full_screen_exclusive
///typedef struct VkSurfaceCapabilitiesFullScreenExclusiveEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           fullScreenExclusiveSupported;
///} VkSurfaceCapabilitiesFullScreenExclusiveEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - `fullScreenExclusiveControlSupported` is a boolean describing whether the surface is able to
///   make use of exclusive full-screen access.
///# Description
///This structure  **can**  be included in the [`p_next`] chain of
///[`SurfaceCapabilities2KHR`] to determine support for exclusive
///full-screen access.
///If [`full_screen_exclusive_supported`] is [`FALSE`], it indicates that
///exclusive full-screen access is not obtainable for this surface.Applications  **must**  not
/// attempt to create swapchains with
///`VK_FULL_SCREEN_EXCLUSIVE_APPLICATION_CONTROLLED_EXT` set if
///[`full_screen_exclusive_supported`] is [`FALSE`].
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES_FULL_SCREEN_EXCLUSIVE_EXT`
///# Related
/// - [`VK_EXT_full_screen_exclusive`]
/// - [`Bool32`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkSurfaceCapabilitiesFullScreenExclusiveEXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct SurfaceCapabilitiesFullScreenExclusiveEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///No documentation found
    pub full_screen_exclusive_supported: Bool32,
}
impl<'lt> Default for SurfaceCapabilitiesFullScreenExclusiveEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::SURFACE_CAPABILITIES_FULL_SCREEN_EXCLUSIVE_EXT,
            p_next: std::ptr::null_mut(),
            full_screen_exclusive_supported: 0,
        }
    }
}
impl<'lt> SurfaceCapabilitiesFullScreenExclusiveEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *mut BaseOutStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::full_screen_exclusive_supported`]
    pub fn full_screen_exclusive_supported_raw(&self) -> Bool32 {
        self.full_screen_exclusive_supported
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *mut BaseOutStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::full_screen_exclusive_supported`]
    pub fn set_full_screen_exclusive_supported_raw(mut self, value: Bool32) -> Self {
        self.full_screen_exclusive_supported = value;
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
    pub unsafe fn p_next(&self) -> &BaseOutStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::full_screen_exclusive_supported`]
    pub fn full_screen_exclusive_supported(&self) -> bool {
        unsafe { std::mem::transmute(self.full_screen_exclusive_supported as u8) }
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next_mut(&mut self) -> &mut BaseOutStructure<'lt> {
        &mut *self.p_next
    }
    ///Gets a mutable reference to the value of [`Self::full_screen_exclusive_supported`]
    pub fn full_screen_exclusive_supported_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.full_screen_exclusive_supported as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.full_screen_exclusive_supported as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the value of [`Self::full_screen_exclusive_supported`]
    pub fn set_full_screen_exclusive_supported(mut self, value: bool) -> Self {
        self.full_screen_exclusive_supported = value as u8 as u32;
        self
    }
}
impl PhysicalDevice {
    ///[vkGetPhysicalDeviceSurfacePresentModes2EXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfacePresentModes2EXT.html) - Query supported presentation modes
    ///# C Specifications
    ///Alternatively, to query the supported presentation modes for a surface
    ///combined with select other fixed swapchain creation parameters, call:
    ///```c
    ///// Provided by VK_EXT_full_screen_exclusive
    ///VkResult vkGetPhysicalDeviceSurfacePresentModes2EXT(
    ///    VkPhysicalDevice                            physicalDevice,
    ///    const VkPhysicalDeviceSurfaceInfo2KHR*      pSurfaceInfo,
    ///    uint32_t*                                   pPresentModeCount,
    ///    VkPresentModeKHR*                           pPresentModes);
    ///```
    ///# Parameters
    /// - [`physical_device`] is the physical device that will be associated with the swapchain to
    ///   be created, as described for [`create_swapchain_khr`].
    /// - [`p_surface_info`] is a pointer to a [`PhysicalDeviceSurfaceInfo2KHR`] structure
    ///   describing the surface and other fixed parameters that would be consumed by
    ///   [`create_swapchain_khr`].
    /// - [`p_present_mode_count`] is a pointer to an integer related to the number of presentation
    ///   modes available or queried, as described below.
    /// - [`p_present_modes`] is either `NULL` or a pointer to an array of [`PresentModeKHR`]
    ///   values, indicating the supported presentation modes.
    ///# Description
    ///[`get_physical_device_surface_present_modes2_ext`] behaves similarly to
    ///[`get_physical_device_surface_present_modes_khr`], with the ability to specify
    ///extended inputs via chained input structures.
    ///## Valid Usage
    /// - If the `[`VK_GOOGLE_surfaceless_query`]` extension is not enabled, `pSurfaceInfo->surface`
    ///   **must**  be a valid [`SurfaceKHR`] handle
    /// - If `pSurfaceInfo->surface` is not [`crate::Handle::null`], it  **must**  be supported by
    ///   [`physical_device`], as reported by [`get_physical_device_surface_support_khr`] or an
    ///   equivalent platform-specific mechanism
    ///
    ///## Valid Usage (Implicit)
    /// - [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
    /// - [`p_surface_info`] **must**  be a valid pointer to a valid
    ///   [`PhysicalDeviceSurfaceInfo2KHR`] structure
    /// - [`p_present_mode_count`] **must**  be a valid pointer to a `uint32_t` value
    /// - If the value referenced by [`p_present_mode_count`] is not `0`, and [`p_present_modes`] is
    ///   not `NULL`, [`p_present_modes`] **must**  be a valid pointer to an array of
    ///   [`p_present_mode_count`][`PresentModeKHR`] values
    ///
    ///## Return Codes
    /// * - `VK_SUCCESS`  - `VK_INCOMPLETE`
    /// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  -
    ///   `VK_ERROR_SURFACE_LOST_KHR`
    ///# Related
    /// - [`VK_EXT_full_screen_exclusive`]
    /// - [`PhysicalDevice`]
    /// - [`PhysicalDeviceSurfaceInfo2KHR`]
    /// - [`PresentModeKHR`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkGetPhysicalDeviceSurfacePresentModes2EXT")]
    #[track_caller]
    #[inline]
    pub unsafe fn get_physical_device_surface_present_modes2_ext<'lt>(
        self: &Unique<PhysicalDevice>,
        p_surface_info: &PhysicalDeviceSurfaceInfo2KHR<'lt>,
        p_present_mode_count: Option<usize>,
    ) -> VulkanResult<SmallVec<PresentModeKHR>> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .instance()
            .vtable()
            .ext_full_screen_exclusive()
            .and_then(|vtable| vtable.get_physical_device_surface_present_modes2_ext())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .instance()
            .vtable()
            .ext_full_screen_exclusive()
            .and_then(|vtable| vtable.get_physical_device_surface_present_modes2_ext())
            .unwrap_unchecked();
        let mut p_present_mode_count = match p_present_mode_count {
            Some(v) => v as _,
            None => {
                let mut v = 0;
                _function(
                    self.as_raw(),
                    p_surface_info as *const PhysicalDeviceSurfaceInfo2KHR<'lt>,
                    &mut v,
                    std::ptr::null_mut(),
                );
                v
            },
        };
        let mut p_present_modes =
            SmallVec::<PresentModeKHR>::from_elem(Default::default(), p_present_mode_count as usize);
        let _return = _function(
            self.as_raw(),
            p_surface_info as *const PhysicalDeviceSurfaceInfo2KHR<'lt>,
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
impl Device {
    ///[vkGetDeviceGroupSurfacePresentModes2EXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceGroupSurfacePresentModes2EXT.html) - Query device group present capabilities for a surface
    ///# C Specifications
    ///Alternatively, to query the supported device group presentation modes for a
    ///surface combined with select other fixed swapchain creation parameters,
    ///call:
    ///```c
    ///// Provided by VK_VERSION_1_1 with VK_EXT_full_screen_exclusive, VK_KHR_device_group with
    ///// VK_EXT_full_screen_exclusive
    ///VkResult vkGetDeviceGroupSurfacePresentModes2EXT(
    ///    VkDevice                                    device,
    ///    const VkPhysicalDeviceSurfaceInfo2KHR*      pSurfaceInfo,
    ///    VkDeviceGroupPresentModeFlagsKHR*           pModes);
    ///```
    ///# Parameters
    /// - [`device`] is the logical device.
    /// - [`p_surface_info`] is a pointer to a [`PhysicalDeviceSurfaceInfo2KHR`] structure
    ///   describing the surface and other fixed parameters that would be consumed by
    ///   [`create_swapchain_khr`].
    /// - [`p_modes`] is a pointer to a [`DeviceGroupPresentModeFlagsKHR`] in which the supported
    ///   device group present modes for the surface are returned.
    ///# Description
    ///[`get_device_group_surface_present_modes2_ext`] behaves similarly to
    ///[`get_device_group_surface_present_modes_khr`], with the ability to specify
    ///extended inputs via chained input structures.
    ///## Valid Usage
    /// - `pSurfaceInfo->surface` **must**  be supported by all physical devices associated with
    ///   [`device`], as reported by [`get_physical_device_surface_support_khr`] or an equivalent
    ///   platform-specific mechanism
    ///
    ///## Valid Usage (Implicit)
    /// - [`device`] **must**  be a valid [`Device`] handle
    /// - [`p_surface_info`] **must**  be a valid pointer to a valid
    ///   [`PhysicalDeviceSurfaceInfo2KHR`] structure
    /// - [`p_modes`] **must**  be a valid pointer to a [`DeviceGroupPresentModeFlagsKHR`] value
    ///
    ///## Return Codes
    /// * - `VK_SUCCESS`
    /// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  -
    ///   `VK_ERROR_SURFACE_LOST_KHR`
    ///# Related
    /// - [`VK_EXT_full_screen_exclusive`]
    /// - [`VK_KHR_device_group`]
    /// - [`crate::vulkan1_1`]
    /// - [`Device`]
    /// - [`DeviceGroupPresentModeFlagsKHR`]
    /// - [`PhysicalDeviceSurfaceInfo2KHR`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkGetDeviceGroupSurfacePresentModes2EXT")]
    #[track_caller]
    #[inline]
    pub unsafe fn get_device_group_surface_present_modes2_ext<'lt>(
        self: &Unique<Device>,
        p_surface_info: &PhysicalDeviceSurfaceInfo2KHR<'lt>,
        p_modes: &mut DeviceGroupPresentModeFlagsKHR,
    ) -> VulkanResult<()> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .ext_full_screen_exclusive()
            .and_then(|vtable| vtable.get_device_group_surface_present_modes2_ext())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .ext_full_screen_exclusive()
            .and_then(|vtable| vtable.get_device_group_surface_present_modes2_ext())
            .unwrap_unchecked();
        let _return = _function(
            self.as_raw(),
            p_surface_info as *const PhysicalDeviceSurfaceInfo2KHR<'lt>,
            p_modes as *mut DeviceGroupPresentModeFlagsKHR,
        );
        match _return {
            VulkanResultCodes::SUCCESS => VulkanResult::Success(_return, ()),
            e => VulkanResult::Err(e),
        }
    }
}
impl Device {
    ///[vkAcquireFullScreenExclusiveModeEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAcquireFullScreenExclusiveModeEXT.html) - Acquire full-screen exclusive mode for a swapchain
    ///# C Specifications
    ///To acquire exclusive full-screen access for a swapchain, call:
    ///```c
    ///// Provided by VK_EXT_full_screen_exclusive
    ///VkResult vkAcquireFullScreenExclusiveModeEXT(
    ///    VkDevice                                    device,
    ///    VkSwapchainKHR                              swapchain);
    ///```
    ///# Parameters
    /// - [`device`] is the device associated with [`swapchain`].
    /// - [`swapchain`] is the swapchain to acquire exclusive full-screen access for.
    ///# Description
    ///## Valid Usage
    /// - [`swapchain`] **must**  not be in the retired state
    /// - [`swapchain`] **must**  be a swapchain created with a
    ///   [`SurfaceFullScreenExclusiveInfoEXT`] structure, with `fullScreenExclusive` set to
    ///   `VK_FULL_SCREEN_EXCLUSIVE_APPLICATION_CONTROLLED_EXT`
    /// - [`swapchain`] **must**  not currently have exclusive full-screen access
    ///A return value of `VK_SUCCESS` indicates that the [`swapchain`]
    ///successfully acquired exclusive full-screen access.
    ///The swapchain will retain this exclusivity until either the application
    ///releases exclusive full-screen access with
    ///[`release_full_screen_exclusive_mode_ext`], destroys the swapchain, or if any
    ///of the swapchain commands return
    ///`VK_ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT` indicating that the mode
    ///was lost because of platform-specific changes.If the swapchain was unable to acquire
    /// exclusive full-screen access to the
    ///display then `VK_ERROR_INITIALIZATION_FAILED` is returned.
    ///An application  **can**  attempt to acquire exclusive full-screen access again
    ///for the same swapchain even if this command fails, or if
    ///`VK_ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT` has been returned by a
    ///swapchain command.
    ///## Valid Usage (Implicit)
    /// - [`device`] **must**  be a valid [`Device`] handle
    /// - [`swapchain`] **must**  be a valid [`SwapchainKHR`] handle
    /// - Both of [`device`], and [`swapchain`] **must**  have been created, allocated, or retrieved
    ///   from the same [`Instance`]
    ///
    ///## Return Codes
    /// * - `VK_SUCCESS`
    /// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  -
    ///   `VK_ERROR_INITIALIZATION_FAILED`  - `VK_ERROR_SURFACE_LOST_KHR`
    ///# Related
    /// - [`VK_EXT_full_screen_exclusive`]
    /// - [`Device`]
    /// - [`SwapchainKHR`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkAcquireFullScreenExclusiveModeEXT")]
    #[track_caller]
    #[inline]
    pub unsafe fn acquire_full_screen_exclusive_mode_ext(
        self: &Unique<Device>,
        swapchain: SwapchainKHR,
    ) -> VulkanResult<()> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .ext_full_screen_exclusive()
            .and_then(|vtable| vtable.acquire_full_screen_exclusive_mode_ext())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .ext_full_screen_exclusive()
            .and_then(|vtable| vtable.acquire_full_screen_exclusive_mode_ext())
            .unwrap_unchecked();
        let _return = _function(self.as_raw(), swapchain);
        match _return {
            VulkanResultCodes::SUCCESS => VulkanResult::Success(_return, ()),
            e => VulkanResult::Err(e),
        }
    }
}
impl Device {
    ///[vkReleaseFullScreenExclusiveModeEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkReleaseFullScreenExclusiveModeEXT.html) - Release full-screen exclusive mode from a swapchain
    ///# C Specifications
    ///To release exclusive full-screen access from a swapchain, call:
    ///```c
    ///// Provided by VK_EXT_full_screen_exclusive
    ///VkResult vkReleaseFullScreenExclusiveModeEXT(
    ///    VkDevice                                    device,
    ///    VkSwapchainKHR                              swapchain);
    ///```
    ///# Parameters
    /// - [`device`] is the device associated with [`swapchain`].
    /// - [`swapchain`] is the swapchain to release exclusive full-screen access from.
    ///# Description
    ///## Valid Usage
    /// - [`swapchain`] **must**  not be in the retired state
    /// - [`swapchain`] **must**  be a swapchain created with a
    ///   [`SurfaceFullScreenExclusiveInfoEXT`] structure, with `fullScreenExclusive` set to
    ///   `VK_FULL_SCREEN_EXCLUSIVE_APPLICATION_CONTROLLED_EXT`
    ///# Related
    /// - [`VK_EXT_full_screen_exclusive`]
    /// - [`Device`]
    /// - [`SwapchainKHR`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkReleaseFullScreenExclusiveModeEXT")]
    #[track_caller]
    #[inline]
    pub unsafe fn release_full_screen_exclusive_mode_ext(
        self: &Unique<Device>,
        swapchain: SwapchainKHR,
    ) -> VulkanResult<()> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .ext_full_screen_exclusive()
            .and_then(|vtable| vtable.release_full_screen_exclusive_mode_ext())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .ext_full_screen_exclusive()
            .and_then(|vtable| vtable.release_full_screen_exclusive_mode_ext())
            .unwrap_unchecked();
        let _return = _function(self.as_raw(), swapchain);
        match _return {
            VulkanResultCodes::SUCCESS => VulkanResult::Success(_return, ()),
            e => VulkanResult::Err(e),
        }
    }
}
///The V-table of [`Instance`] for functions from `VK_EXT_full_screen_exclusive`
pub struct InstanceExtFullScreenExclusiveVTable {
    ///See [`FNGetPhysicalDeviceSurfacePresentModes2Ext`] for more information.
    pub get_physical_device_surface_present_modes2_ext: FNGetPhysicalDeviceSurfacePresentModes2Ext,
}
impl InstanceExtFullScreenExclusiveVTable {
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
            get_physical_device_surface_present_modes2_ext: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkGetPhysicalDeviceSurfacePresentModes2EXT").as_ptr(),
                ))
            },
        }
    }
    ///Gets [`Self::get_physical_device_surface_present_modes2_ext`]. See
    /// [`FNGetPhysicalDeviceSurfacePresentModes2Ext`] for more information.
    pub fn get_physical_device_surface_present_modes2_ext(&self) -> FNGetPhysicalDeviceSurfacePresentModes2Ext {
        self.get_physical_device_surface_present_modes2_ext
    }
}
///The V-table of [`Device`] for functions from `VK_EXT_full_screen_exclusive`
pub struct DeviceExtFullScreenExclusiveVTable {
    ///See [`FNGetDeviceGroupSurfacePresentModes2Ext`] for more information.
    pub get_device_group_surface_present_modes2_ext: FNGetDeviceGroupSurfacePresentModes2Ext,
    ///See [`FNAcquireFullScreenExclusiveModeExt`] for more information.
    pub acquire_full_screen_exclusive_mode_ext: FNAcquireFullScreenExclusiveModeExt,
    ///See [`FNReleaseFullScreenExclusiveModeExt`] for more information.
    pub release_full_screen_exclusive_mode_ext: FNReleaseFullScreenExclusiveModeExt,
}
impl DeviceExtFullScreenExclusiveVTable {
    ///Loads the VTable from the owner and the names
    #[track_caller]
    pub fn load(
        loader_fn: unsafe extern "system" fn(
            Device,
            *const std::os::raw::c_char,
        ) -> Option<unsafe extern "system" fn()>,
        loader: Device,
    ) -> Self {
        Self {
            get_device_group_surface_present_modes2_ext: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkGetDeviceGroupSurfacePresentModes2EXT").as_ptr(),
                ))
            },
            acquire_full_screen_exclusive_mode_ext: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkAcquireFullScreenExclusiveModeEXT").as_ptr(),
                ))
            },
            release_full_screen_exclusive_mode_ext: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkReleaseFullScreenExclusiveModeEXT").as_ptr(),
                ))
            },
        }
    }
    ///Gets [`Self::get_device_group_surface_present_modes2_ext`]. See
    /// [`FNGetDeviceGroupSurfacePresentModes2Ext`] for more information.
    pub fn get_device_group_surface_present_modes2_ext(&self) -> FNGetDeviceGroupSurfacePresentModes2Ext {
        self.get_device_group_surface_present_modes2_ext
    }
    ///Gets [`Self::acquire_full_screen_exclusive_mode_ext`]. See
    /// [`FNAcquireFullScreenExclusiveModeExt`] for more information.
    pub fn acquire_full_screen_exclusive_mode_ext(&self) -> FNAcquireFullScreenExclusiveModeExt {
        self.acquire_full_screen_exclusive_mode_ext
    }
    ///Gets [`Self::release_full_screen_exclusive_mode_ext`]. See
    /// [`FNReleaseFullScreenExclusiveModeExt`] for more information.
    pub fn release_full_screen_exclusive_mode_ext(&self) -> FNReleaseFullScreenExclusiveModeExt {
        self.release_full_screen_exclusive_mode_ext
    }
}
