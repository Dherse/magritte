pub use crate::common::extensions::nv_win32_keyed_mutex::{
    NV_WIN32_KEYED_MUTEX_EXTENSION_NAME, NV_WIN32_KEYED_MUTEX_SPEC_VERSION,
};
use crate::{
    context::Context,
    vulkan1_0::{DeviceMemory, StructureType},
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use std::sync::Arc;
#[doc(alias = "VkWin32KeyedMutexAcquireReleaseInfoNV")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Win32KeyedMutexAcquireReleaseInfoNV {
    #[doc(alias = "pAcquireSyncs")]
    pub acquire_syncs: SmallVec<[DeviceMemory; 8]>,
    #[doc(alias = "pAcquireKeys")]
    pub acquire_keys: SmallVec<[u64; 8]>,
    #[doc(alias = "pAcquireTimeoutMilliseconds")]
    pub acquire_timeout_milliseconds: SmallVec<[u32; 8]>,
    #[doc(alias = "pReleaseSyncs")]
    pub release_syncs: SmallVec<[DeviceMemory; 8]>,
    #[doc(alias = "pReleaseKeys")]
    pub release_keys: SmallVec<[u64; 8]>,
}
impl Win32KeyedMutexAcquireReleaseInfoNV {
    ///Get a reference to the `acquire_syncs` field.
    pub fn acquire_syncs(&self) -> &SmallVec<[DeviceMemory; 8]> {
        &self.acquire_syncs
    }
    ///Get a reference to the `acquire_keys` field.
    pub fn acquire_keys(&self) -> &SmallVec<[u64; 8]> {
        &self.acquire_keys
    }
    ///Get a reference to the `acquire_timeout_milliseconds` field.
    pub fn acquire_timeout_milliseconds(&self) -> &SmallVec<[u32; 8]> {
        &self.acquire_timeout_milliseconds
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
    ///Get a mutable reference to the `acquire_timeout_milliseconds` field.
    pub fn acquire_timeout_milliseconds_mut(&mut self) -> &mut SmallVec<[u32; 8]> {
        &mut self.acquire_timeout_milliseconds
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
    ///Sets the `acquire_timeout_milliseconds` field.
    pub fn set_acquire_timeout_milliseconds(&mut self, acquire_timeout_milliseconds: SmallVec<[u32; 8]>) -> &mut Self {
        self.acquire_timeout_milliseconds = acquire_timeout_milliseconds;
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
    ///Sets the `acquire_timeout_milliseconds` field in a builder way.
    pub fn with_acquire_timeout_milliseconds(mut self, acquire_timeout_milliseconds: SmallVec<[u32; 8]>) -> Self {
        self.acquire_timeout_milliseconds = acquire_timeout_milliseconds;
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
unsafe impl crate::conv::IntoLowLevel for Win32KeyedMutexAcquireReleaseInfoNV {
    type LowLevel = crate::native::extensions::nv_win32_keyed_mutex::Win32KeyedMutexAcquireReleaseInfoNV;
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
        let acquire_timeout_milliseconds = bump
            .alloc_slice_fill_iter(
                self.acquire_timeout_milliseconds
                    .iter()
                    .map(|x| x.into_low_level(context, bump)),
            )
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
        crate::native::extensions::nv_win32_keyed_mutex::Win32KeyedMutexAcquireReleaseInfoNV {
            s_type: StructureType::Win32KeyedMutexAcquireReleaseInfoNv,
            p_next: std::ptr::null(),
            acquire_count: len_acquire_syncs,
            acquire_syncs: acquire_syncs,
            acquire_keys: acquire_keys,
            acquire_timeout_milliseconds: acquire_timeout_milliseconds,
            release_count: len_release_syncs,
            release_syncs: release_syncs,
            release_keys: release_keys,
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for Win32KeyedMutexAcquireReleaseInfoNV {
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
        let acquire_timeout_milliseconds_len = value.acquire_count;
        let mut acquire_timeout_milliseconds = SmallVec::with_capacity(acquire_timeout_milliseconds_len as usize);
        for i in 0..acquire_timeout_milliseconds_len {
            acquire_timeout_milliseconds.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.acquire_timeout_milliseconds.add(i as usize).read(),
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
            acquire_timeout_milliseconds: acquire_timeout_milliseconds,
            release_syncs: release_syncs,
            release_keys: release_keys,
        }
    }
}
