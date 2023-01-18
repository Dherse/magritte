[VkExternalSemaphoreHandleTypeFlagBits](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExternalSemaphoreHandleTypeFlagBits.html) - Bitmask of valid external semaphore handle types

# C Specifications
Bits which  **may**  be set in
[`PhysicalDeviceExternalSemaphoreInfo::handle_type`], specifying an
external semaphore handle type, are:
```c
// Provided by VK_VERSION_1_1
typedef enum VkExternalSemaphoreHandleTypeFlagBits {
    VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_FD_BIT = 0x00000001,
    VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_BIT = 0x00000002,
    VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT = 0x00000004,
    VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE_BIT = 0x00000008,
    VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_SYNC_FD_BIT = 0x00000010,
  // Provided by VK_FUCHSIA_external_semaphore
    VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_ZIRCON_EVENT_BIT_FUCHSIA = 0x00000080,
    VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D11_FENCE_BIT = VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE_BIT,
  // Provided by VK_KHR_external_semaphore_capabilities
    VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_FD_BIT_KHR = VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_FD_BIT,
  // Provided by VK_KHR_external_semaphore_capabilities
    VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_BIT_KHR = VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_BIT,
  // Provided by VK_KHR_external_semaphore_capabilities
    VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_KHR = VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT,
  // Provided by VK_KHR_external_semaphore_capabilities
    VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE_BIT_KHR = VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE_BIT,
  // Provided by VK_KHR_external_semaphore_capabilities
    VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_SYNC_FD_BIT_KHR = VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_SYNC_FD_BIT,
} VkExternalSemaphoreHandleTypeFlagBits;
```
or the equivalent
```c
// Provided by VK_KHR_external_semaphore_capabilities
typedef VkExternalSemaphoreHandleTypeFlagBits VkExternalSemaphoreHandleTypeFlagBitsKHR;
```

# Description
- [`OPAQUE_FD`] specifies a POSIX file descriptor handle that has only limited valid usage outside of Vulkan and other compatible APIs. It  **must**  be compatible with the POSIX system calls `dup`, `dup2`, `close`, and the non-standard system call `dup3`. Additionally, it  **must**  be transportable over a socket using an `SCM_RIGHTS` control message. It owns a reference to the underlying synchronization primitive represented by its Vulkan semaphore object.
- [`OPAQUE_WIN32`] specifies an NT handle that has only limited valid usage outside of Vulkan and other compatible APIs. It  **must**  be compatible with the functions `DuplicateHandle`, `CloseHandle`, `CompareObjectHandles`, `GetHandleInformation`, and `SetHandleInformation`. It owns a reference to the underlying synchronization primitive represented by its Vulkan semaphore object.
- [`OPAQUE_WIN32_KMT`] specifies a global share handle that has only limited valid usage outside of Vulkan and other compatible APIs. It is not compatible with any native APIs. It does not own a reference to the underlying synchronization primitive represented by its Vulkan semaphore object, and will therefore become invalid when all Vulkan semaphore objects associated with it are destroyed.
- [`D3D12_FENCE`] specifies an NT handle returned by `ID3D12Device`::`CreateSharedHandle` referring to a Direct3D 12 fence, or `ID3D11Device5`::`CreateFence` referring to a Direct3D 11 fence. It owns a reference to the underlying synchronization primitive associated with the Direct3D fence.
- [`D3D11_FENCE`] is an alias of [`D3D12_FENCE`] with the same meaning. It is provided for convenience and code clarity when interacting with D3D11 fences.
- [`SYNC_FD`] specifies a POSIX file descriptor handle to a Linux Sync File or Android Fence object. It can be used with any native API accepting a valid sync file or fence as input. It owns a reference to the underlying synchronization primitive associated with the file descriptor. Implementations which support importing this handle type  **must**  accept any type of sync or fence FD supported by the native system they are running on.
- [`ZIRCON_EVENT_FUCHSIA`] specifies a handle to a Zircon event object. It can be used with any native API that accepts a Zircon event handle. Zircon event handles are created with `ZX_RIGHTS_BASIC` and `ZX_RIGHTS_SIGNAL` rights. Vulkan on Fuchsia uses only the ZX_EVENT_SIGNALED bit when signaling or waiting.
Some external semaphore handle types can only be shared within the same
underlying physical device and/or the same driver version, as defined in the
following table:

# Related
- [`crate::vulkan1_1`]
- [`ExternalSemaphoreHandleTypeFlags`]
- [`ImportSemaphoreFdInfoKHR`]
- [`ImportSemaphoreWin32HandleInfoKHR`]
- [`ImportSemaphoreZirconHandleInfoFUCHSIA`]
- [`PhysicalDeviceExternalSemaphoreInfo`]
- [`SemaphoreGetFdInfoKHR`]
- [`SemaphoreGetWin32HandleInfoKHR`]
- [`SemaphoreGetZirconHandleInfoFUCHSIA`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        