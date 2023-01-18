use crate::{
    cstr,
    vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, StructureType, MAX_GLOBAL_PRIORITY_SIZE_KHR},
};
use std::ffi::CStr;
#[doc(alias = "VkDeviceQueueGlobalPriorityCreateInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DeviceQueueGlobalPriorityCreateInfoKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "globalPriority")]
    global_priority: QueueGlobalPriorityKHR,
}
#[doc(alias = "VkPhysicalDeviceGlobalPriorityQueryFeaturesKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceGlobalPriorityQueryFeaturesKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "globalPriorityQuery")]
    global_priority_query: Bool32,
}
#[doc(alias = "VkQueueFamilyGlobalPriorityPropertiesKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct QueueFamilyGlobalPriorityPropertiesKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "priorityCount")]
    priority_count: u32,
    priorities: [QueueGlobalPriorityKHR; MAX_GLOBAL_PRIORITY_SIZE_KHR as usize],
}
#[doc(alias = "VK_KHR_GLOBAL_PRIORITY_SPEC_VERSION")]
pub const KHR_GLOBAL_PRIORITY_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_KHR_GLOBAL_PRIORITY_EXTENSION_NAME")]
pub const KHR_GLOBAL_PRIORITY_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_global_priority");
#[doc(alias = "VkQueueGlobalPriorityKHR")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct QueueGlobalPriorityKHR(i32);
impl QueueGlobalPriorityKHR {
    #[doc(alias = "VK_QUEUE_GLOBAL_PRIORITY_LOW_KHR")]
    pub const LOW: Self = Self(128);
    #[doc(alias = "VK_QUEUE_GLOBAL_PRIORITY_MEDIUM_KHR")]
    pub const MEDIUM: Self = Self(256);
    #[doc(alias = "VK_QUEUE_GLOBAL_PRIORITY_HIGH_KHR")]
    pub const HIGH: Self = Self(512);
    #[doc(alias = "VK_QUEUE_GLOBAL_PRIORITY_REALTIME_KHR")]
    pub const REALTIME: Self = Self(1024);
    #[doc(alias = "VK_QUEUE_GLOBAL_PRIORITY_LOW_EXT")]
    pub const LOW_EXT: Self = Self::LOW;
    #[doc(alias = "VK_QUEUE_GLOBAL_PRIORITY_MEDIUM_EXT")]
    pub const MEDIUM_EXT: Self = Self::MEDIUM;
    #[doc(alias = "VK_QUEUE_GLOBAL_PRIORITY_HIGH_EXT")]
    pub const HIGH_EXT: Self = Self::HIGH;
    #[doc(alias = "VK_QUEUE_GLOBAL_PRIORITY_REALTIME_EXT")]
    pub const REALTIME_EXT: Self = Self::REALTIME;
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
    ///The bits of this variant
    #[inline]
    pub const fn bits(&self) -> i32 {
        self.0
    }
    ///Builds a bitmask from the bits of this variant
    #[inline]
    pub const fn from_bits(bits: i32) -> Option<Self> {
        match bits {
            x if x == Self::LOW.bits() => Some(Self(x)),
            x if x == Self::MEDIUM.bits() => Some(Self(x)),
            x if x == Self::HIGH.bits() => Some(Self(x)),
            x if x == Self::REALTIME.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: i32) -> Self {
        Self(bits)
    }
}
