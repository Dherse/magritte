pub use crate::common::extensions::ext_pageable_device_local_memory::{
    EXT_PAGEABLE_DEVICE_LOCAL_MEMORY_EXTENSION_NAME, EXT_PAGEABLE_DEVICE_LOCAL_MEMORY_SPEC_VERSION,
};
use crate::{context::Context, vulkan1_0::StructureType};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkPhysicalDevicePageableDeviceLocalMemoryFeaturesEXT")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT {
    #[doc(alias = "pageableDeviceLocalMemory")]
    pub pageable_device_local_memory: bool,
}
impl PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT {
    ///Get a reference to the `pageable_device_local_memory` field.
    pub fn pageable_device_local_memory(&self) -> &bool {
        &self.pageable_device_local_memory
    }
    ///Get a mutable reference to the `pageable_device_local_memory` field.
    pub fn pageable_device_local_memory_mut(&mut self) -> &mut bool {
        &mut self.pageable_device_local_memory
    }
    ///Sets the `pageable_device_local_memory` field.
    pub fn set_pageable_device_local_memory(&mut self, pageable_device_local_memory: bool) -> &mut Self {
        self.pageable_device_local_memory = pageable_device_local_memory;
        self
    }
    ///Sets the `pageable_device_local_memory` field in a builder way.
    pub fn with_pageable_device_local_memory(mut self, pageable_device_local_memory: bool) -> Self {
        self.pageable_device_local_memory = pageable_device_local_memory;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT {
    type LowLevel =
        crate::native::extensions::ext_pageable_device_local_memory::PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate :: native :: extensions :: ext_pageable_device_local_memory :: PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT { s_type : StructureType :: PhysicalDevicePageableDeviceLocalMemoryFeaturesExt , p_next : std :: ptr :: null_mut () , pageable_device_local_memory : self . pageable_device_local_memory . into_low_level (context , bump) }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            pageable_device_local_memory: crate::conv::FromLowLevel::from_low_level(
                context,
                value.pageable_device_local_memory,
            ),
        }
    }
}
