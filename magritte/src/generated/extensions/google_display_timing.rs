use crate::vulkan1_0::{BaseInStructure, StructureType};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_GOOGLE_DISPLAY_TIMING_SPEC_VERSION")]
pub const GOOGLE_DISPLAY_TIMING_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_GOOGLE_DISPLAY_TIMING_EXTENSION_NAME")]
pub const GOOGLE_DISPLAY_TIMING_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_GOOGLE_display_timing");
///[VkRefreshCycleDurationGOOGLE](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRefreshCycleDurationGOOGLE.html) - Structure containing the RC duration of a display
///# C Specifications
///The [`RefreshCycleDurationGOOGLE`] structure is defined as:
///```c
///// Provided by VK_GOOGLE_display_timing
///typedef struct VkRefreshCycleDurationGOOGLE {
///    uint64_t    refreshDuration;
///} VkRefreshCycleDurationGOOGLE;
///```
///# Members
/// - [`refresh_duration`] is the number of nanoseconds from the start of one refresh cycle to the
///   next.
///# Related
/// - [`VK_GOOGLE_display_timing`]
/// - [`GetRefreshCycleDurationGOOGLE`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct RefreshCycleDurationGOOGLE {
    ///[`refresh_duration`] is the number of nanoseconds from the start of one
    ///refresh cycle to the next.
    refresh_duration: u64,
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
///# Members
/// - [`present_id`] is an application-provided value that was given to a previous
///   [`QueuePresentKHR`] command via [`PresentTimeGOOGLE`]::[`present_id`] (see below). It **can**
///   be used to uniquely identify a previous present with the [`QueuePresentKHR`] command.
/// - [`desired_present_time`] is an application-provided value that was given to a previous
///   [`QueuePresentKHR`] command via [`PresentTimeGOOGLE`]::[`desired_present_time`]. If non-zero,
///   it was used by the application to indicate that an image not be presented any sooner than
///   [`desired_present_time`].
/// - [`actual_present_time`] is the time when the image of the `swapchain` was actually displayed.
/// - [`earliest_present_time`] is the time when the image of the `swapchain` could have been
///   displayed. This **may** differ from [`actual_present_time`] if the application requested that
///   the image be presented no sooner than [`PresentTimeGOOGLE`]::[`desired_present_time`].
/// - [`present_margin`] is an indication of how early the [`QueuePresentKHR`] command was processed
///   compared to how soon it needed to be processed, and still be presented at
///   [`earliest_present_time`].
///# Description
///The results for a given `swapchain` and [`present_id`] are only
///returned once from [`GetPastPresentationTimingGOOGLE`].The application **can** use the
/// [`PastPresentationTimingGOOGLE`] values to
///occasionally adjust its timing.
///For example, if [`actual_present_time`] is later than expected (e.g. one
///`refreshDuration` late), the application may increase its target IPD to
///a higher multiple of `refreshDuration` (e.g. decrease its frame rate
///from 60Hz to 30Hz).
///If [`actual_present_time`] and [`earliest_present_time`] are consistently
///different, and if [`present_margin`] is consistently large enough, the
///application may decrease its target IPD to a smaller multiple of
///`refreshDuration` (e.g. increase its frame rate from 30Hz to 60Hz).
///If [`actual_present_time`] and [`earliest_present_time`] are same, and if
///[`present_margin`] is consistently high, the application may delay the
///start of its input-render-present loop in order to decrease the latency
///between user input and the corresponding present (always leaving some margin
///in case a new image takes longer to render than the previous image).
///An application that desires its target IPD to always be the same as
///`refreshDuration`, can also adjust features until
///[`actual_present_time`] is never late and [`present_margin`] is
///satisfactory.
///# Related
/// - [`VK_GOOGLE_display_timing`]
/// - [`GetPastPresentationTimingGOOGLE`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct PastPresentationTimingGOOGLE {
    ///[`present_id`] is an application-provided value that was given to a
    ///previous [`QueuePresentKHR`] command via
    ///[`PresentTimeGOOGLE`]::[`present_id`] (see below).
    ///It **can** be used to uniquely identify a previous present with the
    ///[`QueuePresentKHR`] command.
    present_id: u32,
    ///[`desired_present_time`] is an application-provided value that was given
    ///to a previous [`QueuePresentKHR`] command via
    ///[`PresentTimeGOOGLE`]::[`desired_present_time`].
    ///If non-zero, it was used by the application to indicate that an image
    ///not be presented any sooner than [`desired_present_time`].
    desired_present_time: u64,
    ///[`actual_present_time`] is the time when the image of the
    ///`swapchain` was actually displayed.
    actual_present_time: u64,
    ///[`earliest_present_time`] is the time when the image of the
    ///`swapchain` could have been displayed.
    ///This **may** differ from [`actual_present_time`] if the application
    ///requested that the image be presented no sooner than
    ///[`PresentTimeGOOGLE`]::[`desired_present_time`].
    earliest_present_time: u64,
    ///[`present_margin`] is an indication of how early the
    ///[`QueuePresentKHR`] command was processed compared to how soon it
    ///needed to be processed, and still be presented at
    ///[`earliest_present_time`].
    present_margin: u64,
}
///[VkPresentTimesInfoGOOGLE](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPresentTimesInfoGOOGLE.html) - The earliest time each image should be presented
///# C Specifications
///When the `[`VK_GOOGLE_display_timing`]` extension is enabled, additional
///fields **can** be specified that allow an application to specify the earliest
///time that an image should be displayed.
///This allows an application to avoid stutter that is caused by an image being
///displayed earlier than planned.
///Such stuttering can occur with both fixed and variable-refresh-rate
///displays, because stuttering occurs when the geometry is not correctly
///positioned for when the image is displayed.
///An application **can** instruct the presentation engine that an image should
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
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`swapchain_count`] is the number of swapchains being presented to by this command.
/// - [`p_times`] is `NULL` or a pointer to an array of [`PresentTimeGOOGLE`] elements with
///   [`swapchain_count`] entries. If not `NULL`, each element of [`p_times`] contains the earliest
///   time to present the image corresponding to the entry in the
///   [`PresentInfoKHR::p_image_indices`] array.
///# Description
///Valid Usage
/// - [`swapchain_count`]**must** be the same value as [`PresentInfoKHR`]::[`swapchain_count`],
///   where [`PresentInfoKHR`] is included in the [`p_next`] chain of this
///   [`PresentTimesInfoGOOGLE`] structure
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PRESENT_TIMES_INFO_GOOGLE`
/// - If [`p_times`] is not `NULL`, [`p_times`]**must** be a valid pointer to an array of
///   [`swapchain_count`][`PresentTimeGOOGLE`] structures
/// - [`swapchain_count`]**must** be greater than `0`
///# Related
/// - [`VK_GOOGLE_display_timing`]
/// - [`PresentTimeGOOGLE`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct PresentTimesInfoGOOGLE<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`swapchain_count`] is the number of swapchains being presented to by
    ///this command.
    swapchain_count: u32,
    ///[`p_times`] is `NULL` or a pointer to an array of
    ///[`PresentTimeGOOGLE`] elements with [`swapchain_count`] entries.
    ///If not `NULL`, each element of [`p_times`] contains the earliest time
    ///to present the image corresponding to the entry in the
    ///[`PresentInfoKHR`]::`pImageIndices` array.
    p_times: *mut PresentTimeGOOGLE,
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
///# Members
/// - [`present_id`] is an application-provided identification value, that **can** be used with the
///   results of [`GetPastPresentationTimingGOOGLE`], in order to uniquely identify this present. In
///   order to be useful to the application, it **should** be unique within some period of time that
///   is meaningful to the application.
/// - [`desired_present_time`] specifies that the image given **should** not be displayed to the
///   user any earlier than this time. [`desired_present_time`] is a time in nanoseconds, relative
///   to a monotonically-increasing clock (e.g. `CLOCK_MONOTONIC` (see clock_gettime(2)) on Android
///   and Linux). A value of zero specifies that the presentation engine **may** display the image
///   at any time. This is useful when the application desires to provide [`present_id`],
///# Description
///```c but does not need a specific pname:desiredPresentTime.
///```
///# Related
/// - [`VK_GOOGLE_display_timing`]
/// - [`PresentTimesInfoGOOGLE`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct PresentTimeGOOGLE {
    ///[`present_id`] is an application-provided identification value, that
    ///**can** be used with the results of
    ///[`GetPastPresentationTimingGOOGLE`], in order to uniquely identify
    ///this present.
    ///In order to be useful to the application, it **should** be unique within
    ///some period of time that is meaningful to the application.
    present_id: u32,
    ///[`desired_present_time`] specifies that the image given **should** not be
    ///displayed to the user any earlier than this time.
    ///[`desired_present_time`] is a time in nanoseconds, relative to a
    ///monotonically-increasing clock (e.g. `CLOCK_MONOTONIC` (see
    ///clock_gettime(2)) on Android and Linux).
    ///A value of zero specifies that the presentation engine **may** display the
    ///image at any time.
    ///This is useful when the application desires to provide [`present_id`],
    desired_present_time: u64,
}
