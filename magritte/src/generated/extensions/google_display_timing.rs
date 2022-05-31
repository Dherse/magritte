//![VK_GOOGLE_display_timing](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_GOOGLE_display_timing.html) - device extension
//!# Description
//!This device extension allows an application that uses the
//!`[`khr_swapchain`]` extension to obtain information about the
//!presentation engine’s display, to obtain timing information about each
//!present, and to schedule a present to happen no earlier than a desired time.
//!An application can use this to minimize various visual anomalies (e.g.
//!stuttering).Traditional game and real-time animation applications need to correctly
//!position their geometry for when the presentable image will be presented to
//!the user.
//!To accomplish this, applications need various timing information about the
//!presentation engine’s display.
//!They need to know when presentable images were actually presented, and when
//!they could have been presented.
//!Applications also need to tell the presentation engine to display an image
//!no sooner than a given time.
//!This allows the application to avoid stuttering, so the animation looks
//!smooth to the user.This extension treats variable-refresh-rate (VRR) displays as if they are
//!fixed-refresh-rate (FRR) displays.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`khr_swapchain`]`
//!# Contacts
//! - Ian Elliott [ianelliottus](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_GOOGLE_display_timing]
//!   @ianelliottus%0A<<Here describe the issue or question you have about the
//!   VK_GOOGLE_display_timing extension>>)
//!# New functions & commands
//! - [`get_past_presentation_timing_google`]
//! - [`get_refresh_cycle_duration_google`]
//!# New structures
//! - [`PastPresentationTimingGOOGLE`]
//! - [`PresentTimeGOOGLE`]
//! - [`RefreshCycleDurationGOOGLE`]
//! - Extending [`PresentInfoKHR`]:  - [`PresentTimesInfoGOOGLE`]
//!# New constants
//! - [`GOOGLE_DISPLAY_TIMING_EXTENSION_NAME`]
//! - [`GOOGLE_DISPLAY_TIMING_SPEC_VERSION`]
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_PRESENT_TIMES_INFO_GOOGLE`
//!# Version History
//! - Revision 1, 2017-02-14 (Ian Elliott)  - Internal revisions
//!# Other info
//! * 2017-02-14
//! * No known IP claims.
//! * - Ian Elliott, Google  - Jesse Hall, Google
//!# Related
//! - [`PastPresentationTimingGOOGLE`]
//! - [`PresentTimeGOOGLE`]
//! - [`PresentTimesInfoGOOGLE`]
//! - [`RefreshCycleDurationGOOGLE`]
//! - [`get_past_presentation_timing_google`]
//! - [`get_refresh_cycle_duration_google`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::{
    extensions::khr_swapchain::SwapchainKHR,
    vulkan1_0::{BaseInStructure, Device, StructureType, VulkanResultCodes},
    AsRaw, SmallVec, Unique, VulkanResult,
};
use std::{ffi::CStr, marker::PhantomData, mem::MaybeUninit};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_GOOGLE_DISPLAY_TIMING_SPEC_VERSION")]
pub const GOOGLE_DISPLAY_TIMING_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_GOOGLE_DISPLAY_TIMING_EXTENSION_NAME")]
pub const GOOGLE_DISPLAY_TIMING_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_GOOGLE_display_timing");
///[vkGetRefreshCycleDurationGOOGLE](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetRefreshCycleDurationGOOGLE.html) - Obtain the RC duration of the PE's display
///# C Specifications
///To query the duration of a refresh cycle (RC) for the presentation engine’s
///display, call:
///```c
///// Provided by VK_GOOGLE_display_timing
///VkResult vkGetRefreshCycleDurationGOOGLE(
///    VkDevice                                    device,
///    VkSwapchainKHR                              swapchain,
///    VkRefreshCycleDurationGOOGLE*               pDisplayTimingProperties);
///```
/// # Parameters
/// - [`device`] is the device associated with [`swapchain`].
/// - [`swapchain`] is the swapchain to obtain the refresh duration for.
/// - [`p_display_timing_properties`] is a pointer to a [`RefreshCycleDurationGOOGLE`] structure.
/// # Description
/// ## Valid Usage (Implicit)
/// - [`device`] **must**  be a valid [`Device`] handle
/// - [`swapchain`] **must**  be a valid [`SwapchainKHR`] handle
/// - [`p_display_timing_properties`] **must**  be a valid pointer to a
///   [`RefreshCycleDurationGOOGLE`] structure
/// - Both of [`device`], and [`swapchain`] **must**  have been created, allocated, or retrieved
///   from the same [`Instance`]
///
/// ## Host Synchronization
/// - Host access to [`swapchain`] **must**  be externally synchronized
///
/// ## Return Codes
/// * - `VK_SUCCESS`
/// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_DEVICE_LOST`  - `VK_ERROR_SURFACE_LOST_KHR`
/// # Related
/// - [`google_display_timing`]
/// - [`Device`]
/// - [`RefreshCycleDurationGOOGLE`]
/// - [`SwapchainKHR`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkGetRefreshCycleDurationGOOGLE")]
pub type FNGetRefreshCycleDurationGoogle = Option<
    unsafe extern "system" fn(
        device: Device,
        swapchain: SwapchainKHR,
        p_display_timing_properties: *mut RefreshCycleDurationGOOGLE,
    ) -> VulkanResultCodes,
