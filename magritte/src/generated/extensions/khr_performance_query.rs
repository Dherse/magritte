//![VK_KHR_performance_query](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_performance_query.html) - device extension
//!# Description
//!The [`VK_KHR_performance_query`] extension adds a mechanism to allow querying
//!of performance counters for use in applications and by profiling tools.Each queue family
//! **may**  expose counters that  **can**  be enabled on a queue of
//!that family.
//!We extend [`QueryType`] to add a new query type for performance queries,
//!and chain a structure on [`QueryPoolCreateInfo`] to specify the
//!performance queries to enable.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_get_physical_device_properties2`]`
//!# Contacts
//! - Alon Or-bach [alonorbach](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_performance_query]
//!   @alonorbach%0A<<Here describe the issue or question you have about the
//!   VK_KHR_performance_query extension>>)
//!# New functions & commands
//! - [`acquire_profiling_lock_khr`]
//! - [`enumerate_physical_device_queue_family_performance_query_counters_khr`]
//! - [`get_physical_device_queue_family_performance_query_passes_khr`]
//! - [`release_profiling_lock_khr`]
//!# New structures
//! - [`AcquireProfilingLockInfoKHR`]
//! - [`PerformanceCounterDescriptionKHR`]
//! - [`PerformanceCounterKHR`]
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  -
//!   [`PhysicalDevicePerformanceQueryFeaturesKHR`]
//! - Extending [`PhysicalDeviceProperties2`]:  - [`PhysicalDevicePerformanceQueryPropertiesKHR`]
//! - Extending [`QueryPoolCreateInfo`]:  - [`QueryPoolPerformanceCreateInfoKHR`]
//! - Extending [`SubmitInfo`], [`SubmitInfo2`]:  - [`PerformanceQuerySubmitInfoKHR`]
//!# New enums
//! - [`AcquireProfilingLockFlagBitsKHR`]
//! - [`PerformanceCounterDescriptionFlagBitsKHR`]
//! - [`PerformanceCounterScopeKHR`]
//! - [`PerformanceCounterStorageKHR`]
//! - [`PerformanceCounterUnitKHR`]
//!# New bitmasks
//! - [`AcquireProfilingLockFlagsKHR`]
//! - [`PerformanceCounterDescriptionFlagsKHR`]
//!# New constants
//! - [`KHR_PERFORMANCE_QUERY_EXTENSION_NAME`]
//! - [`KHR_PERFORMANCE_QUERY_SPEC_VERSION`]
//! - Extending [`QueryType`]:  - `VK_QUERY_TYPE_PERFORMANCE_QUERY_KHR`
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_ACQUIRE_PROFILING_LOCK_INFO_KHR`  -
//!   `VK_STRUCTURE_TYPE_PERFORMANCE_COUNTER_DESCRIPTION_KHR`  -
//!   `VK_STRUCTURE_TYPE_PERFORMANCE_COUNTER_KHR`  -
//!   `VK_STRUCTURE_TYPE_PERFORMANCE_QUERY_SUBMIT_INFO_KHR`  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PERFORMANCE_QUERY_FEATURES_KHR`  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PERFORMANCE_QUERY_PROPERTIES_KHR`  -
//!   `VK_STRUCTURE_TYPE_QUERY_POOL_PERFORMANCE_CREATE_INFO_KHR`
//!# Known issues & F.A.Q
//!1) Should this extension include a mechanism to begin a query in command
//!buffer *A* and end the query in command buffer *B*? **RESOLVED**  No - queries are tied to
//! command buffer creation and thus have to
//!be encapsulated within a single command buffer.2) Should this extension include a mechanism to
//! begin and end queries
//!globally on the queue, not using the existing command buffer commands? **RESOLVED**  No - for
//! the same reasoning as the resolution of 1).3) Should this extension expose counters that require
//! multiple passes? **RESOLVED**  Yes - users should re-submit a command buffer with the same
//!commands in it multiple times, specifying the pass to count as the query
//!parameter in VkPerformanceQuerySubmitInfoKHR.4) How to handle counters across parallel
//! workloads? **RESOLVED**  In the spirit of Vulkan, a counter description flag
//!`VK_PERFORMANCE_COUNTER_DESCRIPTION_CONCURRENTLY_IMPACTED_BIT_KHR`
//!denotes that the accuracy of a counter result is affected by parallel
//!workloads.5) How to handle secondary command buffers? **RESOLVED**  Secondary command buffers
//! inherit any counter pass index
//!specified in the parent primary command buffer.
//!Note: this is no longer an issue after change from issue 10 resolution6) What commands does the
//! profiling lock have to be held for? **RESOLVED**  For any command buffer that is being queried
//! with a performance
//!query pool, the profiling lock  **must**  be held while that command buffer is in
//!the *recording*, *executable*, or *pending state*.7) Should we support
//! [`cmd_copy_query_pool_results`]? **RESOLVED**  Yes.8) Should we allow performance queries to
//! interact with multiview? **RESOLVED**  Yes, but the performance queries must be performed once
//! for each
//!pass per view.9) Should a `queryCount > 1` be usable for performance queries? **RESOLVED**  Yes.
//!Some vendors will have costly performance counter query pool creation, and
//!would rather if a certain set of counters were to be used multiple times
//!that a `queryCount > 1` can be used to amortize the instantiation cost.10) Should we introduce
//! an indirect mechanism to set the counter pass index? **RESOLVED**  Specify the counter pass
//! index at submit time instead, to avoid
//!requiring re-recording of command buffers when multiple counter passes are
//!needed.
//!# Version History
//! - Revision 1, 2019-10-08
//!# Other info
//! * 2019-10-08
//! * No known IP claims.
//! * - Jesse Barker, Unity Technologies  - Kenneth Benzie, Codeplay  - Jan-Harald Fredriksen, ARM
//!   - Jeff Leger, Qualcomm  - Jesse Hall, Google  - Tobias Hector, AMD  - Neil Henning, Codeplay
//!   - Baldur Karlsson  - Lionel Landwerlin, Intel  - Peter Lohrmann, AMD  - Alon Or-bach, Samsung
//!   - Daniel Rakos, AMD  - Niklas Smedberg, Unity Technologies  - Igor Ostrowski, Intel
//!# Related
//! - [`AcquireProfilingLockFlagBitsKHR`]
//! - [`AcquireProfilingLockFlagsKHR`]
//! - [`AcquireProfilingLockInfoKHR`]
//! - [`PerformanceCounterDescriptionFlagBitsKHR`]
//! - [`PerformanceCounterDescriptionFlagsKHR`]
//! - [`PerformanceCounterDescriptionKHR`]
//! - [`PerformanceCounterKHR`]
//! - [`PerformanceCounterResultKHR`]
//! - [`PerformanceCounterScopeKHR`]
//! - [`PerformanceCounterStorageKHR`]
//! - [`PerformanceCounterUnitKHR`]
//! - [`PerformanceQuerySubmitInfoKHR`]
//! - [`PhysicalDevicePerformanceQueryFeaturesKHR`]
//! - [`PhysicalDevicePerformanceQueryPropertiesKHR`]
//! - [`QueryPoolPerformanceCreateInfoKHR`]
//! - [`acquire_profiling_lock_khr`]
//! - [`enumerate_physical_device_queue_family_performance_query_counters_khr`]
//! - [`get_physical_device_queue_family_performance_query_passes_khr`]
//! - [`release_profiling_lock_khr`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::{
    core::{MAX_DESCRIPTION_SIZE, UUID_SIZE},
    vulkan1_0::{
        BaseInStructure, BaseOutStructure, Bool32, Device, Instance, PhysicalDevice, StructureType, VulkanResultCodes,
    },
};
#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{
    ffi::CStr,
    iter::{Extend, FromIterator, IntoIterator},
    marker::PhantomData,
    os::raw::c_char,
};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_PERFORMANCE_QUERY_SPEC_VERSION")]
pub const KHR_PERFORMANCE_QUERY_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_PERFORMANCE_QUERY_EXTENSION_NAME")]
pub const KHR_PERFORMANCE_QUERY_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_performance_query");
///[vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR.html) - Reports properties of the performance query counters available on a queue family of a device
///# C Specifications
///To enumerate the performance query counters available on a queue family of a
///physical device, call:
///```c
///// Provided by VK_KHR_performance_query
///VkResult vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR(
///    VkPhysicalDevice                            physicalDevice,
///    uint32_t                                    queueFamilyIndex,
///    uint32_t*                                   pCounterCount,
///    VkPerformanceCounterKHR*                    pCounters,
///    VkPerformanceCounterDescriptionKHR*         pCounterDescriptions);
///```
///# Parameters
/// - [`physical_device`] is the handle to the physical device whose queue family performance query
///   counter properties will be queried.
/// - [`queue_family_index`] is the index into the queue family of the physical device we want to
///   get properties for.
/// - [`p_counter_count`] is a pointer to an integer related to the number of counters available or
///   queried, as described below.
/// - [`p_counters`] is either `NULL` or a pointer to an array of [`PerformanceCounterKHR`]
///   structures.
/// - [`p_counter_descriptions`] is either `NULL` or a pointer to an array of
///   [`PerformanceCounterDescriptionKHR`] structures.
///# Description
///If [`p_counters`] is `NULL` and [`p_counter_descriptions`] is `NULL`, then
///the number of counters available is returned in [`p_counter_count`].
///Otherwise, [`p_counter_count`] **must**  point to a variable set by the user to
///the number of elements in the [`p_counters`], [`p_counter_descriptions`],
///or both arrays and on return the variable is overwritten with the number of
///structures actually written out.
///If [`p_counter_count`] is less than the number of counters available, at
///most [`p_counter_count`] structures will be written, and `VK_INCOMPLETE`
///will be returned instead of `VK_SUCCESS`, to indicate that not all the
///available counters were returned.
///## Valid Usage (Implicit)
/// - [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
/// - [`p_counter_count`] **must**  be a valid pointer to a `uint32_t` value
/// - If the value referenced by [`p_counter_count`] is not `0`, and [`p_counters`] is not `NULL`,
///   [`p_counters`] **must**  be a valid pointer to an array of
///   [`p_counter_count`][`PerformanceCounterKHR`] structures
/// - If the value referenced by [`p_counter_count`] is not `0`, and [`p_counter_descriptions`] is
///   not `NULL`, [`p_counter_descriptions`] **must**  be a valid pointer to an array of
///   [`p_counter_count`][`PerformanceCounterDescriptionKHR`] structures
///
///## Return Codes
/// * - `VK_SUCCESS`  - `VK_INCOMPLETE`
/// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  -
///   `VK_ERROR_INITIALIZATION_FAILED`
///# Related
/// - [`VK_KHR_performance_query`]
/// - [`PerformanceCounterDescriptionKHR`]
/// - [`PerformanceCounterKHR`]
/// - [`PhysicalDevice`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR")]
pub type FNEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKhr = Option<
    for<'lt> unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        p_counter_count: *mut u32,
        p_counters: *mut PerformanceCounterKHR<'lt>,
        p_counter_descriptions: *mut PerformanceCounterDescriptionKHR<'lt>,
    ) -> VulkanResultCodes,
