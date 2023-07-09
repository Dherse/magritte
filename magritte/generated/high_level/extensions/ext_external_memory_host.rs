pub use crate::common::extensions::ext_external_memory_host::{
    EXT_EXTERNAL_MEMORY_HOST_EXTENSION_NAME, EXT_EXTERNAL_MEMORY_HOST_SPEC_VERSION,
};
use crate::{
    context::Context,
    vulkan1_0::{DeviceSize, StructureType},
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkMemoryHostPointerPropertiesEXT")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MemoryHostPointerPropertiesEXT {
    #[doc(alias = "memoryTypeBits")]
    pub memory_type_bits: u32,
}
impl MemoryHostPointerPropertiesEXT {
    ///Get a reference to the `memory_type_bits` field.
    pub fn memory_type_bits(&self) -> u32 {
        self.memory_type_bits
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for MemoryHostPointerPropertiesEXT {
    type LowLevel = crate::native::extensions::ext_external_memory_host::MemoryHostPointerPropertiesEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_external_memory_host::MemoryHostPointerPropertiesEXT {
            s_type: StructureType::MemoryHostPointerPropertiesExt,
            p_next: std::ptr::null_mut(),
            memory_type_bits: self.memory_type_bits.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for MemoryHostPointerPropertiesEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            memory_type_bits: crate::conv::FromLowLevel::from_low_level(context, value.memory_type_bits),
        }
    }
}
#[doc(alias = "VkPhysicalDeviceExternalMemoryHostPropertiesEXT")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceExternalMemoryHostPropertiesEXT {
    #[doc(alias = "minImportedHostPointerAlignment")]
    pub min_imported_host_pointer_alignment: DeviceSize,
}
impl PhysicalDeviceExternalMemoryHostPropertiesEXT {
    ///Get a reference to the `min_imported_host_pointer_alignment` field.
    pub fn min_imported_host_pointer_alignment(&self) -> &DeviceSize {
        &self.min_imported_host_pointer_alignment
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceExternalMemoryHostPropertiesEXT {
    type LowLevel = crate::native::extensions::ext_external_memory_host::PhysicalDeviceExternalMemoryHostPropertiesEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_external_memory_host::PhysicalDeviceExternalMemoryHostPropertiesEXT {
            s_type: StructureType::PhysicalDeviceExternalMemoryHostPropertiesExt,
            p_next: std::ptr::null_mut(),
            min_imported_host_pointer_alignment: self.min_imported_host_pointer_alignment.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceExternalMemoryHostPropertiesEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            min_imported_host_pointer_alignment: crate::conv::FromLowLevel::from_low_level(
                context,
                value.min_imported_host_pointer_alignment,
            ),
        }
    }
}