>;
///[vkGetPastPresentationTimingGOOGLE](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPastPresentationTimingGOOGLE.html) - Obtain timing of a previously-presented image
///# C Specifications
///The implementation will maintain a limited amount of history of timing
///information about previous presents.
///Because of the asynchronous nature of the presentation engine, the timing
///information for a given [`queue_present_khr`] command will become
///available some time later.
///These time values can be asynchronously queried, and will be returned if
///available.
///All time values are in nanoseconds, relative to a monotonically-increasing
///clock (e.g. `CLOCK_MONOTONIC` (see clock_gettime(2)) on Android and Linux).To asynchronously
/// query the presentation engine, for newly-available timing
///information about one or more previous presents to a given swapchain, call:
///```c
///// Provided by VK_GOOGLE_display_timing
///VkResult vkGetPastPresentationTimingGOOGLE(
///    VkDevice                                    device,
///    VkSwapchainKHR                              swapchain,
///    uint32_t*                                   pPresentationTimingCount,
///    VkPastPresentationTimingGOOGLE*             pPresentationTimings);
///```
/// # Parameters
/// - [`device`] is the device associated with [`swapchain`].
/// - [`swapchain`] is the swapchain to obtain presentation timing information duration for.
/// - [`p_presentation_timing_count`] is a pointer to an integer related to the number of
///   [`PastPresentationTimingGOOGLE`] structures to query, as described below.
/// - [`p_presentation_timings`] is either `NULL` or a pointer to an array of
///   [`PastPresentationTimingGOOGLE`] structures.
/// # Description
/// If [`p_presentation_timings`] is `NULL`, then the number of newly-available
/// timing records for the given [`swapchain`] is returned in
/// [`p_presentation_timing_count`].
/// Otherwise, [`p_presentation_timing_count`] **must**  point to a variable set by
/// the user to the number of elements in the [`p_presentation_timings`] array,
/// and on return the variable is overwritten with the number of structures
/// actually written to [`p_presentation_timings`].
/// If the value of [`p_presentation_timing_count`] is less than the number of
/// newly-available timing records, at most [`p_presentation_timing_count`]
/// structures will be written, and `VK_INCOMPLETE` will be returned instead
/// of `VK_SUCCESS`, to indicate that not all the available timing records
/// were returned.
/// ## Valid Usage (Implicit)
/// - [`device`] **must**  be a valid [`Device`] handle
/// - [`swapchain`] **must**  be a valid [`SwapchainKHR`] handle
/// - [`p_presentation_timing_count`] **must**  be a valid pointer to a `uint32_t` value
/// - If the value referenced by [`p_presentation_timing_count`] is not `0`, and
///   [`p_presentation_timings`] is not `NULL`, [`p_presentation_timings`] **must**  be a valid
///   pointer to an array of [`p_presentation_timing_count`][`PastPresentationTimingGOOGLE`]
///   structures
/// - Both of [`device`], and [`swapchain`] **must**  have been created, allocated, or retrieved
///   from the same [`Instance`]
///
/// ## Host Synchronization
/// - Host access to [`swapchain`] **must**  be externally synchronized
///
/// ## Return Codes
/// * - `VK_SUCCESS`  - `VK_INCOMPLETE`
/// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_DEVICE_LOST`  - `VK_ERROR_OUT_OF_DATE_KHR`  -
///   `VK_ERROR_SURFACE_LOST_KHR`
/// # Related
/// - [`google_display_timing`]
/// - [`Device`]
/// - [`PastPresentationTimingGOOGLE`]
/// - [`SwapchainKHR`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkGetPastPresentationTimingGOOGLE")]
pub type FNGetPastPresentationTimingGoogle = Option<
    unsafe extern "system" fn(
        device: Device,
        swapchain: SwapchainKHR,
        p_presentation_timing_count: *mut u32,
        p_presentation_timings: *mut PastPresentationTimingGOOGLE,
    ) -> VulkanResultCodes,
