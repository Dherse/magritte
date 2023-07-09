pub use crate::common::extensions::ext_memory_budget::{
    EXT_MEMORY_BUDGET_EXTENSION_NAME, EXT_MEMORY_BUDGET_SPEC_VERSION,
};
use crate::{
    context::Context,
    vulkan1_0::{DeviceSize, StructureType, MAX_MEMORY_HEAPS},
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkPhysicalDeviceMemoryBudgetPropertiesEXT")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceMemoryBudgetPropertiesEXT {
    #[doc(alias = "heapBudget")]
    pub heap_budget: [DeviceSize; MAX_MEMORY_HEAPS as usize],
    #[doc(alias = "heapUsage")]
    pub heap_usage: [DeviceSize; MAX_MEMORY_HEAPS as usize],
}
impl PhysicalDeviceMemoryBudgetPropertiesEXT {
    ///Get a reference to the `heap_budget` field.
    pub fn heap_budget(&self) -> &[DeviceSize; MAX_MEMORY_HEAPS as usize] {
        &self.heap_budget
    }
    ///Get a reference to the `heap_usage` field.
    pub fn heap_usage(&self) -> &[DeviceSize; MAX_MEMORY_HEAPS as usize] {
        &self.heap_usage
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceMemoryBudgetPropertiesEXT {
    type LowLevel = crate::native::extensions::ext_memory_budget::PhysicalDeviceMemoryBudgetPropertiesEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_memory_budget::PhysicalDeviceMemoryBudgetPropertiesEXT {
            s_type: StructureType::PhysicalDeviceMemoryBudgetPropertiesExt,
            p_next: std::ptr::null_mut(),
            heap_budget: self.heap_budget.into_low_level(context, bump),
            heap_usage: self.heap_usage.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceMemoryBudgetPropertiesEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            heap_budget: crate::conv::FromLowLevel::from_low_level(context, value.heap_budget),
            heap_usage: crate::conv::FromLowLevel::from_low_level(context, value.heap_usage),
        }
    }
}
