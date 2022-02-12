//![VK_KHR_global_priority](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_global_priority.html) - device extension
//!# Description
//!In Vulkan, users can specify device-scope queue priorities.
//!In some cases it may be useful to extend this concept to a system-wide
//!scope.
//!This device extension allows applications to query the global queue
//!priorities supported by a queue family, and then set a priority when
//!creating queues.
//!The default queue priority is `VK_QUEUE_GLOBAL_PRIORITY_MEDIUM_EXT`.Implementations can report
//! which global priority levels are treated
//!differently by the implementation.
//!It is intended primarily for use in system integration along with certain
//!platform-specific priority enforcement rules.The driver implementation will attempt to skew
//! hardware resource allocation
//!in favour of the higher-priority task.
//!Therefore, higher-priority work may retain similar latency and throughput
//!characteristics even if the system is congested with lower priority work.The global priority
//! level of a queue shall take precedence over the
//!per-process queue priority
//!([`DeviceQueueCreateInfo::p_queue_priorities`]).Abuse of this feature may result in starving the
//! rest of the system from
//!hardware resources.
//!Therefore, the driver implementation may deny requests to acquire a priority
//!above the default priority (`VK_QUEUE_GLOBAL_PRIORITY_MEDIUM_EXT`) if
//!the caller does not have sufficient privileges.
//!In this scenario `VK_ERROR_NOT_PERMITTED_EXT` is returned.The driver implementation may fail the
//! queue allocation request if resources
//!required to complete the operation have been exhausted (either by the same
//!process or a different process).
//!In this scenario `VK_ERROR_INITIALIZATION_FAILED` is returned.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//!# Contacts
//! - Tobias Hector [tobski](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_global_priority]
//!   @tobski%0A<<Here describe the issue or question you have about the VK_KHR_global_priority
//!   extension>>)
//!# New structures
//! - Extending [`DeviceQueueCreateInfo`]:
//! - [`DeviceQueueGlobalPriorityCreateInfoKHR`]
//!
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:
//! - [`PhysicalDeviceGlobalPriorityQueryFeaturesKHR`]
//!
//! - Extending [`QueueFamilyProperties2`]:
//! - [`QueueFamilyGlobalPriorityPropertiesKHR`]
//!# New enums
//! - [`QueueGlobalPriorityKHR`]
//!# New constants
//! - [`KHR_GLOBAL_PRIORITY_EXTENSION_NAME`]
//! - [`KHR_GLOBAL_PRIORITY_SPEC_VERSION`]
//! - [`MAX_GLOBAL_PRIORITY_SIZE_KHR`]
//! - Extending [`VulkanResultCodes`]:
//! - `VK_ERROR_NOT_PERMITTED_KHR`
//!
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO_KHR`
//! - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GLOBAL_PRIORITY_QUERY_FEATURES_KHR`
//! - `VK_STRUCTURE_TYPE_QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES_KHR`
//!# Known issues & F.A.Q
//!1) Can we additionally query whether a caller is permitted to acquire a
//!specific global queue priority in this extension?**RESOLVED**: No.
//!Whether a caller has enough privilege goes with the OS, and the Vulkan
//!driver cannot really guarantee that the privilege will not change in between
//!this query and the actual queue creation call.2) If more than 1 queue using global priority is
//! requested, is there a good
//!way to know which queue is failing the device creation?**RESOLVED**: No.
//!There is not a good way at this moment, and it is also not quite actionable
//!for the applications to know that because the information may not be
//!accurate.
//!Queue creation can fail because of runtime constraints like insufficient
//!privilege or lack of resource, and the failure is not necessarily tied to
//!that particular queue configuration requested.
//!# Version History
//! - Revision 1, 2021-10-22 (Tobias Hector)
//! - Initial draft
//!# Other info
//! * 2021-10-22
//!*
//! - Tobias Hector, AMD
//! - Contributors to [`VK_EXT_global_priority`]
//! - Contributors to [`VK_EXT_global_priority_query`]
//!# Related
//! - [`MAX_GLOBAL_PRIORITY_SIZE_KHR`]
//! - [`DeviceQueueGlobalPriorityCreateInfoKHR`]
//! - [`PhysicalDeviceGlobalPriorityQueryFeaturesKHR`]
//! - [`QueueFamilyGlobalPriorityPropertiesKHR`]
//! - [`QueueGlobalPriorityKHR`]
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
///[VK_MAX_GLOBAL_PRIORITY_SIZE_KHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_MAX_GLOBAL_PRIORITY_SIZE_KHR.html) - Length of an array of global queue priorities
///# C Specifications
///[`MAX_GLOBAL_PRIORITY_SIZE_KHR`] is the length of an array of
///[`QueueGlobalPriorityKHR`] enumerants representing supported queue
///priorities, as returned in
///[`QueueFamilyGlobalPriorityPropertiesKHR::priorities`].
///```c
///#define VK_MAX_GLOBAL_PRIORITY_SIZE_KHR   16U
///```
///or the equivalent
///```c
///#define VK_MAX_GLOBAL_PRIORITY_SIZE_EXT   VK_MAX_GLOBAL_PRIORITY_SIZE_KHR
///```
///# Related
/// - [`VK_KHR_global_priority`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VK_MAX_GLOBAL_PRIORITY_SIZE_KHR")]
pub const MAX_GLOBAL_PRIORITY_SIZE_KHR: u32 = 16;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_GLOBAL_PRIORITY_SPEC_VERSION")]
pub const KHR_GLOBAL_PRIORITY_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_GLOBAL_PRIORITY_EXTENSION_NAME")]
pub const KHR_GLOBAL_PRIORITY_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_global_priority");
///[VkQueueGlobalPriorityKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueueGlobalPriorityKHR.html) - Values specifying a system-wide queue priority
///# C Specifications
///Possible values of
///[`DeviceQueueGlobalPriorityCreateInfoKHR::global_priority`],
///specifying a system-wide priority level are:
///```c
///// Provided by VK_KHR_global_priority
///typedef enum VkQueueGlobalPriorityKHR {
///    VK_QUEUE_GLOBAL_PRIORITY_LOW_KHR = 128,
///    VK_QUEUE_GLOBAL_PRIORITY_MEDIUM_KHR = 256,
///    VK_QUEUE_GLOBAL_PRIORITY_HIGH_KHR = 512,
///    VK_QUEUE_GLOBAL_PRIORITY_REALTIME_KHR = 1024,
///    VK_QUEUE_GLOBAL_PRIORITY_LOW_EXT = VK_QUEUE_GLOBAL_PRIORITY_LOW_KHR,
///    VK_QUEUE_GLOBAL_PRIORITY_MEDIUM_EXT = VK_QUEUE_GLOBAL_PRIORITY_MEDIUM_KHR,
///    VK_QUEUE_GLOBAL_PRIORITY_HIGH_EXT = VK_QUEUE_GLOBAL_PRIORITY_HIGH_KHR,
///    VK_QUEUE_GLOBAL_PRIORITY_REALTIME_EXT = VK_QUEUE_GLOBAL_PRIORITY_REALTIME_KHR,
///} VkQueueGlobalPriorityKHR;
///```
///or the equivalent
///```c
///// Provided by VK_EXT_global_priority
///typedef VkQueueGlobalPriorityKHR VkQueueGlobalPriorityEXT;
///```
///# Description
///Priority values are sorted in ascending order.
///A comparison operation on the enum values can be used to determine the
///priority order.
/// - [`QUEUE_GLOBAL_PRIORITY_LOW`] is below the system default.
///Useful for non-interactive tasks.
/// - [`QUEUE_GLOBAL_PRIORITY_MEDIUM`] is the system default
///priority.
/// - [`QUEUE_GLOBAL_PRIORITY_HIGH`] is above the system default.
/// - [`QUEUE_GLOBAL_PRIORITY_REALTIME`] is the highest priority.
///Useful for critical tasks.
///# Related
/// - [`VK_EXT_global_priority`]
/// - [`VK_KHR_global_priority`]
/// - [`DeviceQueueGlobalPriorityCreateInfoKHR`]
/// - [`QueueFamilyGlobalPriorityPropertiesKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkQueueGlobalPriorityKHR")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct QueueGlobalPriorityKHR(i32);
impl const Default for QueueGlobalPriorityKHR {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for QueueGlobalPriorityKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple("QueueGlobalPriorityKHR")
            .field(match *self {
                Self::QUEUE_GLOBAL_PRIORITY_LOW => &"QUEUE_GLOBAL_PRIORITY_LOW",
                Self::QUEUE_GLOBAL_PRIORITY_MEDIUM => &"QUEUE_GLOBAL_PRIORITY_MEDIUM",
                Self::QUEUE_GLOBAL_PRIORITY_HIGH => &"QUEUE_GLOBAL_PRIORITY_HIGH",
                Self::QUEUE_GLOBAL_PRIORITY_REALTIME => &"QUEUE_GLOBAL_PRIORITY_REALTIME",
                other => unreachable!("invalid value for `QueueGlobalPriorityKHR`: {:?}", other),
            })
            .finish()
    }
}
impl QueueGlobalPriorityKHR {
    ///[`QUEUE_GLOBAL_PRIORITY_LOW`] is below the system default.
    ///Useful for non-interactive tasks.
    pub const QUEUE_GLOBAL_PRIORITY_LOW: Self = Self(128);
    ///[`QUEUE_GLOBAL_PRIORITY_MEDIUM`] is the system default
    ///priority.
    pub const QUEUE_GLOBAL_PRIORITY_MEDIUM: Self = Self(256);
    ///[`QUEUE_GLOBAL_PRIORITY_HIGH`] is above the system default.
    pub const QUEUE_GLOBAL_PRIORITY_HIGH: Self = Self(512);
    ///[`QUEUE_GLOBAL_PRIORITY_REALTIME`] is the highest priority.
    ///Useful for critical tasks.
    pub const QUEUE_GLOBAL_PRIORITY_REALTIME: Self = Self(1024);
    ///No documentation found
    pub const QUEUE_GLOBAL_PRIORITY_LOW_EXT: Self = Self::QUEUE_GLOBAL_PRIORITY_LOW;
    ///No documentation found
    pub const QUEUE_GLOBAL_PRIORITY_MEDIUM_EXT: Self = Self::QUEUE_GLOBAL_PRIORITY_MEDIUM;
    ///No documentation found
    pub const QUEUE_GLOBAL_PRIORITY_HIGH_EXT: Self = Self::QUEUE_GLOBAL_PRIORITY_HIGH;
    ///No documentation found
    pub const QUEUE_GLOBAL_PRIORITY_REALTIME_EXT: Self = Self::QUEUE_GLOBAL_PRIORITY_REALTIME;
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
