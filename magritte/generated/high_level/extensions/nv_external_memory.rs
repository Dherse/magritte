pub use crate::common::extensions::nv_external_memory::{
    NV_EXTERNAL_MEMORY_EXTENSION_NAME, NV_EXTERNAL_MEMORY_SPEC_VERSION,
};
use crate::{
    context::Context, extensions::nv_external_memory_capabilities::ExternalMemoryHandleTypeFlagsNV,
    vulkan1_0::StructureType,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkExternalMemoryImageCreateInfoNV")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ExternalMemoryImageCreateInfoNV {
    #[doc(alias = "handleTypes")]
    pub handle_types: ExternalMemoryHandleTypeFlagsNV,
}
impl ExternalMemoryImageCreateInfoNV {
    ///Get a reference to the `handle_types` field.
    pub fn handle_types(&self) -> ExternalMemoryHandleTypeFlagsNV {
        self.handle_types
    }
    ///Get a mutable reference to the `handle_types` field.
    pub fn handle_types_mut(&mut self) -> &mut ExternalMemoryHandleTypeFlagsNV {
        &mut self.handle_types
    }
    ///Sets the `handle_types` field.
    pub fn set_handle_types(&mut self, handle_types: ExternalMemoryHandleTypeFlagsNV) -> &mut Self {
        self.handle_types = handle_types;
        self
    }
    ///Sets the `handle_types` field in a builder way.
    pub fn with_handle_types(mut self, handle_types: ExternalMemoryHandleTypeFlagsNV) -> Self {
        self.handle_types = handle_types;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for ExternalMemoryImageCreateInfoNV {
    type LowLevel = crate::native::extensions::nv_external_memory::ExternalMemoryImageCreateInfoNV;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::nv_external_memory::ExternalMemoryImageCreateInfoNV {
            s_type: StructureType::ExternalMemoryImageCreateInfoNv,
            p_next: std::ptr::null(),
            handle_types: self.handle_types.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for ExternalMemoryImageCreateInfoNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            handle_types: crate::conv::FromLowLevel::from_low_level(context, value.handle_types),
        }
    }
}
#[doc(alias = "VkExportMemoryAllocateInfoNV")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ExportMemoryAllocateInfoNV {
    #[doc(alias = "handleTypes")]
    pub handle_types: ExternalMemoryHandleTypeFlagsNV,
}
impl ExportMemoryAllocateInfoNV {
    ///Get a reference to the `handle_types` field.
    pub fn handle_types(&self) -> ExternalMemoryHandleTypeFlagsNV {
        self.handle_types
    }
    ///Get a mutable reference to the `handle_types` field.
    pub fn handle_types_mut(&mut self) -> &mut ExternalMemoryHandleTypeFlagsNV {
        &mut self.handle_types
    }
    ///Sets the `handle_types` field.
    pub fn set_handle_types(&mut self, handle_types: ExternalMemoryHandleTypeFlagsNV) -> &mut Self {
        self.handle_types = handle_types;
        self
    }
    ///Sets the `handle_types` field in a builder way.
    pub fn with_handle_types(mut self, handle_types: ExternalMemoryHandleTypeFlagsNV) -> Self {
        self.handle_types = handle_types;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for ExportMemoryAllocateInfoNV {
    type LowLevel = crate::native::extensions::nv_external_memory::ExportMemoryAllocateInfoNV;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::nv_external_memory::ExportMemoryAllocateInfoNV {
            s_type: StructureType::ExportMemoryAllocateInfoNv,
            p_next: std::ptr::null(),
            handle_types: self.handle_types.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for ExportMemoryAllocateInfoNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            handle_types: crate::conv::FromLowLevel::from_low_level(context, value.handle_types),
        }
    }
}
