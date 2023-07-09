pub use crate::common::extensions::fuchsia_external_memory::{
    FUCHSIA_EXTERNAL_MEMORY_EXTENSION_NAME, FUCHSIA_EXTERNAL_MEMORY_SPEC_VERSION,
};
use crate::{
    context::Context,
    vulkan1_0::{DeviceMemory, StructureType},
    vulkan1_1::ExternalMemoryHandleTypeFlagBits,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkMemoryZirconHandlePropertiesFUCHSIA")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MemoryZirconHandlePropertiesFUCHSIA {
    #[doc(alias = "memoryTypeBits")]
    pub memory_type_bits: u32,
}
impl MemoryZirconHandlePropertiesFUCHSIA {
    ///Get a reference to the `memory_type_bits` field.
    pub fn memory_type_bits(&self) -> u32 {
        self.memory_type_bits
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for MemoryZirconHandlePropertiesFUCHSIA {
    type LowLevel = crate::native::extensions::fuchsia_external_memory::MemoryZirconHandlePropertiesFUCHSIA;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::fuchsia_external_memory::MemoryZirconHandlePropertiesFUCHSIA {
            s_type: StructureType::MemoryZirconHandlePropertiesFuchsia,
            p_next: std::ptr::null_mut(),
            memory_type_bits: self.memory_type_bits.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for MemoryZirconHandlePropertiesFUCHSIA {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            memory_type_bits: crate::conv::FromLowLevel::from_low_level(context, value.memory_type_bits),
        }
    }
}
#[doc(alias = "VkMemoryGetZirconHandleInfoFUCHSIA")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MemoryGetZirconHandleInfoFUCHSIA {
    pub memory: DeviceMemory,
    #[doc(alias = "handleType")]
    pub handle_type: ExternalMemoryHandleTypeFlagBits,
}
impl MemoryGetZirconHandleInfoFUCHSIA {
    ///Get a reference to the `memory` field.
    pub fn memory(&self) -> &DeviceMemory {
        &self.memory
    }
    ///Get a reference to the `handle_type` field.
    pub fn handle_type(&self) -> ExternalMemoryHandleTypeFlagBits {
        self.handle_type
    }
    ///Get a mutable reference to the `memory` field.
    pub fn memory_mut(&mut self) -> &mut DeviceMemory {
        &mut self.memory
    }
    ///Get a mutable reference to the `handle_type` field.
    pub fn handle_type_mut(&mut self) -> &mut ExternalMemoryHandleTypeFlagBits {
        &mut self.handle_type
    }
    ///Sets the `memory` field.
    pub fn set_memory(&mut self, memory: DeviceMemory) -> &mut Self {
        self.memory = memory;
        self
    }
    ///Sets the `handle_type` field.
    pub fn set_handle_type(&mut self, handle_type: ExternalMemoryHandleTypeFlagBits) -> &mut Self {
        self.handle_type = handle_type;
        self
    }
    ///Sets the `memory` field in a builder way.
    pub fn with_memory(mut self, memory: DeviceMemory) -> Self {
        self.memory = memory;
        self
    }
    ///Sets the `handle_type` field in a builder way.
    pub fn with_handle_type(mut self, handle_type: ExternalMemoryHandleTypeFlagBits) -> Self {
        self.handle_type = handle_type;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for MemoryGetZirconHandleInfoFUCHSIA {
    type LowLevel = crate::native::extensions::fuchsia_external_memory::MemoryGetZirconHandleInfoFUCHSIA;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::fuchsia_external_memory::MemoryGetZirconHandleInfoFUCHSIA {
            s_type: StructureType::MemoryGetZirconHandleInfoFuchsia,
            p_next: std::ptr::null(),
            memory: self.memory.into_low_level(context, bump),
            handle_type: self.handle_type.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for MemoryGetZirconHandleInfoFUCHSIA {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            memory: crate::conv::FromLowLevel::from_low_level(context, value.memory),
            handle_type: crate::conv::FromLowLevel::from_low_level(context, value.handle_type),
        }
    }
}
