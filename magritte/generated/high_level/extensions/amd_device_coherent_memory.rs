pub use crate::common::extensions::amd_device_coherent_memory::{
    AMD_DEVICE_COHERENT_MEMORY_EXTENSION_NAME, AMD_DEVICE_COHERENT_MEMORY_SPEC_VERSION,
};
use crate::{context::Context, vulkan1_0::StructureType};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkPhysicalDeviceCoherentMemoryFeaturesAMD")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceCoherentMemoryFeaturesAMD {
    #[doc(alias = "deviceCoherentMemory")]
    pub device_coherent_memory: bool,
}
impl PhysicalDeviceCoherentMemoryFeaturesAMD {
    ///Get a reference to the `device_coherent_memory` field.
    pub fn device_coherent_memory(&self) -> &bool {
        &self.device_coherent_memory
    }
    ///Get a mutable reference to the `device_coherent_memory` field.
    pub fn device_coherent_memory_mut(&mut self) -> &mut bool {
        &mut self.device_coherent_memory
    }
    ///Sets the `device_coherent_memory` field.
    pub fn set_device_coherent_memory(&mut self, device_coherent_memory: bool) -> &mut Self {
        self.device_coherent_memory = device_coherent_memory;
        self
    }
    ///Sets the `device_coherent_memory` field in a builder way.
    pub fn with_device_coherent_memory(mut self, device_coherent_memory: bool) -> Self {
        self.device_coherent_memory = device_coherent_memory;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceCoherentMemoryFeaturesAMD {
    type LowLevel = crate::native::extensions::amd_device_coherent_memory::PhysicalDeviceCoherentMemoryFeaturesAMD;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::amd_device_coherent_memory::PhysicalDeviceCoherentMemoryFeaturesAMD {
            s_type: StructureType::PhysicalDeviceCoherentMemoryFeaturesAmd,
            p_next: std::ptr::null_mut(),
            device_coherent_memory: self.device_coherent_memory.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceCoherentMemoryFeaturesAMD {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            device_coherent_memory: crate::conv::FromLowLevel::from_low_level(context, value.device_coherent_memory),
        }
    }
}
