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
