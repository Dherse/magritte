use crate::vulkan1_0::{BaseInStructure, DeviceMemory, StructureType};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_WIN32_KEYED_MUTEX_SPEC_VERSION")]
pub const KHR_WIN32_KEYED_MUTEX_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_WIN32_KEYED_MUTEX_EXTENSION_NAME")]
pub const KHR_WIN32_KEYED_MUTEX_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_win32_keyed_mutex");
///[VkWin32KeyedMutexAcquireReleaseInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkWin32KeyedMutexAcquireReleaseInfoKHR.html) - Use the Windows keyed mutex mechanism to synchronize work
///# C Specifications
///When submitting work that operates on memory imported from a Direct3D 11
///resource to a queue, the keyed mutex mechanism  **may**  be used in addition to
///Vulkan semaphores to synchronize the work.
///Keyed mutexes are a property of a properly created shareable Direct3D 11
///resource.
///They  **can**  only be used if the imported resource was created with the
///`D3D11_RESOURCE_MISC_SHARED_KEYEDMUTEX` flag.To acquire keyed mutexes before submitted work
/// and/or release them after,
///add a [`Win32KeyedMutexAcquireReleaseInfoKHR`] structure to the
///[`p_next`] chain of the [`SubmitInfo`] structure.The [`Win32KeyedMutexAcquireReleaseInfoKHR`]
/// structure is defined as:
///```c
///// Provided by VK_KHR_win32_keyed_mutex
///typedef struct VkWin32KeyedMutexAcquireReleaseInfoKHR {
///    VkStructureType          sType;
///    const void*              pNext;
///    uint32_t                 acquireCount;
///    const VkDeviceMemory*    pAcquireSyncs;
///    const uint64_t*          pAcquireKeys;
///    const uint32_t*          pAcquireTimeouts;
///    uint32_t                 releaseCount;
///    const VkDeviceMemory*    pReleaseSyncs;
///    const uint64_t*          pReleaseKeys;
///} VkWin32KeyedMutexAcquireReleaseInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`acquire_count`] is the number of entries in the [`acquire_syncs`], [`acquire_keys`], and
///   [`acquire_timeouts`] arrays.
/// - [`acquire_syncs`] is a pointer to an array of [`DeviceMemory`] objects which were imported
///   from Direct3D 11 resources.
/// - [`acquire_keys`] is a pointer to an array of mutex key values to wait for prior to beginning
///   the submitted work. Entries refer to the keyed mutex associated with the corresponding entries
///   in [`acquire_syncs`].
/// - [`acquire_timeouts`] is a pointer to an array of timeout values, in millisecond units, for
///   each acquire specified in [`acquire_keys`].
/// - [`release_count`] is the number of entries in the [`release_syncs`] and [`release_keys`]
///   arrays.
/// - [`release_syncs`] is a pointer to an array of [`DeviceMemory`] objects which were imported
///   from Direct3D 11 resources.
/// - [`release_keys`] is a pointer to an array of mutex key values to set when the submitted work
///   has completed. Entries refer to the keyed mutex associated with the corresponding entries in
///   [`release_syncs`].
///# Description
///## Valid Usage
/// - Each member of [`acquire_syncs`] and [`release_syncs`] **must**  be a device memory object
///   imported by setting [`ImportMemoryWin32HandleInfoKHR::handle_type`] to
///   `VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_BIT` or
///   `VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_KMT_BIT`
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_KHR`
/// - If [`acquire_count`] is not `0`, [`acquire_syncs`] **must**  be a valid pointer to an array of
///   [`acquire_count`] valid [`DeviceMemory`] handles
/// - If [`acquire_count`] is not `0`, [`acquire_keys`] **must**  be a valid pointer to an array of
///   [`acquire_count`]`uint64_t` values
/// - If [`acquire_count`] is not `0`, [`acquire_timeouts`] **must**  be a valid pointer to an array
///   of [`acquire_count`]`uint32_t` values
/// - If [`release_count`] is not `0`, [`release_syncs`] **must**  be a valid pointer to an array of
///   [`release_count`] valid [`DeviceMemory`] handles
/// - If [`release_count`] is not `0`, [`release_keys`] **must**  be a valid pointer to an array of
///   [`release_count`]`uint64_t` values
/// - Both of the elements of [`acquire_syncs`], and the elements of [`release_syncs`] that are
///   valid handles of non-ignored parameters  **must**  have been created, allocated, or retrieved
///   from the same [`Device`]
///# Related
/// - [`VK_KHR_win32_keyed_mutex`]
/// - [`DeviceMemory`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct Win32KeyedMutexAcquireReleaseInfoKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`acquire_count`] is the number of entries in the [`acquire_syncs`],
    ///[`acquire_keys`], and [`acquire_timeouts`] arrays.
    acquire_count: u32,
    ///[`acquire_syncs`] is a pointer to an array of [`DeviceMemory`]
    ///objects which were imported from Direct3D 11 resources.
    acquire_syncs: *const DeviceMemory,
    ///[`acquire_keys`] is a pointer to an array of mutex key values to wait
    ///for prior to beginning the submitted work.
    ///Entries refer to the keyed mutex associated with the corresponding
    ///entries in [`acquire_syncs`].
    acquire_keys: *const u64,
    ///[`acquire_timeouts`] is a pointer to an array of timeout values, in
    ///millisecond units, for each acquire specified in [`acquire_keys`].
    acquire_timeouts: *const u32,
    ///[`release_count`] is the number of entries in the [`release_syncs`]
    ///and [`release_keys`] arrays.
    release_count: u32,
    ///[`release_syncs`] is a pointer to an array of [`DeviceMemory`]
    ///objects which were imported from Direct3D 11 resources.
    release_syncs: *const DeviceMemory,
    ///[`release_keys`] is a pointer to an array of mutex key values to set
    ///when the submitted work has completed.
    ///Entries refer to the keyed mutex associated with the corresponding
    ///entries in [`release_syncs`].
    release_keys: *const u64,
}
impl<'lt> Default for Win32KeyedMutexAcquireReleaseInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            acquire_count: 0,
            acquire_syncs: std::ptr::null(),
            acquire_keys: std::ptr::null(),
            acquire_timeouts: std::ptr::null(),
            release_count: 0,
            release_syncs: std::ptr::null(),
            release_keys: std::ptr::null(),
        }
    }
}
impl<'lt> Win32KeyedMutexAcquireReleaseInfoKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::acquire_syncs`]
    pub fn acquire_syncs_raw(&self) -> *const DeviceMemory {
        self.acquire_syncs
    }
    ///Gets the raw value of [`Self::acquire_keys`]
    pub fn acquire_keys_raw(&self) -> *const u64 {
        self.acquire_keys
    }
    ///Gets the raw value of [`Self::acquire_timeouts`]
    pub fn acquire_timeouts_raw(&self) -> *const u32 {
        self.acquire_timeouts
    }
    ///Gets the raw value of [`Self::release_syncs`]
    pub fn release_syncs_raw(&self) -> *const DeviceMemory {
        self.release_syncs
    }
    ///Gets the raw value of [`Self::release_keys`]
    pub fn release_keys_raw(&self) -> *const u64 {
        self.release_keys
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::acquire_syncs`]
    pub fn set_acquire_syncs_raw(&mut self, value: *const DeviceMemory) -> &mut Self {
        self.acquire_syncs = value;
        self
    }
    ///Sets the raw value of [`Self::acquire_keys`]
    pub fn set_acquire_keys_raw(&mut self, value: *const u64) -> &mut Self {
        self.acquire_keys = value;
        self
    }
    ///Sets the raw value of [`Self::acquire_timeouts`]
    pub fn set_acquire_timeouts_raw(&mut self, value: *const u32) -> &mut Self {
        self.acquire_timeouts = value;
        self
    }
    ///Sets the raw value of [`Self::release_syncs`]
    pub fn set_release_syncs_raw(&mut self, value: *const DeviceMemory) -> &mut Self {
        self.release_syncs = value;
        self
    }
    ///Sets the raw value of [`Self::release_keys`]
    pub fn set_release_keys_raw(&mut self, value: *const u64) -> &mut Self {
        self.release_keys = value;
        self
    }
    ///Gets the value of [`Self::s_type`]
    pub fn s_type(&self) -> StructureType {
        self.s_type
    }
    ///Gets the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next(&self) -> &BaseInStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::acquire_count`]
    pub fn acquire_count(&self) -> u32 {
        self.acquire_count
    }
    ///Gets the value of [`Self::acquire_syncs`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn acquire_syncs(&self) -> &[DeviceMemory] {
        std::slice::from_raw_parts(self.acquire_syncs, self.acquire_count as usize)
    }
    ///Gets the value of [`Self::acquire_keys`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn acquire_keys(&self) -> &[u64] {
        std::slice::from_raw_parts(self.acquire_keys, self.acquire_count as usize)
    }
    ///Gets the value of [`Self::acquire_timeouts`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn acquire_timeouts(&self) -> &[u32] {
        std::slice::from_raw_parts(self.acquire_timeouts, self.acquire_count as usize)
    }
    ///Gets the value of [`Self::release_count`]
    pub fn release_count(&self) -> u32 {
        self.release_count
    }
    ///Gets the value of [`Self::release_syncs`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn release_syncs(&self) -> &[DeviceMemory] {
        std::slice::from_raw_parts(self.release_syncs, self.release_count as usize)
    }
    ///Gets the value of [`Self::release_keys`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn release_keys(&self) -> &[u64] {
        std::slice::from_raw_parts(self.release_keys, self.release_count as usize)
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::acquire_count`]
    pub fn acquire_count_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::release_count`]
    pub fn release_count_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the raw value of [`Self::acquire_count`]
    pub fn set_acquire_count(&mut self, value: u32) -> &mut Self {
        self.acquire_count = value;
        self
    }
    ///Sets the raw value of [`Self::acquire_syncs`]
    pub fn set_acquire_syncs(&mut self, value: &'lt [crate::vulkan1_0::DeviceMemory]) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.acquire_syncs = value.as_ptr();
        self.acquire_count = len_;
        self
    }
    ///Sets the raw value of [`Self::acquire_keys`]
    pub fn set_acquire_keys(&mut self, value: &'lt [u64]) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.acquire_keys = value.as_ptr();
        self.acquire_count = len_;
        self
    }
    ///Sets the raw value of [`Self::acquire_timeouts`]
    pub fn set_acquire_timeouts(&mut self, value: &'lt [u32]) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.acquire_timeouts = value.as_ptr();
        self.acquire_count = len_;
        self
    }
    ///Sets the raw value of [`Self::release_count`]
    pub fn set_release_count(&mut self, value: u32) -> &mut Self {
        self.release_count = value;
        self
    }
    ///Sets the raw value of [`Self::release_syncs`]
    pub fn set_release_syncs(&mut self, value: &'lt [crate::vulkan1_0::DeviceMemory]) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.release_syncs = value.as_ptr();
        self.release_count = len_;
        self
    }
    ///Sets the raw value of [`Self::release_keys`]
    pub fn set_release_keys(&mut self, value: &'lt [u64]) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.release_keys = value.as_ptr();
        self.release_count = len_;
        self
    }
}
