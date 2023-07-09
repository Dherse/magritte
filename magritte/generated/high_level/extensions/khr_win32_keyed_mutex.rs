pub use crate::common::extensions::khr_win32_keyed_mutex::{
    KHR_WIN32_KEYED_MUTEX_EXTENSION_NAME, KHR_WIN32_KEYED_MUTEX_SPEC_VERSION,
};
use crate::{
    context::Context,
    vulkan1_0::{DeviceMemory, StructureType},
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use std::sync::Arc;
#[doc(alias = "VkWin32KeyedMutexAcquireReleaseInfoKHR")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Win32KeyedMutexAcquireReleaseInfoKHR {
    #[doc(alias = "pAcquireSyncs")]
    pub acquire_syncs: SmallVec<[DeviceMemory; 8]>,
    #[doc(alias = "pAcquireKeys")]
    pub acquire_keys: SmallVec<[u64; 8]>,
    #[doc(alias = "pAcquireTimeouts")]
    pub acquire_timeouts: SmallVec<[u32; 8]>,
    #[doc(alias = "pReleaseSyncs")]
    pub release_syncs: SmallVec<[DeviceMemory; 8]>,
    #[doc(alias = "pReleaseKeys")]
    pub release_keys: SmallVec<[u64; 8]>,
}
impl Win32KeyedMutexAcquireReleaseInfoKHR {
    ///Get a reference to the `acquire_syncs` field.
    pub fn acquire_syncs(&self) -> &SmallVec<[DeviceMemory; 8]> {
        &self.acquire_syncs
    }
    ///Get a reference to the `acquire_keys` field.
    pub fn acquire_keys(&self) -> &SmallVec<[u64; 8]> {
        &self.acquire_keys
    }
    ///Get a reference to the `acquire_timeouts` field.
    pub fn acquire_timeouts(&self) -> &SmallVec<[u32; 8]> {
        &self.acquire_timeouts
    }
    ///Get a reference to the `release_syncs` field.
    pub fn release_syncs(&self) -> &SmallVec<[DeviceMemory; 8]> {
        &self.release_syncs
    }
    ///Get a reference to the `release_keys` field.
    pub fn release_keys(&self) -> &SmallVec<[u64; 8]> {
        &self.release_keys
    }
    ///Get a mutable reference to the `acquire_syncs` field.
    pub fn acquire_syncs_mut(&mut self) -> &mut SmallVec<[DeviceMemory; 8]> {
        &mut self.acquire_syncs
    }
    ///Get a mutable reference to the `acquire_keys` field.
    pub fn acquire_keys_mut(&mut self) -> &mut SmallVec<[u64; 8]> {
        &mut self.acquire_keys
    }
    ///Get a mutable reference to the `acquire_timeouts` field.
    pub fn acquire_timeouts_mut(&mut self) -> &mut SmallVec<[u32; 8]> {
        &mut self.acquire_timeouts
    }
    ///Get a mutable reference to the `release_syncs` field.
    pub fn release_syncs_mut(&mut self) -> &mut SmallVec<[DeviceMemory; 8]> {
        &mut self.release_syncs
    }
    ///Get a mutable reference to the `release_keys` field.
    pub fn release_keys_mut(&mut self) -> &mut SmallVec<[u64; 8]> {
        &mut self.release_keys
    }
    ///Sets the `acquire_syncs` field.
    pub fn set_acquire_syncs(&mut self, acquire_syncs: SmallVec<[DeviceMemory; 8]>) -> &mut Self {
        self.acquire_syncs = acquire_syncs;
        self
    }
    ///Sets the `acquire_keys` field.
    pub fn set_acquire_keys(&mut self, acquire_keys: SmallVec<[u64; 8]>) -> &mut Self {
        self.acquire_keys = acquire_keys;
        self
    }
    ///Sets the `acquire_timeouts` field.
    pub fn set_acquire_timeouts(&mut self, acquire_timeouts: SmallVec<[u32; 8]>) -> &mut Self {
        self.acquire_timeouts = acquire_timeouts;
        self
    }
    ///Sets the `release_syncs` field.
    pub fn set_release_syncs(&mut self, release_syncs: SmallVec<[DeviceMemory; 8]>) -> &mut Self {
        self.release_syncs = release_syncs;
        self
    }
    ///Sets the `release_keys` field.
    pub fn set_release_keys(&mut self, release_keys: SmallVec<[u64; 8]>) -> &mut Self {
        self.release_keys = release_keys;
        self
    }
    ///Sets the `acquire_syncs` field in a builder way.
    pub fn with_acquire_syncs(mut self, acquire_syncs: SmallVec<[DeviceMemory; 8]>) -> Self {
        self.acquire_syncs = acquire_syncs;
        self
    }
    ///Sets the `acquire_keys` field in a builder way.
    pub fn with_acquire_keys(mut self, acquire_keys: SmallVec<[u64; 8]>) -> Self {
        self.acquire_keys = acquire_keys;
        self
    }
    ///Sets the `acquire_timeouts` field in a builder way.
    pub fn with_acquire_timeouts(mut self, acquire_timeouts: SmallVec<[u32; 8]>) -> Self {
        self.acquire_timeouts = acquire_timeouts;
        self
    }
    ///Sets the `release_syncs` field in a builder way.
    pub fn with_release_syncs(mut self, release_syncs: SmallVec<[DeviceMemory; 8]>) -> Self {
        self.release_syncs = release_syncs;
        self
    }
    ///Sets the `release_keys` field in a builder way.
    pub fn with_release_keys(mut self, release_keys: SmallVec<[u64; 8]>) -> Self {
        self.release_keys = release_keys;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for Win32KeyedMutexAcquireReleaseInfoKHR {
    type LowLevel = crate::native::extensions::khr_win32_keyed_mutex::Win32KeyedMutexAcquireReleaseInfoKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let len_acquire_syncs = self.acquire_syncs.len() as u32;
        let acquire_syncs = bump
            .alloc_slice_fill_iter(self.acquire_syncs.iter().map(|x| x.into_low_level(context, bump)))
            .as_ptr()
            .cast();
        let acquire_keys = bump
            .alloc_slice_fill_iter(self.acquire_keys.iter().map(|x| x.into_low_level(context, bump)))
            .as_ptr()
            .cast();
        let acquire_timeouts = bump
            .alloc_slice_fill_iter(self.acquire_timeouts.iter().map(|x| x.into_low_level(context, bump)))
            .as_ptr()
            .cast();
        let len_release_syncs = self.release_syncs.len() as u32;
        let release_syncs = bump
            .alloc_slice_fill_iter(self.release_syncs.iter().map(|x| x.into_low_level(context, bump)))
            .as_ptr()
            .cast();
        let release_keys = bump
            .alloc_slice_fill_iter(self.release_keys.iter().map(|x| x.into_low_level(context, bump)))
            .as_ptr()
            .cast();
        crate::native::extensions::khr_win32_keyed_mutex::Win32KeyedMutexAcquireReleaseInfoKHR {
            s_type: StructureType::Win32KeyedMutexAcquireReleaseInfoKhr,
            p_next: std::ptr::null(),
            acquire_count: len_acquire_syncs,
            acquire_syncs: acquire_syncs,
            acquire_keys: acquire_keys,
            acquire_timeouts: acquire_timeouts,
            release_count: len_release_syncs,
            release_syncs: release_syncs,
            release_keys: release_keys,
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for Win32KeyedMutexAcquireReleaseInfoKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let acquire_syncs_len = value.acquire_count;
        let mut acquire_syncs = SmallVec::with_capacity(acquire_syncs_len as usize);
        for i in 0..acquire_syncs_len {
            acquire_syncs.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.acquire_syncs.add(i as usize).read(),
            ));
        }
        let acquire_keys_len = value.acquire_count;
        let mut acquire_keys = SmallVec::with_capacity(acquire_keys_len as usize);
        for i in 0..acquire_keys_len {
            acquire_keys.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.acquire_keys.add(i as usize).read(),
            ));
        }
        let acquire_timeouts_len = value.acquire_count;
        let mut acquire_timeouts = SmallVec::with_capacity(acquire_timeouts_len as usize);
        for i in 0..acquire_timeouts_len {
            acquire_timeouts.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.acquire_timeouts.add(i as usize).read(),
            ));
        }
        let release_syncs_len = value.release_count;
        let mut release_syncs = SmallVec::with_capacity(release_syncs_len as usize);
        for i in 0..release_syncs_len {
            release_syncs.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.release_syncs.add(i as usize).read(),
            ));
        }
        let release_keys_len = value.release_count;
        let mut release_keys = SmallVec::with_capacity(release_keys_len as usize);
        for i in 0..release_keys_len {
            release_keys.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.release_keys.add(i as usize).read(),
            ));
        }
        Self {
            acquire_syncs: acquire_syncs,
            acquire_keys: acquire_keys,
            acquire_timeouts: acquire_timeouts,
            release_syncs: release_syncs,
            release_keys: release_keys,
        }
    }
}
