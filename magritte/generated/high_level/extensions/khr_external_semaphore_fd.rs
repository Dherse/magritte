pub use crate::common::extensions::khr_external_semaphore_fd::{
    KHR_EXTERNAL_SEMAPHORE_FD_EXTENSION_NAME, KHR_EXTERNAL_SEMAPHORE_FD_SPEC_VERSION,
};
use crate::{
    context::Context,
    vulkan1_0::{Semaphore, StructureType},
    vulkan1_1::{ExternalSemaphoreHandleTypeFlagBits, SemaphoreImportFlags},
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkImportSemaphoreFdInfoKHR")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ImportSemaphoreFdInfoKHR {
    pub semaphore: Semaphore,
    pub flags: SemaphoreImportFlags,
    #[doc(alias = "handleType")]
    pub handle_type: ExternalSemaphoreHandleTypeFlagBits,
    pub fd: i32,
}
impl ImportSemaphoreFdInfoKHR {
    ///Get a reference to the `semaphore` field.
    pub fn semaphore(&self) -> &Semaphore {
        &self.semaphore
    }
    ///Get a reference to the `flags` field.
    pub fn flags(&self) -> SemaphoreImportFlags {
        self.flags
    }
    ///Get a reference to the `handle_type` field.
    pub fn handle_type(&self) -> ExternalSemaphoreHandleTypeFlagBits {
        self.handle_type
    }
    ///Get a reference to the `fd` field.
    pub fn fd(&self) -> i32 {
        self.fd
    }
    ///Get a mutable reference to the `semaphore` field.
    pub fn semaphore_mut(&mut self) -> &mut Semaphore {
        &mut self.semaphore
    }
    ///Get a mutable reference to the `flags` field.
    pub fn flags_mut(&mut self) -> &mut SemaphoreImportFlags {
        &mut self.flags
    }
    ///Get a mutable reference to the `handle_type` field.
    pub fn handle_type_mut(&mut self) -> &mut ExternalSemaphoreHandleTypeFlagBits {
        &mut self.handle_type
    }
    ///Get a mutable reference to the `fd` field.
    pub fn fd_mut(&mut self) -> &mut i32 {
        &mut self.fd
    }
    ///Sets the `semaphore` field.
    pub fn set_semaphore(&mut self, semaphore: Semaphore) -> &mut Self {
        self.semaphore = semaphore;
        self
    }
    ///Sets the `flags` field.
    pub fn set_flags(&mut self, flags: SemaphoreImportFlags) -> &mut Self {
        self.flags = flags;
        self
    }
    ///Sets the `handle_type` field.
    pub fn set_handle_type(&mut self, handle_type: ExternalSemaphoreHandleTypeFlagBits) -> &mut Self {
        self.handle_type = handle_type;
        self
    }
    ///Sets the `fd` field.
    pub fn set_fd(&mut self, fd: i32) -> &mut Self {
        self.fd = fd;
        self
    }
    ///Sets the `semaphore` field in a builder way.
    pub fn with_semaphore(mut self, semaphore: Semaphore) -> Self {
        self.semaphore = semaphore;
        self
    }
    ///Sets the `flags` field in a builder way.
    pub fn with_flags(mut self, flags: SemaphoreImportFlags) -> Self {
        self.flags = flags;
        self
    }
    ///Sets the `handle_type` field in a builder way.
    pub fn with_handle_type(mut self, handle_type: ExternalSemaphoreHandleTypeFlagBits) -> Self {
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
unsafe impl crate::conv::IntoLowLevel for ImportSemaphoreFdInfoKHR {
    type LowLevel = crate::native::extensions::khr_external_semaphore_fd::ImportSemaphoreFdInfoKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::khr_external_semaphore_fd::ImportSemaphoreFdInfoKHR {
            s_type: StructureType::ImportSemaphoreFdInfoKhr,
            p_next: std::ptr::null(),
            semaphore: self.semaphore.into_low_level(context, bump),
            flags: self.flags.into_low_level(context, bump),
            handle_type: self.handle_type.into_low_level(context, bump),
            fd: self.fd.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for ImportSemaphoreFdInfoKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            semaphore: crate::conv::FromLowLevel::from_low_level(context, value.semaphore),
            flags: crate::conv::FromLowLevel::from_low_level(context, value.flags),
            handle_type: crate::conv::FromLowLevel::from_low_level(context, value.handle_type),
            fd: crate::conv::FromLowLevel::from_low_level(context, value.fd),
        }
    }
}
#[doc(alias = "VkSemaphoreGetFdInfoKHR")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SemaphoreGetFdInfoKHR {
    pub semaphore: Semaphore,
    #[doc(alias = "handleType")]
    pub handle_type: ExternalSemaphoreHandleTypeFlagBits,
}
impl SemaphoreGetFdInfoKHR {
    ///Get a reference to the `semaphore` field.
    pub fn semaphore(&self) -> &Semaphore {
        &self.semaphore
    }
    ///Get a reference to the `handle_type` field.
    pub fn handle_type(&self) -> ExternalSemaphoreHandleTypeFlagBits {
        self.handle_type
    }
    ///Get a mutable reference to the `semaphore` field.
    pub fn semaphore_mut(&mut self) -> &mut Semaphore {
        &mut self.semaphore
    }
    ///Get a mutable reference to the `handle_type` field.
    pub fn handle_type_mut(&mut self) -> &mut ExternalSemaphoreHandleTypeFlagBits {
        &mut self.handle_type
    }
    ///Sets the `semaphore` field.
    pub fn set_semaphore(&mut self, semaphore: Semaphore) -> &mut Self {
        self.semaphore = semaphore;
        self
    }
    ///Sets the `handle_type` field.
    pub fn set_handle_type(&mut self, handle_type: ExternalSemaphoreHandleTypeFlagBits) -> &mut Self {
        self.handle_type = handle_type;
        self
    }
    ///Sets the `semaphore` field in a builder way.
    pub fn with_semaphore(mut self, semaphore: Semaphore) -> Self {
        self.semaphore = semaphore;
        self
    }
    ///Sets the `handle_type` field in a builder way.
    pub fn with_handle_type(mut self, handle_type: ExternalSemaphoreHandleTypeFlagBits) -> Self {
        self.handle_type = handle_type;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for SemaphoreGetFdInfoKHR {
    type LowLevel = crate::native::extensions::khr_external_semaphore_fd::SemaphoreGetFdInfoKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::khr_external_semaphore_fd::SemaphoreGetFdInfoKHR {
            s_type: StructureType::SemaphoreGetFdInfoKhr,
            p_next: std::ptr::null(),
            semaphore: self.semaphore.into_low_level(context, bump),
            handle_type: self.handle_type.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for SemaphoreGetFdInfoKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            semaphore: crate::conv::FromLowLevel::from_low_level(context, value.semaphore),
            handle_type: crate::conv::FromLowLevel::from_low_level(context, value.handle_type),
        }
    }
}
