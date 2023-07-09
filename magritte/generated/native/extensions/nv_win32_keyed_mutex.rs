use crate::native::vulkan1_0::{BaseInStructure, DeviceMemory, StructureType};
#[doc(alias = "VkWin32KeyedMutexAcquireReleaseInfoNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct Win32KeyedMutexAcquireReleaseInfoNV {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "acquireCount")]
    pub acquire_count: u32,
    #[doc(alias = "pAcquireSyncs")]
    pub acquire_syncs: *const DeviceMemory,
    #[doc(alias = "pAcquireKeys")]
    pub acquire_keys: *const u64,
    #[doc(alias = "pAcquireTimeoutMilliseconds")]
    pub acquire_timeout_milliseconds: *const u32,
    #[doc(alias = "releaseCount")]
    pub release_count: u32,
    #[doc(alias = "pReleaseSyncs")]
    pub release_syncs: *const DeviceMemory,
    #[doc(alias = "pReleaseKeys")]
    pub release_keys: *const u64,
}
impl Default for Win32KeyedMutexAcquireReleaseInfoNV {
    fn default() -> Self {
        Self {
            s_type: StructureType::Win32KeyedMutexAcquireReleaseInfoNv,
            p_next: unsafe { std::mem::zeroed() },
            acquire_count: unsafe { std::mem::zeroed() },
            acquire_syncs: unsafe { std::mem::zeroed() },
            acquire_keys: unsafe { std::mem::zeroed() },
            acquire_timeout_milliseconds: unsafe { std::mem::zeroed() },
            release_count: unsafe { std::mem::zeroed() },
            release_syncs: unsafe { std::mem::zeroed() },
            release_keys: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::nv_win32_keyed_mutex::{
    NV_WIN32_KEYED_MUTEX_EXTENSION_NAME, NV_WIN32_KEYED_MUTEX_SPEC_VERSION,
};
