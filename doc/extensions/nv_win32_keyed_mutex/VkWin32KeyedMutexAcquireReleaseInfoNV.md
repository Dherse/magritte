[VkWin32KeyedMutexAcquireReleaseInfoNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkWin32KeyedMutexAcquireReleaseInfoNV.html) - Use Windows keyex mutex mechanism to synchronize work

# C Specifications
When submitting work that operates on memory imported from a Direct3D 11
resource to a queue, the keyed mutex mechanism  **may**  be used in addition to
Vulkan semaphores to synchronize the work.
Keyed mutexes are a property of a properly created shareable Direct3D 11
resource.
They  **can**  only be used if the imported resource was created with the
`D3D11_RESOURCE_MISC_SHARED_KEYEDMUTEX` flag.To acquire keyed mutexes before submitted work and/or release them after,
add a [`Win32KeyedMutexAcquireReleaseInfoNV`] structure to the
[`p_next`] chain of the [`SubmitInfo`] structure.The [`Win32KeyedMutexAcquireReleaseInfoNV`] structure is defined as:
```c
// Provided by VK_NV_win32_keyed_mutex
typedef struct VkWin32KeyedMutexAcquireReleaseInfoNV {
    VkStructureType          sType;
    const void*              pNext;
    uint32_t                 acquireCount;
    const VkDeviceMemory*    pAcquireSyncs;
    const uint64_t*          pAcquireKeys;
    const uint32_t*          pAcquireTimeoutMilliseconds;
    uint32_t                 releaseCount;
    const VkDeviceMemory*    pReleaseSyncs;
    const uint64_t*          pReleaseKeys;
} VkWin32KeyedMutexAcquireReleaseInfoNV;
```

# Members
- [`acquire_count`] is the number of entries in the [`acquire_syncs`], [`acquire_keys`], and [`acquire_timeout_milliseconds`] arrays.
- [`acquire_syncs`] is a pointer to an array of [`DeviceMemory`] objects which were imported from Direct3D 11 resources.
- [`acquire_keys`] is a pointer to an array of mutex key values to wait for prior to beginning the submitted work. Entries refer to the keyed mutex associated with the corresponding entries in [`acquire_syncs`].
- [`acquire_timeout_milliseconds`] is a pointer to an array of timeout values, in millisecond units, for each acquire specified in [`acquire_keys`].
- [`release_count`] is the number of entries in the [`release_syncs`] and [`release_keys`] arrays.
- [`release_syncs`] is a pointer to an array of [`DeviceMemory`] objects which were imported from Direct3D 11 resources.
- [`release_keys`] is a pointer to an array of mutex key values to set when the submitted work has completed. Entries refer to the keyed mutex associated with the corresponding entries in [`release_syncs`].

# Description
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_NV`
-    If [`acquire_count`] is not `0`, [`acquire_syncs`] **must**  be a valid pointer to an array of [`acquire_count`] valid [`DeviceMemory`] handles
-    If [`acquire_count`] is not `0`, [`acquire_keys`] **must**  be a valid pointer to an array of [`acquire_count`]`uint64_t` values
-    If [`acquire_count`] is not `0`, [`acquire_timeout_milliseconds`] **must**  be a valid pointer to an array of [`acquire_count`]`uint32_t` values
-    If [`release_count`] is not `0`, [`release_syncs`] **must**  be a valid pointer to an array of [`release_count`] valid [`DeviceMemory`] handles
-    If [`release_count`] is not `0`, [`release_keys`] **must**  be a valid pointer to an array of [`release_count`]`uint64_t` values
-    Both of the elements of [`acquire_syncs`], and the elements of [`release_syncs`] that are valid handles of non-ignored parameters  **must**  have been created, allocated, or retrieved from the same [`Device`]

# Related
- [`VK_NV_win32_keyed_mutex`]
- [`DeviceMemory`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        