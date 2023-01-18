[VkExternalFenceHandleTypeFlagBits](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExternalFenceHandleTypeFlagBits.html) - Bitmask of valid external fence handle types

# C Specifications
Bits which  **may**  be set in
  * [`PhysicalDeviceExternalFenceInfo::handle_type`]
  * [`ExternalFenceProperties::export_from_imported_handle_types`]
  * [`ExternalFenceProperties::compatible_handle_types`]
indicate external fence handle types, and are:
```c
// Provided by VK_VERSION_1_1
typedef enum VkExternalFenceHandleTypeFlagBits {
    VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_FD_BIT = 0x00000001,
    VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_BIT = 0x00000002,
    VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT = 0x00000004,
    VK_EXTERNAL_FENCE_HANDLE_TYPE_SYNC_FD_BIT = 0x00000008,
  // Provided by VK_KHR_external_fence_capabilities
    VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_FD_BIT_KHR = VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_FD_BIT,
  // Provided by VK_KHR_external_fence_capabilities
    VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_BIT_KHR = VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_BIT,
  // Provided by VK_KHR_external_fence_capabilities
    VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_KHR = VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT,
  // Provided by VK_KHR_external_fence_capabilities
    VK_EXTERNAL_FENCE_HANDLE_TYPE_SYNC_FD_BIT_KHR = VK_EXTERNAL_FENCE_HANDLE_TYPE_SYNC_FD_BIT,
} VkExternalFenceHandleTypeFlagBits;
```
or the equivalent
```c
// Provided by VK_KHR_external_fence_capabilities
typedef VkExternalFenceHandleTypeFlagBits VkExternalFenceHandleTypeFlagBitsKHR;
```

# Description
- [`OPAQUE_FD`] specifies a POSIX file descriptor handle that has only limited valid usage outside of Vulkan and other compatible APIs. It  **must**  be compatible with the POSIX system calls `dup`, `dup2`, `close`, and the non-standard system call `dup3`. Additionally, it  **must**  be transportable over a socket using an `SCM_RIGHTS` control message. It owns a reference to the underlying synchronization primitive represented by its Vulkan fence object.
- [`OPAQUE_WIN32`] specifies an NT handle that has only limited valid usage outside of Vulkan and other compatible APIs. It  **must**  be compatible with the functions `DuplicateHandle`, `CloseHandle`, `CompareObjectHandles`, `GetHandleInformation`, and `SetHandleInformation`. It owns a reference to the underlying synchronization primitive represented by its Vulkan fence object.
- [`OPAQUE_WIN32_KMT`] specifies a global share handle that has only limited valid usage outside of Vulkan and other compatible APIs. It is not compatible with any native APIs. It does not own a reference to the underlying synchronization primitive represented by its Vulkan fence object, and will therefore become invalid when all Vulkan fence objects associated with it are destroyed.
- [`SYNC_FD`] specifies a POSIX file descriptor handle to a Linux Sync File or Android Fence. It can be used with any native API accepting a valid sync file or fence as input. It owns a reference to the underlying synchronization primitive associated with the file descriptor. Implementations which support importing this handle type  **must**  accept any type of sync or fence FD supported by the native system they are running on.
Some external fence handle types can only be shared within the same
underlying physical device and/or the same driver version, as defined in the
following table:

# Related
- [`crate::vulkan1_1`]
- [`ExternalFenceHandleTypeFlags`]
- [`FenceGetFdInfoKHR`]
- [`FenceGetWin32HandleInfoKHR`]
- [`ImportFenceFdInfoKHR`]
- [`ImportFenceWin32HandleInfoKHR`]
- [`PhysicalDeviceExternalFenceInfo`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        