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
///A queue  **can**  be created with a system-wide priority by adding a
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
///`VK_QUEUE_GLOBAL_PRIORITY_MEDIUM_KHR`.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO_KHR`
/// - [`global_priority`] **must**  be a valid [`QueueGlobalPriorityKHR`] value
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
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct DeviceQueueGlobalPriorityCreateInfoKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`global_priority`] is the system-wide priority associated to this
    ///queue as specified by [`QueueGlobalPriorityEXT`]
    global_priority: QueueGlobalPriorityKHR,
}
impl<'lt> Default for DeviceQueueGlobalPriorityCreateInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            global_priority: Default::default(),
        }
    }
}
impl<'lt> DeviceQueueGlobalPriorityCreateInfoKHR<'lt> {
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
    ///Gets the value of [`Self::global_priority`]
    pub fn global_priority(&self) -> QueueGlobalPriorityKHR {
        self.global_priority
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::global_priority`]
    pub fn global_priority_mut(&mut self) -> &mut QueueGlobalPriorityKHR {
        &mut self.global_priority
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
    ///Sets the raw value of [`Self::global_priority`]
    pub fn set_global_priority(
        &mut self,
        value: crate::extensions::khr_global_priority::QueueGlobalPriorityKHR,
    ) -> &mut Self {
        self.global_priority = value;
        self
    }
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
///[`PhysicalDeviceGlobalPriorityQueryFeaturesKHR`] **can**  also be used in the [`p_next`] chain
/// of
///[`DeviceCreateInfo`] to selectively enable these features.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GLOBAL_PRIORITY_QUERY_FEATURES_KHR`
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
#[derive(Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceGlobalPriorityQueryFeaturesKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseOutStructure<'lt>,
    ///[`global_priority_query`] indicates
    ///whether the implementation supports the ability to query global queue
    ///priorities.
    global_priority_query: Bool32,
}
impl<'lt> Default for PhysicalDeviceGlobalPriorityQueryFeaturesKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            global_priority_query: 0,
        }
    }
}
impl<'lt> PhysicalDeviceGlobalPriorityQueryFeaturesKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::global_priority_query`]
    pub fn global_priority_query_raw(&self) -> Bool32 {
        self.global_priority_query
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::global_priority_query`]
    pub fn set_global_priority_query_raw(&mut self, value: Bool32) -> &mut Self {
        self.global_priority_query = value;
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
    ///Gets the value of [`Self::global_priority_query`]
    pub fn global_priority_query(&self) -> bool {
        unsafe { std::mem::transmute(self.global_priority_query as u8) }
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
    ///Gets a mutable reference to the value of [`Self::global_priority_query`]
    pub fn global_priority_query_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.global_priority_query as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.global_priority_query as *mut Bool32)
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
    ///Sets the raw value of [`Self::global_priority_query`]
    pub fn set_global_priority_query(&mut self, value: bool) -> &mut Self {
        self.global_priority_query = value as u8 as u32;
        self
    }
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
///   and it  **must**  be greater than 0.
/// - [`priorities`] is an array of [`MAX_GLOBAL_PRIORITY_SIZE_EXT`][`QueueGlobalPriorityEXT`] enums
///   representing all supported global queue priorities in this queue family. The first
///   [`priority_count`] elements of the array will be valid.
///# Description
///If the [`QueueFamilyGlobalPriorityPropertiesKHR`] structure is included
///in the [`p_next`] chain of the [`QueueFamilyProperties2`] structure
///passed to [`GetPhysicalDeviceQueueFamilyProperties2`], it is filled in
///with the list of supported global queue priorities for the indicated family.The valid elements
/// of [`priorities`] **must**  not contain any duplicate
///values.The valid elements of [`priorities`] **must**  be a continuous sequence of
///[`QueueGlobalPriorityKHR`] enums in the ascending order.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES_KHR`
/// - Any given element of [`priorities`] **must**  be a valid [`QueueGlobalPriorityKHR`] value
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
#[derive(Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct QueueFamilyGlobalPriorityPropertiesKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseOutStructure<'lt>,
    ///[`priority_count`] is the number of supported global queue priorities
    ///in this queue family, and it  **must**  be greater than 0.
    priority_count: u32,
    ///[`priorities`] is an array of [`MAX_GLOBAL_PRIORITY_SIZE_EXT`][`QueueGlobalPriorityEXT`]
    /// enums representing all supported global queue priorities in this queue family.
    ///The first [`priority_count`] elements of the array will be valid.
    priorities: [QueueGlobalPriorityKHR; MAX_GLOBAL_PRIORITY_SIZE_KHR],
}
impl<'lt> Default for QueueFamilyGlobalPriorityPropertiesKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            priority_count: 0,
            priorities: [Default::default(); MAX_GLOBAL_PRIORITY_SIZE_KHR],
        }
    }
}
impl<'lt> QueueFamilyGlobalPriorityPropertiesKHR<'lt> {
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
    ///Gets the value of [`Self::priority_count`]
    pub fn priority_count(&self) -> u32 {
        self.priority_count
    }
    ///Gets the value of [`Self::priorities`]
    pub fn priorities(&self) -> &[QueueGlobalPriorityKHR; MAX_GLOBAL_PRIORITY_SIZE_KHR] {
        &getter
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
    ///Gets a mutable reference to the value of [`Self::priority_count`]
    pub fn priority_count_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::priorities`]
    pub fn priorities_mut(&mut self) -> &mut [QueueGlobalPriorityKHR; MAX_GLOBAL_PRIORITY_SIZE_KHR] {
        &mut getter
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
    ///Sets the raw value of [`Self::priority_count`]
    pub fn set_priority_count(&mut self, value: u32) -> &mut Self {
        self.priority_count = value;
        self
    }
    ///Sets the raw value of [`Self::priorities`]
    pub fn set_priorities(
        &mut self,
        value: [crate::extensions::khr_global_priority::QueueGlobalPriorityKHR;
            crate::extensions::khr_global_priority::MAX_GLOBAL_PRIORITY_SIZE_KHR],
    ) -> &mut Self {
        self.priorities = value;
        self
    }
}
