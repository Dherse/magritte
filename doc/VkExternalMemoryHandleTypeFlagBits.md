[VkExternalMemoryHandleTypeFlagBits](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExternalMemoryHandleTypeFlagBits.html) - Bit specifying external memory handle types

# C Specifications
Possible values of
[`PhysicalDeviceExternalImageFormatInfo::handle_type`], specifying
an external memory handle type, are:
```c
// Provided by VK_VERSION_1_1
typedef enum VkExternalMemoryHandleTypeFlagBits {
    VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_FD_BIT = 0x00000001,
    VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT = 0x00000002,
    VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT = 0x00000004,
    VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_BIT = 0x00000008,
    VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_KMT_BIT = 0x00000010,
    VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_HEAP_BIT = 0x00000020,
    VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_RESOURCE_BIT = 0x00000040,
  // Provided by VK_EXT_external_memory_dma_buf
    VK_EXTERNAL_MEMORY_HANDLE_TYPE_DMA_BUF_BIT_EXT = 0x00000200,
  // Provided by VK_ANDROID_external_memory_android_hardware_buffer
    VK_EXTERNAL_MEMORY_HANDLE_TYPE_ANDROID_HARDWARE_BUFFER_BIT_ANDROID = 0x00000400,
  // Provided by VK_EXT_external_memory_host
    VK_EXTERNAL_MEMORY_HANDLE_TYPE_HOST_ALLOCATION_BIT_EXT = 0x00000080,
  // Provided by VK_EXT_external_memory_host
    VK_EXTERNAL_MEMORY_HANDLE_TYPE_HOST_MAPPED_FOREIGN_MEMORY_BIT_EXT = 0x00000100,
  // Provided by VK_FUCHSIA_external_memory
    VK_EXTERNAL_MEMORY_HANDLE_TYPE_ZIRCON_VMO_BIT_FUCHSIA = 0x00000800,
  // Provided by VK_NV_external_memory_rdma
    VK_EXTERNAL_MEMORY_HANDLE_TYPE_RDMA_ADDRESS_BIT_NV = 0x00001000,
  // Provided by VK_KHR_external_memory_capabilities
    VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_FD_BIT_KHR = VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_FD_BIT,
  // Provided by VK_KHR_external_memory_capabilities
    VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT_KHR = VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT,
  // Provided by VK_KHR_external_memory_capabilities
    VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_KHR = VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT,
  // Provided by VK_KHR_external_memory_capabilities
    VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_BIT_KHR = VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_BIT,
  // Provided by VK_KHR_external_memory_capabilities
    VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_KMT_BIT_KHR = VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_KMT_BIT,
  // Provided by VK_KHR_external_memory_capabilities
    VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_HEAP_BIT_KHR = VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_HEAP_BIT,
  // Provided by VK_KHR_external_memory_capabilities
    VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_RESOURCE_BIT_KHR = VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_RESOURCE_BIT,
} VkExternalMemoryHandleTypeFlagBits;
```
or the equivalent
```c
// Provided by VK_KHR_external_memory_capabilities
typedef VkExternalMemoryHandleTypeFlagBits VkExternalMemoryHandleTypeFlagBitsKHR;
```

