pub use crate::common::extensions::khr_global_priority::{
    QueueGlobalPriorityKHR, KHR_GLOBAL_PRIORITY_EXTENSION_NAME, KHR_GLOBAL_PRIORITY_SPEC_VERSION,
};
use crate::{
    context::Context,
    vulkan1_0::{StructureType, MAX_GLOBAL_PRIORITY_SIZE_KHR},
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkDeviceQueueGlobalPriorityCreateInfoKHR")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DeviceQueueGlobalPriorityCreateInfoKHR {
    #[doc(alias = "globalPriority")]
    pub global_priority: QueueGlobalPriorityKHR,
}
impl DeviceQueueGlobalPriorityCreateInfoKHR {
    ///Get a reference to the `global_priority` field.
    pub fn global_priority(&self) -> QueueGlobalPriorityKHR {
        self.global_priority
    }
    ///Get a mutable reference to the `global_priority` field.
    pub fn global_priority_mut(&mut self) -> &mut QueueGlobalPriorityKHR {
        &mut self.global_priority
    }
    ///Sets the `global_priority` field.
    pub fn set_global_priority(&mut self, global_priority: QueueGlobalPriorityKHR) -> &mut Self {
        self.global_priority = global_priority;
        self
    }
    ///Sets the `global_priority` field in a builder way.
    pub fn with_global_priority(mut self, global_priority: QueueGlobalPriorityKHR) -> Self {
        self.global_priority = global_priority;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for DeviceQueueGlobalPriorityCreateInfoKHR {
    type LowLevel = crate::native::extensions::khr_global_priority::DeviceQueueGlobalPriorityCreateInfoKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::khr_global_priority::DeviceQueueGlobalPriorityCreateInfoKHR {
            s_type: StructureType::DeviceQueueGlobalPriorityCreateInfoKhr,
            p_next: std::ptr::null(),
            global_priority: self.global_priority.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for DeviceQueueGlobalPriorityCreateInfoKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            global_priority: crate::conv::FromLowLevel::from_low_level(context, value.global_priority),
        }
    }
}
#[doc(alias = "VkPhysicalDeviceGlobalPriorityQueryFeaturesKHR")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceGlobalPriorityQueryFeaturesKHR {
    #[doc(alias = "globalPriorityQuery")]
    pub global_priority_query: bool,
}
impl PhysicalDeviceGlobalPriorityQueryFeaturesKHR {
    ///Get a reference to the `global_priority_query` field.
    pub fn global_priority_query(&self) -> &bool {
        &self.global_priority_query
    }
    ///Get a mutable reference to the `global_priority_query` field.
    pub fn global_priority_query_mut(&mut self) -> &mut bool {
        &mut self.global_priority_query
    }
    ///Sets the `global_priority_query` field.
    pub fn set_global_priority_query(&mut self, global_priority_query: bool) -> &mut Self {
        self.global_priority_query = global_priority_query;
        self
    }
    ///Sets the `global_priority_query` field in a builder way.
    pub fn with_global_priority_query(mut self, global_priority_query: bool) -> Self {
        self.global_priority_query = global_priority_query;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceGlobalPriorityQueryFeaturesKHR {
    type LowLevel = crate::native::extensions::khr_global_priority::PhysicalDeviceGlobalPriorityQueryFeaturesKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::khr_global_priority::PhysicalDeviceGlobalPriorityQueryFeaturesKHR {
            s_type: StructureType::PhysicalDeviceGlobalPriorityQueryFeaturesKhr,
            p_next: std::ptr::null_mut(),
            global_priority_query: self.global_priority_query.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceGlobalPriorityQueryFeaturesKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            global_priority_query: crate::conv::FromLowLevel::from_low_level(context, value.global_priority_query),
        }
    }
}
#[doc(alias = "VkQueueFamilyGlobalPriorityPropertiesKHR")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueueFamilyGlobalPriorityPropertiesKHR {
    #[doc(alias = "priorityCount")]
    pub priority_count: u32,
    pub priorities: [QueueGlobalPriorityKHR; MAX_GLOBAL_PRIORITY_SIZE_KHR as usize],
}
impl QueueFamilyGlobalPriorityPropertiesKHR {
    ///Get a reference to the `priority_count` field.
    pub fn priority_count(&self) -> u32 {
        self.priority_count
    }
    ///Get a reference to the `priorities` field.
    pub fn priorities(&self) -> [QueueGlobalPriorityKHR; MAX_GLOBAL_PRIORITY_SIZE_KHR as usize] {
        self.priorities
    }
    ///Get a mutable reference to the `priority_count` field.
    pub fn priority_count_mut(&mut self) -> &mut u32 {
        &mut self.priority_count
    }
    ///Get a mutable reference to the `priorities` field.
    pub fn priorities_mut(&mut self) -> &mut [QueueGlobalPriorityKHR; MAX_GLOBAL_PRIORITY_SIZE_KHR as usize] {
        &mut self.priorities
    }
    ///Sets the `priority_count` field.
    pub fn set_priority_count(&mut self, priority_count: u32) -> &mut Self {
        self.priority_count = priority_count;
        self
    }
    ///Sets the `priorities` field.
    pub fn set_priorities(
        &mut self,
        priorities: [QueueGlobalPriorityKHR; MAX_GLOBAL_PRIORITY_SIZE_KHR as usize],
    ) -> &mut Self {
        self.priorities = priorities;
        self
    }
    ///Sets the `priority_count` field in a builder way.
    pub fn with_priority_count(mut self, priority_count: u32) -> Self {
        self.priority_count = priority_count;
        self
    }
    ///Sets the `priorities` field in a builder way.
    pub fn with_priorities(
        mut self,
        priorities: [QueueGlobalPriorityKHR; MAX_GLOBAL_PRIORITY_SIZE_KHR as usize],
    ) -> Self {
        self.priorities = priorities;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for QueueFamilyGlobalPriorityPropertiesKHR {
    type LowLevel = crate::native::extensions::khr_global_priority::QueueFamilyGlobalPriorityPropertiesKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::khr_global_priority::QueueFamilyGlobalPriorityPropertiesKHR {
            s_type: StructureType::QueueFamilyGlobalPriorityPropertiesKhr,
            p_next: std::ptr::null_mut(),
            priority_count: self.priority_count.into_low_level(context, bump),
            priorities: self.priorities.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for QueueFamilyGlobalPriorityPropertiesKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            priority_count: crate::conv::FromLowLevel::from_low_level(context, value.priority_count),
            priorities: crate::conv::FromLowLevel::from_low_level(context, value.priorities),
        }
    }
}
