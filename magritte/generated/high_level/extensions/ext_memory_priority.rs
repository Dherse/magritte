pub use crate::common::extensions::ext_memory_priority::{
    EXT_MEMORY_PRIORITY_EXTENSION_NAME, EXT_MEMORY_PRIORITY_SPEC_VERSION,
};
use crate::{context::Context, vulkan1_0::StructureType};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkPhysicalDeviceMemoryPriorityFeaturesEXT")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceMemoryPriorityFeaturesEXT {
    #[doc(alias = "memoryPriority")]
    pub memory_priority: bool,
}
impl PhysicalDeviceMemoryPriorityFeaturesEXT {
    ///Get a reference to the `memory_priority` field.
    pub fn memory_priority(&self) -> &bool {
        &self.memory_priority
    }
    ///Get a mutable reference to the `memory_priority` field.
    pub fn memory_priority_mut(&mut self) -> &mut bool {
        &mut self.memory_priority
    }
    ///Sets the `memory_priority` field.
    pub fn set_memory_priority(&mut self, memory_priority: bool) -> &mut Self {
        self.memory_priority = memory_priority;
        self
    }
    ///Sets the `memory_priority` field in a builder way.
    pub fn with_memory_priority(mut self, memory_priority: bool) -> Self {
        self.memory_priority = memory_priority;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceMemoryPriorityFeaturesEXT {
    type LowLevel = crate::native::extensions::ext_memory_priority::PhysicalDeviceMemoryPriorityFeaturesEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_memory_priority::PhysicalDeviceMemoryPriorityFeaturesEXT {
            s_type: StructureType::PhysicalDeviceMemoryPriorityFeaturesExt,
            p_next: std::ptr::null_mut(),
            memory_priority: self.memory_priority.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceMemoryPriorityFeaturesEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            memory_priority: crate::conv::FromLowLevel::from_low_level(context, value.memory_priority),
        }
    }
}
#[doc(alias = "VkMemoryPriorityAllocateInfoEXT")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MemoryPriorityAllocateInfoEXT {
    pub priority: f32,
}
impl MemoryPriorityAllocateInfoEXT {
    ///Get a reference to the `priority` field.
    pub fn priority(&self) -> f32 {
        self.priority
    }
    ///Get a mutable reference to the `priority` field.
    pub fn priority_mut(&mut self) -> &mut f32 {
        &mut self.priority
    }
    ///Sets the `priority` field.
    pub fn set_priority(&mut self, priority: f32) -> &mut Self {
        self.priority = priority;
        self
    }
    ///Sets the `priority` field in a builder way.
    pub fn with_priority(mut self, priority: f32) -> Self {
        self.priority = priority;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for MemoryPriorityAllocateInfoEXT {
    type LowLevel = crate::native::extensions::ext_memory_priority::MemoryPriorityAllocateInfoEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_memory_priority::MemoryPriorityAllocateInfoEXT {
            s_type: StructureType::MemoryPriorityAllocateInfoExt,
            p_next: std::ptr::null(),
            priority: self.priority.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for MemoryPriorityAllocateInfoEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            priority: crate::conv::FromLowLevel::from_low_level(context, value.priority),
        }
    }
}