>;
///[vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR.html) - Reports the number of passes require for a performance query pool type
///# C Specifications
///To query the number of passes required to query a performance query pool on
///a physical device, call:
///```c
///// Provided by VK_KHR_performance_query
///void vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR(
///    VkPhysicalDevice                            physicalDevice,
///    const VkQueryPoolPerformanceCreateInfoKHR*  pPerformanceQueryCreateInfo,
///    uint32_t*                                   pNumPasses);
///```
///# Parameters
/// - [`physical_device`] is the handle to the physical device whose queue family performance query
///   counter properties will be queried.
/// - [`p_performance_query_create_info`] is a pointer to a [`QueryPoolPerformanceCreateInfoKHR`] of
///   the performance query that is to be created.
/// - [`p_num_passes`] is a pointer to an integer related to the number of passes required to query
///   the performance query pool, as described below.
///# Description
///The [`p_performance_query_create_info`] member
///[`QueryPoolPerformanceCreateInfoKHR::queue_family_index`] **must**  be a
///queue family of [`physical_device`].
///The number of passes required to capture the counters specified in the
///[`p_performance_query_create_info`] member
///[`QueryPoolPerformanceCreateInfoKHR`]`::pCounters` is returned in
///[`p_num_passes`].
///## Valid Usage (Implicit)
/// - [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
/// - [`p_performance_query_create_info`] **must**  be a valid pointer to a valid
///   [`QueryPoolPerformanceCreateInfoKHR`] structure
/// - [`p_num_passes`] **must**  be a valid pointer to a `uint32_t` value
///# Related
/// - [`VK_KHR_performance_query`]
/// - [`PhysicalDevice`]
/// - [`QueryPoolPerformanceCreateInfoKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR")]
pub type FNGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKhr = Option<
    for<'lt> unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_performance_query_create_info: *const QueryPoolPerformanceCreateInfoKHR<'lt>,
        p_num_passes: *mut u32,
    ),
>;
///[vkAcquireProfilingLockKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAcquireProfilingLockKHR.html) - Acquires the profiling lock
///# C Specifications
///To record and submit a command buffer containing a performance query pool
///the profiling lock  **must**  be held.
///The profiling lock  **must**  be acquired prior to any call to
///[`begin_command_buffer`] that will be using a performance query pool.
///The profiling lock  **must**  be held while any command buffer containing a
///performance query pool is in the *recording*, *executable*, or *pending
///state*.
///To acquire the profiling lock, call:
///```c
///// Provided by VK_KHR_performance_query
///VkResult vkAcquireProfilingLockKHR(
///    VkDevice                                    device,
///    const VkAcquireProfilingLockInfoKHR*        pInfo);
///```
///# Parameters
/// - [`device`] is the logical device to profile.
/// - [`p_info`] is a pointer to a [`AcquireProfilingLockInfoKHR`] structure containing information
///   about how the profiling is to be acquired.
///# Description
///Implementations **may**  allow multiple actors to hold the profiling lock
///concurrently.
///## Valid Usage (Implicit)
/// - [`device`] **must**  be a valid [`Device`] handle
/// - [`p_info`] **must**  be a valid pointer to a valid [`AcquireProfilingLockInfoKHR`] structure
///
///## Return Codes
/// * - `VK_SUCCESS`
/// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_TIMEOUT`
///# Related
/// - [`VK_KHR_performance_query`]
/// - [`AcquireProfilingLockInfoKHR`]
/// - [`Device`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkAcquireProfilingLockKHR")]
pub type FNAcquireProfilingLockKhr = Option<
    for<'lt> unsafe extern "system" fn(
        device: Device,
        p_info: *const AcquireProfilingLockInfoKHR<'lt>,
    ) -> VulkanResultCodes,
