pub use crate::common::extensions::khr_external_memory_fd::{
    KHR_EXTERNAL_MEMORY_FD_EXTENSION_NAME, KHR_EXTERNAL_MEMORY_FD_SPEC_VERSION,
};
use crate::{
    context::Context,
    vulkan1_0::{DeviceMemory, StructureType},
    vulkan1_1::ExternalMemoryHandleTypeFlagBits,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkImportMemoryFdInfoKHR")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ImportMemoryFdInfoKHR {
    #[doc(alias = "handleType")]
    pub handle_type: ExternalMemoryHandleTypeFlagBits,
    pub fd: i32,
}
impl ImportMemoryFdInfoKHR {
    ///Get a reference to the `handle_type` field.
    pub fn handle_type(&self) -> ExternalMemoryHandleTypeFlagBits {
        self.handle_type
    }
    ///Get a reference to the `fd` field.
    pub fn fd(&self) -> i32 {
        self.fd
    }
    ///Get a mutable reference to the `handle_type` field.
    pub fn handle_type_mut(&mut self) -> &mut ExternalMemoryHandleTypeFlagBits {
        &mut self.handle_type
    }
    ///Get a mutable reference to the `fd` field.
    pub fn fd_mut(&mut self) -> &mut i32 {
        &mut self.fd
    }
    ///Sets the `handle_type` field.
    pub fn set_handle_type(&mut self, handle_type: ExternalMemoryHandleTypeFlagBits) -> &mut Self {
        self.handle_type = handle_type;
        self
    }
    ///Sets the `fd` field.
    pub fn set_fd(&mut self, fd: i32) -> &mut Self {
        self.fd = fd;
        self
    }
    ///Sets the `handle_type` field in a builder way.
    pub fn with_handle_type(mut self, handle_type: ExternalMemoryHandleTypeFlagBits) -> Self {
        self.handle_type = handle_type;
        self
    }
    ///Sets the `fd` field in a builder way.
    pub fn with_fd(mut self, fd: i32) -> Self {
        self.fd = fd;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for ImportMemoryFdInfoKHR {
    type LowLevel = crate::native::extensions::khr_external_memory_fd::ImportMemoryFdInfoKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::khr_external_memory_fd::ImportMemoryFdInfoKHR {
            s_type: StructureType::ImportMemoryFdInfoKhr,
            p_next: std::ptr::null(),
            handle_type: self.handle_type.into_low_level(context, bump),
            fd: self.fd.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for ImportMemoryFdInfoKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            handle_type: crate::conv::FromLowLevel::from_low_level(context, value.handle_type),
            fd: crate::conv::FromLowLevel::from_low_level(context, value.fd),
        }
    }
}
#[doc(alias = "VkMemoryFdPropertiesKHR")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MemoryFdPropertiesKHR {
    #[doc(alias = "memoryTypeBits")]
    pub memory_type_bits: u32,
}
impl MemoryFdPropertiesKHR {
    ///Get a reference to the `memory_type_bits` field.
    pub fn memory_type_bits(&self) -> u32 {
        self.memory_type_bits
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for MemoryFdPropertiesKHR {
    type LowLevel = crate::native::extensions::khr_external_memory_fd::MemoryFdPropertiesKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::khr_external_memory_fd::MemoryFdPropertiesKHR {
            s_type: StructureType::MemoryFdPropertiesKhr,
            p_next: std::ptr::null_mut(),
            memory_type_bits: self.memory_type_bits.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for MemoryFdPropertiesKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            memory_type_bits: crate::conv::FromLowLevel::from_low_level(context, value.memory_type_bits),
        }
    }
}
#[doc(alias = "VkMemoryGetFdInfoKHR")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MemoryGetFdInfoKHR {
    pub memory: DeviceMemory,
    #[doc(alias = "handleType")]
    pub handle_type: ExternalMemoryHandleTypeFlagBits,
}
impl MemoryGetFdInfoKHR {
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
unsafe impl crate::conv::IntoLowLevel for MemoryGetFdInfoKHR {
    type LowLevel = crate::native::extensions::khr_external_memory_fd::MemoryGetFdInfoKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::khr_external_memory_fd::MemoryGetFdInfoKHR {
            s_type: StructureType::MemoryGetFdInfoKhr,
            p_next: std::ptr::null(),
            memory: self.memory.into_low_level(context, bump),
            handle_type: self.handle_type.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for MemoryGetFdInfoKHR {
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