>;
///[VkRefreshCycleDurationGOOGLE](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRefreshCycleDurationGOOGLE.html) - Structure containing the RC duration of a display
///# C Specifications
///The [`RefreshCycleDurationGOOGLE`] structure is defined as:
///```c
///// Provided by VK_GOOGLE_display_timing
///typedef struct VkRefreshCycleDurationGOOGLE {
///    uint64_t    refreshDuration;
///} VkRefreshCycleDurationGOOGLE;
///```
/// # Members
/// - [`refresh_duration`] is the number of nanoseconds from the start of one refresh cycle to the
///   next.
/// # Related
/// - [`google_display_timing`]
/// - [`get_refresh_cycle_duration_google`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkRefreshCycleDurationGOOGLE")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct RefreshCycleDurationGOOGLE {
    ///[`refresh_duration`] is the number of nanoseconds from the start of one
    ///refresh cycle to the next.
    pub refresh_duration: u64,
}
impl Default for RefreshCycleDurationGOOGLE {
    fn default() -> Self {
        Self { refresh_duration: 0 }
    }
}
impl RefreshCycleDurationGOOGLE {
    ///Gets the value of [`Self::refresh_duration`]
    pub fn refresh_duration(&self) -> u64 {
        self.refresh_duration
    }
    ///Gets a mutable reference to the value of [`Self::refresh_duration`]
    pub fn refresh_duration_mut(&mut self) -> &mut u64 {
        &mut self.refresh_duration
    }
    ///Sets the value of [`Self::refresh_duration`]
    pub fn set_refresh_duration(&mut self, value: u64) -> &mut Self {
        self.refresh_duration = value;
        self
    }
    ///Sets the value of [`Self::refresh_duration`]
    pub fn with_refresh_duration(mut self, value: u64) -> Self {
        self.refresh_duration = value;
        self
    }
}
///[VkPastPresentationTimingGOOGLE](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPastPresentationTimingGOOGLE.html) - Structure containing timing information about a previously-presented image
///# C Specifications
///The [`PastPresentationTimingGOOGLE`] structure is defined as:
///```c
///// Provided by VK_GOOGLE_display_timing
///typedef struct VkPastPresentationTimingGOOGLE {
///    uint32_t    presentID;
///    uint64_t    desiredPresentTime;
///    uint64_t    actualPresentTime;
///    uint64_t    earliestPresentTime;
///    uint64_t    presentMargin;
///} VkPastPresentationTimingGOOGLE;
///```
/// # Members
/// - [`present_id`] is an application-provided value that was given to a previous
///   [`queue_present_khr`] command via [`PresentTimeGOOGLE`]::[`present_id`] (see below). It
///   **can**  be used to uniquely identify a previous present with the [`queue_present_khr`]
///   command.
/// - [`desired_present_time`] is an application-provided value that was given to a previous
///   [`queue_present_khr`] command via [`PresentTimeGOOGLE`]::[`desired_present_time`]. If
///   non-zero, it was used by the application to indicate that an image not be presented any sooner
///   than [`desired_present_time`].
/// - [`actual_present_time`] is the time when the image of the `swapchain` was actually displayed.
/// - [`earliest_present_time`] is the time when the image of the `swapchain` could have been
///   displayed. This  **may**  differ from [`actual_present_time`] if the application requested
///   that the image be presented no sooner than [`PresentTimeGOOGLE`]::[`desired_present_time`].
/// - [`present_margin`] is an indication of how early the [`queue_present_khr`] command was
///   processed compared to how soon it needed to be processed, and still be presented at
///   [`earliest_present_time`].
/// # Description
/// The results for a given `swapchain` and [`present_id`] are only
/// returned once from [`get_past_presentation_timing_google`].The application  **can**  use the
/// [`PastPresentationTimingGOOGLE`] values to
/// occasionally adjust its timing.
/// For example, if [`actual_present_time`] is later than expected (e.g. one
/// `refreshDuration` late), the application may increase its target IPD to
/// a higher multiple of `refreshDuration` (e.g. decrease its frame rate
/// from 60Hz to 30Hz).
/// If [`actual_present_time`] and [`earliest_present_time`] are consistently
/// different, and if [`present_margin`] is consistently large enough, the
/// application may decrease its target IPD to a smaller multiple of
/// `refreshDuration` (e.g. increase its frame rate from 30Hz to 60Hz).
/// If [`actual_present_time`] and [`earliest_present_time`] are same, and if
/// [`present_margin`] is consistently high, the application may delay the
/// start of its input-render-present loop in order to decrease the latency
/// between user input and the corresponding present (always leaving some margin
/// in case a new image takes longer to render than the previous image).
/// An application that desires its target IPD to always be the same as
/// `refreshDuration`, can also adjust features until
/// [`actual_present_time`] is never late and [`present_margin`] is
/// satisfactory.
/// # Related
/// - [`google_display_timing`]
/// - [`get_past_presentation_timing_google`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPastPresentationTimingGOOGLE")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct PastPresentationTimingGOOGLE {
    ///[`present_id`] is an application-provided value that was given to a
    ///previous [`queue_present_khr`] command via
    ///[`PresentTimeGOOGLE`]::[`present_id`] (see below).
    ///It  **can**  be used to uniquely identify a previous present with the
    ///[`queue_present_khr`] command.
    pub present_id: u32,
    ///[`desired_present_time`] is an application-provided value that was given
    ///to a previous [`queue_present_khr`] command via
    ///[`PresentTimeGOOGLE`]::[`desired_present_time`].
    ///If non-zero, it was used by the application to indicate that an image
    ///not be presented any sooner than [`desired_present_time`].
    pub desired_present_time: u64,
    ///[`actual_present_time`] is the time when the image of the
    ///`swapchain` was actually displayed.
    pub actual_present_time: u64,
    ///[`earliest_present_time`] is the time when the image of the
    ///`swapchain` could have been displayed.
    ///This  **may**  differ from [`actual_present_time`] if the application
    ///requested that the image be presented no sooner than
    ///[`PresentTimeGOOGLE`]::[`desired_present_time`].
    pub earliest_present_time: u64,
    ///[`present_margin`] is an indication of how early the
    ///[`queue_present_khr`] command was processed compared to how soon it
    ///needed to be processed, and still be presented at
    ///[`earliest_present_time`].
    pub present_margin: u64,
}
impl Default for PastPresentationTimingGOOGLE {
    fn default() -> Self {
        Self {
            present_id: 0,
            desired_present_time: 0,
            actual_present_time: 0,
            earliest_present_time: 0,
            present_margin: 0,
        }
    }
}
impl PastPresentationTimingGOOGLE {
    ///Gets the value of [`Self::present_id`]
    pub fn present_id(&self) -> u32 {
        self.present_id
    }
    ///Gets the value of [`Self::desired_present_time`]
    pub fn desired_present_time(&self) -> u64 {
        self.desired_present_time
    }
    ///Gets the value of [`Self::actual_present_time`]
    pub fn actual_present_time(&self) -> u64 {
        self.actual_present_time
    }
    ///Gets the value of [`Self::earliest_present_time`]
    pub fn earliest_present_time(&self) -> u64 {
        self.earliest_present_time
    }
    ///Gets the value of [`Self::present_margin`]
    pub fn present_margin(&self) -> u64 {
        self.present_margin
    }
    ///Gets a mutable reference to the value of [`Self::present_id`]
    pub fn present_id_mut(&mut self) -> &mut u32 {
        &mut self.present_id
    }
    ///Gets a mutable reference to the value of [`Self::desired_present_time`]
    pub fn desired_present_time_mut(&mut self) -> &mut u64 {
        &mut self.desired_present_time
    }
    ///Gets a mutable reference to the value of [`Self::actual_present_time`]
    pub fn actual_present_time_mut(&mut self) -> &mut u64 {
        &mut self.actual_present_time
    }
    ///Gets a mutable reference to the value of [`Self::earliest_present_time`]
    pub fn earliest_present_time_mut(&mut self) -> &mut u64 {
        &mut self.earliest_present_time
    }
    ///Gets a mutable reference to the value of [`Self::present_margin`]
    pub fn present_margin_mut(&mut self) -> &mut u64 {
        &mut self.present_margin
    }
    ///Sets the value of [`Self::present_id`]
    pub fn set_present_id(&mut self, value: u32) -> &mut Self {
        self.present_id = value;
        self
    }
    ///Sets the value of [`Self::desired_present_time`]
    pub fn set_desired_present_time(&mut self, value: u64) -> &mut Self {
        self.desired_present_time = value;
        self
    }
    ///Sets the value of [`Self::actual_present_time`]
    pub fn set_actual_present_time(&mut self, value: u64) -> &mut Self {
        self.actual_present_time = value;
        self
    }
    ///Sets the value of [`Self::earliest_present_time`]
    pub fn set_earliest_present_time(&mut self, value: u64) -> &mut Self {
        self.earliest_present_time = value;
        self
    }
    ///Sets the value of [`Self::present_margin`]
    pub fn set_present_margin(&mut self, value: u64) -> &mut Self {
        self.present_margin = value;
        self
    }
    ///Sets the value of [`Self::present_id`]
    pub fn with_present_id(mut self, value: u32) -> Self {
        self.present_id = value;
        self
    }
    ///Sets the value of [`Self::desired_present_time`]
    pub fn with_desired_present_time(mut self, value: u64) -> Self {
        self.desired_present_time = value;
        self
    }
    ///Sets the value of [`Self::actual_present_time`]
    pub fn with_actual_present_time(mut self, value: u64) -> Self {
        self.actual_present_time = value;
        self
    }
    ///Sets the value of [`Self::earliest_present_time`]
    pub fn with_earliest_present_time(mut self, value: u64) -> Self {
        self.earliest_present_time = value;
        self
    }
    ///Sets the value of [`Self::present_margin`]
    pub fn with_present_margin(mut self, value: u64) -> Self {
        self.present_margin = value;
        self
    }
}
///[VkPresentTimesInfoGOOGLE](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPresentTimesInfoGOOGLE.html) - The earliest time each image should be presented
///# C Specifications
///When the `[`google_display_timing`]` extension is enabled, additional
///fields  **can**  be specified that allow an application to specify the earliest
///time that an image should be displayed.
///This allows an application to avoid stutter that is caused by an image being
///displayed earlier than planned.
///Such stuttering can occur with both fixed and variable-refresh-rate
///displays, because stuttering occurs when the geometry is not correctly
///positioned for when the image is displayed.
///An application  **can**  instruct the presentation engine that an image should
///not be displayed earlier than a specified time by adding a
///[`PresentTimesInfoGOOGLE`] structure to the [`p_next`] chain of the
///[`PresentInfoKHR`] structure.The [`PresentTimesInfoGOOGLE`] structure is defined as:
///```c
///// Provided by VK_GOOGLE_display_timing
///typedef struct VkPresentTimesInfoGOOGLE {
///    VkStructureType               sType;
///    const void*                   pNext;
///    uint32_t                      swapchainCount;
///    const VkPresentTimeGOOGLE*    pTimes;
///} VkPresentTimesInfoGOOGLE;
///```
/// # Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`swapchain_count`] is the number of swapchains being presented to by this command.
/// - [`times`] is `NULL` or a pointer to an array of [`PresentTimeGOOGLE`] elements with
///   [`swapchain_count`] entries. If not `NULL`, each element of [`times`] contains the earliest
///   time to present the image corresponding to the entry in the [`PresentInfoKHR::image_indices`]
///   array.
/// # Description
/// ## Valid Usage
/// - [`swapchain_count`] **must**  be the same value as [`PresentInfoKHR`]::[`swapchain_count`],
///   where [`PresentInfoKHR`] is included in the [`p_next`] chain of this
///   [`PresentTimesInfoGOOGLE`] structure
///
/// ## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PRESENT_TIMES_INFO_GOOGLE`
/// - If [`times`] is not `NULL`, [`times`] **must**  be a valid pointer to an array of
///   [`swapchain_count`][`PresentTimeGOOGLE`] structures
/// - [`swapchain_count`] **must**  be greater than `0`
/// # Related
/// - [`google_display_timing`]
/// - [`PresentTimeGOOGLE`]
/// - [`StructureType`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPresentTimesInfoGOOGLE")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct PresentTimesInfoGOOGLE<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`swapchain_count`] is the number of swapchains being presented to by
    ///this command.
    pub swapchain_count: u32,
    ///[`times`] is `NULL` or a pointer to an array of
    ///[`PresentTimeGOOGLE`] elements with [`swapchain_count`] entries.
    ///If not `NULL`, each element of [`times`] contains the earliest time
    ///to present the image corresponding to the entry in the
    ///[`PresentInfoKHR`]::`pImageIndices` array.
    pub times: *const PresentTimeGOOGLE,
}
impl<'lt> Default for PresentTimesInfoGOOGLE<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PRESENT_TIMES_INFO_GOOGLE,
            p_next: std::ptr::null(),
            swapchain_count: 0,
            times: std::ptr::null(),
        }
    }
}
impl<'lt> PresentTimesInfoGOOGLE<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::times`]
    pub fn times_raw(&self) -> *const PresentTimeGOOGLE {
        self.times
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::times`]
    pub fn set_times_raw(&mut self, value: *const PresentTimeGOOGLE) -> &mut Self {
        self.times = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::times`]
    pub fn with_times_raw(mut self, value: *const PresentTimeGOOGLE) -> Self {
        self.times = value;
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
    ///Gets the value of [`Self::swapchain_count`]
    pub fn swapchain_count(&self) -> u32 {
        self.swapchain_count
    }
    ///Gets the value of [`Self::times`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn times(&self) -> &[PresentTimeGOOGLE] {
        std::slice::from_raw_parts(self.times, self.swapchain_count as usize)
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::swapchain_count`]
    pub fn swapchain_count_mut(&mut self) -> &mut u32 {
        &mut self.swapchain_count
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::swapchain_count`]
    pub fn set_swapchain_count(&mut self, value: u32) -> &mut Self {
        self.swapchain_count = value;
        self
    }
    ///Sets the value of [`Self::times`]
    pub fn set_times(
        &mut self,
        value: &'lt [crate::extensions::google_display_timing::PresentTimeGOOGLE],
    ) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.times = value.as_ptr();
        self.swapchain_count = len_;
        self
    }
    ///Sets the value of [`Self::s_type`]
    pub fn with_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn with_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::swapchain_count`]
    pub fn with_swapchain_count(mut self, value: u32) -> Self {
        self.swapchain_count = value;
        self
    }
    ///Sets the value of [`Self::times`]
    pub fn with_times(mut self, value: &'lt [crate::extensions::google_display_timing::PresentTimeGOOGLE]) -> Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.times = value.as_ptr();
        self.swapchain_count = len_;
        self
    }
}
///[VkPresentTimeGOOGLE](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPresentTimeGOOGLE.html) - The earliest time image should be presented
///# C Specifications
///The [`PresentTimeGOOGLE`] structure is defined as:
///```c
///// Provided by VK_GOOGLE_display_timing
///typedef struct VkPresentTimeGOOGLE {
///    uint32_t    presentID;
///    uint64_t    desiredPresentTime;
///} VkPresentTimeGOOGLE;
///```
/// # Members
/// - [`present_id`] is an application-provided identification value, that  **can**  be used with
///   the results of [`get_past_presentation_timing_google`], in order to uniquely identify this
///   present. In order to be useful to the application, it  **should**  be unique within some
///   period of time that is meaningful to the application.
/// - [`desired_present_time`] specifies that the image given  **should**  not be displayed to the
///   user any earlier than this time. [`desired_present_time`] is a time in nanoseconds, relative
///   to a monotonically-increasing clock (e.g. `CLOCK_MONOTONIC` (see clock_gettime(2)) on Android
///   and Linux). A value of zero specifies that the presentation engine  **may**  display the image
///   at any time. This is useful when the application desires to provide [`present_id`],
/// # Description
/// ```c but does not need a specific pname:desiredPresentTime.
///```
/// # Related
/// - [`google_display_timing`]
/// - [`PresentTimesInfoGOOGLE`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPresentTimeGOOGLE")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct PresentTimeGOOGLE {
    ///[`present_id`] is an application-provided identification value, that
    /// **can**  be used with the results of
    ///[`get_past_presentation_timing_google`], in order to uniquely identify
    ///this present.
    ///In order to be useful to the application, it  **should**  be unique within
    ///some period of time that is meaningful to the application.
    pub present_id: u32,
    ///[`desired_present_time`] specifies that the image given  **should**  not be
    ///displayed to the user any earlier than this time.
    ///[`desired_present_time`] is a time in nanoseconds, relative to a
    ///monotonically-increasing clock (e.g. `CLOCK_MONOTONIC` (see
    ///clock_gettime(2)) on Android and Linux).
    ///A value of zero specifies that the presentation engine  **may**  display the
    ///image at any time.
    ///This is useful when the application desires to provide [`present_id`],
    pub desired_present_time: u64,
}
impl Default for PresentTimeGOOGLE {
    fn default() -> Self {
        Self {
            present_id: 0,
            desired_present_time: 0,
        }
    }
}
impl PresentTimeGOOGLE {
    ///Gets the value of [`Self::present_id`]
    pub fn present_id(&self) -> u32 {
        self.present_id
    }
    ///Gets the value of [`Self::desired_present_time`]
    pub fn desired_present_time(&self) -> u64 {
        self.desired_present_time
    }
    ///Gets a mutable reference to the value of [`Self::present_id`]
    pub fn present_id_mut(&mut self) -> &mut u32 {
        &mut self.present_id
    }
    ///Gets a mutable reference to the value of [`Self::desired_present_time`]
    pub fn desired_present_time_mut(&mut self) -> &mut u64 {
        &mut self.desired_present_time
    }
    ///Sets the value of [`Self::present_id`]
    pub fn set_present_id(&mut self, value: u32) -> &mut Self {
        self.present_id = value;
        self
    }
    ///Sets the value of [`Self::desired_present_time`]
    pub fn set_desired_present_time(&mut self, value: u64) -> &mut Self {
        self.desired_present_time = value;
        self
    }
    ///Sets the value of [`Self::present_id`]
    pub fn with_present_id(mut self, value: u32) -> Self {
        self.present_id = value;
        self
    }
    ///Sets the value of [`Self::desired_present_time`]
    pub fn with_desired_present_time(mut self, value: u64) -> Self {
        self.desired_present_time = value;
        self
    }
}
impl Device {
    ///[vkGetRefreshCycleDurationGOOGLE](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetRefreshCycleDurationGOOGLE.html) - Obtain the RC duration of the PE's display
    ///# C Specifications
    ///To query the duration of a refresh cycle (RC) for the presentation engine’s
    ///display, call:
    ///```c
    ///// Provided by VK_GOOGLE_display_timing
    ///VkResult vkGetRefreshCycleDurationGOOGLE(
    ///    VkDevice                                    device,
    ///    VkSwapchainKHR                              swapchain,
    ///    VkRefreshCycleDurationGOOGLE*               pDisplayTimingProperties);
    ///```
    /// # Parameters
    /// - [`device`] is the device associated with [`swapchain`].
    /// - [`swapchain`] is the swapchain to obtain the refresh duration for.
    /// - [`p_display_timing_properties`] is a pointer to a [`RefreshCycleDurationGOOGLE`]
    ///   structure.
    /// # Description
    /// ## Valid Usage (Implicit)
    /// - [`device`] **must**  be a valid [`Device`] handle
    /// - [`swapchain`] **must**  be a valid [`SwapchainKHR`] handle
    /// - [`p_display_timing_properties`] **must**  be a valid pointer to a
    ///   [`RefreshCycleDurationGOOGLE`] structure
    /// - Both of [`device`], and [`swapchain`] **must**  have been created, allocated, or retrieved
    ///   from the same [`Instance`]
    ///
    /// ## Host Synchronization
    /// - Host access to [`swapchain`] **must**  be externally synchronized
    ///
    /// ## Return Codes
    /// * - `VK_SUCCESS`
    /// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_DEVICE_LOST`  - `VK_ERROR_SURFACE_LOST_KHR`
    /// # Related
    /// - [`google_display_timing`]
    /// - [`Device`]
    /// - [`RefreshCycleDurationGOOGLE`]
    /// - [`SwapchainKHR`]
    ///
    /// # Notes and documentation
    /// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    /// This documentation is generated from the Vulkan specification and documentation.
    /// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    /// This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkGetRefreshCycleDurationGOOGLE")]
    #[track_caller]
    #[inline]
    pub unsafe fn get_refresh_cycle_duration_google(
        self: &Unique<Device>,
        swapchain: SwapchainKHR,
    ) -> VulkanResult<RefreshCycleDurationGOOGLE> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .google_display_timing()
            .and_then(|vtable| vtable.get_refresh_cycle_duration_google())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .google_display_timing()
            .and_then(|vtable| vtable.get_refresh_cycle_duration_google())
            .unwrap_unchecked();
        let mut p_display_timing_properties = MaybeUninit::<RefreshCycleDurationGOOGLE>::uninit();
        let _return = _function(self.as_raw(), swapchain, p_display_timing_properties.as_mut_ptr());
        match _return {
            VulkanResultCodes::SUCCESS => VulkanResult::Success(_return, p_display_timing_properties.assume_init()),
            e => VulkanResult::Err(e),
        }
    }
}
impl Device {
    ///[vkGetPastPresentationTimingGOOGLE](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPastPresentationTimingGOOGLE.html) - Obtain timing of a previously-presented image
    ///# C Specifications
    ///The implementation will maintain a limited amount of history of timing
    ///information about previous presents.
    ///Because of the asynchronous nature of the presentation engine, the timing
    ///information for a given [`queue_present_khr`] command will become
    ///available some time later.
    ///These time values can be asynchronously queried, and will be returned if
    ///available.
    ///All time values are in nanoseconds, relative to a monotonically-increasing
    ///clock (e.g. `CLOCK_MONOTONIC` (see clock_gettime(2)) on Android and Linux).To asynchronously
    /// query the presentation engine, for newly-available timing
    ///information about one or more previous presents to a given swapchain, call:
    ///```c
    ///// Provided by VK_GOOGLE_display_timing
    ///VkResult vkGetPastPresentationTimingGOOGLE(
    ///    VkDevice                                    device,
    ///    VkSwapchainKHR                              swapchain,
    ///    uint32_t*                                   pPresentationTimingCount,
    ///    VkPastPresentationTimingGOOGLE*             pPresentationTimings);
    ///```
    /// # Parameters
    /// - [`device`] is the device associated with [`swapchain`].
    /// - [`swapchain`] is the swapchain to obtain presentation timing information duration for.
    /// - [`p_presentation_timing_count`] is a pointer to an integer related to the number of
    ///   [`PastPresentationTimingGOOGLE`] structures to query, as described below.
    /// - [`p_presentation_timings`] is either `NULL` or a pointer to an array of
    ///   [`PastPresentationTimingGOOGLE`] structures.
    /// # Description
    /// If [`p_presentation_timings`] is `NULL`, then the number of newly-available
    /// timing records for the given [`swapchain`] is returned in
    /// [`p_presentation_timing_count`].
    /// Otherwise, [`p_presentation_timing_count`] **must**  point to a variable set by
    /// the user to the number of elements in the [`p_presentation_timings`] array,
    /// and on return the variable is overwritten with the number of structures
    /// actually written to [`p_presentation_timings`].
    /// If the value of [`p_presentation_timing_count`] is less than the number of
    /// newly-available timing records, at most [`p_presentation_timing_count`]
    /// structures will be written, and `VK_INCOMPLETE` will be returned instead
    /// of `VK_SUCCESS`, to indicate that not all the available timing records
    /// were returned.
    /// ## Valid Usage (Implicit)
    /// - [`device`] **must**  be a valid [`Device`] handle
    /// - [`swapchain`] **must**  be a valid [`SwapchainKHR`] handle
    /// - [`p_presentation_timing_count`] **must**  be a valid pointer to a `uint32_t` value
    /// - If the value referenced by [`p_presentation_timing_count`] is not `0`, and
    ///   [`p_presentation_timings`] is not `NULL`, [`p_presentation_timings`] **must**  be a valid
    ///   pointer to an array of [`p_presentation_timing_count`][`PastPresentationTimingGOOGLE`]
    ///   structures
    /// - Both of [`device`], and [`swapchain`] **must**  have been created, allocated, or retrieved
    ///   from the same [`Instance`]
    ///
    /// ## Host Synchronization
    /// - Host access to [`swapchain`] **must**  be externally synchronized
    ///
    /// ## Return Codes
    /// * - `VK_SUCCESS`  - `VK_INCOMPLETE`
    /// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_DEVICE_LOST`  - `VK_ERROR_OUT_OF_DATE_KHR`  -
    ///   `VK_ERROR_SURFACE_LOST_KHR`
    /// # Related
    /// - [`google_display_timing`]
    /// - [`Device`]
    /// - [`PastPresentationTimingGOOGLE`]
    /// - [`SwapchainKHR`]
    ///
    /// # Notes and documentation
    /// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    /// This documentation is generated from the Vulkan specification and documentation.
    /// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    /// This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkGetPastPresentationTimingGOOGLE")]
    #[track_caller]
    #[inline]
    pub unsafe fn get_past_presentation_timing_google(
        self: &Unique<Device>,
        swapchain: SwapchainKHR,
        p_presentation_timing_count: Option<usize>,
    ) -> VulkanResult<SmallVec<PastPresentationTimingGOOGLE>> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .google_display_timing()
            .and_then(|vtable| vtable.get_past_presentation_timing_google())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .google_display_timing()
            .and_then(|vtable| vtable.get_past_presentation_timing_google())
            .unwrap_unchecked();
        let mut p_presentation_timing_count = match p_presentation_timing_count {
            Some(v) => v as _,
            None => {
                let mut v = 0;
                _function(self.as_raw(), swapchain, &mut v, std::ptr::null_mut());
                v
            },
        };
        let mut p_presentation_timings = SmallVec::<PastPresentationTimingGOOGLE>::from_elem(
            Default::default(),
            p_presentation_timing_count as usize,
        );
        let _return = _function(
            self.as_raw(),
            swapchain,
            &mut p_presentation_timing_count,
            p_presentation_timings.as_mut_ptr(),
        );
        match _return {
            VulkanResultCodes::SUCCESS | VulkanResultCodes::INCOMPLETE => {
                VulkanResult::Success(_return, p_presentation_timings)
            },
            e => VulkanResult::Err(e),
        }
    }
}
///The V-table of [`Device`] for functions from `VK_GOOGLE_display_timing`
pub struct DeviceGoogleDisplayTimingVTable {
    ///See [`FNGetRefreshCycleDurationGoogle`] for more information.
    pub get_refresh_cycle_duration_google: FNGetRefreshCycleDurationGoogle,
    ///See [`FNGetPastPresentationTimingGoogle`] for more information.
    pub get_past_presentation_timing_google: FNGetPastPresentationTimingGoogle,
}
impl DeviceGoogleDisplayTimingVTable {
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
            get_refresh_cycle_duration_google: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkGetRefreshCycleDurationGOOGLE").as_ptr(),
                ))
            },
            get_past_presentation_timing_google: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkGetPastPresentationTimingGOOGLE").as_ptr(),
                ))
            },
        }
    }
    ///Gets [`Self::get_refresh_cycle_duration_google`]. See [`FNGetRefreshCycleDurationGoogle`]
    /// for more information.
    pub fn get_refresh_cycle_duration_google(&self) -> FNGetRefreshCycleDurationGoogle {
        self.get_refresh_cycle_duration_google
    }
    ///Gets [`Self::get_past_presentation_timing_google`]. See
    /// [`FNGetPastPresentationTimingGoogle`] for more information.
    pub fn get_past_presentation_timing_google(&self) -> FNGetPastPresentationTimingGoogle {
        self.get_past_presentation_timing_google
    }
}