# Description
- [`VK_EXTERNAL_MEMORY_HANDLE_TYPE_FLAG_BITS`] specifies a POSIX file descriptor handle that has only limited valid usage outside of Vulkan and other compatible APIs. It  **must**  be compatible with the POSIX system calls `dup`, `dup2`, `close`, and the non-standard system call `dup3`. Additionally, it  **must**  be transportable over a socket using an `SCM_RIGHTS` control message. It owns a reference to the underlying memory resource represented by its Vulkan memory object.
- [`VK_EXTERNAL_MEMORY_HANDLE_TYPE_FLAG_BITS`] specifies an NT handle that has only limited valid usage outside of Vulkan and other compatible APIs. It  **must**  be compatible with the functions `DuplicateHandle`, `CloseHandle`, `CompareObjectHandles`, `GetHandleInformation`, and `SetHandleInformation`. It owns a reference to the underlying memory resource represented by its Vulkan memory object.
- [`VK_EXTERNAL_MEMORY_HANDLE_TYPE_FLAG_BITS`] specifies a global share handle that has only limited valid usage outside of Vulkan and other compatible APIs. It is not compatible with any native APIs. It does not own a reference to the underlying memory resource represented by its Vulkan memory object, and will therefore become invalid when all Vulkan memory objects associated with it are destroyed.
- [`VK_EXTERNAL_MEMORY_HANDLE_TYPE_FLAG_BITS`] specifies an NT handle returned by `IDXGIResource1`::`CreateSharedHandle` referring to a Direct3D 10 or 11 texture resource. It owns a reference to the memory used by the Direct3D resource.
- [`VK_EXTERNAL_MEMORY_HANDLE_TYPE_FLAG_BITS`] specifies a global share handle returned by `IDXGIResource`::`GetSharedHandle` referring to a Direct3D 10 or 11 texture resource. It does not own a reference to the underlying Direct3D resource, and will therefore become invalid when all Vulkan memory objects and Direct3D resources associated with it are destroyed.
- [`VK_EXTERNAL_MEMORY_HANDLE_TYPE_FLAG_BITS`] specifies an NT handle returned by `ID3D12Device`::`CreateSharedHandle` referring to a Direct3D 12 heap resource. It owns a reference to the resources used by the Direct3D heap.
- [`VK_EXTERNAL_MEMORY_HANDLE_TYPE_FLAG_BITS`] specifies an NT handle returned by `ID3D12Device`::`CreateSharedHandle` referring to a Direct3D 12 committed resource. It owns a reference to the memory used by the Direct3D resource.
- [`HOST_ALLOCATION_EXT`] specifies a host pointer returned by a host memory allocation command. It does not own a reference to the underlying memory resource, and will therefore become invalid if the host memory is freed.
- [`HOST_MAPPED_FOREIGN_MEMORY_EXT`] specifies a host pointer to *host mapped foreign memory*. It does not own a reference to the underlying memory resource, and will therefore become invalid if the foreign memory is unmapped or otherwise becomes no longer available.
- [`DMA_BUF_EXT`] is a file descriptor for a Linux dma_buf. It owns a reference to the underlying memory resource represented by its Vulkan memory object.
- [`ANDROID_HARDWARE_BUFFER_ANDROID`] specifies an [`AHardwareBuffer`] object defined by the Android NDK. See [Android Hardware Buffers](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-external-android-hardware-buffer) for more details of this handle type.
- [`ZIRCON_VMO_FUCHSIA`] is a Zircon handle to a virtual memory object.
- [`RDMA_ADDRESS_NV`] is a handle to an allocation accessible by remote devices. It owns a reference to the underlying memory resource represented by its Vulkan memory object.
Some external memory handle types can only be shared within the same
underlying physical device and/or the same driver version, as defined in the
following table:

# Related
- [`crate::vulkan1_1`]
- [VkExternalMemoryHandleTypeFlags]()
- [`ImportMemoryFdInfoKHR`]
- [`ImportMemoryHostPointerInfoEXT`]
- [`ImportMemoryWin32HandleInfoKHR`]
- [`ImportMemoryZirconHandleInfoFUCHSIA`]
- [`MemoryGetFdInfoKHR`]
- [`MemoryGetRemoteAddressInfoNV`]
- [`MemoryGetWin32HandleInfoKHR`]
- [`MemoryGetZirconHandleInfoFUCHSIA`]
- [`PhysicalDeviceExternalBufferInfo`]
- [`PhysicalDeviceExternalImageFormatInfo`]
- [`get_memory_fd_properties_khr`]
- [`get_memory_host_pointer_properties_ext`]
- [`get_memory_win32_handle_properties_khr`]
- [`get_memory_zircon_handle_properties_fuchsia`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        