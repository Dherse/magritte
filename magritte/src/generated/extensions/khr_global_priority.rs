use crate::vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, StructureType};
#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{ffi::CStr, marker::PhantomData};
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
/// - [`QueueGlobalPriorityLowKhr`] is below the system default. Useful for non-interactive tasks.
/// - [`QueueGlobalPriorityMediumKhr`] is the system default priority.
/// - [`QueueGlobalPriorityHighKhr`] is above the system default.
/// - [`QueueGlobalPriorityRealtimeKhr`] is the highest priority. Useful for critical tasks.
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(i32)]
pub enum QueueGlobalPriorityKHR {
    #[doc(hidden)]
    Empty = 0,
    ///[`QueueGlobalPriorityLowKhr`] is below the system default.
    ///Useful for non-interactive tasks.
    QueueGlobalPriorityLowKhr = 128,
    ///[`QueueGlobalPriorityMediumKhr`] is the system default
    ///priority.
    QueueGlobalPriorityMediumKhr = 256,
    ///[`QueueGlobalPriorityHighKhr`] is above the system default.
    QueueGlobalPriorityHighKhr = 512,
    ///[`QueueGlobalPriorityRealtimeKhr`] is the highest priority.
    ///Useful for critical tasks.
    QueueGlobalPriorityRealtimeKhr = 1024,
}
impl const Default for QueueGlobalPriorityKHR {
    fn default() -> Self {
        Empty
    }
}
impl QueueGlobalPriorityKHR {
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> i32 {
        self as i32
    }
    ///Gets a value from a raw underlying value, unchecked and therefore unsafe
    #[inline]
    pub const unsafe fn from_bits(bits: i32) -> i32 {
        std::mem::transmute(bits)
    }
}
///[VkDeviceQueueGlobalPriorityCreateInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceQueueGlobalPriorityCreateInfoKHR.html) - Specify a system wide priority
///# C Specifications
///A queue **can** be created with a system-wide priority by adding a
///[`DeviceQueueGlobalPriorityCreateInfoKHR`] structure to the [`p_next`]
///chain of [`DeviceQueueCreateInfo`].The [`DeviceQueueGlobalPriorityCreateInfoKHR`] structure is
/// defined as:
///```c
///// Provided by VK_KHR_global_priority
///typedef struct VkDeviceQueueGlobalPriorityCreateInfoKHR {
///    VkStructureType             sType;
///    const void*                 pNext;
///    VkQueueGlobalPriorityKHR    globalPriority;
///} VkDeviceQueueGlobalPriorityCreateInfoKHR;
///```
///or the equivalent
///```c
///// Provided by VK_EXT_global_priority
///typedef VkDeviceQueueGlobalPriorityCreateInfoKHR VkDeviceQueueGlobalPriorityCreateInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`global_priority`] is the system-wide priority associated to this queue as specified by
///   [`QueueGlobalPriorityEXT`]
///# Description
///A queue created without specifying
///[`DeviceQueueGlobalPriorityCreateInfoKHR`] will default to
///`VK_QUEUE_GLOBAL_PRIORITY_MEDIUM_KHR`.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO_KHR`
/// - [`global_priority`]**must** be a valid [`QueueGlobalPriorityKHR`] value
///# Related
/// - [`VK_EXT_global_priority`]
/// - [`VK_KHR_global_priority`]
/// - [`QueueGlobalPriorityKHR`]
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
pub struct DeviceQueueGlobalPriorityCreateInfoKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`global_priority`] is the system-wide priority associated to this
    ///queue as specified by [`QueueGlobalPriorityEXT`]
    global_priority: QueueGlobalPriorityKHR,
}
///[VkPhysicalDeviceGlobalPriorityQueryFeaturesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceGlobalPriorityQueryFeaturesKHR.html) - Structure describing whether global priority query can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceGlobalPriorityQueryFeaturesKHR`] structure is
///defined as:
///```c
///// Provided by VK_KHR_global_priority
///typedef struct VkPhysicalDeviceGlobalPriorityQueryFeaturesKHR {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           globalPriorityQuery;
///} VkPhysicalDeviceGlobalPriorityQueryFeaturesKHR;
///```
///or the equivalent
///```c
///// Provided by VK_EXT_global_priority_query
///typedef VkPhysicalDeviceGlobalPriorityQueryFeaturesKHR
/// VkPhysicalDeviceGlobalPriorityQueryFeaturesEXT;
///```
///# Members
///The members of the [`PhysicalDeviceGlobalPriorityQueryFeaturesEXT`]
///structure describe the following features:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`global_priority_query`] indicates whether the implementation supports the ability to query
///   global queue priorities.
///If the [`PhysicalDeviceGlobalPriorityQueryFeaturesKHR`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceGlobalPriorityQueryFeaturesKHR`]**can** also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GLOBAL_PRIORITY_QUERY_FEATURES_KHR`
///# Related
/// - [`VK_KHR_global_priority`]
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
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct PhysicalDeviceGlobalPriorityQueryFeaturesKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`global_priority_query`] indicates
    ///whether the implementation supports the ability to query global queue
    ///priorities.
    global_priority_query: Bool32,
}
///[VkQueueFamilyGlobalPriorityPropertiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueueFamilyGlobalPriorityPropertiesKHR.html) - Return structure for queue family global priority information query
///# C Specifications
///The definition of [`QueueFamilyGlobalPriorityPropertiesKHR`] is:
///```c
///// Provided by VK_KHR_global_priority
///typedef struct VkQueueFamilyGlobalPriorityPropertiesKHR {
///    VkStructureType             sType;
///    void*                       pNext;
///    uint32_t                    priorityCount;
///    VkQueueGlobalPriorityKHR    priorities[VK_MAX_GLOBAL_PRIORITY_SIZE_KHR];
///} VkQueueFamilyGlobalPriorityPropertiesKHR;
///```
///or the equivalent
///```c
///// Provided by VK_EXT_global_priority_query
///typedef VkQueueFamilyGlobalPriorityPropertiesKHR VkQueueFamilyGlobalPriorityPropertiesEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`priority_count`] is the number of supported global queue priorities in this queue family,
///   and it **must** be greater than 0.
/// - [`priorities`] is an array of [`MAX_GLOBAL_PRIORITY_SIZE_EXT`][`QueueGlobalPriorityEXT`] enums
///   representing all supported global queue priorities in this queue family. The first
///   [`priority_count`] elements of the array will be valid.
///# Description
///If the [`QueueFamilyGlobalPriorityPropertiesKHR`] structure is included
///in the [`p_next`] chain of the [`QueueFamilyProperties2`] structure
///passed to [`GetPhysicalDeviceQueueFamilyProperties2`], it is filled in
///with the list of supported global queue priorities for the indicated family.The valid elements
/// of [`priorities`]**must** not contain any duplicate
///values.The valid elements of [`priorities`]**must** be a continuous sequence of
///[`QueueGlobalPriorityKHR`] enums in the ascending order.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES_KHR`
/// - Any given element of [`priorities`]**must** be a valid [`QueueGlobalPriorityKHR`] value
///# Related
/// - [`VK_KHR_global_priority`]
/// - [`QueueGlobalPriorityKHR`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct QueueFamilyGlobalPriorityPropertiesKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`priority_count`] is the number of supported global queue priorities
    ///in this queue family, and it **must** be greater than 0.
    priority_count: u32,
    ///[`priorities`] is an array of [`MAX_GLOBAL_PRIORITY_SIZE_EXT`][`QueueGlobalPriorityEXT`]
    /// enums representing all supported global queue priorities in this queue family.
    ///The first [`priority_count`] elements of the array will be valid.
    priorities: [QueueGlobalPriorityKHR; MAX_GLOBAL_PRIORITY_SIZE_KHR],
}
