[VkDeviceMemory](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceMemory.html) - Opaque handle to a device memory object

# C Specifications
A Vulkan device operates on data in device memory via memory objects that
are represented in the API by a [`DeviceMemory`] handle:
```c
// Provided by VK_VERSION_1_0
VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkDeviceMemory)
```

# Related
- [`crate::vulkan1_0`]
- [`BindAccelerationStructureMemoryInfoNV`]
- [`BindBufferMemoryInfo`]
- [`BindImageMemoryInfo`]
- [`DeviceMemoryOpaqueCaptureAddressInfo`]
- [`MappedMemoryRange`]
- [`MemoryGetAndroidHardwareBufferInfoANDROID`]
- [`MemoryGetFdInfoKHR`]
- [`MemoryGetRemoteAddressInfoNV`]
- [`MemoryGetWin32HandleInfoKHR`]
- [`MemoryGetZirconHandleInfoFUCHSIA`]
- [`SparseImageMemoryBind`]
- [`SparseMemoryBind`]
- [`VideoBindMemoryKHR`]
- [`Win32KeyedMutexAcquireReleaseInfoKHR`]
- [`Win32KeyedMutexAcquireReleaseInfoNV`]
- [`allocate_memory`]
- [`bind_buffer_memory`]
- [`bind_image_memory`]
- [`free_memory`]
- [`get_device_memory_commitment`]
- [`get_memory_win32_handle_nv`]
- [`map_memory`]
- [`set_device_memory_priority_ext`]
- [`unmap_memory`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        