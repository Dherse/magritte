//!# [VK_KHR_win32_keyed_mutex](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_win32_keyed_mutex.html)
# ! [doc = include_str ! ("../../../../doc/extensions/khr_win32_keyed_mutex/VK_KHR_win32_keyed_mutex.md")]
use crate::{
    cstr,
    vulkan1_0::{BaseInStructure, DeviceMemory, StructureType},
};
use std::ffi::CStr;
///# [VkWin32KeyedMutexAcquireReleaseInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkWin32KeyedMutexAcquireReleaseInfoKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_win32_keyed_mutex/VkWin32KeyedMutexAcquireReleaseInfoKHR.md")]
#[doc(alias = "VkWin32KeyedMutexAcquireReleaseInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct Win32KeyedMutexAcquireReleaseInfoKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "acquireCount")]
    acquire_count: u32,
    #[doc(alias = "pAcquireSyncs")]
    acquire_syncs: *const DeviceMemory,
    #[doc(alias = "pAcquireKeys")]
    acquire_keys: *const u64,
    #[doc(alias = "pAcquireTimeouts")]
    acquire_timeouts: *const u32,
    #[doc(alias = "releaseCount")]
    release_count: u32,
    #[doc(alias = "pReleaseSyncs")]
    release_syncs: *const DeviceMemory,
    #[doc(alias = "pReleaseKeys")]
    release_keys: *const u64,
}
#[doc(alias = "VK_KHR_WIN32_KEYED_MUTEX_SPEC_VERSION")]
pub const KHR_WIN32_KEYED_MUTEX_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_KHR_WIN32_KEYED_MUTEX_EXTENSION_NAME")]
pub const KHR_WIN32_KEYED_MUTEX_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_win32_keyed_mutex");
