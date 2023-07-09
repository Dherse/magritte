pub use crate::common::extensions::khr_external_semaphore_win32::{
    KHR_EXTERNAL_SEMAPHORE_WIN32_EXTENSION_NAME, KHR_EXTERNAL_SEMAPHORE_WIN32_SPEC_VERSION,
};
use crate::{
    context::Context,
    vulkan1_0::{Semaphore, StructureType},
    vulkan1_1::ExternalSemaphoreHandleTypeFlagBits,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use std::sync::Arc;
#[doc(alias = "VkD3D12FenceSubmitInfoKHR")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct D3d12FenceSubmitInfoKHR {
    #[doc(alias = "pWaitSemaphoreValues")]
    pub wait_semaphore_values: SmallVec<[u64; 8]>,
    #[doc(alias = "pSignalSemaphoreValues")]
    pub signal_semaphore_values: SmallVec<[u64; 8]>,
}
impl D3d12FenceSubmitInfoKHR {
    ///Get a reference to the `wait_semaphore_values` field.
    pub fn wait_semaphore_values(&self) -> &SmallVec<[u64; 8]> {
        &self.wait_semaphore_values
    }
    ///Get a reference to the `signal_semaphore_values` field.
    pub fn signal_semaphore_values(&self) -> &SmallVec<[u64; 8]> {
        &self.signal_semaphore_values
    }
    ///Get a mutable reference to the `wait_semaphore_values` field.
    pub fn wait_semaphore_values_mut(&mut self) -> &mut SmallVec<[u64; 8]> {
        &mut self.wait_semaphore_values
    }
    ///Get a mutable reference to the `signal_semaphore_values` field.
    pub fn signal_semaphore_values_mut(&mut self) -> &mut SmallVec<[u64; 8]> {
        &mut self.signal_semaphore_values
    }
    ///Sets the `wait_semaphore_values` field.
    pub fn set_wait_semaphore_values(&mut self, wait_semaphore_values: SmallVec<[u64; 8]>) -> &mut Self {
        self.wait_semaphore_values = wait_semaphore_values;
        self
    }
    ///Sets the `signal_semaphore_values` field.
    pub fn set_signal_semaphore_values(&mut self, signal_semaphore_values: SmallVec<[u64; 8]>) -> &mut Self {
        self.signal_semaphore_values = signal_semaphore_values;
        self
    }
    ///Sets the `wait_semaphore_values` field in a builder way.
    pub fn with_wait_semaphore_values(mut self, wait_semaphore_values: SmallVec<[u64; 8]>) -> Self {
        self.wait_semaphore_values = wait_semaphore_values;
        self
    }
    ///Sets the `signal_semaphore_values` field in a builder way.
    pub fn with_signal_semaphore_values(mut self, signal_semaphore_values: SmallVec<[u64; 8]>) -> Self {
        self.signal_semaphore_values = signal_semaphore_values;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for D3d12FenceSubmitInfoKHR {
    type LowLevel = crate::native::extensions::khr_external_semaphore_win32::D3d12FenceSubmitInfoKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let len_wait_semaphore_values = self.wait_semaphore_values.len() as u32;
        let wait_semaphore_values = bump
            .alloc_slice_fill_iter(
                self.wait_semaphore_values
                    .iter()
                    .map(|x| x.into_low_level(context, bump)),
            )
            .as_ptr()
            .cast();
        let len_signal_semaphore_values = self.signal_semaphore_values.len() as u32;
        let signal_semaphore_values = bump
            .alloc_slice_fill_iter(
                self.signal_semaphore_values
                    .iter()
                    .map(|x| x.into_low_level(context, bump)),
            )
            .as_ptr()
            .cast();
        crate::native::extensions::khr_external_semaphore_win32::D3d12FenceSubmitInfoKHR {
            s_type: StructureType::D3d12FenceSubmitInfoKhr,
            p_next: std::ptr::null(),
            wait_semaphore_values_count: len_wait_semaphore_values,
            wait_semaphore_values: wait_semaphore_values,
            signal_semaphore_values_count: len_signal_semaphore_values,
            signal_semaphore_values: signal_semaphore_values,
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for D3d12FenceSubmitInfoKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let wait_semaphore_values_len = value.wait_semaphore_values_count;
        let mut wait_semaphore_values = SmallVec::with_capacity(wait_semaphore_values_len as usize);
        for i in 0..wait_semaphore_values_len {
            wait_semaphore_values.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.wait_semaphore_values.add(i as usize).read(),
            ));
        }
        let signal_semaphore_values_len = value.signal_semaphore_values_count;
        let mut signal_semaphore_values = SmallVec::with_capacity(signal_semaphore_values_len as usize);
        for i in 0..signal_semaphore_values_len {
            signal_semaphore_values.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.signal_semaphore_values.add(i as usize).read(),
            ));
        }
        Self {
            wait_semaphore_values: wait_semaphore_values,
            signal_semaphore_values: signal_semaphore_values,
        }
    }
}
#[doc(alias = "VkSemaphoreGetWin32HandleInfoKHR")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SemaphoreGetWin32HandleInfoKHR {
    pub semaphore: Semaphore,
    #[doc(alias = "handleType")]
    pub handle_type: ExternalSemaphoreHandleTypeFlagBits,
}
impl SemaphoreGetWin32HandleInfoKHR {
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
unsafe impl crate::conv::IntoLowLevel for SemaphoreGetWin32HandleInfoKHR {
    type LowLevel = crate::native::extensions::khr_external_semaphore_win32::SemaphoreGetWin32HandleInfoKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::khr_external_semaphore_win32::SemaphoreGetWin32HandleInfoKHR {
            s_type: StructureType::SemaphoreGetWin32HandleInfoKhr,
            p_next: std::ptr::null(),
            semaphore: self.semaphore.into_low_level(context, bump),
            handle_type: self.handle_type.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for SemaphoreGetWin32HandleInfoKHR {
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
