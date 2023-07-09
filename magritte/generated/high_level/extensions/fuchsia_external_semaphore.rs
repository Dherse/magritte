pub use crate::common::extensions::fuchsia_external_semaphore::{
    FUCHSIA_EXTERNAL_SEMAPHORE_EXTENSION_NAME, FUCHSIA_EXTERNAL_SEMAPHORE_SPEC_VERSION,
};
use crate::{
    context::Context,
    vulkan1_0::{Semaphore, StructureType},
    vulkan1_1::ExternalSemaphoreHandleTypeFlagBits,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkSemaphoreGetZirconHandleInfoFUCHSIA")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SemaphoreGetZirconHandleInfoFUCHSIA {
    pub semaphore: Semaphore,
    #[doc(alias = "handleType")]
    pub handle_type: ExternalSemaphoreHandleTypeFlagBits,
}
impl SemaphoreGetZirconHandleInfoFUCHSIA {
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
unsafe impl crate::conv::IntoLowLevel for SemaphoreGetZirconHandleInfoFUCHSIA {
    type LowLevel = crate::native::extensions::fuchsia_external_semaphore::SemaphoreGetZirconHandleInfoFUCHSIA;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::fuchsia_external_semaphore::SemaphoreGetZirconHandleInfoFUCHSIA {
            s_type: StructureType::SemaphoreGetZirconHandleInfoFuchsia,
            p_next: std::ptr::null(),
            semaphore: self.semaphore.into_low_level(context, bump),
            handle_type: self.handle_type.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for SemaphoreGetZirconHandleInfoFUCHSIA {
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
