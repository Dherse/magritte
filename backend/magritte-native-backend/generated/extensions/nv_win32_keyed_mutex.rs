//!# [VK_NV_win32_keyed_mutex](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_NV_win32_keyed_mutex.html)
# ! [doc = include_str ! ("../../../../doc/extensions/nv_win32_keyed_mutex/VK_NV_win32_keyed_mutex.md")]
use crate::{
    cstr,
    vulkan1_0::{BaseInStructure, DeviceMemory, StructureType},
};
use std::ffi::CStr;
///# [VkWin32KeyedMutexAcquireReleaseInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkWin32KeyedMutexAcquireReleaseInfoNV.html)
# [doc = include_str ! ("../../../../doc/extensions/nv_win32_keyed_mutex/VkWin32KeyedMutexAcquireReleaseInfoNV.md")]
#[doc(alias = "VkWin32KeyedMutexAcquireReleaseInfoNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct Win32KeyedMutexAcquireReleaseInfoNV {
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
    #[doc(alias = "pAcquireTimeoutMilliseconds")]
    acquire_timeout_milliseconds: *const u32,
    #[doc(alias = "releaseCount")]
    release_count: u32,
    #[doc(alias = "pReleaseSyncs")]
    release_syncs: *const DeviceMemory,
    #[doc(alias = "pReleaseKeys")]
    release_keys: *const u64,
}
#[doc(alias = "VK_NV_WIN32_KEYED_MUTEX_SPEC_VERSION")]
pub const NV_WIN32_KEYED_MUTEX_SPEC_VERSION: u32 = 2;
#[doc(alias = "VK_NV_WIN32_KEYED_MUTEX_EXTENSION_NAME")]
pub const NV_WIN32_KEYED_MUTEX_EXTENSION_NAME: &'static CStr = cstr!("VK_NV_win32_keyed_mutex");