>;
///[vkReleaseProfilingLockKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkReleaseProfilingLockKHR.html) - Releases the profiling lock
///# C Specifications
///To release the profiling lock, call:
///```c
///// Provided by VK_KHR_performance_query
///void vkReleaseProfilingLockKHR(
///    VkDevice                                    device);
///```
///# Parameters
/// - [`device`] is the logical device to cease profiling on.
///# Description
///## Valid Usage
/// - The profiling lock of [`device`] **must**  have been held via a previous successful call to
///   [`acquire_profiling_lock_khr`]
///
///## Valid Usage (Implicit)
/// - [`device`] **must**  be a valid [`Device`] handle
///# Related
/// - [`VK_KHR_performance_query`]
/// - [`Device`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkReleaseProfilingLockKHR")]
pub type FNReleaseProfilingLockKhr = Option<unsafe extern "system" fn(device: Device)>;
///[VkPerformanceCounterScopeKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPerformanceCounterScopeKHR.html) - Supported counter scope types
///# C Specifications
///Performance counters have an associated scope.
///This scope describes the granularity of a performance counter.The performance counter scope
/// types which  **may**  be returned in
///[`PerformanceCounterKHR::scope`] are:
///```c
///// Provided by VK_KHR_performance_query
///typedef enum VkPerformanceCounterScopeKHR {
///    VK_PERFORMANCE_COUNTER_SCOPE_COMMAND_BUFFER_KHR = 0,
///    VK_PERFORMANCE_COUNTER_SCOPE_RENDER_PASS_KHR = 1,
///    VK_PERFORMANCE_COUNTER_SCOPE_COMMAND_KHR = 2,
///    VK_QUERY_SCOPE_COMMAND_BUFFER_KHR = VK_PERFORMANCE_COUNTER_SCOPE_COMMAND_BUFFER_KHR,
///    VK_QUERY_SCOPE_RENDER_PASS_KHR = VK_PERFORMANCE_COUNTER_SCOPE_RENDER_PASS_KHR,
///    VK_QUERY_SCOPE_COMMAND_KHR = VK_PERFORMANCE_COUNTER_SCOPE_COMMAND_KHR,
///} VkPerformanceCounterScopeKHR;
///```
///# Description
/// - [`PerformanceCounterScopeCommandBufferKhr`] - the performance counter scope is a single
///   complete command buffer.
/// - [`PerformanceCounterScopeRenderPassKhr`] - the performance counter scope is zero or more
///   complete render passes. The performance query containing the performance counter  **must**
///   begin and end outside a render pass instance.
/// - [`PerformanceCounterScopeCommandKhr`] - the performance counter scope is zero or more
///   commands.
///# Related
/// - [`VK_KHR_performance_query`]
/// - [`PerformanceCounterKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPerformanceCounterScopeKHR")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
#[repr(i32)]
pub enum PerformanceCounterScopeKHR {
    ///[`PerformanceCounterScopeCommandBufferKhr`] - the performance
    ///counter scope is a single complete command buffer.
    PerformanceCounterScopeCommandBufferKhr = 0,
    ///[`PerformanceCounterScopeRenderPassKhr`] - the performance
    ///counter scope is zero or more complete render passes.
    ///The performance query containing the performance counter  **must**  begin and
    ///end outside a render pass instance.
    PerformanceCounterScopeRenderPassKhr = 1,
    ///[`PerformanceCounterScopeCommandKhr`] - the performance counter
    ///scope is zero or more commands.
    PerformanceCounterScopeCommandKhr = 2,
}
impl const Default for PerformanceCounterScopeKHR {
    fn default() -> Self {
        Self::PerformanceCounterScopeCommandBufferKhr
    }
}
impl PerformanceCounterScopeKHR {
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
///[VkPerformanceCounterUnitKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPerformanceCounterUnitKHR.html) - Supported counter unit types
///# C Specifications
///Performance counters have an associated unit.
///This unit describes how to interpret the performance counter result.The performance counter unit
/// types which  **may**  be returned in
///[`PerformanceCounterKHR::unit`] are:
///```c
///// Provided by VK_KHR_performance_query
///typedef enum VkPerformanceCounterUnitKHR {
///    VK_PERFORMANCE_COUNTER_UNIT_GENERIC_KHR = 0,
///    VK_PERFORMANCE_COUNTER_UNIT_PERCENTAGE_KHR = 1,
///    VK_PERFORMANCE_COUNTER_UNIT_NANOSECONDS_KHR = 2,
///    VK_PERFORMANCE_COUNTER_UNIT_BYTES_KHR = 3,
///    VK_PERFORMANCE_COUNTER_UNIT_BYTES_PER_SECOND_KHR = 4,
///    VK_PERFORMANCE_COUNTER_UNIT_KELVIN_KHR = 5,
///    VK_PERFORMANCE_COUNTER_UNIT_WATTS_KHR = 6,
///    VK_PERFORMANCE_COUNTER_UNIT_VOLTS_KHR = 7,
///    VK_PERFORMANCE_COUNTER_UNIT_AMPS_KHR = 8,
///    VK_PERFORMANCE_COUNTER_UNIT_HERTZ_KHR = 9,
///    VK_PERFORMANCE_COUNTER_UNIT_CYCLES_KHR = 10,
///} VkPerformanceCounterUnitKHR;
///```
///# Description
/// - [`PerformanceCounterUnitGenericKhr`] - the performance counter unit is a generic data point.
/// - [`PerformanceCounterUnitPercentageKhr`] - the performance counter unit is a percentage (%).
/// - [`PerformanceCounterUnitNanosecondsKhr`] - the performance counter unit is a value of
///   nanoseconds (ns).
/// - [`PerformanceCounterUnitBytesKhr`] - the performance counter unit is a value of bytes.
/// - [`PerformanceCounterUnitBytesPerSecondKhr`] - the performance counter unit is a value of
///   bytes/s.
/// - [`PerformanceCounterUnitKelvinKhr`] - the performance counter unit is a temperature reported
///   in Kelvin.
/// - [`PerformanceCounterUnitWattsKhr`] - the performance counter unit is a value of watts (W).
/// - [`PerformanceCounterUnitVoltsKhr`] - the performance counter unit is a value of volts (V).
/// - [`PerformanceCounterUnitAmpsKhr`] - the performance counter unit is a value of amps (A).
/// - [`PerformanceCounterUnitHertzKhr`] - the performance counter unit is a value of hertz (Hz).
/// - [`PerformanceCounterUnitCyclesKhr`] - the performance counter unit is a value of cycles.
///# Related
/// - [`VK_KHR_performance_query`]
/// - [`PerformanceCounterKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPerformanceCounterUnitKHR")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
#[repr(i32)]
pub enum PerformanceCounterUnitKHR {
    ///[`PerformanceCounterUnitGenericKhr`] - the performance counter
    ///unit is a generic data point.
    PerformanceCounterUnitGenericKhr = 0,
    ///[`PerformanceCounterUnitPercentageKhr`] - the performance
    ///counter unit is a percentage (%).
    PerformanceCounterUnitPercentageKhr = 1,
    ///[`PerformanceCounterUnitNanosecondsKhr`] - the performance
    ///counter unit is a value of nanoseconds (ns).
    PerformanceCounterUnitNanosecondsKhr = 2,
    ///[`PerformanceCounterUnitBytesKhr`] - the performance counter
    ///unit is a value of bytes.
    PerformanceCounterUnitBytesKhr = 3,
    ///[`PerformanceCounterUnitBytesPerSecondKhr`] - the performance
    ///counter unit is a value of bytes/s.
    PerformanceCounterUnitBytesPerSecondKhr = 4,
    ///[`PerformanceCounterUnitKelvinKhr`] - the performance counter
    ///unit is a temperature reported in Kelvin.
    PerformanceCounterUnitKelvinKhr = 5,
    ///[`PerformanceCounterUnitWattsKhr`] - the performance counter
    ///unit is a value of watts (W).
    PerformanceCounterUnitWattsKhr = 6,
    ///[`PerformanceCounterUnitVoltsKhr`] - the performance counter
    ///unit is a value of volts (V).
    PerformanceCounterUnitVoltsKhr = 7,
    ///[`PerformanceCounterUnitAmpsKhr`] - the performance counter
    ///unit is a value of amps (A).
    PerformanceCounterUnitAmpsKhr = 8,
    ///[`PerformanceCounterUnitHertzKhr`] - the performance counter
    ///unit is a value of hertz (Hz).
    PerformanceCounterUnitHertzKhr = 9,
    ///[`PerformanceCounterUnitCyclesKhr`] - the performance counter
    ///unit is a value of cycles.
    PerformanceCounterUnitCyclesKhr = 10,
}
impl const Default for PerformanceCounterUnitKHR {
    fn default() -> Self {
        Self::PerformanceCounterUnitGenericKhr
    }
}
impl PerformanceCounterUnitKHR {
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
///[VkPerformanceCounterStorageKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPerformanceCounterStorageKHR.html) - Supported counter storage types
///# C Specifications
///Performance counters have an associated storage.
///This storage describes the payload of a counter result.The performance counter storage types
/// which  **may**  be returned in
///[`PerformanceCounterKHR::storage`] are:
///```c
///// Provided by VK_KHR_performance_query
///typedef enum VkPerformanceCounterStorageKHR {
///    VK_PERFORMANCE_COUNTER_STORAGE_INT32_KHR = 0,
///    VK_PERFORMANCE_COUNTER_STORAGE_INT64_KHR = 1,
///    VK_PERFORMANCE_COUNTER_STORAGE_UINT32_KHR = 2,
///    VK_PERFORMANCE_COUNTER_STORAGE_UINT64_KHR = 3,
///    VK_PERFORMANCE_COUNTER_STORAGE_FLOAT32_KHR = 4,
///    VK_PERFORMANCE_COUNTER_STORAGE_FLOAT64_KHR = 5,
///} VkPerformanceCounterStorageKHR;
///```
///# Description
/// - [`PerformanceCounterStorageInt32Khr`] - the performance counter storage is a 32-bit signed
///   integer.
/// - [`PerformanceCounterStorageInt64Khr`] - the performance counter storage is a 64-bit signed
///   integer.
/// - [`PerformanceCounterStorageUint32Khr`] - the performance counter storage is a 32-bit unsigned
///   integer.
/// - [`PerformanceCounterStorageUint64Khr`] - the performance counter storage is a 64-bit unsigned
///   integer.
/// - [`PerformanceCounterStorageFloat32Khr`] - the performance counter storage is a 32-bit
///   floating-point.
/// - [`PerformanceCounterStorageFloat64Khr`] - the performance counter storage is a 64-bit
///   floating-point.
///# Related
/// - [`VK_KHR_performance_query`]
/// - [`PerformanceCounterKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPerformanceCounterStorageKHR")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
#[repr(i32)]
pub enum PerformanceCounterStorageKHR {
    ///[`PerformanceCounterStorageInt32Khr`] - the performance counter
    ///storage is a 32-bit signed integer.
    PerformanceCounterStorageInt32Khr = 0,
    ///[`PerformanceCounterStorageInt64Khr`] - the performance counter
    ///storage is a 64-bit signed integer.
    PerformanceCounterStorageInt64Khr = 1,
    ///[`PerformanceCounterStorageUint32Khr`] - the performance
    ///counter storage is a 32-bit unsigned integer.
    PerformanceCounterStorageUint32Khr = 2,
    ///[`PerformanceCounterStorageUint64Khr`] - the performance
    ///counter storage is a 64-bit unsigned integer.
    PerformanceCounterStorageUint64Khr = 3,
    ///[`PerformanceCounterStorageFloat32Khr`] - the performance
    ///counter storage is a 32-bit floating-point.
    PerformanceCounterStorageFloat32Khr = 4,
    ///[`PerformanceCounterStorageFloat64Khr`] - the performance
    ///counter storage is a 64-bit floating-point.
    PerformanceCounterStorageFloat64Khr = 5,
}
impl const Default for PerformanceCounterStorageKHR {
    fn default() -> Self {
        Self::PerformanceCounterStorageInt32Khr
    }
}
impl PerformanceCounterStorageKHR {
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
///[VkPerformanceCounterDescriptionFlagBitsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPerformanceCounterDescriptionFlagBitsKHR.html) - Bitmask specifying usage behavior for a counter
///# C Specifications
///Bits which  **can**  be set in
///[`PerformanceCounterDescriptionKHR::flags`], specifying usage
///behavior for a performance counter, are:
///```c
///// Provided by VK_KHR_performance_query
///typedef enum VkPerformanceCounterDescriptionFlagBitsKHR {
///    VK_PERFORMANCE_COUNTER_DESCRIPTION_PERFORMANCE_IMPACTING_BIT_KHR = 0x00000001,
///    VK_PERFORMANCE_COUNTER_DESCRIPTION_CONCURRENTLY_IMPACTED_BIT_KHR = 0x00000002,
///    VK_PERFORMANCE_COUNTER_DESCRIPTION_PERFORMANCE_IMPACTING_KHR =
/// VK_PERFORMANCE_COUNTER_DESCRIPTION_PERFORMANCE_IMPACTING_BIT_KHR,
///    VK_PERFORMANCE_COUNTER_DESCRIPTION_CONCURRENTLY_IMPACTED_KHR =
/// VK_PERFORMANCE_COUNTER_DESCRIPTION_CONCURRENTLY_IMPACTED_BIT_KHR,
///} VkPerformanceCounterDescriptionFlagBitsKHR;
///```
///# Description
/// - [`PerformanceCounterDescriptionPerformanceImpactingKhr`] specifies that recording the counter
///   **may**  have a noticeable performance impact.
/// - [`PerformanceCounterDescriptionConcurrentlyImpactedKhr`] specifies that concurrently recording
///   the counter while other submitted command buffers are running  **may**  impact the accuracy of
///   the recording.
///# Related
/// - [`VK_KHR_performance_query`]
/// - [`PerformanceCounterDescriptionFlagsKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPerformanceCounterDescriptionFlagBitsKHR")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
#[repr(u32)]
pub enum PerformanceCounterDescriptionFlagBitsKHR {
    #[doc(hidden)]
    Empty = 0,
    ///[`PerformanceCounterDescriptionPerformanceImpactingKhr`]
    ///specifies that recording the counter  **may**  have a noticeable performance
    ///impact.
    PerformanceCounterDescriptionPerformanceImpactingKhr = 1,
    ///[`PerformanceCounterDescriptionConcurrentlyImpactedKhr`]
    ///specifies that concurrently recording the counter while other submitted
    ///command buffers are running  **may**  impact the accuracy of the recording.
    PerformanceCounterDescriptionConcurrentlyImpactedKhr = 2,
}
impl const Default for PerformanceCounterDescriptionFlagBitsKHR {
    fn default() -> Self {
        Self::Empty
    }
}
impl PerformanceCounterDescriptionFlagBitsKHR {
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
///[VkAcquireProfilingLockFlagBitsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAcquireProfilingLockFlagBitsKHR.html) - Reserved for future use
///# C Specifications
///```c
///// Provided by VK_KHR_performance_query
///typedef enum VkAcquireProfilingLockFlagBitsKHR {
///} VkAcquireProfilingLockFlagBitsKHR;
///```
///# Related
/// - [`VK_KHR_performance_query`]
/// - [`AcquireProfilingLockFlagsKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkAcquireProfilingLockFlagBitsKHR")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
#[repr(u32)]
pub enum AcquireProfilingLockFlagBitsKHR {
    #[doc(hidden)]
    Empty = 0,
}
impl const Default for AcquireProfilingLockFlagBitsKHR {
    fn default() -> Self {
        Self::Empty
    }
}
impl AcquireProfilingLockFlagBitsKHR {
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
///[VkPerformanceCounterDescriptionFlagBitsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPerformanceCounterDescriptionFlagBitsKHR.html) - Bitmask specifying usage behavior for a counter
///# C Specifications
///Bits which  **can**  be set in
///[`PerformanceCounterDescriptionKHR::flags`], specifying usage
///behavior for a performance counter, are:
///```c
///// Provided by VK_KHR_performance_query
///typedef enum VkPerformanceCounterDescriptionFlagBitsKHR {
///    VK_PERFORMANCE_COUNTER_DESCRIPTION_PERFORMANCE_IMPACTING_BIT_KHR = 0x00000001,
///    VK_PERFORMANCE_COUNTER_DESCRIPTION_CONCURRENTLY_IMPACTED_BIT_KHR = 0x00000002,
///    VK_PERFORMANCE_COUNTER_DESCRIPTION_PERFORMANCE_IMPACTING_KHR =
/// VK_PERFORMANCE_COUNTER_DESCRIPTION_PERFORMANCE_IMPACTING_BIT_KHR,
///    VK_PERFORMANCE_COUNTER_DESCRIPTION_CONCURRENTLY_IMPACTED_KHR =
/// VK_PERFORMANCE_COUNTER_DESCRIPTION_CONCURRENTLY_IMPACTED_BIT_KHR,
///} VkPerformanceCounterDescriptionFlagBitsKHR;
///```
///# Description
/// - [`PerformanceCounterDescriptionPerformanceImpactingKhr`] specifies that recording the counter
///   **may**  have a noticeable performance impact.
/// - [`PerformanceCounterDescriptionConcurrentlyImpactedKhr`] specifies that concurrently recording
///   the counter while other submitted command buffers are running  **may**  impact the accuracy of
///   the recording.
///# Related
/// - [`VK_KHR_performance_query`]
/// - [`PerformanceCounterDescriptionFlagsKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPerformanceCounterDescriptionFlagsKHR")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct PerformanceCounterDescriptionFlagsKHR(u32);
impl const Default for PerformanceCounterDescriptionFlagsKHR {
    fn default() -> Self {
        Self(0)
    }
}
impl From<PerformanceCounterDescriptionFlagBitsKHR> for PerformanceCounterDescriptionFlagsKHR {
    fn from(from: PerformanceCounterDescriptionFlagBitsKHR) -> Self {
        unsafe { Self::from_bits_unchecked(from as u32) }
    }
}
impl PerformanceCounterDescriptionFlagsKHR {
    ///[`PerformanceCounterDescriptionPerformanceImpactingKhr`]
    ///specifies that recording the counter  **may**  have a noticeable performance
    ///impact.
    pub const PERFORMANCE_COUNTER_DESCRIPTION_PERFORMANCE_IMPACTING_KHR: Self = Self(1);
    ///[`PerformanceCounterDescriptionConcurrentlyImpactedKhr`]
    ///specifies that concurrently recording the counter while other submitted
    ///command buffers are running  **may**  impact the accuracy of the recording.
    pub const PERFORMANCE_COUNTER_DESCRIPTION_CONCURRENTLY_IMPACTED_KHR: Self = Self(2);
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
            all |= Self::PERFORMANCE_COUNTER_DESCRIPTION_PERFORMANCE_IMPACTING_KHR;
        }
        {
            all |= Self::PERFORMANCE_COUNTER_DESCRIPTION_CONCURRENTLY_IMPACTED_KHR;
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
impl const std::ops::BitOr for PerformanceCounterDescriptionFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl const std::ops::BitOrAssign for PerformanceCounterDescriptionFlagsKHR {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for PerformanceCounterDescriptionFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl const std::ops::BitXorAssign for PerformanceCounterDescriptionFlagsKHR {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for PerformanceCounterDescriptionFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl const std::ops::BitAndAssign for PerformanceCounterDescriptionFlagsKHR {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for PerformanceCounterDescriptionFlagsKHR {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl const std::ops::SubAssign for PerformanceCounterDescriptionFlagsKHR {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for PerformanceCounterDescriptionFlagsKHR {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<PerformanceCounterDescriptionFlagsKHR> for PerformanceCounterDescriptionFlagsKHR {
    fn extend<T: IntoIterator<Item = PerformanceCounterDescriptionFlagsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl Extend<PerformanceCounterDescriptionFlagBitsKHR> for PerformanceCounterDescriptionFlagsKHR {
    fn extend<T: IntoIterator<Item = PerformanceCounterDescriptionFlagBitsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, <Self as From<PerformanceCounterDescriptionFlagBitsKHR>>::from(i));
        }
    }
}
impl FromIterator<PerformanceCounterDescriptionFlagsKHR> for PerformanceCounterDescriptionFlagsKHR {
    fn from_iter<T: IntoIterator<Item = PerformanceCounterDescriptionFlagsKHR>>(
        iterator: T,
    ) -> PerformanceCounterDescriptionFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<PerformanceCounterDescriptionFlagsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl FromIterator<PerformanceCounterDescriptionFlagBitsKHR> for PerformanceCounterDescriptionFlagsKHR {
    fn from_iter<T: IntoIterator<Item = PerformanceCounterDescriptionFlagBitsKHR>>(
        iterator: T,
    ) -> PerformanceCounterDescriptionFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<PerformanceCounterDescriptionFlagBitsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for PerformanceCounterDescriptionFlagsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(PerformanceCounterDescriptionFlagsKHR);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == PerformanceCounterDescriptionFlagsKHR::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self . 0 . contains (PerformanceCounterDescriptionFlagsKHR :: PERFORMANCE_COUNTER_DESCRIPTION_PERFORMANCE_IMPACTING_KHR) { if ! first { first = false ; f . write_str (" | ") ? ; } f . write_str (stringify ! (PERFORMANCE_COUNTER_DESCRIPTION_PERFORMANCE_IMPACTING_KHR)) ? ; }
                    if self . 0 . contains (PerformanceCounterDescriptionFlagsKHR :: PERFORMANCE_COUNTER_DESCRIPTION_CONCURRENTLY_IMPACTED_KHR) { if ! first { first = false ; f . write_str (" | ") ? ; } f . write_str (stringify ! (PERFORMANCE_COUNTER_DESCRIPTION_CONCURRENTLY_IMPACTED_KHR)) ? ; }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(PerformanceCounterDescriptionFlagsKHR))
            .field(&Flags(*self))
            .finish()
    }
}
///[VkAcquireProfilingLockFlagBitsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAcquireProfilingLockFlagBitsKHR.html) - Reserved for future use
///# C Specifications
///```c
///// Provided by VK_KHR_performance_query
///typedef enum VkAcquireProfilingLockFlagBitsKHR {
///} VkAcquireProfilingLockFlagBitsKHR;
///```
///# Related
/// - [`VK_KHR_performance_query`]
/// - [`AcquireProfilingLockFlagsKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkAcquireProfilingLockFlagsKHR")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct AcquireProfilingLockFlagsKHR(u32);
impl const Default for AcquireProfilingLockFlagsKHR {
    fn default() -> Self {
        Self(0)
    }
}
impl From<AcquireProfilingLockFlagBitsKHR> for AcquireProfilingLockFlagsKHR {
    fn from(from: AcquireProfilingLockFlagBitsKHR) -> Self {
        unsafe { Self::from_bits_unchecked(from as u32) }
    }
}
impl AcquireProfilingLockFlagsKHR {
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
impl const std::ops::BitOr for AcquireProfilingLockFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl const std::ops::BitOrAssign for AcquireProfilingLockFlagsKHR {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for AcquireProfilingLockFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl const std::ops::BitXorAssign for AcquireProfilingLockFlagsKHR {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for AcquireProfilingLockFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl const std::ops::BitAndAssign for AcquireProfilingLockFlagsKHR {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for AcquireProfilingLockFlagsKHR {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl const std::ops::SubAssign for AcquireProfilingLockFlagsKHR {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for AcquireProfilingLockFlagsKHR {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<AcquireProfilingLockFlagsKHR> for AcquireProfilingLockFlagsKHR {
    fn extend<T: IntoIterator<Item = AcquireProfilingLockFlagsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl Extend<AcquireProfilingLockFlagBitsKHR> for AcquireProfilingLockFlagsKHR {
    fn extend<T: IntoIterator<Item = AcquireProfilingLockFlagBitsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, <Self as From<AcquireProfilingLockFlagBitsKHR>>::from(i));
        }
    }
}
impl FromIterator<AcquireProfilingLockFlagsKHR> for AcquireProfilingLockFlagsKHR {
    fn from_iter<T: IntoIterator<Item = AcquireProfilingLockFlagsKHR>>(iterator: T) -> AcquireProfilingLockFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<AcquireProfilingLockFlagsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl FromIterator<AcquireProfilingLockFlagBitsKHR> for AcquireProfilingLockFlagsKHR {
    fn from_iter<T: IntoIterator<Item = AcquireProfilingLockFlagBitsKHR>>(iterator: T) -> AcquireProfilingLockFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<AcquireProfilingLockFlagBitsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for AcquireProfilingLockFlagsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(AcquireProfilingLockFlagsKHR);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == AcquireProfilingLockFlagsKHR::empty() {
                    f.write_str("empty")?;
                } else {
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(AcquireProfilingLockFlagsKHR))
            .field(&Flags(*self))
            .finish()
    }
}
///[VkPhysicalDevicePerformanceQueryFeaturesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePerformanceQueryFeaturesKHR.html) - Structure describing performance query support for an implementation
///# C Specifications
///The [`PhysicalDevicePerformanceQueryFeaturesKHR`] structure is defined
///as:
///```c
///// Provided by VK_KHR_performance_query
///typedef struct VkPhysicalDevicePerformanceQueryFeaturesKHR {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           performanceCounterQueryPools;
///    VkBool32           performanceCounterMultipleQueryPools;
///} VkPhysicalDevicePerformanceQueryFeaturesKHR;
///```
///# Members
///This structure describes the following features:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`performance_counter_query_pools`] indicates whether the implementation supports performance
///   counter query pools.
/// - [`performance_counter_multiple_query_pools`] indicates whether the implementation supports
///   using multiple performance query pools in a primary command buffer and secondary command
///   buffers executed within it.
///If the [`PhysicalDevicePerformanceQueryFeaturesKHR`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`get_physical_device_features2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDevicePerformanceQueryFeaturesKHR`] **can**  also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PERFORMANCE_QUERY_FEATURES_KHR`
///# Related
/// - [`VK_KHR_performance_query`]
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
#[doc(alias = "VkPhysicalDevicePerformanceQueryFeaturesKHR")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDevicePerformanceQueryFeaturesKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`performance_counter_query_pools`] indicates whether the implementation
    ///supports performance counter query pools.
    pub performance_counter_query_pools: Bool32,
    ///[`performance_counter_multiple_query_pools`] indicates whether the
    ///implementation supports using multiple performance query pools in a
    ///primary command buffer and secondary command buffers executed within it.
    pub performance_counter_multiple_query_pools: Bool32,
}
impl<'lt> Default for PhysicalDevicePerformanceQueryFeaturesKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PhysicalDevicePerformanceQueryFeaturesKhr,
            p_next: std::ptr::null_mut(),
            performance_counter_query_pools: 0,
            performance_counter_multiple_query_pools: 0,
        }
    }
}
impl<'lt> PhysicalDevicePerformanceQueryFeaturesKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::performance_counter_query_pools`]
    pub fn performance_counter_query_pools_raw(&self) -> Bool32 {
        self.performance_counter_query_pools
    }
    ///Gets the raw value of [`Self::performance_counter_multiple_query_pools`]
    pub fn performance_counter_multiple_query_pools_raw(&self) -> Bool32 {
        self.performance_counter_multiple_query_pools
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::performance_counter_query_pools`]
    pub fn set_performance_counter_query_pools_raw(&mut self, value: Bool32) -> &mut Self {
        self.performance_counter_query_pools = value;
        self
    }
    ///Sets the raw value of [`Self::performance_counter_multiple_query_pools`]
    pub fn set_performance_counter_multiple_query_pools_raw(&mut self, value: Bool32) -> &mut Self {
        self.performance_counter_multiple_query_pools = value;
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
    ///Gets the value of [`Self::performance_counter_query_pools`]
    pub fn performance_counter_query_pools(&self) -> bool {
        unsafe { std::mem::transmute(self.performance_counter_query_pools as u8) }
    }
    ///Gets the value of [`Self::performance_counter_multiple_query_pools`]
    pub fn performance_counter_multiple_query_pools(&self) -> bool {
        unsafe { std::mem::transmute(self.performance_counter_multiple_query_pools as u8) }
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
    ///Gets a mutable reference to the value of [`Self::performance_counter_query_pools`]
    pub fn performance_counter_query_pools_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.performance_counter_query_pools as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.performance_counter_query_pools as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::performance_counter_multiple_query_pools`]
    pub fn performance_counter_multiple_query_pools_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.performance_counter_multiple_query_pools as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.performance_counter_multiple_query_pools as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the raw value of [`Self::performance_counter_query_pools`]
    pub fn set_performance_counter_query_pools(&mut self, value: bool) -> &mut Self {
        self.performance_counter_query_pools = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::performance_counter_multiple_query_pools`]
    pub fn set_performance_counter_multiple_query_pools(&mut self, value: bool) -> &mut Self {
        self.performance_counter_multiple_query_pools = value as u8 as u32;
        self
    }
}
///[VkPhysicalDevicePerformanceQueryPropertiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePerformanceQueryPropertiesKHR.html) - Structure describing performance query properties for an implementation
///# C Specifications
///The [`PhysicalDevicePerformanceQueryPropertiesKHR`] structure is defined
///as:
///```c
///// Provided by VK_KHR_performance_query
///typedef struct VkPhysicalDevicePerformanceQueryPropertiesKHR {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           allowCommandBufferQueryCopies;
///} VkPhysicalDevicePerformanceQueryPropertiesKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`allow_command_buffer_query_copies`] is [`TRUE`] if the performance query pools are allowed
///   to be used with [`cmd_copy_query_pool_results`].
///# Description
///If the [`PhysicalDevicePerformanceQueryPropertiesKHR`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceProperties2`] structure passed to
///[`get_physical_device_properties2`], it is filled in with each
///corresponding implementation-dependent property.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PERFORMANCE_QUERY_PROPERTIES_KHR`
///# Related
/// - [`VK_KHR_performance_query`]
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
#[doc(alias = "VkPhysicalDevicePerformanceQueryPropertiesKHR")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDevicePerformanceQueryPropertiesKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`allow_command_buffer_query_copies`] is [`TRUE`] if the performance
    ///query pools are allowed to be used with [`cmd_copy_query_pool_results`].
    pub allow_command_buffer_query_copies: Bool32,
}
impl<'lt> Default for PhysicalDevicePerformanceQueryPropertiesKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PhysicalDevicePerformanceQueryPropertiesKhr,
            p_next: std::ptr::null_mut(),
            allow_command_buffer_query_copies: 0,
        }
    }
}
impl<'lt> PhysicalDevicePerformanceQueryPropertiesKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::allow_command_buffer_query_copies`]
    pub fn allow_command_buffer_query_copies_raw(&self) -> Bool32 {
        self.allow_command_buffer_query_copies
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::allow_command_buffer_query_copies`]
    pub fn set_allow_command_buffer_query_copies_raw(&mut self, value: Bool32) -> &mut Self {
        self.allow_command_buffer_query_copies = value;
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
    ///Gets the value of [`Self::allow_command_buffer_query_copies`]
    pub fn allow_command_buffer_query_copies(&self) -> bool {
        unsafe { std::mem::transmute(self.allow_command_buffer_query_copies as u8) }
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
    ///Gets a mutable reference to the value of [`Self::allow_command_buffer_query_copies`]
    pub fn allow_command_buffer_query_copies_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.allow_command_buffer_query_copies as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.allow_command_buffer_query_copies as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the raw value of [`Self::allow_command_buffer_query_copies`]
    pub fn set_allow_command_buffer_query_copies(&mut self, value: bool) -> &mut Self {
        self.allow_command_buffer_query_copies = value as u8 as u32;
        self
    }
}
///[VkPerformanceCounterKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPerformanceCounterKHR.html) - Structure providing information about a counter
///# C Specifications
///The [`PerformanceCounterKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_performance_query
///typedef struct VkPerformanceCounterKHR {
///    VkStructureType                   sType;
///    void*                             pNext;
///    VkPerformanceCounterUnitKHR       unit;
///    VkPerformanceCounterScopeKHR      scope;
///    VkPerformanceCounterStorageKHR    storage;
///    uint8_t                           uuid[VK_UUID_SIZE];
///} VkPerformanceCounterKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`unit`] is a [`PerformanceCounterUnitKHR`] specifying the unit that the counter data will
///   record.
/// - [`scope`] is a [`PerformanceCounterScopeKHR`] specifying the scope that the counter belongs
///   to.
/// - [`storage`] is a [`PerformanceCounterStorageKHR`] specifying the storage type that the
///   counter’s data uses.
/// - [`uuid`] is an array of size [`UUID_SIZE`], containing 8-bit values that represent a
///   universally unique identifier for the counter of the physical device.
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PERFORMANCE_COUNTER_KHR`
/// - [`p_next`] **must**  be `NULL`
///# Related
/// - [`VK_KHR_performance_query`]
/// - [`PerformanceCounterScopeKHR`]
/// - [`PerformanceCounterStorageKHR`]
/// - [`PerformanceCounterUnitKHR`]
/// - [`StructureType`]
/// - [`enumerate_physical_device_queue_family_performance_query_counters_khr`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPerformanceCounterKHR")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PerformanceCounterKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`unit`] is a [`PerformanceCounterUnitKHR`] specifying the unit
    ///that the counter data will record.
    pub unit: PerformanceCounterUnitKHR,
    ///[`scope`] is a [`PerformanceCounterScopeKHR`] specifying the scope
    ///that the counter belongs to.
    pub scope: PerformanceCounterScopeKHR,
    ///[`storage`] is a [`PerformanceCounterStorageKHR`] specifying the
    ///storage type that the counter’s data uses.
    pub storage: PerformanceCounterStorageKHR,
    ///[`uuid`] is an array of size [`UUID_SIZE`], containing 8-bit
    ///values that represent a universally unique identifier for the counter of
    ///the physical device.
    pub uuid: [u8; UUID_SIZE as usize],
}
impl<'lt> Default for PerformanceCounterKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PerformanceCounterKhr,
            p_next: std::ptr::null_mut(),
            unit: Default::default(),
            scope: Default::default(),
            storage: Default::default(),
            uuid: [0; UUID_SIZE as usize],
        }
    }
}
impl<'lt> PerformanceCounterKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
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
    ///Gets the value of [`Self::unit`]
    pub fn unit(&self) -> PerformanceCounterUnitKHR {
        self.unit
    }
    ///Gets the value of [`Self::scope`]
    pub fn scope(&self) -> PerformanceCounterScopeKHR {
        self.scope
    }
    ///Gets the value of [`Self::storage`]
    pub fn storage(&self) -> PerformanceCounterStorageKHR {
        self.storage
    }
    ///Gets the value of [`Self::uuid`]
    pub fn uuid(&self) -> &[u8; UUID_SIZE as usize] {
        &self.uuid
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
    ///Gets a mutable reference to the value of [`Self::unit`]
    pub fn unit_mut(&mut self) -> &mut PerformanceCounterUnitKHR {
        &mut self.unit
    }
    ///Gets a mutable reference to the value of [`Self::scope`]
    pub fn scope_mut(&mut self) -> &mut PerformanceCounterScopeKHR {
        &mut self.scope
    }
    ///Gets a mutable reference to the value of [`Self::storage`]
    pub fn storage_mut(&mut self) -> &mut PerformanceCounterStorageKHR {
        &mut self.storage
    }
    ///Gets a mutable reference to the value of [`Self::uuid`]
    pub fn uuid_mut(&mut self) -> &mut [u8; UUID_SIZE as usize] {
        &mut self.uuid
    }
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the raw value of [`Self::unit`]
    pub fn set_unit(
        &mut self,
        value: crate::extensions::khr_performance_query::PerformanceCounterUnitKHR,
    ) -> &mut Self {
        self.unit = value;
        self
    }
    ///Sets the raw value of [`Self::scope`]
    pub fn set_scope(
        &mut self,
        value: crate::extensions::khr_performance_query::PerformanceCounterScopeKHR,
    ) -> &mut Self {
        self.scope = value;
        self
    }
    ///Sets the raw value of [`Self::storage`]
    pub fn set_storage(
        &mut self,
        value: crate::extensions::khr_performance_query::PerformanceCounterStorageKHR,
    ) -> &mut Self {
        self.storage = value;
        self
    }
    ///Sets the raw value of [`Self::uuid`]
    pub fn set_uuid(&mut self, value: [u8; crate::core::UUID_SIZE as usize]) -> &mut Self {
        self.uuid = value;
        self
    }
}
///[VkPerformanceCounterDescriptionKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPerformanceCounterDescriptionKHR.html) - Structure providing more detailed information about a counter
///# C Specifications
///The [`PerformanceCounterDescriptionKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_performance_query
///typedef struct VkPerformanceCounterDescriptionKHR {
///    VkStructureType                            sType;
///    void*                                      pNext;
///    VkPerformanceCounterDescriptionFlagsKHR    flags;
///    char                                       name[VK_MAX_DESCRIPTION_SIZE];
///    char                                       category[VK_MAX_DESCRIPTION_SIZE];
///    char                                       description[VK_MAX_DESCRIPTION_SIZE];
///} VkPerformanceCounterDescriptionKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is a bitmask of [`PerformanceCounterDescriptionFlagBitsKHR`] indicating the usage
///   behavior for the counter.
/// - [`name`] is an array of size [`MAX_DESCRIPTION_SIZE`], containing a null-terminated UTF-8
///   string specifying the name of the counter.
/// - [`category`] is an array of size [`MAX_DESCRIPTION_SIZE`], containing a null-terminated UTF-8
///   string specifying the category of the counter.
/// - [`description`] is an array of size [`MAX_DESCRIPTION_SIZE`], containing a null-terminated
///   UTF-8 string specifying the description of the counter.
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PERFORMANCE_COUNTER_DESCRIPTION_KHR`
/// - [`p_next`] **must**  be `NULL`
///# Related
/// - [`VK_KHR_performance_query`]
/// - [`PerformanceCounterDescriptionFlagsKHR`]
/// - [`StructureType`]
/// - [`enumerate_physical_device_queue_family_performance_query_counters_khr`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPerformanceCounterDescriptionKHR")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PerformanceCounterDescriptionKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`flags`] is a bitmask of
    ///[`PerformanceCounterDescriptionFlagBitsKHR`] indicating the usage
    ///behavior for the counter.
    pub flags: PerformanceCounterDescriptionFlagsKHR,
    ///[`name`] is an array of size [`MAX_DESCRIPTION_SIZE`], containing
    ///a null-terminated UTF-8 string specifying the name of the counter.
    pub name: [c_char; MAX_DESCRIPTION_SIZE as usize],
    ///[`category`] is an array of size [`MAX_DESCRIPTION_SIZE`],
    ///containing a null-terminated UTF-8 string specifying the category of the
    ///counter.
    pub category: [c_char; MAX_DESCRIPTION_SIZE as usize],
    ///[`description`] is an array of size [`MAX_DESCRIPTION_SIZE`],
    ///containing a null-terminated UTF-8 string specifying the description of
    ///the counter.
    pub description: [c_char; MAX_DESCRIPTION_SIZE as usize],
}
impl<'lt> Default for PerformanceCounterDescriptionKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PerformanceCounterDescriptionKhr,
            p_next: std::ptr::null_mut(),
            flags: Default::default(),
            name: [b'\0' as i8; MAX_DESCRIPTION_SIZE as usize],
            category: [b'\0' as i8; MAX_DESCRIPTION_SIZE as usize],
            description: [b'\0' as i8; MAX_DESCRIPTION_SIZE as usize],
        }
    }
}
impl<'lt> PerformanceCounterDescriptionKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
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
    ///Gets the value of [`Self::flags`]
    pub fn flags(&self) -> PerformanceCounterDescriptionFlagsKHR {
        self.flags
    }
    ///Gets the value of [`Self::name`]
    pub fn name(&self) -> &[c_char; MAX_DESCRIPTION_SIZE as usize] {
        &self.name
    }
    ///Gets the value of [`Self::category`]
    pub fn category(&self) -> &[c_char; MAX_DESCRIPTION_SIZE as usize] {
        &self.category
    }
    ///Gets the value of [`Self::description`]
    pub fn description(&self) -> &[c_char; MAX_DESCRIPTION_SIZE as usize] {
        &self.description
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
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut PerformanceCounterDescriptionFlagsKHR {
        &mut self.flags
    }
    ///Gets a mutable reference to the value of [`Self::name`]
    pub fn name_mut(&mut self) -> &mut [c_char; MAX_DESCRIPTION_SIZE as usize] {
        &mut self.name
    }
    ///Gets a mutable reference to the value of [`Self::category`]
    pub fn category_mut(&mut self) -> &mut [c_char; MAX_DESCRIPTION_SIZE as usize] {
        &mut self.category
    }
    ///Gets a mutable reference to the value of [`Self::description`]
    pub fn description_mut(&mut self) -> &mut [c_char; MAX_DESCRIPTION_SIZE as usize] {
        &mut self.description
    }
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the raw value of [`Self::flags`]
    pub fn set_flags(
        &mut self,
        value: crate::extensions::khr_performance_query::PerformanceCounterDescriptionFlagsKHR,
    ) -> &mut Self {
        self.flags = value;
        self
    }
    ///Sets the raw value of [`Self::name`]
    pub fn set_name(&mut self, value: [std::os::raw::c_char; crate::core::MAX_DESCRIPTION_SIZE as usize]) -> &mut Self {
        self.name = value;
        self
    }
    ///Sets the raw value of [`Self::category`]
    pub fn set_category(
        &mut self,
        value: [std::os::raw::c_char; crate::core::MAX_DESCRIPTION_SIZE as usize],
    ) -> &mut Self {
        self.category = value;
        self
    }
    ///Sets the raw value of [`Self::description`]
    pub fn set_description(
        &mut self,
        value: [std::os::raw::c_char; crate::core::MAX_DESCRIPTION_SIZE as usize],
    ) -> &mut Self {
        self.description = value;
        self
    }
}
///[VkQueryPoolPerformanceCreateInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueryPoolPerformanceCreateInfoKHR.html) - Structure specifying parameters of a newly created performance query pool
///# C Specifications
///The [`QueryPoolPerformanceCreateInfoKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_performance_query
///typedef struct VkQueryPoolPerformanceCreateInfoKHR {
///    VkStructureType    sType;
///    const void*        pNext;
///    uint32_t           queueFamilyIndex;
///    uint32_t           counterIndexCount;
///    const uint32_t*    pCounterIndices;
///} VkQueryPoolPerformanceCreateInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`queue_family_index`] is the queue family index to create this performance query pool for.
/// - [`counter_index_count`] is the length of the [`counter_indices`] array.
/// - [`counter_indices`] is a pointer to an array of indices into the
///   [`enumerate_physical_device_queue_family_performance_query_counters_khr`]`::pCounters` to
///   enable in this performance query pool.
///# Description
///## Valid Usage
/// - [`queue_family_index`] **must**  be a valid queue family index of the device
/// - The [`performanceCounterQueryPools`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-performanceCounterQueryPools)
///   feature  **must**  be enabled
/// - Each element of [`counter_indices`] **must**  be in the range of counters reported by
///   [`enumerate_physical_device_queue_family_performance_query_counters_khr`] for the queue family
///   specified in [`queue_family_index`]
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_QUERY_POOL_PERFORMANCE_CREATE_INFO_KHR`
/// - [`counter_indices`] **must**  be a valid pointer to an array of
///   [`counter_index_count`]`uint32_t` values
/// - [`counter_index_count`] **must**  be greater than `0`
///# Related
/// - [`VK_KHR_performance_query`]
/// - [`StructureType`]
/// - [`get_physical_device_queue_family_performance_query_passes_khr`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkQueryPoolPerformanceCreateInfoKHR")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct QueryPoolPerformanceCreateInfoKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`queue_family_index`] is the queue family index to create this
    ///performance query pool for.
    pub queue_family_index: u32,
    ///[`counter_index_count`] is the length of the [`counter_indices`]
    ///array.
    pub counter_index_count: u32,
    ///[`counter_indices`] is a pointer to an array of indices into the
    ///[`enumerate_physical_device_queue_family_performance_query_counters_khr`]::`pCounters`
    ///to enable in this performance query pool.
    pub counter_indices: *const u32,
}
impl<'lt> Default for QueryPoolPerformanceCreateInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::QueryPoolPerformanceCreateInfoKhr,
            p_next: std::ptr::null(),
            queue_family_index: 0,
            counter_index_count: 0,
            counter_indices: std::ptr::null(),
        }
    }
}
impl<'lt> QueryPoolPerformanceCreateInfoKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::counter_indices`]
    pub fn counter_indices_raw(&self) -> *const u32 {
        self.counter_indices
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::counter_indices`]
    pub fn set_counter_indices_raw(&mut self, value: *const u32) -> &mut Self {
        self.counter_indices = value;
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
    ///Gets the value of [`Self::queue_family_index`]
    pub fn queue_family_index(&self) -> u32 {
        self.queue_family_index
    }
    ///Gets the value of [`Self::counter_index_count`]
    pub fn counter_index_count(&self) -> u32 {
        self.counter_index_count
    }
    ///Gets the value of [`Self::counter_indices`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn counter_indices(&self) -> &[u32] {
        std::slice::from_raw_parts(self.counter_indices, self.counter_index_count as usize)
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::queue_family_index`]
    pub fn queue_family_index_mut(&mut self) -> &mut u32 {
        &mut self.queue_family_index
    }
    ///Gets a mutable reference to the value of [`Self::counter_index_count`]
    pub fn counter_index_count_mut(&mut self) -> &mut u32 {
        &mut self.counter_index_count
    }
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the raw value of [`Self::queue_family_index`]
    pub fn set_queue_family_index(&mut self, value: u32) -> &mut Self {
        self.queue_family_index = value;
        self
    }
    ///Sets the raw value of [`Self::counter_index_count`]
    pub fn set_counter_index_count(&mut self, value: u32) -> &mut Self {
        self.counter_index_count = value;
        self
    }
    ///Sets the raw value of [`Self::counter_indices`]
    pub fn set_counter_indices(&mut self, value: &'lt [u32]) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.counter_indices = value.as_ptr();
        self.counter_index_count = len_;
        self
    }
}
///[VkAcquireProfilingLockInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAcquireProfilingLockInfoKHR.html) - Structure specifying parameters to acquire the profiling lock
///# C Specifications
///The [`AcquireProfilingLockInfoKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_performance_query
///typedef struct VkAcquireProfilingLockInfoKHR {
///    VkStructureType                   sType;
///    const void*                       pNext;
///    VkAcquireProfilingLockFlagsKHR    flags;
///    uint64_t                          timeout;
///} VkAcquireProfilingLockInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is reserved for future use.
/// - [`timeout`] indicates how long the function waits, in nanoseconds, if the profiling lock is
///   not available.
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_ACQUIRE_PROFILING_LOCK_INFO_KHR`
/// - [`p_next`] **must**  be `NULL`
/// - [`flags`] **must**  be `0`
///If [`timeout`] is 0, [`acquire_profiling_lock_khr`] will not block while
///attempting to acquire the profling lock.
///If [`timeout`] is `UINT64_MAX`, the function will not return until the
///profiling lock was acquired.
///# Related
/// - [`VK_KHR_performance_query`]
/// - [`AcquireProfilingLockFlagsKHR`]
/// - [`StructureType`]
/// - [`acquire_profiling_lock_khr`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkAcquireProfilingLockInfoKHR")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct AcquireProfilingLockInfoKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`flags`] is reserved for future use.
    pub flags: AcquireProfilingLockFlagsKHR,
    ///[`timeout`] indicates how long the function waits, in nanoseconds, if
    ///the profiling lock is not available.
    pub timeout: u64,
}
impl<'lt> Default for AcquireProfilingLockInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::AcquireProfilingLockInfoKhr,
            p_next: std::ptr::null(),
            flags: Default::default(),
            timeout: 0,
        }
    }
}
impl<'lt> AcquireProfilingLockInfoKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
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
    pub fn flags(&self) -> AcquireProfilingLockFlagsKHR {
        self.flags
    }
    ///Gets the value of [`Self::timeout`]
    pub fn timeout(&self) -> u64 {
        self.timeout
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut AcquireProfilingLockFlagsKHR {
        &mut self.flags
    }
    ///Gets a mutable reference to the value of [`Self::timeout`]
    pub fn timeout_mut(&mut self) -> &mut u64 {
        &mut self.timeout
    }
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the raw value of [`Self::flags`]
    pub fn set_flags(
        &mut self,
        value: crate::extensions::khr_performance_query::AcquireProfilingLockFlagsKHR,
    ) -> &mut Self {
        self.flags = value;
        self
    }
    ///Sets the raw value of [`Self::timeout`]
    pub fn set_timeout(&mut self, value: u64) -> &mut Self {
        self.timeout = value;
        self
    }
}
///[VkPerformanceQuerySubmitInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPerformanceQuerySubmitInfoKHR.html) - Structure indicating which counter pass index is active for performance queries
///# C Specifications
///The [`PerformanceQuerySubmitInfoKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_performance_query
///typedef struct VkPerformanceQuerySubmitInfoKHR {
///    VkStructureType    sType;
///    const void*        pNext;
///    uint32_t           counterPassIndex;
///} VkPerformanceQuerySubmitInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`counter_pass_index`] specifies which counter pass index is active.
///# Description
///If the [`SubmitInfo`]::[`p_next`] chain does not include this
///structure, the batch defaults to use counter pass index 0.
///## Valid Usage
/// - [`counter_pass_index`] **must**  be less than the number of counter passes required by any
///   queries within the batch. The required number of counter passes for a performance query is
///   obtained by calling [`get_physical_device_queue_family_performance_query_passes_khr`]
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PERFORMANCE_QUERY_SUBMIT_INFO_KHR`
///# Related
/// - [`VK_KHR_performance_query`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPerformanceQuerySubmitInfoKHR")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PerformanceQuerySubmitInfoKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`counter_pass_index`] specifies which counter pass index is active.
    pub counter_pass_index: u32,
}
impl<'lt> Default for PerformanceQuerySubmitInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PerformanceQuerySubmitInfoKhr,
            p_next: std::ptr::null(),
            counter_pass_index: 0,
        }
    }
}
impl<'lt> PerformanceQuerySubmitInfoKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
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
    ///Gets the value of [`Self::counter_pass_index`]
    pub fn counter_pass_index(&self) -> u32 {
        self.counter_pass_index
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::counter_pass_index`]
    pub fn counter_pass_index_mut(&mut self) -> &mut u32 {
        &mut self.counter_pass_index
    }
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the raw value of [`Self::counter_pass_index`]
    pub fn set_counter_pass_index(&mut self, value: u32) -> &mut Self {
        self.counter_pass_index = value;
        self
    }
}
///[VkPerformanceCounterResultKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPerformanceCounterResultKHR.html) - Union containing a performance counter result
///# C Specifications
///The [`PerformanceCounterResultKHR`] union is defined as:
/// - [`int_32`] is a 32-bit signed integer value.
/// - [`int_64`] is a 64-bit signed integer value.
/// - [`uint_32`] is a 32-bit unsigned integer value.
/// - [`uint_64`] is a 64-bit unsigned integer value.
/// - [`float_32`] is a 32-bit floating-point value.
/// - [`float_64`] is a 64-bit floating-point value.
///Performance query results are returned in an array of
///[`PerformanceCounterResultKHR`] unions containing the data associated
///with each counter in the query, stored in the same order as the counters
///supplied in `pCounterIndices` when creating the performance query.
///The [`PerformanceCounterKHR::unit`] enumeration specifies how to
///parse the counter data.
///```c
///// Provided by VK_KHR_performance_query
///typedef union VkPerformanceCounterResultKHR {
///    int32_t     int32;
///    int64_t     int64;
///    uint32_t    uint32;
///    uint64_t    uint64;
///    float       float32;
///    double      float64;
///} VkPerformanceCounterResultKHR;
///```
///# Related
/// - [`VK_KHR_performance_query`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPerformanceCounterResultKHR")]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub union PerformanceCounterResultKHR {
    ///No documentation found
    pub int_32: i32,
    ///No documentation found
    pub int_64: i64,
    ///No documentation found
    pub uint_32: u32,
    ///No documentation found
    pub uint_64: u64,
    ///No documentation found
    pub float_32: f32,
    ///No documentation found
    pub float_64: f64,
}
impl Default for PerformanceCounterResultKHR {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
///The V-table of [`Instance`] for functions from VK_KHR_performance_query
pub struct InstanceKhrPerformanceQueryVTable {
    ///See [`FNEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKhr`] for more
    /// information.
    pub enumerate_physical_device_queue_family_performance_query_counters_khr:
        FNEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKhr,
    ///See [`FNGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKhr`] for more information.
    pub get_physical_device_queue_family_performance_query_passes_khr:
        FNGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKhr,
}
impl InstanceKhrPerformanceQueryVTable {
    ///Loads the VTable from the owner and the names
    pub fn load<F>(loader_fn: F, loader: Instance) -> Self
    where
        F: Fn(Instance, &'static CStr) -> Option<extern "system" fn()>,
    {
        Self {
            enumerate_physical_device_queue_family_performance_query_counters_khr: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR"),
                ))
            },
            get_physical_device_queue_family_performance_query_passes_khr: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR"),
                ))
            },
        }
    }
    ///Gets [`Self::enumerate_physical_device_queue_family_performance_query_counters_khr`]. See
    /// [`FNEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKhr`] for more information.
    pub fn enumerate_physical_device_queue_family_performance_query_counters_khr(
        &self,
    ) -> FNEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKhr {
        self.enumerate_physical_device_queue_family_performance_query_counters_khr
    }
    ///Gets [`Self::get_physical_device_queue_family_performance_query_passes_khr`]. See
    /// [`FNGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKhr`] for more information.
    pub fn get_physical_device_queue_family_performance_query_passes_khr(
        &self,
    ) -> FNGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKhr {
        self.get_physical_device_queue_family_performance_query_passes_khr
    }
}
///The V-table of [`Device`] for functions from VK_KHR_performance_query
pub struct DeviceKhrPerformanceQueryVTable {
    ///See [`FNAcquireProfilingLockKhr`] for more information.
    pub acquire_profiling_lock_khr: FNAcquireProfilingLockKhr,
    ///See [`FNReleaseProfilingLockKhr`] for more information.
    pub release_profiling_lock_khr: FNReleaseProfilingLockKhr,
}
impl DeviceKhrPerformanceQueryVTable {
    ///Loads the VTable from the owner and the names
    pub fn load<F>(loader_fn: F, loader: Device) -> Self
    where
        F: Fn(Device, &'static CStr) -> Option<extern "system" fn()>,
    {
        Self {
            acquire_profiling_lock_khr: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkAcquireProfilingLockKHR")))
            },
            release_profiling_lock_khr: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkReleaseProfilingLockKHR")))
            },
        }
    }
    ///Gets [`Self::acquire_profiling_lock_khr`]. See [`FNAcquireProfilingLockKhr`] for more
    /// information.
    pub fn acquire_profiling_lock_khr(&self) -> FNAcquireProfilingLockKhr {
        self.acquire_profiling_lock_khr
    }
    ///Gets [`Self::release_profiling_lock_khr`]. See [`FNReleaseProfilingLockKhr`] for more
    /// information.
    pub fn release_profiling_lock_khr(&self) -> FNReleaseProfilingLockKhr {
        self.release_profiling_lock_khr
    }
}
