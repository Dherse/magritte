use crate::native::vulkan1_0::{BaseInStructure, DeviceMemory, StructureType};
#[doc(alias = "VkWin32KeyedMutexAcquireReleaseInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct Win32KeyedMutexAcquireReleaseInfoKHR {
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
    #[doc(alias = "pAcquireTimeouts")]
    pub acquire_timeouts: *const u32,
    #[doc(alias = "releaseCount")]
    pub release_count: u32,
    #[doc(alias = "pReleaseSyncs")]
    pub release_syncs: *const DeviceMemory,
    #[doc(alias = "pReleaseKeys")]
    pub release_keys: *const u64,
}
impl Default for Win32KeyedMutexAcquireReleaseInfoKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::Win32KeyedMutexAcquireReleaseInfoKhr,
            p_next: unsafe { std::mem::zeroed() },
            acquire_count: unsafe { std::mem::zeroed() },
            acquire_syncs: unsafe { std::mem::zeroed() },
            acquire_keys: unsafe { std::mem::zeroed() },
            acquire_timeouts: unsafe { std::mem::zeroed() },
            release_count: unsafe { std::mem::zeroed() },
            release_syncs: unsafe { std::mem::zeroed() },
            release_keys: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::khr_win32_keyed_mutex::{
    KHR_WIN32_KEYED_MUTEX_EXTENSION_NAME, KHR_WIN32_KEYED_MUTEX_SPEC_VERSION,
};
